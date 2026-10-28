//! The heuristic value of a state, from the team's point of view. It reads
//! only what the table shows: tokens, tracks, resources, open hands and
//! buildings, active threats and what is paid on them, deck sizes. Units
//! are roughly "credits": a credit in hand is worth `credits` (0.6) because
//! credits buy cards that do more.
//!
//! The weights are agent parameters, not rules: they live in [`Weights`]
//! and an experiment can override any of them (`greedy:w.pool=20`).

use std::collections::BTreeMap;

use aic_core::modules::{adversary, ai, board, cards, opposition, threats};
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
    /// Each adversary piece that keeps the current opposition level's bar
    /// from being met (the win condition).
    pub blocker: f64,
    /// Each opposition point.
    pub opposition: f64,
    /// Each opposition level reached.
    pub level: f64,
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
            opposition: 6.0,
            level: 10.0,
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
        }
    }
}

impl Weights {
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
    max_level: i64,
    per_level: i64,
    /// The adversary variant: the Militarist's pieces and ravage forecast,
    /// or the spread variants' clusters or corrupted assets.
    style: adversary::Style,
    /// Spread variants: pieces allowed per opposition level (purge win).
    allowance: i64,
    /// Ravage damage of a drone, outpost and foundry (Militarist).
    damage: [i64; 3],
    /// Extra ravage damage where there is a foundry (difficulty 3+).
    foundry_bonus: i64,
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
            loss_percent: world.int("opposition.loss_population_percent"),
            max_level: world.int("opposition.max_level"),
            per_level: world.int("opposition.per_player") * seats as i64,
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
            foundry_bonus: if adversary::style(&world) == adversary::Style::Militarist && adversary::level(&world, 3) {
                world.int("adversary.foundry_bonus")
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
        let world = rules.world();
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
        let level = g.var(opposition::LEVEL).min(self.max_level);
        // A spread variant's piece weighs as an outpost; its purge bar
        // allows `allowance` pieces per level.
        v -= w.outpost * spread as f64;
        let blockers = match self.style {
            adversary::Style::Militarist => match level {
                0 => drones + outposts + foundries,
                1 => outposts + foundries,
                l if l < self.max_level => foundries,
                _ => 0,
            },
            _ if level >= self.max_level => 0,
            _ => (spread - level * self.allowance).max(0),
        };
        v -= w.blocker * blockers as f64;
        let points = g
            .var(opposition::POINTS)
            .min(self.per_level * self.max_level);
        v += w.opposition * points as f64 + w.level * level as f64;
        v += w.factory * factories as f64
            + w.data_centre * dcs as f64
            + w.processor * processors as f64;
        v += w.defense * defense as f64;
        for s in 0..self.seats {
            let res = |k: &str| g.resource(s, k) as f64;
            v += w.credits * res("credits")
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
                v += w.income * unit * n as f64;
            }
            for id in g.pile(&format!("cards.hand.{s}")) {
                v += w.hand + w.hand_cost * self.cost(id);
            }
            for id in g.pile(&format!("cards.drafted.{s}")) {
                v += w.drafted * (w.hand + w.hand_cost * self.cost(id));
            }
            for id in g.pile(&format!("cards.built.{s}")) {
                v += w.built + w.built_cost * self.cost(id);
            }
        }
        // The coming ravages are public (the ravage and build slots): what
        // they would cost if the table stayed as it is.
        for (slot, share) in [
            (adversary::SLOTS[2], w.forecast),
            (adversary::SLOTS[1], w.forecast_next),
        ] {
            if share != 0.0 {
                if let Some(id) = g.pile(slot).first() {
                    v -= share * self.ravage_cost(rules, g, id);
                }
            }
        }
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
            v -= w.threat * (1.0 - (paid as f64 / req as f64).min(1.0));
        }
        v
    }

    /// What a ravage with adversary card `id` would cost now, in heuristic
    /// units: corruption (and cascades) and population lost.
    fn ravage_cost(&self, rules: &Rules, g: &GameState, id: &str) -> f64 {
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
        let defense: Vec<i64> = (0..self.seats)
            .map(|s| cards::defense_value(&world, g, s))
            .collect();
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
                raw += self.foundry_bonus;
            }
            let prevented: i64 = defense
                .iter()
                .enumerate()
                .map(|(s, d)| g.tokens(h, &format!("defense:{s}")) * d)
                .sum();
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
