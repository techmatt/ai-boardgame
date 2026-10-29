//! Threats: dealt each round, countered by pooled payments, executed if
//! still uncountered when the action phase ends. Variants: `countries`
//! (each threat sits on a country), `shared_row` (one row for every
//! country) and `off` (no threats).

use crate::action::Action;
use crate::content::{Passive, Side, Spot, ThreatDef, ThreatOp};
use crate::module::{Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{adversary, alignment, board, cards};

const PARAMS: &[ParamSpec] = &[ParamSpec::int("per_country", 0, 5, "threats dealt each round per country")];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto("deal", "deal threats face up (tier 1 until stage II begins, then tier 2)"),
    PhaseSpec::decision("counter", "pay toward threat counters (pooled; withdraw until the phase ends)"),
    PhaseSpec::auto("execute", "refund partial payments, then uncountered threats execute in order"),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "threats",
    summary: "Threats: dealt each round, countered by pooled payments, executed if not",
    requires: &["adversary", "alignment", "board", "cards"],
    variants: &[
        VariantDef {
            name: "countries",
            summary: "Each threat sits on one country; only players with a data centre there may pay",
            params: PARAMS,
            phases: PHASES,
            rng: &["threats"],
            uses_cards: false,
            build: build_countries,
        },
        VariantDef {
            name: "shared_row",
            summary: "Threats sit in one row, apply in every country, and anyone may pay",
            params: PARAMS,
            phases: PHASES,
            rng: &["threats"],
            uses_cards: false,
            build: build_shared,
        },
        VariantDef {
            name: "off",
            summary: "No threats: nothing is dealt, countered or executed (cards that act on threats have no target)",
            params: &[],
            phases: PHASES,
            rng: &[],
            uses_cards: false,
            build: build_off,
        },
    ],
};

/// Variant `off`: the threat phases stay in the round and do nothing.
#[derive(Debug)]
struct NoThreats;

impl Module for NoThreats {}

fn build_off(_: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(NoThreats))
}

pub const ROW: &str = "threats.row";

fn deck(tier: u8) -> String {
    format!("threats.deck.{tier}")
}

fn discard(tier: u8) -> String {
    format!("threats.discard.{tier}")
}

fn on(country: usize) -> String {
    format!("threats.on.{country}")
}

/// Prefix of every payment toward `threat` (by any seat, in any resource).
pub fn paid_prefix(threat: &str) -> String {
    format!("threats.paid.{threat}.")
}

fn paid_key(threat: &str, seat: usize, resource: &str) -> String {
    format!("threats.paid.{threat}.{seat}.{resource}")
}

fn jam_key(threat: &str) -> String {
    format!("threats.jam.{threat}")
}

fn def<'w>(w: &World<'w>, id: &str) -> &'w ThreatDef {
    &w.content.threats[w.content.threat_index(id).expect("threat in content")]
}

/// Active threats in execution order, each with its scope (its country, or
/// `None` for a shared-row threat).
pub fn active(g: &GameState) -> Vec<(String, Option<usize>)> {
    let mut out: Vec<(String, Option<usize>)> = g.pile(ROW).iter().map(|t| (t.clone(), None)).collect();
    let mut on: Vec<(usize, &Vec<String>)> = g
        .piles
        .iter()
        .filter_map(|(k, v)| Some((k.strip_prefix("threats.on.")?.parse().ok()?, v)))
        .collect();
    on.sort_by_key(|(c, _)| *c);
    for (c, pile) in on {
        out.extend(pile.iter().map(|t| (t.clone(), Some(c))));
    }
    out
}

fn pile_of(scope: Option<usize>) -> String {
    scope.map_or(ROW.to_string(), on)
}

/// Where a threat dealt for country `c` goes: the shared row, or on `c`.
fn deal_pile(shared: bool, c: usize) -> String {
    if shared { ROW.to_string() } else { on(c) }
}

/// Deals the top threat of the `tier` deck onto `to`, counted under
/// `threats.dealt` whichever module is running. False if the deck is empty
/// (no reshuffle here: the caller refills first if it may).
fn deal(g: &mut GameState, cx: &mut Cx, tier: u8, to: &str) -> bool {
    let Some(id) = g.pile(&deck(tier)).first().cloned() else {
        return false;
    };
    cx.move_card(g, &id, &deck(tier), to, false).unwrap();
    cx.tally(g, "threats.dealt", 1);
    true
}

/// Whether the threats variant in play deals to the shared row (`None`:
/// no threats). For rules code outside the module, which has no `Threats`.
fn shared_row(w: &World) -> Option<bool> {
    match w.variant("threats") {
        Some("countries") => Some(false),
        Some("shared_row") => Some(true),
        _ => None,
    }
}

/// What countering `threat` asks for now, resource by resource (difficulty
/// and Jam Signals applied).
pub fn requirement(w: &World, g: &GameState, threat: &str) -> Vec<(String, i64)> {
    let jam = g.var(&jam_key(threat));
    let (bonus, all) = if adversary::at(w, g, 1) {
        (w.int("adversary.counter_bonus"), w.int("adversary.counter_bonus_all"))
    } else {
        (0, 0)
    };
    def(w, threat)
        .counter
        .iter()
        .enumerate()
        .map(|(k, (r, n))| {
            let n = n + if k == 0 { bonus } else { 0 } + all - jam;
            (r.clone(), n.max(0))
        })
        .collect()
}

/// Paid so far toward `threat` in `resource`, by all seats.
fn paid(g: &GameState, threat: &str, resource: &str) -> i64 {
    (0..g.seats.len()).map(|s| g.var(&paid_key(threat, s, resource))).sum()
}

/// Returns every contribution on `threat` to its payers.
fn refund(g: &mut GameState, cx: &mut Cx, threat: &str) {
    let w = cx.world;
    for (r, _) in &def(&w, threat).counter {
        for s in 0..g.seats.len() {
            let k = paid_key(threat, s, r);
            let n = g.var(&k);
            if n > 0 {
                cx.add_resource(g, s, r, n);
                cx.set_key(g, k, 0);
            }
        }
    }
}

fn clear(g: &mut GameState, cx: &mut Cx, threat: &str, scope: Option<usize>) {
    let w = cx.world;
    let t = def(&w, threat);
    cx.set_key(g, jam_key(threat), 0);
    cx.move_card(g, threat, &pile_of(scope), &discard(t.tier), true)
        .expect("active threat");
}

/// Counters `threat` without payment (a card or a reward): partial
/// payments are refunded and it is discarded.
pub fn counter(g: &mut GameState, cx: &mut Cx, threat: &str) {
    let Some((_, scope)) = active(g).into_iter().find(|(t, _)| t == threat) else {
        return;
    };
    refund(g, cx, threat);
    clear(g, cx, threat, scope);
    cx.tally(g, "threats.countered", 1);
}

/// Deals `n` extra threats of the current tier now, one per country in turn
/// from country 0 (Discord cards). An empty deck deals nothing (no reshuffle
/// out of turn). Variant `off`: nothing.
pub fn deal_extra(g: &mut GameState, cx: &mut Cx, n: i64) {
    let w = cx.world;
    let Some(shared) = shared_row(&w) else {
        return;
    };
    let tier = adversary::threat_tier(&w, g);
    let countries = w.map.countries.len().max(1);
    for k in 0..n.max(0) as usize {
        if !deal(g, cx, tier, &deal_pile(shared, k % countries)) {
            return;
        }
    }
}

/// Jam Signals: `threat` needs `n` less of each resource.
pub fn jam(g: &mut GameState, cx: &mut Cx, threat: &str, n: i64) {
    cx.add_key(g, jam_key(threat), n);
    try_complete(g, cx, threat);
}

/// Counters `threat` if what is paid covers what it needs now: the
/// payments are spent (any excess, after Jam Signals, goes back to the
/// payers, later seats first), payers' Logistics AI pays out, and the
/// threat is discarded. Returns whether it was countered.
pub fn try_complete(g: &mut GameState, cx: &mut Cx, threat: &str) -> bool {
    let w = cx.world;
    let Some((_, scope)) = active(g).into_iter().find(|(t, _)| t == threat) else {
        return false;
    };
    let req = requirement(&w, g, threat);
    if !req.iter().all(|(r, n)| paid(g, threat, r) >= *n) {
        return false;
    }
    let payers: Vec<usize> = (0..g.seats.len())
        .filter(|&p| req.iter().any(|(r, _)| g.var(&paid_key(threat, p, r)) > 0))
        .collect();
    for (r, need) in &req {
        let mut excess = paid(g, threat, r) - need;
        for p in (0..g.seats.len()).rev() {
            let k = paid_key(threat, p, r);
            let back = excess.min(g.var(&k));
            if back > 0 {
                cx.add_resource(g, p, r, back);
                excess -= back;
            }
            cx.set_key(g, k, 0);
        }
    }
    clear(g, cx, threat, scope);
    cx.tally(g, "threats.countered", 1);
    cx.note(None, || format!("{} is countered", def(&w, threat).name));
    for p in payers {
        for pa in cards::passives(&w, g, p) {
            if let Passive::Logistics { resource, amount } = pa {
                cx.add_resource(g, p, resource, *amount);
            }
        }
    }
    true
}

#[derive(Debug)]
struct Threats {
    shared: bool,
    per_country: i64,
}

fn build_countries(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Threats {
        shared: false,
        per_country: b.int("per_country")?,
    }))
}

fn build_shared(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Threats {
        shared: true,
        per_country: b.int("per_country")?,
    }))
}

impl Threats {
    fn can_pay(&self, w: &World, g: &GameState, seat: usize, scope: Option<usize>) -> bool {
        match scope {
            Some(c) if !self.shared => has_dc_in(w, g, seat, c),
            _ => true,
        }
    }

    /// An empty `tier` deck is refilled from its shuffled discard (the
    /// round's deal only; extra deals out of turn don't reshuffle).
    fn refill(&self, g: &mut GameState, cx: &mut Cx, tier: u8) {
        if g.pile(&deck(tier)).is_empty() {
            let mut cards = g.pile(&discard(tier)).to_vec();
            if cards.is_empty() {
                return;
            }
            shuffle(&mut cards, cx.rng(g, "threats"));
            cx.set_pile(g, &discard(tier), Vec::new());
            cx.set_pile(g, &deck(tier), cards);
        }
    }
}

impl Module for Threats {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for tier in [1, 2] {
            let mut ids: Vec<String> = w
                .content
                .threats
                .iter()
                .filter(|t| t.tier == tier)
                .map(|t| t.id.clone())
                .collect();
            shuffle(&mut ids, cx.rng(g, "threats"));
            cx.set_pile(g, &deck(tier), ids);
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        if phase == 0 {
            let tier = adversary::threat_tier(&w, g);
            // Difficulty 1: `extra_threats` more a round in all, one per
            // country in turn (country index 0 first in round 1, then 1, ...).
            let n = w.map.countries.len();
            let extra = if adversary::at(&w, g, 1) {
                w.int("adversary.extra_threats").max(0) as usize
            } else {
                0
            };
            let first = (g.round.max(1) as usize - 1) % n.max(1);
            for c in 0..n {
                let more = (0..extra).filter(|k| (first + k) % n == c).count() as i64;
                for _ in 0..self.per_country + more {
                    self.refill(g, cx, tier);
                    if !deal(g, cx, tier, &deal_pile(self.shared, c)) {
                        break;
                    }
                }
            }
            return Ok(());
        }
        // Execute: anything fully paid is countered, partial payments come
        // back, then each threat in order.
        for (t, _) in active(g) {
            if !try_complete(g, cx, &t) {
                refund(g, cx, &t);
            }
        }
        for (t, scope) in active(g) {
            let d = def(&w, &t);
            cx.note(None, || match scope {
                Some(c) => format!("Threat {} executes in country {c}", d.name),
                None => format!("Threat {} executes everywhere", d.name),
            });
            cx.count(g, "executed", 1);
            for op in &d.effects {
                execute(g, cx, op, scope)?;
            }
            alignment::sway(g, cx, Side::Discord, w.int("alignment.threat_discord"));
            clear(g, cx, &t, scope);
        }
        Ok(())
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<crate::Action>) {
        let s = seat as usize;
        for (t, scope) in active(g) {
            if !self.can_pay(w, g, s, scope) {
                continue;
            }
            for (r, need) in requirement(w, g, &t) {
                let have = g.resource(s, &r);
                let left = need - paid(g, &t, &r);
                let most = have.min(left);
                let amounts: &[i64] = match most {
                    ..=0 => &[],
                    1 => &[1],
                    _ => &[1, most],
                };
                for &amount in amounts {
                    out.push(Action::Pay {
                        seat,
                        threat: t.clone(),
                        resource: r.clone(),
                        amount,
                    });
                }
                if g.var(&paid_key(&t, s, &r)) > 0 {
                    out.push(Action::Withdraw {
                        seat,
                        threat: t.clone(),
                        resource: r.clone(),
                    });
                }
            }
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Pay { .. } | Action::Withdraw { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let s = a.seat() as usize;
        match a {
            Action::Pay {
                threat,
                resource,
                amount,
                ..
            } => {
                let Some((_, scope)) = active(g).into_iter().find(|(t, _)| t == threat) else {
                    return Err(Halt::Illegal(format!("no active threat {threat}")));
                };
                if !self.can_pay(&w, g, s, scope) {
                    return Err("only players with a data centre in its country can pay".into());
                }
                let req = requirement(&w, g, threat);
                let need = req.iter().find(|(r, _)| r == resource).map(|(_, n)| *n);
                let Some(need) = need else {
                    return Err("that resource is not part of its counter".into());
                };
                if *amount < 1
                    || *amount > g.resource(s, resource)
                    || *amount > need - paid(g, threat, resource)
                {
                    return Err("can't pay that much".into());
                }
                cx.add_resource(g, s, resource, -amount);
                cx.add_key(g, paid_key(threat, s, resource), *amount);
                try_complete(g, cx, threat);
                Ok(())
            }
            Action::Withdraw {
                threat, resource, ..
            } => {
                let k = paid_key(threat, s, resource);
                let n = g.var(&k);
                if n == 0 {
                    return Err("nothing to withdraw".into());
                }
                cx.set_key(g, k, 0);
                cx.add_resource(g, s, resource, n);
                Ok(())
            }
            _ => Err(Halt::Illegal(format!("not a threat action: {a:?}"))),
        }
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        // Payments and jams belong to a threat card: shown in its text.
        out.bookkeeping(&["threats.paid.", "threats.jam."]);
        let show = |t: &str| {
            let d = def(w, t);
            let req = requirement(w, g, t);
            let progress: Vec<String> = req
                .iter()
                .map(|(r, n)| format!("{}/{n} {r}", paid(g, t, r)))
                .collect();
            // Who paid what so far (returned if they withdraw), and Jam Signals.
            let mut extra = String::new();
            let payers: Vec<String> = (0..g.seats.len())
                .flat_map(|s| {
                    req.iter().filter_map(move |(r, _)| {
                        let n = g.var(&paid_key(t, s, r));
                        (n > 0).then(|| format!("seat {s} {n} {r}"))
                    })
                })
                .collect();
            if !payers.is_empty() {
                extra.push_str(&format!(" Paid: {}.", payers.join(", ")));
            }
            let jam = g.var(&jam_key(t));
            if jam > 0 {
                extra.push_str(&format!(" Jammed: needs {jam} less of each."));
            }
            CardView {
                id: t.to_string(),
                name: d.name.clone(),
                text: format!("{} Counter: {}.{extra}", d.text, progress.join(", ")),
                status: progress.join(", "),
                cost: None,
            }
        };
        if self.shared {
            let row: Vec<CardView> = g.pile(ROW).iter().map(|t| show(t)).collect();
            out.pile("row", "Threats (every country)", None, row.len(), row);
        } else {
            for c in 0..w.map.countries.len() {
                let cards: Vec<CardView> = g.pile(&on(c)).iter().map(|t| show(t)).collect();
                if !cards.is_empty() {
                    out.country_cards(c, "on", "threat", cards);
                }
            }
        }
        out.global(
            "tier",
            "threat tier",
            adversary::threat_tier(w, g) as i64,
            Some(2),
            "the tier of threat dealt next round start",
        );
    }
}

/// Runs one threat effect in `scope` (a country, or everywhere).
fn execute(g: &mut GameState, cx: &mut Cx, op: &ThreatOp, scope: Option<usize>) -> Res {
    let w = cx.world;
    let hexes = scope_hexes(&w, scope);
    match op {
        ThreatOp::Ambassadors { at, count } => {
            for _ in 0..*count {
                let spots: Vec<usize> = hexes
                    .iter()
                    .copied()
                    .filter(|&h| match at {
                        Spot::PlayerBuildings => player_owned(g, h),
                        Spot::CleanCities => clean_city(&w, g, h),
                        Spot::Factories => g.tokens(h, FACTORY) > 0,
                        Spot::Bases => is_base(&w, h),
                    })
                    .collect();
                let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
                let Some(i) = cx.choose(0, "Threat: an adversary ambassador goes on which piece?", opts)? else {
                    break;
                };
                adversary::place(g, cx, spots[i], 1);
            }
        }
        ThreatOp::Troop { count } => {
            for _ in 0..*count {
                let mut spots: Vec<usize> = hexes.iter().copied().filter(|&h| g.tokens(h, FACTORY) > 0).collect();
                if spots.is_empty() {
                    spots = hexes.iter().copied().filter(|&h| is_city(&w, h)).collect();
                }
                let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
                let Some(i) = cx.choose(0, "Threat: an adversary troop lands on which hex?", opts)? else {
                    break;
                };
                adversary::place_troop(g, cx, spots[i]);
                cx.adversary_ops(g, 1);
                cx.tally(g, "threats.troops", 1);
            }
        }
        ThreatOp::FlipStation => {
            let spots: Vec<usize> =
                hexes.iter().copied().filter(|&h| g.tokens(h, STATION) > 0 && !corrupted(g, h)).collect();
            let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
            if let Some(i) = cx.choose(0, "Threat: which research station flips to the adversary?", opts)? {
                adversary::corrupt_station(g, cx, spots[i]);
            }
        }
        ThreatOp::CorruptCity => {
            for c in scope_countries(&w, scope) {
                if let Some(h) = adversary::most_infiltrated_city(&w, g, c, |h| is_city(&w, h) && !corrupted(g, h)) {
                    adversary::corrupt_city(g, cx, h);
                }
            }
        }
        ThreatOp::PlayersLose { resource, amount } => {
            for s in seats_in_scope(&w, g, scope) {
                let k = (*amount).min(g.resource(s, resource));
                cx.add_resource(g, s, resource, -k);
            }
        }
        ThreatOp::Flag { name } => {
            for c in scope_countries(&w, scope) {
                cx.set_key(g, board::flag(*name, c), 1);
            }
        }
        ThreatOp::Requisition {
            resource,
            amount,
            cards: n,
        } => {
            for s in seats_in_scope(&w, g, scope) {
                let mut opts = Vec::new();
                if g.resource(s, resource) >= *amount {
                    opts.push(opt(format!("pay {amount} {resource}")));
                }
                if g.pile(&cards::hand(s)).len() as i64 >= *n {
                    opts.push(opt(format!("discard {n} cards")));
                }
                let prompt = format!("Seat {s}: pay {amount} {resource} or discard {n} cards?");
                match cx.choose(s, &prompt, opts.clone())? {
                    Some(i) if opts[i].option.starts_with("pay") => {
                        cx.add_resource(g, s, resource, -amount);
                    }
                    Some(_) => {
                        for k in 0..*n {
                            let hand = g.pile(&cards::hand(s)).to_vec();
                            let opts = hand.iter().map(|c| opt(c.clone())).collect();
                            let prompt = format!("Seat {s}: discard which card ({} of {n})?", k + 1);
                            if let Some(j) = cx.choose(s, &prompt, opts)? {
                                cards::discard(g, cx, &hand[j], &cards::hand(s));
                            }
                        }
                    }
                    None => {
                        // Neither possible: pay what you have and discard your hand.
                        let k = g.resource(s, resource);
                        cx.add_resource(g, s, resource, -k);
                        for c in g.pile(&cards::hand(s)).to_vec() {
                            cards::discard(g, cx, &c, &cards::hand(s));
                        }
                    }
                }
            }
        }
        ThreatOp::LoseAmbassador => {
            for s in seats_in_scope(&w, g, scope) {
                if g.resource(s, AMBASSADORS) > 0 {
                    cx.add_resource(g, s, AMBASSADORS, -1);
                    continue;
                }
                let k = ambassador(s);
                let on: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, &k) > 0).collect();
                let opts = on.iter().map(|&h| hex_opt(&w, h)).collect();
                let prompt = format!("Seat {s}: lose the ambassador on which hex?");
                if let Some(i) = cx.choose(s, &prompt, opts)? {
                    cx.add_tokens(g, on[i], &k, -1);
                }
            }
        }
        ThreatOp::Discord { amount } => alignment::sway(g, cx, Side::Discord, *amount),
        ThreatOp::Build => adversary::build_all(g, cx, scope)?,
        ThreatOp::DiscardMostExpensive => {
            for s in seats_in_scope(&w, g, scope) {
                let hand = g.pile(&cards::hand(s)).to_vec();
                let cost = |id: &str| cards::list_cost(&w, id);
                let Some(top) = hand.iter().map(|c| cost(c)).max() else {
                    continue;
                };
                let tied: Vec<String> = hand.into_iter().filter(|c| cost(c) == top).collect();
                let opts = tied.iter().map(|c| opt(c.clone())).collect();
                let prompt = format!("Purge protocols: seat {s} discards which card?");
                if let Some(i) = cx.choose(s, &prompt, opts)? {
                    cards::discard(g, cx, &tied[i], &cards::hand(s));
                }
            }
        }
    }
    Ok(())
}
