//! What every [`Op`] does: the one place an instant op's meaning is
//! dispatched, whichever card, threat or board prints it. [`Ctx`] says who
//! chooses, who is hit and where (see `op.rs`). Rules that belong to a module
//! stay there; an arm here calls them. Standing and twist ops are read by the
//! rule they change and never resolve.

use crate::content::{Cond, CountryRule, DamageAt, HexKind, Op, Side};
use crate::module::{Cx, Res, World};
use crate::state::GameState;

use super::common::*;
use super::{adversary, ai, alignment, board, cards, economy, threats};

/// Who is resolving an op.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// A player's card, ability, growth option or power: that seat chooses.
    Seat(usize),
    /// An executing threat: seat 0 chooses for the team, the rest by scope.
    Threat,
    /// A Unity or Discord card or a project: seat 0 chooses for the team.
    Team,
}

/// The context an op resolves in.
#[derive(Debug)]
pub struct Ctx {
    pub source: Source,
    /// Where it applies: a threat's country, or the whole board.
    pub scope: Option<usize>,
    /// What earlier effects of the same card chose (the other player).
    pub other: Option<usize>,
    /// The kind of source, for tallying what it destroys.
    pub label: &'static str,
}

impl Ctx {
    fn new(source: Source, scope: Option<usize>) -> Ctx {
        Ctx { source, scope, other: None, label: "card" }
    }

    /// Names the kind of source, for tallies (`discord`, `threat2`, ...).
    pub fn named(mut self, label: &'static str) -> Ctx {
        self.label = label;
        self
    }

    pub fn seat(seat: usize) -> Ctx {
        Ctx::new(Source::Seat(seat), None)
    }

    pub fn threat(scope: Option<usize>) -> Ctx {
        Ctx::new(Source::Threat, scope)
    }

    pub fn team() -> Ctx {
        Ctx::new(Source::Team, None)
    }

    /// The seat that answers a choice.
    fn chooser(&self) -> usize {
        match self.source {
            Source::Seat(s) => s,
            Source::Threat | Source::Team => 0,
        }
    }

    /// The seats an effect on "players" hits.
    fn seats(&self, w: &World, g: &GameState) -> Vec<usize> {
        match self.source {
            Source::Seat(s) => vec![s],
            Source::Threat | Source::Team => seats_in_scope(w, g, self.scope),
        }
    }

    /// The hex sets an effect acts in: the scope as one, or with
    /// `each_country` one per country in it.
    fn regions(&self, w: &World, each_country: bool) -> Vec<Vec<usize>> {
        if each_country {
            scope_countries(w, self.scope)
                .into_iter()
                .map(|c| scope_hexes(w, Some(c)))
                .collect()
        } else {
            vec![scope_hexes(w, self.scope)]
        }
    }

    /// `threat` from a threat, else `team`: the two wordings of a shared op.
    fn says<T>(&self, threat: T, team: T) -> T {
        if self.source == Source::Threat {
            threat
        } else {
            team
        }
    }
}

/// Hexes a removal effect of seat `s` may target: every card's map target
/// is within the seat's reach (016), and the card may narrow it further.
pub fn damage_hexes(w: &World, g: &GameState, seat: usize, at: DamageAt) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| has_adversary(g, h) && in_reach(w, g, seat, h))
        .filter(|&h| match at {
            DamageAt::Reach => true,
            DamageAt::PresenceCountry => {
                has_presence_in(w, g, seat, w.map.hexes[h].country as usize)
            }
        })
        .collect()
}

/// A corrupted research station or factory, or with `cities` (024: tier-2
/// ultimates only) a corrupted city: a cleanse target.
fn cleansable(w: &World, g: &GameState, h: usize, cities: bool) -> bool {
    corrupted(g, h)
        && ((cities && w.map.hexes[h].kind == HexKind::City)
            || g.tokens(h, STATION) > 0
            || g.tokens(h, FACTORY) > 0)
}

/// Corrupted cities in seat `s`'s reach with an adversary ambassador on or
/// next to them (a `clear_city` target, 024).
fn guarded_cities(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| {
            is_city(w, h) && corrupted(g, h) && in_reach(w, g, seat, h) && board::guarded(w, g, h)
        })
        .collect()
}

/// Free industrial zones in seat `s`'s reach (a card's factory site).
fn factory_sites(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    free_zones(w, g)
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
    g.resource(seat, Resource::Ambassadors) + board::ambassadors_on_board(g, seat)
}

/// Hexes holding seat `s`'s ambassadors on the board, in hex order.
fn ambassador_hexes(g: &GameState, seat: usize) -> Vec<usize> {
    let k = ambassador(seat);
    (0..g.hexes.len()).filter(|&h| g.tokens(h, &k) > 0).collect()
}

/// Moves one of seat `s`'s ambassadors from hex `from` onto hex `to` (021:
/// movement keywords); it arrives as in rules.md §2.
fn walk(g: &mut GameState, cx: &mut Cx, seat: usize, from: usize, to: usize) {
    cx.add_tokens(g, from, &ambassador(seat), -1);
    board::place_ambassador(g, cx, seat, to);
    cx.tally(g, "effects.ambassador_moves", 1);
}

/// Whether every targeted effect in `effects` has something to act on
/// (a card is not offered when it would do nothing).
pub fn feasible(w: &World, g: &GameState, seat: usize, effects: &[Op]) -> bool {
    effects.iter().all(|e| match e {
        Op::Damage { at, .. } => !damage_hexes(w, g, seat, *at).is_empty(),
        Op::DamageCountry { .. } | Op::DestroyAll => any_hex(g, |h| has_adversary(g, h)),
        Op::RemoveNode | Op::CutNode => {
            any_hex(g, |h| g.tokens(h, NODE) > 0 && in_reach(w, g, seat, h))
        }
        Op::AddTokens { kind, .. } => !cards::token_cards(w, g, seat, Some(*kind)).is_empty(),
        Op::Consolidate { .. } => ambassador_hexes(g, seat)
            .iter()
            .any(|&h| in_reach(w, g, seat, h)),
        Op::RemoveAmbassadorsCountry { .. } => any_hex(g, |h| g.tokens(h, ADV_AMBASSADOR) > 0),
        Op::BuildFactory => !factory_sites(w, g, seat).is_empty(),
        Op::Cleanse { cities, .. } => {
            any_hex(g, |h| cleansable(w, g, h, *cities) && in_reach(w, g, seat, h))
        }
        Op::ClearCity { .. } => !guarded_cities(w, g, seat).is_empty(),
        Op::Jam { .. } | Op::CounterThreat => !threats::active(g).is_empty(),
        Op::DrainProject { .. } => adversary::projects(g)
            .iter()
            .any(|p| g.var(&adversary::project_key(p)) > 0),
        Op::DiscardProject => !adversary::projects(g).is_empty(),
        Op::DrainAll { .. } => adversary::projects(g)
            .iter()
            .any(|p| g.var(&adversary::project_key(p)) > 0),
        // 024: too few to count, nothing to cash in: the card would do nothing.
        Op::GainPerTokens { kind, per, .. } => {
            let cards = cards::token_cards(w, g, seat, Some(*kind));
            cards.iter().map(|c| g.var(&cards::tokens_key(seat, c))).sum::<i64>() >= *per
        }
        Op::CashOut { kind, .. } => cards::token_cards(w, g, seat, Some(*kind))
            .iter()
            .any(|c| g.var(&cards::tokens_key(seat, c)) > 0),
        Op::MoveAmbassadors { .. } => movable_ambassadors(g, seat) > 0,
        Op::PlacePresence { .. } => {
            ai::can_place(w, g, seat) && !board::presence_hexes(w, g, seat).is_empty()
        }
        // Giving needs someone to give to (Grant and Venture Capital still pay you).
        Op::Transfer { .. } => g.seats.len() > 1,
        // 023: the branch that would fire needs its targets (was always true).
        Op::If {
            cond: c,
            then,
            otherwise,
        } => feasible(w, g, seat, if cond(w, g, seat, *c) { then } else { otherwise }),
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

/// Resolves `ops` in order.
pub fn run(g: &mut GameState, cx: &mut Cx, ops: &[Op], ctx: &mut Ctx) -> Res {
    for e in ops {
        one(g, cx, e, ctx)?;
        if g.is_over() {
            break;
        }
    }
    Ok(())
}

fn one(g: &mut GameState, cx: &mut Cx, e: &Op, ctx: &mut Ctx) -> Res {
    let w = cx.world;
    let seat = ctx.chooser();
    match e {
        Op::Gain { resource, amount } => {
            for s in ctx.seats(&w, g) {
                cx.add_resource(g, s, *resource, *amount);
            }
        }
        Op::GainPerPresence { resource, max } => {
            let n = presence_count(g, seat).min(*max);
            cx.add_resource(g, seat, *resource, n);
        }
        Op::ChoosePlayer => {
            let others: Vec<usize> = (0..g.seats.len()).filter(|&s| s != seat).collect();
            let opts = others.iter().map(|&s| seat_opt(s)).collect();
            let prompt = format!("Seat {seat}: which other player?");
            ctx.other = cx.choose(seat, &prompt, opts)?.map(|i| others[i]);
        }
        Op::OtherGain { resource, amount } => {
            if let Some(o) = ctx.other {
                cx.add_resource(g, o, *resource, *amount);
            }
        }
        Op::Transfer {
            resource,
            max,
            bonus,
        } => {
            if let Some(o) = ctx.other {
                let most = (*max).min(g.resource(seat, *resource)).max(0);
                let opts = (0..=most)
                    .map(|k| opt(format!("give {}", resource.amount(k))))
                    .collect();
                let prompt = format!("Seat {seat}: give how much {resource} to seat {o}?");
                let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
                if k > 0 {
                    cx.add_resource(g, seat, *resource, -k);
                    cx.add_resource(g, o, *resource, k + bonus);
                }
            }
        }
        Op::Convert {
            from,
            to,
            max,
            rate,
        } => {
            let most = (*max).min(g.resource(seat, *from)).max(0);
            let opts = (0..=most)
                .map(|k| opt(format!("convert {}", from.amount(k))))
                .collect();
            let prompt = format!("Seat {seat}: convert how much {from} into {to}?");
            let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
            if k > 0 {
                cx.add_resource(g, seat, *from, -k);
                cx.add_resource(g, seat, *to, k * rate);
            }
        }
        Op::PlacePresence { count } => {
            for _ in 0..*count {
                if !board::place_presence(g, cx, seat)? {
                    break;
                }
            }
        }
        Op::Damage { amount, at } => {
            let hexes = damage_hexes(&w, g, seat, *at);
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt =
                format!("Seat {seat}: remove {amount} worth of adversary pieces in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::damage(g, cx, seat, hexes[i], *amount)?;
            }
        }
        Op::DamageCountry { amount } => {
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
        Op::DestroyAll => {
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
        Op::RemoveNode => {
            let hexes: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| g.tokens(h, NODE) > 0 && in_reach(&w, g, seat, h))
                .collect();
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: remove the node in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::remove_node(g, cx, hexes[i]);
            }
        }
        Op::RemoveAmbassadorsCountry { unity_per } => {
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
        Op::BuildFactory => {
            let hexes = factory_sites(&w, g, seat);
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: build a factory on which zone?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                cx.add_tokens(g, hexes[i], FACTORY, 1);
                cx.add_tokens(g, hexes[i], &cube(seat), 1);
                cx.tally(g, "board.factories_built.player", 1);
            }
        }
        Op::Cleanse {
            amount,
            one_country,
            cities,
        } => {
            let mut country: Option<u8> = None;
            for k in 0..*amount {
                let hexes: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| cleansable(&w, g, h, *cities) && in_reach(&w, g, seat, h))
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
                cx.tally(g, "effects.cleansed", 1);
                if is_city(&w, hexes[i]) {
                    board::liberate(g, cx, hexes[i]);
                } else {
                    cx.add_tokens(g, hexes[i], ADV_CUBE, -1);
                }
                if *one_country {
                    country = Some(w.map.hexes[hexes[i]].country);
                }
            }
        }
        Op::ClearCity { max } => {
            let cities = guarded_cities(&w, g, seat);
            let opts = cities.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: clear adversary ambassadors around which corrupted city?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let city = cities[i];
                let mut left = if *max > 0 { *max } else { i64::MAX };
                let mut hexes = w.map.hexes[city].neighbours.to_vec();
                hexes.sort_unstable();
                hexes.push(city);
                for h in hexes {
                    while left > 0 && g.tokens(h, ADV_AMBASSADOR) > 0 {
                        adversary::remove_piece(g, cx, Some(seat), h, ADV_AMBASSADOR);
                        left -= 1;
                    }
                }
            }
        }
        Op::Harden { count, in_country } => {
            let cs: Vec<usize> = (0..w.map.countries.len())
                .filter(|&c| match in_country {
                    CountryRule::Any => true,
                    CountryRule::PresenceCountry => has_presence_in(&w, g, seat, c),
                })
                .filter(|&c| {
                    w.map.countries[c]
                        .hexes
                        .iter()
                        .any(|&h| board::hardenable(g, h) && in_reach(&w, g, seat, h))
                })
                .collect();
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt = format!("Seat {seat}: harden {count} buildings in which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                // 022: only buildings a token can protect, each at most once
                // ("harden 2 buildings" is two buildings; fewer if fewer exist).
                let mut hexes: Vec<usize> = w.map.countries[cs[i]]
                    .hexes
                    .iter()
                    .copied()
                    .filter(|&h| board::hardenable(g, h) && in_reach(&w, g, seat, h))
                    .collect();
                for k in 0..*count {
                    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                    let prompt = format!(
                        "Seat {seat}: hardening {} of {count} on which building?",
                        k + 1
                    );
                    let Some(j) = cx.choose(seat, &prompt, opts)? else {
                        break;
                    };
                    cx.add_tokens(g, hexes.remove(j), HARDEN, 1);
                }
            }
        }
        Op::MoveAmbassadors { max } => {
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
        Op::GainAmbassadors { count } => cx.add_resource(g, seat, Resource::Ambassadors, *count),
        Op::Shelter => {
            let cs: Vec<usize> = (0..w.map.countries.len()).collect();
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt = format!("Seat {seat}: shelter the buildings of which country?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                board::shelter(g, cx, cs[i]);
            }
        }
        Op::Peek { count } => adversary::peek(g, cx, *count),
        Op::Jam { amount } => {
            let active = threats::active(g);
            let opts = active.iter().map(|(t, _)| opt(t.clone())).collect();
            let prompt = format!("Seat {seat}: jam which threat?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                threats::jam(g, cx, &active[i].0, *amount);
            }
        }
        Op::CounterThreat => {
            let active = threats::active(g);
            let opts = active.iter().map(|(t, _)| opt(t.clone())).collect();
            let prompt = match ctx.source {
                Source::Seat(_) => format!("Seat {seat}: counter which threat?"),
                _ => "Unity: counter which threat?".to_string(),
            };
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                threats::counter(g, cx, &active[i].0);
            }
        }
        Op::DrainProject { amount } => {
            let ids: Vec<crate::key::Key> = adversary::projects(g)
                .into_iter()
                .filter(|p| g.var(&adversary::project_key(p)) > 0)
                .collect();
            let opts = ids.iter().map(|p| opt(p.clone())).collect();
            let prompt = format!("Seat {seat}: remove compute from which adversary project?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::drain_project(g, cx, &ids[i], *amount);
            }
        }
        Op::DiscardProject => {
            let ids = adversary::projects(g);
            let opts = ids.iter().map(|p| opt(p.clone())).collect();
            let prompt = format!("Seat {seat}: discard which adversary project?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                adversary::discard_project(g, cx, &ids[i]);
            }
        }
        Op::DrainAll { amount } => {
            for p in adversary::projects(g) {
                adversary::drain_project(g, cx, &p, *amount);
            }
        }
        Op::Draw { count } => cards::draw(g, cx, seat, *count)?,
        Op::GainPerTokens {
            resource,
            kind,
            per,
            max,
        } => {
            let held: i64 = cards::token_cards(&w, g, seat, Some(*kind))
                .iter()
                .map(|c| g.var(&cards::tokens_key(seat, c)))
                .sum();
            cx.add_resource(g, seat, *resource, (held / (*per).max(1)).min(*max));
        }
        Op::CashOut {
            kind,
            resource,
            max,
        } => {
            let mut n = 0;
            for c in cards::token_cards(&w, g, seat, Some(*kind)) {
                let mut k = g.var(&cards::tokens_key(seat, &c));
                if *max > 0 {
                    k = k.min(*max - n);
                }
                cards::add_tokens(g, cx, seat, &c, -k);
                n += k;
            }
            cx.add_resource(g, seat, *resource, n * (n + 1) / 2);
        }
        Op::Sway { side, amount } => alignment::sway(g, cx, *side, *amount),
        Op::NextRoundInsight { amount } => economy::add_bonus_insight(g, cx, *amount),
        Op::NextBuildingDiscount { amount } => {
            cx.add_key(g, &cards::next_discount(seat), *amount)
        }
        Op::Production { resource, amount } => {
            cx.add_key(g, &economy::production_key(seat, *resource), *amount)
        }
        Op::AddTokens { kind, amount } => {
            let ids = cards::token_cards(&w, g, seat, Some(*kind));
            let opts = ids.iter().map(|c| opt(c.clone())).collect();
            let prompt = format!("Seat {seat}: add {amount} {kind} to which card?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                cards::add_tokens(g, cx, seat, &ids[i], *amount);
            }
        }
        Op::MoveOne { range } => {
            let from = ambassador_hexes(g, seat);
            let mut opts: Vec<_> = from.iter().map(|&h| hex_opt(&w, h)).collect();
            opts.push(opt("stop"));
            let prompt = format!("Seat {seat}: move which of your ambassadors (up to {range} hexes)?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                if let Some(&a) = from.get(i) {
                    let dest: Vec<usize> = (0..g.hexes.len())
                        .filter(|&h| {
                            h != a && has_piece(&w, g, h) && w.map.distance(a, h) as i64 <= *range
                        })
                        .collect();
                    let opts = dest.iter().map(|&h| hex_opt(&w, h)).collect();
                    let prompt = format!("Seat {seat}: move it to which piece?");
                    if let Some(j) = cx.choose(seat, &prompt, opts)? {
                        walk(g, cx, seat, a, dest[j]);
                    }
                }
            }
        }
        Op::SpreadAmbassadors => {
            // Each ambassador on the board when it starts, in hex order.
            let mut each: Vec<usize> = Vec::new();
            for h in ambassador_hexes(g, seat) {
                for _ in 0..g.tokens(h, &ambassador(seat)) {
                    each.push(h);
                }
            }
            for a in each {
                let dest: Vec<usize> = w.map.hexes[a]
                    .neighbours
                    .iter()
                    .copied()
                    .filter(|&h| has_piece(&w, g, h))
                    .collect();
                let mut opts = vec![opt("stay")];
                opts.extend(dest.iter().map(|&h| hex_opt(&w, h)));
                let prompt = format!("Seat {seat}: spread your ambassador at {}?", hex_name(&w, a));
                if let Some(j) = cx.choose(seat, &prompt, opts)? {
                    if j > 0 && g.tokens(a, &ambassador(seat)) > 0 {
                        walk(g, cx, seat, a, dest[j - 1]);
                    }
                }
            }
        }
        Op::Consolidate { max } => {
            let dest: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| in_reach(&w, g, seat, h) && has_piece(&w, g, h))
                .collect();
            let opts = dest.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: consolidate up to {max} ambassadors onto which piece?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                let to = dest[i];
                for _ in 0..*max {
                    let from: Vec<usize> = ambassador_hexes(g, seat)
                        .into_iter()
                        .filter(|&h| h != to && in_reach(&w, g, seat, h))
                        .collect();
                    if from.is_empty() {
                        break;
                    }
                    let mut opts: Vec<_> = from.iter().map(|&h| hex_opt(&w, h)).collect();
                    opts.push(opt("stop"));
                    let prompt = format!("Seat {seat}: bring which ambassador to {}?", hex_name(&w, to));
                    let Some(j) = cx.choose(seat, &prompt, opts)? else {
                        break;
                    };
                    let Some(&a) = from.get(j) else {
                        break;
                    };
                    walk(g, cx, seat, a, to);
                }
            }
        }
        Op::CutNode => {
            let hexes: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| g.tokens(h, NODE) > 0 && in_reach(&w, g, seat, h))
                .collect();
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: cut off the node in which hex?");
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                cx.set_key(g, &adversary::cut_key(hexes[i]), 1);
                cx.tally(g, "effects.nodes_cut", 1);
            }
        }
        Op::If {
            cond: c,
            then,
            otherwise,
        } => {
            let branch = if cond(&w, g, seat, *c) {
                then
            } else {
                otherwise
            };
            run(g, cx, branch, ctx)?;
        }

        // Threats and the team: the wordings differ, the rule doesn't.
        Op::Troops { count, each_country, destroy } => {
            let prompt = ctx.says(
                "Threat: an adversary troop lands on which hex?",
                "An adversary troop lands on which hex?",
            );
            let tally = ctx.says("threats.troops", "adversary.project_troops");
            for hexes in ctx.regions(&w, *each_country) {
                let kills = destroy.then_some(ctx.label);
                adversary::land_troops(g, cx, &hexes, *count, prompt, tally, kills)?;
            }
        }
        Op::FlipStation { each_country } => {
            let prompt = ctx.says(
                "Threat: which research station flips to the adversary?",
                "Which research station flips to the adversary?",
            );
            for hexes in ctx.regions(&w, *each_country) {
                let spots: Vec<usize> = hexes
                    .into_iter()
                    .filter(|&h| g.tokens(h, STATION) > 0 && !corrupted(g, h))
                    .collect();
                let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
                if let Some(i) = cx.choose(0, prompt, opts)? {
                    adversary::corrupt_station(g, cx, spots[i]);
                }
            }
        }
        Op::CorruptCity => {
            for c in scope_countries(&w, ctx.scope) {
                let city = |h: usize| is_city(&w, h) && !corrupted(g, h);
                if let Some(h) = adversary::most_infiltrated_city(&w, g, c, city) {
                    adversary::corrupt_city(g, cx, h);
                }
            }
        }
        Op::PlayersLose { resource, amount } => {
            for s in ctx.seats(&w, g) {
                let k = (*amount).min(g.resource(s, *resource));
                cx.add_resource(g, s, *resource, -k);
            }
        }
        Op::AdversaryBuild => adversary::build_all(g, cx, ctx.scope)?,
        Op::DestroyPresence => {
            let threat = |g: &GameState, h: usize| g.tokens(h, ADV_AMBASSADOR) + g.tokens(h, TROOP);
            // `max_by_key` takes the last of equals: walk from the top.
            let top = scope_hexes(&w, ctx.scope)
                .into_iter()
                .rev()
                .filter(|&h| presence_owner(g, h).is_some())
                .max_by_key(|&h| threat(g, h));
            if let Some(h) = top {
                for s in 0..g.seats.len() {
                    if g.tokens(h, &presence(s)) > 0 {
                        board::destroy_building(g, cx, h, &presence(s), board::Cause::Effect(ctx.label));
                    }
                }
            }
        }
        Op::EachDestroyPresence { count } => {
            let hexes = scope_hexes(&w, ctx.scope);
            for s in ctx.seats(&w, g) {
                for _ in 0..*count {
                    board::destroy_own_presence(g, cx, s, &hexes, board::Cause::Effect(ctx.label))?;
                }
            }
        }
        Op::PullUnity { amount } => alignment::pull_unity(g, cx, *amount),
        Op::LoseInsight { amount } => {
            for s in ctx.seats(&w, g) {
                cx.add_key(g, &economy::insight_debt(s), *amount);
                cx.tally(g, "effects.insight_lost", *amount);
            }
        }

        // Threats.
        Op::Ambassadors { at, count } => threats::place_ambassadors(g, cx, ctx.scope, *at, *count)?,
        Op::Flag { name } => {
            // Threats execute after production and upkeep: the flag lasts through
            // the next round (cleanup counts it down), or it would never land (020).
            for c in scope_countries(&w, ctx.scope) {
                cx.set_key(g, &board::flag(*name, c), 2);
            }
        }
        Op::Requisition { resource, amount, cards: n } => {
            threats::requisition(g, cx, ctx.scope, *resource, *amount, *n)?
        }
        Op::LoseAmbassador => threats::lose_ambassador(g, cx, ctx.scope)?,
        Op::LosePresenceOr { resource, amount } => {
            threats::lose_presence_or(g, cx, ctx.scope, *resource, *amount, ctx.label)?
        }
        Op::DiscardMostExpensive => threats::discard_most_expensive(g, cx, ctx.scope)?,
        Op::Compute { amount } => adversary::gain_compute(g, cx, *amount)?,
        Op::LoseTokens { amount } => threats::lose_tokens(g, cx, ctx.scope, *amount)?,

        // The team.
        Op::TeamIncome { resource, amount } => alignment::add_team_income(g, cx, *resource, *amount),
        Op::FreePurchases { count } => alignment::add_free_purchases(g, cx, *count),
        Op::CounterAll => {
            for (t, _) in threats::active(g) {
                threats::counter(g, cx, &t);
            }
        }
        Op::Pushback { ambassadors } => {
            for c in 0..w.map.countries.len() {
                for _ in 0..*ambassadors {
                    adversary::pushback(g, cx, c)?;
                }
            }
        }
        Op::DealThreats { count } => threats::deal_extra(g, cx, *count),
        Op::Infiltrate { amount } => adversary::infiltrate(g, cx, *amount),
        Op::AdversarySpread => adversary::spread(g, cx, None)?,
        Op::Sacrifice => {
            for s in ctx.seats(&w, g) {
                alignment::sacrifice(g, cx, s)?;
            }
        }
        Op::NodeAmbassadors { amount } => {
            let nodes: Vec<(usize, i64)> = (0..g.hexes.len())
                .map(|h| (h, g.tokens(h, NODE)))
                .filter(|&(_, n)| n > 0)
                .collect();
            for (h, n) in nodes {
                adversary::place(g, cx, h, n * amount);
            }
        }
        Op::CleanCityAmbassadors { amount } => {
            let cities: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| clean_city(&w, g, h))
                .collect();
            for h in cities {
                adversary::place(g, cx, h, *amount);
            }
        }
        Op::RemoveNodes { count } => {
            for _ in 0..*count {
                let nodes: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| g.tokens(h, NODE) > 0)
                    .collect();
                let opts = nodes.iter().map(|&h| hex_opt(&w, h)).collect();
                let Some(i) = cx.choose(0, "Remove the node in which hex?", opts)? else {
                    break;
                };
                adversary::remove_node(g, cx, nodes[i]);
            }
        }

        // Read, not resolved; `Content::check` keeps them out of resolving lists.
        Op::Income { .. }
        | Op::IncomePerTag { .. }
        | Op::IncomeIfTags { .. }
        | Op::BuildingDiscount { .. }
        | Op::HardenAction
        | Op::Slots { .. }
        | Op::Logistics { .. }
        | Op::UnityBonus { .. }
        | Op::GiveBonus { .. }
        | Op::SpreadShield
        | Op::CivicShelter
        | Op::TagsFromHand { .. }
        | Op::ConvertUpkeep { .. }
        | Op::CityBuild { .. }
        | Op::CleanFactories { .. }
        | Op::StationBuild { .. }
        | Op::SpreadReach { .. }
        | Op::CorruptSooner { .. }
        | Op::Arrivals { .. }
        | Op::War => unreachable!("{e:?} is read, not resolved"),
    }
    Ok(())
}
