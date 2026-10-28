//! Team agents for simulation: the skill ladder. One agent drives every seat,
//! with hands and drafts open (as at a table where the team talks it
//! through), and makes the "players choose" decisions too. Agents play
//! through the public API only (`Rules::legal_actions_into`, `apply_mut`,
//! reading the state) and see only what the printed game shows: before any
//! search they hide the decks' order and the dice behind a guess of their
//! own ([`determinize`]).
//!
//! Each agent has its own RNG, seeded from the game seed on a stream no
//! engine purpose uses, so a game still replays from seed + actions and
//! results do not depend on thread count or game order.
//!
//! The ladder, weakest first:
//!
//! - `random`: uniform over legal actions. A floor, not a model of a weak team.
//! - `greedy`: one action at a time, each scored by the heuristic after a
//!   short look ahead (`depth` actions, the choices an action asks for
//!   resolved best-first up to `choices` deep), averaged over `samples`
//!   guesses of the hidden cards.
//! - `beam`: beam search over the rest of the round's action sequences
//!   (`width` kept per step, at most `depth` steps), each scored by the
//!   heuristic at the start of the next round: the end-of-round preview.
//!   It plans once and follows the plan while the table matches it.
//! - `rollout`: the beam's `candidates` best end-of-round positions, each
//!   continued by `rollouts` greedy playouts for `horizon` more rounds (on
//!   fresh guesses of the hidden cards); the best mean wins. It looks across
//!   rounds where the beam stops at the round's end.
//!
//! An agent is named by a spec, `kind[:key=value,...]`: `greedy`,
//! `beam:width=4`, `rollout:rollouts=8,horizon=3`, and `w.<weight>=V` to
//! change a heuristic weight (see [`eval::Weights`]).

pub mod determinize;
pub mod eval;
pub mod search;

use std::collections::BTreeMap;

use aic_core::{fnv1a64, Action, Agent, GameState, Rules};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub use search::{Beam, Greedy};

/// Uniform over legal actions. Seeded per game, so results do not depend on
/// thread count or game order. A floor, not a model of a weak player.
pub struct RandomAgent(ChaCha8Rng);

impl RandomAgent {
    pub fn new(game_seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(game_seed);
        // A stream no engine purpose uses, so the agent never mirrors the game's dice.
        rng.set_stream(0xa6e7_a6e7);
        RandomAgent(rng)
    }
}

impl Agent for RandomAgent {
    fn choose(&mut self, _rules: &Rules, _state: &GameState, legal: &[Action]) -> usize {
        (self.0.next_u64() % legal.len() as u64) as usize
    }
}

/// The agent's own RNG for one game: the game seed on a stream named after
/// the agent kind, apart from every engine purpose's stream.
pub fn agent_rng(game_seed: u64, kind: &str) -> ChaCha8Rng {
    let mut rng = ChaCha8Rng::seed_from_u64(game_seed);
    rng.set_stream(fnv1a64(format!("agent:{kind}").as_bytes()));
    rng
}

/// A parsed agent spec: `kind[:key=value,...]`.
#[derive(Debug, Clone, PartialEq)]
pub struct AgentSpec {
    pub kind: String,
    pub params: BTreeMap<String, f64>,
    pub weights: BTreeMap<String, f64>,
}

/// A budget parameter: name, default, what it controls.
pub type Param = (&'static str, f64, &'static str);

/// The skill tiers, weakest first, with each one's budget parameters and
/// their defaults.
pub const LADDER: &[(&str, &[Param])] = &[
    ("random", &[]),
    (
        "greedy",
        &[
            (
                "samples",
                1.0,
                "guesses of the hidden cards averaged per decision",
            ),
            ("depth", 1.0, "actions looked ahead (1 = the action itself)"),
            (
                "choices",
                2.0,
                "levels of an action's own choices resolved best-first before scoring",
            ),
        ],
    ),
    (
        "beam",
        &[
            ("width", 6.0, "sequences kept per step"),
            (
                "depth",
                120.0,
                "most steps searched (a round is about 25-45 decisions)",
            ),
            (
                "branch",
                0.0,
                "children kept per sequence, best first (0 = all)",
            ),
        ],
    ),
    (
        "rollout",
        &[
            ("width", 4.0, "beam width for the round's candidates"),
            ("depth", 120.0, "most beam steps"),
            ("branch", 0.0, "children kept per sequence (0 = all)"),
            ("candidates", 3.0, "end-of-round positions rolled out"),
            ("rollouts", 4.0, "greedy playouts per candidate"),
            (
                "horizon",
                2.0,
                "rounds each playout looks past the current one",
            ),
        ],
    ),
];

impl AgentSpec {
    pub fn parse(spec: &str) -> Result<AgentSpec, String> {
        let (kind, rest) = spec.split_once(':').unwrap_or((spec, ""));
        let Some((_, known)) = LADDER.iter().find(|(k, _)| *k == kind) else {
            let names: Vec<&str> = LADDER.iter().map(|(k, _)| *k).collect();
            return Err(format!(
                "unknown agent {kind:?}; known: {}",
                names.join(", ")
            ));
        };
        let mut params: BTreeMap<String, f64> =
            known.iter().map(|(k, v, _)| (k.to_string(), *v)).collect();
        let mut weights = BTreeMap::new();
        for kv in rest.split(',').filter(|s| !s.is_empty()) {
            let (k, v) = kv
                .split_once('=')
                .ok_or_else(|| format!("agent {spec:?}: {kv:?} is not key=value"))?;
            let v: f64 = v
                .parse()
                .map_err(|_| format!("agent {spec:?}: {k}: not a number: {v:?}"))?;
            if let Some(w) = k.strip_prefix("w.") {
                if kind == "random" || !eval::Weights::default().set(w, v) {
                    return Err(format!("agent {spec:?}: no heuristic weight {w:?}"));
                }
                weights.insert(w.to_string(), v);
            } else if params.contains_key(k) {
                if v < 0.0 || !v.is_finite() {
                    return Err(format!("agent {spec:?}: {k} must be >= 0"));
                }
                params.insert(k.to_string(), v);
            } else {
                let names: Vec<&str> = known.iter().map(|(k, _, _)| *k).collect();
                return Err(format!(
                    "agent {spec:?}: {kind} has no parameter {k:?}; it has: {}",
                    if names.is_empty() {
                        "none".to_string()
                    } else {
                        names.join(", ")
                    }
                ));
            }
        }
        Ok(AgentSpec {
            kind: kind.to_string(),
            params,
            weights,
        })
    }

    /// The canonical spec: the kind, then every parameter that differs from
    /// its default, then weights.
    pub fn label(&self) -> String {
        let known = LADDER
            .iter()
            .find(|(k, _)| *k == self.kind)
            .map_or(&[][..], |l| l.1);
        let mut parts: Vec<String> = known
            .iter()
            .filter(|(k, d, _)| self.params[*k] != *d)
            .map(|(k, _, _)| format!("{k}={}", self.params[*k]))
            .collect();
        parts.extend(self.weights.iter().map(|(k, v)| format!("w.{k}={v}")));
        if parts.is_empty() {
            self.kind.clone()
        } else {
            format!("{}:{}", self.kind, parts.join(","))
        }
    }

    /// Position on the ladder (0 = random).
    pub fn tier(&self) -> usize {
        LADDER
            .iter()
            .position(|(k, _)| *k == self.kind)
            .unwrap_or(0)
    }

    fn p(&self, k: &str) -> usize {
        self.params[k].round() as usize
    }

    fn weights(&self) -> eval::Weights {
        let mut w = eval::Weights::default();
        for (k, v) in &self.weights {
            w.set(k, *v);
        }
        w
    }

    /// A fresh agent for one game.
    pub fn build(&self, rules: &Rules, game_seed: u64) -> Box<dyn Agent + Send> {
        let rng = agent_rng(game_seed, &self.kind);
        let ev = eval::Eval::new(rules, self.weights());
        match self.kind.as_str() {
            "random" => Box::new(RandomAgent::new(game_seed)),
            "greedy" => Box::new(Greedy::new(
                ev,
                rng,
                self.p("samples").max(1),
                self.p("depth").max(1),
                self.p("choices"),
            )),
            "beam" => Box::new(Beam::new(
                ev,
                rng,
                self.p("width").max(1),
                self.p("depth").max(1),
                self.p("branch"),
                None,
            )),
            "rollout" => Box::new(Beam::new(
                ev,
                rng,
                self.p("width").max(1),
                self.p("depth").max(1),
                self.p("branch"),
                Some(search::RolloutCfg {
                    candidates: self.p("candidates").max(1),
                    rollouts: self.p("rollouts").max(1),
                    horizon: self.p("horizon"),
                }),
            )),
            k => unreachable!("agent kind {k} is checked in parse"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aic_core::{play, replay, Config};

    fn rules(difficulty: i64) -> Rules {
        let text = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../configs/baseline.json"
        ))
        .unwrap();
        let mut c = Config::from_json(&text).unwrap();
        c.apply_override(&format!("adversary.difficulty={difficulty}"))
            .unwrap();
        Rules::new(&c).unwrap()
    }

    /// Games replay from seed + actions, and the same agent on the same seed
    /// plays the same game: its RNG is its own and seeded per game.
    #[test]
    fn agents_replay_and_repeat() {
        let r = rules(4);
        for spec in [
            "greedy",
            "beam:width=2",
            "rollout:width=2,candidates=2,rollouts=1,horizon=1",
        ] {
            let spec = AgentSpec::parse(spec).unwrap();
            let seed = 3;
            let (g, rec) = play(&r, seed, &mut *spec.build(&r, seed), 3000);
            assert!(g.is_over(), "{}", spec.label());
            assert_eq!(replay(&r, seed, &rec.actions).unwrap(), g);
            let (_, again) = play(&r, seed, &mut *spec.build(&r, seed), 3000);
            assert_eq!(rec.actions, again.actions, "{}", spec.label());
        }
    }

    /// An agent's decision is the same on two tables that differ only in
    /// what the printed game hides (deck order and the dice).
    #[test]
    fn decisions_ignore_hidden_information() {
        let r = rules(0);
        let mut legal = Vec::new();
        for spec in ["greedy", "beam:width=2"] {
            let spec = AgentSpec::parse(spec).unwrap();
            for seed in 0..3 {
                // Some way into the game, so piles have moved.
                let (mut a, _) = play(&r, seed, &mut RandomAgent::new(seed), 12);
                if a.is_over() {
                    a = r.new_game(seed);
                }
                let mut b = a.clone();
                for (k, p) in b.piles.iter_mut() {
                    if k.starts_with("cards.deck.") || k.starts_with("threats.deck.") {
                        p.reverse();
                    }
                }
                for (purpose, s) in b.rng.iter_mut() {
                    *s = aic_core::rng::Stream::new(seed + 77, purpose);
                }
                r.legal_actions_into(&a, &mut legal);
                let ia = spec.build(&r, 9).choose(&r, &a, &legal);
                let ib = spec.build(&r, 9).choose(&r, &b, &legal);
                assert_eq!(ia, ib, "{} seed {seed}", spec.label());
            }
        }
    }

    #[test]
    fn specs() {
        let s = AgentSpec::parse("beam:width=4,w.pool=20").unwrap();
        assert_eq!(s.label(), "beam:width=4,w.pool=20");
        assert_eq!(AgentSpec::parse("greedy").unwrap().label(), "greedy");
        assert!(AgentSpec::parse("greedy:width=3").is_err());
        assert!(AgentSpec::parse("greedy:w.nope=3").is_err());
        assert!(AgentSpec::parse("mcts").is_err());
        assert!(AgentSpec::parse("random:samples=2").is_err());
    }
}
