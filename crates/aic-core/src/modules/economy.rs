//! Economy: insight each round, income in upkeep (AI credit production,
//! AI tracks, buildings, Unity cards), building upkeep, and giving insight
//! to another player. What the board produces (energy, compute,
//! megaproject cubes) is the board's production phase.

use crate::action::Action;
use crate::content::{Passive, RESOURCES};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, Res, VariantDef,
    World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::{ai, alignment, cards};

pub static DEF: ModuleDef = ModuleDef {
    id: "economy",
    summary: "Insight, income, production and upkeep; giving insight",
    requires: &["ai", "cards", "alignment"],
    variants: &[VariantDef {
        name: "standard",
        summary: "Income in upkeep, insight at the start of each round",
        params: &[
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
                doc: "collect income (credit production, tracks, buildings); pay building upkeep",
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
    give_insight: i64,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Economy {
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
    add("credits", ai::board(w, seat).credits);
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
    add("credits", g.var(alignment::INCOME_BONUS));
    add("insight", g.var(alignment::INSIGHT_BONUS));
    out
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
        for s in 0..g.seats.len() {
            let inc = income(&w, g, s);
            for (i, r) in RESOURCES.iter().enumerate() {
                if *r != "insight" {
                    cx.add_resource(g, s, r, inc[i]);
                }
            }
            cx.count(g, "credits_income", inc[0]);
            cards::upkeep(g, cx, s);
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        for s in 0..g.seats.len() {
            let inc = income(w, g, s);
            let parts = [
                ("credits", inc[0], "credits in upkeep: credit production, tracks, buildings"),
                ("compute", inc[1], "compute in upkeep: buildings (data centres make compute in production)"),
                ("energy", inc[2], "energy in upkeep: tracks, buildings (cities make energy in production)"),
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
