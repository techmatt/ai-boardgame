# Cycle report: screen-022-cards

022 quick screen (r13): insight costs on cards, plan-based keeping. 2 and 3 countries, d0 to d4, random, greedy and engine: win rates, hand size, kept and played by round, insight on keeps versus plays, unspent credits.

480 games (30 cells × 16 seeds, 0..=15) on 12 threads in 316.6 s. Engine 0.1.0, rules r13, content e3d97d1e972a7982.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates random 0%, greedy 75%, engine 94%; skill gap +94 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) random 8.25, greedy 12.75, engine 12.94; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c2): win rates random 0%, greedy 44%, engine 69%; skill gap +69 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c2): survival (rounds; wins count extra) random 6.81, greedy 11.81, engine 12.69; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c2): win rates random 0%, greedy 31%, engine 62%; skill gap +62 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.75, greedy 10.25, engine 11.06; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c2): win rates random 0%, greedy 25%, engine 44%; skill gap +44 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.06, greedy 8.69, engine 10.62; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c2): win rates random 0%, greedy 12%, engine 25%; skill gap +25 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) random 3.94, greedy 6.19, engine 7.44; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): win rates random 0%, greedy 25%, engine 94%; skill gap +94 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) random 7.88, greedy 12.25, engine 12.94; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c3): win rates random 0%, greedy 19%, engine 62%; skill gap +62 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c3): survival (rounds; wins count extra) random 6.69, greedy 11.69, engine 12.62; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates random 0%, greedy 38%, engine 50%; skill gap +50 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.94, greedy 10.75, engine 11.50; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c3): win rates random 0%, greedy 25%, engine 44%; skill gap +44 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.06, greedy 9.06, engine 10.94; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates random 0%, greedy 12%, engine 6%; skill gap +6 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) random 3.81, greedy 6.38, engine 5.81; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4 (mean over 2 configs/variants): +94 → +16 pts win rate, +4.88 → +2.75 survival (target: widens with difficulty).
- Game length for the strongest agent (engine; configs/baseline.json, c2): 6.9 rounds at difficulty 0, 5.9 at 4 (target 8–10).
- Game length for the strongest agent (engine; configs/baseline.json, c3): 6.1 rounds at difficulty 0, 5.4 at 4 (target 8–10).
- Games ended by: cities 12, deck_empty 90, purge 137, supply 241.
- Adversary operations: 26.6 per round on average across cells, at most 135 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: orbital_lance; over 90%: assembly_cluster, civic_shelter, data_lake, data_mining, diaspora, emergency_fab, evacuate, fab_line, field_office, firewall_node, fork_process, grassroots_movement, grid_battery, holding_company, jam_signals, liquidate_reserves, logistics_ai, microgrid, overclock_rig, power_surge, precision_strike, rally, rally_point, rebuild, recon_sweep, recruitment_hub, security_cell, security_patch, seed_funding, shore_up, solar_field, sweep, trade_office, unity_network, watchtower_net.
- Luck share of survival (seed's share of the variance): 15% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 8.61 gifts per game; 62% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 94% wins) plays 7.31 buildings and 1.62 tier-2 cards per game (of 29.1 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (c2): the top agent there (engine, 69% wins) plays 6.81 buildings and 3.12 tier-2 cards per game (of 38.1 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 2 (c2): the top agent there (engine, 62% wins) plays 5.75 buildings and 1.12 tier-2 cards per game (of 31.8 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 3 (c2): the top agent there (engine, 44% wins) plays 5.75 buildings and 1.56 tier-2 cards per game (of 33.6 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (c2): the top agent there (engine, 25% wins) plays 4.88 buildings and 0.56 tier-2 cards per game (of 22.2 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 25% vs greedy 12% over 16 seeds (only engine 3, only greedy 1), +12.5 ±12.5 pts, sign test p 0.625: no better than greedy (ahead, not significant).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±0.0.
- Variants against the base (c2) at difficulty 1, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -6.2 ±17.0.
- Variants against the base (c2) at difficulty 2, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -12.5 ±18.0.
- Variants against the base (c2) at difficulty 3, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±20.4.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -18.8 ±13.6.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.

| c2 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.25) | 75.0 ±10.8 (12.75) | 93.8 ±6.1 (12.94) | +93.8 pts (+4.69 survival) |

| c2 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.81) | 43.8 ±12.4 (11.81) | 68.8 ±11.6 (12.69) | +68.8 pts (+5.88 survival) |

| c2 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.75) | 31.2 ±11.6 (10.25) | 62.5 ±12.1 (11.06) | +62.5 pts (+5.31 survival) |

| c2 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.06) | 25.0 ±10.8 (8.69) | 43.8 ±12.4 (10.62) | +43.8 pts (+5.56 survival) |

| c2 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.94) | 12.5 ±8.3 (6.19) | 25.0 ±10.8 (7.44) | +25.0 pts (+3.50 survival) |

| c3 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.88) | 25.0 ±10.8 (12.25) | 93.8 ±6.1 (12.94) | +93.8 pts (+5.06 survival) |

| c3 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.69) | 18.8 ±9.8 (11.69) | 62.5 ±12.1 (12.62) | +62.5 pts (+5.94 survival) |

| c3 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.94) | 37.5 ±12.1 (10.75) | 50.0 ±12.5 (11.50) | +50.0 pts (+5.56 survival) |

| c3 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.06) | 25.0 ±10.8 (9.06) | 43.8 ±12.4 (10.94) | +43.8 pts (+5.88 survival) |

| c3 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.81) | 12.5 ±8.3 (6.38) | 6.2 ±6.1 (5.81) | +6.2 pts (+2.00 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 random | 16 | 0.0 | 75.0 | 0 / 0 / 12 / 4 | -75.0 ±11.2 | <0.001 | worse than greedy |
| c2 d0 engine | 16 | 93.8 | 75.0 | 12 / 3 / 0 / 1 | +18.8 ±10.1 | 0.250 | no better than greedy (ahead, not significant) |
| c2 d1 random | 16 | 0.0 | 43.8 | 0 / 0 / 7 / 9 | -43.8 ±12.8 | 0.016 | worse than greedy |
| c2 d1 engine | 16 | 68.8 | 43.8 | 7 / 4 / 0 / 5 | +25.0 ±11.2 | 0.125 | no better than greedy (ahead, not significant) |
| c2 d2 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c2 d2 engine | 16 | 62.5 | 31.2 | 5 / 5 / 0 / 6 | +31.2 ±12.0 | 0.062 | no better than greedy (ahead, not significant) |
| c2 d3 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c2 d3 engine | 16 | 43.8 | 25.0 | 4 / 3 / 0 / 9 | +18.8 ±10.1 | 0.250 | no better than greedy (ahead, not significant) |
| c2 d4 random | 16 | 0.0 | 12.5 | 0 / 0 / 2 / 14 | -12.5 ±8.5 | 0.500 | no better than greedy (behind, not significant) |
| c2 d4 engine | 16 | 25.0 | 12.5 | 1 / 3 / 1 / 11 | +12.5 ±12.5 | 0.625 | no better than greedy (ahead, not significant) |
| c3 d0 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c3 d0 engine | 16 | 93.8 | 25.0 | 4 / 11 / 0 / 1 | +68.8 ±12.0 | <0.001 | beats greedy |
| c3 d1 random | 16 | 0.0 | 18.8 | 0 / 0 / 3 / 13 | -18.8 ±10.1 | 0.250 | no better than greedy (behind, not significant) |
| c3 d1 engine | 16 | 62.5 | 18.8 | 3 / 7 / 0 / 6 | +43.8 ±12.8 | 0.016 | beats greedy |
| c3 d2 random | 16 | 0.0 | 37.5 | 0 / 0 / 6 / 10 | -37.5 ±12.5 | 0.031 | worse than greedy |
| c3 d2 engine | 16 | 50.0 | 37.5 | 4 / 4 / 2 / 6 | +12.5 ±15.5 | 0.688 | no better than greedy (ahead, not significant) |
| c3 d3 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c3 d3 engine | 16 | 43.8 | 25.0 | 3 / 4 / 1 / 8 | +18.8 ±13.6 | 0.375 | no better than greedy (ahead, not significant) |
| c3 d4 random | 16 | 0.0 | 12.5 | 0 / 0 / 2 / 14 | -12.5 ±8.5 | 0.500 | no better than greedy (behind, not significant) |
| c3 d4 engine | 16 | 6.2 | 12.5 | 1 / 0 / 1 / 14 | -6.2 ±6.2 | 1.000 | no better than greedy (behind, not significant) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 8.25 → 7.88 |
| c3 | 0 | greedy | 75.0 → 25.0 | -50.0 ±15.8 | 0.021 | 8.12 → 10.69 |
| c3 | 0 | engine | 93.8 → 93.8 | +0.0 ±0.0 | 1.000 | 6.94 → 6.06 |
| c3 | 1 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.81 → 6.69 |
| c3 | 1 | greedy | 43.8 → 18.8 | -25.0 ±17.1 | 0.289 | 9.00 → 10.12 |
| c3 | 1 | engine | 68.8 → 62.5 | -6.2 ±17.0 | 1.000 | 9.31 → 8.94 |
| c3 | 2 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.75 → 5.94 |
| c3 | 2 | greedy | 31.2 → 37.5 | +6.2 ±17.0 | 1.000 | 8.62 → 8.62 |
| c3 | 2 | engine | 62.5 → 50.0 | -12.5 ±18.0 | 0.727 | 8.00 → 8.81 |
| c3 | 3 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.06 → 5.06 |
| c3 | 3 | greedy | 25.0 → 25.0 | +0.0 ±18.3 | 1.000 | 7.38 → 7.31 |
| c3 | 3 | engine | 43.8 → 43.8 | +0.0 ±20.4 | 1.000 | 8.38 → 9.00 |
| c3 | 4 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 3.94 → 3.81 |
| c3 | 4 | greedy | 12.5 → 12.5 | +0.0 ±12.9 | 1.000 | 5.38 → 5.38 |
| c3 | 4 | engine | 25.0 → 6.2 | -18.8 ±13.6 | 0.375 | 5.94 → 5.44 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 random | 8.25 (1.79, 6..12) | cities 4, deck_empty 2, supply 10 | – / – | 43.5 / 0.25 | 4.38 | 4.31 / 0.44 |
| c2 d0 greedy | 8.12 (2.60, 4..12) | deck_empty 4, purge 12 | 4.17 / 22.50 | 2.5 / 2.50 | 23.12 | 0.25 / 1.75 |
| c2 d0 engine | 6.94 (2.84, 3..12) | deck_empty 1, purge 15 | 4.40 / 24.67 | 4.0 / 2.00 | 24.88 | 0.12 / 2.44 |
| c2 d1 random | 6.81 (1.01, 5..9) | cities 3, supply 13 | – / – | 45.1 / 0.00 | 2.75 | 4.19 / 0.31 |
| c2 d1 greedy | 9.00 (2.74, 5..12) | deck_empty 6, purge 7, supply 3 | 4.43 / 19.86 | 2.9 / 2.67 | 15.44 | 1.00 / 1.19 |
| c2 d1 engine | 9.31 (2.52, 4..12) | deck_empty 5, purge 11 | 2.91 / 24.82 | 2.4 / 2.60 | 23.56 | 0.50 / 2.19 |
| c2 d2 random | 5.75 (0.97, 4..7) | cities 2, supply 14 | – / – | 43.6 / 0.00 | 1.56 | 3.50 / 0.38 |
| c2 d2 greedy | 8.62 (2.18, 5..11) | deck_empty 5, purge 5, supply 6 | 3.20 / 17.20 | 7.9 / 2.45 | 12.56 | 2.00 / 1.06 |
| c2 d2 engine | 8.00 (1.97, 5..11) | deck_empty 3, purge 10, supply 3 | 2.90 / 23.90 | 11.5 / 1.83 | 18.94 | 0.75 / 1.31 |
| c2 d3 random | 5.06 (0.43, 4..6) | supply 16 | – / – | 40.5 / 0.12 | 0.50 | 3.44 / 0.25 |
| c2 d3 greedy | 7.38 (1.54, 6..11) | deck_empty 2, purge 4, supply 10 | 3.25 / 21.50 | 30.2 / 0.92 | 7.06 | 2.06 / 0.38 |
| c2 d3 engine | 8.38 (2.47, 4..11) | deck_empty 5, purge 7, supply 4 | 3.14 / 22.00 | 7.0 / 2.56 | 16.75 | 1.31 / 0.25 |
| c2 d4 random | 3.94 (0.56, 3..5) | supply 16 | – / – | 32.8 / 0.00 | 0.12 | 2.56 / 0.31 |
| c2 d4 greedy | 5.38 (0.86, 4..7) | purge 2, supply 14 | 4.50 / 10.50 | 20.6 / 0.86 | 1.31 | 1.75 / 0.25 |
| c2 d4 engine | 5.94 (1.75, 3..9) | purge 4, supply 12 | 4.00 / 13.75 | 20.1 / 1.00 | 3.44 | 1.50 / 0.12 |
| c3 d0 random | 7.88 (1.32, 6..12) | cities 2, deck_empty 1, supply 13 | – / – | 68.3 / 0.00 | 3.44 | 5.44 / 0.69 |
| c3 d0 greedy | 10.69 (2.49, 5..12) | deck_empty 12, purge 4 | 4.25 / 35.50 | 3.1 / 2.58 | 34.88 | 1.69 / 1.88 |
| c3 d0 engine | 6.06 (2.01, 4..12) | deck_empty 1, purge 15 | 5.33 / 35.07 | 5.0 / 2.00 | 34.44 | 0.12 / 2.81 |
| c3 d1 random | 6.69 (0.92, 5..8) | cities 1, supply 15 | – / – | 66.2 / 0.00 | 1.75 | 5.25 / 0.56 |
| c3 d1 greedy | 10.12 (3.00, 3..12) | deck_empty 11, purge 3, supply 2 | 6.33 / 33.33 | 8.1 / 2.69 | 22.31 | 2.62 / 0.88 |
| c3 d1 engine | 8.94 (2.90, 4..12) | deck_empty 6, purge 10 | 3.90 / 34.90 | 1.7 / 2.83 | 35.25 | 0.56 / 2.94 |
| c3 d2 random | 5.94 (1.48, 4..11) | deck_empty 1, supply 15 | – / – | 63.2 / 0.00 | 3.06 | 4.75 / 0.44 |
| c3 d2 greedy | 8.62 (2.55, 3..11) | deck_empty 7, purge 6, supply 3 | 3.67 / 32.00 | 10.5 / 2.50 | 24.00 | 2.62 / 0.25 |
| c3 d2 engine | 8.81 (2.51, 4..11) | deck_empty 8, purge 8 | 3.38 / 37.38 | 3.5 / 2.62 | 35.88 | 1.31 / 2.00 |
| c3 d3 random | 5.06 (0.56, 4..6) | supply 16 | – / – | 61.4 / 0.00 | 0.75 | 4.25 / 0.31 |
| c3 d3 greedy | 7.31 (2.20, 4..11) | deck_empty 3, purge 4, supply 9 | 5.00 / 23.25 | 32.9 / 1.42 | 9.62 | 2.56 / 0.25 |
| c3 d3 engine | 9.00 (2.18, 5..11) | deck_empty 7, purge 7, supply 2 | 2.43 / 33.14 | 6.2 / 2.78 | 26.38 | 1.62 / 0.50 |
| c3 d4 random | 3.81 (0.73, 2..5) | supply 16 | – / – | 47.2 / 0.00 | 0.62 | 3.69 / 0.38 |
| c3 d4 greedy | 5.38 (1.05, 3..7) | purge 2, supply 14 | 6.00 / 14.50 | 33.6 / 0.71 | 2.31 | 2.69 / 0.12 |
| c3 d4 engine | 5.44 (1.22, 3..8) | purge 1, supply 15 | 4.00 / 23.00 | 29.5 / 0.93 | 1.81 | 2.81 / 0.06 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 random | 374.8 | 8.95 | 2% | – | – | 24.69 (84) | 8.69 | 18.38 | 4.19 | 6.00 / 23.06 (26%) |
| c2 d0 greedy | 403.2 | 21.56 | 5% | – | – | 11.52 (46) | 16.56 | 13.19 | 0.00 | 10.44 / 18.69 (56%) |
| c2 d0 engine | 373.6 | 17.77 | 3% | – | – | 7.65 (20) | 7.44 | 14.62 | 0.00 | 12.06 / 14.88 (81%) |
| c2 d1 random | 300.2 | 9.97 | 2% | – | – | 25.31 (71) | 6.69 | 20.62 | 5.25 | 6.50 / 31.31 (21%) |
| c2 d1 greedy | 489.2 | 22.47 | 4% | – | – | 20.51 (73) | 18.31 | 24.94 | 0.00 | 19.25 / 39.56 (49%) |
| c2 d1 engine | 534.8 | 19.53 | 3% | – | – | 13.38 (45) | 12.94 | 34.62 | 0.00 | 26.88 / 40.62 (66%) |
| c2 d2 random | 257.3 | 9.98 | 2% | – | – | 26.88 (93) | 6.00 | 16.75 | 3.88 | 5.75 / 27.06 (21%) |
| c2 d2 greedy | 473.1 | 21.87 | 4% | – | – | 23.95 (74) | 18.06 | 21.88 | 0.00 | 16.00 / 39.50 (41%) |
| c2 d2 engine | 461.4 | 18.34 | 2% | – | – | 16.51 (53) | 9.56 | 27.38 | 0.00 | 19.44 / 34.75 (56%) |
| c2 d3 random | 222.3 | 10.28 | 2% | – | – | 29.15 (90) | 4.88 | 14.44 | 4.56 | 4.94 / 21.81 (23%) |
| c2 d3 greedy | 386.1 | 19.38 | 3% | – | – | 27.11 (80) | 12.00 | 17.81 | 0.00 | 13.06 / 32.94 (40%) |
| c2 d3 engine | 500.6 | 17.79 | 2% | – | – | 24.10 (91) | 9.31 | 27.00 | 0.00 | 21.06 / 37.38 (56%) |
| c2 d4 random | 146.6 | 10.97 | 2% | – | – | 22.43 (64) | 3.25 | 8.81 | 3.06 | 2.50 / 14.00 (18%) |
| c2 d4 greedy | 257.9 | 16.57 | 2% | – | – | 23.15 (53) | 5.50 | 11.00 | 0.00 | 7.56 / 21.12 (36%) |
| c2 d4 engine | 313.1 | 15.40 | 2% | – | – | 22.66 (72) | 3.56 | 16.62 | 0.00 | 11.00 / 22.81 (48%) |
| c3 d0 random | 530.2 | 12.44 | 2% | – | – | 34.87 (121) | 22.12 | 24.88 | 6.62 | 8.69 / 28.69 (30%) |
| c3 d0 greedy | 748.8 | 26.60 | 4% | – | – | 22.86 (88) | 28.88 | 24.50 | 0.00 | 17.44 / 37.06 (47%) |
| c3 d0 engine | 482.1 | 20.40 | 3% | – | – | 14.78 (45) | 6.12 | 16.94 | 0.00 | 14.38 / 19.31 (74%) |
| c3 d1 random | 463.9 | 13.18 | 1% | – | – | 37.28 (110) | 16.62 | 28.56 | 8.44 | 8.81 / 37.00 (24%) |
| c3 d1 greedy | 728.5 | 26.13 | 4% | – | – | 30.04 (82) | 26.06 | 31.81 | 0.00 | 23.12 / 54.25 (43%) |
| c3 d1 engine | 739.1 | 23.20 | 4% | – | – | 19.48 (58) | 12.25 | 40.00 | 0.00 | 32.50 / 47.50 (68%) |
| c3 d2 random | 404.4 | 12.81 | 1% | – | – | 38.24 (113) | 16.25 | 21.69 | 6.69 | 6.94 / 31.69 (22%) |
| c3 d2 greedy | 645.6 | 24.05 | 4% | – | – | 34.00 (101) | 18.12 | 23.88 | 0.00 | 19.88 / 47.25 (42%) |
| c3 d2 engine | 736.7 | 22.70 | 4% | – | – | 24.21 (66) | 12.31 | 35.75 | 0.00 | 28.31 / 46.88 (60%) |
| c3 d3 random | 316.0 | 13.25 | 1% | – | – | 41.62 (131) | 13.19 | 17.50 | 4.62 | 5.94 / 25.31 (23%) |
| c3 d3 greedy | 545.1 | 22.27 | 4% | – | – | 43.01 (135) | 13.62 | 20.69 | 0.00 | 15.19 / 39.75 (38%) |
| c3 d3 engine | 767.3 | 20.55 | 4% | – | – | 36.92 (118) | 8.50 | 33.06 | 0.00 | 27.69 / 48.62 (57%) |
| c3 d4 random | 218.1 | 14.09 | 1% | – | – | 32.97 (74) | 9.25 | 12.38 | 3.69 | 4.06 / 16.94 (24%) |
| c3 d4 greedy | 358.8 | 20.62 | 3% | – | – | 35.41 (91) | 7.88 | 12.31 | 0.00 | 9.12 / 25.88 (35%) |
| c3 d4 engine | 404.5 | 19.58 | 2% | – | – | 33.45 (104) | 4.12 | 18.88 | 0.00 | 12.31 / 25.75 (48%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 15% / 67% / 18% | 7% / 80% / 13% |
| c2 d1 | 35% / 34% / 31% | 6% / 84% / 10% |
| c2 d2 | 35% / 30% / 34% | 12% / 71% / 18% |
| c2 d3 | 45% / 18% / 37% | 17% / 62% / 20% |
| c2 d4 | 37% / 10% / 54% | 38% / 30% / 32% |
| c3 d0 | 13% / 66% / 21% | 3% / 88% / 8% |
| c3 d1 | 30% / 35% / 36% | 7% / 86% / 7% |
| c3 d2 | 33% / 22% / 45% | 9% / 78% / 13% |
| c3 d3 | 37% / 18% / 45% | 14% / 67% / 19% |
| c3 d4 | 53% / 4% / 43% | 42% / 27% / 32% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 random | 16 of 16 | 0.93 | 0 of 0 | – |
| c2 d0 greedy | 4 of 16 | 0.25 | 4 of 12 | 0.24 |
| c2 d0 engine | 16 of 16 | 0.67 | 15 of 15 | 0.63 |
| c2 d1 random | 13 of 16 | 0.85 | 0 of 0 | – |
| c2 d1 greedy | 5 of 16 | 0.27 | 3 of 7 | 0.20 |
| c2 d1 engine | 16 of 16 | 0.74 | 11 of 11 | 0.71 |
| c2 d2 random | 15 of 16 | 0.92 | 0 of 0 | – |
| c2 d2 greedy | 3 of 16 | 0.25 | 3 of 5 | 0.24 |
| c2 d2 engine | 15 of 16 | 0.67 | 10 of 10 | 0.72 |
| c2 d3 random | 14 of 16 | 0.94 | 0 of 0 | – |
| c2 d3 greedy | 5 of 16 | 0.25 | 2 of 4 | 0.20 |
| c2 d3 engine | 16 of 16 | 0.71 | 7 of 7 | 0.68 |
| c2 d4 random | 15 of 16 | 0.85 | 0 of 0 | – |
| c2 d4 greedy | 5 of 16 | 0.24 | 2 of 2 | 0.40 |
| c2 d4 engine | 16 of 16 | 0.62 | 4 of 4 | 0.75 |
| c3 d0 random | 16 of 16 | 0.93 | 0 of 0 | – |
| c3 d0 greedy | 7 of 16 | 0.22 | 3 of 4 | 0.13 |
| c3 d0 engine | 16 of 16 | 0.63 | 15 of 15 | 0.64 |
| c3 d1 random | 16 of 16 | 0.92 | 0 of 0 | – |
| c3 d1 greedy | 8 of 16 | 0.23 | 2 of 3 | 0.13 |
| c3 d1 engine | 16 of 16 | 0.65 | 10 of 10 | 0.66 |
| c3 d2 random | 16 of 16 | 0.93 | 0 of 0 | – |
| c3 d2 greedy | 7 of 16 | 0.26 | 4 of 6 | 0.29 |
| c3 d2 engine | 16 of 16 | 0.68 | 8 of 8 | 0.66 |
| c3 d3 random | 16 of 16 | 0.95 | 0 of 0 | – |
| c3 d3 greedy | 6 of 16 | 0.22 | 3 of 4 | 0.25 |
| c3 d3 engine | 16 of 16 | 0.69 | 7 of 7 | 0.68 |
| c3 d4 random | 16 of 16 | 0.92 | 0 of 0 | – |
| c3 d4 greedy | 5 of 16 | 0.23 | 2 of 2 | 0.00 |
| c3 d4 engine | 16 of 16 | 0.65 | 1 of 1 | – |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 3.62 / 0.38 / 9.9, – | 4.94 / 1.00 / 13.1, – |
| greedy | 2.62 / 2.00 / 14.0, 100% (100%) | 2.69 / 6.12 / 26.9, 100% (100%) |
| engine | 7.31 / 1.62 / 29.1, 100% (100%) | 9.56 / 3.69 / 37.9, 100% (100%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 3.12 / 0.25 / 8.1, – | 5.19 / 0.62 / 12.9, – |
| greedy | 2.62 / 2.69 / 17.2, 100% (100%) | 2.75 / 5.69 / 26.4, 100% (100%) |
| engine | 6.81 / 3.12 / 38.1, 100% (100%) | 8.31 / 6.50 / 51.4, 100% (100%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 3.19 / 0.25 / 8.1, – | 4.62 / 0.38 / 11.9, – |
| greedy | 2.50 / 2.38 / 17.4, 100% (100%) | 2.56 / 5.75 / 25.8, 100% (100%) |
| engine | 5.75 / 1.12 / 31.8, 100% (100%) | 7.62 / 7.19 / 50.5, 100% (100%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.38 / 0.19 / 6.6, – | 4.06 / 0.12 / 10.3, – |
| greedy | 2.19 / 1.19 / 14.6, 100% (100%) | 2.69 / 3.44 / 21.2, 100% (100%) |
| engine | 5.75 / 1.56 / 33.6, 100% (100%) | 7.00 / 6.69 / 49.2, 100% (100%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 1.88 / 0.00 / 5.1, – | 3.75 / 0.06 / 8.7, – |
| greedy | 2.00 / 0.06 / 10.8, 100% (100%) | 2.31 / 1.00 / 16.2, 100% (100%) |
| engine | 4.88 / 0.56 / 22.2, 100% (100%) | 6.19 / 1.44 / 30.4, 100% (100%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|---|
| antimatter_refinery | 2.5 (251) / 44% / 0% | 2.0 (419) / 0% / – | 1.6 (176) / 32% / 0% |
| assembly_cluster | 2.6 (97) / 99% / 23% | 1.1 (102) / 164% / 0% | 1.1 (125) / 156% / 6% |
| civic_shelter | 2.4 (83) / 120% / 33% | 1.4 (80) / 200% / 0% | 1.4 (82) / 196% / 10% |
| contract_bid | 2.3 (107) / 72% / 39% | 3.8 (168) / 101% / 0% | 3.5 (752) / 76% / 82% |
| counter_intelligence | 2.7 (273) / 48% / 9% | 2.7 (190) / 48% / 0% | 3.1 (197) / 64% / 52% |
| data_lake | 2.6 (109) / 84% / 25% | 2.2 (110) / 145% / 0% | 2.1 (113) / 142% / 17% |
| data_mining | 2.2 (91) / 114% / 22% | 3.7 (204) / 77% / 0% | 3.6 (163) / 101% / 81% |
| diaspora | 2.5 (249) / 50% / 12% | 3.4 (129) / 87% / 46% | 2.9 (113) / 98% / 86% |
| emergency_fab | 2.8 (65) / 132% / 19% | 2.9 (71) / 228% / 0% | 2.6 (102) / 187% / 63% |
| evacuate | 2.6 (57) / 179% / 26% | 3.1 (49) / 318% / 98% | 2.9 (57) / 309% / 95% |
| fab_line | 2.3 (93) / 109% / 25% | 1.4 (100) / 157% / 0% | 1.4 (112) / 155% / 67% |
| field_office | 2.8 (89) / 121% / 29% | 1.9 (91) / 173% / 0% | 2.0 (85) / 185% / 26% |
| firewall_node | 2.6 (97) / 98% / 24% | 1.4 (99) / 161% / 0% | 1.4 (98) / 159% / 4% |
| fork_process | 2.6 (80) / 106% / 33% | 2.9 (95) / 165% / 85% | 2.7 (118) / 155% / 87% |
| fusion_plant | 2.5 (255) / 47% / 0% | 1.0 (428) / 0% / – | 1.6 (165) / 33% / 0% |
| global_mandate | 2.5 (260) / 33% / 2% | 3.3 (258) / 35% / 96% | 2.6 (154) / 55% / 96% |
| grant | 2.6 (84) / 121% / 37% | 3.8 (211) / 70% / 99% | 3.6 (210) / 74% / 98% |
| grassroots_movement | 2.6 (88) / 108% / 37% | 2.7 (98) / 163% / 0% | 2.7 (100) / 165% / 0% |
| grid_battery | 2.7 (61) / 159% / 28% | 2.1 (59) / 271% / 0% | 2.1 (61) / 269% / 25% |
| holding_company | 2.5 (73) / 122% / 29% | 2.3 (62) / 258% / 0% | 2.1 (69) / 245% / 19% |
| jam_signals | 2.5 (83) / 129% / 49% | 3.2 (264) / 124% / 65% | 2.4 (644) / 110% / 91% |
| liquidate_reserves | 2.4 (59) / 163% / 38% | 3.8 (111) / 150% / 0% | 3.6 (219) / 97% / 33% |
| logistics_ai | 2.6 (105) / 84% / 19% | 1.3 (122) / 135% / 0% | 1.3 (113) / 149% / 1% |
| megastructure | 2.6 (256) / 43% / 0% | 1.3 (331) / 16% / 0% | 1.1 (221) / 17% / 0% |
| microgrid | 2.3 (68) / 165% / 23% | 2.3 (56) / 286% / 0% | 2.4 (53) / 298% / 80% |
| modular_expansion | – (0) / – / 0.41/game | – (0) / – / 0.00/game | – (0) / – / 0.39/game |
| node_takedown | 2.6 (243) / 43% / 9% | 2.4 (363) / 59% / 86% | 2.7 (206) / 79% / 81% |
| orbital_lance | 2.6 (263) / 47% / 0% | 1.5 (425) / 0% / – | 1.3 (257) / 0% / 0% |
| overclock_rig | 2.2 (55) / 180% / 26% | 1.4 (57) / 309% / 0% | 1.3 (56) / 305% / 0% |
| point_defense | – (0) / – / 0.36/game | – (0) / – / 0.00/game | – (0) / – / 0.37/game |
| power_share | 2.6 (92) / 70% / 64% | 3.7 (240) / 60% / 0% | 3.6 (295) / 56% / 39% |
| power_surge | 2.7 (79) / 144% / 56% | 2.9 (112) / 138% / 0% | 2.8 (112) / 151% / 96% |
| precision_strike | 2.4 (112) / 75% / 23% | 2.9 (404) / 97% / 78% | 2.1 (466) / 107% / 80% |
| purge_protocol | 2.4 (260) / 42% / 6% | 2.5 (293) / 48% / 69% | 2.7 (184) / 67% / 54% |
| quantum_core | 2.5 (259) / 47% / 0% | 2.1 (389) / 5% / 0% | 2.2 (204) / 14% / 0% |
| rally | 2.6 (62) / 150% / 48% | 3.2 (92) / 168% / 99% | 3.1 (104) / 175% / 99% |
| rally_point | 2.5 (82) / 99% / 4% | 3.0 (96) / 173% / 0% | 2.7 (118) / 158% / 0% |
| rebuild | 2.3 (103) / 83% / 16% | 2.0 (113) / 143% / 89% | 1.9 (119) / 155% / 93% |
| recon_sweep | 2.7 (90) / 109% / 38% | 3.4 (127) / 128% / 0% | 3.2 (165) / 119% / 0% |
| reconstruction | 2.5 (246) / 48% / 2% | 2.9 (321) / 26% / 72% | 2.4 (192) / 45% / 70% |
| recruitment_hub | 2.4 (79) / 123% / 32% | 1.4 (89) / 176% / 100% | 1.4 (80) / 209% / 100% |
| scavenge | 2.6 (94) / 84% / 47% | 3.7 (232) / 67% / 0% | 3.5 (735) / 49% / 88% |
| security_cell | 2.3 (93) / 95% / 26% | 2.5 (130) / 122% / 0% | 2.3 (123) / 134% / 0% |
| security_patch | 2.5 (106) / 92% / 39% | 2.5 (170) / 91% / 0% | 2.4 (176) / 107% / 21% |
| seed_funding | 2.3 (86) / 103% / 3% | 3.4 (170) / 89% / 0% | 3.0 (179) / 109% / 0% |
| server_farm | 2.4 (107) / 89% / 5% | 1.3 (274) / 54% / 55% | 1.1 (235) / 65% / 10% |
| shelter_order | – (0) / – / 0.48/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shore_up | 2.1 (83) / 94% / 41% | 3.2 (157) / 99% / 0% | 3.1 (151) / 105% / 72% |
| solar_field | 2.5 (81) / 105% / 22% | 2.4 (89) / 175% / 0% | 2.3 (85) / 192% / 64% |
| strike_team | – (0) / – / 0.40/game | – (0) / – / 0.86/game | – (0) / – / 0.97/game |
| swarm_override | 2.4 (258) / 43% / 0% | 3.1 (420) / 2% / 90% | 2.8 (232) / 15% / 94% |
| sweep | 2.8 (94) / 115% / 27% | 2.4 (779) / 82% / 97% | 1.6 (670) / 105% / 94% |
| trade_office | 2.2 (88) / 117% / 22% | 1.5 (108) / 145% / 0% | 1.3 (123) / 145% / 61% |
| turnkey_factory | – (0) / – / 0.39/game | – (0) / – / 1.00/game | – (0) / – / 0.97/game |
| unity_network | 2.2 (70) / 127% / 21% | 1.5 (77) / 210% / 0% | 1.5 (79) / 220% / 6% |
| venture_capital | – (0) / – / 0.41/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 2.3 (73) / 123% / 29% | 1.7 (111) / 138% / 0% | 1.7 (90) / 179% / 2% |
| zero_day_wipe | 2.5 (269) / 43% / 4% | 1.6 (298) / 16% / 0% | 2.6 (138) / 54% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 random | 0.0 | 0.000 | 0.0 | 90.0 |
| c2 d0 greedy | 4708.2 | 11.677 | 0.0 | 4805.5 |
| c2 d0 engine | 9262.0 | 24.794 | 0.0 | 9350.7 |
| c2 d1 random | 0.0 | 0.000 | 0.0 | 63.8 |
| c2 d1 greedy | 7706.9 | 15.752 | 0.0 | 7828.7 |
| c2 d1 engine | 14029.5 | 26.233 | 0.0 | 14155.6 |
| c2 d2 random | 0.0 | 0.000 | 0.0 | 50.4 |
| c2 d2 greedy | 6916.0 | 14.620 | 0.0 | 7023.9 |
| c2 d2 engine | 10576.3 | 22.924 | 0.0 | 10702.1 |
| c2 d3 random | 0.0 | 0.000 | 0.0 | 49.8 |
| c2 d3 greedy | 4940.5 | 12.795 | 0.0 | 5061.2 |
| c2 d3 engine | 10918.5 | 21.810 | 0.0 | 11071.1 |
| c2 d4 random | 0.0 | 0.000 | 0.0 | 26.1 |
| c2 d4 greedy | 2756.1 | 10.688 | 0.0 | 2807.5 |
| c2 d4 engine | 5596.0 | 17.875 | 0.0 | 5652.6 |
| c3 d0 random | 0.0 | 0.000 | 0.0 | 159.1 |
| c3 d0 greedy | 13591.9 | 18.151 | 0.0 | 13802.9 |
| c3 d0 engine | 12964.8 | 26.891 | 0.0 | 13105.0 |
| c3 d1 random | 0.0 | 0.000 | 0.0 | 141.0 |
| c3 d1 greedy | 14020.3 | 19.245 | 0.0 | 14240.4 |
| c3 d1 engine | 23615.5 | 31.953 | 0.0 | 23849.2 |
| c3 d2 random | 0.0 | 0.000 | 0.0 | 118.8 |
| c3 d2 greedy | 11743.5 | 18.189 | 0.0 | 11963.5 |
| c3 d2 engine | 21099.2 | 28.641 | 0.0 | 21348.6 |
| c3 d3 random | 0.0 | 0.000 | 0.0 | 91.1 |
| c3 d3 greedy | 9905.7 | 18.171 | 0.0 | 10066.0 |
| c3 d3 engine | 21720.9 | 28.308 | 0.0 | 21966.8 |
| c3 d4 random | 0.0 | 0.000 | 0.0 | 56.8 |
| c3 d4 greedy | 6232.2 | 17.372 | 0.0 | 6352.9 |
| c3 d4 engine | 10105.5 | 24.983 | 0.0 | 10229.4 |

Where the games' time went (summed over workers): agents 98.3%, probes 0.0%, apply 0.6%, legal 0.5%, replay check 0.5%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c13-baseline-c2-d4-greedy-seed6.json`](records/close-loss-c13-baseline-c2-d4-greedy-seed6.json): close loss at difficulty 4: supply in round 6, 3 adversary piece(s) over the level-2 bar, supply 0 of 20, 1 cities corrupted
- [`records/close-loss-c14-baseline-c2-d4-engine-seed14.json`](records/close-loss-c14-baseline-c2-d4-engine-seed14.json): close loss at difficulty 4: supply in round 4, 4 adversary piece(s) over the level-2 bar, supply 0 of 20, 2 cities corrupted

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c28-baseline-c3-d4-greedy-seed1.json`](records/surprising-win-c28-baseline-c3-d4-greedy-seed1.json): won at difficulty 4 by greedy, below the top agent
- [`records/surprising-win-c13-baseline-c2-d4-greedy-seed3.json`](records/surprising-win-c13-baseline-c2-d4-greedy-seed3.json): won at difficulty 4 by greedy, below the top agent

