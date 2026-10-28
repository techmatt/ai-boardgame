//! The heuristic value of a state, from the team's point of view. It reads
//! only what the table shows: tokens, tracks, resources, open hands and
//! buildings, active threats and what is paid on them, deck sizes. Units
//! are roughly "credits": a credit in hand is worth `credits` (0.6) because
//! credits buy cards that do more.
//!
//! The weights are agent parameters, not rules: they live in [`Weights`]
//! and an experiment can override any of them (`greedy:w.pool=20`).
//!
//! **Horizon mode** (`w.horizon` > 0; the `engine` agent's default): income
//! and buildings are valued by what they will pay over the rounds the game
//! is expected to last ([`Eval::rounds_left`]: the adversary deck's clock,
//! cut short by how soon the team is on course to win and by the danger it
//! is in), not by a flat weight; a card in hand is valued by whether it can
//! be paid for before the game ends; and resources lose their value as the
//! end nears (nothing is left to buy).

use std::collections::BTreeMap;

use aic_core::content::{Passive, RESOURCES};
use aic_core::content::Side;
use aic_core::modules::{adversary, ai, alignment, board, cards, economy, threats};
use aic_core::{GameState, Rules};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    /// A finished game: won or lost.
    pub win: f64,
    /// Added per adversary card left in the deck on a win (a wider margin).
    pub win_margin: f64,
    /// Each corruption left in the pool.
    pub pool: f64,
    /// Extra per corruption when the pool is down to 2 or less.
    pub pool_danger: f64,
    /// Each corruption on the board (it stops production).
    pub corruption: f64,
    /// Each population above the loss threshold.
    pub population: f64,
    /// Extra when only 1 population of slack is left.
    pub population_danger: f64,
    pub drone: f64,
    pub outpost: f64,
    pub foundry: f64,
    /// Each adversary piece that keeps the current clear-board bar from
    /// being met (the win condition).
    pub blocker: f64,
    /// Each Unity tip made; Sway in the Unity bucket counts as its share of
    /// the next tip (so a tip about to happen is worth nearly a tip).
    pub unity_tip: f64,
    /// Each Discord tip made (a cost); Sway in the Discord bucket costs its
    /// share of the next one.
    pub discord_tip: f64,
    /// Each level the clear-board bar has dropped (Unity tips).
    pub level: f64,
    /// Each difficulty level Discord has escalated the adversary (a cost).
    pub escalation: f64,
    pub factory: f64,
    pub data_centre: f64,
    /// Each processor on the board.
    pub processor: f64,
    /// Each point of per-round income (AI tracks).
    pub income: f64,
    pub credits: f64,
    pub compute: f64,
    pub energy: f64,
    pub antimatter: f64,
    pub insight: f64,
    /// A card in hand: `hand + hand_cost * cost`, cost in credits (antimatter
    /// counts 3, insight 2).
    pub hand: f64,
    pub hand_cost: f64,
    /// A drafted card not yet kept, as a share of its hand value.
    pub drafted: f64,
    /// A building in play: `built + built_cost * cost`.
    pub built: f64,
    pub built_cost: f64,
    /// Each active threat, scaled by the share still unpaid.
    pub threat: f64,
    /// Each defense token on the board.
    pub defense: f64,
    /// Share of the coming ravage's cost (the ravage-slot card) counted now.
    pub forecast: f64,
    /// Share of the next round's ravage (the build-slot card) counted now.
    pub forecast_next: f64,
    /// Horizon mode, when above 0: each unit of income (AI tracks,
    /// buildings, rewards) is worth `horizon` × the resource's own weight ×
    /// the rounds left, replacing `income`. 0 = the flat weights above.
    pub horizon: f64,
    /// Horizon mode: Sway toward Unity the team is assumed to gain per round
    /// before it has a record of its own (round 1), for the win estimate.
    pub opp_rate: f64,
    /// Horizon mode: rounds a win needs after the last bar drop is reached
    /// (clearing the last foundries).
    pub win_lag: f64,
    /// Horizon mode: chance of losing per round per unit of danger (an
    /// unpaid threat per country, a pool or population at its edge); the
    /// rounds left are the expected survival over the clock.
    pub risk: f64,
    /// Horizon mode: per round left, a building that pays no income (an
    /// ability, a discount, protection), on top of `built`.
    pub ability: f64,
    /// Horizon mode: resources keep their full value only while at least
    /// this many rounds are left, then fall toward 0 (nothing left to buy).
    pub spend_horizon: f64,
    /// Horizon mode: a tier-2 card in hand before tier 2 is unlocked counts
    /// this share of its value.
    pub locked: f64,
    /// Horizon mode: production (working factories' credits, processors'
    /// compute) counts this share of income over the rounds left.
    pub production: f64,
    /// Horizon mode, when above 0: cards in hand wait their turn for
    /// insight (each needs the insight of the cards before it too).
    pub insight_wait: f64,
    /// The deck clock (011), any mode: per round the team's projected win
    /// (Unity still needed at its rate so far, plus `win_lag`) comes
    /// after the adversary deck runs out. 0 = off.
    pub clock: f64,
}

impl Default for Weights {
    fn default() -> Self {
        Weights {
            win: 1000.0,
            win_margin: 5.0,
            // 007: 12 suited v0's 6-corruption pool; on the tuned pool it made
            // beam buy back corruption instead of purging (58% vs 96% won).
            pool: 6.0,
            pool_danger: 10.0,
            corruption: 2.0,
            population: 6.0,
            population_danger: 12.0,
            drone: 2.0,
            outpost: 5.0,
            foundry: 9.0,
            blocker: 2.0,
            // 013: a Unity tip replaces about 4 opposition points (24) and
            // brings a card; a Discord tip brings one of the adversary's.
            unity_tip: 24.0,
            discord_tip: 16.0,
            level: 10.0,
            escalation: 20.0,
            factory: 3.0,
            data_centre: 2.0,
            processor: 4.0,
            income: 3.0,
            credits: 0.6,
            compute: 0.8,
            energy: 0.8,
            antimatter: 2.0,
            insight: 0.5,
            hand: 3.0,
            hand_cost: 0.4,
            drafted: 0.3,
            built: 4.0,
            built_cost: 0.5,
            threat: 6.0,
            defense: 0.5,
            forecast: 0.9,
            forecast_next: 0.3,
            horizon: 0.0,
            opp_rate: 2.0,
            win_lag: 1.0,
            risk: 0.1,
            ability: 0.5,
            spend_horizon: 2.0,
            locked: 0.5,
            // 008 sweep (d4, 96 seeds): 1 won 65.6%, 0 won 72.9% (greedy 68.8%).
            production: 0.0,
            // 008: on, it cut the engine to 4% wins at d4 (96 games): spending
            // insight on one card lowered every other card's chance, so
            // playing anything looked costly. Off until it is modelled per round.
            insight_wait: 0.0,
            clock: 0.0,
        }
    }
}

impl Weights {
    /// The `engine` agent's defaults: horizon mode, with hand and building
    /// weights that no longer stand in for the future (008 sweep).
    pub fn engine() -> Weights {
        Weights {
            horizon: 1.0,
            hand: 1.0,
            hand_cost: 0.2,
            built: 1.0,
            built_cost: 0.0,
            // 011: see `clock`; chosen on 009's cheap-tier-2 patch, where
            // without it the engine ran out the deck.
            clock: 30.0,
            ..Weights::default()
        }
    }

    /// Sets one weight by name; false if there is no such weight.
    pub fn set(&mut self, name: &str, v: f64) -> bool {
        let mut map = match serde_json::to_value(&*self) {
            Ok(serde_json::Value::Object(m)) => m,
            _ => return false,
        };
        if !map.contains_key(name) {
            return false;
        }
        map.insert(name.to_string(), serde_json::json!(v));
        match serde_json::from_value(serde_json::Value::Object(map)) {
            Ok(w) => *self = w,
            Err(_) => return false,
        }
        true
    }
}

/// A [`Weights`] bound to one game's rules, with per-card values cached.
pub struct Eval {
    pub w: Weights,
    card_value: BTreeMap<String, f64>,
    seats: usize,
    loss_percent: i64,
    /// Unity tip counts at which the bar drops; the last is the win target.
    ease: Vec<i64>,
    /// The adversary variant: the Militarist's pieces and ravage forecast,
    /// or the spread variants' clusters or corrupted assets.
    style: adversary::Style,
    /// Spread variants: pieces allowed per bar level (purge win).
    allowance: i64,
    /// Ravage damage of a drone, outpost and foundry (Militarist).
    damage: [i64; 3],
    /// Extra ravage damage where there is a foundry (difficulty 3+ in play).
    foundry_bonus: i64,
    /// Damage a ravaged foundry deals to each adjacent hex (difficulty 3+).
    foundry_reach: i64,
}

impl Eval {
    pub fn new(rules: &Rules, w: Weights) -> Eval {
        let world = rules.world();
        let card_value = world
            .content
            .cards
            .iter()
            .map(|c| {
                let cost = world.amount(&c.cost)
                    + 3 * world.amount(&c.antimatter)
                    + 2 * world.amount(&c.insight);
                (c.id.clone(), cost.max(0) as f64)
            })
            .collect();
        let seats = rules.config.countries as usize;
        Eval {
            w,
            card_value,
            seats,
            loss_percent: world.int("alignment.loss_population_percent"),
            ease: world.list("alignment.unity_ease").to_vec(),
            style: adversary::style(&world),
            allowance: match adversary::style(&world) {
                adversary::Style::Militarist => 0,
                _ => world.int("adversary.purge_allowance") * world.map.countries.len() as i64,
            },
            damage: match adversary::style(&world) {
                adversary::Style::Militarist => [
                    world.int("adversary.drone_damage"),
                    world.int("adversary.outpost_damage"),
                    world.int("adversary.foundry_damage"),
                ],
                _ => [0; 3],
            },
            foundry_bonus: if adversary::style(&world) == adversary::Style::Militarist {
                world.int("adversary.foundry_bonus")
            } else {
                0
            },
            foundry_reach: if adversary::style(&world) == adversary::Style::Militarist {
                world.int("adversary.foundry_reach")
            } else {
                0
            },
        }
    }

    fn cost(&self, id: &str) -> f64 {
        self.card_value.get(id).copied().unwrap_or(0.0)
    }

    /// The value of `g`. Finished games score `±win` plus a margin bonus.
    pub fn value(&self, rules: &Rules, g: &GameState) -> f64 {
        let w = &self.w;
        if let Some(o) = &g.outcome {
            return if o.won {
                w.win + w.win_margin * g.pile(adversary::DECK).len() as f64
            } else {
                -w.win
            };
        }
        let mut v = 0.0;
        let (mut drones, mut outposts, mut foundries, mut pop, mut corruption) =
            (0i64, 0i64, 0i64, 0i64, 0i64);
        let (mut factories, mut dcs, mut processors, mut defense) = (0i64, 0i64, 0i64, 0i64);
        // Spread variants' pieces: clusters, or corrupted assets.
        let mut spread = 0i64;
        for h in &g.hexes {
            for (k, &n) in &h.tokens {
                match k.as_str() {
                    "drone" => drones += n,
                    "outpost" => outposts += n,
                    "foundry" => foundries += n,
                    "population" => pop += n,
                    "corruption" => corruption += n,
                    "cluster" | "rogue_factory" | "rogue_data_centre" => spread += n,
                    "factory" => factories += n,
                    "data_centre" => dcs += n,
                    _ if k.starts_with("processor:") => processors += n,
                    _ if k.starts_with("defense:") => defense += n,
                    _ => {}
                }
            }
        }
        let pool = g.var(adversary::POOL);
        v += w.pool * pool as f64 - w.pool_danger * (3 - pool).max(0) as f64;
        v -= w.corruption * corruption as f64;
        let start = g.var(board::START_POPULATION);
        // Lose when pop * 100 < start * percent: the smallest safe population.
        let safe = (start * self.loss_percent + 99) / 100;
        let slack = pop - safe;
        v += w.population * slack as f64 - w.population_danger * (2 - slack).max(0) as f64;
        v -= w.drone * drones as f64 + w.outpost * outposts as f64 + w.foundry * foundries as f64;
        let world = rules.world();
        let level = alignment::bar_level(&world, g);
        // A spread variant's piece weighs as an outpost; its purge bar
        // allows `allowance` pieces per level.
        v -= w.outpost * spread as f64;
        let blockers = match self.style {
            adversary::Style::Militarist => match level {
                0 => drones + outposts + foundries,
                1 => outposts + foundries,
                _ => foundries,
            },
            _ => (spread - level * self.allowance).max(0),
        };
        v -= w.blocker * blockers as f64;
        // Alignment: tips made, and each bucket's share of its next tip.
        for (side, weight) in [(Side::Unity, w.unity_tip), (Side::Discord, -w.discord_tip)] {
            let cap = alignment::capacity(&world, g, side).max(1) as f64;
            let fill = (g.var(alignment::bucket(side)) as f64 / cap).min(1.0);
            v += weight * (g.var(alignment::tips_key(side)) as f64 + fill);
        }
        v += w.level * level as f64 - w.escalation * g.var(adversary::ESCALATION) as f64;
        v += w.factory * factories as f64
            + w.data_centre * dcs as f64
            + w.processor * processors as f64;
        v += w.defense * defense as f64;
        // Added in place, term by term, so the flat agents' values (and so
        // their games) are bit-for-bit what they were before horizon mode.
        let unpaid = self.unpaid_threats(rules, g);
        if w.horizon > 0.0 {
            self.horizon_value(rules, g, &unpaid, &mut v);
        } else {
            self.flat_value(rules, g, &mut v);
        }
        // The coming ravages are public (the ravage and build slots): what
        // they would cost if the table stayed as it is. Each seat's defense
        // value is the same for both: worked out once, when first needed.
        let mut defense = None;
        for (slot, share) in [
            (adversary::SLOTS[2], w.forecast),
            (adversary::SLOTS[1], w.forecast_next),
        ] {
            if share != 0.0 {
                if let Some(id) = g.pile(slot).first() {
                    v -= share * self.ravage_cost(rules, g, id, &mut defense);
                }
            }
        }
        for share in unpaid {
            v -= w.threat * share;
        }
        v -= self.clock_penalty(rules, g);
        v
    }

    /// Each active threat's share still unpaid.
    fn unpaid_threats(&self, rules: &Rules, g: &GameState) -> Vec<f64> {
        let world = rules.world();
        let mut out = Vec::new();
        for (t, _) in threats::active(g) {
            let req: i64 = threats::requirement(&world, g, &t)
                .iter()
                .map(|(_, n)| n)
                .sum();
            if req <= 0 {
                continue;
            }
            let prefix = format!("threats.paid.{t}.");
            let paid: i64 = g
                .vars
                .range(prefix.clone()..)
                .take_while(|(k, _)| k.starts_with(&prefix))
                .map(|(_, &n)| n)
                .sum();
            out.push(1.0 - (paid as f64 / req as f64).min(1.0));
        }
        out
    }

    /// The rounds of income still to come, as the table reads: the adversary
    /// deck's clock (a card is revealed each round; an empty deck at a
    /// reveal loses), cut short by when the team is on course to win
    /// (Unity still needed at its rate so far, plus `win_lag`), then
    /// the expected rounds survived at a per-round loss chance of `risk` per
    /// unit of danger (unpaid threats per country, the pool or population at
    /// its edge). `unpaid`: [`Eval::unpaid_threats`], which the caller has.
    fn rounds_left(&self, rules: &Rules, g: &GameState, unpaid: &[f64]) -> f64 {
        let w = &self.w;
        let clock = g.pile(adversary::DECK).len() as f64;
        let (points, bar) = self.unity_progress(rules, g);
        let to_win = ((bar - points).max(0.0) / self.unity_rate(rules, g)).ceil() + w.win_lag;
        let rounds = clock.min(to_win).max(0.0);
        let world = rules.world();
        let countries = world.map.countries.len().max(1) as f64;
        let mut danger = unpaid.iter().sum::<f64>() / countries;
        if g.var(adversary::POOL) <= 4 {
            danger += 0.5;
        }
        let start = g.var(board::START_POPULATION);
        let safe = (start * self.loss_percent + 99) / 100;
        let pop: i64 = g.hexes.iter().map(|h| h.tokens.get("population").copied().unwrap_or(0)).sum();
        if pop - safe <= 2 {
            danger += 0.5;
        }
        let p = (1.0 - w.risk * danger).clamp(0.0, 1.0);
        // Sum of p^k for k = 1..=rounds, with a fractional last round.
        let mut total = 0.0;
        let mut surv = 1.0;
        let mut k = 0.0;
        while k < rounds {
            surv *= p;
            total += surv * (rounds - k).min(1.0);
            k += 1.0;
        }
        total
    }

    /// Unity so far, in Sway (each tip counts a full bucket at the base
    /// size), and the Sway the last bar drop needs.
    fn unity_progress(&self, rules: &Rules, g: &GameState) -> (f64, f64) {
        let world = rules.world();
        let per = (world.int("alignment.tip_per_player") * g.seats.len() as i64) as f64;
        let tips = g.var(alignment::tips_key(Side::Unity)) as f64;
        let points = tips * per + g.var(alignment::bucket(Side::Unity)) as f64;
        let last = self.ease.last().copied().unwrap_or(1) as f64;
        (points, last * per)
    }

    /// Unity gained per round so far, in Sway (`opp_rate` before the team
    /// has a record of its own). Sway held may include this round's, so
    /// this round counts as played.
    fn unity_rate(&self, rules: &Rules, g: &GameState) -> f64 {
        let w = &self.w;
        let points = self.unity_progress(rules, g).0;
        if g.round > 1 {
            (points / g.round as f64).max(w.opp_rate * 0.25)
        } else {
            w.opp_rate
        }
        .max(0.1)
    }

    /// The deck-clock penalty: `clock` per round the team, at its Unity
    /// rate so far, would still be short of the last bar drop when the deck
    /// runs out (013: there is no instant win any more; before, the target
    /// was the top opposition level, a win at once). With `k` cards left, `k` action
    /// phases come after this one (the reveal after the last loses); this
    /// round's points count as they are scored, not as a round still to
    /// come, so in the last round every point short costs `clock` / rate.
    /// The top level, not the purge bar: a team sitting at level 2 with
    /// foundries it can't clear is not on course (the cheap-tier-2 engine
    /// ended 19-23 points of 24 into a deck-out).
    fn clock_penalty(&self, rules: &Rules, g: &GameState) -> f64 {
        if self.w.clock == 0.0 {
            return 0.0;
        }
        let clock = g.pile(adversary::DECK).len() as f64;
        let (points, top) = self.unity_progress(rules, g);
        let need = (top - points).max(0.0) / self.unity_rate(rules, g);
        self.w.clock * (need - clock).max(0.0)
    }

    /// Per-seat resources, income, hands and buildings, by flat weights.
    fn flat_value(&self, rules: &Rules, g: &GameState, v: &mut f64) {
        let w = &self.w;
        let world = rules.world();
        for s in 0..self.seats {
            let res = |k: &str| g.resource(s, k) as f64;
            *v += w.credits * res("credits")
                + w.compute * res("compute")
                + w.energy * res("energy")
                + w.antimatter * res("antimatter")
                + w.insight * res("insight");
            for (r, n) in ai::track_income(&world, g, s) {
                let unit = match r.as_str() {
                    "antimatter" => 3.0,
                    "insight" => 1.5,
                    _ => 1.0,
                };
                *v += w.income * unit * n as f64;
            }
            for id in g.pile(&format!("cards.hand.{s}")) {
                *v += w.hand + w.hand_cost * self.cost(id);
            }
            for id in g.pile(&format!("cards.drafted.{s}")) {
                *v += w.drafted * (w.hand + w.hand_cost * self.cost(id));
            }
            for id in g.pile(&format!("cards.built.{s}")) {
                *v += w.built + w.built_cost * self.cost(id);
            }
        }
    }

    /// Per-seat resources, income, hands and buildings in horizon mode.
    fn horizon_value(&self, rules: &Rules, g: &GameState, unpaid: &[f64], out: &mut f64) {
        let w = &self.w;
        let world = rules.world();
        let left = self.rounds_left(rules, g, unpaid);
        // Factories' credits and processors' compute, if upkeep came now.
        let prod = economy::production(&world, g);
        // Resources fade once fewer than `spend_horizon` rounds are left.
        let fade = if w.spend_horizon > 0.0 {
            ((left + 1.0) / w.spend_horizon).min(1.0)
        } else {
            1.0
        };
        let unit = [w.credits, w.compute, w.energy, w.antimatter, w.insight];
        let mut v = 0.0;
        for s in 0..self.seats {
            let res = |k: &str| g.resource(s, k);
            // Insight is reset each round, so what is held now only counts this round.
            for (i, r) in RESOURCES.iter().enumerate() {
                v += fade * unit[i] * res(r) as f64;
            }
            // Income (tracks, buildings with income passives, rewards) over
            // the rounds left, less building upkeep; production counts
            // `production` of that.
            let mut income = economy::income(&world, g, s);
            for id in g.pile(&cards::built(s)) {
                for (r, n) in &cards::def(&world, id).upkeep {
                    if let Some(i) = RESOURCES.iter().position(|x| x == r) {
                        income[i] -= n;
                    }
                }
            }
            for (i, n) in income.iter().enumerate() {
                v += w.horizon * left * unit[i] * *n as f64;
            }
            let (pc, pm) = prod.get(s).copied().unwrap_or((0, 0));
            v += w.horizon * w.production * left * (unit[0] * pc as f64 + unit[1] * pm as f64);
            for id in g.pile(&cards::built(s)) {
                let d = cards::def(&world, id);
                let pays = d.passives.iter().any(|p| {
                    matches!(
                        p,
                        Passive::Income { .. } | Passive::IncomePerTag { .. } | Passive::IncomeIfTags { .. }
                    )
                });
                v += w.built + w.built_cost * self.cost(id);
                if !pays {
                    v += w.ability * left;
                }
            }
            // A card in hand is worth its value times the chance of paying
            // for it before the game ends, from what the seat has and earns
            // (factory credits included). Insight is spent one card at a
            // time: each card waits for the insight the cards before it in
            // hand take.
            let tier2 = ai::tier2(&world, g, s);
            let earn_credits = income[0] + pc;
            let mut insight_queue = 0i64;
            // The same for every building in hand: worked out once.
            let discount = cards::building_discount(&world, g, s);
            let mut play = |id: &str| -> f64 {
                let d = cards::def(&world, id);
                let mut p = 1.0;
                let insight = world.amount(&d.insight);
                insight_queue += insight;
                let insight_need = if w.insight_wait > 0.0 { insight_queue } else { 0 };
                for (have, need, earn) in [
                    (res("credits"), cards::discounted_cost(&world, d, discount), earn_credits),
                    (res("antimatter"), world.amount(&d.antimatter), income[3]),
                    (res("insight"), insight_need, income[4]),
                ] {
                    let short = need - have;
                    if short > 0 {
                        if earn <= 0 {
                            return 0.0;
                        }
                        let rounds = (short as f64 / earn as f64).ceil();
                        p = f64::min(p, (1.0 - rounds / (left + 1.0)).max(0.0));
                    }
                }
                if d.tier == 2 && !tier2 {
                    p *= w.locked;
                }
                p * (w.hand + w.hand_cost * self.cost(id))
            };
            for id in g.pile(&cards::hand(s)) {
                v += play(id);
            }
            for id in g.pile(&format!("cards.drafted.{s}")) {
                v += w.drafted * play(id);
            }
        }
        *out += v;
    }

    /// What a ravage with adversary card `id` would cost now, in heuristic
    /// units: corruption (and cascades) and population lost.
    fn ravage_cost(
        &self,
        rules: &Rules,
        g: &GameState,
        id: &str,
        defense: &mut Option<Vec<i64>>,
    ) -> f64 {
        let world = rules.world();
        let content = world.content;
        let Some(card) = content.adversary_index(id).map(|i| &content.adversary[i]) else {
            return 0.0;
        };
        if self.style != adversary::Style::Militarist {
            return 0.0;
        }
        let corruption_at = world.int("adversary.corruption_at");
        let pop_health = world.int("adversary.population_health").max(1);
        let defense: &[i64] = defense.get_or_insert_with(|| {
            (0..self.seats)
                .map(|s| cards::defense_value(&world, g, s))
                .collect()
        });
        // Difficulty 3 in play (Discord escalation counts).
        let d3 = adversary::at(&world, g, 3);
        let (foundry_bonus, foundry_reach) = if d3 { (self.foundry_bonus, self.foundry_reach) } else { (0, 0) };
        let mut cost = 0.0;
        for (h, cell) in world.map.hexes.iter().enumerate() {
            if !card.terrains.contains(&cell.terrain) {
                continue;
            }
            let mut raw = g.tokens(h, "drone") * self.damage[0]
                + g.tokens(h, "outpost") * self.damage[1]
                + g.tokens(h, "foundry") * self.damage[2];
            if raw == 0 {
                continue;
            }
            if g.tokens(h, "foundry") > 0 {
                raw += foundry_bonus;
            }
            let prevented_at = |x: usize| -> i64 {
                defense
                    .iter()
                    .enumerate()
                    .map(|(s, d)| g.tokens(x, &format!("defense:{s}")) * d)
                    .sum()
            };
            // Difficulty 3: the foundries' reach into each adjacent hex
            // (population only: no corruption).
            let reach = foundry_reach * g.tokens(h, "foundry");
            if reach > 0 {
                for &n in &cell.neighbours {
                    let d = (reach - prevented_at(n)).max(0);
                    let lost = (d / pop_health).min(g.tokens(n, "population"));
                    cost += lost as f64 * self.w.population;
                }
            }
            let prevented = prevented_at(h);
            let dmg = (raw - prevented).max(0);
            if dmg >= corruption_at {
                let cascade = if g.tokens(h, "corruption") > 0 {
                    2.0
                } else {
                    1.0
                };
                cost += cascade * (self.w.pool + self.w.corruption);
            }
            let lost = (dmg / pop_health).min(g.tokens(h, "population"));
            cost += lost as f64 * self.w.population;
        }
        cost
    }
}
