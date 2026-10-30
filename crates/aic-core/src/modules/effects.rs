//! The player effect primitives ([`Effect`]): what cards, building
//! abilities and growth options do. Any choice an effect needs is asked of
//! the acting seat with `Cx::choose`.

use crate::content::{Cond, CountryRule, DamageAt, Effect, HexKind, Side};
use crate::module::{Cx, Res, World};
use crate::state::GameState;

use super::common::*;
use super::{adversary, ai, alignment, board, cards, economy, threats};

/// What earlier effects of the same card chose (the other player).
#[derive(Debug, Default)]
pub struct Ctx {
    pub other: Option<usize>,
}

/// Hexes a removal effect of seat `s` may target: every card's map target
/// is within the seat's reach (016), and the card may narrow it further.
pub fn damage_hexes(w: &World, g: &GameState, seat: usize, at: DamageAt) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| has_adversary(g, h) && in_reach(w, g, seat, h))
        .filter(|&h| match at {
            DamageAt::Any => true,
            DamageAt::Reach => in_reach(w, g, seat, h),
            DamageAt::PresenceCountry => {
                has_presence_in(w, g, seat, w.map.hexes[h].country as usize)
            }
        })
        .collect()
}

/// A corrupted city, research station or factory (a cleanse target).
fn cleansable(w: &World, g: &GameState, h: usize) -> bool {
    corrupted(g, h)
        && (w.map.hexes[h].kind == HexKind::City
            || g.tokens(h, STATION) > 0
            || g.tokens(h, FACTORY) > 0)
}

/// Free industrial zones in seat `s`'s reach (a card's factory site).
fn factory_sites(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    factory_hexes(w, g)
        .into_iter()
        .filter(|&h| in_reach(w, g, seat, h))
        .collect()
}

fn countries_where(w: &World, f: impl Fn(usize) -> bool) -> Vec<usize> {
    (0..w.map.countries.len())
        .filter(|&c| w.map.countries[c].hexes.iter().any(|&h| f(h)))
        .collect()
}

fn any_hex(g: &GameState, f: impl Fn(usize) -> bool) -> bool {
    (0..g.hexes.len()).any(f)
}

/// Ambassadors seat `s` could move: in its supply or on the board.
fn movable_ambassadors(g: &GameState, seat: usize) -> i64 {
    g.resource(seat, AMBASSADORS) + board::ambassadors_on_board(g, seat)
}

/// Whether every targeted effect in `effects` has something to act on
/// (a card is not offered when it would do nothing).
pub fn feasible(w: &World, g: &GameState, seat: usize, effects: &[Effect]) -> bool {
    effects.iter().all(|e| match e {
        Effect::Damage { at, .. } => !damage_hexes(w, g, seat, *at).is_empty(),
        Effect::DamageCountry { .. } | Effect::DestroyAll => any_hex(g, |h| has_adversary(g, h)),
        Effect::RemoveNode => any_hex(g, |h| g.tokens(h, NODE) > 0 && in_reach(w, g, seat, h)),
        Effect::RemoveAmbassadorsCountry { .. } => any_hex(g, |h| g.tokens(h, ADV_AMBASSADOR) > 0),
        Effect::BuildFactory => !factory_sites(w, g, seat).is_empty(),
        Effect::Cleanse { .. } => any_hex(g, |h| cleansable(w, g, h) && in_reach(w, g, seat, h)),
        Effect::Jam { .. } | Effect::CounterThreat => !threats::active(g).is_empty(),
        Effect::DrainProject { .. } => adversary::projects(g)
            .iter()
            .any(|p| g.var(&adversary::project_key(p)) > 0),
        Effect::DiscardProject => !adversary::projects(g).is_empty(),
        Effect::MoveAmbassadors { .. } => movable_ambassadors(g, seat) > 0,
        Effect::PlacePresence { .. } => {
            ai::can_place(w, g, seat) && !board::presence_hexes(w, g, seat).is_empty()
        }
        // Giving needs someone to give to (Grant and Venture Capital still pay you).
        Effect::Transfer { .. } => g.seats.len() > 1,
        _ => true,
    })
}

fn cond(w: &World, g: &GameState, seat: usize, c: Cond) -> bool {
    match c {
        Cond::RemovedTroop => g.var(&adversary::removed_troop(seat)) > 0,
        Cond::CityWithAmbassador => any_hex(g, |h| {
            g.tokens(h, ADV_AMBASSADOR) > 0 && clean_city(w, g, h)
        }),
    }
}

/// Resolves `effects` in order for `seat`.
pub fn run(g: &mut GameState, cx: &mut Cx, seat: usize, effects: &[Effect], ctx: &mut Ctx) -> Res {
    for e in effects {
        one(g, cx, seat, e, ctx)?;
        if g.is_over() {
            break;
        }
    }
    Ok(())
}

fn one(g: &mut GameState, cx: &mut Cx, seat: usize, e: &Effect, ctx: &mut Ctx) -> Res {
    let w = cx.world;
    match e {
        Effect::Gain { resource, amount } => cx.add_resource(g, seat, resource, *amount),
        Effect::GainPerPresence { resource, max } => {
            let n = presence_count(g, seat).min(*max);
            cx.add_resource(g, seat, resource, n);
        }
        Effect::ChoosePlayer => {
            let others: Vec<usize> = (0..g.seats.len()).filter(|&s| s != seat).collect();
            let opts = others.iter().map(|&s| seat_opt(s)).collect();
            let prompt = format!("Seat {seat}: which other player?");
            ctx.other = cx.choose(seat, &prompt, opts)?.map(|i| others[i]);
        }
        Effect::OtherGain { resource, amount } => {
            if let Some(o) = ctx.other {
                cx.add_resource(g, o, resource, *amount);
            }
        }
        Effect::Transfer {
            resource,
            max,
            bonus,
        } => {
            if let Some(o) = ctx.other {
                let most = (*max).min(g.resource(seat, resource)).max(0);
                let opts = (0..=most)
                    .map(|k| opt(format!("give {k} {resource}")))
                    .collect();
                let prompt = format!("Seat {seat}: give how much {resource} to seat {o}?");
                let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
                if k > 0 {
                    cx.add_resource(g, seat, resource, -k);
                    cx.add_resource(g, o, resource, k + bonus);
                }
            }
        }
        Effect::Convert {
            from,
            to,
            max,
            rate,
        } => {
            let most = (*max).min(g.resource(seat, from)).max(0);
            let opts = (0..=most)
                .map(|k| opt(format!("convert {k} {from}")))
                .collect();
            let prompt = format!("Seat {seat}: convert how much {from} into {to}?");
            let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
            if k > 0 {
                cx.add_resource(g, seat, from, -k);
                cx.add_resource(g, seat, to, k * rate);
            }
        }
        Effect::PlacePresence { count } => {
            for _ in 0..*count {
                if !board::place_presence(g, cx, seat)? {
                    break;
                }
            }
        }
        Effect::Damage { amount, at } => {
            let hexes = damage_hexes(&w, g, seat, *at);
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt =
                format!("Seat {seat}: remove {amount} worth of adversary pieces in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::damage(g, cx, seat, hexes[i], *amount)?;
            }
        }
        Effect::DamageCountry { amount } => {
            let cs = countries_where(&w, |h| has_adversary(g, h));
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt = format!(
                "Seat {seat}: remove {amount} worth in each adversary hex of which country?"
            );
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let hexes: Vec<usize> = w.map.countries[cs[i]]
                    .hexes
                    .iter()
                    .copied()
                    .filter(|&h| has_adversary(g, h))
                    .collect();
                for h in hexes {
                    adversary::damage(g, cx, seat, h, *amount)?;
                }
            }
        }
        Effect::DestroyAll => {
            let hexes: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| has_adversary(g, h))
                .collect();
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt =
                format!("Seat {seat}: remove every adversary ambassador and troop in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::destroy_all(g, cx, seat, hexes[i]);
            }
        }
        Effect::RemoveNode => {
            let hexes: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| g.tokens(h, NODE) > 0 && in_reach(&w, g, seat, h))
                .collect();
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: remove the node in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::remove_node(g, cx, hexes[i]);
            }
        }
        Effect::RemoveAmbassadorsCountry { unity_per } => {
            let cs = countries_where(&w, |h| g.tokens(h, ADV_AMBASSADOR) > 0);
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt =
                format!("Seat {seat}: remove every adversary ambassador in which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let removed = adversary::remove_all_in_country(g, cx, cs[i]);
                if *unity_per > 0 {
                    alignment::sway(g, cx, Side::Unity, removed / unity_per);
                }
            }
        }
        Effect::BuildFactory => {
            let hexes = factory_sites(&w, g, seat);
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: build a factory on which zone?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                cx.add_tokens(g, hexes[i], FACTORY, 1);
                cx.add_tokens(g, hexes[i], &cube(seat), 1);
                cx.tally(g, "board.factories_built.player", 1);
            }
        }
        Effect::Cleanse {
            amount,
            one_country,
        } => {
            let mut country: Option<u8> = None;
            for k in 0..*amount {
                let hexes: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| cleansable(&w, g, h) && in_reach(&w, g, seat, h))
                    .filter(|&h| country.is_none_or(|c| w.map.hexes[h].country == c))
                    .collect();
                if hexes.is_empty() {
                    break;
                }
                let mut opts: Vec<_> = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                if k > 0 {
                    opts.push(opt("stop"));
                }
                let prompt = format!("Seat {seat}: take the adversary cube off which hex?");
                let Some(i) = cx.choose(seat, &prompt, opts)? else {
                    break;
                };
                if i >= hexes.len() {
                    break;
                }
                cx.add_tokens(g, hexes[i], ADV_CUBE, -1);
                cx.tally(g, "effects.cleansed", 1);
                if is_city(&w, hexes[i]) {
                    alignment::for_city_freed(g, cx);
                }
                if *one_country {
                    country = Some(w.map.hexes[hexes[i]].country);
                }
            }
        }
        Effect::Harden { count, in_country } => {
            let cs: Vec<usize> = (0..w.map.countries.len())
                .filter(|&c| match in_country {
                    CountryRule::Any => true,
                    CountryRule::PresenceCountry => has_presence_in(&w, g, seat, c),
                })
                .filter(|&c| {
                    w.map.countries[c]
                        .hexes
                        .iter()
                        .any(|&h| has_building(g, h) && in_reach(&w, g, seat, h))
                })
                .collect();
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt = format!("Seat {seat}: harden {count} buildings in which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let hexes: Vec<usize> = w.map.countries[cs[i]]
                    .hexes
                    .iter()
                    .copied()
                    .filter(|&h| has_building(g, h) && in_reach(&w, g, seat, h))
                    .collect();
                for k in 0..*count {
                    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                    let prompt = format!(
                        "Seat {seat}: hardening {} of {count} on which building?",
                        k + 1
                    );
                    if let Some(j) = cx.choose(seat, &prompt, opts)? {
                        cx.add_tokens(g, hexes[j], HARDEN, 1);
                    }
                }
            }
        }
        Effect::MoveAmbassadors { max } => {
            for k in 0..*max {
                if movable_ambassadors(g, seat) == 0 {
                    break;
                }
                let dest: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| in_reach(&w, g, seat, h) && has_piece(&w, g, h))
                    .collect();
                let mut opts: Vec<_> = dest.iter().map(|&h| hex_opt(&w, h)).collect();
                opts.push(opt("stop"));
                let prompt = format!(
                    "Seat {seat}: move ambassador {} of up to {max} to which piece?",
                    k + 1
                );
                let Some(i) = cx.choose(seat, &prompt, opts)? else {
                    break;
                };
                if i >= dest.len() {
                    break;
                }
                board::move_ambassador(g, cx, seat, dest[i])?;
            }
        }
        Effect::GainAmbassadors { count } => cx.add_resource(g, seat, AMBASSADORS, *count),
        Effect::Shelter => {
            let cs: Vec<usize> = (0..w.map.countries.len()).collect();
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt = format!("Seat {seat}: shelter the buildings of which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                board::shelter(g, cx, cs[i]);
            }
        }
        Effect::Peek { count } => adversary::peek(g, cx, *count),
        Effect::Jam { amount } => {
            let active = threats::active(g);
            let opts = active.iter().map(|(t, _)| opt(t.clone())).collect();
            let prompt = format!("Seat {seat}: jam which threat?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                threats::jam(g, cx, &active[i].0, *amount);
            }
        }
        Effect::CounterThreat => {
            let active = threats::active(g);
            let opts = active.iter().map(|(t, _)| opt(t.clone())).collect();
            let prompt = format!("Seat {seat}: counter which threat?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                threats::counter(g, cx, &active[i].0);
            }
        }
        Effect::DrainProject { amount } => {
            let ids: Vec<String> = adversary::projects(g)
                .into_iter()
                .filter(|p| g.var(&adversary::project_key(p)) > 0)
                .collect();
            let opts = ids.iter().map(|p| opt(p.clone())).collect();
            let prompt = format!("Seat {seat}: remove compute from which adversary project?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::drain_project(g, cx, &ids[i], *amount);
            }
        }
        Effect::DiscardProject => {
            let ids = adversary::projects(g);
            let opts = ids.iter().map(|p| opt(p.clone())).collect();
            let prompt = format!("Seat {seat}: discard which adversary project?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::discard_project(g, cx, &ids[i]);
            }
        }
        Effect::Unity { amount } => alignment::sway(g, cx, Side::Unity, *amount),
        Effect::NextRoundInsight { amount } => economy::add_bonus_insight(g, cx, *amount),
        Effect::NextBuildingDiscount { amount } => {
            cx.add_key(g, cards::next_discount(seat), *amount)
        }
        Effect::If {
            cond: c,
            then,
            otherwise,
        } => {
            let branch = if cond(&w, g, seat, *c) {
                then
            } else {
                otherwise
            };
            run(g, cx, seat, branch, ctx)?;
        }
    }
    Ok(())
}
