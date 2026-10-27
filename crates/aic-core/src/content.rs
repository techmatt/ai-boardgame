//! Game content as data: map pieces and layouts, cards built from effect
//! primitives. Embedded from `content/` at compile time (no I/O at runtime).
//!
//! A new *effect primitive* (a new [`Effect`] or [`TargetRule`] variant) is a
//! code change. A new card that only combines existing primitives is a data
//! change to `content/cards.json`.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::hash::fnv1a64;
use crate::hex::{Dir, Hex};

const MAP_JSON: &str = include_str!("../../../content/map.json");
const CARDS_JSON: &str = include_str!("../../../content/cards.json");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    pub map: MapContent,
    pub cards: Vec<CardDef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapContent {
    pub piece: PieceDef,
    pub layouts: Vec<LayoutDef>,
}

/// The country piece: a `size` x `size` rhombus of hexes with a strip of
/// `card_slots` slots for per-country cards.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PieceDef {
    pub size: u8,
    pub card_slots: u8,
}

/// Where the pieces go for a given number of countries. Piece `i` is seat `i`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutDef {
    pub countries: u8,
    pub pieces: Vec<PiecePlacement>,
}

/// A placed piece covers `anchor + i*axes[0] + j*axes[1]` for `i, j` in
/// `0..size`. The two axes must be 60 or 120 degrees apart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PiecePlacement {
    pub anchor: Hex,
    pub axes: [Dir; 2],
    /// Display colour from the reference images; the core never reads it.
    #[serde(default)]
    pub colour: String,
}

/// A number in content: a literal, or `{"param": "name"}` naming a tunable
/// of the module that uses the content. Only parameter references are
/// tunable from config; literals change only by editing content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Amount {
    Lit(i64),
    Param { param: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub text: String,
    /// Paid in credits when played.
    pub cost: Amount,
    #[serde(default)]
    pub target: Option<TargetRule>,
    pub effects: Vec<Effect>,
}

/// What a card is played on. Targets are always within the playing seat's
/// own country: a simultaneous phase must not let seats interfere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum TargetRule {
    /// A hex in the player's own country, optionally one holding at least
    /// one of a token kind (no useless targets are offered).
    OwnHex {
        #[serde(default)]
        with_token: Option<String>,
    },
}

/// Effect primitives. Adding one is a code change (here, in
/// `modules/actions.rs`, and in `docs/architecture.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Effect {
    /// The playing seat gains `amount` of `resource`.
    Gain { resource: String, amount: Amount },
    /// Remove up to `amount` tokens of `token` from the target hex.
    RemoveTokens { token: String, amount: Amount },
    /// Add `amount` tokens of `token` to the target hex.
    PlaceTokens { token: String, amount: Amount },
}

impl Effect {
    pub fn amount(&self) -> &Amount {
        match self {
            Effect::Gain { amount, .. }
            | Effect::RemoveTokens { amount, .. }
            | Effect::PlaceTokens { amount, .. } => amount,
        }
    }

    pub fn needs_target(&self) -> bool {
        matches!(
            self,
            Effect::RemoveTokens { .. } | Effect::PlaceTokens { .. }
        )
    }
}

impl CardDef {
    /// Parameter names this card refers to, in first-use order.
    pub fn params(&self) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        let amounts = std::iter::once(&self.cost).chain(self.effects.iter().map(Effect::amount));
        for a in amounts {
            if let Amount::Param { param } = a {
                if !out.contains(&param.as_str()) {
                    out.push(param);
                }
            }
        }
        out
    }
}

impl Content {
    pub fn parse(map_json: &str, cards_json: &str) -> Result<Content, String> {
        let map: MapContent =
            serde_json::from_str(map_json).map_err(|e| format!("content/map.json: {e}"))?;
        let cards: Vec<CardDef> =
            serde_json::from_str(cards_json).map_err(|e| format!("content/cards.json: {e}"))?;
        let c = Content { map, cards };
        c.check()?;
        Ok(c)
    }

    fn check(&self) -> Result<(), String> {
        let mut ids: Vec<&str> = self.cards.iter().map(|c| c.id.as_str()).collect();
        ids.sort_unstable();
        if let Some(w) = ids.windows(2).find(|w| w[0] == w[1]) {
            return Err(format!("content/cards.json: duplicate card id {:?}", w[0]));
        }
        for c in &self.cards {
            let targeted = c.effects.iter().any(Effect::needs_target);
            if targeted != c.target.is_some() {
                return Err(format!(
                    "card {:?}: a target rule is needed exactly when an effect acts on a hex",
                    c.id
                ));
            }
        }
        for l in &self.map.layouts {
            if l.pieces.len() != l.countries as usize {
                return Err(format!(
                    "layout for {} countries has {} pieces",
                    l.countries,
                    l.pieces.len()
                ));
            }
            for p in &l.pieces {
                let d = (p.axes[0].index() + 6 - p.axes[1].index()) % 6;
                if d == 0 || d == 3 {
                    return Err(format!(
                        "piece at {:?}: axes {:?} do not span a rhombus",
                        p.anchor, p.axes
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn card_index(&self, id: &str) -> Option<usize> {
        self.cards.iter().position(|c| c.id == id)
    }

    pub fn layout(&self, countries: u8) -> Option<&LayoutDef> {
        self.map.layouts.iter().find(|l| l.countries == countries)
    }
}

/// The embedded content, parsed once.
pub fn content() -> &'static Content {
    static C: OnceLock<Content> = OnceLock::new();
    C.get_or_init(|| Content::parse(MAP_JSON, CARDS_JSON).expect("embedded content is valid"))
}

/// Hash of the embedded content files, recorded with every result so a
/// content edit is never mistaken for the same experiment.
pub fn content_hash() -> String {
    let mut bytes = Vec::with_capacity(MAP_JSON.len() + CARDS_JSON.len() + 1);
    bytes.extend_from_slice(MAP_JSON.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(CARDS_JSON.as_bytes());
    format!("{:016x}", fnv1a64(&bytes))
}
