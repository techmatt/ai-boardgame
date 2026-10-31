//! Saved games: config + seed + action list, the one format for a game
//! the simulator flags for a human look, a browser bug report and a saved
//! browser session. Everything else in a record is identity (hashes,
//! versions) and a summary (outcome, final fingerprint), for checking that
//! a replay is the game that was saved.

use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::config::Config;
use crate::rules::Rules;
use crate::state::{GameState, Outcome};

/// The `format` field of every record. Bump the number on an incompatible
/// change to the record (not to the rules: that is `rules_revision`).
pub const RECORD_FORMAT: &str = "aic-game-record/1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedGame {
    pub format: String,
    /// Why it was saved: a sim selection, or a bug description.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    /// The resolved config (overrides applied, round filled in).
    pub config: Config,
    /// Decimal string in JSON.
    #[serde(with = "crate::rng::as_string")]
    pub seed: u64,
    #[serde(default)]
    pub config_hash: String,
    #[serde(default)]
    pub content_hash: String,
    #[serde(default)]
    pub engine_version: String,
    #[serde(default)]
    pub rules_revision: u32,
    /// The outcome when saved; `None` for a game in progress or timed out.
    #[serde(default)]
    pub outcome: Option<Outcome>,
    /// Rounds reached when saved.
    #[serde(default)]
    pub rounds: u32,
    /// `GameState::fingerprint` after the last action, when saved.
    #[serde(default)]
    pub final_fingerprint: String,
    pub actions: Vec<Action>,
}

impl SavedGame {
    /// A record of `actions` played from `seed`, ending in `last`.
    pub fn new(
        rules: &Rules,
        seed: u64,
        actions: Vec<Action>,
        last: &GameState,
        note: &str,
    ) -> Self {
        SavedGame {
            format: RECORD_FORMAT.to_string(),
            note: note.to_string(),
            config: (*rules.config).clone(),
            seed,
            config_hash: rules.config_hash.clone(),
            content_hash: rules.content_hash.to_string(),
            engine_version: crate::VERSION.to_string(),
            rules_revision: crate::RULES_REVISION,
            outcome: last.outcome.clone(),
            rounds: last.round,
            final_fingerprint: last.fingerprint(),
            actions,
        }
    }

    /// Parses a record. Only `config`, `seed` and `actions` are required,
    /// so a hand-written `{config, seed, actions}` loads too.
    pub fn from_json(text: &str) -> Result<SavedGame, String> {
        #[derive(Deserialize)]
        struct Loose {
            #[serde(default)]
            format: String,
            #[serde(flatten)]
            rest: serde_json::Value,
        }
        let loose: Loose =
            serde_json::from_str(text).map_err(|e| format!("bad record JSON: {e}"))?;
        if !loose.format.is_empty() && loose.format != RECORD_FORMAT {
            return Err(format!(
                "record format {:?}, this engine reads {RECORD_FORMAT:?}",
                loose.format
            ));
        }
        let mut rest = loose.rest;
        if let Some(o) = rest.as_object_mut() {
            o.insert("format".into(), RECORD_FORMAT.into());
            // A hand-written seed may be a JSON number; records write a string.
            if let Some(n) = o.get("seed").and_then(|s| s.as_u64()) {
                o.insert("seed".into(), n.to_string().into());
            } else if let Some(s) = o.get("seed").filter(|s| s.is_number()) {
                // 020: a negative, fractional or too-large number said "expected a
                // string", naming no field.
                return Err(format!(
                    "bad record: seed {s} is not a whole number from 0 to {}",
                    u64::MAX
                ));
            }
        }
        serde_json::from_value(rest).map_err(|e| format!("bad record: {e}"))
    }

    /// JSON with one action per line, so a record reads and diffs well.
    pub fn to_text(&self) -> String {
        let head = serde_json::to_value(self).expect("records serialize");
        let obj = head.as_object().expect("an object");
        let mut out = String::from("{\n");
        // Field order, not the map's alphabetical order.
        for k in HEAD {
            if let Some(v) = obj.get(*k) {
                out.push_str(&format!("  {}: {},\n", json(k), v));
            }
        }
        // A field `HEAD` doesn't list yet must not vanish from saved files.
        for (k, v) in obj {
            if k != "actions" && !HEAD.contains(&k.as_str()) {
                out.push_str(&format!("  {}: {},\n", json(k), v));
            }
        }
        out.push_str("  \"actions\": [");
        for (i, a) in self.actions.iter().enumerate() {
            out.push_str(if i == 0 { "\n    " } else { ",\n    " });
            out.push_str(&serde_json::to_string(a).expect("actions serialize"));
        }
        out.push_str(if self.actions.is_empty() {
            "]\n}\n"
        } else {
            "\n  ]\n}\n"
        });
        out
    }

    /// Replays the record and checks it against what was saved. Returns the
    /// final state and a list of warnings (a different rules revision,
    /// content hash, config hash or final fingerprint; a different engine
    /// version alone is not one). An invalid config or a rejected action is
    /// an error.
    pub fn replay(&self) -> Result<(GameState, Vec<String>), String> {
        let rules = Rules::new(&self.config)?;
        let g = crate::replay(&rules, self.seed, &self.actions)?;
        let mut warnings = Vec::new();
        let mut differs = |what: &str, saved: &str, now: &str| {
            if !saved.is_empty() && saved != now {
                warnings.push(format!("{what}: saved {saved}, now {now}"));
            }
        };
        differs(
            "rules revision",
            &self.rules_revision_str(),
            &crate::RULES_REVISION.to_string(),
        );
        differs("content hash", &self.content_hash, rules.content_hash);
        differs("config hash", &self.config_hash, &rules.config_hash);
        differs(
            "final state fingerprint",
            &self.final_fingerprint,
            &g.fingerprint(),
        );
        Ok((g, warnings))
    }

    fn rules_revision_str(&self) -> String {
        if self.rules_revision == 0 {
            String::new()
        } else {
            self.rules_revision.to_string()
        }
    }
}

/// Every field but `actions`, in the order `to_text` writes them.
const HEAD: &[&str] = &[
    "format",
    "note",
    "seed",
    "outcome",
    "rounds",
    "final_fingerprint",
    "config_hash",
    "content_hash",
    "engine_version",
    "rules_revision",
    "config",
];

fn json(s: &str) -> String {
    serde_json::to_string(s).expect("strings serialize")
}
