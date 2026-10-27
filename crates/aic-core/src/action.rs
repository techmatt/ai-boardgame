//! Player actions. Every action names its seat: within a decision phase any
//! seat may act in any order until every seat has passed.

use serde::{Deserialize, Serialize};

use crate::hex::Hex;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// The seat is done for this phase.
    Pass { seat: u8 },
    /// Play card `card` (a content card id, so action logs survive content
    /// edits), on `target` if the card has a target rule.
    Play {
        seat: u8,
        card: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target: Option<Hex>,
    },
}

impl Action {
    pub fn seat(&self) -> u8 {
        match self {
            Action::Pass { seat } | Action::Play { seat, .. } => *seat,
        }
    }
}
