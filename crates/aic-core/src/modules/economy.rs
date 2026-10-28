//! Economy and production: insight each round, income and production in
//! upkeep (AI tracks, buildings, factories, data centres), building upkeep,
//! and giving insight to another player.

use crate::action::Action;
use crate::content::{Passive, RESOURCES};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, Res, VariantDef,
    World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::*;
use super::{ai, board, cards, opposition};

pub static DEF: ModuleDef = ModuleDef {
    id: "economy",
    summary: "Insight, income, production and upkeep; giving insight",
    requires: &["ai", "board", "cards", "opposition"],
    variants: &[VariantDef {
        name: "standard",
        summary: "Income in upkeep, insight at the start of each round",
        params: &[
            ParamSpec {
                name: "factory_credits",
                kind: ParamKind::Int,
                min: 0,
                max: 10,
                doc: "credits a working factory pays every player with a processor in its country",
            },
            ParamSpec {
                name: "processor_compute",
                kind: ParamKind::Int,
                min: 0,
                max: 10,
                doc: "compute a processor on a working data centre produces",
            },
            ParamSpec {
                name: "give_insight",
                kind: ParamKind::Int,
                min: 0,
                max: 10,
                doc: "insight the receiver gains for 1 given",
            },
        ],
        phases: &[
            PhaseSpec {
                name: "give",
                kind: PhaseKind::Decision,
                doc: "give 1 insight to another player (combine with the action phase)",
            },
            PhaseSpec {
                name: "upkeep",
                kind: PhaseKind::Auto,
                doc: "collect income and production; pay building upkeep",
            },
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// Insight every player gains next round (Global Mandate).
pub const BONUS_INSIGHT: &str = "economy.bonus_insight";

#[derive(Debug)]
struct Economy {
    factory_credits: i64,
    processor_compute: i64,
    give_insight: i64,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Economy {
        factory_credits: b.int("factory_credits")?,
        processor_compute: b.int("processor_compute")?,
        give_insight: b.int("give_insight")?,
    }))
}

/// Seat `s`'s income per resource from its AI tracks, buildings and level
/// rewards (not production), in `RESOURCES` order.
pub fn income(w: &World, g: &GameState, seat: usize) -> [i64; 5] {
    let mut out = [0i64; 5];
    let mut add = |r: &str, n: i64| {
        if let Some(i) = RESOURCES.iter().position(|x| *x == r) {
            out[i] += n;
        }
    };
    for (r, n) in ai::track_income(w, g, seat) {
        add(&r, n);
    }
    for p in cards::passives(w, g, seat) {
        match p {
            Passive::Income { resource, amount } => add(resource, *amount),
            Passive::IncomePerTag {
                resource,
                tag,
                per,
                max,
            } => add(resource, (cards::tags(w, g, seat, tag) / per).min(*max)),
            Passive::IncomeIfTags {
                resource,
                tag,
                count,
                amount,
            } => {
                if cards::tags(w, g, seat, tag) >= *count {
                    add(resource, *amount)
                }
            }
            _ => {}
        }
    }
    add("credits", g.var(opposition::INCOME_BONUS));
    out
}

/// Production per seat if upkeep came now: (credits from working factories,
/// compute from processors on working data centres).
pub fn production(w: &World, g: &GameState) -> Vec<(i64, i64)> {
    Economy {
        factory_credits: w.int("economy.factory_credits"),
        processor_compute: w.int("economy.processor_compute"),
        give_insight: 0,
    }
    .production(w, g)
}

impl Economy {
    /// Production per seat: (credits from factories, compute from processors).
    fn production(&self, w: &World, g: &GameState) -> Vec<(i64, i64)> {
        let mut out = vec![(0, 0); g.seats.len()];
        for c in 0..w.map.countries.len() {
            let blockade = board::flagged(g, "blockade", c);
            let sabotage = board::flagged(g, "sabotage", c);
            let mut factories = 0;
            for &h in &w.map.countries[c].hexes {
                let clean = g.tokens(h, CORRUPTION) == 0;
                let near_adversary = next_to(w, h, |x| has_adversary(g, x));
                if clean
                    && g.tokens(h, FACTORY) > 0
                    && next_to(w, h, |x| g.tokens(x, POPULATION) > 0)
                    && !(blockade && near_adversary)
                {
                    factories += g.tokens(h, FACTORY);
                }
                if clean && g.tokens(h, DATA_CENTRE) > 0 && !(sabotage && near_adversary) {
                    for (s, o) in out.iter_mut().enumerate() {
                        o.1 += g.tokens(h, &processor(s)) * self.processor_compute;
                    }
                }
            }
            for s in seats_in_country(w, g, c) {
                out[s].0 += factories * self.factory_credits;
            }
        }
        out
    }
}

impl Module for Economy {
    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        let bonus = g.var(BONUS_INSIGHT);
        for s in 0..g.seats.len() {
            // Unspent insight is lost.
            let have = g.resource(s, "insight");
            let n = income(&w, g, s)[4] + bonus;
            cx.add_resource(g, s, "insight", n - have);
        }
        cx.set_key(g, BONUS_INSIGHT.into(), 0);
    }

    fn legal(&self, _phase: usize, g: &GameState, _w: &World, seat: u8, out: &mut Vec<Action>) {
        if g.resource(seat as usize, "insight") < 1 {
            return;
        }
        for to in 0..g.seats.len() as u8 {
            if to != seat {
                out.push(Action::Give { seat, to });
            }
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Give { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let Action::Give { seat, to } = *a else {
            return Err(Halt::Illegal("not a give".into()));
        };
        let (s, t) = (seat as usize, to as usize);
        if s == t || t >= g.seats.len() || g.resource(s, "insight") < 1 {
            return Err("can't give insight there".into());
        }
        cx.add_resource(g, s, "insight", -1);
        cx.add_resource(g, t, "insight", self.give_insight);
        for p in cards::passives(&w, g, s) {
            if let Passive::GiveBonus { resource, amount } = p {
                cx.add_resource(g, t, resource, *amount);
            }
        }
        cx.count(g, "insight_given", 1);
        Ok(())
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        let prod = self.production(&w, g);
        for s in 0..g.seats.len() {
            let inc = income(&w, g, s);
            for (i, r) in RESOURCES.iter().enumerate() {
                if *r != "insight" {
                    cx.add_resource(g, s, r, inc[i]);
                }
            }
            cx.add_resource(g, s, "credits", prod[s].0);
            cx.add_resource(g, s, "compute", prod[s].1);
            cx.count(g, "credits_income", inc[0] + prod[s].0);
            cards::upkeep(g, cx, s);
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let prod = self.production(w, g);
        for s in 0..g.seats.len() {
            let inc = income(w, g, s);
            let parts = [
                ("credits", inc[0] + prod[s].0, "credits in upkeep: tracks, buildings, factories"),
                ("compute", inc[1] + prod[s].1, "compute in upkeep: processors on working data centres, buildings"),
                ("energy", inc[2], "energy in upkeep: tracks, buildings"),
                ("antimatter", inc[3], "antimatter in upkeep: late track values"),
                ("insight", inc[4], "insight at the start of each round (unspent insight is lost)"),
            ];
            for (r, n, doc) in parts {
                out.seat(s, &format!("income.{r}"), &format!("+{r}"), n, None, doc);
            }
        }
        out.bookkeeping(&[BONUS_INSIGHT]);
        if g.var(BONUS_INSIGHT) > 0 {
            out.global("bonus_insight", "insight next round", g.var(BONUS_INSIGHT), None, "every player gains this next round");
        }
    }
}
