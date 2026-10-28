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

#[test]
fn experiment_writes_report_and_records() {
    let root = fresh_dir("aic-sim-experiment");
    let spec = root.join("spec.json");
    std::fs::write(&spec, SPEC).unwrap();
    let mut outs = Vec::new();
    for threads in ["1", "3"] {
        let out = root.join(format!("out-{threads}"));
        let res = sim()
            .arg("experiment")
            .arg(&spec)
            .arg("--out")
            .arg(&out)
            .args(["--threads", threads])
            .output()
            .expect("runs aic-sim");
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
        assert_eq!(results["cells"].as_array().unwrap().len(), 4);
        assert_eq!(results["format"], "aic-experiment/1");
        // 12 games, one row each.
        assert_eq!(
            std::fs::read_to_string(out.join("games.csv"))
                .unwrap()
                .lines()
                .count(),
            13
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
