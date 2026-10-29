//! Simulation CLI: plays seeded games for one or two configs with a
//! selectable team agent (random by default), prints win rate, game length
//! and counters side by side, and replay-checks every game; runs experiment
//! grids into a cycle report. See README.md for usage.

mod agents;
mod cli;
mod experiment;
mod experiment_run;
mod prof;
mod report;
mod sweep;

use std::process::exit;

use aic_core::content_hash;
use cli::{die, load, parse_args, rules, Options};

/// The search clones game states by the hundred thousand, and the system
/// allocator serializes them across workers: on 12 threads a state clone
/// took 134 us against 24 us on one (012). mimalloc keeps a heap per thread.
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let o = parse_args();
    match o.command.as_str() {
        "modules" => println!(
            "{}",
            serde_json::to_string_pretty(&aic_core::describe_modules()).unwrap()
        ),
        "show" => show(&o),
        "run" | "compare" => sweep::run(&o),
        "replay" => sweep::replay_all(&o),
        "experiment" => {
            let [path] = o.paths.as_slice() else {
                die("experiment takes one spec file")
            };
            exit(experiment_run::run_experiment(path, &o));
        }
        other => die(&format!("unknown command {other:?}")),
    }
}

/// `aic-sim show`: the resolved config, its hashes and the tunables.
fn show(o: &Options) {
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
