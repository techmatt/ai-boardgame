//! Aggregates an experiment's games into the measures of
//! `docs/principles.md` and writes the cycle report: `report.md` (tables,
//! findings, the saved records), `results.json` (everything, machine
//! readable), `games.csv` and `cards.csv` (one row per game, per card), and
//! `records/` (game records worth a human look, openable in the viewer).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use aic_core::{content_hash, SavedGame};
use serde::Serialize;

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
    /// Margin on wins: adversary cards left (rounds to spare), pool left.
    pub win_deck_left: Option<f64>,
    pub win_pool_left: Option<f64>,
    /// Distance on losses: adversary pieces over the bar, opposition level.
    pub loss_blockers: Option<f64>,
    pub loss_level: Option<f64>,
    pub pool_left_mean: f64,
    pub population_slack_mean: f64,
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
                win_pool_left: mean(won.iter().map(|g| g.margin.pool_left as f64)),
                loss_blockers: mean(lost.iter().map(|g| g.margin.blockers as f64)),
                loss_level: mean(lost.iter().map(|g| g.margin.level as f64)),
                pool_left_mean: mean(gs.iter().map(|g| g.margin.pool_left as f64)).unwrap_or(0.0),
                population_slack_mean: mean(gs.iter().map(|g| g.margin.population_slack as f64))
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
            let table = |f: &dyn Fn(&GameResult) -> f64| -> Vec<Vec<f64>> {
                idx.iter()
                    .map(|&i| games.iter().filter(|g| g.cell == i).map(f).collect())
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
    // Records from an earlier run of this experiment would sit next to the
    // new ones unlisted: remove the ones this harness names.
    for e in std::fs::read_dir(&rec_dir)
        .map_err(|e| format!("{rec_dir}: {e}"))?
        .flatten()
    {
        let name = e.file_name().to_string_lossy().to_string();
        let ours = ["close-loss-", "early-lock-in-", "surprising-win-"]
            .iter()
            .any(|p| name.starts_with(p));
        if ours && name.ends_with(".json") {
            std::fs::remove_file(e.path()).map_err(|err| format!("{name}: {err}"))?;
        }
    }
    let top_d = cells.iter().map(|c| c.difficulty).max().unwrap_or(0);
    let top_tier = cells.iter().map(|c| c.agent.tier()).max().unwrap_or(0);
    let low_tier = cells.iter().map(|c| c.agent.tier()).min().unwrap_or(0);
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
    // the longest games, stronger agents first.
    let mut close: Vec<(usize, &GameResult)> = games
        .iter()
        .enumerate()
        .filter(|(_, g)| !g.won && cells[g.cell].difficulty == top_d && g.reason != "timeout")
        .collect();
    close.sort_by_key(|(_, g)| {
        (
            g.margin.blockers,
            std::cmp::Reverse(g.rounds),
            std::cmp::Reverse(cells[g.cell].agent.tier()),
            g.seed,
        )
    });
    take(
        "close-loss",
        close
            .iter()
            .map(|(i, g)| {
                (*i, format!(
                    "close loss at difficulty {top_d}: {} in round {}, {} adversary piece(s) over the level-{} bar, pool {} of {}",
                    g.reason, g.rounds, g.margin.blockers, g.margin.level, g.margin.pool_left, g.margin.pool_start
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
            std::cmp::Reverse(cells[g.cell].agent.tier()),
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
        let why = if c.agent.tier() == low_tier && low_tier < top_tier {
            Some((0, format!("won by the weakest agent ({})", c.agent.label())))
        } else if low <= 0.25 {
            Some((1, format!("won after the playouts gave it {low:.2}")))
        } else if c.difficulty == top_d && c.agent.tier() < top_tier {
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
        std::fs::write(&file, rec.to_text()).map_err(|e| format!("{file}: {e}"))?;
        out.push(SavedRecord {
            category: cat,
            path: file,
            why,
        });
    }
    Ok(out)
}

/// A CSV text field, quoted (with `"` doubled).
fn q(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

pub fn games_csv(cells: &[Cell], games: &[GameResult]) -> String {
    let mut s = String::from(
        "config,variant,difficulty,agent,seed,won,reason,rounds,decisions,options,forced,probed,close,gap_mean,deck_left,pool_left,pool_start,population_slack,blockers,level,points,adversary_ops,adversary_ops_max,setup_ops,gives,pays,withdraws,threats_dealt,threats_countered,build,winprob,ms,replay_ok\n",
    );
    for g in games {
        let c = &cells[g.cell];
        let ops: u32 = g.adversary_ops.iter().sum();
        let wp: Vec<String> = g.winprob.iter().map(|p| format!("{p:.3}")).collect();
        let cnt = |k: &str| g.counters.get(k).copied().unwrap_or(0);
        let _ = writeln!(
            s,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.1},{}",
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
            g.margin.pool_left,
            g.margin.pool_start,
            g.margin.population_slack,
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
}

pub fn results_json(run: &Run) -> String {
    let v = serde_json::json!({
        "format": "aic-experiment/1",
        "spec": run.spec,
        "wall_seconds": run.wall_s,
        "threads": run.threads,
        "engine_version": aic_core::VERSION,
        "rules_revision": aic_core::RULES_REVISION,
        "content_hash": content_hash(),
        "cells": run.sums,
        "groups": run.groups,
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

pub fn markdown(run: &Run) -> String {
    let spec = run.spec;
    let sums = run.sums;
    let multi_c = spec.configs.len() > 1;
    let multi_v = spec.variants.len() > 1;
    let mut m = String::new();
    let w = &mut m;
    let _ = writeln!(w, "# Cycle report: {}\n", spec.name);
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

    let _ = writeln!(w, "## Game length, margin and end\n");
    let _ = writeln!(w, "| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |");
    let _ = writeln!(w, "|---|---|---|---|---|---|---|");
    for s in sums {
        let ended: Vec<String> = s.ended.iter().map(|(k, v)| format!("{k} {v}")).collect();
        let _ = writeln!(
            w,
            "| {} | {:.2} ({:.2}, {}..{}) | {} | {} / {} | {} / {} | {:.2} | {:.2} |",
            short(s, multi_c, multi_v),
            s.rounds_mean,
            s.rounds_sd,
            s.rounds_min,
            s.rounds_max,
            ended.join(", "),
            opt(s.win_deck_left, 2),
            opt(s.win_pool_left, 2),
            opt(s.loss_blockers, 1),
            opt(s.loss_level, 2),
            s.pool_left_mean,
            s.population_slack_mean
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

    let _ = writeln!(w, "\n## Pick and play rates per card\n");
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
    let tops: Vec<&CellSummary> = sums.iter().filter(|s| s.tier == top_tier).collect();
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
    let probed: Vec<&CellSummary> = sums.iter().filter(|s| s.tier == top_tier).collect();
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
        .find(|(l, _)| sums.iter().any(|s| s.agent == **l && s.tier == top_tier))
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
            .filter(|s| s.tier == top_tier)
            .map(|s| s.gives_per_game),
    )
    .unwrap_or(0.0);
    let countered = mean(
        sums.iter()
            .filter(|s| s.tier == top_tier)
            .map(|s| s.countered_share),
    )
    .unwrap_or(0.0);
    f.push(format!(
        "Help and counters (strongest agent): {gives:.2} gifts per game; {} of threats countered.",
        pct(countered)
    ));
    f
}
