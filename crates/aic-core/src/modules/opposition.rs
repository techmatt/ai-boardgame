//! Human opposition and the end of the game: the shared opposition track,
//! its levels and their one-time reward (the variants), and the win and
//! loss conditions, checked after every action and every automatic phase.

use crate::content::Passive;
use crate::module::{
    Build, Cx, Module, ModuleDef, ParamKind, ParamSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::*;
use super::{adversary, board, cards, threats};

const fn i(name: &'static str, min: i64, max: i64, doc: &'static str) -> ParamSpec {
    ParamSpec {
        name,
        kind: ParamKind::Int,
        min,
        max,
        doc,
    }
}

const BASE: [ParamSpec; 6] = [
    i("per_player", 1, 50, "opposition per player that fills one level"),
    i("max_level", 1, 10, "the top level; reaching it wins"),
    i("outpost", 0, 10, "opposition for destroying an outpost"),
    i("foundry", 0, 10, "opposition for destroying a foundry"),
    i("loss_population_percent", 0, 100, "players lose when population falls below this % of the start"),
    i("clear_win_stage", 1, 3, "the level-0 win (no adversary pieces) counts only once a card of this stage has been revealed (1: any time)"),
];

const PUSHBACK: [ParamSpec; 7] = [
    BASE[0],
    BASE[1],
    BASE[2],
    BASE[3],
    BASE[4],
    BASE[5],
    i("pushback_drones", 0, 5, "drones removed from each country when a level is reached"),
];
const INCOME: [ParamSpec; 7] = [
    BASE[0],
    BASE[1],
    BASE[2],
    BASE[3],
    BASE[4],
    BASE[5],
    i("income_credits", 0, 20, "credit income every player gains when a level is reached"),
];
const FREE: [ParamSpec; 7] = [
    BASE[0],
    BASE[1],
    BASE[2],
    BASE[3],
    BASE[4],
    BASE[5],
    i("free_purchases", 0, 20, "card purchases (keeping a drafted card) that become free when a level is reached"),
];
const INSIGHT: [ParamSpec; 7] = [
    BASE[0],
    BASE[1],
    BASE[2],
    BASE[3],
    BASE[4],
    BASE[5],
    i("insight_income", 0, 5, "insight every player gains each round, for the rest of the game, when a level is reached"),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "opposition",
    summary: "The opposition track, its level rewards, and winning and losing",
    requires: &["adversary", "board", "cards", "threats"],
    variants: &[
        VariantDef {
            name: "pushback",
            summary: "Reaching a level: remove drones in each country (players choose which)",
            params: &PUSHBACK,
            phases: &[],
            rng: &[],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "income",
            summary: "Reaching a level: every player gains credit income",
            params: &INCOME,
            phases: &[],
            rng: &[],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "free_purchases",
            summary: "Reaching a level: the next K card purchases are free",
            params: &FREE,
            phases: &[],
            rng: &[],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "counter_threat",
            summary: "Reaching a level: counter one threat (players choose)",
            params: &BASE,
            phases: &[],
            rng: &[],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "insight",
            summary: "Reaching a level: every player gains insight each round for the rest of the game",
            params: &INSIGHT,
            phases: &[],
            rng: &[],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "counter_all",
            summary: "Reaching a level: counter every active threat",
            params: &BASE,
            phases: &[],
            rng: &[],
            uses_cards: false,
            build: build_counter_all,
        },
        VariantDef {
            name: "off",
            summary: "Reaching a level gives no reward (the level still lowers the purge bar)",
            params: &BASE,
            phases: &[],
            rng: &[],
            uses_cards: false,
            build: build_off,
        },
    ],
};

fn build_off(_: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Opposition { reward: Reward::None }))
}

fn build_counter_all(_: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Opposition { reward: Reward::CounterAll }))
}

pub const POINTS: &str = "opposition.points";
pub const LEVEL: &str = "opposition.level";
/// Credit income every player has from levels (variant `income`).
pub const INCOME_BONUS: &str = "opposition.income_bonus";
/// Free card purchases left (variant `free_purchases`).
pub const FREE_LEFT: &str = "opposition.free_purchases";
/// Insight every player gains each round from levels (variant `insight`).
pub const INSIGHT_BONUS: &str = "opposition.insight_bonus";

#[derive(Debug, Clone, Copy)]
enum Reward {
    Pushback(i64),
    Income(i64),
    Free(i64),
    Insight(i64),
    Counter,
    CounterAll,
    None,
}

#[derive(Debug)]
struct Opposition {
    reward: Reward,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let reward = if b.params.contains_key("pushback_drones") {
        Reward::Pushback(b.int("pushback_drones")?)
    } else if b.params.contains_key("income_credits") {
        Reward::Income(b.int("income_credits")?)
    } else if b.params.contains_key("free_purchases") {
        Reward::Free(b.int("free_purchases")?)
    } else if b.params.contains_key("insight_income") {
        Reward::Insight(b.int("insight_income")?)
    } else {
        Reward::Counter
    };
    Ok(Box::new(Opposition { reward }))
}

/// Adds opposition.
pub fn gain(g: &mut GameState, cx: &mut Cx, n: i64) {
    if n > 0 {
        cx.add_key(g, POINTS.into(), n);
    }
}

/// Opposition for a destroyed adversary piece (plus the destroyer's
/// Opposition Network bonus).
pub fn for_kill(g: &mut GameState, cx: &mut Cx, seat: Option<usize>, kind: &str) {
    let w = cx.world;
    let base = match kind {
        OUTPOST => w.int("opposition.outpost"),
        FOUNDRY => w.int("opposition.foundry"),
        CLUSTER => w.int("adversary.cluster_opposition"),
        ROGUE_FACTORY | ROGUE_DATA_CENTRE => w.int("adversary.reclaim_opposition"),
        _ => return,
    };
    let bonus: i64 = seat.map_or(0, |s| {
        cards::passives(&w, g, s)
            .iter()
            .map(|p| match p {
                Passive::OppositionBonus { amount } => *amount,
                _ => 0,
            })
            .sum()
    });
    gain(g, cx, base + bonus);
}

/// The level the track has reached.
pub fn level_of(w: &World, g: &GameState) -> i64 {
    let per = w.int("opposition.per_player") * g.seats.len() as i64;
    (g.var(POINTS) / per).min(w.int("opposition.max_level"))
}

fn population(g: &GameState) -> i64 {
    (0..g.hexes.len()).map(|h| g.tokens(h, POPULATION)).sum()
}

/// Whether the adversary's presence is under the bar for `level` (the
/// bars per adversary variant are in `adversary::under_bar`).
/// The level-0 bar (no adversary pieces) counts only once a card of stage
/// `clear_win_stage` has been revealed (the stages are stacked and public).
fn under_bar(w: &World, g: &GameState, level: i64) -> bool {
    if level >= w.int("opposition.max_level") {
        return true;
    }
    if level == 0 && g.var(adversary::STAGE) < w.int("opposition.clear_win_stage") {
        return false;
    }
    adversary::under_bar(w, g, level)
}

impl Module for Opposition {
    fn after(&self, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        // Losses first.
        if g.var(adversary::POOL) <= 0 {
            cx.end(g, false, "corruption", "the corruption pool ran out".into());
            return Ok(());
        }
        let start = g.var(board::START_POPULATION);
        let pop = population(g);
        if pop * 100 < start * w.int("opposition.loss_population_percent") {
            cx.end(
                g,
                false,
                "population",
                format!("population fell to {pop} of {start}"),
            );
            return Ok(());
        }
        // New levels, each with its one-time reward.
        let reached = level_of(&w, g);
        while g.var(LEVEL) < reached {
            let lvl = g.var(LEVEL) + 1;
            cx.set_key(g, LEVEL.into(), lvl);
            cx.note(None, || format!("Opposition reaches level {lvl}"));
            // The top level wins at once: its reward would change nothing.
            if lvl < w.int("opposition.max_level") {
                self.reward(g, cx)?;
            }
        }
        let level = g.var(LEVEL);
        if under_bar(&w, g, level) {
            cx.end(
                g,
                true,
                "opposition",
                format!("the adversary fell under the bar for opposition level {level}"),
            );
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let per = w.int("opposition.per_player") * g.seats.len() as i64;
        let level = g.var(LEVEL);
        let max = w.int("opposition.max_level");
        out.global(
            "points",
            "opposition",
            g.var(POINTS),
            Some(per * (level + 1).min(max)),
            "humanity rising: every per_player x players opposition fills a level",
        );
        let mut text = win_text(w, level, max);
        let stage = w.int("opposition.clear_win_stage");
        if level == 0 && g.var(adversary::STAGE) < stage {
            text += &format!(" (from adversary stage {stage})");
        }
        out.global("level", "opposition level", level, Some(max), &text);
        let start = g.var(board::START_POPULATION);
        out.global(
            "population",
            "population",
            population(g),
            None,
            "players lose when population falls below the loss threshold",
        );
        out.global(
            "loss_population",
            "lose below",
            (start * w.int("opposition.loss_population_percent") + 99) / 100,
            None,
            "population at which players lose (it must stay at or above this)",
        );
        if g.var(INCOME_BONUS) > 0 {
            out.global("income_bonus", "level income bonus", g.var(INCOME_BONUS), None, "credit income every player has from levels");
        }
        if g.var(INSIGHT_BONUS) > 0 {
            out.global("insight_bonus", "level insight bonus", g.var(INSIGHT_BONUS), None, "insight every player gains each round from levels");
        }
        if g.var(FREE_LEFT) > 0 {
            out.global("free_purchases", "free purchases", g.var(FREE_LEFT), None, "card purchases that are free");
        }
    }
}

fn win_text(w: &World, level: i64, max: i64) -> String {
    if level >= max {
        return "players win".into();
    }
    adversary::win_text(w, level)
}

impl Opposition {
    fn reward(&self, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match self.reward {
            Reward::Pushback(n) => {
                for c in 0..w.map.countries.len() {
                    for _ in 0..n {
                        adversary::pushback(g, cx, c)?;
                    }
                }
            }
            Reward::None => {}
            Reward::Income(n) => cx.add_key(g, INCOME_BONUS.into(), n),
            Reward::Free(n) => cx.add_key(g, FREE_LEFT.into(), n),
            Reward::Insight(n) => cx.add_key(g, INSIGHT_BONUS.into(), n),
            Reward::CounterAll => {
                for (t, _) in threats::active(g) {
                    threats::counter(g, cx, &t);
                }
            }
            Reward::Counter => {
                let active = threats::active(g);
                let opts = active.iter().map(|(t, _)| opt(t.clone())).collect();
                if let Some(i) = cx.choose(0, "Opposition: counter which threat?", opts)? {
                    threats::counter(g, cx, &active[i].0);
                }
            }
        }
        Ok(())
    }
}
