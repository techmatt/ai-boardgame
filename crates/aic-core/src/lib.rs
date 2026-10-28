//! AI Complete rules engine.
//!
//! Pure and deterministic: no I/O, no wall clock, no global or thread RNG,
//! and no hash-order leaks (every map is a `BTreeMap`). All randomness comes
//! from seeded per-purpose streams held in [`GameState`]; the state is
//! serde-serializable and a game replays exactly from config + seed +
//! action list.
//!
//! A game is built from a [`Config`]: modules, each in one variant with
//! named integer parameters. [`Rules::new`] compiles a config once; its
//! `new_game`, `legal_actions_into` and `apply_mut` are the hot-loop path.
//! The free functions below are the convenient (slower) path that rebuilds
//! the rules from the config embedded in the state on every call.
//!
//! What the rules are is in `docs/rules.md`; how modules, configs and
//! tunables fit together is in `docs/architecture.md`.

pub mod action;
pub mod config;
pub mod content;
pub mod driver;
pub mod event;
mod hash;
pub mod hex;
pub mod map;
pub mod module;
pub mod modules;
pub mod record;
pub mod rng;
pub mod rules;
pub mod state;
pub mod view;

pub use action::Action;
pub use config::{Config, ModuleConfig, ParamValue};
pub use content::{content, content_hash, Content};
pub use driver::{play, replay, Agent, GameRecord};
pub use event::Event;
pub use hash::fnv1a64;
pub use hex::{Dir, Hex};
pub use map::Map;
pub use record::{SavedGame, RECORD_FORMAT};
pub use rules::Rules;
pub use state::{GameState, Outcome, Seat, Stats};
pub use view::View;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Bump on **any** change to how the rules play out (module behaviour,
/// engine loop, RNG use), so results from different rules are never
/// mistaken for the same experiment. Recorded with every result. The
/// `golden_game` test fails until it is bumped and the golden fingerprint
/// updated (also in `scripts/wasm-smoke.mjs`).
pub const RULES_REVISION: u32 = 5;

/// Starts a game: setup, then on to the first decision.
pub fn new_game(config: &Config, seed: u64) -> Result<GameState, String> {
    Ok(Rules::new(config)?.new_game(seed))
}

/// Legal actions in `state` (empty once the game is over).
pub fn legal_actions(state: &GameState) -> Result<Vec<Action>, String> {
    let rules = Rules::new(&state.config)?;
    rules.check_state(state)?;
    Ok(rules.legal_actions(state))
}

/// Applies an action, returning the next state and the events it produced.
pub fn apply(state: &GameState, action: &Action) -> Result<(GameState, Vec<Event>), String> {
    let rules = Rules::new(&state.config)?;
    rules.check_state(state)?;
    rules.apply(state, action)
}

/// The generic view of a state (see `view.rs`), for clients.
pub fn view(state: &GameState) -> Result<View, String> {
    let rules = Rules::new(&state.config)?;
    rules.check_state(state)?;
    Ok(rules.view(state))
}

/// The module registry as JSON-friendly data: every module, its variants,
/// their parameters and phases. For UIs, docs and the sim's `modules` command.
pub fn describe_modules() -> serde_json::Value {
    use serde_json::json;
    let modules: Vec<_> = modules::REGISTRY
        .iter()
        .map(|m| {
            let variants: Vec<_> = m
                .variants
                .iter()
                .map(|v| {
                    let params: Vec<_> = v
                        .params
                        .iter()
                        .map(|p| {
                            let kind = match p.kind {
                                module::ParamKind::Int => "int",
                                module::ParamKind::List => "list",
                            };
                            json!({"name": p.name, "kind": kind, "min": p.min, "max": p.max, "doc": p.doc})
                        })
                        .collect();
                    let phases: Vec<_> = v
                        .phases
                        .iter()
                        .map(|p| {
                            let kind = match p.kind {
                                module::PhaseKind::Auto => "auto",
                                module::PhaseKind::Decision => "decision",
                            };
                            json!({"name": p.name, "kind": kind, "doc": p.doc})
                        })
                        .collect();
                    json!({
                        "name": v.name, "summary": v.summary, "params": params, "phases": phases,
                        "rng": v.rng, "uses_cards": v.uses_cards,
                    })
                })
                .collect();
            json!({"id": m.id, "summary": m.summary, "variants": variants})
        })
        .collect();
    json!(modules)
}

#[cfg(test)]
mod tests;
