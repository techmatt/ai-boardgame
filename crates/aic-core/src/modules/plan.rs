//! Read-only forecasts of rules steps, for agents and clients: what a step
//! would do on the current table, without doing it. Not a module: no
//! phases, no state. Everything here reads public information only.
//!
//! The adversary's build step is the one forecast so far (015's deferred
//! call 9: the agents' heuristic used to copy it). It mirrors
//! `adversary::build_all`, and a core test keeps the two in step.

use crate::content::Twist;
use crate::module::World;
use crate::state::GameState;

use super::adversary;
use super::common::{clean_city, corrupted, is_base, is_city, scope_hexes, FACTORY, STATION};

/// What the adversary's build step places with `twists` in force, in country
/// `scope` or everywhere: `(hex, ambassadors)` for every hex that gains any,
/// in hex order. Each corrupted city gains `city_build` (plus `build_bonus`
/// at difficulty 3 or more in play, plus city-build twists); each factory
/// next to a corrupted city `factory_build`, and next to a clean city the
/// clean-factory twists; each corrupted station the station twists. The
/// supply is not checked: the plan is what the step asks for.
pub fn build_plan(
    w: &World,
    g: &GameState,
    twists: &[&Twist],
    scope: Option<usize>,
) -> Vec<(usize, i64)> {
    let sum = |f: fn(&Twist) -> Option<i64>| -> i64 { twists.iter().filter_map(|t| f(t)).sum() };
    let mut city = w.int("adversary.city_build");
    if adversary::level(w, 3) {
        city += w.int("adversary.build_bonus");
    }
    city += sum(|t| match t {
        Twist::CityBuild { amount } => Some(*amount),
        _ => None,
    });
    let clean = sum(|t| match t {
        Twist::CleanFactories { amount } => Some(*amount),
        _ => None,
    });
    let stations = sum(|t| match t {
        Twist::Stations { amount } => Some(*amount),
        _ => None,
    });
    let factory = w.int("adversary.factory_build");
    let mut plan = Vec::new();
    for h in scope_hexes(w, scope) {
        let mut n = 0;
        if is_city(w, h) && corrupted(g, h) {
            n += city;
        }
        if g.tokens(h, FACTORY) > 0 {
            let adj =
                |pred: &dyn Fn(usize) -> bool| w.map.hexes[h].neighbours.iter().any(|&x| pred(x));
            if adj(&|x| is_city(w, x) && corrupted(g, x)) {
                n += factory;
            }
            if adj(&|x| clean_city(w, g, x)) {
                n += clean;
            }
        }
        if g.tokens(h, STATION) > 0 && corrupted(g, h) {
            n += stations;
        }
        if n > 0 {
            plan.push((h, n));
        }
    }
    plan
}

/// The next round's build, read off the face-up adversary card (public):
/// [`build_plan`] with that card's twists, plus its escalation
/// (`escalation_ambassadors` on every military base) when it carries one.
/// Empty when no card is face up (the deck has run out). Read on the table
/// as it stands now, so anything that changes before the adversary phase
/// (a corruption, escalation from Discord) is not foreseen.
pub fn next_build(w: &World, g: &GameState) -> Vec<(usize, i64)> {
    let Some(card) = g
        .pile(adversary::NEXT)
        .first()
        .and_then(|id| w.content.adversary_index(id))
        .map(|i| &w.content.adversary[i])
    else {
        return Vec::new();
    };
    let twists: Vec<&Twist> = card.twists.iter().collect();
    let mut plan = build_plan(w, g, &twists, None);
    if card.escalation {
        let n = w.int("adversary.escalation_ambassadors");
        if n > 0 {
            for h in (0..g.hexes.len()).filter(|&h| is_base(w, h)) {
                match plan.binary_search_by_key(&h, |&(x, _)| x) {
                    Ok(i) => plan[i].1 += n,
                    Err(i) => plan.insert(i, (h, n)),
                }
            }
        }
    }
    plan
}
