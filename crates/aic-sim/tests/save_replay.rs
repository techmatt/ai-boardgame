//! Runs the built binary: `run --save` writes records that parse, replay
//! cleanly and pass `aic-sim replay`; a tampered record fails it.

use std::path::{Path, PathBuf};
use std::process::Command;

use aic_core::SavedGame;

fn sim() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_aic-sim"));
    // configs/ is relative to the workspace root.
    c.current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
    c
}

fn fresh_dir(name: &str) -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&d);
    d
}

#[test]
fn save_then_replay() {
    let dir = fresh_dir("aic-sim-save");
    let out = sim()
        .args(["run", "configs/baseline.json", "--games", "30", "--save"])
        .arg(&dir)
        .args(["--save-when", "all", "--save-max", "5"])
        .output()
        .expect("runs aic-sim");
    assert!(
        out.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("30 matched"), "{stdout}");

    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    files.sort();
    assert_eq!(files.len(), 5, "{files:?}");
    for (f, seed) in files.iter().zip(["0", "1", "2", "3", "4"]) {
        let name = f.file_name().unwrap().to_string_lossy();
        assert_eq!(name, format!("baseline-run-seed{seed}.json"));
        let rec = SavedGame::from_json(&std::fs::read_to_string(f).unwrap()).unwrap();
        let (g, warnings) = rec.replay().unwrap();
        assert!(warnings.is_empty(), "{name}: {warnings:?}");
        assert_eq!(g.fingerprint(), rec.final_fingerprint);
        assert!(rec.note.contains("seed"), "{}", rec.note);
    }

    let ok = sim().arg("replay").args(&files).output().unwrap();
    assert!(
        ok.status.success(),
        "replay failed: {}",
        String::from_utf8_lossy(&ok.stdout)
    );

    // Edit the saved final fingerprint: the replay still runs but warns.
    let text = std::fs::read_to_string(&files[0]).unwrap();
    let rec = SavedGame::from_json(&text).unwrap();
    let flipped: String = rec
        .final_fingerprint
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect();
    let tampered = text.replace(&rec.final_fingerprint, &flipped);
    assert_ne!(tampered, text);
    let bad = dir.join("tampered.json");
    std::fs::write(&bad, tampered).unwrap();
    let out = sim()
        .arg("replay")
        .arg(&files[1])
        .arg(&bad)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stdout).contains("WARNING final state fingerprint"));
}
