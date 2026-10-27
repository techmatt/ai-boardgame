//! Typed events emitted while applying actions: the event log, for UIs to
//! animate and for traces.

use serde::{Deserialize, Serialize};

use crate::hex::Hex;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    RoundStarted {
        round: u32,
    },
    /// `phase` is `"<module>.<phase>"`.
    PhaseStarted {
        phase: String,
    },
    ResourceChanged {
        seat: u8,
        resource: String,
        delta: i64,
        total: i64,
    },
    TokensChanged {
        hex: Hex,
        token: String,
        delta: i64,
        total: i64,
    },
    /// A module track (`GameState::vars`) changed.
    VarChanged {
        key: String,
        delta: i64,
        total: i64,
    },
    CardPlayed {
        seat: u8,
        card: String,
        target: Option<Hex>,
    },
    Passed {
        seat: u8,
    },
    GameEnded {
        won: bool,
        round: u32,
        reason: String,
        detail: String,
    },
}
