//! `report.md`: the run, the findings, then the tables, the timing and the
//! saved records.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use aic_core::content_hash;

use super::findings::findings;
use super::stats::{agent_order, time_split, CellSummary, Paired};
use super::{Run, SavedRecord};

/// `+4.2 ±3.1` (percentage points).
pub(super) fn pts(x: f64, se: f64) -> String {
    // `+ 0.0` turns a negated zero into a plain one.
    format!("{:+.1} ±{:.1}", 100.0 * x + 0.0, 100.0 * se)
}

pub(super) fn p_value(p: f64) -> String {
    if p < 0.001 {
        "<0.001".into()
    } else {
        format!("{p:.3}")
    }
}

pub(super) fn pct(x: f64) -> String {
    format!("{:.0}%", 100.0 * x)
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
    let mut overlays: Vec<String> = sums
        .iter()
        .filter(|s| s.content_hash != content_hash())
        .map(|s| format!("{} {}", s.variant, s.content_hash))
        .collect();
    overlays.dedup();
    if !overlays.is_empty() {
        let _ = writeln!(w, "- Overlaid content: {}", overlays.join("; "));
    }
    let ds: Vec<String> = spec.difficulties.iter().map(|d| d.to_string()).collect();
    let _ = writeln!(w, "- Difficulties: {}", ds.join(", "));
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

    // Findings first: what a reader needs before the tables.
    let _ = writeln!(w, "## Findings\n");
    for f in findings(run) {
        let _ = writeln!(w, "- {f}");
    }

    if let Some(g) = run.gate {
        let _ = write!(w, "{}", g.markdown());
    }

    let _ = writeln!(w, "\n## Win rate by skill tier × difficulty\n");
    let _ = writeln!(w, "Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.\n");
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
    let h2h = run.head_to_head;
    if let Some(first) = h2h.first() {
        let _ = writeln!(w, "## Head to head against greedy\n");
        let _ = writeln!(w, "Each agent and `{}` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.\n", first.greedy);
        let _ = writeln!(w, "| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |");
        let _ = writeln!(w, "|---|---|---|---|---|---|---|---|");
        for h in h2h {
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
    if !run.agent_pairs.is_empty() {
        let _ = writeln!(w, "## Agents paired, per variant\n");
        let _ = writeln!(w, "A against B on the same seeds. An agent labelled `x:...` is a check against `x`; a bare agent is paired with the next bare agent listed. A minus B: the win-rate difference ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.\n");
        let _ = writeln!(w, "| variant | d | A | B | games | A win % | B win % | both / only A / only B / neither | A minus B (pts ± se) | p |");
        let _ = writeln!(w, "|---|---|---|---|---|---|---|---|---|---|");
        for x in run.agent_pairs {
            let p = &x.paired;
            let _ = writeln!(
                w,
                "| {}{} | {} | {} | {} | {} | {:.1} | {:.1} | {} / {} / {} / {} | {} | {} |",
                cfg(&x.config),
                x.variant,
                x.difficulty,
                x.a,
                x.b,
                p.games,
                100.0 * p.a_win_rate,
                100.0 * p.b_win_rate,
                p.both,
                p.only_a,
                p.only_b,
                p.neither,
                pts(p.diff, p.diff_se),
                p_value(p.sign_p)
            );
        }
        let _ = writeln!(w);
    }
    let vds = run.variant_diffs;
    if let Some(first) = vds.first() {
        let _ = writeln!(w, "## Variants against the base\n");
        let _ = writeln!(w, "The first variant, `{}`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.\n", first.base);
        let _ = writeln!(
            w,
            "| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |"
        );
        let _ = writeln!(w, "|---|---|---|---|---|---|---|");
        for v in vds {
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
    if let Some(sel) = &spec.selection_seeds {
        let _ = writeln!(w, "## Fresh-seed confirmation\n");
        let fresh = run.fresh_confirmation;
        let _ = writeln!(w, "Selection seeds {}..{} picked the winner; fresh seeds are every other seed played. Variant minus base, ± the standard error of the per-seed difference, with p from the exact sign test. The two are never pooled: only the fresh row confirms.\n", sel.first, sel.first.saturating_add(sel.games));
        if fresh.is_empty() {
            let _ = writeln!(w, "Nothing to confirm: no variant after the base has a pair.\n");
        } else if fresh.iter().all(|f| f.fresh.games == 0) {
            let _ = writeln!(w, "No fresh seeds yet: every game so far is on the selection seeds.\n");
        }
        if !fresh.is_empty() {
            let _ = writeln!(w, "| variant | d | agent | fresh games | fresh win % base → variant | fresh difference (pts) | fresh p | selection games | selection win % base → variant | selection difference (pts) | selection p |");
            let _ = writeln!(w, "|---|---|---|---|---|---|---|---|---|---|---|");
            let side = |p: &Paired| {
                if p.games == 0 {
                    "0 | – | – | –".to_string()
                } else {
                    format!(
                        "{} | {:.1} → {:.1} | {} | {}",
                        p.games,
                        100.0 * p.b_win_rate,
                        100.0 * p.a_win_rate,
                        pts(p.diff, p.diff_se),
                        p_value(p.sign_p)
                    )
                }
            };
            for f in fresh {
                let _ = writeln!(
                    w,
                    "| {}{} | {} | {} | {} | {} |",
                    cfg(&f.config),
                    f.variant,
                    f.difficulty,
                    f.agent,
                    side(&f.fresh),
                    side(&f.selection)
                );
            }
            let _ = writeln!(w);
        }
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
    let _ = writeln!(w, "Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played, economy buildings (printing standing income) standing at the end; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.\n");
    let order = agent_order(run.cells);
    let mut gkeys: Vec<(String, String)> = Vec::new();
    for c in run.cells {
        let k = (c.config_path.clone(), c.variant.clone());
        if !gkeys.contains(&k) {
            gkeys.push(k);
        }
    }
    let wins2 = |s: &CellSummary| match (
        s.wins_2plus_buildings_played,
        s.wins_2plus_buildings_standing,
    ) {
        (Some(a), Some(b)) => format!("{} ({})", pct(a), pct(b)),
        _ => "–".into(),
    };
    for &d in &spec.difficulties {
        let cell_of = |a: &str, k: &(String, String)| {
            sums.iter()
                .find(|s| s.difficulty == d && s.agent == a && s.config == k.0 && s.variant == k.1)
        };
        if gkeys.len() == 1 {
            let _ = writeln!(w, "| difficulty {d} | buildings played/game | tier 2 played/game | cards played/game | economy buildings standing/game | wins with 2+ buildings played (standing) |");
            let _ = writeln!(w, "|---|---|---|---|---|---|");
            for a in &order {
                let l = a.label();
                if let Some(s) = cell_of(&l, &gkeys[0]) {
                    let _ = writeln!(
                        w,
                        "| {l} | {:.2} | {:.2} | {:.1} | {:.2} | {} |",
                        s.buildings_played_per_game,
                        s.tier2_played_per_game,
                        s.cards_played_per_game,
                        s.economy_buildings_standing_per_game,
                        wins2(s)
                    );
                }
            }
        } else {
            let mut head = format!(
                "| difficulty {d}: buildings / tier 2 / cards / economy standing per game, wins with 2+ buildings |"
            );
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
                                " {:.2} / {:.2} / {:.1} / {:.2}, {} |",
                                s.buildings_played_per_game,
                                s.tier2_played_per_game,
                                s.cards_played_per_game,
                                s.economy_buildings_standing_per_game,
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
    let _ = writeln!(w, "Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per variant, config and difficulty in `cards.csv`.\n");
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
            let play = if n.kept + n.drawn == 0 && n.offered == 0 {
                format!("{:.2}/game", n.played as f64 / games_of as f64)
            } else {
                r(n.played, n.kept + n.drawn)
            };
            let pos = if n.drafted == 0 {
                "–".to_string()
            } else {
                format!("{:.1}", n.pick_sum as f64 / n.drafted as f64)
            };
            let _ = write!(
                row,
                " {} ({}) / {} / {} |",
                pos,
                n.offered,
                r(n.kept, n.offered),
                play
            );
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
