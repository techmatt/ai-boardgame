//! Player actions. Every action names its seat: within a decision phase any
//! seat may act in any order until every seat has passed. Cards, threats
//! and choices are named by content id or option text, so action logs
//! survive content reordering.

use serde::{Deserialize, Serialize};

use crate::hex::Hex;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// The seat is done for this phase.
    Pass { seat: u8 },
    /// Answer the open choice (`GameState::pending`).
    Choose {
        seat: u8,
        option: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target: Option<Hex>,
    },
    /// Draft: choose which tier to be dealt from.
    Tier { seat: u8, tier: u8 },
    /// Draft: take a card from the hand in front of you.
    Draft { seat: u8, card: String },
    /// Draft: buy a drafted card into your hand.
    Keep { seat: u8, card: String },
    /// Pick a growth option of your AI board.
    Growth { seat: u8, option: u8 },
    /// Play a card from your hand.
    Play { seat: u8, card: String },
    /// Use a building's once-a-round ability.
    Use { seat: u8, card: String },
    /// Pay compute to draw a card from a tier's deck.
    Draw { seat: u8, tier: u8 },
    /// Put `amount` of a resource on a threat's counter.
    Pay {
        seat: u8,
        threat: String,
        resource: String,
        amount: i64,
    },
    /// Take back everything you put on a threat's counter in a resource.
    Withdraw {
        seat: u8,
        threat: String,
        resource: String,
    },
    /// Give 1 insight to another seat.
    Give { seat: u8, to: u8 },
    /// Sway toward Discord to produce one antimatter (module `alignment`).
    Antimatter { seat: u8 },
}

impl Action {
    pub fn seat(&self) -> u8 {
        match self {
            Action::Pass { seat }
            | Action::Choose { seat, .. }
            | Action::Tier { seat, .. }
            | Action::Draft { seat, .. }
            | Action::Keep { seat, .. }
            | Action::Growth { seat, .. }
            | Action::Play { seat, .. }
            | Action::Use { seat, .. }
            | Action::Draw { seat, .. }
            | Action::Pay { seat, .. }
            | Action::Withdraw { seat, .. }
            | Action::Give { seat, .. }
            | Action::Antimatter { seat } => *seat,
        }
    }
}
