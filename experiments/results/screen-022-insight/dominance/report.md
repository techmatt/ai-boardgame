# Cycle report: screen-022-dominance

022 dominance checks (r13): the engine against seven fixed policies (the engine with one behaviour forced) at d0, d2 and d4, 2 and 3 countries. Each check should lose ground to the engine at d2 and above, and none match it at d4.

768 games (48 cells × 16 seeds, 0..=15) on 12 threads in 483.7 s. Engine 0.1.0, rules r13, content e3d97d1e972a7982.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 2, 4
- Agents (weakest first): engine, engine:keep_all=1, engine:keep_k=2, engine:tier1_only=1, engine:tier2_rush=1, engine:no_powers=1, engine:no_counters=1, engine:no_presence=1
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates engine 94%, engine:keep_all=1 94%, engine:keep_k=2 25%, engine:tier1_only=1 81%, engine:tier2_rush=1 100%, engine:no_powers=1 94%, engine:no_counters=1 81%, engine:no_presence=1 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) engine 12.94, engine:keep_all=1 12.94, engine:keep_k=2 12.06, engine:tier1_only=1 12.81, engine:tier2_rush=1 13.00, engine:no_powers=1 12.94, engine:no_counters=1 12.81, engine:no_presence=1 10.44; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c2): win rates engine 62%, engine:keep_all=1 81%, engine:keep_k=2 0%, engine:tier1_only=1 44%, engine:tier2_rush=1 50%, engine:no_powers=1 56%, engine:no_counters=1 25%, engine:no_presence=1 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) engine 11.06, engine:keep_all=1 11.44, engine:keep_k=2 8.25, engine:tier1_only=1 10.88, engine:tier2_rush=1 10.94, engine:no_powers=1 11.19, engine:no_counters=1 11.06, engine:no_presence=1 7.81; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c2): win rates engine 25%, engine:keep_all=1 19%, engine:keep_k=2 0%, engine:tier1_only=1 19%, engine:tier2_rush=1 25%, engine:no_powers=1 19%, engine:no_counters=1 0%, engine:no_presence=1 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) engine 7.44, engine:keep_all=1 6.69, engine:keep_k=2 4.88, engine:tier1_only=1 7.19, engine:tier2_rush=1 6.75, engine:no_powers=1 7.06, engine:no_counters=1 5.19, engine:no_presence=1 4.62; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): win rates engine 94%, engine:keep_all=1 38%, engine:keep_k=2 0%, engine:tier1_only=1 94%, engine:tier2_rush=1 69%, engine:no_powers=1 81%, engine:no_counters=1 69%, engine:no_presence=1 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) engine 12.94, engine:keep_all=1 12.38, engine:keep_k=2 12.00, engine:tier1_only=1 12.94, engine:tier2_rush=1 12.69, engine:no_powers=1 12.81, engine:no_counters=1 12.69, engine:no_presence=1 11.44; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates engine 50%, engine:keep_all=1 25%, engine:keep_k=2 6%, engine:tier1_only=1 31%, engine:tier2_rush=1 25%, engine:no_powers=1 56%, engine:no_counters=1 25%, engine:no_presence=1 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) engine 11.50, engine:keep_all=1 10.62, engine:keep_k=2 8.88, engine:tier1_only=1 11.31, engine:tier2_rush=1 9.75, engine:no_powers=1 11.56, engine:no_counters=1 10.31, engine:no_presence=1 8.88; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates engine 6%, engine:keep_all=1 6%, engine:keep_k=2 0%, engine:tier1_only=1 6%, engine:tier2_rush=1 0%, engine:no_powers=1 0%, engine:no_counters=1 0%, engine:no_presence=1 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) engine 5.81, engine:keep_all=1 4.94, engine:keep_k=2 4.62, engine:tier1_only=1 5.88, engine:tier2_rush=1 4.75, engine:no_powers=1 5.50, engine:no_counters=1 5.19, engine:no_presence=1 4.75; the ladder is ordered.
- Skill gap from difficulty 0 to 4 (mean over 2 configs/variants): +0 → +0 pts win rate, +0.00 → +0.00 survival (target: widens with difficulty).
- Game length for the strongest agent (engine:tier2_rush=1; configs/baseline.json, c2): 5.0 rounds at difficulty 0, 4.8 at 4 (target 8–10).
- Game length for the strongest agent (engine:tier2_rush=1; configs/baseline.json, c3): 8.6 rounds at difficulty 0, 4.8 at 4 (target 8–10).
- Games ended by: cities 16, deck_empty 173, purge 268, supply 311.
- Adversary operations: 22.6 per round on average across cells, at most 143 in one round (target: low).
- Cards (engine:tier2_rush=1, offered at least 10 times): kept under 10% of the time: antimatter_refinery, fusion_plant, orbital_lance, quantum_core, swarm_override; over 90%: assembly_cluster, civic_shelter, contract_bid, data_lake, data_mining, diaspora, emergency_fab, fab_line, field_office, firewall_node, fork_process, grant, grassroots_movement, holding_company, jam_signals, liquidate_reserves, logistics_ai, power_share, power_surge, precision_strike, rally, rally_point, rebuild, recon_sweep, scavenge, security_cell, security_patch, seed_funding, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 20% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 7.73 gifts per game; 53% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine:tier2_rush=1, 100% wins) plays 5.00 buildings and 4.00 tier-2 cards per game (of 15.2 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 2 (c2): the top agent there (engine:keep_all=1, 81% wins) plays 4.44 buildings and 1.38 tier-2 cards per game (of 32.2 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (c2): the top agent there (engine:tier2_rush=1, 25% wins) plays 3.69 buildings and 2.31 tier-2 cards per game (of 12.4 cards); 75% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Variants against the base (c2) at difficulty 0, engine:tier2_rush=1, win rate difference in pts (bold: sign test p < 0.05): c3 -31.2 ±12.0.
- Variants against the base (c2) at difficulty 2, engine:tier2_rush=1, win rate difference in pts (bold: sign test p < 0.05): c3 -25.0 ±17.1.
- Variants against the base (c2) at difficulty 4, engine:tier2_rush=1, win rate difference in pts (bold: sign test p < 0.05): c3 -25.0 ±11.2.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.

| c2 difficulty 0 | engine | engine:keep_all=1 | engine:keep_k=2 | engine:tier1_only=1 | engine:tier2_rush=1 | engine:no_powers=1 | engine:no_counters=1 | engine:no_presence=1 | skill gap |
|---|---|---|---|---|---|---|---|---|---|
| win % (survival) | 93.8 ±6.1 (12.94) | 93.8 ±6.1 (12.94) | 25.0 ±10.8 (12.06) | 81.2 ±9.8 (12.81) | 100.0 ±0.0 (13.00) | 93.8 ±6.1 (12.94) | 81.2 ±9.8 (12.81) | 0.0 ±0.0 (10.44) | +0.0 pts (+0.00 survival) |

| c2 difficulty 2 | engine | engine:keep_all=1 | engine:keep_k=2 | engine:tier1_only=1 | engine:tier2_rush=1 | engine:no_powers=1 | engine:no_counters=1 | engine:no_presence=1 | skill gap |
|---|---|---|---|---|---|---|---|---|---|
| win % (survival) | 62.5 ±12.1 (11.06) | 81.2 ±9.8 (11.44) | 0.0 ±0.0 (8.25) | 43.8 ±12.4 (10.88) | 50.0 ±12.5 (10.94) | 56.2 ±12.4 (11.19) | 25.0 ±10.8 (11.06) | 0.0 ±0.0 (7.81) | +0.0 pts (+0.00 survival) |

| c2 difficulty 4 | engine | engine:keep_all=1 | engine:keep_k=2 | engine:tier1_only=1 | engine:tier2_rush=1 | engine:no_powers=1 | engine:no_counters=1 | engine:no_presence=1 | skill gap |
|---|---|---|---|---|---|---|---|---|---|
| win % (survival) | 25.0 ±10.8 (7.44) | 18.8 ±9.8 (6.69) | 0.0 ±0.0 (4.88) | 18.8 ±9.8 (7.19) | 25.0 ±10.8 (6.75) | 18.8 ±9.8 (7.06) | 0.0 ±0.0 (5.19) | 0.0 ±0.0 (4.62) | +0.0 pts (+0.00 survival) |

| c3 difficulty 0 | engine | engine:keep_all=1 | engine:keep_k=2 | engine:tier1_only=1 | engine:tier2_rush=1 | engine:no_powers=1 | engine:no_counters=1 | engine:no_presence=1 | skill gap |
|---|---|---|---|---|---|---|---|---|---|
| win % (survival) | 93.8 ±6.1 (12.94) | 37.5 ±12.1 (12.38) | 0.0 ±0.0 (12.00) | 93.8 ±6.1 (12.94) | 68.8 ±11.6 (12.69) | 81.2 ±9.8 (12.81) | 68.8 ±11.6 (12.69) | 0.0 ±0.0 (11.44) | +0.0 pts (+0.00 survival) |

| c3 difficulty 2 | engine | engine:keep_all=1 | engine:keep_k=2 | engine:tier1_only=1 | engine:tier2_rush=1 | engine:no_powers=1 | engine:no_counters=1 | engine:no_presence=1 | skill gap |
|---|---|---|---|---|---|---|---|---|---|
| win % (survival) | 50.0 ±12.5 (11.50) | 25.0 ±10.8 (10.62) | 6.2 ±6.1 (8.88) | 31.2 ±11.6 (11.31) | 25.0 ±10.8 (9.75) | 56.2 ±12.4 (11.56) | 25.0 ±10.8 (10.31) | 0.0 ±0.0 (8.88) | +0.0 pts (+0.00 survival) |

| c3 difficulty 4 | engine | engine:keep_all=1 | engine:keep_k=2 | engine:tier1_only=1 | engine:tier2_rush=1 | engine:no_powers=1 | engine:no_counters=1 | engine:no_presence=1 | skill gap |
|---|---|---|---|---|---|---|---|---|---|
| win % (survival) | 6.2 ±6.1 (5.81) | 6.2 ±6.1 (4.94) | 0.0 ±0.0 (4.62) | 6.2 ±6.1 (5.88) | 0.0 ±0.0 (4.75) | 0.0 ±0.0 (5.50) | 0.0 ±0.0 (5.19) | 0.0 ±0.0 (4.75) | +0.0 pts (+0.00 survival) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | engine | 93.8 → 93.8 | +0.0 ±0.0 | 1.000 | 6.94 → 6.06 |
| c3 | 0 | engine:keep_all=1 | 93.8 → 37.5 | -56.2 ±12.8 | 0.004 | 7.06 → 10.06 |
| c3 | 0 | engine:keep_k=2 | 25.0 → 0.0 | -25.0 ±11.2 | 0.125 | 10.94 → 12.00 |
| c3 | 0 | engine:tier1_only=1 | 81.2 → 93.8 | +12.5 ±8.5 | 0.500 | 7.44 → 6.25 |
| c3 | 0 | engine:tier2_rush=1 | 100.0 → 68.8 | -31.2 ±12.0 | 0.062 | 5.00 → 8.62 |
| c3 | 0 | engine:no_powers=1 | 93.8 → 81.2 | -12.5 ±12.5 | 0.625 | 7.50 → 6.75 |
| c3 | 0 | engine:no_counters=1 | 81.2 → 68.8 | -12.5 ±18.0 | 0.727 | 8.12 → 8.44 |
| c3 | 0 | engine:no_presence=1 | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 10.44 → 11.44 |
| c3 | 2 | engine | 62.5 → 50.0 | -12.5 ±18.0 | 0.727 | 8.00 → 8.81 |
| c3 | 2 | engine:keep_all=1 | 81.2 → 25.0 | -56.2 ±12.8 | 0.004 | 7.31 → 9.31 |
| c3 | 2 | engine:keep_k=2 | 0.0 → 6.2 | +6.2 ±6.2 | 1.000 | 8.25 → 8.75 |
| c3 | 2 | engine:tier1_only=1 | 43.8 → 31.2 | -12.5 ±20.2 | 0.754 | 8.69 → 9.38 |
| c3 | 2 | engine:tier2_rush=1 | 50.0 → 25.0 | -25.0 ±17.1 | 0.289 | 7.31 → 8.31 |
| c3 | 2 | engine:no_powers=1 | 56.2 → 56.2 | +0.0 ±15.8 | 1.000 | 8.31 → 8.69 |
| c3 | 2 | engine:no_counters=1 | 25.0 → 25.0 | +0.0 ±12.9 | 1.000 | 10.06 → 9.06 |
| c3 | 2 | engine:no_presence=1 | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 7.81 → 8.88 |
| c3 | 4 | engine | 25.0 → 6.2 | -18.8 ±13.6 | 0.375 | 5.94 → 5.44 |
| c3 | 4 | engine:keep_all=1 | 18.8 → 6.2 | -12.5 ±8.5 | 0.500 | 5.81 → 4.50 |
| c3 | 4 | engine:keep_k=2 | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.88 → 4.62 |
| c3 | 4 | engine:tier1_only=1 | 18.8 → 6.2 | -12.5 ±12.5 | 0.625 | 5.88 → 5.50 |
| c3 | 4 | engine:tier2_rush=1 | 25.0 → 0.0 | -25.0 ±11.2 | 0.125 | 4.75 → 4.75 |
| c3 | 4 | engine:no_powers=1 | 18.8 → 0.0 | -18.8 ±10.1 | 0.250 | 5.81 → 5.50 |
| c3 | 4 | engine:no_counters=1 | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.19 → 5.19 |
| c3 | 4 | engine:no_presence=1 | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.62 → 4.75 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 engine | 6.94 (2.84, 3..12) | deck_empty 1, purge 15 | 4.40 / 24.67 | 4.0 / 2.00 | 24.88 | 0.12 / 2.44 |
| c2 d0 engine:keep_all=1 | 7.06 (2.36, 3..12) | deck_empty 1, purge 15 | 4.27 / 25.20 | 1.0 / 2.00 | 25.38 | 0.00 / 3.06 |
| c2 d0 engine:keep_k=2 | 10.94 (2.25, 3..12) | deck_empty 11, purge 4, supply 1 | 2.50 / 23.25 | 4.6 / 1.83 | 23.38 | 0.56 / 2.56 |
| c2 d0 engine:tier1_only=1 | 7.44 (3.32, 3..12) | deck_empty 3, purge 13 | 4.62 / 25.38 | 2.0 / 2.00 | 25.69 | 0.12 / 2.62 |
| c2 d0 engine:tier2_rush=1 | 5.00 (2.12, 2..11) | purge 16 | 6.00 / 23.88 | – / – | 23.88 | 0.00 / 1.38 |
| c2 d0 engine:no_powers=1 | 7.50 (2.15, 3..12) | deck_empty 1, purge 15 | 3.80 / 24.40 | 2.0 / 2.00 | 24.44 | 0.00 / 2.88 |
| c2 d0 engine:no_counters=1 | 8.12 (2.39, 4..12) | deck_empty 3, purge 13 | 3.77 / 26.08 | 4.7 / 2.00 | 26.06 | 0.31 / 2.56 |
| c2 d0 engine:no_presence=1 | 10.44 (1.80, 7..12) | cities 5, deck_empty 7, supply 4 | – / – | 28.2 / 1.25 | 14.56 | 4.75 / 0.69 |
| c2 d2 engine | 8.00 (1.97, 5..11) | deck_empty 3, purge 10, supply 3 | 2.90 / 23.90 | 11.5 / 1.83 | 18.94 | 0.75 / 1.31 |
| c2 d2 engine:keep_all=1 | 7.31 (1.83, 4..10) | purge 13, supply 3 | 3.08 / 23.08 | 20.0 / 1.67 | 18.75 | 0.69 / 1.25 |
| c2 d2 engine:keep_k=2 | 8.25 (1.85, 6..11) | deck_empty 4, supply 12 | – / – | 15.9 / 1.75 | 5.19 | 2.94 / 0.56 |
| c2 d2 engine:tier1_only=1 | 8.69 (2.20, 5..11) | deck_empty 6, purge 7, supply 3 | 3.00 / 22.00 | 9.2 / 2.22 | 18.06 | 1.31 / 1.25 |
| c2 d2 engine:tier2_rush=1 | 7.31 (3.23, 2..11) | cities 1, deck_empty 5, purge 8, supply 2 | 5.25 / 22.50 | 5.0 / 2.62 | 16.88 | 1.75 / 0.56 |
| c2 d2 engine:no_powers=1 | 8.31 (2.17, 5..11) | deck_empty 5, purge 9, supply 2 | 3.11 / 23.67 | 12.4 / 1.71 | 20.31 | 0.81 / 1.50 |
| c2 d2 engine:no_counters=1 | 10.06 (1.48, 7..11) | deck_empty 11, purge 4, supply 1 | 2.00 / 23.50 | 9.0 / 2.33 | 18.94 | 2.00 / 0.88 |
| c2 d2 engine:no_presence=1 | 7.81 (1.51, 5..11) | cities 4, deck_empty 2, supply 10 | – / – | 38.8 / 0.62 | 6.81 | 4.88 / 0.38 |
| c2 d4 engine | 5.94 (1.75, 3..9) | purge 4, supply 12 | 4.00 / 13.75 | 20.1 / 1.00 | 3.44 | 1.50 / 0.12 |
| c2 d4 engine:keep_all=1 | 5.81 (1.63, 4..10) | purge 3, supply 13 | 2.67 / 18.00 | 18.5 / 1.08 | 3.38 | 1.62 / 0.06 |
| c2 d4 engine:keep_k=2 | 4.88 (0.78, 4..6) | supply 16 | – / – | 20.1 / 1.00 | 0.00 | 2.44 / 0.00 |
| c2 d4 engine:tier1_only=1 | 5.88 (1.80, 3..10) | purge 3, supply 13 | 5.00 / 14.67 | 20.2 / 1.00 | 2.75 | 1.62 / 0.19 |
| c2 d4 engine:tier2_rush=1 | 4.75 (1.15, 2..7) | purge 4, supply 12 | 6.00 / 9.00 | 11.4 / 1.83 | 2.50 | 1.38 / 0.06 |
| c2 d4 engine:no_powers=1 | 5.81 (1.59, 4..9) | purge 3, supply 13 | 4.67 / 11.00 | 24.1 / 0.62 | 2.06 | 1.56 / 0.12 |
| c2 d4 engine:no_counters=1 | 5.19 (1.42, 3..8) | supply 16 | – / – | 22.1 / 0.75 | 0.00 | 1.88 / 0.25 |
| c2 d4 engine:no_presence=1 | 4.62 (0.78, 3..6) | supply 16 | – / – | 23.2 / 0.62 | 0.12 | 2.38 / 0.56 |
| c3 d0 engine | 6.06 (2.01, 4..12) | deck_empty 1, purge 15 | 5.33 / 35.07 | 5.0 / 2.00 | 34.44 | 0.12 / 2.81 |
| c3 d0 engine:keep_all=1 | 10.06 (2.59, 5..12) | deck_empty 10, purge 6 | 4.17 / 37.17 | 2.5 / 2.20 | 39.44 | 0.25 / 4.31 |
| c3 d0 engine:keep_k=2 | 12.00 (0.00, 12..12) | deck_empty 16 | – / – | 11.8 / 1.50 | 36.12 | 0.81 / 2.81 |
| c3 d0 engine:tier1_only=1 | 6.25 (1.98, 4..12) | deck_empty 1, purge 15 | 5.13 / 35.67 | 5.0 / 2.00 | 35.00 | 0.12 / 2.88 |
| c3 d0 engine:tier2_rush=1 | 8.62 (3.10, 3..12) | deck_empty 5, purge 11 | 3.91 / 38.45 | 4.4 / 2.00 | 38.12 | 0.31 / 3.25 |
| c3 d0 engine:no_powers=1 | 6.75 (2.75, 4..12) | deck_empty 3, purge 13 | 5.46 / 35.62 | 3.0 / 2.33 | 35.38 | 0.25 / 3.50 |
| c3 d0 engine:no_counters=1 | 8.44 (2.65, 5..12) | deck_empty 5, purge 11 | 4.18 / 33.91 | 2.0 / 2.60 | 34.38 | 0.38 / 3.62 |
| c3 d0 engine:no_presence=1 | 11.44 (0.86, 9..12) | cities 3, deck_empty 10, supply 3 | – / – | 46.2 / 1.19 | 20.62 | 7.50 / 1.06 |
| c3 d2 engine | 8.81 (2.51, 4..11) | deck_empty 8, purge 8 | 3.38 / 37.38 | 3.5 / 2.62 | 35.88 | 1.31 / 2.00 |
| c3 d2 engine:keep_all=1 | 9.31 (2.34, 5..11) | deck_empty 10, purge 4, supply 2 | 3.25 / 33.25 | 13.2 / 2.17 | 28.62 | 2.12 / 0.62 |
| c3 d2 engine:keep_k=2 | 8.75 (1.95, 6..11) | deck_empty 6, purge 1, supply 9 | 0.00 / 40.00 | 37.3 / 1.00 | 11.56 | 3.69 / 0.50 |
| c3 d2 engine:tier1_only=1 | 9.38 (2.50, 4..11) | deck_empty 11, purge 5 | 4.20 / 37.60 | 3.6 / 2.73 | 34.69 | 1.62 / 2.19 |
| c3 d2 engine:tier2_rush=1 | 8.31 (2.44, 4..11) | deck_empty 5, purge 4, supply 7 | 3.75 / 30.25 | 14.4 / 2.17 | 17.81 | 2.69 / 0.44 |
| c3 d2 engine:no_powers=1 | 8.69 (2.34, 4..11) | deck_empty 7, purge 9 | 3.11 / 34.78 | 5.3 / 2.57 | 31.69 | 1.44 / 1.81 |
| c3 d2 engine:no_counters=1 | 9.06 (2.44, 3..11) | deck_empty 8, purge 4, supply 4 | 3.00 / 36.75 | 10.2 / 2.33 | 22.56 | 1.94 / 1.31 |
| c3 d2 engine:no_presence=1 | 8.88 (1.62, 6..11) | cities 3, deck_empty 4, supply 9 | – / – | 58.2 / 0.88 | 8.44 | 7.00 / 0.75 |
| c3 d4 engine | 5.44 (1.22, 3..8) | purge 1, supply 15 | 4.00 / 23.00 | 29.5 / 0.93 | 1.81 | 2.81 / 0.06 |
| c3 d4 engine:keep_all=1 | 4.50 (0.79, 3..6) | purge 1, supply 15 | 5.00 / 18.00 | 45.7 / 0.00 | 1.75 | 3.06 / 0.00 |
| c3 d4 engine:keep_k=2 | 4.62 (0.78, 3..6) | supply 16 | – / – | 43.9 / 0.12 | 0.00 | 3.31 / 0.00 |
| c3 d4 engine:tier1_only=1 | 5.50 (1.46, 2..8) | purge 1, supply 15 | 4.00 / 23.00 | 29.2 / 0.93 | 1.44 | 2.81 / 0.06 |
| c3 d4 engine:tier2_rush=1 | 4.75 (1.09, 2..7) | supply 16 | – / – | 26.8 / 1.06 | 0.00 | 3.25 / 0.06 |
| c3 d4 engine:no_powers=1 | 5.50 (1.37, 2..7) | supply 16 | – / – | 30.4 / 0.94 | 0.00 | 3.19 / 0.12 |
| c3 d4 engine:no_counters=1 | 5.19 (1.38, 3..8) | supply 16 | – / – | 35.8 / 0.44 | 0.00 | 2.62 / 0.12 |
| c3 d4 engine:no_presence=1 | 4.75 (0.97, 3..7) | supply 16 | – / – | 36.4 / 0.50 | 0.00 | 3.56 / 0.69 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 engine | 373.6 | 17.77 | 3% | – | – | 7.65 (20) | 7.44 | 14.62 | 0.00 | 12.06 / 14.88 (81%) |
| c2 d0 engine:keep_all=1 | 374.1 | 18.96 | 4% | – | – | 7.07 (22) | 3.94 | 15.12 | 0.00 | 12.50 / 15.00 (83%) |
| c2 d0 engine:keep_k=2 | 532.6 | 13.09 | 2% | – | – | 12.06 (61) | 1.75 | 25.06 | 0.00 | 16.94 / 26.75 (63%) |
| c2 d0 engine:tier1_only=1 | 397.4 | 18.32 | 4% | – | – | 7.46 (20) | 7.62 | 15.81 | 0.00 | 12.81 / 15.75 (81%) |
| c2 d0 engine:tier2_rush=1 | 234.6 | 11.42 | 2% | – | – | 8.57 (29) | 1.50 | 8.25 | 0.00 | 7.50 / 10.75 (70%) |
| c2 d0 engine:no_powers=1 | 401.4 | 18.07 | 4% | – | – | 7.40 (19) | 8.25 | 15.38 | 0.00 | 12.88 / 16.19 (80%) |
| c2 d0 engine:no_counters=1 | 460.2 | 18.25 | 3% | – | – | 11.45 (37) | 10.31 | 0.00 | 0.00 | 4.69 / 18.69 (25%) |
| c2 d0 engine:no_presence=1 | 431.5 | 17.13 | 5% | – | – | 21.31 (82) | 13.75 | 9.56 | 0.00 | 9.06 / 28.81 (31%) |
| c2 d2 engine | 461.4 | 18.34 | 2% | – | – | 16.51 (53) | 9.56 | 27.38 | 0.00 | 19.44 / 34.75 (56%) |
| c2 d2 engine:keep_all=1 | 423.3 | 18.56 | 3% | – | – | 16.22 (54) | 2.25 | 27.44 | 0.00 | 19.56 / 31.44 (62%) |
| c2 d2 engine:keep_k=2 | 410.9 | 12.05 | 2% | – | – | 23.01 (67) | 0.69 | 25.44 | 0.00 | 16.00 / 35.69 (45%) |
| c2 d2 engine:tier1_only=1 | 496.2 | 18.52 | 2% | – | – | 17.10 (48) | 10.75 | 29.06 | 0.00 | 20.75 / 37.81 (55%) |
| c2 d2 engine:tier2_rush=1 | 377.2 | 12.99 | 2% | – | – | 19.17 (59) | 7.31 | 23.06 | 0.00 | 17.12 / 32.00 (54%) |
| c2 d2 engine:no_powers=1 | 474.4 | 18.44 | 3% | – | – | 16.01 (53) | 11.06 | 27.62 | 0.00 | 19.62 / 35.38 (55%) |
| c2 d2 engine:no_counters=1 | 596.1 | 18.54 | 2% | – | – | 25.01 (70) | 18.44 | 0.00 | 0.00 | 7.12 / 46.50 (15%) |
| c2 d2 engine:no_presence=1 | 333.6 | 16.45 | 3% | – | – | 27.82 (79) | 10.88 | 12.62 | 0.00 | 10.69 / 36.88 (29%) |
| c2 d4 engine | 313.1 | 15.40 | 2% | – | – | 22.66 (72) | 3.56 | 16.62 | 0.00 | 11.00 / 22.81 (48%) |
| c2 d4 engine:keep_all=1 | 303.5 | 16.31 | 2% | – | – | 21.87 (56) | 1.62 | 17.75 | 0.00 | 10.25 / 23.00 (45%) |
| c2 d4 engine:keep_k=2 | 218.9 | 10.68 | 1% | – | – | 23.18 (58) | 0.00 | 13.06 | 0.00 | 7.44 / 18.31 (41%) |
| c2 d4 engine:tier1_only=1 | 308.2 | 15.14 | 2% | – | – | 22.30 (64) | 3.31 | 15.62 | 0.00 | 10.50 / 22.31 (47%) |
| c2 d4 engine:tier2_rush=1 | 219.3 | 11.67 | 1% | – | – | 22.37 (58) | 3.50 | 11.94 | 0.00 | 7.94 / 18.31 (43%) |
| c2 d4 engine:no_powers=1 | 302.1 | 15.52 | 2% | – | – | 22.49 (72) | 3.56 | 16.12 | 0.00 | 10.38 / 22.00 (47%) |
| c2 d4 engine:no_counters=1 | 272.0 | 14.96 | 1% | – | – | 24.46 (59) | 6.12 | 0.00 | 0.00 | 1.44 / 20.19 (7%) |
| c2 d4 engine:no_presence=1 | 181.2 | 15.10 | 2% | – | – | 22.59 (49) | 2.75 | 9.56 | 0.00 | 5.88 / 18.12 (32%) |
| c3 d0 engine | 482.1 | 20.40 | 3% | – | – | 14.78 (45) | 6.12 | 16.94 | 0.00 | 14.38 / 19.31 (74%) |
| c3 d0 engine:keep_all=1 | 693.0 | 27.87 | 2% | – | – | 13.44 (40) | 12.94 | 28.06 | 0.00 | 25.31 / 31.19 (81%) |
| c3 d0 engine:keep_k=2 | 900.2 | 18.42 | 2% | – | – | 20.50 (73) | 8.81 | 38.38 | 0.00 | 25.50 / 40.00 (64%) |
| c3 d0 engine:tier1_only=1 | 498.8 | 20.51 | 3% | – | – | 14.42 (45) | 6.06 | 17.50 | 0.00 | 14.94 / 19.88 (75%) |
| c3 d0 engine:tier2_rush=1 | 667.4 | 20.15 | 4% | – | – | 17.96 (82) | 15.00 | 25.62 | 0.00 | 19.62 / 29.38 (67%) |
| c3 d0 engine:no_powers=1 | 522.9 | 21.59 | 3% | – | – | 13.93 (45) | 8.62 | 18.31 | 0.00 | 15.88 / 21.12 (75%) |
| c3 d0 engine:no_counters=1 | 684.0 | 21.86 | 4% | – | – | 18.33 (63) | 12.19 | 0.00 | 0.00 | 8.75 / 28.12 (31%) |
| c3 d0 engine:no_presence=1 | 676.2 | 20.51 | 6% | – | – | 31.65 (143) | 20.56 | 12.56 | 0.00 | 14.31 / 40.81 (35%) |
| c3 d2 engine | 736.7 | 22.70 | 4% | – | – | 24.21 (66) | 12.31 | 35.75 | 0.00 | 28.31 / 46.88 (60%) |
| c3 d2 engine:keep_all=1 | 704.4 | 25.43 | 2% | – | – | 28.16 (81) | 7.12 | 39.25 | 0.00 | 28.94 / 48.88 (59%) |
| c3 d2 engine:keep_k=2 | 653.4 | 16.33 | 1% | – | – | 38.39 (117) | 1.38 | 33.50 | 0.00 | 20.62 / 46.69 (44%) |
| c3 d2 engine:tier1_only=1 | 787.6 | 23.01 | 4% | – | – | 24.93 (79) | 14.25 | 36.94 | 0.00 | 29.44 / 49.44 (60%) |
| c3 d2 engine:tier2_rush=1 | 652.5 | 18.72 | 3% | – | – | 33.86 (94) | 14.06 | 32.69 | 0.00 | 21.25 / 44.62 (48%) |
| c3 d2 engine:no_powers=1 | 722.9 | 22.09 | 4% | – | – | 25.79 (73) | 12.12 | 33.38 | 0.00 | 27.12 / 46.81 (58%) |
| c3 d2 engine:no_counters=1 | 753.0 | 20.78 | 4% | – | – | 34.10 (93) | 13.88 | 0.00 | 0.00 | 10.06 / 48.94 (21%) |
| c3 d2 engine:no_presence=1 | 536.9 | 20.16 | 5% | – | – | 39.92 (126) | 13.50 | 15.75 | 0.00 | 15.44 / 47.62 (32%) |
| c3 d4 engine | 404.5 | 19.58 | 2% | – | – | 33.45 (104) | 4.12 | 18.88 | 0.00 | 12.31 / 25.75 (48%) |
| c3 d4 engine:keep_all=1 | 297.8 | 18.54 | 3% | – | – | 31.86 (91) | 0.56 | 16.88 | 0.00 | 9.94 / 20.19 (49%) |
| c3 d4 engine:keep_k=2 | 280.3 | 14.23 | 1% | – | – | 33.46 (78) | 0.00 | 14.56 | 0.00 | 8.75 / 20.81 (42%) |
| c3 d4 engine:tier1_only=1 | 407.4 | 19.61 | 3% | – | – | 33.41 (104) | 4.31 | 19.06 | 0.00 | 12.50 / 26.06 (48%) |
| c3 d4 engine:tier2_rush=1 | 292.9 | 16.41 | 2% | – | – | 31.50 (76) | 5.00 | 14.12 | 0.00 | 8.19 / 21.62 (38%) |
| c3 d4 engine:no_powers=1 | 392.5 | 19.42 | 3% | – | – | 33.83 (104) | 3.44 | 18.81 | 0.00 | 11.94 / 26.00 (46%) |
| c3 d4 engine:no_counters=1 | 368.2 | 18.58 | 2% | – | – | 33.69 (80) | 4.56 | 0.00 | 0.00 | 3.75 / 24.00 (16%) |
| c3 d4 engine:no_presence=1 | 256.4 | 20.64 | 3% | – | – | 31.17 (90) | 3.44 | 11.62 | 0.00 | 8.25 / 20.81 (40%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 8% / 59% / 32% | 8% / 55% / 37% |
| c2 d2 | 17% / 31% / 51% | 15% / 46% / 39% |
| c2 d4 | 23% / 10% / 67% | 29% / 17% / 54% |
| c3 d0 | 8% / 53% / 39% | 9% / 54% / 37% |
| c3 d2 | 23% / 16% / 61% | 14% / 30% / 56% |
| c3 d4 | 19% / 4% / 77% | 44% / 8% / 47% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 engine | 16 of 16 | 0.67 | 15 of 15 | 0.63 |
| c2 d0 engine:keep_all=1 | 16 of 16 | 0.70 | 15 of 15 | 0.68 |
| c2 d0 engine:keep_k=2 | 13 of 16 | 0.44 | 4 of 4 | 0.42 |
| c2 d0 engine:tier1_only=1 | 16 of 16 | 0.69 | 13 of 13 | 0.64 |
| c2 d0 engine:tier2_rush=1 | 16 of 16 | 0.67 | 16 of 16 | 0.67 |
| c2 d0 engine:no_powers=1 | 16 of 16 | 0.59 | 15 of 15 | 0.60 |
| c2 d0 engine:no_counters=1 | 15 of 16 | 0.63 | 12 of 13 | 0.62 |
| c2 d0 engine:no_presence=1 | 16 of 16 | 0.70 | 0 of 0 | – |
| c2 d2 engine | 15 of 16 | 0.67 | 10 of 10 | 0.72 |
| c2 d2 engine:keep_all=1 | 15 of 16 | 0.71 | 13 of 13 | 0.72 |
| c2 d2 engine:keep_k=2 | 10 of 16 | 0.42 | 0 of 0 | – |
| c2 d2 engine:tier1_only=1 | 15 of 16 | 0.67 | 7 of 7 | 0.75 |
| c2 d2 engine:tier2_rush=1 | 15 of 16 | 0.76 | 8 of 8 | 0.63 |
| c2 d2 engine:no_powers=1 | 16 of 16 | 0.60 | 9 of 9 | 0.58 |
| c2 d2 engine:no_counters=1 | 13 of 16 | 0.49 | 4 of 4 | 0.59 |
| c2 d2 engine:no_presence=1 | 16 of 16 | 0.70 | 0 of 0 | – |
| c2 d4 engine | 16 of 16 | 0.62 | 4 of 4 | 0.75 |
| c2 d4 engine:keep_all=1 | 15 of 16 | 0.60 | 3 of 3 | 0.57 |
| c2 d4 engine:keep_k=2 | 7 of 16 | 0.35 | 0 of 0 | – |
| c2 d4 engine:tier1_only=1 | 16 of 16 | 0.63 | 3 of 3 | 0.72 |
| c2 d4 engine:tier2_rush=1 | 15 of 16 | 0.67 | 4 of 4 | 0.60 |
| c2 d4 engine:no_powers=1 | 16 of 16 | 0.57 | 3 of 3 | 0.51 |
| c2 d4 engine:no_counters=1 | 9 of 16 | 0.43 | 0 of 0 | – |
| c2 d4 engine:no_presence=1 | 16 of 16 | 0.60 | 0 of 0 | – |
| c3 d0 engine | 16 of 16 | 0.63 | 15 of 15 | 0.64 |
| c3 d0 engine:keep_all=1 | 16 of 16 | 0.65 | 6 of 6 | 0.61 |
| c3 d0 engine:keep_k=2 | 16 of 16 | 0.56 | 0 of 0 | – |
| c3 d0 engine:tier1_only=1 | 16 of 16 | 0.63 | 15 of 15 | 0.64 |
| c3 d0 engine:tier2_rush=1 | 16 of 16 | 0.76 | 11 of 11 | 0.68 |
| c3 d0 engine:no_powers=1 | 16 of 16 | 0.57 | 13 of 13 | 0.57 |
| c3 d0 engine:no_counters=1 | 16 of 16 | 0.58 | 11 of 11 | 0.52 |
| c3 d0 engine:no_presence=1 | 16 of 16 | 0.69 | 0 of 0 | – |
| c3 d2 engine | 16 of 16 | 0.68 | 8 of 8 | 0.66 |
| c3 d2 engine:keep_all=1 | 16 of 16 | 0.56 | 4 of 4 | 0.66 |
| c3 d2 engine:keep_k=2 | 16 of 16 | 0.57 | 1 of 1 | – |
| c3 d2 engine:tier1_only=1 | 16 of 16 | 0.71 | 5 of 5 | 0.60 |
| c3 d2 engine:tier2_rush=1 | 16 of 16 | 0.70 | 4 of 4 | 0.75 |
| c3 d2 engine:no_powers=1 | 16 of 16 | 0.58 | 9 of 9 | 0.58 |
| c3 d2 engine:no_counters=1 | 15 of 16 | 0.54 | 4 of 4 | 0.40 |
| c3 d2 engine:no_presence=1 | 16 of 16 | 0.65 | 0 of 0 | – |
| c3 d4 engine | 16 of 16 | 0.65 | 1 of 1 | – |
| c3 d4 engine:keep_all=1 | 12 of 16 | 0.46 | 1 of 1 | – |
| c3 d4 engine:keep_k=2 | 11 of 16 | 0.47 | 0 of 0 | – |
| c3 d4 engine:tier1_only=1 | 16 of 16 | 0.65 | 1 of 1 | – |
| c3 d4 engine:tier2_rush=1 | 16 of 16 | 0.69 | 0 of 0 | – |
| c3 d4 engine:no_powers=1 | 16 of 16 | 0.55 | 0 of 0 | – |
| c3 d4 engine:no_counters=1 | 14 of 16 | 0.48 | 0 of 0 | – |
| c3 d4 engine:no_presence=1 | 16 of 16 | 0.62 | 0 of 0 | – |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| engine | 7.31 / 1.62 / 29.1, 100% (100%) | 9.56 / 3.69 / 37.9, 100% (100%) |
| engine:keep_all=1 | 5.69 / 1.62 / 31.1, 100% (100%) | 6.31 / 8.06 / 54.4, 100% (100%) |
| engine:keep_k=2 | 4.62 / 0.00 / 28.2, 100% (100%) | 7.31 / 0.06 / 47.1, – |
| engine:tier1_only=1 | 7.31 / 1.81 / 30.5, 100% (100%) | 9.62 / 3.75 / 38.8, 100% (100%) |
| engine:tier2_rush=1 | 5.00 / 4.00 / 15.2, 100% (100%) | 8.44 / 8.62 / 33.9, 100% (100%) |
| engine:no_powers=1 | 7.38 / 1.94 / 31.6, 100% (100%) | 9.31 / 4.31 / 39.7, 100% (100%) |
| engine:no_counters=1 | 5.88 / 1.75 / 32.6, 100% (100%) | 6.81 / 7.31 / 47.3, 100% (100%) |
| engine:no_presence=1 | 7.62 / 1.00 / 33.8, – | 10.00 / 2.19 / 50.8, – |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| engine | 5.75 / 1.12 / 31.8, 100% (100%) | 7.62 / 7.19 / 50.5, 100% (100%) |
| engine:keep_all=1 | 4.44 / 1.38 / 32.2, 100% (100%) | 4.25 / 7.31 / 50.9, 100% (100%) |
| engine:keep_k=2 | 2.50 / 0.00 / 19.7, – | 4.00 / 0.00 / 32.2, 100% (100%) |
| engine:tier1_only=1 | 5.69 / 1.31 / 33.1, 100% (100%) | 7.56 / 7.94 / 53.5, 100% (100%) |
| engine:tier2_rush=1 | 3.75 / 5.38 / 15.8, 88% (100%) | 5.88 / 7.81 / 28.4, 100% (100%) |
| engine:no_powers=1 | 5.62 / 1.75 / 32.1, 100% (100%) | 7.44 / 7.00 / 49.6, 100% (100%) |
| engine:no_counters=1 | 3.50 / 1.06 / 33.7, 100% (100%) | 4.44 / 7.06 / 48.0, 100% (100%) |
| engine:no_presence=1 | 5.75 / 0.25 / 25.2, – | 8.06 / 1.81 / 41.5, – |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| engine | 4.88 / 0.56 / 22.2, 100% (100%) | 6.19 / 1.44 / 30.4, 100% (100%) |
| engine:keep_all=1 | 3.19 / 0.38 / 23.2, 100% (100%) | 2.12 / 0.12 / 19.4, 100% (100%) |
| engine:keep_k=2 | 2.19 / 0.00 / 13.1, – | 2.62 / 0.00 / 16.6, – |
| engine:tier1_only=1 | 5.00 / 0.44 / 21.6, 100% (100%) | 6.25 / 1.56 / 30.7, 100% (100%) |
| engine:tier2_rush=1 | 3.69 / 2.31 / 12.4, 75% (100%) | 5.44 / 2.44 / 17.1, – |
| engine:no_powers=1 | 4.81 / 0.25 / 21.5, 100% (100%) | 6.12 / 1.25 / 29.5, – |
| engine:no_counters=1 | 3.00 / 0.00 / 16.9, – | 4.19 / 1.19 / 26.2, – |
| engine:no_presence=1 | 5.00 / 0.00 / 15.9, – | 6.94 / 0.38 / 24.6, – |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | engine: pick (offered) / keep / play | engine:keep_all=1: pick (offered) / keep / play | engine:keep_k=2: pick (offered) / keep / play | engine:no_counters=1: pick (offered) / keep / play | engine:no_powers=1: pick (offered) / keep / play | engine:no_presence=1: pick (offered) / keep / play | engine:tier1_only=1: pick (offered) / keep / play | engine:tier2_rush=1: pick (offered) / keep / play |
|---|---|---|---|---|---|---|---|---|
| antimatter_refinery | 1.5 (75) / 39% / 0% | 1.4 (69) / 100% / 0% | 1.2 (4) / 0% / – | 1.7 (98) / 19% / 0% | 1.6 (90) / 36% / 0% | 1.4 (84) / 46% / 0% | 1.6 (84) / 36% / 0% | 1.6 (329) / 10% / 0% |
| assembly_cluster | 1.1 (73) / 158% / 5% | 1.1 (64) / 166% / 0% | 1.2 (280) / 2% / 0% | 1.0 (81) / 151% / 2% | 1.1 (58) / 172% / 7% | 1.1 (85) / 136% / 9% | 1.1 (75) / 155% / 5% | 1.3 (28) / 246% / 12% |
| civic_shelter | 1.4 (48) / 200% / 11% | 1.4 (46) / 204% / 1% | 2.0 (167) / 32% / 4% | 1.4 (51) / 194% / 0% | 1.4 (49) / 198% / 12% | 1.4 (52) / 183% / 13% | 1.4 (48) / 200% / 11% | 2.1 (12) / 500% / 18% |
| contract_bid | 3.5 (378) / 78% / 84% | 3.0 (432) / 108% / 89% | 3.9 (286) / 113% / 90% | 3.6 (421) / 74% / 86% | 3.5 (400) / 80% / 86% | 3.5 (451) / 88% / 92% | 3.5 (399) / 76% / 84% | 3.6 (45) / 167% / 71% |
| counter_intelligence | 3.2 (93) / 63% / 47% | 2.3 (137) / 96% / 53% | 1.8 (4) / 0% / – | 3.1 (113) / 75% / 82% | 3.2 (106) / 60% / 47% | 3.3 (159) / 0% / – | 3.2 (102) / 66% / 51% | 3.5 (346) / 29% / 58% |
| data_lake | 2.1 (65) / 145% / 17% | 2.1 (64) / 147% / 4% | 2.4 (136) / 57% / 23% | 2.1 (74) / 141% / 4% | 2.1 (69) / 142% / 17% | 2.2 (66) / 142% / 28% | 2.1 (64) / 147% / 17% | 2.3 (19) / 253% / 31% |
| data_mining | 3.6 (95) / 102% / 77% | 3.6 (69) / 165% / 78% | 3.7 (61) / 174% / 95% | 3.6 (103) / 95% / 82% | 3.5 (106) / 95% / 78% | 3.4 (132) / 98% / 23% | 3.6 (95) / 102% / 78% | 3.4 (15) / 373% / 71% |
| diaspora | 2.9 (61) / 100% / 80% | 3.0 (66) / 100% / 61% | 3.4 (5) / 20% / 100% | 3.0 (60) / 95% / 82% | 2.9 (60) / 100% / 80% | 2.2 (58) / 84% / 90% | 3.0 (62) / 98% / 80% | 2.5 (100) / 100% / 93% |
| emergency_fab | 2.6 (57) / 193% / 65% | 2.8 (42) / 236% / 43% | 3.0 (97) / 82% / 51% | 2.6 (57) / 198% / 61% | 2.9 (40) / 240% / 66% | 2.7 (51) / 204% / 38% | 2.6 (58) / 191% / 63% | 2.8 (12) / 575% / 67% |
| evacuate | 2.9 (31) / 332% / 94% | 3.0 (35) / 323% / 90% | 3.5 (54) / 183% / 95% | 3.0 (39) / 272% / 88% | 3.2 (23) / 413% / 96% | 2.9 (41) / 271% / 67% | 3.0 (32) / 325% / 92% | 3.2 (8) / 988% / 91% |
| fab_line | 1.5 (62) / 161% / 67% | 1.3 (58) / 167% / 48% | 1.9 (194) / 25% / 20% | 1.4 (58) / 166% / 51% | 1.4 (56) / 170% / 74% | 1.4 (70) / 147% / 69% | 1.4 (62) / 161% / 68% | 2.6 (15) / 360% / 74% |
| field_office | 2.0 (49) / 190% / 27% | 2.0 (47) / 202% / 8% | 2.7 (127) / 57% / 15% | 1.9 (54) / 181% / 3% | 2.0 (48) / 194% / 22% | 2.0 (50) / 188% / 28% | 2.0 (49) / 190% / 26% | 2.6 (11) / 536% / 32% |
| firewall_node | 1.4 (58) / 159% / 7% | 1.4 (57) / 163% / 0% | 2.0 (193) / 21% / 0% | 1.4 (57) / 163% / 0% | 1.4 (60) / 157% / 6% | 1.5 (57) / 163% / 10% | 1.4 (58) / 159% / 7% | 1.9 (16) / 325% / 17% |
| fork_process | 2.8 (66) / 159% / 84% | 2.8 (64) / 161% / 84% | 3.1 (78) / 114% / 85% | 2.9 (62) / 160% / 83% | 2.9 (54) / 172% / 89% | 2.7 (65) / 154% / 0% | 2.8 (67) / 158% / 84% | 3.0 (16) / 344% / 91% |
| fusion_plant | 1.6 (76) / 42% / 0% | 1.7 (66) / 98% / 0% | 1.5 (6) / 0% / – | 1.4 (86) / 38% / 0% | 1.6 (98) / 35% / 0% | 3.3 (166) / 0% / – | 1.5 (84) / 38% / 0% | 2.4 (374) / 4% / 0% |
| global_mandate | 2.6 (70) / 60% / 93% | 2.2 (69) / 100% / 49% | 2.8 (6) / 17% / 0% | 2.6 (72) / 50% / 97% | 2.6 (92) / 59% / 94% | 1.9 (82) / 55% / 89% | 2.6 (76) / 58% / 91% | 2.4 (224) / 31% / 88% |
| grant | 3.6 (112) / 78% / 99% | 3.6 (57) / 168% / 100% | 3.9 (66) / 155% / 100% | 3.7 (105) / 80% / 100% | 3.6 (105) / 79% / 99% | 3.6 (108) / 87% / 100% | 3.6 (122) / 71% / 100% | 3.8 (21) / 186% / 97% |
| grassroots_movement | 2.7 (59) / 166% / 0% | 2.8 (56) / 170% / 0% | 2.9 (106) / 80% / 0% | 2.5 (72) / 154% / 0% | 2.6 (61) / 162% / 0% | 2.8 (55) / 169% / 0% | 2.7 (60) / 165% / 0% | 2.9 (17) / 329% / 0% |
| grid_battery | 2.1 (35) / 277% / 27% | 2.1 (32) / 297% / 5% | 2.3 (175) / 27% / 2% | 2.0 (36) / 275% / 12% | 2.1 (35) / 277% / 27% | 2.1 (36) / 269% / 33% | 2.1 (35) / 277% / 28% | 3.2 (8) / 888% / 34% |
| holding_company | 2.1 (38) / 258% / 20% | 2.1 (37) / 262% / 8% | 2.6 (109) / 70% / 13% | 2.1 (41) / 246% / 4% | 2.2 (35) / 271% / 20% | 2.2 (42) / 243% / 19% | 2.1 (38) / 258% / 20% | 3.2 (11) / 645% / 23% |
| jam_signals | 2.4 (311) / 114% / 90% | 1.9 (362) / 117% / 91% | 3.3 (239) / 101% / 88% | 2.4 (349) / 113% / 93% | 2.3 (324) / 111% / 90% | 2.2 (387) / 97% / 97% | 2.4 (328) / 112% / 91% | 2.7 (38) / 255% / 94% |
| liquidate_reserves | 3.7 (116) / 100% / 35% | 3.1 (87) / 176% / 22% | 3.9 (58) / 203% / 23% | 3.6 (130) / 100% / 38% | 3.5 (122) / 107% / 35% | 3.7 (74) / 142% / 84% | 3.6 (122) / 103% / 33% | 3.7 (15) / 433% / 17% |
| logistics_ai | 1.2 (66) / 150% / 1% | 1.2 (61) / 154% / 0% | 1.6 (246) / 11% / 0% | 1.3 (79) / 138% / 0% | 1.3 (66) / 150% / 1% | 1.2 (72) / 138% / 4% | 1.2 (67) / 149% / 1% | 1.5 (19) / 274% / 2% |
| megastructure | 1.1 (117) / 15% / 0% | 1.2 (76) / 100% / 0% | 1.1 (9) / 0% / – | 1.1 (118) / 13% / 0% | 1.2 (118) / 18% / 0% | 1.1 (148) / 4% / 17% | 1.1 (124) / 16% / 0% | 1.1 (250) / 30% / 5% |
| microgrid | 2.4 (30) / 310% / 81% | 2.4 (31) / 303% / 61% | 2.7 (105) / 66% / 70% | 2.2 (37) / 270% / 59% | 2.4 (31) / 303% / 79% | 2.4 (30) / 310% / 87% | 2.4 (30) / 310% / 82% | 2.2 (6) / 1150% / 90% |
| modular_expansion | – (0) / – / 0.42/game | – (0) / – / 0.11/game | – (0) / – / 0.56/game | – (0) / – / 0.11/game | – (0) / – / 0.44/game | – (0) / – / 0.56/game | – (0) / – / 0.42/game | – (0) / – / 0.39/game |
| node_takedown | 2.7 (103) / 82% / 80% | 2.0 (136) / 99% / 60% | 2.0 (4) / 0% / – | 2.7 (95) / 78% / 84% | 2.7 (113) / 75% / 79% | 3.2 (161) / 0% / – | 2.7 (104) / 80% / 76% | 3.1 (353) / 35% / 82% |
| orbital_lance | 1.3 (121) / 1% / 0% | 1.3 (67) / 100% / 0% | 1.0 (4) / 0% / – | 1.5 (130) / 0% / – | 1.3 (130) / 5% / 0% | 1.2 (167) / 0% / – | 1.4 (132) / 1% / 0% | 1.3 (405) / 1% / 0% |
| overclock_rig | 1.3 (34) / 303% / 0% | 1.3 (27) / 367% / 0% | 1.2 (256) / 1% / 0% | 1.2 (44) / 264% / 0% | 1.3 (30) / 337% / 0% | 1.3 (28) / 354% / 0% | 1.2 (31) / 329% / 0% | 1.2 (6) / 1300% / 0% |
| point_defense | – (0) / – / 0.35/game | – (0) / – / 0.15/game | – (0) / – / 0.78/game | – (0) / – / 0.00/game | – (0) / – / 0.35/game | – (0) / – / 0.39/game | – (0) / – / 0.36/game | – (0) / – / 0.41/game |
| power_share | 3.6 (148) / 60% / 44% | 3.4 (93) / 135% / 29% | 3.8 (90) / 137% / 27% | 3.6 (205) / 50% / 43% | 3.6 (164) / 54% / 43% | 3.5 (128) / 69% / 85% | 3.6 (155) / 59% / 45% | 3.6 (23) / 104% / 21% |
| power_surge | 2.9 (64) / 153% / 95% | 3.1 (43) / 240% / 98% | 3.4 (74) / 128% / 99% | 2.8 (59) / 161% / 98% | 2.9 (61) / 156% / 97% | 2.7 (71) / 137% / 99% | 2.9 (64) / 153% / 96% | 3.0 (14) / 343% / 98% |
| precision_strike | 2.2 (234) / 110% / 77% | 2.1 (169) / 118% / 60% | 3.1 (244) / 85% / 89% | 2.2 (252) / 107% / 82% | 2.2 (230) / 110% / 77% | 2.5 (107) / 115% / 28% | 2.2 (243) / 109% / 78% | 2.9 (38) / 179% / 81% |
| purge_protocol | 2.7 (94) / 67% / 43% | 2.3 (105) / 100% / 41% | 2.3 (10) / 10% / 0% | 2.7 (94) / 67% / 65% | 2.8 (109) / 56% / 46% | 3.4 (168) / 0% / – | 2.7 (106) / 61% / 54% | 3.1 (326) / 29% / 71% |
| quantum_core | 2.2 (99) / 11% / 0% | 2.2 (71) / 97% / 0% | 2.2 (5) / 0% / – | 2.2 (105) / 10% / 0% | 2.2 (108) / 13% / 0% | 3.2 (158) / 0% / – | 2.2 (112) / 10% / 0% | 2.8 (405) / 2% / 0% |
| rally | 3.0 (59) / 180% / 99% | 3.1 (39) / 262% / 100% | 3.5 (66) / 155% / 99% | 2.9 (59) / 173% / 99% | 3.0 (45) / 207% / 100% | 3.0 (63) / 171% / 100% | 3.1 (60) / 177% / 99% | 3.1 (16) / 388% / 100% |
| rally_point | 2.8 (64) / 167% / 0% | 2.9 (58) / 188% / 1% | 3.2 (98) / 94% / 0% | 2.8 (95) / 143% / 0% | 2.7 (67) / 164% / 0% | 2.9 (55) / 178% / 0% | 2.7 (67) / 163% / 0% | 3.0 (18) / 333% / 0% |
| rebuild | 2.0 (67) / 161% / 93% | 1.9 (70) / 160% / 89% | 2.5 (140) / 51% / 94% | 2.0 (64) / 162% / 96% | 2.1 (54) / 176% / 98% | 1.9 (76) / 138% / 41% | 2.0 (70) / 159% / 93% | 2.3 (16) / 362% / 97% |
| recon_sweep | 3.2 (90) / 124% / 0% | 3.1 (72) / 171% / 0% | 3.6 (78) / 138% / 0% | 3.0 (140) / 114% / 0% | 3.1 (93) / 123% / 0% | 3.2 (91) / 115% / 0% | 3.2 (84) / 129% / 0% | 3.3 (22) / 245% / 0% |
| reconstruction | 2.5 (85) / 45% / 58% | 2.0 (76) / 100% / 29% | 2.2 (5) / 20% / 0% | 2.4 (82) / 43% / 71% | 2.4 (79) / 52% / 66% | 1.7 (103) / 31% / 3% | 2.5 (91) / 47% / 65% | 2.1 (286) / 43% / 65% |
| recruitment_hub | 1.4 (45) / 216% / 100% | 1.4 (39) / 254% / 81% | 1.8 (234) / 9% / 100% | 1.3 (54) / 194% / 100% | 1.4 (43) / 221% / 99% | 1.4 (49) / 202% / 100% | 1.4 (47) / 209% / 100% | 1.9 (9) / 700% / 98% |
| scavenge | 3.5 (366) / 51% / 90% | 3.0 (429) / 109% / 88% | 3.9 (280) / 115% / 89% | 3.5 (412) / 54% / 89% | 3.5 (387) / 54% / 87% | 3.4 (442) / 64% / 93% | 3.5 (390) / 52% / 90% | 3.6 (41) / 110% / 84% |
| security_cell | 2.3 (72) / 135% / 0% | 2.3 (57) / 168% / 0% | 2.8 (116) / 69% / 0% | 2.1 (83) / 125% / 0% | 2.3 (76) / 134% / 0% | 2.3 (71) / 131% / 0% | 2.3 (75) / 132% / 0% | 2.4 (22) / 223% / 0% |
| security_patch | 2.4 (93) / 106% / 13% | 2.4 (76) / 159% / 16% | 3.0 (107) / 93% / 19% | 2.4 (126) / 106% / 16% | 2.4 (97) / 106% / 14% | 2.4 (113) / 92% / 27% | 2.4 (95) / 106% / 16% | 3.1 (20) / 160% / 9% |
| seed_funding | 3.0 (99) / 111% / 0% | 3.0 (69) / 187% / 0% | 3.6 (81) / 141% / 0% | 3.0 (137) / 101% / 0% | 3.1 (100) / 109% / 0% | 3.0 (109) / 102% / 0% | 3.0 (101) / 113% / 0% | 3.4 (16) / 250% / 0% |
| server_farm | 1.1 (139) / 63% / 11% | 1.0 (66) / 145% / 5% | 1.2 (292) / 1% / 0% | 1.1 (129) / 72% / 12% | 1.1 (136) / 66% / 9% | 1.1 (134) / 63% / 0% | 1.1 (139) / 62% / 10% | 1.7 (32) / 94% / 7% |
| shore_up | 3.1 (87) / 105% / 67% | 3.1 (53) / 196% / 67% | 3.4 (78) / 112% / 78% | 2.9 (99) / 105% / 74% | 3.0 (87) / 108% / 71% | 2.9 (90) / 104% / 37% | 3.1 (87) / 107% / 68% | 3.4 (17) / 194% / 61% |
| solar_field | 2.3 (49) / 196% / 66% | 2.4 (46) / 204% / 48% | 2.6 (124) / 52% / 41% | 2.3 (55) / 180% / 43% | 2.3 (49) / 196% / 61% | 2.3 (52) / 181% / 65% | 2.3 (49) / 196% / 67% | 2.7 (11) / 527% / 67% |
| strike_team | – (0) / – / 0.96/game | – (0) / – / 0.78/game | – (0) / – / 1.00/game | – (0) / – / 0.96/game | – (0) / – / 0.95/game | – (0) / – / 0.82/game | – (0) / – / 0.96/game | – (0) / – / 0.94/game |
| swarm_override | 2.6 (109) / 17% / 95% | 2.3 (75) / 100% / 19% | 2.2 (6) / 0% / – | 2.9 (113) / 14% / 88% | 2.6 (125) / 14% / 72% | 1.8 (162) / 3% / 80% | 2.7 (124) / 19% / 83% | 2.6 (410) / 6% / 96% |
| sweep | 1.7 (329) / 109% / 92% | 1.4 (274) / 119% / 86% | 2.7 (269) / 57% / 96% | 1.6 (395) / 105% / 97% | 1.6 (348) / 110% / 91% | 1.5 (424) / 90% / 99% | 1.7 (354) / 107% / 93% | 1.8 (45) / 213% / 95% |
| trade_office | 1.3 (66) / 152% / 61% | 1.4 (56) / 186% / 45% | 1.8 (263) / 6% / 0% | 1.4 (66) / 159% / 44% | 1.3 (58) / 164% / 64% | 1.3 (79) / 130% / 50% | 1.3 (69) / 149% / 57% | 1.7 (19) / 316% / 35% |
| turnkey_factory | – (0) / – / 0.98/game | – (0) / – / 0.88/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 0.97/game | – (0) / – / 0.96/game | – (0) / – / 0.98/game | – (0) / – / 0.99/game |
| unity_network | 1.5 (45) / 227% / 6% | 1.6 (29) / 338% / 0% | 1.8 (255) / 3% / 0% | 1.5 (44) / 239% / 0% | 1.5 (40) / 242% / 6% | 1.4 (47) / 206% / 2% | 1.5 (44) / 230% / 6% | 2.1 (9) / 800% / 8% |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.7 (50) / 186% / 3% | 1.7 (42) / 229% / 0% | 2.2 (191) / 23% / 2% | 1.7 (58) / 160% / 0% | 1.7 (51) / 182% / 2% | 1.7 (62) / 144% / 6% | 1.7 (50) / 186% / 3% | 2.5 (10) / 580% / 9% |
| zero_day_wipe | 2.5 (72) / 56% / 0% | 2.6 (81) / 99% / 0% | 2.4 (7) / 0% / – | 2.3 (61) / 67% / 0% | 2.5 (88) / 44% / 0% | 3.3 (167) / 0% / – | 2.5 (75) / 56% / 0% | 3.3 (318) / 14% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 engine | 7173.0 | 19.202 | 0.0 | 7246.1 |
| c2 d0 engine:keep_all=1 | 6196.2 | 16.562 | 0.0 | 6259.6 |
| c2 d0 engine:keep_k=2 | 5801.5 | 10.894 | 0.0 | 5881.7 |
| c2 d0 engine:tier1_only=1 | 7803.2 | 19.634 | 0.0 | 7869.3 |
| c2 d0 engine:tier2_rush=1 | 1905.2 | 8.120 | 0.0 | 1935.1 |
| c2 d0 engine:no_powers=1 | 4966.0 | 12.372 | 0.0 | 5050.1 |
| c2 d0 engine:no_counters=1 | 7016.1 | 15.244 | 0.0 | 7096.5 |
| c2 d0 engine:no_presence=1 | 6441.1 | 14.927 | 0.0 | 6547.2 |
| c2 d2 engine | 7573.8 | 16.416 | 0.0 | 7657.6 |
| c2 d2 engine:keep_all=1 | 5964.4 | 14.090 | 0.0 | 6042.0 |
| c2 d2 engine:keep_k=2 | 3292.5 | 8.012 | 0.0 | 3359.9 |
| c2 d2 engine:tier1_only=1 | 8284.6 | 16.694 | 0.0 | 8384.1 |
| c2 d2 engine:tier2_rush=1 | 3198.0 | 8.477 | 0.0 | 3257.4 |
| c2 d2 engine:no_powers=1 | 5895.2 | 12.427 | 0.0 | 5968.6 |
| c2 d2 engine:no_counters=1 | 9714.8 | 16.297 | 0.0 | 9839.9 |
| c2 d2 engine:no_presence=1 | 3741.8 | 11.216 | 0.0 | 3817.0 |
| c2 d4 engine | 3922.8 | 12.530 | 0.0 | 3974.5 |
| c2 d4 engine:keep_all=1 | 3606.6 | 11.883 | 0.0 | 3684.0 |
| c2 d4 engine:keep_k=2 | 1177.9 | 5.380 | 0.0 | 1208.3 |
| c2 d4 engine:tier1_only=1 | 3646.6 | 11.830 | 0.0 | 3704.3 |
| c2 d4 engine:tier2_rush=1 | 1605.1 | 7.319 | 0.0 | 1633.0 |
| c2 d4 engine:no_powers=1 | 3168.0 | 10.488 | 0.0 | 3224.7 |
| c2 d4 engine:no_counters=1 | 2996.9 | 11.018 | 0.0 | 3052.6 |
| c2 d4 engine:no_presence=1 | 1228.2 | 6.779 | 0.0 | 1255.5 |
| c3 d0 engine | 9306.4 | 19.303 | 0.0 | 9407.9 |
| c3 d0 engine:keep_all=1 | 18888.3 | 27.256 | 0.0 | 19044.0 |
| c3 d0 engine:keep_k=2 | 14599.5 | 16.218 | 0.0 | 14796.3 |
| c3 d0 engine:tier1_only=1 | 9923.0 | 19.893 | 0.0 | 10024.1 |
| c3 d0 engine:tier2_rush=1 | 13307.2 | 19.938 | 0.0 | 13489.8 |
| c3 d0 engine:no_powers=1 | 7630.7 | 14.592 | 0.0 | 7753.2 |
| c3 d0 engine:no_counters=1 | 14448.6 | 21.124 | 0.0 | 14609.4 |
| c3 d0 engine:no_presence=1 | 11393.1 | 16.848 | 0.0 | 11575.2 |
| c3 d2 engine | 15225.2 | 20.667 | 0.0 | 15393.5 |
| c3 d2 engine:keep_all=1 | 14534.0 | 20.634 | 0.0 | 14703.9 |
| c3 d2 engine:keep_k=2 | 9634.6 | 14.744 | 0.0 | 9793.3 |
| c3 d2 engine:tier1_only=1 | 16481.5 | 20.927 | 0.0 | 16681.7 |
| c3 d2 engine:tier2_rush=1 | 10988.8 | 16.841 | 0.0 | 11114.6 |
| c3 d2 engine:no_powers=1 | 12369.3 | 17.111 | 0.0 | 12537.9 |
| c3 d2 engine:no_counters=1 | 15166.8 | 20.142 | 0.0 | 15354.0 |
| c3 d2 engine:no_presence=1 | 8385.3 | 15.617 | 0.0 | 8524.2 |
| c3 d4 engine | 6877.9 | 17.003 | 0.0 | 6953.0 |
| c3 d4 engine:keep_all=1 | 4033.4 | 13.544 | 0.0 | 4100.7 |
| c3 d4 engine:keep_k=2 | 3004.6 | 10.719 | 0.0 | 3048.4 |
| c3 d4 engine:tier1_only=1 | 6567.3 | 16.119 | 0.0 | 6640.8 |
| c3 d4 engine:tier2_rush=1 | 3699.8 | 12.633 | 0.0 | 3748.3 |
| c3 d4 engine:no_powers=1 | 5432.8 | 13.842 | 0.0 | 5507.8 |
| c3 d4 engine:no_counters=1 | 5460.3 | 14.830 | 0.0 | 5523.2 |
| c3 d4 engine:no_presence=1 | 2586.2 | 10.085 | 0.0 | 2642.1 |

Where the games' time went (summed over workers): agents 98.7%, probes 0.0%, apply 0.5%, legal 0.4%, replay check 0.4%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

None matched.
