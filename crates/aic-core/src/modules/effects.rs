//! The player effect primitives ([`Effect`]): what cards, building
//! abilities and growth options do. Any choice an effect needs is asked of
//! the acting seat with `Cx::choose`.

use crate::content::{Cond, CountryRule, DamageAt, Effect, Side, Terrain};
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

/// Hexes a damage effect of seat `s` may target.
pub fn damage_hexes(w: &World, g: &GameState, seat: usize, at: DamageAt) -> Vec<usize> {
    let mine = processor(seat);
    let own: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, &mine) > 0).collect();
    (0..g.hexes.len())
        .filter(|&h| has_adversary(g, h))
        .filter(|&h| match at {
            DamageAt::Any => true,
            DamageAt::NearProcessor => own.iter().any(|&p| w.map.distance(p, h) <= 1),
            DamageAt::ProcessorCountry => own.iter().any(|&p| w.map.hexes[p].country == w.map.hexes[h].country),
        })
        .collect()
}

/// Industrial hexes next to population (`empty`: also holding no pieces).
pub fn factory_hexes(w: &World, g: &GameState, empty: bool) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| w.map.hexes[h].terrain == Terrain::Industrial)
        .filter(|&h| next_to(w, h, |x| g.tokens(x, POPULATION) > 0))
        .filter(|&h| {
            !empty
                || [POPULATION, FACTORY, DATA_CENTRE]
                    .iter()
                    .all(|k| g.tokens(h, k) == 0)
                    && !has_adversary(g, h)
        })
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

/// Whether every targeted effect in `effects` has something to act on
/// (a card is not offered when it would do nothing).
pub fn feasible(w: &World, g: &GameState, seat: usize, effects: &[Effect]) -> bool {
    effects.iter().all(|e| match e {
        Effect::Damage { at, .. } => !damage_hexes(w, g, seat, *at).is_empty(),
        Effect::DamageCountry { .. } | Effect::DestroyAll => any_hex(g, |h| has_adversary(g, h)),
        Effect::RemoveDronesCountry { .. } => any_hex(g, |h| g.tokens(h, drone_kind(w)) > 0),
        Effect::BuildFactory { empty } => !factory_hexes(w, g, *empty).is_empty(),
        Effect::RemoveCorruption { .. } => any_hex(g, |h| corrupted(w, g, h)),
        Effect::Jam { .. } | Effect::CounterThreat => !threats::active(g).is_empty(),
        Effect::MovePopulation { .. } => any_hex(g, |h| g.tokens(h, POPULATION) > 0),
        Effect::PlaceProcessor { .. } => ai::can_place(w, g, seat),
        // Giving needs someone to give to (Grant and Venture Capital still pay you).
        Effect::Transfer { .. } => g.seats.len() > 1,
        _ => true,
    })
}

/// What "drones" means to a card: the Militarist's drones; with clusters,
/// clusters (a hex loses one); with asset corruption, nothing (a token kind
/// no hex ever holds).
fn drone_kind(w: &World) -> &'static str {
    match adversary::style(w) {
        adversary::Style::Militarist => DRONE,
        adversary::Style::Clusters => CLUSTER,
        adversary::Style::Assets => "no_drones",
    }
}

/// What "corruption" means to a card: corruption on the hex; with asset
/// corruption, a corrupted asset there (returning it reclaims the asset).
fn corrupted(w: &World, g: &GameState, h: usize) -> bool {
    match adversary::style(w) {
        adversary::Style::Assets => ROGUE.iter().any(|k| g.tokens(h, k) > 0),
        _ => g.tokens(h, CORRUPTION) > 0,
    }
}

fn cond(g: &GameState, seat: usize, c: Cond) -> bool {
    match c {
        Cond::DestroyedFoundry => g.var(&adversary::killed_foundry(seat)) > 0,
        Cond::PopulationWithAdversary => any_hex(g, |h| g.tokens(h, POPULATION) > 0 && has_adversary(g, h)),
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
        Effect::GainPerProcessor { resource, max } => {
            let n = processors_of(g, seat).min(*max);
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
        Effect::PlaceProcessor { count } => {
            for _ in 0..*count {
                if !ai::place_processor(g, cx, seat)? {
                    break;
                }
            }
        }
        Effect::Damage { amount, at } => {
            let hexes = damage_hexes(&w, g, seat, *at);
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: {amount} damage in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::damage(g, cx, seat, hexes[i], *amount)?;
            }
        }
        Effect::DamageCountry { amount } => {
            let cs = countries_where(&w, |h| has_adversary(g, h));
            let opts = cs.iter().map(|&c| country_opt(c)).collect();
            let prompt = format!("Seat {seat}: {amount} damage in each adversary hex of which country?");
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
            let prompt = format!("Seat {seat}: destroy every adversary piece in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::destroy_all(g, cx, seat, hexes[i]);
            }
        }
        Effect::RemoveDronesCountry { unity_per } => {
            let kind = drone_kind(&w);
            let cs = countries_where(&w, |h| g.tokens(h, kind) > 0);
            let opts = cs.iter().map(|&c| country_opt(c)).collect();
            let prompt = format!("Seat {seat}: remove every drone in which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let mut removed = 0;
                for &h in &w.map.countries[cs[i]].hexes {
                    // Clusters: one per hex (clusters are every adversary piece).
                    let n = if kind == DRONE { g.tokens(h, DRONE) } else { 1 };
                    removed += -cx.add_tokens(g, h, kind, -n);
                }
                if *unity_per > 0 {
                    alignment::sway(g, cx, Side::Unity, removed / unity_per);
                }
            }
        }
        Effect::BuildFactory { empty } => {
            let hexes = factory_hexes(&w, g, *empty);
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: build a factory on which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                cx.add_tokens(g, hexes[i], FACTORY, 1);
            }
        }
        Effect::RemoveCorruption { amount, one_country } => {
            let mut country: Option<u8> = None;
            for k in 0..*amount {
                let hexes: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| corrupted(&w, g, h))
                    .filter(|&h| country.is_none_or(|c| w.map.hexes[h].country == c))
                    .collect();
                if hexes.is_empty() {
                    break;
                }
                let mut opts: Vec<_> = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                if k > 0 {
                    opts.push(opt("stop"));
                }
                let prompt = format!("Seat {seat}: return corruption from which hex?");
                let Some(i) = cx.choose(seat, &prompt, opts)? else {
                    break;
                };
                if i >= hexes.len() {
                    break;
                }
                if adversary::style(&w) == adversary::Style::Assets {
                    adversary::reclaim_any(g, cx, hexes[i]);
                } else {
                    adversary::remove_corruption(g, cx, hexes[i], 1);
                }
                if *one_country {
                    country = Some(w.map.hexes[hexes[i]].country);
                }
            }
        }
        Effect::PlaceDefense { count, in_country } => {
            let cs: Vec<usize> = match in_country {
                CountryRule::Any => (0..w.map.countries.len()).collect(),
                CountryRule::ProcessorCountry => (0..w.map.countries.len())
                    .filter(|&c| has_processor_in(&w, g, seat, c))
                    .collect(),
            };
            let opts = cs.iter().map(|&c| country_opt(c)).collect();
            let prompt = format!("Seat {seat}: place {count} defense in which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let hexes = w.map.countries[cs[i]].hexes.clone();
                for k in 0..*count {
                    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                    let prompt = format!("Seat {seat}: defense {} of {count} on which hex?", k + 1);
                    if let Some(j) = cx.choose(seat, &prompt, opts)? {
                        cx.add_tokens(g, hexes[j], &defense(seat), 1);
                    }
                }
            }
        }
        Effect::MovePopulation { max } => {
            let mut country: Option<u8> = None;
            for k in 0..*max {
                let src: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| g.tokens(h, POPULATION) > 0)
                    .filter(|&h| country.is_none_or(|c| w.map.hexes[h].country == c))
                    .collect();
                if src.is_empty() {
                    break;
                }
                let mut opts: Vec<_> = src.iter().map(|&h| hex_opt(&w, h)).collect();
                opts.push(opt("stop"));
                let prompt = format!("Seat {seat}: move population {} of up to {max} from which hex?", k + 1);
                let Some(i) = cx.choose(seat, &prompt, opts)? else {
                    break;
                };
                if i >= src.len() {
                    break;
                }
                let from = src[i];
                let c = w.map.hexes[from].country;
                let dest: Vec<usize> = w.map.countries[c as usize]
                    .hexes
                    .iter()
                    .copied()
                    .filter(|&h| h != from)
                    .collect();
                let opts = dest.iter().map(|&h| hex_opt(&w, h)).collect();
                let prompt = format!("Seat {seat}: move the population from {} to which hex?", hex_name(&w, from));
                if let Some(j) = cx.choose(seat, &prompt, opts)? {
                    cx.add_tokens(g, from, POPULATION, -1);
                    cx.add_tokens(g, dest[j], POPULATION, 1);
                }
                country = Some(c);
            }
        }
        Effect::Shelter => {
            let cs: Vec<usize> = (0..w.map.countries.len()).collect();
            let opts = cs.iter().map(|&c| country_opt(c)).collect();
            let prompt = format!("Seat {seat}: shelter the population of which country?");
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
            let branch = if cond(g, seat, *c) { then } else { otherwise };
            run(g, cx, seat, branch, ctx)?;
        }
    }
    Ok(())
}
