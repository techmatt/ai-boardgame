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

const BASE: [ParamSpec; 5] = [
    i("per_player", 1, 50, "opposition per player that fills one level"),
    i("max_level", 1, 10, "the top level; reaching it wins"),
    i("outpost", 0, 10, "opposition for destroying an outpost"),
    i("foundry", 0, 10, "opposition for destroying a foundry"),
    i("loss_population_percent", 0, 100, "players lose when population falls below this % of the start"),
];

const PUSHBACK: [ParamSpec; 6] = [
    BASE[0],
    BASE[1],
    BASE[2],
    BASE[3],
    BASE[4],
    i("pushback_drones", 0, 5, "drones removed from each country when a level is reached"),
];
const INCOME: [ParamSpec; 6] = [
    BASE[0],
    BASE[1],
    BASE[2],
    BASE[3],
    BASE[4],
    i("income_credits", 0, 20, "credit income every player gains when a level is reached"),
];
const FREE: [ParamSpec; 6] = [
    BASE[0],
    BASE[1],
    BASE[2],
    BASE[3],
    BASE[4],
    i("free_purchases", 0, 20, "card purchases (keeping a drafted card) that become free when a level is reached"),
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
    ],
};

pub const POINTS: &str = "opposition.points";
pub const LEVEL: &str = "opposition.level";
/// Credit income every player has from levels (variant `income`).
pub const INCOME_BONUS: &str = "opposition.income_bonus";
/// Free card purchases left (variant `free_purchases`).
pub const FREE_LEFT: &str = "opposition.free_purchases";

#[derive(Debug, Clone, Copy)]
enum Reward {
    Pushback(i64),
    Income(i64),
    Free(i64),
    Counter,
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

/// Whether the adversary's presence is under the bar for `level`.
fn under_bar(w: &World, g: &GameState, level: i64) -> bool {
    let count = |k: &str| (0..g.hexes.len()).map(|h| g.tokens(h, k)).sum::<i64>();
    if level >= w.int("opposition.max_level") {
        return true;
    }
    match level {
        0 => count(DRONE) + count(OUTPOST) + count(FOUNDRY) == 0,
        1 => count(OUTPOST) + count(FOUNDRY) == 0,
        _ => count(FOUNDRY) == 0,
    }
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
        out.global("level", "opposition level", level, Some(max), win_text(level, max));
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
        if g.var(FREE_LEFT) > 0 {
            out.global("free_purchases", "free purchases", g.var(FREE_LEFT), None, "card purchases that are free");
        }
    }
}

fn win_text(level: i64, max: i64) -> &'static str {
    if level >= max {
        return "players win";
    }
    match level {
        0 => "win when no adversary pieces remain",
        1 => "win when no outposts or foundries remain",
        _ => "win when no foundries remain",
    }
}

impl Opposition {
    fn reward(&self, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match self.reward {
            Reward::Pushback(n) => {
                for c in 0..w.map.countries.len() {
                    for _ in 0..n {
                        let hexes: Vec<usize> = w.map.countries[c]
                            .hexes
                            .iter()
                            .copied()
                            .filter(|&h| g.tokens(h, DRONE) > 0)
                            .collect();
                        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                        let prompt = format!("Pushback in country {c}: remove a drone from which hex?");
                        if let Some(i) = cx.choose(0, &prompt, opts)? {
                            cx.add_tokens(g, hexes[i], DRONE, -1);
                        }
                    }
                }
            }
            Reward::Income(n) => cx.add_key(g, INCOME_BONUS.into(), n),
            Reward::Free(n) => cx.add_key(g, FREE_LEFT.into(), n),
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
