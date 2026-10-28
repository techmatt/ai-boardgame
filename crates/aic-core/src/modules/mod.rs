//! Every module the engine knows: the v0 ruleset, one module per rules
//! area. To add one: write `modules/<id>.rs` with a `pub static DEF:
//! ModuleDef`, add it to [`REGISTRY`], and document it in `docs/rules.md`
//! and `docs/architecture.md`.
//!
//! The rules areas lean on each other (a card's damage destroys adversary
//! pieces and scores opposition), so modules call each other's public
//! functions and read each other's parameters; each declares the modules it
//! needs in `ModuleDef::requires`. `common` and `effects` are shared code,
//! not modules.

use crate::module::ModuleDef;

pub mod adversary;
pub mod ai;
pub mod board;
pub mod cards;
pub mod common;
pub mod economy;
pub mod effects;
pub mod opposition;
pub mod threats;

pub static REGISTRY: &[&ModuleDef] = &[
    &board::DEF,
    &adversary::DEF,
    &threats::DEF,
    &opposition::DEF,
    &economy::DEF,
    &ai::DEF,
    &cards::DEF,
];

pub fn find(id: &str) -> Option<&'static ModuleDef> {
    REGISTRY.iter().copied().find(|m| m.id == id)
}
