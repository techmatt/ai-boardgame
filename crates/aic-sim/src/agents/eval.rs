//! The heuristic value of a state, from the team's point of view. It reads
//! only what the table shows: pieces on the board, tracks, resources, open
//! hands and buildings, active threats and what is paid on them, deck
//! sizes and the face-up adversary card. Units are roughly "credits": a
//! credit in hand is worth `credits` (0.6) because credits buy cards and
//! standard actions that do more.
//!
//! The board (014): cities are clean, aligned (a player cube) or corrupted
//! (the adversary's cube; every city corrupted loses). Adversary ambassadors
//! gather on clean cities until `adversary.corrupt_at` corrupts them, divert
//! the production of the pieces they sit on (a factory builds a troop),
//! and come out of a supply that loses the game when it runs dry.
//!
//! The weights are agent parameters, not rules: they live in [`Weights`]
//! and an experiment can override any of them (`greedy:w.corrupted=40`).
//!
//! **Horizon mode** (`w.horizon` > 0; the `engine` agent's default): income
//! and buildings are valued by what they will pay over the rounds the game
//! is expected to last ([`Eval::rounds_left`]: the adversary deck's clock,
//! cut short by how soon the team is on course to win and by the danger it
//! is in), not by a flat weight; a card in hand is valued by whether it can
//! be paid for before the game ends; and resources lose their value as the
//! end nears (nothing is left to buy).

use std::collections::BTreeMap;

use aic_core::content::{Passive, Side, Twist, ANTIMATTER, CREDITS, ENERGY, INSIGHT, RESOURCES};
use aic_core::modules::common::{
    self, ADV_AMBASSADOR, ADV_CUBE, AMBASSADORS, FACTORY, HARDEN, STATION, TROOP,
};
use aic_core::modules::{adversary, ai, alignment, board, cards, economy, plan, threats};
use aic_core::{GameState, Rules};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    /// A finished game: won or lost.
    pub win: f64,
    /// Added per adversary card left (deck and face-up) on a win (a wider margin).
    pub win_margin: f64,
    /// Each corrupted city (it builds ambassadors, diverts its energy to
    /// Discord, and every city corrupted loses).
    pub corrupted: f64,
    /// Extra per city short of 2 not corrupted (all corrupted = loss).
    pub cities_danger: f64,
    /// Each aligned city (a player cube: extra energy for its owner).
    pub aligned: f64,
    /// Each adversary ambassador on the board (the bar-0 win needs none).
    pub adv_ambassador: f64,
    /// Ambassadors on a city not yet corrupted: this share of `corrupted`
    /// times (ambassadors / the next corrupt threshold)², capped at 1.
    pub pressure: f64,
    /// Each adversary ambassador on a player piece, base or factory (it
    /// diverts the piece's production toward Discord).
    pub on_piece: f64,
    /// Extra per adversary ambassador on a factory (it builds a troop).
    pub on_factory: f64,
    /// Each adversary troop (it destroys a building every round).
    pub troop: f64,
    /// Per ambassador of slack short of 4 between the supply and the next
    /// build's placements.
    pub supply: f64,
    /// Per ambassador the supply is short of the next build (starved = loss).
    pub supply_danger: f64,
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
    /// Each visible adversary project's fill, as a share of its cost (a
    /// cost: it will resolve; 017).
    pub project: f64,
    /// Each player presence on the board (reach, site claims, a share of
    /// city energy; its track income is counted as income).
    pub data_centre: f64,
    /// Each node on the board (a cost: adversary compute, contests presence,
    /// blocks the purge).
    pub node: f64,
    /// Each player-owned factory (hardware each production step); an
    /// adversary-owned one costs as much.
    pub owned_factory: f64,
    /// Each hardware a seat holds.
    pub hardware: f64,
    /// Each city, base or station hex inside some seat's reach.
    pub reach: f64,
    /// Each research station a player owns; a corrupted one costs as much.
    pub station: f64,
    /// Each player cube on a megaproject; an adversary cube costs half.
    pub project_cube: f64,
    pub factory: f64,
    /// Each hardening token.
    pub harden: f64,
    /// Each player ambassador in a seat's supply.
    pub ambassador: f64,
    /// Each player ambassador on the board (shields a building, cancels an
    /// arriving adversary ambassador, unites a base).
    pub ambassador_board: f64,
    /// Player ambassadors on a corrupted city: this share of `corrupted`
    /// per ambassador of the `board.free_city_at` that free it.
    pub freeing: f64,
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
    /// Horizon mode, when above 0: each unit of income (AI tracks,
    /// buildings, rewards) is worth `horizon` × the resource's own weight ×
    /// the rounds left, replacing `income`. 0 = the flat weights above.
    pub horizon: f64,
    /// Horizon mode: Sway toward Unity the team is assumed to gain per round
    /// before it has a record of its own (round 1), for the win estimate.
    /// Was `opp_rate` (still accepted).
    #[serde(alias = "opp_rate")]
    pub unity_rate_start: f64,
    /// Horizon mode: rounds a win needs after the Unity target is reached.
    pub win_lag: f64,
    /// Horizon mode: chance of losing per round per unit of danger (an
    /// unpaid threat per country, the supply or the cities at their edge);
    /// the rounds left are the expected survival over the clock.
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
    /// Horizon mode: board production (energy of aligned cities, compute of
    /// presence not diverted) counts this share of income over the
    /// rounds left, on top of the flat piece weights.
    pub production: f64,
    /// Horizon mode, when above 0: cards in hand wait their turn for
    /// insight (each needs the insight of the cards before it too).
    pub insight_wait: f64,
    /// The deck clock (011), any mode: per round the team's projected win
    /// (Unity still needed at its rate so far, plus `win_lag`) comes
    /// after the adversary deck runs out. 0 = off.
    pub clock: f64,
}

/// Weights renamed since experiments named them: (former name, name).
const RENAMED: [(&str, &str); 1] = [("opp_rate", "unity_rate_start")];

/// A card's cost in credits: each antimatter counts this many credits...
const ANTIMATTER_COST: i64 = 3;
/// ...and each insight this many.
const INSIGHT_COST: i64 = 2;
/// Flat mode: a point of track income is worth this many of the resource's
/// own points for antimatter and insight (1 for the rest), times `income`.
const ANTIMATTER_INCOME_UNIT: f64 = 3.0;
const INSIGHT_INCOME_UNIT: f64 = 1.5;

impl Default for Weights {
    fn default() -> Self {
        Weights {
            win: 1000.0,
            win_margin: 5.0,
            // 014: chosen by hand for the new board, not yet swept.
            corrupted: 30.0,
            cities_danger: 60.0,
            aligned: 8.0,
            adv_ambassador: 3.0,
            pressure: 0.7,
            on_piece: 2.0,
            on_factory: 6.0,
            troop: 12.0,
            supply: 5.0,
            supply_danger: 40.0,
            blocker: 2.0,
            // 013: a Unity tip replaces about 4 opposition points (24) and
            // brings a card; a Discord tip brings one of the adversary's.
            unity_tip: 24.0,
            discord_tip: 16.0,
            level: 10.0,
            // 017: chosen by hand, not swept.
            project: 8.0,
            data_centre: 4.0,
            // 016: chosen by hand for the r7 board, not swept.
            node: 6.0,
            owned_factory: 4.0,
            hardware: 2.0,
            reach: 0.5,
            station: 5.0,
            project_cube: 2.0,
            factory: 0.5,
            harden: 1.0,
            // 018: was 3, below the 6 credits (3.6) an ambassador costs, so no
            // agent ever bought one and corrupted cities were never freed.
            ambassador: 5.0,
            ambassador_board: 2.5,
            freeing: 0.8,
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
            horizon: 0.0,
            unity_rate_start: 2.0,
            win_lag: 1.0,
            risk: 0.1,
            ability: 0.5,
            spend_horizon: 2.0,
            locked: 0.5,
            // Horizon mode only; the engine turns it on (see `engine`).
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
            // 014 (d0, 2 countries, 48 seeds, the checkpoint's map): 1 won
            // 64.6%, 0 won ~37% (32 seeds); 2 won 66.7%. On the map as since
            // edited, level within noise at d1-d3 (32 seeds). Energy and
            // compute now come from the board.
            production: 1.0,
            ..Weights::default()
        }
    }

    /// Sets one weight by name (or a former name, see [`RENAMED`]); false
    /// if there is no such weight.
    pub fn set(&mut self, name: &str, v: f64) -> bool {
        let name = RENAMED
            .iter()
            .find(|(old, _)| *old == name)
            .map_or(name, |(_, new)| *new);
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

/// What the next adversary build and corrupt steps will do, as the face-up
/// card shows: ambassadors the build places, and the corrupt threshold.
struct Forecast {
    placements: i64,
    corrupt_at: i64,
}

/// A [`Weights`] bound to one game's rules, with per-card values cached.
pub struct Eval {
    pub w: Weights,
    card_value: BTreeMap<String, f64>,
    seats: usize,
    /// Unity tip count the team aims for: the outright win, or with none
    /// the last bar drop.
    target_tips: i64,
    corrupt_at: i64,
    free_at: i64,
    city_energy: i64,
    aligned_bonus: i64,
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
                    + ANTIMATTER_COST * world.amount(&c.antimatter)
                    + INSIGHT_COST * world.amount(&c.insight);
                (c.id.clone(), cost.max(0) as f64)
            })
            .collect();
        let seats = rules.config.countries as usize;
        let outright = world.int("alignment.unity_win");
        let target_tips = if outright > 0 {
            outright
        } else {
            world
                .list("alignment.unity_ease")
                .last()
                .copied()
                .unwrap_or(1)
        };
        Eval {
            w,
            card_value,
            seats,
            target_tips,
            corrupt_at: world.int("adversary.corrupt_at"),
            free_at: world.int("board.free_city_at").max(1),
            city_energy: world.int("board.city_energy"),
            aligned_bonus: world.int("board.aligned_bonus"),
        }
    }

    fn cost(&self, id: &str) -> f64 {
        self.card_value.get(id).copied().unwrap_or(0.0)
    }

    /// Adversary cards still to come: the face-up one and the deck.
    fn cards_left(g: &GameState) -> usize {
        g.pile(adversary::DECK).len() + g.pile(adversary::NEXT).len()
    }

    /// The next build and corrupt steps, read off the face-up card (public).
    /// The build is the core's own forecast (`plan::next_build`).
    fn forecast(&self, rules: &Rules, g: &GameState) -> Forecast {
        let world = rules.world();
        let content = world.content;
        let sooner: i64 = g
            .pile(adversary::NEXT)
            .first()
            .and_then(|id| content.adversary_index(id))
            .map_or(0, |i| {
                content.adversary[i]
                    .twists
                    .iter()
                    .filter_map(|t| match t {
                        Twist::CorruptSooner { amount } => Some(*amount),
                        _ => None,
                    })
                    .sum()
            });
        Forecast {
            placements: plan::next_build(&world, g).iter().map(|&(_, n)| n).sum(),
            corrupt_at: (self.corrupt_at - sooner).max(1),
        }
    }

    /// The value of `g`. Finished games score `±win` plus a margin bonus.
    pub fn value(&self, rules: &Rules, g: &GameState) -> f64 {
        let w = &self.w;
        if let Some(o) = &g.outcome {
            return if o.won {
                w.win + w.win_margin * Self::cards_left(g) as f64
            } else {
                -w.win
            };
        }
        let world = rules.world();
        let fc = self.forecast(rules, g);
        let mut v = 0.0;
        let (mut corrupted, mut safe_cities) = (0i64, 0i64);
        let mut reach = 0i64;
        for h in 0..g.hexes.len() {
            let city = common::is_city(&world, h);
            let adv = g.tokens(h, ADV_AMBASSADOR);
            let bad = common::corrupted(g, h);
            if city {
                if bad {
                    corrupted += 1;
                    let mine: i64 = (0..g.seats.len())
                        .map(|s| g.tokens(h, &common::ambassador(s)))
                        .sum();
                    let share = (mine as f64 / self.free_at as f64).min(1.0);
                    v += w.corrupted * w.freeing * share;
                } else {
                    safe_cities += 1;
                    if common::cube_owner(g, h).is_some() {
                        v += w.aligned;
                    }
                    let p = (adv as f64 / fc.corrupt_at as f64).min(1.0);
                    v -= w.corrupted * w.pressure * p * p;
                }
            }
            v -= w.adv_ambassador * adv as f64;
            if adv > 0 {
                let piece = common::player_owned(g, h)
                    || common::is_base(&world, h)
                    || g.tokens(h, FACTORY) > 0
                    || g.tokens(h, STATION) > 0;
                if piece {
                    v -= w.on_piece * adv as f64;
                }
                if g.tokens(h, FACTORY) > 0 {
                    v -= w.on_factory;
                }
            }
            v -= w.troop * g.tokens(h, TROOP) as f64;
            if g.tokens(h, STATION) > 0 {
                if bad {
                    v -= w.station;
                } else if common::cube_owner(g, h).is_some() {
                    v += w.station;
                }
            }
            v += w.factory * g.tokens(h, FACTORY) as f64 + w.harden * g.tokens(h, HARDEN) as f64;
            if g.tokens(h, FACTORY) > 0 {
                if bad {
                    v -= w.owned_factory;
                } else if common::cube_owner(g, h).is_some() {
                    v += w.owned_factory;
                }
            }
            v -= w.node * g.tokens(h, common::NODE) as f64;
            if common::presence_owner(g, h).is_some() {
                v += w.data_centre;
            }
            for s in 0..self.seats {
                v += w.ambassador_board * g.tokens(h, &common::ambassador(s)) as f64;
            }
            let counts = city || common::is_base(&world, h) || g.tokens(h, STATION) > 0;
            if counts && (0..self.seats).any(|s| common::in_reach(&world, g, s, h)) {
                reach += 1;
            }
        }
        v -= w.corrupted * corrupted as f64;
        v -= w.cities_danger * (2 - safe_cities).max(0) as f64;
        v += w.reach * reach as f64;
        // The supply: the next build must find its ambassadors there.
        let slack = g.var(adversary::SUPPLY) - fc.placements;
        v -= w.supply_danger * (-slack).max(0) as f64 + w.supply * (4 - slack).clamp(0, 4) as f64;
        // Megaprojects: each player cube pays when the project completes.
        for c in 0..world.map.countries.len() {
            if g.var(&board::project_done(c)) > 0 {
                continue;
            }
            let mine: i64 = (0..self.seats)
                .map(|s| g.var(&board::project_key(c, Some(s))))
                .sum();
            let theirs = g.var(&board::project_key(c, None));
            v += w.project_cube * (mine as f64 - 0.5 * theirs as f64);
        }
        for s in 0..self.seats {
            v += w.ambassador * g.resource(s, AMBASSADORS) as f64;
            v += w.hardware * g.resource(s, common::HARDWARE) as f64;
        }
        // The win bar: what still stands between the team and a clear board.
        let level = alignment::bar_level(&world, g);
        v -= w.blocker * adversary::purge_count(&world, g, level) as f64;
        // Alignment: tips made, and each bucket's share of its next tip.
        for (side, weight) in [(Side::Unity, w.unity_tip), (Side::Discord, -w.discord_tip)] {
            let cap = alignment::capacity(&world, g, side).max(1) as f64;
            let fill = (g.var(alignment::bucket(side)) as f64 / cap).min(1.0);
            v += weight * (g.var(alignment::tips_key(side)) as f64 + fill);
        }
        v += w.level * level as f64;
        // A resolved (or ongoing) project counts in full, so letting one
        // fill never scores better than holding it back.
        for id in adversary::projects(g) {
            let cost = adversary::project_cost(&world, g, &id).max(1) as f64;
            v -= w.project * g.var(&adversary::project_key(&id)) as f64 / cost;
        }
        let resolved = g.stats.counters.get("adversary.projects_resolved");
        v -= w.project * resolved.copied().unwrap_or(0) as f64;
        // Added in place, term by term, so the flat agents' values (and so
        // their games) do not depend on horizon mode.
        let unpaid = self.unpaid_threats(rules, g);
        if w.horizon > 0.0 {
            self.horizon_value(rules, g, &unpaid, safe_cities, slack, &mut v);
        } else {
            self.flat_value(rules, g, &mut v);
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
            let prefix = threats::paid_prefix(&t);
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
    /// deck's clock (each card left is one more round), cut short by when
    /// the team is on course to win (Unity still needed at its rate so far,
    /// plus `win_lag`), then the expected rounds survived at a per-round
    /// loss chance of `risk` per unit of danger (unpaid threats per
    /// country, the supply or the cities at their edge).
    fn rounds_left(
        &self,
        rules: &Rules,
        g: &GameState,
        unpaid: &[f64],
        safe_cities: i64,
        slack: i64,
    ) -> f64 {
        let w = &self.w;
        let clock = Self::cards_left(g) as f64;
        let (points, bar) = self.unity_progress(rules, g);
        let to_win = ((bar - points).max(0.0) / self.unity_rate(rules, g)).ceil() + w.win_lag;
        let rounds = clock.min(to_win).max(0.0);
        let world = rules.world();
        let countries = world.map.countries.len().max(1) as f64;
        let mut danger = unpaid.iter().sum::<f64>() / countries;
        if slack <= 2 {
            danger += 0.5;
        }
        if safe_cities <= 2 {
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
    /// size), and the Sway the target (the outright Unity win) needs.
    fn unity_progress(&self, rules: &Rules, g: &GameState) -> (f64, f64) {
        let world = rules.world();
        let per = (world.int("alignment.tip_per_player") * g.seats.len() as i64) as f64;
        let tips = g.var(alignment::tips_key(Side::Unity)) as f64;
        let points = tips * per + g.var(alignment::bucket(Side::Unity)) as f64;
        (points, self.target_tips as f64 * per)
    }

    /// Unity gained per round so far, in Sway (`unity_rate_start` before the
    /// team has a record of its own). Sway held may include this round's, so
    /// this round counts as played.
    fn unity_rate(&self, rules: &Rules, g: &GameState) -> f64 {
        let w = &self.w;
        let points = self.unity_progress(rules, g).0;
        if g.round > 1 {
            (points / g.round as f64).max(w.unity_rate_start * 0.25)
        } else {
            w.unity_rate_start
        }
        .max(0.1)
    }

    /// The deck-clock penalty: `clock` per round the team, at its Unity
    /// rate so far, would still be short of its Unity target when the deck
    /// runs out. With `k` adversary cards left (the face-up one included),
    /// `k` action phases come after this one (the build step after the last
    /// loses); this round's points count as they are scored, not as a round
    /// still to come, so in the last round every point short costs
    /// `clock` / rate.
    fn clock_penalty(&self, rules: &Rules, g: &GameState) -> f64 {
        if self.w.clock == 0.0 {
            return 0.0;
        }
        let clock = Self::cards_left(g) as f64;
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
                    "antimatter" => ANTIMATTER_INCOME_UNIT,
                    "insight" => INSIGHT_INCOME_UNIT,
                    _ => 1.0,
                };
                *v += w.income * unit * n as f64;
            }
            for id in g.pile(&cards::hand(s)) {
                *v += w.hand + w.hand_cost * self.cost(id);
            }
            for id in g.pile(&cards::drafted(s)) {
                *v += w.drafted * (w.hand + w.hand_cost * self.cost(id));
            }
            for id in g.pile(&cards::built(s)) {
                *v += w.built + w.built_cost * self.cost(id);
            }
        }
    }

    /// What the board makes each seat a round if production came now, in
    /// `RESOURCES` order: the energy of its aligned cities and the compute
    /// of its presence, less what adversary ambassadors there divert.
    fn board_production(&self, rules: &Rules, g: &GameState) -> Vec<[i64; RESOURCES.len()]> {
        let world = rules.world();
        let mut out = vec![[0i64; RESOURCES.len()]; self.seats];
        for h in 0..g.hexes.len() {
            if g.tokens(h, ADV_AMBASSADOR) > 0 || g.tokens(h, ADV_CUBE) > 0 {
                continue;
            }
            if common::is_city(&world, h) {
                if let Some(s) = common::cube_owner(g, h).filter(|&s| s < self.seats) {
                    out[s][ENERGY] += self.city_energy + self.aligned_bonus;
                }
            }
        }
        out
    }

    /// Per-seat resources, income, hands and buildings in horizon mode.
    fn horizon_value(
        &self,
        rules: &Rules,
        g: &GameState,
        unpaid: &[f64],
        safe_cities: i64,
        slack: i64,
        out: &mut f64,
    ) {
        let w = &self.w;
        let world = rules.world();
        let left = self.rounds_left(rules, g, unpaid, safe_cities, slack);
        let prod = if w.production != 0.0 {
            self.board_production(rules, g)
        } else {
            vec![[0; RESOURCES.len()]; self.seats]
        };
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
            // the rounds left, less building upkeep; board production counts
            // `production` of its own.
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
            for (i, n) in prod[s].iter().enumerate() {
                v += w.horizon * w.production * left * unit[i] * *n as f64;
            }
            for id in g.pile(&cards::built(s)) {
                let d = cards::def(&world, id);
                let pays = d.passives.iter().any(|p| {
                    matches!(
                        p,
                        Passive::Income { .. }
                            | Passive::IncomePerTag { .. }
                            | Passive::IncomeIfTags { .. }
                    )
                });
                v += w.built + w.built_cost * self.cost(id);
                if !pays {
                    v += w.ability * left;
                }
            }
            // A card in hand is worth its value times the chance of paying
            // for it before the game ends, from what the seat has and earns.
            // Insight is spent one card at a time: each card waits for the
            // insight the cards before it in hand take.
            let tier2 = ai::tier2(&world, g, s);
            let earn_credits = income[CREDITS];
            let mut insight_queue = 0i64;
            // The same for every building in hand: worked out once.
            let discount = cards::building_discount(&world, g, s);
            let mut play = |id: &str| -> f64 {
                let d = cards::def(&world, id);
                let mut p = 1.0;
                let insight = world.amount(&d.insight);
                insight_queue += insight;
                let insight_need = if w.insight_wait > 0.0 {
                    insight_queue
                } else {
                    0
                };
                for (have, need, earn) in [
                    // The AI's discount: energy or compute spent as credits.
                    (
                        res("credits")
                            + cards::discount_room(&world, g, s, d)
                                .map_or(0, |(_, rate, k)| rate * k),
                        cards::discounted_cost(&world, d, discount),
                        earn_credits,
                    ),
                    (
                        res("antimatter"),
                        world.amount(&d.antimatter),
                        income[ANTIMATTER],
                    ),
                    (res("insight"), insight_need, income[INSIGHT]),
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
            for id in g.pile(&cards::drafted(s)) {
                v += w.drafted * play(id);
            }
        }
        *out += v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renamed_weight_keeps_its_old_name() {
        let mut w = Weights::default();
        assert!(w.set("opp_rate", 1.5));
        assert_eq!(w.unity_rate_start, 1.5);
        assert!(w.set("unity_rate_start", 0.5));
        assert_eq!(w.unity_rate_start, 0.5);
        let mut v = serde_json::to_value(Weights::default()).unwrap();
        let m = v.as_object_mut().unwrap();
        m.remove("unity_rate_start");
        m.insert("opp_rate".into(), serde_json::json!(3.0));
        let w: Weights = serde_json::from_value(v).unwrap();
        assert_eq!(w.unity_rate_start, 3.0);
    }
}
