//! Keeping by plan, and the forced behaviours of the dominance checks (022).
//!
//! [`Policy`] wraps a greedy or engine agent. At keep time it sketches the
//! seat's next rounds ([`plan_keeps`]) and keeps a drafted card only if the
//! card lands in that plan and the plan is worth more with it than without
//! it (the keep's insight would otherwise pay for plays this round). The
//! dominance checks ([`Forced`]) are fixed policies, mostly the engine with
//! one behaviour forced: they must lose to thoughtful play
//! (`docs/principles.md`, *Dominance checks*).
//!
//! The plan is simple and deterministic, and uses full information (allowed
//! for agents, principles.md *Information*): projected credit, insight,
//! hardware and energy income; the cards it would play (hand plus the
//! candidates) in value-per-cost order, filled into each round's budget.

use aic_core::content::{CardDef, Effect, RESOURCES};
use aic_core::modules::{ai, cards, economy};
use aic_core::module::World;
use aic_core::{Action, Agent, GameState, Rules};

/// A play's value falls by this share for each round it waits.
const WAIT: f64 = 0.15;

/// A card's credit-equivalent cost: each antimatter counts this many
/// credits, each insight this many, each point of income lowered this many.
const ANTIMATTER: i64 = 3;
const INSIGHT: i64 = 2;
const LOWERED: i64 = 3;

/// The behaviour a dominance check forces (none for thoughtful play).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Forced {
    /// Keep every drafted card it can afford.
    pub keep_all: bool,
    /// Keep only its `keep_k` cheapest drafted cards (0: off).
    pub keep_k: usize,
    /// Never draft from tier 2.
    pub tier1_only: bool,
    /// Draft from tier 2 whenever it can afford the keep.
    pub tier2_rush: bool,
    /// Never use an AI-board power.
    pub no_powers: bool,
    /// Never pay toward a threat's counter.
    pub no_counters: bool,
    /// Place no presence beyond setup (no standard action, card or growth
    /// that places one).
    pub no_presence: bool,
    /// Mono-tag (023): keep and play only cards carrying this tag.
    pub only_tag: Option<String>,
    /// Ignore-X (023): never keep or play a card carrying this tag.
    pub never_tag: Option<String>,
    /// Event-hoarder (023): keep and play only events.
    pub events_only: bool,
    /// Keep by plan (off: the wrapped agent's heuristic decides keeps).
    pub plan_keep: bool,
    /// Rounds the keep plan looks at: this round's action phase and the
    /// ones after it.
    pub plan_rounds: usize,
}

impl Forced {
    /// Whether the card restrictions (mono-tag, ignore-X, event-hoarder)
    /// let this seat keep or play `d`. AI-unique cards (tier 0) are exempt.
    pub fn allows(&self, d: &CardDef) -> bool {
        if d.tier == 0 {
            return true;
        }
        let has = |t: &Option<String>| t.as_ref().is_some_and(|t| d.tags.contains(t));
        (self.only_tag.is_none() || has(&self.only_tag))
            && !has(&self.never_tag)
            && (!self.events_only || d.is_event())
    }
}

pub struct Policy {
    inner: Box<dyn Agent + Send>,
    forced: Forced,
    /// This round's keep set for a seat: (round, seat, cards).
    keeps: Vec<(u32, usize, Vec<String>)>,
}

impl Policy {
    pub fn new(inner: Box<dyn Agent + Send>, forced: Forced) -> Policy {
        Policy {
            inner,
            forced,
            keeps: Vec::new(),
        }
    }

    /// The cards seat `s` keeps this round, worked out at its first keep
    /// decision of the round.
    fn keep_set(&mut self, rules: &Rules, g: &GameState, s: usize) -> Vec<String> {
        let round = g.round;
        if let Some((_, _, set)) = self.keeps.iter().find(|(r, x, _)| *r == round && *x == s) {
            return set.clone();
        }
        let world = rules.world();
        let drafted = g.pile(&cards::drafted(s)).to_vec();
        let set = if self.forced.keep_all {
            drafted
        } else if self.forced.keep_k > 0 {
            let mut d = drafted;
            d.sort_by_key(|id| (list_cost(cards::def(&world, id)), id.clone()));
            d.truncate(self.forced.keep_k);
            d
        } else {
            plan_keeps(&world, g, s, self.forced.plan_rounds)
        };
        let set: Vec<String> = set
            .into_iter()
            .filter(|id| self.forced.allows(cards::def(&world, id)))
            .collect();
        self.keeps.retain(|(r, _, _)| *r == round);
        self.keeps.push((round, s, set.clone()));
        set
    }

    /// Whether `a` is an action the forced behaviour rules out.
    fn ruled_out(&self, world: &World, g: &GameState, a: &Action) -> bool {
        let f = &self.forced;
        match a {
            Action::Tier { tier: 2, .. } => f.tier1_only,
            Action::Power { .. } => f.no_powers,
            Action::Pay { .. } => f.no_counters,
            Action::Standard { act, .. } => f.no_presence && act == "presence",
            Action::Play { card, .. } => {
                let d = cards::def(world, card);
                (f.no_presence && places_presence(&d.effects)) || !f.allows(d)
            }
            Action::Keep { card, .. } => !f.allows(cards::def(world, card)),
            Action::Growth { seat, option } => {
                f.no_presence
                    && ai::board(world, *seat as usize)
                        .growth
                        .get(*option as usize)
                        .is_some_and(|o| places_presence(&o.effects))
            }
            _ => {
                let _ = g;
                false
            }
        }
    }
}

fn places_presence(effects: &[Effect]) -> bool {
    effects.iter().any(|e| matches!(e, Effect::PlacePresence { .. }))
}

impl Agent for Policy {
    fn choose(&mut self, rules: &Rules, g: &GameState, legal: &[Action]) -> usize {
        let world = rules.world();
        let phase = g.config.round.get(g.phase).map(String::as_str).unwrap_or("");
        let pending = legal.iter().any(|a| matches!(a, Action::Choose { .. }));
        // Keeping: by plan, or by the forced rule.
        let keeps = phase == "cards.keep"
            && !pending
            && (self.forced.plan_keep || self.forced.keep_all || self.forced.keep_k > 0);
        if keeps {
            if let Some(s) = legal.first().map(|a| a.seat() as usize) {
                let set = self.keep_set(rules, g, s);
                if let Some(i) = legal.iter().position(
                    |a| matches!(a, Action::Keep { seat, card } if *seat as usize == s && set.contains(card)),
                ) {
                    return i;
                }
                if let Some(i) = legal
                    .iter()
                    .position(|a| matches!(a, Action::Pass { seat } if *seat as usize == s))
                {
                    return i;
                }
            }
        }
        // Tier-2 rush: tier 2 whenever the keep is affordable.
        if self.forced.tier2_rush && !pending {
            if let Some(i) = legal.iter().position(|a| {
                matches!(a, Action::Tier { seat, tier: 2 } if keep_affordable(&world, g, *seat as usize, 2))
            }) {
                return i;
            }
        }
        let keep: Vec<usize> = (0..legal.len())
            .filter(|&i| !self.ruled_out(&world, g, &legal[i]))
            .collect();
        if keep.is_empty() || keep.len() == legal.len() {
            return self.inner.choose(rules, g, legal);
        }
        let sub: Vec<Action> = keep.iter().map(|&i| legal[i].clone()).collect();
        keep[self.inner.choose(rules, g, &sub)]
    }
}

/// A card's printed cost in credits (antimatter, insight, energy, hardware
/// and lowered income at fixed rates): the plan's measure of what it does.
pub fn list_cost(d: &CardDef) -> i64 {
    d.cost
        + ANTIMATTER * d.antimatter
        + d.energy
        + d.hardware
        + INSIGHT * d.insight
        + LOWERED * d.lowers.iter().map(|(_, n)| n).sum::<i64>()
}

/// The keep price of a tier for seat `s` in insight, after the round's
/// opening rules: (insight, credits for an opening keep).
fn keep_need(w: &World, g: &GameState, s: usize, tier: u8, kept: i64) -> (i64, i64) {
    if g.round <= 1 {
        if kept < w.int("cards.opening_free") {
            return (0, 0);
        }
        return (0, w.int("cards.opening_keep_credits"));
    }
    let need = match tier {
        2 if ai::keep2(w, g, s) => w.int("cards.keep_tier2_upgraded"),
        2 => w.int("cards.keep_tier2"),
        _ => w.int("cards.keep_tier1"),
    };
    (need, 0)
}

/// Whether seat `s` could pay to keep a card of `tier` now (insight, with
/// any shortfall in credits).
fn keep_affordable(w: &World, g: &GameState, s: usize, tier: u8) -> bool {
    let (ins, cr) = keep_need(w, g, s, tier, 0);
    let short = (ins - g.resource(s, "insight").max(0)).max(0);
    g.resource(s, "credits") >= cr + short * w.int("cards.insight_credits")
}

/// One round's budget in the plan.
#[derive(Clone, Copy)]
struct Budget {
    credits: i64,
    insight: i64,
    hardware: i64,
    energy: i64,
    antimatter: i64,
}

/// The drafted cards seat `s` keeps: each, best value per cost first, if
/// the plan with it is worth more than the plan without it.
pub fn plan_keeps(w: &World, g: &GameState, s: usize, rounds: usize) -> Vec<String> {
    let mut drafted = g.pile(&cards::drafted(s)).to_vec();
    let order = |ids: &mut Vec<String>| {
        ids.sort_by(|a, b| {
            ratio(w, g, s, b)
                .total_cmp(&ratio(w, g, s, a))
                .then_with(|| a.cmp(b))
        })
    };
    order(&mut drafted);
    let mut kept: Vec<String> = Vec::new();
    let mut best = plan_value(w, g, s, &kept, rounds);
    for c in drafted {
        let mut with = kept.clone();
        with.push(c.clone());
        let Some(v) = plan_value_checked(w, g, s, &with, &c, rounds) else {
            continue;
        };
        if v > best + 1e-9 {
            best = v;
            kept = with;
        }
    }
    kept
}

/// [`plan_value`] with `kept`, or `None` if the keeps can't be paid or the
/// new card `c` is not played in the plan.
fn plan_value_checked(
    w: &World,
    g: &GameState,
    s: usize,
    kept: &[String],
    c: &str,
    rounds: usize,
) -> Option<f64> {
    let (v, planned) = plan(w, g, s, kept, rounds)?;
    planned.iter().any(|p| p == c).then_some(v)
}

fn plan_value(w: &World, g: &GameState, s: usize, kept: &[String], rounds: usize) -> f64 {
    plan(w, g, s, kept, rounds).map_or(f64::NEG_INFINITY, |(v, _)| v)
}

/// The card's value: its printed cost plus 1 (a play is worth something).
fn value(d: &CardDef) -> f64 {
    1.0 + list_cost(d) as f64
}

/// What seat `s` pays to play `d`: credits after its building discount.
fn spend(w: &World, g: &GameState, s: usize, d: &CardDef) -> i64 {
    let credits = cards::play_cost(w, g, s, d);
    credits + ANTIMATTER * d.antimatter + d.energy + d.hardware + INSIGHT * d.insight
        + LOWERED * d.lowers.iter().map(|(_, n)| n).sum::<i64>()
}

fn ratio(w: &World, g: &GameState, s: usize, id: &str) -> f64 {
    let d = cards::def(w, id);
    value(d) / (1 + spend(w, g, s, d)) as f64
}

/// The plan with `kept` bought now: its value, and the cards it plays.
/// `None` if the keeps can't be paid.
fn plan(
    w: &World,
    g: &GameState,
    s: usize,
    kept: &[String],
    rounds: usize,
) -> Option<(f64, Vec<String>)> {
    let res = |r: &str| g.resource(s, r).max(0);
    let income = economy::income(w, g, s);
    let inc = |r: &str| RESOURCES.iter().position(|x| *x == r).map_or(0, |i| income[i]);
    let mut now = Budget {
        credits: res("credits"),
        insight: res("insight"),
        hardware: res("hardware"),
        energy: res("energy"),
        antimatter: res("antimatter"),
    };
    // Pay the keeps: insight first, the shortfall in credits.
    let rate = w.int("cards.insight_credits");
    let mut count = g.var(&format!("cards.kept_round.{s}"));
    for id in kept {
        let (ins, cr) = keep_need(w, g, s, cards::def(w, id).tier, count);
        count += 1;
        let from_insight = ins.min(now.insight);
        now.insight -= from_insight;
        now.credits -= cr + (ins - from_insight) * rate;
        if now.credits < 0 {
            return None;
        }
    }
    let mut pool: Vec<String> = g.pile(&cards::hand(s)).to_vec();
    pool.extend(kept.iter().cloned());
    pool.sort_by(|a, b| {
        ratio(w, g, s, b)
            .total_cmp(&ratio(w, g, s, a))
            .then_with(|| a.cmp(b))
    });
    let mut planned: Vec<String> = Vec::new();
    let mut total = 0.0;
    let mut b = now;
    for round in 0..rounds.max(1) {
        if round > 0 {
            // Credits, hardware, energy and antimatter carry over; insight resets.
            b = Budget {
                credits: b.credits + inc("credits"),
                insight: inc("insight"),
                hardware: b.hardware + inc("hardware"),
                energy: b.energy + inc("energy"),
                antimatter: b.antimatter + inc("antimatter"),
            };
        }
        for id in &pool {
            if planned.contains(id) {
                continue;
            }
            let d = cards::def(w, id);
            let credits = cards::play_cost(w, g, s, d);
            let fits = b.credits >= credits
                && b.insight >= d.insight
                && b.hardware >= d.hardware
                && b.energy >= d.energy
                && b.antimatter >= d.antimatter
                && cards::lowers_ok(w, g, s, d)
                && d.needs.iter().all(|(t, n)| cards::tags(w, g, s, t) >= *n);
            if fits {
                b.credits -= credits;
                b.insight -= d.insight;
                b.hardware -= d.hardware;
                b.energy -= d.energy;
                b.antimatter -= d.antimatter;
                total += value(d) * (1.0 - WAIT * round as f64).max(0.1);
                planned.push(id.clone());
            }
        }
    }
    Some((total, planned))
}
