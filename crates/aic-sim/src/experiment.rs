//! The experiment harness: configs × variants × difficulties × agents over a
//! seed range, played on every core, with the measures of
//! `docs/principles.md` gathered per game. `report.rs` aggregates and writes
//! them out. See README.md ("Experiments") for the spec format.
//!
//! Everything a game measures is read from the public state or the actions
//! taken, so the engine needs no extra bookkeeping: card offers are read off
//! the draft packets, picks, keeps, plays, payments and gifts off the
//! actions, margins off the final table. Probes that simulate (decision
//! closeness, win probability) use their own RNG and determinize first, like
//! the agents, and never touch the game being played.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use aic_core::modules::{adversary, board, opposition};
use aic_core::{replay, Action, Config, GameState, Rules};
use serde::{Deserialize, Serialize};

use crate::agents::determinize::determinize;
use crate::agents::eval::{Eval, Weights};
use crate::agents::{agent_rng, search, AgentSpec};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    pub name: String,
    /// What the experiment is for; printed at the top of the report.
    #[serde(default)]
    pub note: String,
    /// Config files, relative to the working directory.
    pub configs: Vec<String>,
    /// Named override sets (`--set` syntax), each applied to every config.
    #[serde(default = "default_variants")]
    pub variants: Vec<Variant>,
    /// `adversary.difficulty` levels.
    #[serde(default = "default_difficulties")]
    pub difficulties: Vec<i64>,
    /// Agent specs, weakest first (see `agents/mod.rs`).
    pub agents: Vec<String>,
    #[serde(default, with = "seed_string")]
    pub first_seed: u64,
    /// Seeds per cell: `first_seed .. first_seed + games`.
    pub games: u64,
    #[serde(default = "default_max_actions")]
    pub max_actions: usize,
    #[serde(default)]
    pub winprob: WinProb,
    #[serde(default)]
    pub closeness: Closeness,
    /// Records saved per category (close losses, lock-ins, surprises).
    #[serde(default = "default_records")]
    pub records: usize,
}

mod seed_string {
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(v: &u64, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(v)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum N {
            S(String),
            N(u64),
        }
        match N::deserialize(d)? {
            N::N(n) => Ok(n),
            N::S(s) => s.parse().map_err(serde::de::Error::custom),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,
    #[serde(default)]
    pub set: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct WinProb {
    /// Games per cell (the first seeds) that get win-probability checkpoints;
    /// 0 turns them off.
    pub games: u64,
    /// Playouts per checkpoint (the start of each round).
    pub rollouts: usize,
    /// The agent that plays the playouts.
    pub agent: String,
}

impl Default for WinProb {
    fn default() -> Self {
        WinProb {
            games: 0,
            rollouts: 8,
            agent: "greedy".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Closeness {
    /// Probe every n-th decision that offers a real choice; 0 = off.
    pub every: usize,
    /// A decision is close when the best and second-best options score
    /// within this many heuristic points (about credits).
    pub gap: f64,
}

impl Default for Closeness {
    fn default() -> Self {
        Closeness { every: 1, gap: 2.0 }
    }
}

fn default_variants() -> Vec<Variant> {
    vec![Variant {
        name: "base".into(),
        set: Vec::new(),
    }]
}
fn default_difficulties() -> Vec<i64> {
    vec![0]
}
fn default_max_actions() -> usize {
    5000
}
fn default_records() -> usize {
    4
}

/// One cell of the grid: a config under a variant at a difficulty, played
/// by one agent.
pub struct Cell {
    pub config_path: String,
    pub variant: String,
    pub difficulty: i64,
    pub agent: AgentSpec,
    pub rules: Rules,
    /// Overrides applied, in order (variant, then difficulty).
    pub overrides: Vec<String>,
}

impl Cell {
    /// The cell without its agent: what a skill gap or luck share compares across.
    pub fn group(&self) -> (String, String, i64) {
        (
            self.config_path.clone(),
            self.variant.clone(),
            self.difficulty,
        )
    }
}

/// Builds every cell, checking every config, override and agent first.
pub fn cells(spec: &Spec) -> Result<Vec<Cell>, String> {
    if spec.configs.is_empty()
        || spec.agents.is_empty()
        || spec.variants.is_empty()
        || spec.difficulties.is_empty()
    {
        return Err(
            "configs, variants, difficulties and agents must each have at least one entry".into(),
        );
    }
    if spec.games == 0 {
        return Err("games must be at least 1".into());
    }
    if spec.first_seed.checked_add(spec.games - 1).is_none() {
        return Err("first_seed + games overflows u64".into());
    }
    let agents: Vec<AgentSpec> = spec
        .agents
        .iter()
        .map(|a| AgentSpec::parse(a))
        .collect::<Result<_, _>>()?;
    if spec.winprob.games > 0 {
        AgentSpec::parse(&spec.winprob.agent).map_err(|e| format!("winprob.agent: {e}"))?;
        if spec.winprob.rollouts == 0 {
            return Err("winprob.rollouts must be at least 1 (or winprob.games 0 for none)".into());
        }
    }
    if !spec.closeness.gap.is_finite() || spec.closeness.gap < 0.0 {
        return Err("closeness.gap must be a number >= 0".into());
    }
    let mut out = Vec::new();
    for path in &spec.configs {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let base = Config::from_json(&text).map_err(|e| format!("{path}: {e}"))?;
        for v in &spec.variants {
            for &d in &spec.difficulties {
                let mut overrides = v.set.clone();
                overrides.push(format!("adversary.difficulty={d}"));
                let mut c = base.clone();
                for o in &overrides {
                    c.apply_override(o)
                        .map_err(|e| format!("{path} ({}): {e}", v.name))?;
                }
                for a in &agents {
                    let rules = Rules::new(&c)
                        .map_err(|e| format!("{path} ({}, difficulty {d}): {e}", v.name))?;
                    out.push(Cell {
                        config_path: path.clone(),
                        variant: v.name.clone(),
                        difficulty: d,
                        agent: a.clone(),
                        rules,
                        overrides: overrides.clone(),
                    });
                }
            }
        }
    }
    Ok(out)
}

/// Everything measured in one game.
#[derive(Debug, Clone, Serialize)]
pub struct GameResult {
    pub cell: usize,
    #[serde(with = "seed_string")]
    pub seed: u64,
    pub won: bool,
    /// End reason, or `timeout`.
    pub reason: String,
    pub rounds: u32,
    pub decisions: u32,
    pub options_total: u64,
    pub forced: u32,
    /// Decisions probed for closeness, how many were close, and the sum of
    /// best-minus-second gaps (capped at 50 so one blunder-or-win option
    /// does not swamp the mean).
    pub probed: u32,
    pub close: u32,
    pub gap_sum: f64,
    pub margin: Margin,
    pub adversary_ops: Vec<u32>,
    pub setup_ops: u32,
    pub counters: BTreeMap<String, i64>,
    /// Per card: times offered in a draft packet, drafted, kept, played,
    /// the sum of its pick positions (1 = a seat's first pick of the
    /// round), and times drawn (paid compute for). Every dealt card is
    /// drafted by someone (packets are passed until empty), so how early it
    /// goes is the draft's signal.
    pub cards: BTreeMap<String, [u32; 6]>,
    pub gives: u32,
    pub pays: u32,
    pub withdraws: u32,
    /// Each seat's AI and buildings at the end, `builder:a+b|protector:c`.
    pub build: String,
    /// Win probability at the start of each round (round 1 first), from playouts.
    pub winprob: Vec<f64>,
    pub ms: f64,
    /// Of `ms`, the time spent in the agent's own decisions and in the
    /// closeness and win-probability probes.
    pub agent_ms: f64,
    pub probe_ms: f64,
    pub replay_ok: bool,
    #[serde(skip)]
    pub actions: Vec<Action>,
    #[serde(skip)]
    pub final_state: Option<Box<GameState>>,
}

/// How a game ended, measured on the final table.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Margin {
    /// Adversary cards still in the deck: rounds to spare on a win.
    pub deck_left: u32,
    pub pool_left: i64,
    pub pool_start: i64,
    /// Population above the loss threshold.
    pub population_slack: i64,
    /// Adversary pieces keeping the current level's bar from being met
    /// (0 on a win): how far a loss was from winning.
    pub blockers: i64,
    pub level: i64,
    pub points: i64,
}

fn margin(rules: &Rules, g: &GameState) -> Margin {
    let w = rules.world();
    let count = |k: &str| (0..g.hexes.len()).map(|h| g.tokens(h, k)).sum::<i64>();
    let level = g.var(opposition::LEVEL);
    let max = w.int("opposition.max_level");
    let (d, o, f) = (count("drone"), count("outpost"), count("foundry"));
    let blockers = if g.outcome.as_ref().is_some_and(|o| o.won) || level >= max {
        0
    } else {
        match level {
            0 => d + o + f,
            1 => o + f,
            _ => f,
        }
    };
    let start = g.var(board::START_POPULATION);
    let safe = (start * w.int("opposition.loss_population_percent") + 99) / 100;
    Margin {
        deck_left: g.pile(adversary::DECK).len() as u32,
        pool_left: g.var(adversary::POOL),
        pool_start: adversary::start_pool(&w),
        population_slack: count("population") - safe,
        blockers,
        level,
        points: g.var(opposition::POINTS),
    }
}

fn build_signature(rules: &Rules, g: &GameState) -> String {
    let w = rules.world();
    (0..g.seats.len())
        .map(|s| {
            let ai = &aic_core::modules::ai::board(&w, s).id;
            let mut b: Vec<&str> = g
                .pile(&format!("cards.built.{s}"))
                .iter()
                .map(|x| x.as_str())
                .collect();
            b.sort_unstable();
            format!("{ai}:{}", b.join("+"))
        })
        .collect::<Vec<_>>()
        .join("|")
}

/// Plays `rollouts` games from a guess of `g` with `agent`; the share won.
fn win_probability(
    rules: &Rules,
    g: &GameState,
    agent: &AgentSpec,
    rollouts: usize,
    salt: u64,
    max_actions: usize,
) -> f64 {
    let mut rng = agent_rng(g.seed ^ salt, "winprob");
    let mut wins = 0;
    let mut legal = Vec::new();
    for k in 0..rollouts {
        let mut s = g.clone();
        determinize(rules, &mut s, &mut rng);
        let mut a = agent.build(
            rules,
            g.seed
                .wrapping_add(salt)
                .wrapping_add(k as u64 * 0x9e37_79b9),
        );
        let mut n = 0;
        while n < max_actions {
            rules.legal_actions_into(&s, &mut legal);
            if legal.is_empty() {
                break;
            }
            let i = a.choose(rules, &s, &legal);
            if rules.apply_mut(&mut s, &legal[i], None).is_err() {
                break;
            }
            n += 1;
        }
        wins += s.outcome.as_ref().is_some_and(|o| o.won) as usize;
    }
    wins as f64 / rollouts.max(1) as f64
}

/// Plays one game of one cell and measures it.
pub fn play_one(spec: &Spec, cells: &[Cell], ci: usize, seed: u64) -> GameResult {
    let cell = &cells[ci];
    let rules = &cell.rules;
    let start = Instant::now();
    let mut agent = cell.agent.build(rules, seed);
    let probe_eval = Eval::new(rules, Weights::default());
    let mut probe_rng = agent_rng(seed, "probe");
    let wp_agent = (spec.winprob.games > 0 && seed - spec.first_seed < spec.winprob.games)
        .then(|| AgentSpec::parse(&spec.winprob.agent).expect("checked in cells()"));
    let mut g = rules.new_game(seed);
    let mut legal = Vec::new();
    let mut r = GameResult {
        cell: ci,
        seed,
        won: false,
        reason: String::new(),
        rounds: 0,
        decisions: 0,
        options_total: 0,
        forced: 0,
        probed: 0,
        close: 0,
        gap_sum: 0.0,
        margin: Margin::default(),
        adversary_ops: Vec::new(),
        setup_ops: 0,
        counters: BTreeMap::new(),
        cards: BTreeMap::new(),
        gives: 0,
        pays: 0,
        withdraws: 0,
        build: String::new(),
        winprob: Vec::new(),
        ms: 0.0,
        agent_ms: 0.0,
        probe_ms: 0.0,
        replay_ok: false,
        actions: Vec::new(),
        final_state: None,
    };
    let (mut offered_round, mut checked_round, mut real_choices) = (0u32, 0u32, 0usize);
    let mut picks = vec![0u32; g.seats.len()];
    while r.actions.len() < spec.max_actions {
        rules.legal_actions_into(&g, &mut legal);
        if legal.is_empty() {
            break;
        }
        if let Some(a) = &wp_agent {
            if g.round != checked_round && g.round >= 1 {
                checked_round = g.round;
                let t = Instant::now();
                let p = win_probability(
                    rules,
                    &g,
                    a,
                    spec.winprob.rollouts,
                    0x5eed_0000 + g.round as u64,
                    spec.max_actions,
                );
                r.winprob.push(p);
                r.probe_ms += t.elapsed().as_secs_f64() * 1000.0;
            }
        }
        // Draft packets are dealt face down to each seat and passed: the
        // first draft decision of a round sees every card dealt.
        if g.round != offered_round && legal.iter().any(|a| matches!(a, Action::Draft { .. })) {
            offered_round = g.round;
            picks.iter_mut().for_each(|p| *p = 0);
            for (k, pile) in g.piles.range("cards.packet.".to_string()..) {
                if !k.starts_with("cards.packet.") {
                    break;
                }
                for c in pile {
                    r.cards.entry(c.clone()).or_default()[0] += 1;
                }
            }
        }
        let n = legal.len();
        r.decisions += 1;
        r.options_total += n as u64;
        if n == 1 {
            r.forced += 1;
        } else if spec.closeness.every > 0 {
            real_choices += 1;
            if real_choices % spec.closeness.every == 0 {
                let t = Instant::now();
                let mut s =
                    search::greedy_scores(rules, &probe_eval, &mut probe_rng, &g, &legal, 1, 1, 1);
                s.sort_by(|a, b| b.total_cmp(a));
                // An option the rules refused on a guess scores -inf: two of
                // them would make the gap NaN; count that as wide open.
                let gap = (s[0] - s[1]).clamp(0.0, 50.0);
                let gap = if gap.is_nan() { 50.0 } else { gap };
                r.probed += 1;
                r.gap_sum += gap;
                r.close += (gap < spec.closeness.gap) as u32;
                r.probe_ms += t.elapsed().as_secs_f64() * 1000.0;
            }
        }
        let t = Instant::now();
        let i = agent.choose(rules, &g, &legal);
        r.agent_ms += t.elapsed().as_secs_f64() * 1000.0;
        let a = legal[i].clone();
        match &a {
            Action::Draft { seat, card } => {
                let s = *seat as usize;
                picks[s] += 1;
                let e = r.cards.entry(card.clone()).or_default();
                e[1] += 1;
                e[4] += picks[s];
            }
            Action::Keep { card, .. } => r.cards.entry(card.clone()).or_default()[2] += 1,
            Action::Play { card, .. } => r.cards.entry(card.clone()).or_default()[3] += 1,
            Action::Give { .. } => r.gives += 1,
            Action::Pay { .. } => r.pays += 1,
            Action::Withdraw { .. } => r.withdraws += 1,
            _ => {}
        }
        // A draw's card is only known once drawn: whatever is new in the hand.
        let hand_before = match &a {
            Action::Draw { seat, .. } => {
                Some((*seat, g.pile(&format!("cards.hand.{seat}")).to_vec()))
            }
            _ => None,
        };
        rules
            .apply_mut(&mut g, &a, None)
            .expect("a legal action applies");
        if let Some((seat, mut before)) = hand_before {
            for c in g.pile(&format!("cards.hand.{seat}")) {
                match before.iter().position(|b| b == c) {
                    Some(k) => {
                        before.swap_remove(k);
                    }
                    None => r.cards.entry(c.clone()).or_default()[5] += 1,
                }
            }
        }
        r.actions.push(a);
    }
    r.ms = start.elapsed().as_secs_f64() * 1000.0;
    r.replay_ok = replay(rules, seed, &r.actions).is_ok_and(|h| h == g);
    r.won = g.outcome.as_ref().is_some_and(|o| o.won);
    r.reason = g
        .outcome
        .as_ref()
        .map_or("timeout".into(), |o| o.reason.clone());
    r.rounds = g.round;
    r.margin = margin(rules, &g);
    r.adversary_ops = g.stats.adversary_ops.clone();
    r.setup_ops = g.stats.setup_ops;
    r.counters = g.stats.counters.clone();
    r.build = build_signature(rules, &g);
    r.final_state = Some(Box::new(g));
    r
}

/// Plays every (cell, seed) on `threads` workers. Work is handed out one
/// game at a time (agents differ a hundredfold in cost); results come back
/// sorted by cell then seed, so nothing depends on the thread count.
pub fn run(
    spec: &Spec,
    cells: &[Cell],
    threads: usize,
    progress: &(dyn Fn(usize, usize) + Sync),
) -> Vec<GameResult> {
    let jobs: Vec<(usize, u64)> = (0..cells.len())
        .flat_map(|c| (0..spec.games).map(move |k| (c, spec.first_seed + k)))
        .collect();
    // Expensive cells first, so the last games to finish are cheap ones.
    let mut order: Vec<usize> = (0..jobs.len()).collect();
    order.sort_by_key(|&j| std::cmp::Reverse(cells[jobs[j].0].agent.tier()));
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let out = Mutex::new(Vec::with_capacity(jobs.len()));
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                let Some(&j) = order.get(k) else { break };
                let (c, seed) = jobs[j];
                let res = play_one(spec, cells, c, seed);
                out.lock().expect("results lock").push(res);
                progress(done.fetch_add(1, Ordering::Relaxed) + 1, jobs.len());
            });
        }
    });
    let mut v = out.into_inner().expect("results lock");
    v.sort_by_key(|r| (r.cell, r.seed));
    v
}
