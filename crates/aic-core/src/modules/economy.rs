//! Economy: insight each round, income in upkeep (AI credit production,
//! AI tracks, buildings, Unity cards), building upkeep, and giving insight
//! to another player. What the board produces (energy, compute,
//! megaproject cubes) is the board's production phase.

use crate::action::Action;
use crate::content::{Passive, ANTIMATTER, COMPUTE, CREDITS, ENERGY, INSIGHT, RESOURCES};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::has_presence_in;
use super::{ai, alignment, cards};

pub static DEF: ModuleDef = ModuleDef {
    id: "economy",
    summary: "Insight, income and upkeep; giving insight",
    requires: &["ai"],
    variants: &[VariantDef {
        name: "standard",
        summary: "Income in upkeep, insight at the start of each round",
        params: &[ParamSpec::int(
            "give_insight",
            0,
            10,
            "insight the receiver gains for 1 given",
        )],
        phases: &[
            PhaseSpec::decision(
                "give",
                "give 1 insight to another player (combine with the action phase)",
            ),
            PhaseSpec::auto(
                "upkeep",
                "collect income (credit production, tracks, buildings); pay building upkeep",
            ),
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// Insight every player gains next round (Global Mandate).
pub const BONUS_INSIGHT: &str = "economy.bonus_insight";

/// Every player gains `n` more insight next round.
pub fn add_bonus_insight(g: &mut GameState, cx: &mut Cx, n: i64) {
    cx.add_key(g, BONUS_INSIGHT.into(), n);
}

#[derive(Debug)]
struct Economy {
    give_insight: i64,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Economy {
        give_insight: b.int("give_insight")?,
    }))
}

/// What the next upkeep pays seat `s`: its `income`, less the compute of a
/// seat with presence in a country sabotaged then. Once this round's upkeep
/// has run, a flag counts only if it outlasts this round's cleanup (2+).
pub fn upkeep_income(w: &World, g: &GameState, seat: usize) -> [i64; RESOURCES.len()] {
    let mut inc = income(w, g, seat);
    let upkeep_done = g
        .config
        .round
        .iter()
        .position(|p| p == "economy.upkeep")
        .is_some_and(|i| g.phase > i);
    let at = if upkeep_done { 2 } else { 1 };
    if (0..w.map.countries.len()).any(|c| {
        g.var(&format!("board.sabotage.{c}")) >= at && has_presence_in(w, g, seat, c)
    }) {
        inc[COMPUTE] = 0;
    }
    inc
}

/// Seat `s`'s income per resource from its AI board, tracks, buildings and
/// Unity cards (not the board's production), in `RESOURCES` order.
pub fn income(w: &World, g: &GameState, seat: usize) -> [i64; RESOURCES.len()] {
    let mut out = [0i64; RESOURCES.len()];
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
    add("credits", alignment::income_bonus(g));
    add("insight", alignment::insight_bonus(g));
    out
}

impl Module for Economy {
    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        let bonus = g.var(BONUS_INSIGHT);
        for s in 0..g.seats.len() {
            // Unspent insight is lost.
            let have = g.resource(s, "insight");
            let n = income(&w, g, s)[INSIGHT] + bonus;
            cx.add_resource(g, s, "insight", n - have);
        }
        cx.set_key(g, BONUS_INSIGHT.into(), 0);
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        // `give_insight` 0 switches giving off (rules.md §9; 020: it was still offered
        // and cost the giver an insight for nothing).
        if g.resource(seat as usize, "insight") < 1 || w.int("economy.give_insight") <= 0 {
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
        if s == t || t >= g.seats.len() || g.resource(s, "insight") < 1 || w.int("economy.give_insight") <= 0 {
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
            let inc = upkeep_income(&w, g, s);
            for (i, r) in RESOURCES.iter().enumerate() {
                if *r != "insight" {
                    cx.add_resource(g, s, r, inc[i]);
                }
            }
            cx.count(g, "credits_income", inc[CREDITS]);
            cards::upkeep(g, cx, s);
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        for s in 0..g.seats.len() {
            // What upkeep will pay, Sabotage included (020: the view showed the
            // compute a sabotaged seat would not get).
            let inc = upkeep_income(w, g, s);
            let parts = [
                (
                    "credits",
                    inc[CREDITS],
                    "credits in upkeep: credit production, tracks, buildings",
                ),
                (
                    "compute",
                    inc[COMPUTE],
                    "compute in upkeep: the presence track, buildings",
                ),
                (
                    "energy",
                    inc[ENERGY],
                    "energy in upkeep: the AI board, buildings (cities make energy in production)",
                ),
                (
                    "antimatter",
                    inc[ANTIMATTER],
                    "antimatter in upkeep: buildings",
                ),
                (
                    "insight",
                    inc[INSIGHT],
                    "insight at the start of each round (unspent insight is lost)",
                ),
            ];
            for (r, n, doc) in parts {
                out.seat(s, &format!("income.{r}"), &format!("+{r}"), n, None, doc);
            }
        }
        out.bookkeeping(&[BONUS_INSIGHT]);
        if g.var(BONUS_INSIGHT) > 0 {
            out.global(
                "bonus_insight",
                "insight next round",
                g.var(BONUS_INSIGHT),
                None,
                "every player gains this next round",
            );
        }
    }
}
