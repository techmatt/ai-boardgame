//! Simulation CLI: plays seeded random-agent games for one or two configs,
//! prints win rate, game length and counters side by side, and replay-checks
//! every game. See README.md for usage.

mod agents;
mod experiment;
mod report;

use std::collections::BTreeMap;
use std::process::exit;
use std::time::Instant;

use agents::AgentSpec;
use aic_core::{
    content_hash, play, replay, Action, Config, Event, GameRecord, Outcome, Rules, SavedGame,
};
use serde::Serialize;

const USAGE: &str = "\
usage: aic-sim run <config.json> [options]
       aic-sim compare <a.json> <b.json> [options]
       aic-sim show <config.json> [--set k=v]...     resolved config + hash
       aic-sim modules                               module registry as JSON
       aic-sim replay <record.json>...               check saved game records

options:
  --games N         games per config (default 200)
  --seed S          first seed (default 0); seeds S..S+N
  --set K=V         override for every config: countries=N, <module>.<param>=V,
                    <module>.variant=NAME; V is 3 or 1,2,3 (repeatable)
  --set-a K=V       override for config A only (repeatable)
  --set-b K=V       override for config B only (repeatable)
  --agent SPEC      the team agent: random (default), greedy, beam, rollout, engine,
                    with budget parameters as kind:key=value,... (see README)
  --max-actions N   per-game action cap; a capped game is a timeout (default 100000)
  --threads N       worker threads (default: all cores); results do not depend on it
  --json            print the summary as JSON instead of a table
  --trace           print the first game of each config as JSON lines (actions, events)

saving game records (run, compare):
  --save DIR        write the selected games as game records into DIR (created if needed),
                    named <config name>-<run|A|B>-seed<S>.json
  --save-when W     select games by outcome: all, win, loss, timeout, or an end reason
                    code such as overrun or survived (repeatable; default loss, or none
                    when only --save-seed is given)
  --save-seed S     always save this seed, whatever --save-when says (repeatable)
  --save-max N      at most the first N games --save-when selects, in seed order, per
                    config (default 20); --save-seed games are not capped

replay:
  aic-sim replay <record.json>...   replay each record and check it against what was
                    saved; exit 1 if any record fails or has warnings

experiment:
  aic-sim experiment <spec.json> [--out DIR] [--games N] [--seed S] [--threads N]
                    play the spec's grid (configs x variants x difficulties x agents
                    over a seed range) and write the cycle report into DIR (default
                    out/experiments/<name>): report.md, results.json, games.csv,
                    cards.csv, records/. --games and --seed override the spec.";

struct Options {
    command: String,
    paths: Vec<String>,
    games: u64,
    seed: u64,
    set: Vec<String>,
    set_a: Vec<String>,
    set_b: Vec<String>,
    max_actions: usize,
    threads: usize,
    json: bool,
    trace: bool,
    save: Option<String>,
    save_when: Vec<String>,
    save_seeds: Vec<u64>,
    save_max: usize,
    agent: AgentSpec,
    /// `experiment`: output directory, and whether --games/--seed were given.
    out: Option<String>,
    games_given: bool,
    seed_given: bool,
    max_actions_given: bool,
}

fn die(msg: &str) -> ! {
    eprintln!("{msg}\n\n{USAGE}");
    exit(2)
}

fn parse_args() -> Options {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| die("missing command"));
    let mut o = Options {
        command,
        paths: Vec::new(),
        games: 200,
        seed: 0,
        set: Vec::new(),
        set_a: Vec::new(),
        set_b: Vec::new(),
        max_actions: 100_000,
        threads: std::thread::available_parallelism().map_or(1, |n| n.get()),
        json: false,
        trace: false,
        save: None,
        save_when: Vec::new(),
        save_seeds: Vec::new(),
        save_max: 20,
        agent: AgentSpec::parse("random").expect("random is an agent"),
        out: None,
        games_given: false,
        seed_given: false,
        max_actions_given: false,
    };
    let mut save_max_given = false;
    while let Some(a) = args.next() {
        let mut value = |name: &str| {
            args.next()
                .unwrap_or_else(|| die(&format!("{name} needs a value")))
        };
        fn num<T: std::str::FromStr>(name: &str, v: String) -> T {
            v.parse()
                .unwrap_or_else(|_| die(&format!("{name}: not a number: {v:?}")))
        }
        match a.as_str() {
            "--games" => {
                o.games = num("--games", value("--games"));
                o.games_given = true;
            }
            "--seed" => {
                o.seed = num("--seed", value("--seed"));
                o.seed_given = true;
            }
            "--out" => o.out = Some(value("--out")),
            "--set" => o.set.push(value("--set")),
            "--set-a" => o.set_a.push(value("--set-a")),
            "--set-b" => o.set_b.push(value("--set-b")),
            "--max-actions" => {
                o.max_actions = num("--max-actions", value("--max-actions"));
                o.max_actions_given = true;
            }
            "--agent" => o.agent = AgentSpec::parse(&value("--agent")).unwrap_or_else(|e| die(&e)),
            "--threads" => o.threads = num::<usize>("--threads", value("--threads")).max(1),
            "--json" => o.json = true,
            "--trace" => o.trace = true,
            "--save" => {
                let d = value("--save");
                if d.is_empty() {
                    die("--save needs a directory");
                }
                o.save = Some(d)
            }
            "--save-when" => {
                let w = value("--save-when");
                if w.is_empty() {
                    die("--save-when needs a value");
                }
                o.save_when.push(w)
            }
            "--save-seed" => o.save_seeds.push(num("--save-seed", value("--save-seed"))),
            "--save-max" => {
                o.save_max = num("--save-max", value("--save-max"));
                save_max_given = true;
            }
            "-h" | "--help" => {
                println!("{USAGE}");
                exit(0)
            }
            s if s.starts_with("--") => die(&format!("unknown option {s:?}")),
            _ => o.paths.push(a),
        }
    }
    if o.command == "experiment" {
        let ignored = [
            (
                !o.set.is_empty() || !o.set_a.is_empty() || !o.set_b.is_empty(),
                "--set (use the spec's variants)",
            ),
            (
                o.agent.kind != "random" || !o.agent.params.is_empty(),
                "--agent (use the spec's agents)",
            ),
            (o.json, "--json (results.json is always written)"),
            (o.trace, "--trace"),
            (o.save.is_some(), "--save (records are always saved)"),
        ];
        for (given, what) in ignored {
            if given {
                die(&format!("experiment does not take {what}"));
            }
        }
        if o.out
            .as_deref()
            .is_some_and(|d| d.trim_end_matches(['/', '\\']).is_empty())
        {
            die("--out needs a directory");
        }
    }
    if o.out.is_some() && o.command != "experiment" {
        die("--out needs experiment");
    }
    if o.save.is_none() && (!o.save_when.is_empty() || !o.save_seeds.is_empty() || save_max_given) {
        die("--save-when, --save-seed and --save-max need --save DIR");
    }
    if o.save.is_some() && o.command != "run" && o.command != "compare" {
        die("--save needs run or compare");
    }
    if o.save.is_some() && o.save_when.is_empty() && o.save_seeds.is_empty() {
        o.save_when.push("loss".to_string());
    }
    o.save_seeds.sort_unstable();
    o.save_seeds.dedup();
    if o.games == 0 {
        die("--games must be at least 1");
    }
    if o.seed.checked_add(o.games - 1).is_none() {
        die(&format!(
            "--seed {} + --games {} overflows u64",
            o.seed, o.games
        ));
    }
    o
}

impl Options {
    /// The last seed played (inclusive), so `u64::MAX` itself is reachable.
    fn last_seed(&self) -> u64 {
        self.seed + (self.games - 1)
    }
}

fn load(path: &str, overrides: &[&String]) -> Config {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut c = Config::from_json(&text).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    for o in overrides {
        c.apply_override(o)
            .unwrap_or_else(|e| die(&format!("{path}: {e}")));
    }
    c
}

fn rules(path: &str, c: &Config) -> Rules {
    Rules::new(c).unwrap_or_else(|e| die(&format!("{path}: invalid config: {e}")))
}

fn main() {
    let o = parse_args();
    match o.command.as_str() {
        "modules" => println!(
            "{}",
            serde_json::to_string_pretty(&aic_core::describe_modules()).unwrap()
        ),
        "show" => {
            let [path] = o.paths.as_slice() else {
                die("show takes one config")
            };
            let r = rules(path, &load(path, &o.set.iter().collect::<Vec<_>>()));
            println!(
                "config hash {}  content hash {}  rules revision {}",
                r.config_hash,
                content_hash(),
                aic_core::RULES_REVISION
            );
            println!("{}", r.config.to_json_pretty());
            println!("\ntunables (--set <key>=<value>):");
            for t in r.tunables() {
                println!(
                    "  {:28} {:>12}  {} {}..={}  {}",
                    t.key,
                    t.value.to_string(),
                    t.kind,
                    t.min,
                    t.max,
                    t.doc
                );
            }
        }
        "run" | "compare" => {
            let want = if o.command == "run" { 1 } else { 2 };
            if o.paths.len() != want {
                die(&format!("{} takes {want} config path(s)", o.command));
            }
            if o.command == "run" && !o.set_b.is_empty() {
                die("--set-b needs compare");
            }
            let mut sides = Vec::new();
            let mut records = Vec::new();
            for (i, path) in o.paths.iter().enumerate() {
                let own = if i == 0 { &o.set_a } else { &o.set_b };
                let overrides: Vec<&String> = o.set.iter().chain(own).collect();
                let config = load(path, &overrides);
                let r = rules(path, &config);
                if o.trace {
                    trace(&r, &o.agent, o.seed, o.max_actions, o.json);
                }
                let label = if want == 1 {
                    "run".to_string()
                } else {
                    ["A", "B"][i].to_string()
                };
                let (summary, saved) = simulate(label, path, overrides, &r, &o);
                sides.push(summary);
                records.push(saved);
            }
            if let Some(dir) = &o.save {
                let outside: Vec<String> = o
                    .save_seeds
                    .iter()
                    .filter(|&&s| s < o.seed || s > o.last_seed())
                    .map(|s| s.to_string())
                    .collect();
                if !outside.is_empty() {
                    eprintln!(
                        "warning: --save-seed {} outside the seeds played ({}..={}), not saved",
                        outside.join(", "),
                        o.seed,
                        o.last_seed()
                    );
                }
                for (side, saved) in sides.iter_mut().zip(&records) {
                    let paths = write_records(dir, side, saved);
                    if let Some(rep) = side.saved.as_mut() {
                        rep.paths = paths;
                    }
                }
            }
            let diff = if sides.len() == 2 {
                sides[0].config.diff(&sides[1].config)
            } else {
                Vec::new()
            };
            if o.json {
                let out = serde_json::json!({
                    "games": o.games,
                    "first_seed": o.seed.to_string(),
                    "last_seed": o.last_seed().to_string(),
                    "agent": o.agent.label(),
                    "max_actions": o.max_actions,
                    "rules_revision": aic_core::RULES_REVISION,
                    "diff": diff,
                    "results": sides,
                });
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                print_table(&sides, &diff, &o);
                print_saved(&sides);
            }
            let bad: u64 = sides.iter().map(|s| s.games - s.replay_ok).sum();
            if bad > 0 {
                eprintln!("REPLAY MISMATCH in {bad} game(s)");
                exit(1);
            }
        }
        "replay" => {
            if o.paths.is_empty() {
                die("replay takes one or more game records");
            }
            let bad = o.paths.iter().filter(|p| !replay_record(p)).count();
            if bad > 0 {
                eprintln!(
                    "{bad} of {} record(s) failed or had warnings",
                    o.paths.len()
                );
                exit(1);
            }
        }
        "experiment" => {
            let [path] = o.paths.as_slice() else {
                die("experiment takes one spec file")
            };
            exit(run_experiment(path, &o));
        }
        other => die(&format!("unknown command {other:?}")),
    }
}

/// Replays one saved record and prints what it found. True if it replays
/// with no warnings.
fn replay_record(path: &str) -> bool {
    let loaded = std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|t| SavedGame::from_json(&t));
    let rec = match loaded {
        Ok(r) => r,
        Err(e) => {
            println!("{path}: ERROR {e}");
            return false;
        }
    };
    match rec.replay() {
        Err(e) => {
            println!("{path}: seed {}: ERROR {e}", rec.seed);
            false
        }
        Ok((g, warnings)) => {
            println!(
                "{path}: seed {}, {}, round {}, {} actions, fingerprint {}{}",
                rec.seed,
                describe(g.outcome.as_ref(), rec.actions.len()),
                g.round,
                rec.actions.len(),
                g.fingerprint(),
                if warnings.is_empty() { ", ok" } else { "" }
            );
            if !rec.note.is_empty() {
                println!("  note: {}", rec.note);
            }
            for w in &warnings {
                println!("  WARNING {w}");
            }
            warnings.is_empty()
        }
    }
}

/// `win (survived)`, `loss (overrun)`, `timeout (100000 actions)`.
fn describe(outcome: Option<&Outcome>, actions: usize) -> String {
    match outcome {
        Some(o) => format!("{} ({})", if o.won { "win" } else { "loss" }, o.reason),
        None => format!("timeout ({actions} actions)"),
    }
}

/// Which `--save-when` selector a game matches first, if any.
fn when_match<'a>(when: &'a [String], outcome: Option<&Outcome>) -> Option<&'a String> {
    when.iter().find(|w| match (w.as_str(), outcome) {
        ("all", _) => true,
        ("win", Some(o)) => o.won,
        ("loss", Some(o)) => !o.won,
        ("timeout", None) => true,
        (code, Some(o)) => o.reason == code,
        _ => false,
    })
}

/// A file-name-safe version of a config name.
fn file_stem(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .take(64)
        .collect();
    if s.trim_matches('_').is_empty() {
        "game".to_string()
    } else {
        s
    }
}

/// Writes one side's records into `dir`; exits 2 on any failure.
fn write_records(dir: &str, side: &Summary, saved: &[SavedGame]) -> Vec<String> {
    let fail = |what: &str, e: std::io::Error| -> ! {
        eprintln!("--save {dir}: cannot write {what}: {e}");
        exit(2)
    };
    std::fs::create_dir_all(dir).unwrap_or_else(|e| fail("the directory", e));
    let stem = file_stem(&side.name);
    // `dir/name` rather than Path::join, so the paths printed on Windows
    // don't mix separators.
    let base = match dir.trim_end_matches(['/', '\\']) {
        "" => dir,
        d => d,
    };
    let sep = if base.ends_with(['/', '\\']) { "" } else { "/" };
    saved
        .iter()
        .map(|rec| {
            let file = format!("{base}{sep}{stem}-{}-seed{}.json", side.label, rec.seed);
            std::fs::write(&file, rec.to_text()).unwrap_or_else(|e| fail(&file, e));
            file
        })
        .collect()
}

fn print_saved(sides: &[Summary]) {
    for s in sides {
        let Some(rep) = &s.saved else { continue };
        let mut why = Vec::new();
        if !rep.when.is_empty() {
            let capped = if rep.kept_by_when < rep.matched {
                format!(", first {} kept (--save-max {})", rep.kept_by_when, rep.max)
            } else {
                String::new()
            };
            why.push(format!(
                "--save-when {}: {} matched{capped}",
                rep.when.join(","),
                rep.matched
            ));
        }
        if !rep.seeds.is_empty() {
            why.push(format!("--save-seed {}", rep.seeds.join(",")));
        }
        if why.is_empty() {
            why.push("no --save-seed in the seeds played".to_string());
        }
        println!(
            "\nsaved {}: {} record(s) written ({})",
            s.label,
            rep.paths.len(),
            why.join("; ")
        );
        for p in &rep.paths {
            println!("  {p}");
        }
    }
}

/// Everything aggregated for one config.
#[derive(Serialize)]
struct Summary {
    label: String,
    path: String,
    overrides: Vec<String>,
    name: String,
    config_hash: String,
    content_hash: String,
    engine_version: String,
    #[serde(skip)]
    config: Config,
    games: u64,
    wins: u64,
    win_rate: f64,
    win_rate_se: f64,
    rounds_mean: f64,
    rounds_min: u32,
    rounds_max: u32,
    /// Games by end reason; `timeout` when the action cap hit first.
    ended: BTreeMap<String, u64>,
    /// Games by the round they ended in.
    end_round: BTreeMap<u32, u64>,
    decisions_mean: f64,
    options_per_decision: f64,
    forced_share: f64,
    max_options: u32,
    adversary_ops_per_round: f64,
    adversary_ops_max_round: u32,
    /// Adversary operations during setup, mean per game.
    setup_ops_per_game: f64,
    /// Mean per game, by counter name.
    counters: BTreeMap<String, f64>,
    replay_ok: u64,
    ms_per_game: f64,
    /// With `--save`: what was selected and where it went.
    #[serde(skip_serializing_if = "Option::is_none")]
    saved: Option<SaveReport>,
}

#[derive(Serialize)]
struct SaveReport {
    /// The `--save-when` selectors (empty when only `--save-seed` was given).
    when: Vec<String>,
    /// Games any `--save-when` selector matched.
    matched: u64,
    /// `--save-max`.
    max: usize,
    /// Of `matched`, the first `max` in seed order.
    kept_by_when: u64,
    /// `--save-seed` seeds within the range played.
    seeds: Vec<String>,
    /// The files written, one per record.
    paths: Vec<String>,
}

struct Played {
    record: GameRecord,
    replay_ok: bool,
    /// A `--save-when` selector matched.
    when: bool,
}

/// A game kept for `--save`, before the global `--save-max` cut.
struct Kept {
    by_when: bool,
    by_seed: bool,
    record: SavedGame,
}

fn simulate(
    label: String,
    path: &str,
    overrides: Vec<&String>,
    r: &Rules,
    o: &Options,
) -> (Summary, Vec<SavedGame>) {
    let seeds: Vec<u64> = (o.seed..=o.last_seed()).collect();
    // Replays go through rules rebuilt from the resolved config, as a
    // saved state would: proof that config + seed + actions is enough.
    let rebuilt = Rules::new(&r.config).expect("a resolved config is valid");
    let saving = o.save.is_some();
    // The note on every saved record: what was run, and why it was picked.
    let side = if o.command == "compare" {
        format!(" (config {label})")
    } else {
        String::new()
    };
    let note_head = format!("aic-sim {} {}{side}", o.command, o.paths.join(" "));
    let note_tail = if overrides.is_empty() {
        String::new()
    } else {
        format!(
            ", overrides: {}",
            overrides
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    };
    let start = Instant::now();
    let chunk = seeds.len().div_ceil(o.threads.max(1)).max(1);
    let (mut played, mut kept): (Vec<Played>, Vec<Kept>) = std::thread::scope(|s| {
        let handles: Vec<_> = seeds
            .chunks(chunk)
            .map(|part| {
                let rebuilt = &rebuilt;
                let (note_head, note_tail) = (&note_head, &note_tail);
                s.spawn(move || {
                    // Each chunk is a run of consecutive seeds, so the
                    // global first `save_max` matches are among each
                    // chunk's first `save_max`: keeping only those keeps
                    // the result independent of `--threads`.
                    let mut kept = Vec::new();
                    let mut when_seen = 0usize;
                    let played = part
                        .iter()
                        .map(|&seed| {
                            let (g, mut record) =
                                play(r, seed, &mut *o.agent.build(r, seed), o.max_actions);
                            let replay_ok =
                                replay(rebuilt, seed, &record.actions).is_ok_and(|h| h == g);
                            let w = if saving {
                                when_match(&o.save_when, record.outcome.as_ref())
                            } else {
                                None
                            };
                            let by_seed = saving && o.save_seeds.binary_search(&seed).is_ok();
                            let by_when = w.is_some() && when_seen < o.save_max;
                            when_seen += w.is_some() as usize;
                            if by_when || by_seed {
                                let why = match w {
                                    Some(w) if !by_seed => format!("--save-when {w}"),
                                    _ => "--save-seed".to_string(),
                                };
                                let note = format!(
                                    "{note_head}: {}, seed {seed}, picked by {why}{note_tail}",
                                    describe(record.outcome.as_ref(), record.actions.len()),
                                );
                                // The summary needs no actions; the record does.
                                let actions = std::mem::take(&mut record.actions);
                                kept.push(Kept {
                                    by_when: w.is_some(),
                                    by_seed,
                                    record: SavedGame::new(r, seed, actions, &g, &note),
                                });
                            }
                            Played {
                                record,
                                replay_ok,
                                when: w.is_some(),
                            }
                        })
                        .collect::<Vec<_>>();
                    (played, kept)
                })
            })
            .collect();
        let mut all = (Vec::new(), Vec::new());
        for h in handles {
            let (p, k) = h.join().expect("worker");
            all.0.extend(p);
            all.1.extend(k);
        }
        all
    });
    let elapsed = start.elapsed().as_secs_f64();
    played.sort_by_key(|p| p.record.seed);
    kept.sort_by_key(|k| k.record.seed);
    let mut rank = 0usize;
    let saved_games: Vec<SavedGame> = kept
        .into_iter()
        .filter(|k| {
            let keep = k.by_seed || (k.by_when && rank < o.save_max);
            rank += k.by_when as usize;
            keep
        })
        .map(|k| k.record)
        .collect();
    let matched = played.iter().filter(|p| p.when).count() as u64;
    // A reason code nobody ended with is most likely a typo: say so.
    let reasons: std::collections::BTreeSet<&str> = played
        .iter()
        .filter_map(|p| p.record.outcome.as_ref().map(|o| o.reason.as_str()))
        .collect();
    for w in &o.save_when {
        if !["all", "win", "loss", "timeout"].contains(&w.as_str()) && !reasons.contains(w.as_str())
        {
            let seen: Vec<&str> = reasons.iter().copied().collect();
            eprintln!(
                "warning: --save-when {w:?} matched no game ({label}); known: all, win, loss, timeout, {}",
                seen.join(", ")
            );
        }
    }
    let saved = saving.then(|| SaveReport {
        when: o.save_when.clone(),
        matched,
        max: o.save_max,
        kept_by_when: matched.min(o.save_max as u64),
        seeds: o
            .save_seeds
            .iter()
            .filter(|&&s| s >= o.seed && s <= o.last_seed())
            .map(|s| s.to_string())
            .collect(),
        paths: Vec::new(),
    });

    let n = played.len().max(1) as f64;
    let wins = played
        .iter()
        .filter(|p| p.record.outcome.as_ref().is_some_and(|o| o.won))
        .count() as u64;
    let p = wins as f64 / n;
    let mut ended = BTreeMap::new();
    let mut end_round = BTreeMap::new();
    let mut counters: BTreeMap<String, f64> = BTreeMap::new();
    let (mut rounds, mut decisions, mut options, mut forced, mut ops) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut rmin, mut rmax, mut max_options, mut ops_max) = (u32::MAX, 0u32, 0u32, 0u32);
    let mut setup_ops = 0u64;
    for g in &played {
        let rec = &g.record;
        let reason = rec
            .outcome
            .as_ref()
            .map_or("timeout".to_string(), |o| o.reason.clone());
        *ended.entry(reason).or_insert(0) += 1;
        *end_round.entry(rec.rounds).or_insert(0) += 1;
        rounds += rec.rounds as u64;
        rmin = rmin.min(rec.rounds);
        rmax = rmax.max(rec.rounds);
        decisions += rec.decisions as u64;
        options += rec.options_total;
        forced += rec.forced as u64;
        max_options = max_options.max(rec.max_options);
        ops += rec
            .stats
            .adversary_ops
            .iter()
            .map(|&x| x as u64)
            .sum::<u64>();
        ops_max = ops_max.max(rec.stats.adversary_ops.iter().copied().max().unwrap_or(0));
        setup_ops += rec.stats.setup_ops as u64;
        for (k, v) in &rec.stats.counters {
            *counters.entry(k.clone()).or_insert(0.0) += *v as f64;
        }
    }
    for v in counters.values_mut() {
        *v /= n;
    }
    let first = played.first().map(|p| &p.record);
    let summary = Summary {
        label,
        path: path.to_string(),
        overrides: overrides.into_iter().cloned().collect(),
        name: r.config.name.clone(),
        config_hash: r.config_hash.clone(),
        content_hash: first.map_or_else(content_hash, |f| f.content_hash.clone()),
        engine_version: aic_core::VERSION.to_string(),
        config: r.config.clone(),
        games: played.len() as u64,
        wins,
        win_rate: p,
        win_rate_se: (p * (1.0 - p) / n).sqrt(),
        rounds_mean: rounds as f64 / n,
        rounds_min: if played.is_empty() { 0 } else { rmin },
        rounds_max: rmax,
        ended,
        end_round,
        decisions_mean: decisions as f64 / n,
        options_per_decision: options as f64 / (decisions.max(1) as f64),
        forced_share: forced as f64 / (decisions.max(1) as f64),
        max_options,
        adversary_ops_per_round: ops as f64 / (rounds.max(1) as f64),
        adversary_ops_max_round: ops_max,
        setup_ops_per_game: setup_ops as f64 / n,
        counters,
        replay_ok: played.iter().filter(|p| p.replay_ok).count() as u64,
        ms_per_game: elapsed * 1000.0 / n,
        saved,
    };
    (summary, saved_games)
}

fn print_table(sides: &[Summary], diff: &[String], o: &Options) {
    let mut rows: Vec<(String, Vec<String>)> = Vec::new();
    let mut row = |name: &str, f: &dyn Fn(&Summary) -> String| {
        rows.push((name.to_string(), sides.iter().map(f).collect()))
    };
    row("config", &|s| s.path.clone());
    row("name", &|s| s.name.clone());
    row("config hash", &|s| s.config_hash.clone());
    row("games", &|s| s.games.to_string());
    row("win %", &|s| {
        format!("{:.1} ±{:.1}", 100.0 * s.win_rate, 100.0 * s.win_rate_se)
    });
    row("rounds mean", &|s| format!("{:.2}", s.rounds_mean));
    row("rounds min..max", &|s| {
        format!("{}..{}", s.rounds_min, s.rounds_max)
    });
    let mut reasons: Vec<&String> = sides.iter().flat_map(|s| s.ended.keys()).collect();
    reasons.sort();
    reasons.dedup();
    for r in reasons {
        row(&format!("ended: {r}"), &|s| {
            s.ended.get(r).copied().unwrap_or(0).to_string()
        });
    }
    let mut ends: Vec<u32> = sides
        .iter()
        .flat_map(|s| s.end_round.keys().copied())
        .collect();
    ends.sort();
    ends.dedup();
    for e in ends {
        row(&format!("ended in round {e}"), &|s| {
            s.end_round.get(&e).copied().unwrap_or(0).to_string()
        });
    }
    row("decisions/game", &|s| format!("{:.1}", s.decisions_mean));
    row("options/decision", &|s| {
        format!("{:.2}", s.options_per_decision)
    });
    row("forced decisions %", &|s| {
        format!("{:.1}", 100.0 * s.forced_share)
    });
    row("max options", &|s| s.max_options.to_string());
    row("adversary ops/round", &|s| {
        format!("{:.2}", s.adversary_ops_per_round)
    });
    row("adversary ops max/round", &|s| {
        s.adversary_ops_max_round.to_string()
    });
    row("adversary ops at setup", &|s| {
        format!("{:.2}", s.setup_ops_per_game)
    });
    let mut names: Vec<&String> = sides.iter().flat_map(|s| s.counters.keys()).collect();
    names.sort();
    names.dedup();
    for k in names {
        row(&format!("{k} /game"), &|s| {
            s.counters.get(k).map_or("-".into(), |v| format!("{v:.2}"))
        });
    }
    row("replay ok", &|s| format!("{}/{}", s.replay_ok, s.games));
    row("ms/game (incl. replay)", &|s| {
        format!("{:.3}", s.ms_per_game)
    });

    println!(
        "seeds {}..={}, agent {} (max {} actions), content {}, engine {} rules r{}",
        o.seed,
        o.last_seed(),
        o.agent.label(),
        o.max_actions,
        sides[0].content_hash,
        aic_core::VERSION,
        aic_core::RULES_REVISION
    );
    for s in sides {
        if !s.overrides.is_empty() {
            println!("{} overrides: {}", s.label, s.overrides.join(" "));
        }
    }
    if !diff.is_empty() {
        println!("A -> B: {}", diff.join("; "));
    }
    println!();
    let w0 = rows.iter().map(|r| r.0.chars().count()).max().unwrap_or(0);
    let widths: Vec<usize> = (0..sides.len())
        .map(|i| {
            rows.iter()
                .map(|r| r.1[i].chars().count())
                .max()
                .unwrap_or(0)
                .max(10)
        })
        .collect();
    let header: Vec<String> = sides
        .iter()
        .zip(&widths)
        .map(|(s, w)| format!("{:>w$}", s.label, w = w))
        .collect();
    println!("{:w0$}  {}", "", header.join("  "));
    for (name, vals) in &rows {
        let cells: Vec<String> = vals
            .iter()
            .zip(&widths)
            .map(|(v, w)| format!("{v:>w$}"))
            .collect();
        println!("{name:w0$}  {}", cells.join("  "));
    }
}

/// Prints one game as JSON lines: `{"action": ...}` then its events. To
/// stderr when `to_stderr` (with `--json`, so stdout stays one document).
fn trace(r: &Rules, agent: &AgentSpec, seed: u64, max_actions: usize, to_stderr: bool) {
    let out = |v: serde_json::Value| {
        if to_stderr {
            eprintln!("{v}")
        } else {
            println!("{v}")
        }
    };
    let mut events: Vec<Event> = Vec::new();
    let mut g = r.new_game_logged(seed, Some(&mut events));
    out(
        serde_json::json!({"trace": r.config.name, "seed": seed.to_string(), "config_hash": r.config_hash}),
    );
    let line = |e: &Event| out(serde_json::json!({ "event": e }));
    events.iter().for_each(line);
    let mut agent = agent.build(r, seed);
    for n in 0.. {
        let legal: Vec<Action> = r.legal_actions(&g);
        if legal.is_empty() {
            break;
        }
        // Same cap as driver::play, so the trace is the game the summary counts.
        if n == max_actions {
            out(serde_json::json!({ "timeout": true, "actions": n }));
            break;
        }
        let a = legal[aic_core::Agent::choose(&mut *agent, r, &g, &legal)].clone();
        out(serde_json::json!({ "action": a, "options": legal.len() }));
        events.clear();
        r.apply_mut(&mut g, &a, Some(&mut events)).expect("legal");
        events.iter().for_each(line);
    }
}

/// `aic-sim experiment`: plays the grid and writes the cycle report.
/// Returns the exit code: 1 if any game failed its replay check.
fn run_experiment(path: &str, o: &Options) -> i32 {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut spec: experiment::Spec =
        serde_json::from_str(&text).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    if o.games_given {
        spec.games = o.games;
    }
    if o.seed_given {
        spec.first_seed = o.seed;
    }
    if o.max_actions_given {
        spec.max_actions = o.max_actions;
    }
    let cells = experiment::cells(&spec).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let dir = o
        .out
        .clone()
        .unwrap_or_else(|| format!("out/experiments/{}", file_stem(&spec.name)));
    let dir = dir.trim_end_matches(['/', '\\']).to_string();
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| die(&format!("--out {dir}: {e}")));
    let total = cells.len() as u64 * spec.games;
    eprintln!(
        "{}: {} cells x {} seeds = {total} games on {} threads",
        spec.name,
        cells.len(),
        spec.games,
        o.threads
    );
    let start = Instant::now();
    let last = std::sync::Mutex::new(Instant::now());
    let progress = |done: usize, all: usize| {
        let mut l = last.lock().expect("progress lock");
        if done == all || l.elapsed().as_secs() >= 10 {
            *l = Instant::now();
            eprintln!(
                "  {done}/{all} games, {:.0} s",
                start.elapsed().as_secs_f64()
            );
        }
    };
    let games = experiment::run(&spec, &cells, o.threads, &progress);
    let wall_s = start.elapsed().as_secs_f64();
    let sums = report::summarize(&cells, &games);
    let groups = report::groups(&cells, &sums, &games);
    let cards = report::card_table(&cells, &games);
    let records = report::save_records(&dir, &spec, &cells, &games).unwrap_or_else(|e| {
        eprintln!("cannot write records: {e}");
        exit(2)
    });
    let run = report::Run {
        spec: &spec,
        cells: &cells,
        games: &games,
        sums: &sums,
        groups: &groups,
        cards: &cards,
        records: &records,
        wall_s,
        threads: o.threads,
        out_dir: &dir,
    };
    let write = |name: &str, body: String| {
        let f = format!("{dir}/{name}");
        std::fs::write(&f, body).unwrap_or_else(|e| {
            eprintln!("cannot write {f}: {e}");
            exit(2)
        });
    };
    write("report.md", report::markdown(&run));
    write("results.json", report::results_json(&run));
    write("games.csv", report::games_csv(&cells, &games));
    write("cards.csv", report::cards_csv(&cells, &games));
    println!(
        "{} games in {wall_s:.1} s; report {dir}/report.md, {} record(s) in {dir}/records",
        games.len(),
        records.len()
    );
    for s in &sums {
        println!(
            "  d{} {:24} win {:5.1}%  rounds {:5.2}  agent {:8.1} ms/game",
            s.difficulty,
            s.agent,
            100.0 * s.win_rate,
            s.rounds_mean,
            s.agent_ms_per_game
        );
    }
    let bad = games.iter().filter(|g| !g.replay_ok).count();
    if bad > 0 {
        eprintln!("REPLAY MISMATCH in {bad} game(s)");
        return 1;
    }
    0
}
