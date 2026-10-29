//! Aggregates an experiment's games into the measures of
//! `docs/principles.md` and writes the cycle report: `report.md` (tables,
//! findings, the saved records), `results.json` (everything, machine
//! readable), `games.csv` and `cards.csv` (one row per game, per card), and
//! `records/` (game records worth a human look, openable in the viewer).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use aic_core::{content_hash, SavedGame};
use serde::Serialize;

use crate::agents::AgentSpec;
use crate::experiment::{Cell, GameResult, Spec};

#[derive(Debug, Clone, Serialize)]
pub struct CellSummary {
    pub cell: usize,
    pub config: String,
    pub config_name: String,
    pub config_hash: String,
    pub variant: String,
    pub difficulty: i64,
    pub agent: String,
    pub tier: usize,
    pub games: usize,
    pub wins: usize,
    pub win_rate: f64,
    pub win_rate_se: f64,
    pub rounds_mean: f64,
    pub rounds_sd: f64,
    /// Mean of [`survival`].
    pub survival_mean: f64,
    pub rounds_min: u32,
    pub rounds_max: u32,
    pub ended: BTreeMap<String, usize>,
    pub end_round: BTreeMap<u32, usize>,
    pub decisions_mean: f64,
    pub options_per_decision: f64,
    pub forced_share: f64,
    /// Of the decisions probed (real choices), the share that were close.
    pub close_share: f64,
    pub gap_mean: f64,
    pub adversary_ops_per_round: f64,
    pub adversary_ops_max_round: u32,
    pub setup_ops: f64,
    pub gives_per_game: f64,
    pub pays_per_game: f64,
    pub withdraws_per_game: f64,
    pub threats_dealt_per_game: f64,
    pub threats_countered_per_game: f64,
    pub countered_share: f64,
    /// Margin on wins: adversary cards left (rounds to spare), ambassador
    /// supply left.
    pub win_deck_left: Option<f64>,
    pub win_supply_left: Option<f64>,
    /// Distance on losses: adversary pieces over the bar, bar level.
    pub loss_blockers: Option<f64>,
    pub loss_level: Option<f64>,
    pub supply_left_mean: f64,
    /// Corrupted and aligned cities at the end, per game.
    pub cities_corrupted_mean: f64,
    pub cities_aligned_mean: f64,
    /// Mean win probability at the start of round r (index r-1), with the
    /// number of games that reached it.
    pub winprob_by_round: Vec<(f64, usize)>,
    /// Share of checkpoints with 0.1 < p < 0.9.
    pub uncertain_share: Option<f64>,
    /// Games whose outcome locked in (p stayed <= 0.05 or >= 0.95 to the end,
    /// matching the result) by round 2.
    pub early_lock_in_share: Option<f64>,
    pub distinct_builds: usize,
    pub distinct_builds_wins: usize,
    /// Mean Jaccard distance between two games' building sets (0 = always
    /// the same buildings, 1 = never a shared building).
    pub build_distance: f64,
    pub build_distance_wins: Option<f64>,
    /// Strategy health (see [`plays`]): cards, buildings (any tier, AI
    /// uniques too) and tier-2 cards played per game, buildings standing at
    /// the end per game, and the share of wins with 2 or more buildings
    /// played (standing at the end).
    pub cards_played_per_game: f64,
    pub buildings_played_per_game: f64,
    pub tier2_played_per_game: f64,
    pub buildings_standing_per_game: f64,
    pub wins_2plus_buildings_played: Option<f64>,
    pub wins_2plus_buildings_standing: Option<f64>,
    pub ms_per_game: f64,
    pub ms_per_decision: f64,
    /// The agent's own thinking per game and per decision, without probes.
    pub agent_ms_per_game: f64,
    pub agent_ms_per_decision: f64,
    pub probe_ms_per_game: f64,
    pub replay_ok: usize,
}

/// One config × variant × difficulty, across agents.
#[derive(Debug, Clone, Serialize)]
pub struct GroupSummary {
    pub config: String,
    pub variant: String,
    pub difficulty: i64,
    /// Agent label → win rate, in ladder order.
    pub win_rates: Vec<(String, f64)>,
    /// Agent label → mean survival (see [`survival`]).
    pub survival: Vec<(String, f64)>,
    /// The top tier's win rate minus the bottom tier's (each tier by its best agent).
    pub skill_gap: f64,
    pub skill_gap_survival: f64,
    /// Win rate never falls from one tier to the next (best agent per tier).
    pub ordered: bool,
    pub ordered_survival: bool,
    /// Two-way decomposition of outcome variance over seeds × agents:
    /// shares of the seed (luck), the agent (skill) and the rest.
    pub luck_win: Option<[f64; 3]>,
    pub luck_survival: Option<[f64; 3]>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SavedRecord {
    pub category: String,
    pub path: String,
    pub why: String,
}

/// How long a team held out: the round a loss came in; for a win, its
/// round plus the adversary cards left plus 2, so any win ranks above any
/// loss (a deck-out loss comes about one round after the last card). The
/// ladder's signal while few games are won.
pub fn survival(g: &GameResult) -> f64 {
    if g.won {
        (g.rounds + g.margin.deck_left + 2) as f64
    } else {
        g.rounds as f64
    }
}

fn mean(v: impl Iterator<Item = f64>) -> Option<f64> {
    let (mut s, mut n) = (0.0, 0usize);
    for x in v {
        s += x;
        n += 1;
    }
    (n > 0).then(|| s / n as f64)
}

fn buildings(sig: &str) -> BTreeSet<String> {
    sig.split('|')
        .filter_map(|seat| seat.split_once(':'))
        .flat_map(|(ai, b)| {
            b.split('+')
                .filter(|x| !x.is_empty())
                .map(move |x| format!("{ai}:{x}"))
        })
        .collect()
}

/// Buildings standing at the end, over every seat (copies counted).
fn standing(sig: &str) -> usize {
    sig.split('|')
        .filter_map(|seat| seat.split_once(':'))
        .map(|(_, b)| b.split('+').filter(|x| !x.is_empty()).count())
        .sum()
}

/// One game's plays: [cards, buildings (any tier, AI uniques too), tier-2
/// cards], from the per-card counts and the content's tier and kind.
pub fn plays(g: &GameResult) -> [u32; 3] {
    let content = aic_core::content();
    let mut out = [0u32; 3];
    for (id, n) in &g.cards {
        if n[3] == 0 {
            continue;
        }
        out[0] += n[3];
        if let Some(d) = content.cards.iter().find(|d| d.id == *id) {
            if d.is_building() {
                out[1] += n[3];
            }
            if d.tier == 2 {
                out[2] += n[3];
            }
        }
    }
    out
}

/// Two cells compared on the same seeds: `a` against `b`.
#[derive(Debug, Clone, Serialize)]
pub struct Paired {
    pub games: usize,
    pub a_win_rate: f64,
    pub b_win_rate: f64,
    pub both: usize,
    pub only_a: usize,
    pub only_b: usize,
    pub neither: usize,
    /// `a`'s win rate minus `b`'s, with the standard error of the mean
    /// per-seed difference.
    pub diff: f64,
    pub diff_se: f64,
    /// Exact two-sided sign test on the discordant seeds (won by one only).
    pub sign_p: f64,
    pub a_rounds: f64,
    pub b_rounds: f64,
}

/// Exact two-sided binomial (p = 1/2) test of `x` against `y` successes.
fn sign_test(x: usize, y: usize) -> f64 {
    let n = x + y;
    if n == 0 {
        return 1.0;
    }
    let (mut ln_c, mut tail) = (0.0f64, 0.0f64);
    for i in 0..=x.min(y) {
        if i > 0 {
            ln_c += ((n - i + 1) as f64).ln() - (i as f64).ln();
        }
        tail += (ln_c - n as f64 * std::f64::consts::LN_2).exp();
    }
    (2.0 * tail).min(1.0)
}

fn paired(games: &[GameResult], a: usize, b: usize) -> Paired {
    let bs: BTreeMap<u64, &GameResult> = games
        .iter()
        .filter(|g| g.cell == b)
        .map(|g| (g.seed, g))
        .collect();
    let (mut n, mut both, mut oa, mut ob, mut neither) = (0usize, 0, 0, 0, 0);
    let (mut ra, mut rb) = (0.0, 0.0);
    for ga in games.iter().filter(|g| g.cell == a) {
        let Some(gb) = bs.get(&ga.seed) else { continue };
        n += 1;
        ra += ga.rounds as f64;
        rb += gb.rounds as f64;
        match (ga.won, gb.won) {
            (true, true) => both += 1,
            (true, false) => oa += 1,
            (false, true) => ob += 1,
            (false, false) => neither += 1,
        }
    }
    let nf = n.max(1) as f64;
    let diff = (oa as f64 - ob as f64) / nf;
    let se = if n > 1 {
        let var = ((oa + ob) as f64 - nf * diff * diff) / (nf - 1.0);
        (var.max(0.0) / nf).sqrt()
    } else {
        0.0
    };
    Paired {
        games: n,
        a_win_rate: (both + oa) as f64 / nf,
        b_win_rate: (both + ob) as f64 / nf,
        both,
        only_a: oa,
        only_b: ob,
        neither,
        diff,
        diff_se: se,
        sign_p: sign_test(oa, ob),
        a_rounds: ra / nf,
        b_rounds: rb / nf,
    }
}

/// The agents in spec order, by label (first cell of each).
fn agent_order(cells: &[Cell]) -> Vec<&AgentSpec> {
    let mut out: Vec<&AgentSpec> = Vec::new();
    for c in cells {
        if !out.iter().any(|a| a.label() == c.agent.label()) {
            out.push(&c.agent);
        }
    }
    out
}

/// The greedy agent the others are compared with: the one labelled exactly
/// `greedy`, else the first of kind greedy.
fn greedy_reference(cells: &[Cell]) -> Option<String> {
    let agents = agent_order(cells);
    agents
        .iter()
        .find(|a| a.label() == "greedy")
        .or_else(|| agents.iter().find(|a| a.kind == "greedy"))
        .map(|a| a.label())
}

fn find_cell(cells: &[Cell], config: &str, variant: &str, d: i64, agent: &str) -> Option<usize> {
    cells.iter().position(|c| {
        c.config_path == config
            && c.variant == variant
            && c.difficulty == d
            && c.agent.label() == agent
    })
}

/// The top agent: highest win rate at the highest difficulty on the base
/// variant (mean over configs), ties to the later agent in the list.
fn top_agent(cells: &[Cell], sums: &[CellSummary]) -> Option<(String, i64)> {
    let hi = cells.iter().map(|c| c.difficulty).max()?;
    // On the base variant (the first), as the strategy-health line reads it.
    let base = &cells.first()?.variant;
    let mut best: Option<(String, f64)> = None;
    for a in agent_order(cells) {
        let l = a.label();
        let Some(p) = mean(
            sums.iter()
                .filter(|s| s.difficulty == hi && s.agent == l && s.variant == *base)
                .map(|s| s.win_rate),
        ) else {
            continue;
        };
        match &best {
            Some((_, b)) if p < *b => {}
            _ => best = Some((l, p)),
        }
    }
    best.map(|(l, _)| (l, hi))
}

/// Each agent's rank by measured strength, 0 = weakest: its win rate at
/// the highest difficulty on the base variant (mean over configs), as
/// [`top_agent`] reads it. Equal win rates share a rank, so the top rank
/// holds the top agent (and any agent tied with it), and a "weakest"
/// agent exists only when some agent measured below the rest.
fn measured_rank(cells: &[Cell], games: &[GameResult]) -> BTreeMap<String, usize> {
    let hi = cells.iter().map(|c| c.difficulty).max().unwrap_or(0);
    let base = cells.first().map(|c| c.variant.clone()).unwrap_or_default();
    let score: Vec<(String, f64)> = agent_order(cells)
        .iter()
        .map(|a| {
            let l = a.label();
            let mut by_config: BTreeMap<&str, (u64, u64)> = BTreeMap::new();
            for g in games {
                let c = &cells[g.cell];
                if c.difficulty == hi && c.variant == base && c.agent.label() == l {
                    let e = by_config.entry(c.config_path.as_str()).or_default();
                    e.0 += g.won as u64;
                    e.1 += 1;
                }
            }
            let rates = by_config.values().map(|&(w, n)| w as f64 / n as f64);
            (l, mean(rates).unwrap_or(0.0))
        })
        .collect();
    let mut levels: Vec<f64> = score.iter().map(|s| s.1).collect();
    levels.sort_by(f64::total_cmp);
    levels.dedup();
    score
        .into_iter()
        .map(|(l, p)| (l, levels.iter().position(|&x| x == p).unwrap_or(0)))
        .collect()
}

#[derive(Debug, Clone, Serialize)]
pub struct HeadToHead {
    pub config: String,
    pub variant: String,
    pub difficulty: i64,
    pub agent: String,
    pub greedy: String,
    /// `a` = the agent, `b` = greedy.
    pub paired: Paired,
    pub verdict: String,
}

fn verdict(p: &Paired) -> String {
    if p.sign_p < 0.05 && p.diff > 0.0 {
        "beats greedy".into()
    } else if p.sign_p < 0.05 && p.diff < 0.0 {
        "worse than greedy".into()
    } else if p.diff > 0.0 {
        "no better than greedy (ahead, not significant)".into()
    } else if p.diff < 0.0 {
        "no better than greedy (behind, not significant)".into()
    } else {
        "no better than greedy (level)".into()
    }
}

/// Every other agent against greedy on the same seeds, per cell.
pub fn head_to_head(cells: &[Cell], games: &[GameResult]) -> Vec<HeadToHead> {
    let Some(gl) = greedy_reference(cells) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (i, c) in cells.iter().enumerate() {
        let l = c.agent.label();
        if l == gl {
            continue;
        }
        let Some(j) = find_cell(cells, &c.config_path, &c.variant, c.difficulty, &gl) else {
            continue;
        };
        let p = paired(games, i, j);
        out.push(HeadToHead {
            config: c.config_path.clone(),
            variant: c.variant.clone(),
            difficulty: c.difficulty,
            agent: l,
            greedy: gl.clone(),
            verdict: verdict(&p),
            paired: p,
        });
    }
    out
}

/// Modules a variant switches off: set entries `<module>.variant=off` or
/// anything else ending `=off`.
fn switched_off(set: &[String]) -> Vec<String> {
    // Only a pure removal: a variant that also changes other numbers is not
    // a measure of what the module is worth.
    if set.is_empty() || !set.iter().all(|s| s.ends_with("=off")) {
        return Vec::new();
    }
    set.iter()
        .filter_map(|s| s.strip_suffix("=off"))
        .map(|k| k.strip_suffix(".variant").unwrap_or(k).to_string())
        .collect()
}

#[derive(Debug, Clone, Serialize)]
pub struct VariantDiff {
    pub config: String,
    pub base: String,
    pub variant: String,
    /// Modules the variant switches off (empty if none).
    pub off: Vec<String>,
    pub difficulty: i64,
    pub agent: String,
    /// `a` = the variant, `b` = the base. For a variant that switches a
    /// module off, `-diff` is what the module is worth.
    pub paired: Paired,
}

/// Every variant after the first against the first (the base), per config,
/// difficulty and agent, on the same seeds.
pub fn variant_diffs(spec: &Spec, cells: &[Cell], games: &[GameResult]) -> Vec<VariantDiff> {
    let Some(base) = spec.variants.first() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for v in spec.variants.iter().skip(1) {
        for (i, c) in cells.iter().enumerate().filter(|(_, c)| c.variant == v.name) {
            let l = c.agent.label();
            let Some(j) = find_cell(cells, &c.config_path, &base.name, c.difficulty, &l) else {
                continue;
            };
            out.push(VariantDiff {
                config: c.config_path.clone(),
                base: base.name.clone(),
                variant: v.name.clone(),
                off: switched_off(&v.set),
                difficulty: c.difficulty,
                agent: l,
                paired: paired(games, i, j),
            });
        }
    }
    out
}

/// `+4.2 ±3.1` (percentage points).
fn pts(x: f64, se: f64) -> String {
    // `+ 0.0` turns a negated zero into a plain one.
    format!("{:+.1} ±{:.1}", 100.0 * x + 0.0, 100.0 * se)
}

fn p_value(p: f64) -> String {
    if p < 0.001 {
        "<0.001".into()
    } else {
        format!("{p:.3}")
    }
}

fn jaccard_mean(sigs: &[&str]) -> Option<f64> {
    if sigs.len() < 2 {
        return None;
    }
    let sets: Vec<BTreeSet<String>> = sigs.iter().map(|s| buildings(s)).collect();
    let (mut s, mut n) = (0.0, 0usize);
    // At most 200 games, so this stays quadratic in something small.
    let k = sets.len().min(200);
    for i in 0..k {
        for j in i + 1..k {
            let u = sets[i].union(&sets[j]).count();
            let d = if u == 0 {
                0.0
            } else {
                1.0 - sets[i].intersection(&sets[j]).count() as f64 / u as f64
            };
            s += d;
            n += 1;
        }
    }
    Some(s / n as f64)
}

/// The round from which p stayed decided (<= 0.05 on a loss, >= 0.95 on a
/// win) to the end of the game, if it did.
fn lock_in_round(r: &GameResult) -> Option<usize> {
    if r.winprob.is_empty() {
        return None;
    }
    let decided = |p: f64| if r.won { p >= 0.95 } else { p <= 0.05 };
    let mut k = r.winprob.len();
    while k > 0 && decided(r.winprob[k - 1]) {
        k -= 1;
    }
    (k < r.winprob.len()).then_some(k + 1)
}

pub fn summarize(cells: &[Cell], games: &[GameResult]) -> Vec<CellSummary> {
    cells
        .iter()
        .enumerate()
        .map(|(ci, c)| {
            let gs: Vec<&GameResult> = games.iter().filter(|g| g.cell == ci).collect();
            let n = gs.len().max(1) as f64;
            let wins = gs.iter().filter(|g| g.won).count();
            let p = wins as f64 / n;
            let rounds: Vec<f64> = gs.iter().map(|g| g.rounds as f64).collect();
            let rm = mean(rounds.iter().copied()).unwrap_or(0.0);
            let rsd = (rounds.iter().map(|x| (x - rm).powi(2)).sum::<f64>() / n).sqrt();
            let mut ended = BTreeMap::new();
            let mut end_round = BTreeMap::new();
            for g in &gs {
                *ended.entry(g.reason.clone()).or_insert(0) += 1;
                *end_round.entry(g.rounds).or_insert(0) += 1;
            }
            let decisions: u64 = gs.iter().map(|g| g.decisions as u64).sum();
            let options: u64 = gs.iter().map(|g| g.options_total).sum();
            let forced: u64 = gs.iter().map(|g| g.forced as u64).sum();
            let probed: u64 = gs.iter().map(|g| g.probed as u64).sum();
            let close: u64 = gs.iter().map(|g| g.close as u64).sum();
            let gap: f64 = gs.iter().map(|g| g.gap_sum).sum();
            let ops: u64 = gs
                .iter()
                .flat_map(|g| g.adversary_ops.iter())
                .map(|&x| x as u64)
                .sum();
            let op_rounds: u64 = gs.iter().map(|g| g.adversary_ops.len() as u64).sum();
            let counter = |k: &str| {
                gs.iter()
                    .map(|g| *g.counters.get(k).unwrap_or(&0) as f64)
                    .sum::<f64>()
                    / n
            };
            let dealt = counter("threats.dealt");
            let countered = counter("threats.countered");
            let won: Vec<&&GameResult> = gs.iter().filter(|g| g.won).collect();
            let lost: Vec<&&GameResult> = gs.iter().filter(|g| !g.won).collect();
            let max_round = gs.iter().map(|g| g.winprob.len()).max().unwrap_or(0);
            let winprob_by_round = (0..max_round)
                .map(|k| {
                    let v: Vec<f64> = gs
                        .iter()
                        .filter_map(|g| g.winprob.get(k).copied())
                        .collect();
                    (mean(v.iter().copied()).unwrap_or(0.0), v.len())
                })
                .collect();
            let checkpoints: Vec<f64> = gs.iter().flat_map(|g| g.winprob.iter().copied()).collect();
            let with_wp: Vec<&&GameResult> = gs.iter().filter(|g| !g.winprob.is_empty()).collect();
            let sigs: Vec<&str> = gs.iter().map(|g| g.build.as_str()).collect();
            let win_sigs: Vec<&str> = won.iter().map(|g| g.build.as_str()).collect();
            let ms: f64 = gs.iter().map(|g| g.ms).sum();
            CellSummary {
                cell: ci,
                config: c.config_path.clone(),
                config_name: c.rules.config.name.clone(),
                config_hash: c.rules.config_hash.clone(),
                variant: c.variant.clone(),
                difficulty: c.difficulty,
                agent: c.agent.label(),
                tier: c.agent.tier(),
                games: gs.len(),
                wins,
                win_rate: p,
                win_rate_se: (p * (1.0 - p) / n).sqrt(),
                rounds_mean: rm,
                survival_mean: mean(gs.iter().map(|g| survival(g))).unwrap_or(0.0),
                rounds_sd: rsd,
                rounds_min: gs.iter().map(|g| g.rounds).min().unwrap_or(0),
                rounds_max: gs.iter().map(|g| g.rounds).max().unwrap_or(0),
                ended,
                end_round,
                decisions_mean: decisions as f64 / n,
                options_per_decision: options as f64 / decisions.max(1) as f64,
                forced_share: forced as f64 / decisions.max(1) as f64,
                close_share: close as f64 / probed.max(1) as f64,
                gap_mean: gap / probed.max(1) as f64,
                adversary_ops_per_round: ops as f64 / op_rounds.max(1) as f64,
                adversary_ops_max_round: gs
                    .iter()
                    .flat_map(|g| g.adversary_ops.iter().copied())
                    .max()
                    .unwrap_or(0),
                setup_ops: gs.iter().map(|g| g.setup_ops as f64).sum::<f64>() / n,
                gives_per_game: gs.iter().map(|g| g.gives as f64).sum::<f64>() / n,
                pays_per_game: gs.iter().map(|g| g.pays as f64).sum::<f64>() / n,
                withdraws_per_game: gs.iter().map(|g| g.withdraws as f64).sum::<f64>() / n,
                threats_dealt_per_game: dealt,
                threats_countered_per_game: countered,
                countered_share: if dealt > 0.0 { countered / dealt } else { 0.0 },
                win_deck_left: mean(won.iter().map(|g| g.margin.deck_left as f64)),
                win_supply_left: mean(won.iter().map(|g| g.margin.supply_left as f64)),
                loss_blockers: mean(lost.iter().map(|g| g.margin.blockers as f64)),
                loss_level: mean(lost.iter().map(|g| g.margin.level as f64)),
                supply_left_mean: mean(gs.iter().map(|g| g.margin.supply_left as f64)).unwrap_or(0.0),
                cities_corrupted_mean: mean(gs.iter().map(|g| g.margin.cities_corrupted_end as f64))
                    .unwrap_or(0.0),
                cities_aligned_mean: mean(gs.iter().map(|g| g.margin.cities_aligned_end as f64))
                    .unwrap_or(0.0),
                winprob_by_round,
                uncertain_share: mean(
                    checkpoints
                        .iter()
                        .map(|&p| (p > 0.1 && p < 0.9) as u8 as f64),
                ),
                // Of games long enough to be decided later (3+ rounds).
                early_lock_in_share: mean(
                    with_wp
                        .iter()
                        .filter(|g| g.rounds >= 3)
                        .map(|g| lock_in_round(g).is_some_and(|r| r <= 2) as u8 as f64),
                ),
                distinct_builds: sigs.iter().collect::<BTreeSet<_>>().len(),
                distinct_builds_wins: win_sigs.iter().collect::<BTreeSet<_>>().len(),
                build_distance: jaccard_mean(&sigs).unwrap_or(0.0),
                build_distance_wins: jaccard_mean(&win_sigs),
                cards_played_per_game: gs.iter().map(|g| plays(g)[0] as f64).sum::<f64>() / n,
                buildings_played_per_game: gs.iter().map(|g| plays(g)[1] as f64).sum::<f64>()
                    / n,
                tier2_played_per_game: gs.iter().map(|g| plays(g)[2] as f64).sum::<f64>() / n,
                buildings_standing_per_game: gs
                    .iter()
                    .map(|g| standing(&g.build) as f64)
                    .sum::<f64>()
                    / n,
                wins_2plus_buildings_played: mean(
                    won.iter().map(|g| (plays(g)[1] >= 2) as u8 as f64),
                ),
                wins_2plus_buildings_standing: mean(
                    won.iter().map(|g| (standing(&g.build) >= 2) as u8 as f64),
                ),
                ms_per_game: ms / n,
                ms_per_decision: ms / decisions.max(1) as f64,
                agent_ms_per_game: gs.iter().map(|g| g.agent_ms).sum::<f64>() / n,
                agent_ms_per_decision: gs.iter().map(|g| g.agent_ms).sum::<f64>()
                    / decisions.max(1) as f64,
                probe_ms_per_game: gs.iter().map(|g| g.probe_ms).sum::<f64>() / n,
                replay_ok: gs.iter().filter(|g| g.replay_ok).count(),
            }
        })
        .collect()
}

/// Shares of the variance of `y[agent][seed]` due to the seed, the agent
/// and the rest (two-way, no replication). `None` if nothing varies.
fn decompose(y: &[Vec<f64>]) -> Option<[f64; 3]> {
    let a = y.len();
    let s = y.first()?.len();
    if a < 2 || s < 2 || y.iter().any(|r| r.len() != s) {
        return None;
    }
    let m = y.iter().flatten().sum::<f64>() / (a * s) as f64;
    let total: f64 = y.iter().flatten().map(|v| (v - m).powi(2)).sum();
    if total <= 1e-12 {
        return None;
    }
    let agent: f64 = y
        .iter()
        .map(|r| (r.iter().sum::<f64>() / s as f64 - m).powi(2))
        .sum::<f64>()
        * s as f64;
    let seed: f64 = (0..s)
        .map(|k| (y.iter().map(|r| r[k]).sum::<f64>() / a as f64 - m).powi(2))
        .sum::<f64>()
        * a as f64;
    Some([
        seed / total,
        agent / total,
        ((total - seed - agent) / total).max(0.0),
    ])
}

pub fn groups(cells: &[Cell], sums: &[CellSummary], games: &[GameResult]) -> Vec<GroupSummary> {
    let mut keys: Vec<(String, String, i64)> = Vec::new();
    for c in cells {
        if !keys.contains(&c.group()) {
            keys.push(c.group());
        }
    }
    keys.into_iter()
        .map(|key| {
            let mut idx: Vec<usize> = (0..cells.len())
                .filter(|&i| cells[i].group() == key)
                .collect();
            idx.sort_by_key(|&i| (cells[i].agent.tier(), i));
            let win_rates: Vec<(String, f64)> = idx
                .iter()
                .map(|&i| (sums[i].agent.clone(), sums[i].win_rate))
                .collect();
            let surv: Vec<(String, f64)> = idx
                .iter()
                .map(|&i| (sums[i].agent.clone(), sums[i].survival_mean))
                .collect();
            // The ladder is read per tier: several agents of one kind (a
            // budget sweep) are siblings on a rung, so each tier counts with
            // its best agent.
            let tiers: Vec<usize> = idx.iter().map(|&i| cells[i].agent.tier()).collect();
            let best_per_tier = |v: &[(String, f64)]| -> Vec<f64> {
                let mut out: Vec<(usize, f64)> = Vec::new();
                for (k, (_, x)) in v.iter().enumerate() {
                    match out.last_mut() {
                        Some((t, b)) if *t == tiers[k] => *b = b.max(*x),
                        _ => out.push((tiers[k], *x)),
                    }
                }
                out.into_iter().map(|(_, b)| b).collect()
            };
            let gap = |v: &[(String, f64)]| {
                let b = best_per_tier(v);
                b.last().unwrap_or(&0.0) - b.first().unwrap_or(&0.0)
            };
            let ordered =
                |v: &[(String, f64)]| best_per_tier(v).windows(2).all(|w| w[1] >= w[0] - 1e-9);
            // Rows are seeds: only those every agent of the group has
            // played (all of them, unless the report is partial).
            let common = idx
                .iter()
                .map(|&i| {
                    games
                        .iter()
                        .filter(|g| g.cell == i)
                        .map(|g| g.seed)
                        .collect::<BTreeSet<u64>>()
                })
                .reduce(|a, b| &a & &b)
                .unwrap_or_default();
            let table = |f: &dyn Fn(&GameResult) -> f64| -> Vec<Vec<f64>> {
                idx.iter()
                    .map(|&i| {
                        games
                            .iter()
                            .filter(|g| g.cell == i && common.contains(&g.seed))
                            .map(f)
                            .collect()
                    })
                    .collect()
            };
            GroupSummary {
                config: key.0,
                variant: key.1,
                difficulty: key.2,
                skill_gap: gap(&win_rates),
                skill_gap_survival: gap(&surv),
                ordered: ordered(&win_rates),
                ordered_survival: ordered(&surv),
                win_rates,
                survival: surv,
                luck_win: decompose(&table(&|g| g.won as u8 as f64)),
                luck_survival: decompose(&table(&survival)),
            }
        })
        .collect()
}

/// Per agent: card → [offered, drafted, kept, played], summed over cells.
pub fn card_table(
    cells: &[Cell],
    games: &[GameResult],
) -> BTreeMap<String, BTreeMap<String, [u32; 6]>> {
    let mut out: BTreeMap<String, BTreeMap<String, [u32; 6]>> = BTreeMap::new();
    for g in games {
        let t = out.entry(cells[g.cell].agent.label()).or_default();
        for (card, n) in &g.cards {
            let e = t.entry(card.clone()).or_default();
            for k in 0..6 {
                e[k] += n[k];
            }
        }
    }
    out
}

fn pct(x: f64) -> String {
    format!("{:.0}%", 100.0 * x)
}

fn file_safe(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Picks and writes the records worth a look. Returns what was saved.
pub fn save_records(
    dir: &str,
    spec: &Spec,
    cells: &[Cell],
    games: &[GameResult],
) -> Result<Vec<SavedRecord>, String> {
    let rec_dir = format!("{dir}/records");
    std::fs::create_dir_all(&rec_dir).map_err(|e| format!("{rec_dir}: {e}"))?;
    let top_d = cells.iter().map(|c| c.difficulty).max().unwrap_or(0);
    // Agents ranked by measured strength, not by ladder tier (011): the
    // ladder's top tier is not always the strongest agent.
    let strength = measured_rank(cells, games);
    let rank = |c: &Cell| strength.get(&c.agent.label()).copied().unwrap_or(0);
    let top_tier = strength.values().copied().max().unwrap_or(0);
    let low_tier = 0;
    let mut picked: Vec<(String, usize, String)> = Vec::new();
    let mut taken = BTreeSet::new();
    let mut take =
        |cat: &str, list: Vec<(usize, String)>, picked: &mut Vec<(String, usize, String)>| {
            let mut n = 0;
            for (i, why) in list {
                if n == spec.records {
                    break;
                }
                if taken.insert(i) {
                    picked.push((cat.to_string(), i, why));
                    n += 1;
                }
            }
        };
    // Close losses at the top difficulty: fewest pieces over the bar, then
    // the longest games, measured-stronger agents first.
    let mut close: Vec<(usize, &GameResult)> = games
        .iter()
        .enumerate()
        .filter(|(_, g)| !g.won && cells[g.cell].difficulty == top_d && g.reason != "timeout")
        .collect();
    close.sort_by_key(|(_, g)| {
        (
            g.margin.blockers,
            std::cmp::Reverse(g.rounds),
            std::cmp::Reverse(rank(&cells[g.cell])),
            g.seed,
        )
    });
    take(
        "close-loss",
        close
            .iter()
            .map(|(i, g)| {
                (*i, format!(
                    "close loss at difficulty {top_d}: {} in round {}, {} adversary piece(s) over the level-{} bar, supply {} of {}, {} cities corrupted",
                    g.reason,
                    g.rounds,
                    g.margin.blockers,
                    g.margin.level,
                    g.margin.supply_left,
                    g.margin.supply_start,
                    g.margin.cities_corrupted_end
                ))
            })
            .collect(),
        &mut picked,
    );
    // Early lock-ins: decided by round 2 in the longest games.
    let mut lock: Vec<(usize, &GameResult, usize)> = games
        .iter()
        .enumerate()
        .filter_map(|(i, g)| Some((i, g, lock_in_round(g)?)))
        .filter(|(_, g, r)| *r <= 2 && g.rounds >= 3)
        .collect();
    lock.sort_by_key(|(_, g, r)| {
        (
            *r,
            std::cmp::Reverse(g.rounds),
            std::cmp::Reverse(rank(&cells[g.cell])),
            g.seed,
        )
    });
    take(
        "early-lock-in",
        lock.iter()
            .map(|(i, g, r)| {
                let ps: Vec<String> = g.winprob.iter().map(|p| format!("{p:.2}")).collect();
                (
                    *i,
                    format!(
                        "outcome ({}) decided from round {r} of {}: win probability by round {}",
                        if g.won { "win" } else { "loss" },
                        g.rounds,
                        ps.join(" ")
                    ),
                )
            })
            .collect(),
        &mut picked,
    );
    // Surprising wins: by the weakest agent, by a weaker agent at the top
    // difficulty, or after the playouts gave them little chance.
    let mut surprise: Vec<(usize, String, (u8, u64))> = Vec::new();
    for (i, g) in games.iter().enumerate().filter(|(_, g)| g.won) {
        let c = &cells[g.cell];
        let low = g.winprob.iter().copied().fold(f64::INFINITY, f64::min);
        let why = if rank(c) == low_tier && low_tier < top_tier {
            Some((0, format!("won by the weakest agent as measured ({})", c.agent.label())))
        } else if low <= 0.25 {
            Some((1, format!("won after the playouts gave it {low:.2}")))
        } else if c.difficulty == top_d && rank(c) < top_tier {
            Some((
                2,
                format!(
                    "won at difficulty {top_d} by {}, below the top agent",
                    c.agent.label()
                ),
            ))
        } else {
            None
        };
        if let Some((rank, w)) = why {
            surprise.push((i, w, (rank, g.seed)));
        }
    }
    surprise.sort_by_key(|s| s.2);
    take(
        "surprising-win",
        surprise.into_iter().map(|(i, w, _)| (i, w)).collect(),
        &mut picked,
    );

    let mut out = Vec::new();
    for (cat, i, why) in picked {
        let g = &games[i];
        let c = &cells[g.cell];
        let Some(last) = g.final_state.as_deref() else {
            continue;
        };
        let note = format!(
            "aic-sim experiment {}: {cat}: {}, variant {} ({}), difficulty {}, agent {}, seed {}: {why}",
            spec.name,
            c.config_path,
            c.variant,
            c.overrides.join(" "),
            c.difficulty,
            c.agent.label(),
            g.seed
        );
        let rec = SavedGame::new(&c.rules, g.seed, g.actions.clone(), last, &note);
        // The cell number keeps two agents of one kind (or two configs of
        // one name) apart; the label says which agent it was.
        let label: String = file_safe(&c.agent.label()).chars().take(48).collect();
        let file = format!(
            "{rec_dir}/{cat}-c{}-{}-{}-d{}-{label}-seed{}.json",
            g.cell,
            file_safe(&c.rules.config.name),
            file_safe(&c.variant),
            c.difficulty,
            g.seed
        );
        write_whole(&file, &rec.to_text())?;
        out.push(SavedRecord {
            category: cat,
            path: file,
            why,
        });
    }
    Ok(out)
}

/// Removes the records an earlier report of this experiment saved that the
/// latest one (`keep`) did not, so none sit there unlisted. Called once the
/// latest report is in place.
pub fn prune_records(dir: &str, keep: &[SavedRecord]) -> Result<(), String> {
    let rec_dir = format!("{dir}/records");
    let keep: BTreeSet<std::path::PathBuf> = keep.iter().map(|r| r.path.clone().into()).collect();
    for e in std::fs::read_dir(&rec_dir)
        .map_err(|e| format!("{rec_dir}: {e}"))?
        .flatten()
    {
        let name = e.file_name().to_string_lossy().to_string();
        let ours = ["close-loss-", "early-lock-in-", "surprising-win-"]
            .iter()
            .any(|p| name.starts_with(p));
        let path = std::path::PathBuf::from(format!("{rec_dir}/{name}"));
        if ours && name.ends_with(".json") && !keep.contains(&path) {
            std::fs::remove_file(e.path()).map_err(|err| format!("{name}: {err}"))?;
        }
    }
    Ok(())
}

/// Writes `body` to `path` whole: into `path.tmp`, then renamed over
/// `path`, so a run killed mid-write never leaves half a file.
pub fn write_whole(path: &str, body: &str) -> Result<(), String> {
    let tmp = format!("{path}.tmp");
    std::fs::write(&tmp, body)
        .and_then(|_| std::fs::rename(&tmp, path))
        .map_err(|e| {
            let _ = std::fs::remove_file(&tmp);
            format!("cannot write {path}: {e}")
        })
}

/// A CSV text field, quoted (with `"` doubled).
fn q(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

pub fn games_csv(cells: &[Cell], games: &[GameResult]) -> String {
    let mut s = String::from(
        "config,variant,difficulty,agent,seed,won,reason,rounds,decisions,options,forced,probed,close,gap_mean,deck_left,supply_left,supply_start,cities_corrupted_end,cities_aligned_end,blockers,level,points,adversary_ops,adversary_ops_max,setup_ops,gives,pays,withdraws,threats_dealt,threats_countered,corruptions,first_corruption_round,std_actions,war_destroyed,cards_played,buildings_played,tier2_played,buildings_standing,build,winprob,ms,replay_ok\n",
    );
    for g in games {
        let c = &cells[g.cell];
        let ops: u32 = g.adversary_ops.iter().sum();
        let wp: Vec<String> = g.winprob.iter().map(|p| format!("{p:.3}")).collect();
        let cnt = |k: &str| g.counters.get(k).copied().unwrap_or(0);
        let pl = plays(g);
        let _ = writeln!(
            s,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.1},{}",
            q(&c.config_path),
            q(&c.variant),
            c.difficulty,
            q(&c.agent.label()),
            g.seed,
            g.won as u8,
            q(&g.reason),
            g.rounds,
            g.decisions,
            g.options_total,
            g.forced,
            g.probed,
            g.close,
            g.gap_sum / g.probed.max(1) as f64,
            g.margin.deck_left,
            g.margin.supply_left,
            g.margin.supply_start,
            g.margin.cities_corrupted_end,
            g.margin.cities_aligned_end,
            g.margin.blockers,
            g.margin.level,
            g.margin.points,
            ops,
            g.adversary_ops.iter().max().unwrap_or(&0),
            g.setup_ops,
            g.gives,
            g.pays,
            g.withdraws,
            cnt("threats.dealt"),
            cnt("threats.countered"),
            cnt("adversary.corruptions"),
            cnt("adversary.first_corruption_round"),
            cnt("board.std_actions"),
            cnt("board.war_destroyed"),
            pl[0],
            pl[1],
            pl[2],
            standing(&g.build),
            q(&g.build),
            wp.join(";"),
            g.ms,
            g.replay_ok as u8
        );
    }
    s
}

pub fn cards_csv(cells: &[Cell], games: &[GameResult]) -> String {
    let mut per: BTreeMap<(String, i64, String), [u32; 6]> = BTreeMap::new();
    for g in games {
        let c = &cells[g.cell];
        for (card, n) in &g.cards {
            let e = per
                .entry((c.agent.label(), c.difficulty, card.clone()))
                .or_default();
            for k in 0..6 {
                e[k] += n[k];
            }
        }
    }
    let mut s =
        String::from("agent,difficulty,card,offered,drafted,kept,played,pick_position_sum,drawn\n");
    for ((a, d, card), n) in per {
        let _ = writeln!(
            s,
            "{},{d},{card},{},{},{},{},{},{}",
            q(&a),
            n[0],
            n[1],
            n[2],
            n[3],
            n[4],
            n[5]
        );
    }
    s
}

pub struct Run<'a> {
    pub spec: &'a Spec,
    pub cells: &'a [Cell],
    pub games: &'a [GameResult],
    pub sums: &'a [CellSummary],
    pub groups: &'a [GroupSummary],
    pub cards: &'a BTreeMap<String, BTreeMap<String, [u32; 6]>>,
    pub records: &'a [SavedRecord],
    pub wall_s: f64,
    pub threads: usize,
    pub out_dir: &'a str,
    /// Games the grid has; fewer in `games` is a partial report (a run
    /// still going, or stopped).
    pub planned: usize,
    /// Of `games`, those read back from an earlier run (`--resume`).
    pub resumed: usize,
}

pub fn results_json(run: &Run) -> String {
    let vds = variant_diffs(run.spec, run.cells, run.games);
    // What a module is worth: the base's win rate minus the variant's that
    // switches it off, on the same seeds.
    let removals: Vec<serde_json::Value> = vds
        .iter()
        .filter(|v| !v.off.is_empty())
        .map(|v| {
            serde_json::json!({
                "config": v.config,
                "variant": v.variant,
                "modules": v.off,
                "difficulty": v.difficulty,
                "agent": v.agent,
                "base_win_rate": v.paired.b_win_rate,
                "off_win_rate": v.paired.a_win_rate,
                "worth": -v.paired.diff,
                "worth_se": v.paired.diff_se,
                "sign_p": v.paired.sign_p,
            })
        })
        .collect();
    let v = serde_json::json!({
        "format": "aic-experiment/1",
        "spec": run.spec,
        "wall_seconds": run.wall_s,
        "games_played": run.games.len(),
        "games_planned": run.planned,
        "games_resumed": run.resumed,
        "threads": run.threads,
        "engine_version": aic_core::VERSION,
        "rules_revision": aic_core::RULES_REVISION,
        "content_hash": content_hash(),
        "cells": run.sums,
        "groups": run.groups,
        "top_agent": top_agent(run.cells, run.sums).map(|(a, d)| serde_json::json!({"agent": a, "difficulty": d})),
        "head_to_head": head_to_head(run.cells, run.games),
        "variants": vds,
        "removals": removals,
        "cards": run.cards,
        "records": run.records,
    });
    serde_json::to_string_pretty(&v).expect("results serialize")
}

fn opt(x: Option<f64>, digits: usize) -> String {
    x.map_or("–".into(), |v| format!("{v:.digits$}"))
}

/// Short cell label for tables.
fn short(c: &CellSummary, multi_config: bool, multi_variant: bool) -> String {
    let mut s = String::new();
    if multi_config {
        s.push_str(&c.config_name);
        s.push(' ');
    }
    if multi_variant {
        s.push_str(&c.variant);
        s.push(' ');
    }
    format!("{s}d{} {}", c.difficulty, c.agent)
}

/// Each part's share of the games' own time: agents, probes, applying the
/// actions, legal-action generation, the replay check, setup, and the rest.
pub fn time_split(games: &[GameResult]) -> Vec<(&'static str, f64)> {
    let sum = |f: fn(&GameResult) -> f64| games.iter().map(f).sum::<f64>();
    let total = sum(|g| g.ms).max(1e-9);
    let mut parts = vec![
        ("agents", sum(|g| g.agent_ms)),
        ("probes", sum(|g| g.probe_ms)),
        ("apply", sum(|g| g.apply_ms)),
        ("legal", sum(|g| g.legal_ms)),
        ("replay check", sum(|g| g.replay_ms)),
        ("setup", sum(|g| g.setup_ms)),
    ];
    let other = total - parts.iter().map(|p| p.1).sum::<f64>();
    parts.push(("other", other.max(0.0)));
    parts.into_iter().map(|(n, ms)| (n, ms / total)).collect()
}

pub fn markdown(run: &Run) -> String {
    let spec = run.spec;
    let sums = run.sums;
    let multi_c = spec.configs.len() > 1;
    let multi_v = spec.variants.len() > 1;
    let mut m = String::new();
    let w = &mut m;
    let _ = writeln!(w, "# Cycle report: {}\n", spec.name);
    if run.games.len() < run.planned {
        let _ = writeln!(
            w,
            "**Partial: {} of {} games.** The run is still going or was stopped; every \
             table below covers only the games finished (`--resume` plays the rest).\n",
            run.games.len(),
            run.planned
        );
    }
    if !spec.note.is_empty() {
        let _ = writeln!(w, "{}\n", spec.note);
    }
    let _ = writeln!(
        w,
        "{} games ({} cells × {} seeds, {}..={}) on {} threads in {:.1} s. Engine {}, rules r{}, content {}.\n",
        run.games.len(),
        run.cells.len(),
        spec.games,
        spec.first_seed,
        spec.first_seed + spec.games - 1,
        run.threads,
        run.wall_s,
        aic_core::VERSION,
        aic_core::RULES_REVISION,
        content_hash()
    );
    if run.resumed > 0 {
        let _ = writeln!(
            w,
            "{} of the games were played by an earlier, stopped run (`--resume`); the time is this run's.\n",
            run.resumed
        );
    }
    let _ = writeln!(w, "- Configs: {}", spec.configs.join(", "));
    let vs: Vec<String> = spec
        .variants
        .iter()
        .map(|v| {
            if v.set.is_empty() {
                v.name.clone()
            } else {
                format!("{} ({})", v.name, v.set.join(" "))
            }
        })
        .collect();
    let _ = writeln!(w, "- Variants: {}", vs.join("; "));
    let ds: Vec<String> = spec.difficulties.iter().map(|d| d.to_string()).collect();
    let _ = writeln!(w, "- Difficulties: {}", ds.join(", "));
    let agents: Vec<String> = run
        .cells
        .iter()
        .map(|c| c.agent.label())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let _ = writeln!(w, "- Agents (weakest first): {}", spec.agents.join(", "));
    let _ = writeln!(
        w,
        "- Win probability: {}",
        if spec.winprob.games > 0 {
            format!(
                "first {} seeds per cell, {} `{}` playouts at the start of each round",
                spec.winprob.games, spec.winprob.rollouts, spec.winprob.agent
            )
        } else {
            "off".into()
        }
    );
    let _ = writeln!(
        w,
        "- Decision closeness: {}",
        if spec.closeness.every > 0 {
            format!(
                "every {} real choice(s) probed by 1-ply greedy scoring; close = best two within {} points",
                spec.closeness.every, spec.closeness.gap
            )
        } else {
            "off".into()
        }
    );
    if spec.winprob.games == 0 || spec.closeness.every == 0 {
        let _ = writeln!(
            w,
            "- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`."
        );
    }
    let bad: usize = sums.iter().map(|s| s.games - s.replay_ok).sum();
    let _ = writeln!(
        w,
        "- Replay check: {}\n",
        if bad == 0 {
            "every game replays from seed + actions".to_string()
        } else {
            format!("**{bad} game(s) failed to replay**")
        }
    );
    let _ = agents;

    // Findings first: what a reader needs before the tables.
    let _ = writeln!(w, "## Findings\n");
    for f in findings(run) {
        let _ = writeln!(w, "- {f}");
    }

    let _ = writeln!(w, "\n## Win rate by skill tier × difficulty\n");
    let _ = writeln!(w, "Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).\n");
    for g in run.groups {
        let mut head = format!(
            "| {}{}difficulty {} |",
            if multi_c {
                format!("{} ", g.config)
            } else {
                String::new()
            },
            if multi_v {
                format!("{} ", g.variant)
            } else {
                String::new()
            },
            g.difficulty
        );
        let mut sep = String::from("|---|");
        let mut row = String::from("| win % (survival) |");
        for (a, _) in &g.win_rates {
            let s = sums
                .iter()
                .find(|s| {
                    s.agent == *a
                        && s.difficulty == g.difficulty
                        && s.variant == g.variant
                        && s.config == g.config
                })
                .expect("cell");
            let _ = write!(head, " {a} |");
            sep.push_str("---|");
            let _ = write!(
                row,
                " {:.1} ±{:.1} ({:.2}) |",
                100.0 * s.win_rate,
                100.0 * s.win_rate_se,
                s.survival_mean
            );
        }
        let _ = writeln!(
            w,
            "{head} skill gap |\n{sep}---|\n{row} {:+.1} pts ({:+.2} survival) |\n",
            100.0 * g.skill_gap,
            g.skill_gap_survival
        );
    }

    let cfg = |c: &str| {
        if multi_c {
            format!("{c} ")
        } else {
            String::new()
        }
    };
    let var = |v: &str| {
        if multi_v {
            format!("{v} ")
        } else {
            String::new()
        }
    };
    let h2h = head_to_head(run.cells, run.games);
    if let Some(first) = h2h.first() {
        let _ = writeln!(w, "## Head to head against greedy\n");
        let _ = writeln!(w, "Each agent and `{}` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.\n", first.greedy);
        let _ = writeln!(w, "| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |");
        let _ = writeln!(w, "|---|---|---|---|---|---|---|---|");
        for h in &h2h {
            let p = &h.paired;
            let _ = writeln!(
                w,
                "| {}{}d{} {} | {} | {:.1} | {:.1} | {} / {} / {} / {} | {} | {} | {} |",
                cfg(&h.config),
                var(&h.variant),
                h.difficulty,
                h.agent,
                p.games,
                100.0 * p.a_win_rate,
                100.0 * p.b_win_rate,
                p.both,
                p.only_a,
                p.only_b,
                p.neither,
                pts(p.diff, p.diff_se),
                p_value(p.sign_p),
                h.verdict
            );
        }
        let _ = writeln!(w);
    }
    let vds = variant_diffs(spec, run.cells, run.games);
    if let Some(first) = vds.first() {
        let _ = writeln!(w, "## Variants against the base\n");
        let _ = writeln!(w, "The first variant, `{}`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.\n", first.base);
        let _ = writeln!(
            w,
            "| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |"
        );
        let _ = writeln!(w, "|---|---|---|---|---|---|---|");
        for v in &vds {
            let p = &v.paired;
            let _ = writeln!(
                w,
                "| {}{} | {} | {} | {:.1} → {:.1} | {} | {} | {:.2} → {:.2} |",
                cfg(&v.config),
                v.variant,
                v.difficulty,
                v.agent,
                100.0 * p.b_win_rate,
                100.0 * p.a_win_rate,
                pts(p.diff, p.diff_se),
                p_value(p.sign_p),
                p.b_rounds,
                p.a_rounds
            );
        }
        let _ = writeln!(w);
    }
    if vds.iter().any(|v| !v.off.is_empty()) {
        let _ = writeln!(w, "## Removing a module\n");
        let _ = writeln!(w, "What a module is worth: the base's win rate minus the win rate with the module switched off, on the same seeds, ± the standard error of the per-seed difference. Positive: the module helps the players; negative: it holds them back.\n");
        let _ = writeln!(
            w,
            "| module (variant) | difficulty | agent | base win % | off win % | worth (pts) | p |"
        );
        let _ = writeln!(w, "|---|---|---|---|---|---|---|");
        for v in vds.iter().filter(|v| !v.off.is_empty()) {
            let p = &v.paired;
            let _ = writeln!(
                w,
                "| {}{} ({}) | {} | {} | {:.1} | {:.1} | {} | {} |",
                cfg(&v.config),
                v.off.join(" + "),
                v.variant,
                v.difficulty,
                v.agent,
                100.0 * p.b_win_rate,
                100.0 * p.a_win_rate,
                pts(-p.diff, p.diff_se),
                p_value(p.sign_p)
            );
        }
        let _ = writeln!(w);
    }

    let _ = writeln!(w, "## Game length, margin and end\n");
    let _ = writeln!(w, "| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |");
    let _ = writeln!(w, "|---|---|---|---|---|---|---|");
    for s in sums {
        let ended: Vec<String> = s.ended.iter().map(|(k, v)| format!("{k} {v}")).collect();
        let _ = writeln!(
            w,
            "| {} | {:.2} ({:.2}, {}..{}) | {} | {} / {} | {} / {} | {:.2} | {:.2} / {:.2} |",
            short(s, multi_c, multi_v),
            s.rounds_mean,
            s.rounds_sd,
            s.rounds_min,
            s.rounds_max,
            ended.join(", "),
            opt(s.win_deck_left, 2),
            opt(s.win_supply_left, 2),
            opt(s.loss_blockers, 1),
            opt(s.loss_level, 2),
            s.supply_left_mean,
            s.cities_corrupted_mean,
            s.cities_aligned_mean
        );
    }

    let _ = writeln!(
        w,
        "\n## Decisions, adversary operations, help and counters\n"
    );
    let _ = writeln!(w, "| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |");
    let _ = writeln!(w, "|---|---|---|---|---|---|---|---|---|---|---|");
    for s in sums {
        let _ = writeln!(
            w,
            "| {} | {:.1} | {:.2} | {} | {} | {} | {:.2} ({}) | {:.2} | {:.2} | {:.2} | {:.2} / {:.2} ({}) |",
            short(s, multi_c, multi_v),
            s.decisions_mean,
            s.options_per_decision,
            pct(s.forced_share),
            if spec.closeness.every > 0 { pct(s.close_share) } else { "–".into() },
            if spec.closeness.every > 0 { format!("{:.1}", s.gap_mean) } else { "–".into() },
            s.adversary_ops_per_round,
            s.adversary_ops_max_round,
            s.gives_per_game,
            s.pays_per_game,
            s.withdraws_per_game,
            s.threats_countered_per_game,
            s.threats_dealt_per_game,
            pct(s.countered_share)
        );
    }

    if spec.winprob.games > 0 {
        let _ = writeln!(w, "\n## Win probability by round\n");
        let _ = writeln!(w, "Mean estimated win probability at the start of each round (games still running in brackets). Uncertain: checkpoints with 0.1 < p < 0.9. Early lock-in: games whose result was already decided (p ≤ 0.05 for a loss, ≥ 0.95 for a win, to the end) by round 2.\n");
        let maxr = sums
            .iter()
            .map(|s| s.winprob_by_round.len())
            .max()
            .unwrap_or(0);
        let mut head = String::from("| cell | uncertain | early lock-in |");
        let mut sep = String::from("|---|---|---|");
        for r in 1..=maxr {
            let _ = write!(head, " r{r} |");
            sep.push_str("---|");
        }
        let _ = writeln!(w, "{head}\n{sep}");
        for s in sums {
            let mut row = format!(
                "| {} | {} | {} |",
                short(s, multi_c, multi_v),
                s.uncertain_share.map_or("–".into(), pct),
                s.early_lock_in_share.map_or("–".into(), pct)
            );
            for r in 0..maxr {
                match s.winprob_by_round.get(r) {
                    Some((p, n)) if *n > 0 => {
                        let _ = write!(row, " {p:.2} ({n}) |");
                    }
                    _ => row.push_str(" |"),
                }
            }
            let _ = writeln!(w, "{row}");
        }
    }

    let _ = writeln!(w, "\n## Luck share\n");
    let _ = writeln!(w, "Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.\n");
    let _ = writeln!(
        w,
        "| group | win: luck / skill / rest | survival: luck / skill / rest |"
    );
    let _ = writeln!(w, "|---|---|---|");
    for g in run.groups {
        let f = |x: Option<[f64; 3]>| {
            x.map_or("– (no variance)".into(), |v| {
                format!("{} / {} / {}", pct(v[0]), pct(v[1]), pct(v[2]))
            })
        };
        let _ = writeln!(
            w,
            "| {}{}d{} | {} | {} |",
            if multi_c {
                format!("{} ", g.config)
            } else {
                String::new()
            },
            if multi_v {
                format!("{} ", g.variant)
            } else {
                String::new()
            },
            g.difficulty,
            f(g.luck_win),
            f(g.luck_survival)
        );
    }

    let _ = writeln!(w, "\n## Build variety\n");
    let _ = writeln!(w, "Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.\n");
    let _ = writeln!(w, "| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |");
    let _ = writeln!(w, "|---|---|---|---|---|");
    for s in sums {
        let _ = writeln!(
            w,
            "| {} | {} of {} | {:.2} | {} of {} | {} |",
            short(s, multi_c, multi_v),
            s.distinct_builds,
            s.games,
            s.build_distance,
            s.distinct_builds_wins,
            s.wins,
            opt(s.build_distance_wins, 2)
        );
    }

    let _ = writeln!(w, "\n## Strategy health\n");
    let _ = writeln!(w, "Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.\n");
    let order = agent_order(run.cells);
    let mut gkeys: Vec<(String, String)> = Vec::new();
    for c in run.cells {
        let k = (c.config_path.clone(), c.variant.clone());
        if !gkeys.contains(&k) {
            gkeys.push(k);
        }
    }
    let wins2 = |s: &CellSummary| match (s.wins_2plus_buildings_played, s.wins_2plus_buildings_standing) {
        (Some(a), Some(b)) => format!("{} ({})", pct(a), pct(b)),
        _ => "–".into(),
    };
    for &d in &spec.difficulties {
        let cell_of = |a: &str, k: &(String, String)| {
            sums.iter()
                .find(|s| s.difficulty == d && s.agent == a && s.config == k.0 && s.variant == k.1)
        };
        if gkeys.len() == 1 {
            let _ = writeln!(w, "| difficulty {d} | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |");
            let _ = writeln!(w, "|---|---|---|---|---|");
            for a in &order {
                let l = a.label();
                if let Some(s) = cell_of(&l, &gkeys[0]) {
                    let _ = writeln!(
                        w,
                        "| {l} | {:.2} | {:.2} | {:.1} | {} |",
                        s.buildings_played_per_game,
                        s.tier2_played_per_game,
                        s.cards_played_per_game,
                        wins2(s)
                    );
                }
            }
        } else {
            let mut head = format!("| difficulty {d}: buildings / tier 2 / cards per game, wins with 2+ buildings |");
            let mut sep = String::from("|---|");
            for (c, v) in &gkeys {
                let _ = write!(head, " {}{} |", cfg(c), v);
                sep.push_str("---|");
            }
            let _ = writeln!(w, "{head}\n{sep}");
            for a in &order {
                let l = a.label();
                let mut row = format!("| {l} |");
                for k in &gkeys {
                    match cell_of(&l, k) {
                        Some(s) => {
                            let _ = write!(
                                row,
                                " {:.2} / {:.2} / {:.1}, {} |",
                                s.buildings_played_per_game,
                                s.tier2_played_per_game,
                                s.cards_played_per_game,
                                wins2(s)
                            );
                        }
                        None => row.push_str(" |"),
                    }
                }
                let _ = writeln!(w, "{row}");
            }
        }
        let _ = writeln!(w);
    }

    let _ = writeln!(w, "## Pick and play rates per card\n");
    let _ = writeln!(w, "Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.\n");
    let labels: Vec<&String> = run.cards.keys().collect();
    let mut order: Vec<&String> = labels.clone();
    order.sort_by_key(|l| {
        run.cells
            .iter()
            .find(|c| c.agent.label() == **l)
            .map_or(0, |c| c.agent.tier())
    });
    let mut head = String::from("| card |");
    let mut sep = String::from("|---|");
    for l in &order {
        let _ = write!(head, " {l}: pick (offered) / keep / play |");
        sep.push_str("---|");
    }
    let _ = writeln!(w, "{head}\n{sep}");
    let all_cards: BTreeSet<&String> = run.cards.values().flat_map(|t| t.keys()).collect();
    for card in all_cards {
        let mut row = format!("| {card} |");
        for l in &order {
            let games_of = run
                .games
                .iter()
                .filter(|g| run.cells[g.cell].agent.label() == **l)
                .count()
                .max(1);
            let n = run.cards[*l].get(card).copied().unwrap_or_default();
            let r = |a: u32, b: u32| {
                if b == 0 {
                    "–".to_string()
                } else {
                    pct(a as f64 / b as f64)
                }
            };
            // In hand by keeping or drawing; unique AI cards start there.
            let play = if n[2] + n[5] == 0 && n[0] == 0 {
                format!("{:.2}/game", n[3] as f64 / games_of as f64)
            } else {
                r(n[3], n[2] + n[5])
            };
            let pos = if n[1] == 0 {
                "–".to_string()
            } else {
                format!("{:.1}", n[4] as f64 / n[1] as f64)
            };
            let _ = write!(row, " {} ({}) / {} / {} |", pos, n[0], r(n[2], n[0]), play);
        }
        let _ = writeln!(w, "{row}");
    }

    let _ = writeln!(w, "\n## Timing\n");
    let _ = writeln!(w, "Per game on one thread ({} threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.\n", run.threads);
    let _ = writeln!(
        w,
        "| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |"
    );
    let _ = writeln!(w, "|---|---|---|---|---|");
    for s in sums {
        let _ = writeln!(
            w,
            "| {} | {:.1} | {:.3} | {:.1} | {:.1} |",
            short(s, multi_c, multi_v),
            s.agent_ms_per_game,
            s.agent_ms_per_decision,
            s.probe_ms_per_game,
            s.ms_per_game
        );
    }
    let split: Vec<String> = time_split(run.games)
        .iter()
        .map(|(n, share)| format!("{n} {:.1}%", 100.0 * share))
        .collect();
    let _ = writeln!(
        w,
        "\nWhere the games' time went (summed over workers): {}. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.",
        split.join(", ")
    );

    let _ = writeln!(w, "\n## Saved records\n");
    if run.records.is_empty() {
        let _ = writeln!(w, "None matched.");
    } else {
        let _ = writeln!(w, "Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.\n");
        let cats = ["close-loss", "early-lock-in", "surprising-win"];
        for cat in cats {
            let rs: Vec<&SavedRecord> = run.records.iter().filter(|r| r.category == cat).collect();
            let _ = writeln!(
                w,
                "**{cat}**{}\n",
                if rs.is_empty() { ": none matched" } else { "" }
            );
            for r in rs {
                let rel = r
                    .path
                    .strip_prefix(&format!("{}/", run.out_dir))
                    .unwrap_or(&r.path);
                let _ = writeln!(w, "- [`{rel}`]({rel}): {}", r.why);
            }
            let _ = writeln!(w);
        }
    }
    m
}

/// Plain statements read off the tables, against the targets in
/// `docs/principles.md`.
fn findings(run: &Run) -> Vec<String> {
    let mut f = Vec::new();
    let sums = run.sums;
    let ds: BTreeSet<i64> = sums.iter().map(|s| s.difficulty).collect();
    let lo = *ds.iter().next().unwrap_or(&0);
    let hi = *ds.iter().last().unwrap_or(&0);
    let top_tier = sums.iter().map(|s| s.tier).max().unwrap_or(0);
    // "The strongest agent" below: the top agent by win rate (the ladder's
    // top tier is not always the strongest: 008's engine sits above greedy
    // and two agents share its tier), else the top tier.
    let top_label = top_agent(run.cells, sums).map(|(l, _)| l);
    let strongest = |s: &CellSummary| match &top_label {
        Some(l) => s.agent == *l,
        None => s.tier == top_tier,
    };
    let any_win = sums.iter().any(|s| s.wins > 0);
    if !any_win {
        f.push(format!(
            "**No agent won a game** ({} games). Targets: at difficulty 0 moderately skilled play essentially always wins; at the top difficulty the strongest agent wins narrowly. The ladder can only be read off survival (rounds, margins) until the rules are tuned.",
            run.games.len()
        ));
    }
    for g in run.groups {
        let wr: Vec<String> = g
            .win_rates
            .iter()
            .map(|(a, p)| format!("{a} {:.0}%", 100.0 * p))
            .collect();
        let rr: Vec<String> = g
            .survival
            .iter()
            .map(|(a, r)| format!("{a} {r:.2}"))
            .collect();
        let head = format!(
            "Difficulty {}{}",
            g.difficulty,
            if run.spec.variants.len() > 1 || run.spec.configs.len() > 1 {
                format!(" ({}, {})", g.config, g.variant)
            } else {
                String::new()
            }
        );
        if any_win {
            f.push(format!(
                "{head}: win rates {}; skill gap {:+.0} pts; {}.",
                wr.join(", "),
                100.0 * g.skill_gap,
                if g.ordered {
                    "stronger agents never do worse"
                } else {
                    "**a stronger agent does worse than a weaker one**"
                }
            ));
        }
        f.push(format!(
            "{head}: survival (rounds; wins count extra) {}; {}.",
            rr.join(", "),
            if g.ordered_survival {
                "the ladder is ordered"
            } else {
                "**the ladder is not ordered on survival**"
            }
        ));
    }
    if ds.len() > 1 {
        // Mean over the configs and variants played at each difficulty.
        let gap = |d: i64| {
            mean(
                run.groups
                    .iter()
                    .filter(|g| g.difficulty == d)
                    .map(|g| g.skill_gap),
            )
            .unwrap_or(0.0)
        };
        let gap_r = |d: i64| {
            mean(
                run.groups
                    .iter()
                    .filter(|g| g.difficulty == d)
                    .map(|g| g.skill_gap_survival),
            )
            .unwrap_or(0.0)
        };
        f.push(format!(
            "Skill gap from difficulty {lo} to {hi}: {:+.0} → {:+.0} pts win rate, {:+.2} → {:+.2} survival (target: widens with difficulty).",
            100.0 * gap(lo),
            100.0 * gap(hi),
            gap_r(lo),
            gap_r(hi)
        ));
    }
    let tops: Vec<&CellSummary> = sums.iter().filter(|s| strongest(s)).collect();
    if let (Some(lo_s), Some(hi_s)) = (
        tops.iter().find(|s| s.difficulty == lo),
        tops.iter().find(|s| s.difficulty == hi),
    ) {
        f.push(format!(
            "Game length for the strongest agent ({}): {:.1} rounds at difficulty {lo}, {:.1} at {hi} (target 8–10).",
            hi_s.agent, lo_s.rounds_mean, hi_s.rounds_mean
        ));
    }
    let ends: BTreeMap<&str, usize> =
        sums.iter()
            .flat_map(|s| s.ended.iter())
            .fold(BTreeMap::new(), |mut m, (k, v)| {
                *m.entry(k.as_str()).or_insert(0) += v;
                m
            });
    let e: Vec<String> = ends.iter().map(|(k, v)| format!("{k} {v}")).collect();
    f.push(format!("Games ended by: {}.", e.join(", ")));
    let ops = mean(sums.iter().map(|s| s.adversary_ops_per_round)).unwrap_or(0.0);
    let ops_max = sums
        .iter()
        .map(|s| s.adversary_ops_max_round)
        .max()
        .unwrap_or(0);
    f.push(format!("Adversary operations: {ops:.1} per round on average across cells, at most {ops_max} in one round (target: low)."));
    let probed: Vec<&CellSummary> = sums.iter().filter(|s| strongest(s)).collect();
    if run.spec.closeness.every > 0 {
        let close = mean(probed.iter().map(|s| s.close_share)).unwrap_or(0.0);
        let forced = mean(probed.iter().map(|s| s.forced_share)).unwrap_or(0.0);
        f.push(format!(
            "Decisions (strongest agent): {} forced, {} of real choices close (best two within {} points).",
            pct(forced),
            pct(close),
            run.spec.closeness.gap
        ));
    }
    if let Some(top) = run
        .cards
        .iter()
        .find(|(l, _)| sums.iter().any(|s| s.agent == **l && strongest(s)))
    {
        // Every dealt card is drafted by someone, so keeping is the choice.
        let rate = |n: &[u32; 6]| {
            if n[0] >= 10 {
                Some(n[2] as f64 / n[0] as f64)
            } else {
                None
            }
        };
        let mut dead: Vec<String> = top
            .1
            .iter()
            .filter(|(_, n)| rate(n).is_some_and(|r| r < 0.1))
            .map(|(c, _)| c.clone())
            .collect();
        let mut must: Vec<String> = top
            .1
            .iter()
            .filter(|(_, n)| rate(n).is_some_and(|r| r > 0.9))
            .map(|(c, _)| c.clone())
            .collect();
        dead.sort();
        must.sort();
        f.push(format!(
            "Cards ({}, offered at least 10 times): kept under 10% of the time: {}; over 90%: {}.",
            top.0,
            if dead.is_empty() {
                "none".into()
            } else {
                dead.join(", ")
            },
            if must.is_empty() {
                "none".into()
            } else {
                must.join(", ")
            }
        ));
    }
    let lw: Vec<f64> = run
        .groups
        .iter()
        .filter_map(|g| g.luck_survival.map(|v| v[0]))
        .collect();
    if let Some(l) = mean(lw.iter().copied()) {
        f.push(format!("Luck share of survival (seed's share of the variance): {} on average across groups (target: most variance from decisions).", pct(l)));
    }
    if run.spec.winprob.games > 0 {
        let lock = mean(sums.iter().filter_map(|s| s.early_lock_in_share)).unwrap_or(0.0);
        let unc = mean(sums.iter().filter_map(|s| s.uncertain_share)).unwrap_or(0.0);
        f.push(format!(
            "Win probability: {} of checkpoints uncertain, {} of games decided by round 2 (target: uncertain into the late game). Playouts use `{}`, so they measure that agent's chances.",
            pct(unc),
            pct(lock),
            run.spec.winprob.agent
        ));
    }
    let gives = mean(
        sums.iter()
            .filter(|s| strongest(s))
            .map(|s| s.gives_per_game),
    )
    .unwrap_or(0.0);
    let countered = mean(
        sums.iter()
            .filter(|s| strongest(s))
            .map(|s| s.countered_share),
    )
    .unwrap_or(0.0);
    f.push(format!(
        "Help and counters (strongest agent): {gives:.2} gifts per game; {} of threats countered.",
        pct(countered)
    ));
    f.extend(comparison_findings(run));
    f
}

/// Findings for strategy health, the head-to-head against greedy, the
/// variants and the removals.
fn comparison_findings(run: &Run) -> Vec<String> {
    let mut f = Vec::new();
    let sums = run.sums;
    let order = agent_order(run.cells);
    let base = run.spec.variants.first().map_or("", |v| v.name.as_str());
    let multi_c = run.spec.configs.len() > 1;
    let multi_v = run.spec.variants.len() > 1;
    let where_ = |c: &str| match (multi_c, multi_v) {
        (true, true) => format!(" ({c}, {base})"),
        (true, false) => format!(" ({c})"),
        (false, true) => format!(" ({base})"),
        _ => String::new(),
    };
    // Strategy health: the top agent of each difficulty (base variant).
    for c in &run.spec.configs {
        for &d in &run.spec.difficulties {
            let mut top: Option<&CellSummary> = None;
            for a in &order {
                let l = a.label();
                if let Some(s) = sums
                    .iter()
                    .find(|s| s.config == *c && s.variant == base && s.difficulty == d && s.agent == l)
                {
                    // Ties go to the later agent.
                    match top {
                        Some(t) if t.win_rate > s.win_rate => {}
                        _ => top = Some(s),
                    }
                }
            }
            let Some(s) = top else { continue };
            f.push(format!(
                "Strategy health at difficulty {d}{}: the top agent there ({}, {:.0}% wins) plays {:.2} buildings and {:.2} tier-2 cards per game (of {:.1} cards); {} of its wins had 2+ buildings played ({} with 2+ standing at the end). {}; {}.",
                where_(c),
                s.agent,
                100.0 * s.win_rate,
                s.buildings_played_per_game,
                s.tier2_played_per_game,
                s.cards_played_per_game,
                s.wins_2plus_buildings_played.map_or("–".into(), pct),
                s.wins_2plus_buildings_standing.map_or("–".into(), pct),
                if s.buildings_played_per_game >= 2.0 {
                    "It builds (healthy: 2+ buildings per game)"
                } else {
                    "**It hardly builds** (under 2 buildings per game)"
                },
                if s.tier2_played_per_game >= 0.5 {
                    "it plays tier 2"
                } else {
                    "**it hardly plays tier 2** (under 0.5 per game)"
                }
            ));
        }
    }
    // Head to head: the top agent against greedy at the top difficulty.
    let h2h = head_to_head(run.cells, run.games);
    if let (Some((top, hi)), Some(first)) = (top_agent(run.cells, sums), h2h.first()) {
        let gl = &first.greedy;
        let line = |h: &HeadToHead| {
            let p = &h.paired;
            format!(
                "{} {:.0}% vs greedy {:.0}% over {} seeds (only {} {}, only greedy {}), {} pts, sign test p {}: {}",
                h.agent,
                100.0 * p.a_win_rate,
                100.0 * p.b_win_rate,
                p.games,
                h.agent,
                p.only_a,
                p.only_b,
                pts(p.diff, p.diff_se),
                p_value(p.sign_p),
                h.verdict
            )
        };
        let at_top = |a: &str| -> Vec<&HeadToHead> {
            h2h.iter()
                .filter(|h| h.agent == a && h.difficulty == hi && h.variant == base)
                .collect()
        };
        if top == *gl {
            // Greedy is the top agent: report the best of the rest.
            let rest: Vec<&HeadToHead> = h2h
                .iter()
                .filter(|h| h.difficulty == hi && h.variant == base)
                .collect();
            let best = rest.iter().copied().fold(None::<&HeadToHead>, |b, h| match b {
                Some(b) if h.paired.diff < b.paired.diff => Some(b),
                _ => Some(h),
            });
            let mut s = format!(
                "Top agent ({top}: highest win rate at difficulty {hi}, ties to the later agent) is greedy itself"
            );
            if let Some(b) = best {
                let _ = write!(s, "; the closest other agent at difficulty {hi}{}: {}", where_(&b.config), line(b));
            }
            f.push(format!("{s}."));
        } else {
            for h in at_top(&top) {
                f.push(format!(
                    "Top agent ({top}: highest win rate at difficulty {hi}, ties to the later agent) vs greedy at difficulty {hi}{}: {}.",
                    where_(&h.config),
                    line(h)
                ));
            }
        }
    }
    // Variants: each against the base for the top agent at each difficulty.
    let vds = variant_diffs(run.spec, run.cells, run.games);
    if !vds.is_empty() {
        let who = top_agent(run.cells, sums).map(|(a, _)| a);
        for c in &run.spec.configs {
            for &d in &run.spec.difficulties {
                let Some(a) = &who else { continue };
                let parts: Vec<String> = vds
                    .iter()
                    .filter(|v| v.config == *c && v.difficulty == d && v.agent == *a)
                    .map(|v| {
                        let p = &v.paired;
                        let s = format!("{} {}", v.variant, pts(p.diff, p.diff_se));
                        if p.sign_p < 0.05 {
                            format!("**{s}**")
                        } else {
                            s
                        }
                    })
                    .collect();
                if parts.is_empty() {
                    continue;
                }
                f.push(format!(
                    "Variants against the base ({base}) at difficulty {d}{}, {a}, win rate difference in pts (bold: sign test p < 0.05): {}.",
                    if multi_c { format!(" ({c})") } else { String::new() },
                    parts.join("; ")
                ));
            }
        }
        let offs: Vec<&VariantDiff> = vds.iter().filter(|v| !v.off.is_empty()).collect();
        if !offs.is_empty() {
            for c in &run.spec.configs {
                for &d in &run.spec.difficulties {
                    let Some(a) = &who else { continue };
                    let parts: Vec<String> = offs
                        .iter()
                        .filter(|v| v.config == *c && v.difficulty == d && v.agent == *a)
                        .map(|v| {
                            let p = &v.paired;
                            let s = format!("{} {}", v.off.join(" + "), pts(-p.diff, p.diff_se));
                            if p.sign_p < 0.05 {
                                format!("**{s}**")
                            } else {
                                s
                            }
                        })
                        .collect();
                    if parts.is_empty() {
                        continue;
                    }
                    f.push(format!(
                        "What a module is worth (base win rate minus with it off) at difficulty {d}{}, {a}: {} pts.",
                        if multi_c { format!(" ({c})") } else { String::new() },
                        parts.join("; ")
                    ));
                }
            }
        }
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_test_is_exact() {
        assert_eq!(sign_test(0, 0), 1.0);
        assert!((sign_test(0, 3) - 0.25).abs() < 1e-12);
        assert!((sign_test(6, 0) - 2.0 / 64.0).abs() < 1e-12);
        // 10 of 12 discordant one way: 2 * (1 + 12 + 66) / 4096.
        assert!((sign_test(2, 10) - 158.0 / 4096.0).abs() < 1e-12);
        assert_eq!(sign_test(5, 5), 1.0);
        assert!(sign_test(400, 600) < 1e-9);
    }

    #[test]
    fn modules_switched_off() {
        let set = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(
            switched_off(&set(&["opposition.variant=off", "threats=off"])),
            vec!["opposition", "threats"]
        );
        // Not a pure removal: other numbers change too.
        assert!(switched_off(&set(&["opposition.variant=off", "x.y=2"])).is_empty());
        assert!(switched_off(&set(&["threats.variant=shared_row"])).is_empty());
        assert_eq!(standing("builder:a+b|protector:"), 2);
    }
}
