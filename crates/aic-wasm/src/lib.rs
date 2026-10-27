//! Thin wasm-bindgen layer over `aic-core`. No rules live here.
//!
//! Configs, states, actions and events cross the boundary as JSON strings.
//! Seeds are decimal strings. `apply` returns an [`Outcome`] whose `state`
//! and `events` are separate JSON strings. Errors (bad JSON, bad seed, an
//! invalid config, an illegal action) are thrown as JavaScript strings.

use aic_core::{Action, Config, GameState, Rules};
use serde::Serialize;
use wasm_bindgen::prelude::*;

/// The result of `apply`: `state` (JSON string, pass it back as-is) and
/// `events` (JSON array string).
#[wasm_bindgen(getter_with_clone)]
pub struct Outcome {
    pub state: String,
    pub events: String,
}

fn to_json<T: Serialize + ?Sized>(value: &T) -> String {
    serde_json::to_string(value).expect("engine types serialize")
}

fn parse<T: for<'de> serde::Deserialize<'de>>(what: &str, json: &str) -> Result<T, JsValue> {
    serde_json::from_str(json).map_err(|e| JsValue::from_str(&format!("bad {what} JSON: {e}")))
}

fn rules(config_json: &str) -> Result<Rules, JsValue> {
    let config: Config = parse("config", config_json)?;
    Rules::new(&config).map_err(|e| JsValue::from_str(&e))
}

fn err(e: String) -> JsValue {
    JsValue::from_str(&e)
}

#[wasm_bindgen]
pub fn version() -> String {
    aic_core::VERSION.to_string()
}

/// All content (map pieces and layouts, cards) as JSON.
#[wasm_bindgen]
pub fn content() -> String {
    to_json(aic_core::content())
}

/// The module registry: modules, variants, parameters, phases.
#[wasm_bindgen]
pub fn modules() -> String {
    to_json(&aic_core::describe_modules())
}

/// Validates a config. Returns `{"config": <resolved>, "hash": "<16 hex>"}`.
#[wasm_bindgen]
pub fn resolve_config(config_json: &str) -> Result<String, JsValue> {
    let r = rules(config_json)?;
    Ok(to_json(
        &serde_json::json!({ "config": r.config, "hash": r.config_hash }),
    ))
}

/// The board for a config: hexes (axial `[q, r]`, country, neighbours) and
/// countries (hex indices, card slots, colour). `state.hexes[i]` is `hexes[i]`.
#[wasm_bindgen]
pub fn board(config_json: &str) -> Result<String, JsValue> {
    Ok(to_json(&rules(config_json)?.map))
}

/// Starts a game. `seed` is a decimal string (any `u64`).
#[wasm_bindgen]
pub fn new_game(config_json: &str, seed: &str) -> Result<String, JsValue> {
    let seed: u64 = seed.trim().parse().map_err(|_| {
        JsValue::from_str(&format!("bad seed {seed:?}: expected a decimal u64 string"))
    })?;
    Ok(to_json(&rules(config_json)?.new_game(seed)))
}

/// Legal actions for a state, as a JSON array.
#[wasm_bindgen]
pub fn legal_actions(state_json: &str) -> Result<String, JsValue> {
    let state: GameState = parse("state", state_json)?;
    Ok(to_json(&aic_core::legal_actions(&state).map_err(err)?))
}

/// Applies an action. Returns an `Outcome` (`.state`, `.events`).
#[wasm_bindgen]
pub fn apply(state_json: &str, action_json: &str) -> Result<Outcome, JsValue> {
    let state: GameState = parse("state", state_json)?;
    let action: Action = parse("action", action_json)?;
    let (state, events) = aic_core::apply(&state, &action).map_err(err)?;
    Ok(Outcome {
        state: to_json(&state),
        events: to_json(&events),
    })
}
