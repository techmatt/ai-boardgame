//! The command line: usage text, options and their checks, and loading a
//! config with its overrides.

use std::process::exit;

use aic_core::{Config, Rules};

use crate::agents::AgentSpec;

pub const USAGE: &str = "\
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
                    code such as supply or unity (repeatable; default loss, or none
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
                    cards.csv, records/. --games and --seed override the spec.
  Expensive measures are opt-in (off unless the spec or these turn them on):
    --winprob N     win-probability playouts for the first N seeds of each cell
                    (about 5x an experiment's time at N = 10 of 200)
    --closeness N   closeness probe on every N-th real choice
    --variants A,B  play only these of the spec's variants (the base is always
                    kept, first): screen without probes, then probe the finalists
  Scoping runs, results as they go:
    --quick         the cheap preset: shallow agents only (deep-search agents are
                    dropped), at most 16 seeds per cell unless --games is given, no
                    probes unless --winprob or --closeness is given
    --report-every S  rewrite the report from the games finished so far every S
                    seconds (default 60; 0 = only at the end). Every game is also
                    appended to DIR/games.jsonl as it finishes
    --resume        continue a stopped run of the same spec from DIR/games.jsonl:
                    play only the games it lacks, then report them all";

pub struct Options {
    pub command: String,
    pub paths: Vec<String>,
    pub games: u64,
    pub seed: u64,
    pub set: Vec<String>,
    pub set_a: Vec<String>,
    pub set_b: Vec<String>,
    pub max_actions: usize,
    pub threads: usize,
    pub json: bool,
    pub trace: bool,
    pub save: Option<String>,
    pub save_when: Vec<String>,
    pub save_seeds: Vec<u64>,
    pub save_max: usize,
    pub agent: AgentSpec,
    /// `experiment`: output directory, and whether --games/--seed were given.
    pub out: Option<String>,
    pub games_given: bool,
    pub seed_given: bool,
    pub max_actions_given: bool,
    /// `experiment`: probe overrides and a variant filter (two-stage runs).
    pub winprob: Option<u64>,
    pub closeness: Option<usize>,
    pub variants: Option<Vec<String>>,
    /// `experiment`: the scoping preset, continuing a stopped run, and how
    /// often (seconds) the report of the games so far is rewritten.
    pub quick: bool,
    pub resume: bool,
    pub report_every: u64,
}

pub fn die(msg: &str) -> ! {
    eprintln!("{msg}\n\n{USAGE}");
    exit(2)
}

pub fn parse_args() -> Options {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| die("missing command"));
    // 020: `aic-sim --help` said "unknown command" and exited 2.
    if command == "-h" || command == "--help" {
        println!("{USAGE}");
        exit(0);
    }
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
        winprob: None,
        closeness: None,
        variants: None,
        quick: false,
        resume: false,
        report_every: 60,
    };
    let mut save_max_given = false;
    let mut report_every_given = false;
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
            "--winprob" => o.winprob = Some(num("--winprob", value("--winprob"))),
            "--closeness" => o.closeness = Some(num("--closeness", value("--closeness"))),
            "--variants" => {
                o.variants = Some(
                    value("--variants")
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                )
            }
            "--quick" => o.quick = true,
            "--resume" => o.resume = true,
            "--report-every" => {
                o.report_every = num("--report-every", value("--report-every"));
                report_every_given = true;
            }
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
    // Commands that would silently drop an option say so instead (020: `show --set-a`
    // printed the unmodified config; `replay --json` printed text).
    let refuse = |what: &str| die(&format!("{} does not take {what}", o.command));
    if o.command == "show" && (!o.set_a.is_empty() || !o.set_b.is_empty()) {
        refuse("--set-a / --set-b (one config: use --set)");
    }
    if (o.command == "replay" || o.command == "modules")
        && (!o.set.is_empty() || !o.set_a.is_empty() || !o.set_b.is_empty())
    {
        refuse("--set (a record carries its own config)");
    }
    if (o.command == "replay" || o.command == "modules") && o.json {
        refuse("--json");
    }
    if (o.winprob.is_some() || o.closeness.is_some() || o.variants.is_some())
        && o.command != "experiment"
    {
        die("--winprob, --closeness and --variants need experiment");
    }
    if o.out.is_some() && o.command != "experiment" {
        die("--out needs experiment");
    }
    if (o.quick || o.resume || report_every_given) && o.command != "experiment" {
        die("--quick, --resume and --report-every need experiment");
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
    pub fn last_seed(&self) -> u64 {
        self.seed + (self.games - 1)
    }
}

pub fn load(path: &str, overrides: &[&String]) -> Config {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut c = Config::from_json(&text).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    for o in overrides {
        c.apply_override(o)
            .unwrap_or_else(|e| die(&format!("{path}: {e}")));
    }
    c
}

pub fn rules(path: &str, c: &Config) -> Rules {
    Rules::new(c).unwrap_or_else(|e| die(&format!("{path}: invalid config: {e}")))
}
