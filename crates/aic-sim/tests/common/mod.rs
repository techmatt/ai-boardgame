//! Helpers shared by the integration tests.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The built binary, run from the workspace root (configs/ is relative to it).
pub fn sim() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_aic-sim"));
    c.current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
    c
}

/// `name` under the test temp dir, emptied of an earlier run; `create`
/// makes it again, empty, or leaves it absent for the binary to make.
pub fn fresh_dir(name: &str, create: bool) -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&d);
    if create {
        std::fs::create_dir_all(&d).unwrap();
    }
    d
}
