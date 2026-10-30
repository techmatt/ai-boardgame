# Cycle report: screen-021-cards

021 quick screen (r12): the card primitives and the resource model. 2 and 3 countries, d0 to d4, random, greedy and engine: win rates, the strategy-health gate, hand size and keeps by round, unspent insight and credits, powers, tags sacrificed, building replacements, tokens.

480 games (30 cells × 16 seeds, 0..=15) on 12 threads in 196.9 s. Engine 0.1.0, rules r12, content 6340e1c141d3442e.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates random 0%, greedy 88%, engine 100%; skill gap +100 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) random 8.50, greedy 12.88, engine 13.00; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c2): win rates random 0%, greedy 81%, engine 100%; skill gap +100 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c2): survival (rounds; wins count extra) random 7.19, greedy 12.81, engine 13.00; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c2): win rates random 0%, greedy 88%, engine 94%; skill gap +94 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) random 6.25, greedy 11.88, engine 11.94; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c2): win rates random 0%, greedy 75%, engine 75%; skill gap +75 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.38, greedy 11.25, engine 10.75; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, c2): win rates random 0%, greedy 19%, engine 19%; skill gap +19 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) random 3.94, greedy 6.69, engine 6.50; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, c3): win rates random 0%, greedy 69%, engine 88%; skill gap +88 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) random 8.44, greedy 12.69, engine 12.88; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c3): win rates random 0%, greedy 50%, engine 94%; skill gap +94 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c3): survival (rounds; wins count extra) random 7.62, greedy 12.50, engine 12.94; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates random 0%, greedy 31%, engine 81%; skill gap +81 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) random 6.31, greedy 11.31, engine 11.81; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c3): win rates random 0%, greedy 31%, engine 75%; skill gap +75 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.19, greedy 9.75, engine 11.31; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates random 0%, greedy 6%, engine 31%; skill gap +31 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) random 4.06, greedy 5.88, engine 8.00; the ladder is ordered.
- Skill gap from difficulty 0 to 4 (mean over 2 configs/variants): +94 → +25 pts win rate, +4.47 → +3.25 survival (target: widens with difficulty).
- Game length for the strongest agent (engine; configs/baseline.json, c2): 6.8 rounds at difficulty 0, 5.5 at 4 (target 8–10).
- Game length for the strongest agent (engine; configs/baseline.json, c3): 6.1 rounds at difficulty 0, 6.1 at 4 (target 8–10).
- Games ended by: cities 18, deck_empty 55, purge 206, supply 200, unity 1.
- Adversary operations: 26.5 per round on average across cells, at most 117 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: antimatter_refinery, assembly_cluster, civic_shelter, data_lake, emergency_fab, evacuate, fab_line, field_office, firewall_node, fork_process, fusion_plant, grant, grassroots_movement, grid_battery, holding_company, jam_signals, liquidate_reserves, logistics_ai, megastructure, microgrid, orbital_lance, overclock_rig, power_surge, precision_strike, rally, rally_point, rebuild, recon_sweep, reconstruction, recruitment_hub, security_cell, security_patch, seed_funding, server_farm, shore_up, solar_field, swarm_override, sweep, trade_office, unity_network, watchtower_net.
- Luck share of survival (seed's share of the variance): 12% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 4.47 gifts per game; 62% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 100% wins) plays 10.00 buildings and 1.62 tier-2 cards per game (of 31.4 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (c2): the top agent there (engine, 100% wins) plays 9.19 buildings and 1.38 tier-2 cards per game (of 32.4 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 2 (c2): the top agent there (engine, 94% wins) plays 7.81 buildings and 1.31 tier-2 cards per game (of 31.2 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 3 (c2): the top agent there (engine, 75% wins) plays 7.88 buildings and 1.56 tier-2 cards per game (of 31.3 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (c2): the top agent there (engine, 19% wins) plays 7.06 buildings and 0.19 tier-2 cards per game (of 20.5 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 19% vs greedy 19% over 16 seeds (only engine 3, only greedy 3), +0.0 ±15.8 pts, sign test p 1.000: no better than greedy (level).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -12.5 ±8.5.
- Variants against the base (c2) at difficulty 1, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -6.2 ±6.2.
- Variants against the base (c2) at difficulty 2, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -12.5 ±12.5.
- Variants against the base (c2) at difficulty 3, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±15.8.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +12.5 ±8.5.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.

| c2 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.50) | 87.5 ±8.3 (12.88) | 100.0 ±0.0 (13.00) | +100.0 pts (+4.50 survival) |

| c2 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.19) | 81.2 ±9.8 (12.81) | 100.0 ±0.0 (13.00) | +100.0 pts (+5.81 survival) |

| c2 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.25) | 87.5 ±8.3 (11.88) | 93.8 ±6.1 (11.94) | +93.8 pts (+5.69 survival) |

| c2 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.38) | 75.0 ±10.8 (11.25) | 75.0 ±10.8 (10.75) | +75.0 pts (+5.38 survival) |

| c2 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.94) | 18.8 ±9.8 (6.69) | 18.8 ±9.8 (6.50) | +18.8 pts (+2.56 survival) |

| c3 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.44) | 68.8 ±11.6 (12.69) | 87.5 ±8.3 (12.88) | +87.5 pts (+4.44 survival) |

| c3 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.62) | 50.0 ±12.5 (12.50) | 93.8 ±6.1 (12.94) | +93.8 pts (+5.31 survival) |

| c3 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.31) | 31.2 ±11.6 (11.31) | 81.2 ±9.8 (11.81) | +81.2 pts (+5.50 survival) |

| c3 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.19) | 31.2 ±11.6 (9.75) | 75.0 ±10.8 (11.31) | +75.0 pts (+6.12 survival) |

| c3 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.06) | 6.2 ±6.1 (5.88) | 31.2 ±11.6 (8.00) | +31.2 pts (+3.94 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 random | 16 | 0.0 | 87.5 | 0 / 0 / 14 / 2 | -87.5 ±8.5 | <0.001 | worse than greedy |
| c2 d0 engine | 16 | 100.0 | 87.5 | 14 / 2 / 0 / 0 | +12.5 ±8.5 | 0.500 | no better than greedy (ahead, not significant) |
| c2 d1 random | 16 | 0.0 | 81.2 | 0 / 0 / 13 / 3 | -81.2 ±10.1 | <0.001 | worse than greedy |
| c2 d1 engine | 16 | 100.0 | 81.2 | 13 / 3 / 0 / 0 | +18.8 ±10.1 | 0.250 | no better than greedy (ahead, not significant) |
| c2 d2 random | 16 | 0.0 | 87.5 | 0 / 0 / 14 / 2 | -87.5 ±8.5 | <0.001 | worse than greedy |
| c2 d2 engine | 16 | 93.8 | 87.5 | 13 / 2 / 1 / 0 | +6.2 ±11.1 | 1.000 | no better than greedy (ahead, not significant) |
| c2 d3 random | 16 | 0.0 | 75.0 | 0 / 0 / 12 / 4 | -75.0 ±11.2 | <0.001 | worse than greedy |
| c2 d3 engine | 16 | 75.0 | 75.0 | 10 / 2 / 2 / 2 | +0.0 ±12.9 | 1.000 | no better than greedy (level) |
| c2 d4 random | 16 | 0.0 | 18.8 | 0 / 0 / 3 / 13 | -18.8 ±10.1 | 0.250 | no better than greedy (behind, not significant) |
| c2 d4 engine | 16 | 18.8 | 18.8 | 0 / 3 / 3 / 10 | +0.0 ±15.8 | 1.000 | no better than greedy (level) |
| c3 d0 random | 16 | 0.0 | 68.8 | 0 / 0 / 11 / 5 | -68.8 ±12.0 | <0.001 | worse than greedy |
| c3 d0 engine | 16 | 87.5 | 68.8 | 10 / 4 / 1 / 1 | +18.8 ±13.6 | 0.375 | no better than greedy (ahead, not significant) |
| c3 d1 random | 16 | 0.0 | 50.0 | 0 / 0 / 8 / 8 | -50.0 ±12.9 | 0.008 | worse than greedy |
| c3 d1 engine | 16 | 93.8 | 50.0 | 8 / 7 / 0 / 1 | +43.8 ±12.8 | 0.016 | beats greedy |
| c3 d2 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c3 d2 engine | 16 | 81.2 | 31.2 | 5 / 8 / 0 / 3 | +50.0 ±12.9 | 0.008 | beats greedy |
| c3 d3 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c3 d3 engine | 16 | 75.0 | 31.2 | 4 / 8 / 1 / 3 | +43.8 ±15.7 | 0.039 | beats greedy |
| c3 d4 random | 16 | 0.0 | 6.2 | 0 / 0 / 1 / 15 | -6.2 ±6.2 | 1.000 | no better than greedy (behind, not significant) |
| c3 d4 engine | 16 | 31.2 | 6.2 | 0 / 5 / 1 / 10 | +25.0 ±14.4 | 0.219 | no better than greedy (ahead, not significant) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 8.50 → 8.44 |
| c3 | 0 | greedy | 87.5 → 68.8 | -18.8 ±10.1 | 0.250 | 7.56 → 7.94 |
| c3 | 0 | engine | 100.0 → 87.5 | -12.5 ±8.5 | 0.500 | 6.81 → 6.06 |
| c3 | 1 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 7.19 → 7.62 |
| c3 | 1 | greedy | 81.2 → 50.0 | -31.2 ±15.1 | 0.125 | 7.62 → 9.06 |
| c3 | 1 | engine | 100.0 → 93.8 | -6.2 ±6.2 | 1.000 | 7.00 → 6.56 |
| c3 | 2 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.25 → 6.31 |
| c3 | 2 | greedy | 87.5 → 31.2 | -56.2 ±15.7 | 0.012 | 6.44 → 9.69 |
| c3 | 2 | engine | 93.8 → 81.2 | -12.5 ±12.5 | 0.625 | 6.88 → 7.12 |
| c3 | 3 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.38 → 5.19 |
| c3 | 3 | greedy | 75.0 → 31.2 | -43.8 ±15.7 | 0.039 | 7.38 → 7.69 |
| c3 | 3 | engine | 75.0 → 75.0 | +0.0 ±15.8 | 1.000 | 7.19 → 6.88 |
| c3 | 4 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 3.94 → 4.06 |
| c3 | 4 | greedy | 18.8 → 6.2 | -12.5 ±12.5 | 0.625 | 5.38 → 5.38 |
| c3 | 4 | engine | 18.8 → 31.2 | +12.5 ±8.5 | 0.500 | 5.50 → 6.06 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 random | 8.50 (1.50, 7..12) | cities 3, deck_empty 2, supply 11 | – / – | 41.1 / 0.44 | 4.44 | 4.50 / 0.38 |
| c2 d0 greedy | 7.56 (2.45, 3..12) | deck_empty 2, purge 14 | 4.07 / 24.21 | 1.5 / 3.00 | 24.06 | 0.19 / 1.12 |
| c2 d0 engine | 6.81 (1.42, 3..9) | purge 16 | 4.19 / 24.25 | – / – | 24.25 | 0.00 / 2.50 |
| c2 d1 random | 7.19 (1.70, 5..12) | cities 5, deck_empty 1, supply 10 | – / – | 41.7 / 0.12 | 5.62 | 4.56 / 0.19 |
| c2 d1 greedy | 7.62 (2.52, 3..12) | deck_empty 3, purge 13 | 4.38 / 24.00 | 6.0 / 2.67 | 23.62 | 0.56 / 0.88 |
| c2 d1 engine | 7.00 (1.27, 5..10) | purge 16 | 4.00 / 23.25 | – / – | 23.25 | 0.00 / 1.75 |
| c2 d2 random | 6.25 (0.97, 4..8) | cities 4, supply 12 | – / – | 43.6 / 0.00 | 3.44 | 4.06 / 0.12 |
| c2 d2 greedy | 6.44 (2.03, 3..11) | deck_empty 2, purge 14 | 4.21 / 22.64 | 3.0 / 3.00 | 22.62 | 0.38 / 0.19 |
| c2 d2 engine | 6.88 (1.58, 5..11) | deck_empty 1, purge 15 | 3.40 / 24.80 | 2.0 / 3.00 | 24.75 | 0.12 / 0.88 |
| c2 d3 random | 5.38 (0.78, 4..7) | cities 1, supply 15 | – / – | 41.4 / 0.12 | 0.44 | 3.44 / 0.25 |
| c2 d3 greedy | 7.38 (1.73, 5..11) | deck_empty 2, purge 12, supply 2 | 3.17 / 24.83 | 15.0 / 2.00 | 21.12 | 0.56 / 0.06 |
| c2 d3 engine | 7.19 (1.91, 4..11) | deck_empty 1, purge 12, supply 3 | 2.75 / 23.33 | 9.8 / 2.25 | 19.12 | 0.69 / 0.12 |
| c2 d4 random | 3.94 (0.75, 2..5) | supply 16 | – / – | 30.6 / 0.12 | 0.81 | 2.81 / 0.44 |
| c2 d4 greedy | 5.38 (1.11, 3..8) | purge 3, supply 13 | 5.00 / 16.67 | 24.5 / 0.85 | 3.25 | 1.94 / 0.06 |
| c2 d4 engine | 5.50 (1.32, 4..9) | purge 3, supply 13 | 3.33 / 17.00 | 16.6 / 1.46 | 3.19 | 2.06 / 0.06 |
| c3 d0 random | 8.44 (1.27, 7..12) | cities 3, deck_empty 1, supply 12 | – / – | 60.7 / 0.38 | 4.62 | 6.88 / 0.31 |
| c3 d0 greedy | 7.94 (2.99, 5..12) | deck_empty 5, purge 11 | 4.91 / 34.18 | 6.2 / 2.80 | 33.19 | 0.81 / 1.25 |
| c3 d0 engine | 6.06 (2.73, 3..12) | deck_empty 2, purge 14 | 5.79 / 38.36 | 1.5 / 2.50 | 38.75 | 0.06 / 2.56 |
| c3 d1 random | 7.62 (1.58, 5..12) | cities 1, deck_empty 1, supply 14 | – / – | 71.8 / 0.00 | 1.62 | 6.00 / 0.44 |
| c3 d1 greedy | 9.06 (3.15, 5..12) | deck_empty 8, purge 8 | 4.88 / 35.75 | 3.6 / 3.00 | 33.94 | 1.81 / 0.94 |
| c3 d1 engine | 6.56 (1.97, 4..12) | deck_empty 1, purge 15 | 4.80 / 36.47 | 2.0 / 3.00 | 36.50 | 0.12 / 2.19 |
| c3 d2 random | 6.31 (1.16, 4..9) | cities 1, supply 15 | – / – | 60.4 / 0.25 | 1.44 | 4.94 / 0.50 |
| c3 d2 greedy | 9.69 (2.31, 5..11) | deck_empty 11, purge 5 | 3.20 / 35.80 | 3.5 / 3.00 | 34.62 | 2.44 / 0.75 |
| c3 d2 engine | 7.12 (2.50, 3..11) | deck_empty 3, purge 13 | 3.77 / 34.00 | 2.7 / 3.00 | 33.00 | 0.50 / 0.69 |
| c3 d3 random | 5.19 (0.88, 4..7) | supply 16 | – / – | 62.8 / 0.00 | 0.56 | 4.75 / 0.38 |
| c3 d3 greedy | 7.69 (2.54, 4..11) | deck_empty 5, purge 5, supply 6 | 4.60 / 34.40 | 16.5 / 2.36 | 19.50 | 2.81 / 0.19 |
| c3 d3 engine | 6.88 (2.03, 5..11) | deck_empty 2, purge 11, supply 2, unity 1 | 3.92 / 33.08 | 18.2 / 2.25 | 29.31 | 1.00 / 0.50 |
| c3 d4 random | 4.06 (0.83, 2..5) | supply 16 | – / – | 47.4 / 0.00 | 0.12 | 4.00 / 0.31 |
| c3 d4 greedy | 5.38 (1.32, 3..8) | purge 1, supply 15 | 6.00 / 21.00 | 27.5 / 1.27 | 1.94 | 3.25 / 0.25 |
| c3 d4 engine | 6.06 (2.28, 3..11) | deck_empty 2, purge 5, supply 9 | 4.20 / 22.80 | 30.8 / 1.09 | 9.75 | 2.00 / 0.00 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 random | 378.9 | 8.36 | 2% | – | – | 25.97 (95) | 7.88 | 10.69 | 2.25 | 4.94 / 22.50 (22%) |
| c2 d0 greedy | 365.9 | 21.05 | 4% | – | – | 12.60 (45) | 7.69 | 9.56 | 0.00 | 9.19 / 17.00 (54%) |
| c2 d0 engine | 385.8 | 16.35 | 3% | – | – | 8.64 (24) | 4.75 | 12.06 | 0.00 | 11.00 / 14.50 (76%) |
| c2 d1 random | 334.9 | 9.29 | 2% | – | – | 27.60 (80) | 6.44 | 18.31 | 4.00 | 6.19 / 30.19 (20%) |
| c2 d1 greedy | 380.8 | 20.97 | 4% | – | – | 16.83 (46) | 6.69 | 18.31 | 0.00 | 16.25 / 31.62 (51%) |
| c2 d1 engine | 398.4 | 17.28 | 3% | – | – | 11.74 (34) | 4.12 | 24.19 | 0.00 | 20.56 / 29.38 (70%) |
| c2 d2 random | 280.1 | 9.29 | 2% | – | – | 27.26 (73) | 5.25 | 13.75 | 3.06 | 5.44 / 25.19 (22%) |
| c2 d2 greedy | 336.2 | 18.99 | 3% | – | – | 18.61 (87) | 4.88 | 13.50 | 0.00 | 13.94 / 26.94 (52%) |
| c2 d2 engine | 402.4 | 16.50 | 2% | – | – | 16.52 (46) | 3.69 | 19.69 | 0.00 | 17.44 / 28.62 (61%) |
| c2 d3 random | 235.1 | 9.51 | 2% | – | – | 30.06 (88) | 3.25 | 12.94 | 3.44 | 4.75 / 20.75 (23%) |
| c2 d3 greedy | 392.8 | 18.31 | 3% | – | – | 27.78 (75) | 5.12 | 13.31 | 0.00 | 13.88 / 30.94 (45%) |
| c2 d3 engine | 426.2 | 15.86 | 2% | – | – | 26.19 (68) | 4.06 | 18.44 | 0.00 | 16.31 / 30.12 (54%) |
| c2 d4 random | 165.6 | 10.41 | 2% | – | – | 24.73 (60) | 3.44 | 9.12 | 2.56 | 3.38 / 14.62 (23%) |
| c2 d4 greedy | 260.9 | 16.66 | 2% | – | – | 25.64 (66) | 3.56 | 8.12 | 0.00 | 7.50 / 20.12 (37%) |
| c2 d4 engine | 280.4 | 14.98 | 2% | – | – | 23.36 (55) | 3.12 | 12.25 | 0.00 | 8.75 / 20.06 (44%) |
| c3 d0 random | 586.1 | 11.75 | 2% | – | – | 36.90 (99) | 20.38 | 18.38 | 4.88 | 7.44 / 29.50 (25%) |
| c3 d0 greedy | 504.4 | 27.98 | 3% | – | – | 21.42 (62) | 15.25 | 14.19 | 0.00 | 12.81 / 26.06 (49%) |
| c3 d0 engine | 481.4 | 20.41 | 2% | – | – | 13.97 (40) | 5.25 | 15.56 | 0.00 | 13.94 / 19.00 (73%) |
| c3 d1 random | 529.1 | 12.77 | 2% | – | – | 38.51 (96) | 22.56 | 24.69 | 5.81 | 9.81 / 36.69 (27%) |
| c3 d1 greedy | 584.8 | 29.26 | 2% | – | – | 27.33 (69) | 19.50 | 25.31 | 0.00 | 22.75 / 44.12 (52%) |
| c3 d1 engine | 522.9 | 20.52 | 3% | – | – | 17.40 (53) | 5.50 | 25.06 | 0.00 | 23.12 / 33.31 (69%) |
| c3 d2 random | 426.1 | 12.88 | 2% | – | – | 38.74 (105) | 16.31 | 19.00 | 4.12 | 7.56 / 29.38 (26%) |
| c3 d2 greedy | 636.8 | 29.04 | 2% | – | – | 31.39 (100) | 21.81 | 21.12 | 0.00 | 21.56 / 45.44 (47%) |
| c3 d2 engine | 573.9 | 20.69 | 3% | – | – | 24.72 (63) | 6.00 | 22.12 | 0.00 | 21.62 / 35.00 (62%) |
| c3 d3 random | 338.9 | 12.73 | 2% | – | – | 42.82 (107) | 12.88 | 16.94 | 5.06 | 5.88 / 24.19 (24%) |
| c3 d3 greedy | 513.0 | 25.12 | 2% | – | – | 41.97 (117) | 11.94 | 16.81 | 0.00 | 16.75 / 35.69 (47%) |
| c3 d3 engine | 565.3 | 19.33 | 3% | – | – | 35.19 (84) | 4.06 | 21.88 | 0.00 | 20.44 / 33.75 (61%) |
| c3 d4 random | 246.6 | 13.72 | 1% | – | – | 33.54 (72) | 10.62 | 11.69 | 3.88 | 4.19 / 16.94 (25%) |
| c3 d4 greedy | 350.5 | 20.90 | 3% | – | – | 35.49 (98) | 4.94 | 9.94 | 0.00 | 9.19 / 23.12 (40%) |
| c3 d4 engine | 459.5 | 19.47 | 2% | – | – | 32.96 (76) | 4.12 | 15.62 | 0.00 | 13.88 / 27.31 (51%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 5% / 84% / 10% | 5% / 85% / 10% |
| c2 d1 | 7% / 79% / 14% | 5% / 88% / 8% |
| c2 d2 | 7% / 77% / 16% | 2% / 95% / 3% |
| c2 d3 | 22% / 50% / 28% | 15% / 69% / 17% |
| c2 d4 | 24% / 7% / 69% | 33% / 23% / 44% |
| c3 d0 | 17% / 57% / 27% | 4% / 87% / 9% |
| c3 d1 | 17% / 59% / 25% | 5% / 86% / 9% |
| c3 d2 | 23% / 48% / 29% | 4% / 92% / 5% |
| c3 d3 | 21% / 41% / 38% | 12% / 69% / 19% |
| c3 d4 | 24% / 17% / 60% | 34% / 32% / 34% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 random | 16 of 16 | 0.96 | 0 of 0 | – |
| c2 d0 greedy | 11 of 16 | 0.46 | 9 of 14 | 0.45 |
| c2 d0 engine | 16 of 16 | 0.65 | 16 of 16 | 0.65 |
| c2 d1 random | 16 of 16 | 0.94 | 0 of 0 | – |
| c2 d1 greedy | 14 of 16 | 0.50 | 12 of 13 | 0.49 |
| c2 d1 engine | 16 of 16 | 0.67 | 16 of 16 | 0.67 |
| c2 d2 random | 15 of 16 | 0.97 | 0 of 0 | – |
| c2 d2 greedy | 14 of 16 | 0.50 | 13 of 14 | 0.48 |
| c2 d2 engine | 16 of 16 | 0.62 | 15 of 15 | 0.58 |
| c2 d3 random | 15 of 16 | 0.97 | 0 of 0 | – |
| c2 d3 greedy | 12 of 16 | 0.46 | 9 of 12 | 0.45 |
| c2 d3 engine | 16 of 16 | 0.65 | 12 of 12 | 0.66 |
| c2 d4 random | 16 of 16 | 0.96 | 0 of 0 | – |
| c2 d4 greedy | 13 of 16 | 0.43 | 3 of 3 | 0.40 |
| c2 d4 engine | 16 of 16 | 0.69 | 3 of 3 | 0.81 |
| c3 d0 random | 16 of 16 | 0.94 | 0 of 0 | – |
| c3 d0 greedy | 15 of 16 | 0.44 | 11 of 11 | 0.45 |
| c3 d0 engine | 16 of 16 | 0.62 | 14 of 14 | 0.60 |
| c3 d1 random | 16 of 16 | 0.93 | 0 of 0 | – |
| c3 d1 greedy | 16 of 16 | 0.44 | 8 of 8 | 0.41 |
| c3 d1 engine | 16 of 16 | 0.60 | 15 of 15 | 0.60 |
| c3 d2 random | 16 of 16 | 0.93 | 0 of 0 | – |
| c3 d2 greedy | 16 of 16 | 0.46 | 5 of 5 | 0.47 |
| c3 d2 engine | 16 of 16 | 0.59 | 13 of 13 | 0.59 |
| c3 d3 random | 16 of 16 | 0.92 | 0 of 0 | – |
| c3 d3 greedy | 16 of 16 | 0.46 | 5 of 5 | 0.46 |
| c3 d3 engine | 16 of 16 | 0.59 | 12 of 12 | 0.61 |
| c3 d4 random | 16 of 16 | 0.91 | 0 of 0 | – |
| c3 d4 greedy | 16 of 16 | 0.45 | 1 of 1 | – |
| c3 d4 engine | 16 of 16 | 0.62 | 5 of 5 | 0.65 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 5.56 / 1.12 / 13.2, – | 7.00 / 2.44 / 18.4, – |
| greedy | 3.44 / 2.75 / 16.4, 100% (100%) | 5.00 / 4.12 / 23.9, 100% (100%) |
| engine | 10.00 / 1.62 / 31.4, 100% (100%) | 13.00 / 3.69 / 39.9, 100% (100%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 4.88 / 1.12 / 11.0, – | 6.81 / 1.94 / 16.3, – |
| greedy | 3.50 / 1.94 / 17.9, 100% (100%) | 5.19 / 4.19 / 26.8, 100% (100%) |
| engine | 9.19 / 1.38 / 32.4, 100% (100%) | 11.62 / 4.25 / 41.7, 100% (100%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 4.19 / 0.56 / 9.7, – | 6.25 / 1.38 / 15.7, – |
| greedy | 3.50 / 2.19 / 17.3, 100% (100%) | 5.12 / 4.62 / 29.6, 100% (100%) |
| engine | 7.81 / 1.31 / 31.2, 100% (100%) | 10.12 / 5.62 / 45.1, 100% (100%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 4.44 / 0.44 / 9.4, – | 5.25 / 0.56 / 12.3, – |
| greedy | 3.38 / 1.88 / 19.4, 100% (100%) | 4.88 / 3.69 / 24.9, 100% (100%) |
| engine | 7.88 / 1.56 / 31.3, 100% (100%) | 10.50 / 4.88 / 43.1, 100% (100%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 3.62 / 0.56 / 8.2, – | 5.19 / 0.38 / 10.9, – |
| greedy | 3.19 / 0.12 / 12.7, 100% (100%) | 4.44 / 1.44 / 18.0, 100% (100%) |
| engine | 7.06 / 0.19 / 20.5, 100% (100%) | 9.81 / 3.12 / 35.9, 100% (100%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|---|
| antimatter_refinery | 2.6 (266) / 45% / 1% | 1.9 (123) / 98% / 0% | 1.5 (80) / 100% / 0% |
| assembly_cluster | 2.5 (113) / 93% / 43% | 1.0 (94) / 174% / 0% | 1.0 (130) / 154% / 4% |
| civic_shelter | 2.3 (95) / 101% / 34% | 1.4 (80) / 200% / 0% | 1.4 (89) / 184% / 21% |
| contract_bid | 2.3 (119) / 72% / 33% | 3.7 (199) / 83% / 0% | 3.6 (577) / 84% / 77% |
| counter_intelligence | 2.5 (291) / 47% / 27% | 2.6 (133) / 88% / 0% | 2.7 (152) / 74% / 58% |
| data_lake | 2.5 (119) / 95% / 50% | 2.2 (111) / 145% / 48% | 2.0 (124) / 128% / 41% |
| data_mining | 2.6 (103) / 92% / 31% | 3.5 (181) / 86% / 0% | 3.6 (177) / 88% / 72% |
| diaspora | 2.5 (248) / 46% / 37% | 3.0 (190) / 55% / 48% | 2.8 (103) / 72% / 93% |
| emergency_fab | 2.7 (87) / 109% / 17% | 3.2 (66) / 244% / 0% | 2.6 (167) / 99% / 45% |
| evacuate | 2.6 (64) / 159% / 38% | 2.7 (151) / 101% / 82% | 3.0 (163) / 99% / 77% |
| fab_line | 2.4 (99) / 101% / 44% | 1.4 (92) / 171% / 0% | 1.4 (107) / 151% / 80% |
| field_office | 2.5 (97) / 102% / 48% | 2.0 (77) / 204% / 31% | 1.9 (89) / 179% / 46% |
| firewall_node | 2.6 (97) / 95% / 50% | 1.4 (97) / 162% / 0% | 1.4 (103) / 158% / 13% |
| fork_process | 2.6 (90) / 92% / 54% | 2.6 (91) / 171% / 84% | 2.4 (156) / 117% / 81% |
| fusion_plant | 2.6 (278) / 40% / 0% | 1.1 (136) / 100% / 0% | 1.9 (82) / 91% / 0% |
| global_mandate | 2.7 (265) / 39% / 9% | 2.6 (160) / 75% / 48% | 2.8 (97) / 75% / 78% |
| grant | 2.5 (98) / 93% / 51% | 3.7 (181) / 83% / 95% | 3.7 (167) / 94% / 97% |
| grassroots_movement | 2.6 (104) / 96% / 59% | 3.1 (92) / 171% / 0% | 2.7 (147) / 107% / 0% |
| grid_battery | 2.7 (68) / 157% / 51% | 2.1 (55) / 291% / 0% | 1.9 (64) / 248% / 33% |
| holding_company | 2.2 (85) / 109% / 29% | 2.2 (57) / 275% / 20% | 1.9 (102) / 188% / 48% |
| jam_signals | 2.5 (92) / 118% / 35% | 2.2 (291) / 134% / 65% | 2.2 (497) / 104% / 83% |
| liquidate_reserves | 2.6 (65) / 135% / 35% | 3.7 (196) / 78% / 0% | 3.6 (170) / 91% / 29% |
| logistics_ai | 2.7 (109) / 73% / 40% | 1.2 (107) / 151% / 0% | 1.2 (119) / 146% / 11% |
| megastructure | 2.4 (286) / 39% / 0% | 1.1 (142) / 100% / 0% | 1.1 (100) / 100% / 0% |
| microgrid | 2.3 (75) / 149% / 37% | 2.3 (56) / 288% / 34% | 1.9 (92) / 176% / 88% |
| modular_expansion | – (0) / – / 0.47/game | – (0) / – / 0.00/game | – (0) / – / 0.94/game |
| node_takedown | 2.3 (252) / 46% / 18% | 1.6 (284) / 98% / 75% | 2.0 (171) / 88% / 84% |
| orbital_lance | 2.5 (275) / 44% / 0% | 1.6 (142) / 97% / 0% | 1.2 (77) / 100% / 0% |
| overclock_rig | 2.5 (66) / 180% / 27% | 1.3 (53) / 326% / 0% | 1.2 (62) / 294% / 0% |
| point_defense | – (0) / – / 0.36/game | – (0) / – / 0.00/game | – (0) / – / 0.79/game |
| power_share | 2.6 (109) / 76% / 40% | 3.6 (208) / 78% / 0% | 3.7 (204) / 82% / 29% |
| power_surge | 2.5 (78) / 151% / 45% | 3.2 (59) / 269% / 0% | 2.5 (163) / 96% / 90% |
| precision_strike | 2.5 (126) / 57% / 46% | 2.5 (282) / 118% / 64% | 2.4 (421) / 97% / 74% |
| purge_protocol | 2.6 (278) / 43% / 31% | 2.0 (177) / 95% / 36% | 2.4 (131) / 79% / 50% |
| quantum_core | 2.4 (299) / 43% / 0% | 1.6 (136) / 100% / 0% | 2.1 (80) / 82% / 0% |
| rally | 2.3 (70) / 137% / 51% | 2.8 (158) / 97% / 99% | 3.0 (166) / 99% / 98% |
| rally_point | 2.5 (107) / 86% / 3% | 2.7 (77) / 210% / 0% | 2.5 (115) / 135% / 1% |
| rebuild | 2.5 (103) / 88% / 29% | 2.0 (92) / 176% / 96% | 1.9 (144) / 134% / 97% |
| recon_sweep | 2.6 (88) / 101% / 46% | 2.9 (165) / 93% / 0% | 3.2 (169) / 92% / 0% |
| reconstruction | 2.6 (267) / 42% / 4% | 2.3 (159) / 85% / 32% | 2.2 (102) / 97% / 53% |
| recruitment_hub | 2.5 (78) / 114% / 37% | 1.3 (60) / 267% / 100% | 1.3 (76) / 232% / 100% |
| scavenge | 2.4 (109) / 91% / 53% | 3.5 (204) / 75% / 0% | 3.5 (551) / 85% / 76% |
| security_cell | 2.5 (110) / 81% / 40% | 2.6 (95) / 168% / 0% | 2.5 (130) / 122% / 0% |
| security_patch | 2.2 (119) / 86% / 36% | 2.3 (85) / 188% / 0% | 2.2 (138) / 118% / 0% |
| seed_funding | 2.5 (93) / 105% / 0% | 3.0 (162) / 94% / 0% | 3.1 (168) / 95% / 0% |
| server_farm | 2.5 (123) / 85% / 14% | 1.0 (109) / 146% / 35% | 1.0 (110) / 145% / 8% |
| shelter_order | – (0) / – / 0.39/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shore_up | 2.7 (92) / 100% / 39% | 2.8 (167) / 93% / 0% | 3.0 (156) / 101% / 63% |
| solar_field | 2.7 (86) / 100% / 44% | 2.4 (79) / 201% / 50% | 2.2 (92) / 176% / 76% |
| strike_team | – (0) / – / 0.47/game | – (0) / – / 0.97/game | – (0) / – / 0.99/game |
| swarm_override | 2.4 (264) / 51% / 1% | 2.7 (157) / 79% / 9% | 2.4 (94) / 90% / 25% |
| sweep | 2.3 (99) / 113% / 44% | 1.7 (669) / 113% / 96% | 1.6 (553) / 113% / 92% |
| trade_office | 2.6 (105) / 94% / 31% | 1.4 (78) / 203% / 0% | 1.4 (92) / 187% / 70% |
| turnkey_factory | – (0) / – / 0.41/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| unity_network | 2.3 (73) / 125% / 32% | 1.7 (47) / 345% / 0% | 1.6 (54) / 313% / 7% |
| venture_capital | – (0) / – / 0.42/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 2.4 (90) / 112% / 45% | 1.8 (69) / 230% / 0% | 1.6 (96) / 173% / 19% |
| zero_day_wipe | 2.5 (280) / 38% / 13% | 1.5 (130) / 100% / 0% | 2.4 (96) / 76% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 random | 0.0 | 0.000 | 0.0 | 50.6 |
| c2 d0 greedy | 3230.5 | 8.829 | 0.0 | 3282.1 |
| c2 d0 engine | 5355.3 | 13.881 | 0.0 | 5405.8 |
| c2 d1 random | 0.0 | 0.000 | 0.0 | 45.6 |
| c2 d1 greedy | 4043.6 | 10.620 | 0.0 | 4095.8 |
| c2 d1 engine | 6595.9 | 16.554 | 0.0 | 6656.4 |
| c2 d2 random | 0.0 | 0.000 | 0.0 | 35.7 |
| c2 d2 greedy | 2966.4 | 8.822 | 0.0 | 3015.5 |
| c2 d2 engine | 5918.5 | 14.707 | 0.0 | 5977.1 |
| c2 d3 random | 0.0 | 0.000 | 0.0 | 29.5 |
| c2 d3 greedy | 3346.1 | 8.518 | 0.0 | 3399.0 |
| c2 d3 engine | 5121.5 | 12.015 | 0.0 | 5181.7 |
| c2 d4 random | 0.0 | 0.000 | 0.0 | 19.1 |
| c2 d4 greedy | 1942.5 | 7.444 | 0.0 | 1976.1 |
| c2 d4 engine | 3252.9 | 11.602 | 0.0 | 3299.5 |
| c3 d0 random | 0.1 | 0.000 | 0.0 | 105.6 |
| c3 d0 greedy | 8395.0 | 16.642 | 0.0 | 8485.8 |
| c3 d0 engine | 10374.5 | 21.552 | 0.0 | 10455.7 |
| c3 d1 random | 0.1 | 0.000 | 0.0 | 91.1 |
| c3 d1 greedy | 11366.1 | 19.435 | 0.0 | 11500.7 |
| c3 d1 engine | 9999.1 | 19.121 | 0.0 | 10074.3 |
| c3 d2 random | 0.0 | 0.000 | 0.0 | 73.2 |
| c3 d2 greedy | 10877.8 | 17.083 | 0.0 | 11002.0 |
| c3 d2 engine | 13646.4 | 23.779 | 0.0 | 13752.6 |
| c3 d3 random | 0.0 | 0.000 | 0.0 | 54.7 |
| c3 d3 greedy | 6930.3 | 13.509 | 0.0 | 7021.3 |
| c3 d3 engine | 11282.2 | 19.957 | 0.0 | 11393.6 |
| c3 d4 random | 0.0 | 0.000 | 0.0 | 38.7 |
| c3 d4 greedy | 3881.5 | 11.074 | 0.0 | 3938.7 |
| c3 d4 engine | 8360.2 | 18.194 | 0.0 | 8439.9 |

Where the games' time went (summed over workers): agents 98.6%, probes 0.0%, apply 0.5%, legal 0.4%, replay check 0.5%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c29-baseline-c3-d4-engine-seed4.json`](records/close-loss-c29-baseline-c3-d4-engine-seed4.json): close loss at difficulty 4: deck_empty in round 11, 2 adversary piece(s) over the level-3 bar, supply 24 of 30, 2 cities corrupted
- [`records/close-loss-c13-baseline-c2-d4-greedy-seed0.json`](records/close-loss-c13-baseline-c2-d4-greedy-seed0.json): close loss at difficulty 4: supply in round 8, 2 adversary piece(s) over the level-3 bar, supply 2 of 20, 2 cities corrupted

**early-lock-in**: none matched


**surprising-win**: none matched


