//! The measures: per cell, per group of cells across agents, per card, and
//! paired comparisons of two cells on the same seeds.

use std::collections::{BTreeMap, BTreeSet};

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

pub(super) fn mean(v: impl Iterator<Item = f64>) -> Option<f64> {
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
pub(super) fn standing(sig: &str) -> usize {
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
pub(super) fn agent_order(cells: &[Cell]) -> Vec<&AgentSpec> {
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
pub(super) fn top_agent(cells: &[Cell], sums: &[CellSummary]) -> Option<(String, i64)> {
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
pub(super) fn measured_rank(cells: &[Cell], games: &[GameResult]) -> BTreeMap<String, usize> {
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
pub(super) fn lock_in_round(r: &GameResult) -> Option<usize> {
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
            switched_off(&set(&["alignment.variant=off", "threats=off"])),
            vec!["alignment", "threats"]
        );
        // Not a pure removal: other numbers change too.
        assert!(switched_off(&set(&["alignment.variant=off", "x.y=2"])).is_empty());
        assert!(switched_off(&set(&["threats.variant=shared_row"])).is_empty());
        assert_eq!(standing("builder:a+b|protector:"), 2);
    }
}
