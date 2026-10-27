//! Placeholder player phase: every seat may play each of an open set of
//! content cards once per round, paying its cost in credits.

use super::CREDITS;
use crate::action::Action;
use crate::content::{Effect, TargetRule};
use crate::event::Event;
use crate::module::{Build, Cx, Module, ModuleDef, PhaseKind, PhaseSpec, VariantDef, World};
use crate::GameState;

pub static DEF: ModuleDef = ModuleDef {
    id: "actions",
    summary: "Seats play open cards built from effect primitives.",
    variants: &[VariantDef {
        name: "open_cards",
        summary: "Every seat may play each configured card once per round if it can pay. \
                  Card numbers given as {\"param\": ...} are this module's tunables.",
        params: &[],
        phases: &[PhaseSpec {
            name: "play",
            kind: PhaseKind::Decision,
            doc: "Seats play cards, then pass.",
        }],
        rng: &[],
        uses_cards: true,
        build,
    }],
};

/// The same behaviour under a second module id, so tests can put two card
/// modules into one combined decision phase.
#[cfg(test)]
pub static TEST_TWIN: ModuleDef = ModuleDef {
    id: "twin_actions",
    summary: "Test-only copy of actions.",
    variants: DEF.variants,
};

#[derive(Debug)]
struct Card {
    id: String,
    cost: i64,
    target: Option<TargetRule>,
    effects: Vec<Op>,
}

#[derive(Debug)]
enum Op {
    Gain(String, i64),
    Remove(String, i64),
    Place(String, i64),
}

#[derive(Debug)]
struct OpenCards {
    cards: Vec<Card>,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let mut cards = Vec::new();
    for &i in &b.cards {
        let d = &b.world.content.cards[i];
        let mut effects = Vec::new();
        for e in &d.effects {
            let n = b.amount(e.amount())?;
            effects.push(match e {
                Effect::Gain { resource, .. } => Op::Gain(resource.clone(), n),
                Effect::RemoveTokens { token, .. } => Op::Remove(token.clone(), n),
                Effect::PlaceTokens { token, .. } => Op::Place(token.clone(), n),
            });
        }
        cards.push(Card {
            id: d.id.clone(),
            cost: b.amount(&d.cost)?,
            target: d.target.clone(),
            effects,
        });
    }
    Ok(Box::new(OpenCards { cards }))
}

impl OpenCards {
    fn card(&self, id: &str) -> Option<&Card> {
        self.cards.iter().find(|c| c.id == id)
    }

    fn playable(&self, g: &GameState, seat: usize, c: &Card) -> bool {
        g.seats[seat].exhausted.binary_search(&c.id).is_err() && g.resource(seat, CREDITS) >= c.cost
    }

    fn valid_target(g: &GameState, w: &World, seat: usize, rule: &TargetRule, hex: usize) -> bool {
        match rule {
            TargetRule::OwnHex { with_token } => {
                w.map.country_of(hex) as usize == seat
                    && with_token.as_ref().is_none_or(|t| g.tokens(hex, t) > 0)
            }
        }
    }
}

impl Module for OpenCards {
    /// Used cards refresh at the start of every round.
    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            for c in &self.cards {
                cx.refresh(g, s, &c.id);
            }
        }
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        for c in &self.cards {
            if !self.playable(g, s, c) {
                continue;
            }
            match &c.target {
                None => out.push(Action::Play {
                    seat,
                    card: c.id.clone(),
                    target: None,
                }),
                Some(rule) => {
                    for &h in &w.map.countries[s].hexes {
                        if Self::valid_target(g, w, s, rule, h) {
                            out.push(Action::Play {
                                seat,
                                card: c.id.clone(),
                                target: Some(w.map.hexes[h].hex),
                            });
                        }
                    }
                }
            }
        }
    }

    fn owns(&self, _phase: usize, action: &Action) -> bool {
        matches!(action, Action::Play { card, .. } if self.card(card).is_some())
    }

    fn act(
        &self,
        _phase: usize,
        g: &mut GameState,
        cx: &mut Cx,
        action: &Action,
    ) -> Result<(), String> {
        let Action::Play { seat, card, target } = action else {
            return Err(format!("unexpected action {action:?}"));
        };
        let (seat, target) = (*seat, *target);
        let s = seat as usize;
        let c = self
            .card(card)
            .ok_or_else(|| format!("card {card:?} is not in play"))?;
        if !self.playable(g, s, c) {
            return Err(format!(
                "seat {seat} cannot play {} now (used or unaffordable)",
                c.id
            ));
        }
        let hex = match (&c.target, target) {
            (None, None) => None,
            (Some(rule), Some(t)) => {
                let h = cx
                    .world
                    .map
                    .find(t)
                    .ok_or_else(|| format!("{t:?} is not on the board"))?;
                if !Self::valid_target(g, &cx.world, s, rule, h) {
                    return Err(format!("{t:?} is not a valid target for {}", c.id));
                }
                Some(h)
            }
            (None, Some(_)) => return Err(format!("{} takes no target", c.id)),
            (Some(_), None) => return Err(format!("{} needs a target", c.id)),
        };
        cx.emit(|| Event::CardPlayed {
            seat,
            card: c.id.clone(),
            target,
        });
        cx.add_resource(g, s, CREDITS, -c.cost);
        cx.exhaust(g, s, &c.id);
        for op in &c.effects {
            match op {
                Op::Gain(r, n) => cx.add_resource(g, s, r, *n),
                Op::Remove(t, n) => {
                    let removed = -cx.add_tokens(g, hex.expect("checked"), t, -n);
                    cx.count(g, &format!("removed.{t}"), removed);
                }
                Op::Place(t, n) => {
                    cx.add_tokens(g, hex.expect("checked"), t, *n);
                }
            }
        }
        cx.count(g, &format!("played.{}", c.id), 1);
        Ok(())
    }
}
