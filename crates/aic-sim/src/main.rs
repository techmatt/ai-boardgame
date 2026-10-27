//! Simulation CLI: plays seeded random-agent games for one or two configs,
//! prints win rate, game length and counters side by side, and replay-checks
//! every game. See README.md for usage.

mod agents;

use std::collections::BTreeMap;
use std::process::exit;
use std::time::Instant;

use agents::RandomAgent;
use aic_core::{content_hash, play, replay, Action, Config, Event, GameRecord, Rules};
use serde::Serialize;

const USAGE: &str = "\
usage: aic-sim run <config.json> [options]
       aic-sim compare <a.json> <b.json> [options]
       aic-sim show <config.json> [--set k=v]...     resolved config + hash
       aic-sim modules                               module registry as JSON

options:
  --games N         games per config (default 200)
  --seed S          first seed (default 0); seeds S..S+N
  --set K=V         override for every config: countries=N, <module>.<param>=V,
                    <module>.variant=NAME; V is 3 or 1,2,3 (repeatable)
  --set-a K=V       override for config A only (repeatable)
  --set-b K=V       override for config B only (repeatable)
  --max-actions N   per-game action cap; a capped game is a timeout (default 100000)
  --threads N       worker threads (default: all cores); results do not depend on it
  --json            print the summary as JSON instead of a table
  --trace           print the first game of each config as JSON lines (actions, events)";

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
    };
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
            "--games" => o.games = num("--games", value("--games")),
            "--seed" => o.seed = num("--seed", value("--seed")),
            "--set" => o.set.push(value("--set")),
            "--set-a" => o.set_a.push(value("--set-a")),
            "--set-b" => o.set_b.push(value("--set-b")),
            "--max-actions" => o.max_actions = num("--max-actions", value("--max-actions")),
            "--threads" => o.threads = num::<usize>("--threads", value("--threads")).max(1),
            "--json" => o.json = true,
            "--trace" => o.trace = true,
            "-h" | "--help" => {
                println!("{USAGE}");
                exit(0)
            }
            s if s.starts_with("--") => die(&format!("unknown option {s:?}")),
            _ => o.paths.push(a),
        }
    }
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
            for (i, path) in o.paths.iter().enumerate() {
                let own = if i == 0 { &o.set_a } else { &o.set_b };
                let overrides: Vec<&String> = o.set.iter().chain(own).collect();
                let config = load(path, &overrides);
                let r = rules(path, &config);
                if o.trace {
                    trace(&r, o.seed, o.max_actions, o.json);
                }
                let label = if want == 1 {
                    "run".to_string()
                } else {
                    ["A", "B"][i].to_string()
                };
                sides.push(simulate(label, path, overrides, &r, &o));
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
                    "agent": "random",
                    "max_actions": o.max_actions,
                    "rules_revision": aic_core::RULES_REVISION,
                    "diff": diff,
                    "results": sides,
                });
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                print_table(&sides, &diff, &o);
            }
            let bad: u64 = sides.iter().map(|s| s.games - s.replay_ok).sum();
            if bad > 0 {
                eprintln!("REPLAY MISMATCH in {bad} game(s)");
                exit(1);
            }
        }
        other => die(&format!("unknown command {other:?}")),
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
}

struct Played {
    record: GameRecord,
    replay_ok: bool,
}

fn simulate(label: String, path: &str, overrides: Vec<&String>, r: &Rules, o: &Options) -> Summary {
    let seeds: Vec<u64> = (o.seed..=o.last_seed()).collect();
    // Replays go through rules rebuilt from the resolved config, as a
    // saved state would: proof that config + seed + actions is enough.
    let rebuilt = Rules::new(&r.config).expect("a resolved config is valid");
    let start = Instant::now();
    let chunk = seeds.len().div_ceil(o.threads.max(1)).max(1);
    let mut played: Vec<Played> = std::thread::scope(|s| {
        let handles: Vec<_> = seeds
            .chunks(chunk)
            .map(|part| {
                let rebuilt = &rebuilt;
                s.spawn(move || {
                    part.iter()
                        .map(|&seed| {
                            let (g, record) =
                                play(r, seed, &mut RandomAgent::new(seed), o.max_actions);
                            let replay_ok =
                                replay(rebuilt, seed, &record.actions).is_ok_and(|h| h == g);
                            Played { record, replay_ok }
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("worker"))
            .collect()
    });
    let elapsed = start.elapsed().as_secs_f64();
    played.sort_by_key(|p| p.record.seed);

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
    Summary {
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
    }
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
        "seeds {}..={}, random agent (max {} actions), content {}, engine {} rules r{}",
        o.seed,
        o.last_seed(),
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
fn trace(r: &Rules, seed: u64, max_actions: usize, to_stderr: bool) {
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
    let mut agent = RandomAgent::new(seed);
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
        let a = legal[aic_core::Agent::choose(&mut agent, r, &g, &legal)].clone();
        out(serde_json::json!({ "action": a, "options": legal.len() }));
        events.clear();
        r.apply_mut(&mut g, &a, Some(&mut events)).expect("legal");
        events.iter().for_each(line);
    }
}
