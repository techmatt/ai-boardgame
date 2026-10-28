//! Runs the built binary's `experiment` command on a tiny grid: it writes
//! the report, the machine-readable results and records that replay, and
//! its results do not depend on the thread count.

use std::path::{Path, PathBuf};
use std::process::Command;

use aic_core::SavedGame;

fn sim() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_aic-sim"));
    c.current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
    c
}

fn fresh_dir(name: &str) -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

const SPEC: &str = r#"{
  "name": "test-grid",
  "configs": ["configs/baseline.json"],
  "difficulties": [0, 4],
  "agents": ["random", "greedy"],
  "games": 3,
  "winprob": {"games": 1, "rollouts": 2, "agent": "random"},
  "records": 2
}"#;

/// One CSV line's fields; quoted fields may hold commas and doubled quotes.
fn fields(line: &str) -> Vec<String> {
    let (mut out, mut cur, mut quoted) = (Vec::new(), String::new(), false);
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                cur.push('"');
                chars.next();
            }
            ('"', _) => quoted = !quoted,
            (',', false) => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// games.csv without the timing column, which varies run to run.
fn results_without_timing(dir: &Path) -> Vec<Vec<String>> {
    let text = std::fs::read_to_string(dir.join("games.csv")).unwrap();
    let head = fields(text.lines().next().unwrap());
    let ms = head.iter().position(|h| h == "ms").unwrap();
    text.lines()
        .skip(1)
        .map(|l| {
            let mut f = fields(l);
            assert_eq!(f.len(), head.len(), "{l}");
            f.remove(ms);
            f
        })
        .collect()
}

/// Fast lane: the grid with random only and a closeness probe every fourth
/// choice (2–4 s; greedy games are seconds each even optimized).
#[test]
fn experiment_writes_report_and_records_random() {
    let body = SPEC
        .replace(r#"["random", "greedy"]"#, r#"["random"]"#)
        .replace(r#""records": 2"#, r#""closeness": {"every": 4}, "records": 2"#);
    writes_report_and_records("aic-sim-experiment-random", &body, 2);
}

/// Slow lane: the whole grid, greedy included, every choice probed.
#[test]
#[ignore = "slow lane: greedy experiment games, twice"]
fn experiment_writes_report_and_records() {
    writes_report_and_records("aic-sim-experiment", SPEC, 4);
}

fn writes_report_and_records(dir: &str, body: &str, cells: usize) {
    let root = fresh_dir(dir);
    let spec = root.join("spec.json");
    std::fs::write(&spec, body).unwrap();
    // Both runs at once: the one-thread run is the long one.
    let runs = std::thread::scope(|s| {
        ["1", "3"]
            .map(|threads| {
                let out = root.join(format!("out-{threads}"));
                let spec = &spec;
                s.spawn(move || {
                    let res = sim()
                        .arg("experiment")
                        .arg(spec)
                        .arg("--out")
                        .arg(&out)
                        .args(["--threads", threads])
                        .output()
                        .expect("runs aic-sim");
                    (out, res)
                })
            })
            .map(|h| h.join().unwrap())
    });
    let mut outs = Vec::new();
    for (out, res) in runs {
        assert!(
            res.status.success(),
            "experiment failed: {}",
            String::from_utf8_lossy(&res.stderr)
        );
        for f in ["report.md", "results.json", "games.csv", "cards.csv"] {
            assert!(out.join(f).exists(), "{f} missing");
        }
        let report = std::fs::read_to_string(out.join("report.md")).unwrap();
        for section in [
            "## Findings",
            "## Win rate by skill tier",
            "## Luck share",
            "## Pick and play rates",
            "## Timing",
            "## Saved records",
        ] {
            assert!(report.contains(section), "report lacks {section}");
        }
        let results: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(out.join("results.json")).unwrap())
                .unwrap();
        assert_eq!(results["cells"].as_array().unwrap().len(), cells);
        assert_eq!(results["format"], "aic-experiment/1");
        // 3 games per cell, one row each.
        assert_eq!(
            std::fs::read_to_string(out.join("games.csv"))
                .unwrap()
                .lines()
                .count(),
            3 * cells + 1
        );
        let records: Vec<PathBuf> = std::fs::read_dir(out.join("records"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        assert!(!records.is_empty(), "no records saved");
        for p in &records {
            let rec = SavedGame::from_json(&std::fs::read_to_string(p).unwrap()).unwrap();
            let (_, warnings) = rec.replay().unwrap();
            assert!(warnings.is_empty(), "{}: {warnings:?}", p.display());
            assert!(rec.note.contains("aic-sim experiment test-grid"));
        }
        outs.push(out);
    }
    assert_eq!(
        results_without_timing(&outs[0]),
        results_without_timing(&outs[1])
    );
}

const VARIANT_SPEC: &str = r#"{
  "name": "test-variants",
  "configs": ["configs/baseline.json"],
  "variants": [
    {"name": "base"},
    {"name": "threats_off", "set": ["threats.variant=off"]}
  ],
  "difficulties": [0],
  "agents": ["random", "greedy"],
  "games": 3,
  "closeness": {"every": 0},
  "records": 0
}"#;

/// Two agents and two variants, one switching a module off: the
/// comparison sections appear, with their machine-readable twins.
#[test]
fn experiment_reports_comparisons() {
    let root = fresh_dir("aic-sim-experiment-variants");
    let spec = root.join("spec.json");
    std::fs::write(&spec, VARIANT_SPEC).unwrap();
    let out = root.join("out");
    let res = sim()
        .arg("experiment")
        .arg(&spec)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("runs aic-sim");
    assert!(
        res.status.success(),
        "experiment failed: {}",
        String::from_utf8_lossy(&res.stderr)
    );
    let report = std::fs::read_to_string(out.join("report.md")).unwrap();
    for section in [
        "## Strategy health",
        "## Head to head against greedy",
        "## Variants against the base",
        "## Removing a module",
        "Strategy health at difficulty 0",
        // greedy wins most here, so it is the top agent itself.
        "Top agent (greedy",
        "What a module is worth",
        "| threats (threats_off) |",
    ] {
        assert!(report.contains(section), "report lacks {section}\n{report}");
    }
    let results: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("results.json")).unwrap()).unwrap();
    // random against greedy, in each of the two variants.
    let h2h = results["head_to_head"].as_array().unwrap();
    assert_eq!(h2h.len(), 2);
    for h in h2h {
        assert_eq!(h["agent"], "random");
        assert_eq!(h["greedy"], "greedy");
        assert_eq!(h["paired"]["games"], 3);
    }
    assert_eq!(results["variants"].as_array().unwrap().len(), 2);
    let removals = results["removals"].as_array().unwrap();
    assert_eq!(removals.len(), 2);
    assert_eq!(removals[0]["modules"][0], "threats");
    assert_eq!(results["top_agent"]["agent"], "greedy");
    for c in results["cells"].as_array().unwrap() {
        assert!(c["buildings_played_per_game"].as_f64().unwrap() >= 0.0);
        assert!(c["tier2_played_per_game"].is_number());
    }
}

/// One agent and one variant: no comparison sections.
#[test]
fn experiment_omits_comparisons_that_do_not_apply() {
    let root = fresh_dir("aic-sim-experiment-single");
    let spec = root.join("spec.json");
    let body = VARIANT_SPEC
        .replace(
            r#"{"name": "threats_off", "set": ["threats.variant=off"]}"#,
            r#"{"name": "threats_on", "set": []}"#,
        )
        .replace(r#""random", "greedy""#, r#""random""#);
    std::fs::write(&spec, body).unwrap();
    let out = root.join("out");
    let res = sim()
        .arg("experiment")
        .arg(&spec)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("runs aic-sim");
    assert!(res.status.success(), "{}", String::from_utf8_lossy(&res.stderr));
    let report = std::fs::read_to_string(out.join("report.md")).unwrap();
    assert!(report.contains("## Strategy health"));
    assert!(report.contains("## Variants against the base"));
    assert!(!report.contains("## Head to head against greedy"));
    assert!(!report.contains("## Removing a module"));
}

#[test]
fn experiment_rejects_bad_specs() {
    let root = fresh_dir("aic-sim-experiment-bad");
    for (name, body) in [
        ("agent", SPEC.replace("\"greedy\"", "\"greedy:nope=1\"")),
        ("field", SPEC.replace("\"records\": 2", "\"recordz\": 2")),
        ("config", SPEC.replace("baseline.json", "missing.json")),
    ] {
        let spec = root.join(format!("{name}.json"));
        std::fs::write(&spec, body).unwrap();
        let res = sim()
            .arg("experiment")
            .arg(&spec)
            .arg("--out")
            .arg(root.join("out"))
            .output()
            .unwrap();
        assert_eq!(
            res.status.code(),
            Some(2),
            "{name}: {}",
            String::from_utf8_lossy(&res.stderr)
        );
    }
}
