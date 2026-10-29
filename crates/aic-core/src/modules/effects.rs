//! The player effect primitives ([`Effect`]): what cards, building
//! abilities and growth options do. Any choice an effect needs is asked of
//! the acting seat with `Cx::choose`.

use crate::content::{Cond, CountryRule, DamageAt, Effect, HexKind, Side};
use crate::module::{Cx, Res, World};
use crate::state::GameState;

use super::common::*;
use super::{adversary, ai, alignment, board, economy, threats};

/// What earlier effects of the same card chose (the other player).
#[derive(Debug, Default)]
pub struct Ctx {
    pub other: Option<usize>,
}

fn next_discount(seat: usize) -> String {
    format!("cards.next_discount.{seat}")
}

/// Hexes a removal effect of seat `s` may target.
pub fn damage_hexes(w: &World, g: &GameState, seat: usize, at: DamageAt) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| has_adversary(g, h))
        .filter(|&h| match at {
            DamageAt::Any => true,
            DamageAt::Reach => in_reach(w, g, seat, h),
            DamageAt::DataCentreCountry => has_dc_in(w, g, seat, w.map.hexes[h].country as usize),
        })
        .collect()
}

/// Empty open hexes touching a city: where a factory may be built.
pub fn factory_hexes(w: &World, g: &GameState) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| buildable(w, g, h) && w.map.hexes[h].neighbours.iter().any(|&x| is_city(w, x)))
        .collect()
}

/// A corrupted city or research station (a cleanse target).
fn cleansable(w: &World, g: &GameState, h: usize) -> bool {
    corrupted(g, h) && (w.map.hexes[h].kind == HexKind::City || g.tokens(h, STATION) > 0)
}

fn countries_where(w: &World, f: impl Fn(usize) -> bool) -> Vec<usize> {
    (0..w.map.countries.len())
        .filter(|&c| w.map.countries[c].hexes.iter().any(|&h| f(h)))
        .collect()
}

fn any_hex(g: &GameState, f: impl Fn(usize) -> bool) -> bool {
    (0..g.hexes.len()).any(f)
}

/// Whether every targeted effect in `effects` has something to act on
/// (a card is not offered when it would do nothing).
pub fn feasible(w: &World, g: &GameState, seat: usize, effects: &[Effect]) -> bool {
    effects.iter().all(|e| match e {
        Effect::Damage { at, .. } => !damage_hexes(w, g, seat, *at).is_empty(),
        Effect::DamageCountry { .. } | Effect::DestroyAll => any_hex(g, |h| has_adversary(g, h)),
        Effect::RemoveAmbassadorsCountry { .. } => any_hex(g, |h| g.tokens(h, ADV_AMBASSADOR) > 0),
        Effect::BuildFactory => !factory_hexes(w, g).is_empty(),
        Effect::Cleanse { .. } => any_hex(g, |h| cleansable(w, g, h)),
        Effect::Jam { .. } | Effect::CounterThreat => !threats::active(g).is_empty(),
        Effect::MoveAmbassadors { .. } => g.resource(seat, AMBASSADORS) + board::ambassadors_on_board(g, seat) > 0,
        Effect::PlaceDataCentre { .. } => ai::can_place(w, g, seat) && !ai::dc_hexes(w, g, seat).is_empty(),
        // Giving needs someone to give to (Grant and Venture Capital still pay you).
        Effect::Transfer { .. } => g.seats.len() > 1,
        _ => true,
    })
}

fn cond(w: &World, g: &GameState, seat: usize, c: Cond) -> bool {
    match c {
        Cond::RemovedTroop => g.var(&adversary::removed_troop(seat)) > 0,
        Cond::CityWithAmbassador => any_hex(g, |h| g.tokens(h, ADV_AMBASSADOR) > 0 && clean_city(w, g, h)),
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
        Effect::GainPerDataCentre { resource, max } => {
            let n = dc_count(g, seat).min(*max);
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
        Effect::Transfer { resource, max, bonus } => {
            if let Some(o) = ctx.other {
                let most = (*max).min(g.resource(seat, resource)).max(0);
                let opts = (0..=most).map(|k| opt(format!("give {k} {resource}"))).collect();
                let prompt = format!("Seat {seat}: give how much {resource} to seat {o}?");
                let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
                if k > 0 {
                    cx.add_resource(g, seat, resource, -k);
                    cx.add_resource(g, o, resource, k + bonus);
                }
            }
        }
        Effect::Convert { from, to, max, rate } => {
            let most = (*max).min(g.resource(seat, from)).max(0);
            let opts = (0..=most).map(|k| opt(format!("convert {k} {from}"))).collect();
            let prompt = format!("Seat {seat}: convert how much {from} into {to}?");
            let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
            if k > 0 {
                cx.add_resource(g, seat, from, -k);
                cx.add_resource(g, seat, to, k * rate);
            }
        }
        Effect::PlaceDataCentre { count } => {
            for _ in 0..*count {
                if !ai::place_data_centre(g, cx, seat)? {
                    break;
                }
            }
        }
        Effect::Damage { amount, at } => {
            let hexes = damage_hexes(&w, g, seat, *at);
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: remove {amount} worth of adversary pieces in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::damage(g, cx, seat, hexes[i], *amount)?;
            }
        }
        Effect::DamageCountry { amount } => {
            let cs = countries_where(&w, |h| has_adversary(g, h));
            let opts = cs.iter().map(|&c| country_opt(c)).collect();
            let prompt = format!("Seat {seat}: remove {amount} worth in each adversary hex of which country?");
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
            let hexes: Vec<usize> = (0..g.hexes.len()).filter(|&h| has_adversary(g, h)).collect();
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: remove every adversary ambassador and troop in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::destroy_all(g, cx, seat, hexes[i]);
            }
        }
        Effect::RemoveAmbassadorsCountry { unity_per } => {
            let cs = countries_where(&w, |h| g.tokens(h, ADV_AMBASSADOR) > 0);
            let opts = cs.iter().map(|&c| country_opt(c)).collect();
            let prompt = format!("Seat {seat}: remove every adversary ambassador in which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let mut removed = 0;
                for &h in &w.map.countries[cs[i]].hexes {
                    let n = g.tokens(h, ADV_AMBASSADOR);
                    removed += adversary::remove(g, cx, h, n);
                }
                cx.tally(g, "adversary.ambassadors_removed", removed);
                if *unity_per > 0 {
                    alignment::sway(g, cx, Side::Unity, removed / unity_per);
                }
            }
        }
        Effect::BuildFactory => {
            let hexes = factory_hexes(&w, g);
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: build a factory on which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                cx.add_tokens(g, hexes[i], FACTORY, 1);
            }
        }
        Effect::Cleanse { amount, one_country } => {
            let mut country: Option<u8> = None;
            for k in 0..*amount {
                let hexes: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| cleansable(&w, g, h))
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
                if *one_country {
                    country = Some(w.map.hexes[hexes[i]].country);
                }
            }
        }
        Effect::Harden { count, in_country } => {
            let cs: Vec<usize> = (0..w.map.countries.len())
                .filter(|&c| match in_country {
                    CountryRule::Any => true,
                    CountryRule::DataCentreCountry => has_dc_in(&w, g, seat, c),
                })
                .filter(|&c| w.map.countries[c].hexes.iter().any(|&h| has_building(g, h)))
                .collect();
            let opts = cs.iter().map(|&c| country_opt(c)).collect();
            let prompt = format!("Seat {seat}: harden {count} buildings in which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let hexes: Vec<usize> =
                    w.map.countries[cs[i]].hexes.iter().copied().filter(|&h| has_building(g, h)).collect();
                for k in 0..*count {
                    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                    let prompt = format!("Seat {seat}: hardening {} of {count} on which building?", k + 1);
                    if let Some(j) = cx.choose(seat, &prompt, opts)? {
                        cx.add_tokens(g, hexes[j], HARDEN, 1);
                    }
                }
            }
        }
        Effect::MoveAmbassadors { max } => {
            for k in 0..*max {
                if g.resource(seat, AMBASSADORS) + board::ambassadors_on_board(g, seat) == 0 {
                    break;
                }
                let dest: Vec<usize> =
                    (0..g.hexes.len()).filter(|&h| in_reach(&w, g, seat, h) && has_piece(&w, g, h)).collect();
                let mut opts: Vec<_> = dest.iter().map(|&h| hex_opt(&w, h)).collect();
                opts.push(opt("stop"));
                let prompt = format!("Seat {seat}: move ambassador {} of up to {max} to which piece?", k + 1);
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
            let opts = cs.iter().map(|&c| country_opt(c)).collect();
            let prompt = format!("Seat {seat}: shelter the buildings of which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                cx.set_key(g, board::flag("shelter", cs[i]), 1);
            }
        }
        Effect::Peek { count } => {
            let cur = g.var(adversary::PEEK);
            cx.set_key(g, adversary::PEEK.into(), cur.max(*count));
        }
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
        Effect::Unity { amount } => alignment::sway(g, cx, Side::Unity, *amount),
        Effect::NextRoundInsight { amount } => cx.add_key(g, economy::BONUS_INSIGHT.into(), *amount),
        Effect::NextBuildingDiscount { amount } => cx.add_key(g, next_discount(seat), *amount),
        Effect::If {
            cond: c,
            then,
            otherwise,
        } => {
            let branch = if cond(&w, g, seat, *c) { then } else { otherwise };
            run(g, cx, seat, branch, ctx)?;
        }
    }
    Ok(())
}
