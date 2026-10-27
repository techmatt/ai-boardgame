//! Every module the engine knows. To add one: write `modules/<id>.rs` with a
//! `pub static DEF: ModuleDef`, add it to [`REGISTRY`], and document it in
//! `docs/rules.md` and `docs/architecture.md`.
//!
//! The current modules are placeholders that exercise the framework; none
//! of them is a real rule of AI Complete.

use crate::module::ModuleDef;

pub mod actions;
pub mod income;
pub mod spread;
pub mod victory;

pub static REGISTRY: &[&ModuleDef] = &[
    &income::DEF,
    &spread::DEF,
    &actions::DEF,
    &victory::DEF,
    #[cfg(test)]
    &actions::TEST_TWIN,
];

/// Resource every placeholder module trades in.
pub const CREDITS: &str = "credits";
/// The placeholder adversary's token.
pub const BLIGHT: &str = "blight";

pub fn find(id: &str) -> Option<&'static ModuleDef> {
    REGISTRY.iter().copied().find(|m| m.id == id)
}

/// Total `kind` tokens on a country's hexes.
pub(crate) fn country_tokens(
    g: &crate::GameState,
    w: &crate::module::World,
    country: usize,
    kind: &str,
) -> i64 {
    w.map.countries[country]
        .hexes
        .iter()
        .map(|&h| g.tokens(h, kind))
        .sum()
}
