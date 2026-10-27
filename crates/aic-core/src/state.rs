//! The serializable game state. Everything the rules read is something a
//! physical copy could hold: tokens on hexes, resources on seat boards,
//! exhausted cards, card slots, a round and phase marker, numbered tracks.
//! Two parts are not game components: `rng` (the dice) and `stats`
//! (observation for the simulator; rules never read it). Geometry lives in
//! [`crate::Map`].

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::rng::{as_string, RngStreams};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    /// Decimal string in JSON.
    #[serde(with = "as_string")]
    pub seed: u64,
    /// The resolved config this game was built from (round filled in), so a
    /// state is self-describing and replayable.
    pub config: Config,
    /// 1-based once play starts.
    pub round: u32,
    /// Index into `config.round` of the current phase.
    pub phase: usize,
    pub seats: Vec<Seat>,
    /// Per-hex contents, aligned with `Map::hexes`.
    pub hexes: Vec<HexState>,
    /// Global tracks owned by modules, keyed `"<module>.<name>"`.
    pub vars: BTreeMap<String, i64>,
    pub rng: RngStreams,
    pub stats: Stats,
    pub outcome: Option<Outcome>,
}

/// One seat = one country = one megacorp. All seats are driven by the same
/// team agent; hands are open.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Seat {
    pub resources: BTreeMap<String, i64>,
    /// Passed in the current decision phase.
    pub passed: bool,
    /// Ids of this seat's used (face-down) cards, sorted. Set and refreshed
    /// by the module that owns each card, via `Cx::exhaust` / `Cx::refresh`.
    pub exhausted: Vec<String>,
    /// The country piece's card strip: a content card id per slot. Unused so far.
    pub slots: Vec<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HexState {
    pub tokens: BTreeMap<String, i64>,
}

/// Counters the simulator aggregates. Filled by the engine and modules;
/// deterministic and replayed like the rest of the state.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Stats {
    /// Adversary operations (token placements/moves) per round; entry
    /// `r - 1` is round `r`. A proxy for table time.
    pub adversary_ops: Vec<u32>,
    /// Adversary operations during setup, before round 1.
    pub setup_ops: u32,
    /// Player actions applied, passes included.
    pub actions: u32,
    /// Free-form counters, keyed `"<module>.<name>"`.
    pub counters: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub won: bool,
    /// The round in which the game ended.
    pub round: u32,
    /// Stable reason code (`"survived"`, `"overrun"`, ...), for aggregation.
    pub reason: String,
    /// Human-readable detail.
    pub detail: String,
}

impl GameState {
    pub fn tokens(&self, hex: usize, kind: &str) -> i64 {
        self.hexes[hex].tokens.get(kind).copied().unwrap_or(0)
    }

    pub fn resource(&self, seat: usize, name: &str) -> i64 {
        self.seats[seat].resources.get(name).copied().unwrap_or(0)
    }

    pub fn var(&self, key: &str) -> i64 {
        self.vars.get(key).copied().unwrap_or(0)
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    /// Serialized JSON (stable: every map is ordered).
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("state serializes")
    }

    /// FNV-1a of the JSON form, as 16 hex digits; for replay checks and logs.
    pub fn fingerprint(&self) -> String {
        format!("{:016x}", crate::hash::fnv1a64(self.to_json().as_bytes()))
    }
}
