//! The generated docs are current: `aic-sim docs --check`, and docs/cards.md
//! against content (its generator is a Python script, so it is checked here
//! by what it must hold).

mod common;

use std::fs;
use std::path::Path;

use aic_core::content::content;

#[test]
fn generated_docs_are_current() {
    let out = common::sim().args(["docs", "--check"]).output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn the_card_list_names_every_card_and_this_revision() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/cards.md");
    let doc = fs::read_to_string(path).unwrap();
    assert!(
        doc.contains(&format!("Rules revision {}", aic_core::RULES_REVISION)),
        "docs/cards.md is stale: run python scripts/cards/cards_doc.py"
    );
    for c in &content().cards {
        assert!(doc.contains(&c.name), "docs/cards.md lacks {}", c.name);
    }
}
