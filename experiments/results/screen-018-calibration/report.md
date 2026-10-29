# Cycle report: screen-018-calibration

018 quick screen: the structural fixes (war-only destruction Discord, presence-owned clean-city factories, 2-energy node removal, the simpler corrupted-city placement and contested claims) and the calibration on top. 2 and 3 countries, d0 to d4, random, greedy and engine: the strategy-health gate (presence, factories, hardware, node ambassadors, Security Patch) and the difficulty curve.

480 games (30 cells × 16 seeds, 0..=15) on 12 threads in 138.6 s. Engine 0.1.0, rules r9, content 428f15e336a3b070.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates random 0%, greedy 69%, engine 100%; skill gap +100 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) random 9.06, greedy 12.69, engine 13.00; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c2): win rates random 0%, greedy 81%, engine 88%; skill gap +88 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c2): survival (rounds; wins count extra) random 7.94, greedy 12.81, engine 12.88; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c2): win rates random 0%, greedy 75%, engine 50%; skill gap +50 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) random 6.56, greedy 11.75, engine 11.19; **the ladder is not ordered on survival**.
- Difficulty 3 (configs/baseline.json, c2): win rates random 0%, greedy 62%, engine 62%; skill gap +62 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.44, greedy 11.44, engine 11.31; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, c2): win rates random 0%, greedy 25%, engine 25%; skill gap +25 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) random 4.19, greedy 7.44, engine 7.56; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): win rates random 0%, greedy 56%, engine 100%; skill gap +100 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) random 9.06, greedy 12.56, engine 13.00; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c3): win rates random 0%, greedy 56%, engine 88%; skill gap +88 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c3): survival (rounds; wins count extra) random 8.38, greedy 12.56, engine 12.88; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates random 0%, greedy 50%, engine 62%; skill gap +62 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) random 7.00, greedy 11.50, engine 11.62; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c3): win rates random 0%, greedy 25%, engine 44%; skill gap +44 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.50, greedy 10.50, engine 11.44; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates random 0%, greedy 12%, engine 38%; skill gap +38 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) random 4.12, greedy 7.75, engine 9.00; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +100 → +31 pts win rate, +3.94 → +4.12 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 4.9 rounds at difficulty 0, 6.4 at 4 (target 8–10).
- Games ended by: cities 28, deck_empty 87, purge 175, supply 178, unity 12.
- Adversary operations: 25.5 per round on average across cells, at most 127 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, fab_line, firewall_node, fork_process, grid_battery, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rebuild, recruitment_hub, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 10% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 7.78 gifts per game; 57% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 100% wins) plays 3.75 buildings and 0.12 tier-2 cards per game (of 14.3 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 1 (c2): the top agent there (engine, 88% wins) plays 3.62 buildings and 0.31 tier-2 cards per game (of 19.3 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 2 (c2): the top agent there (greedy, 75% wins) plays 1.94 buildings and 0.00 tier-2 cards per game (of 13.1 cards); 75% of its wins had 2+ buildings played (75% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 3 (c2): the top agent there (engine, 62% wins) plays 3.12 buildings and 0.12 tier-2 cards per game (of 29.2 cards); 90% of its wins had 2+ buildings played (90% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 4 (c2): the top agent there (engine, 25% wins) plays 2.44 buildings and 0.19 tier-2 cards per game (of 20.8 cards); 75% of its wins had 2+ buildings played (75% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 25% vs greedy 25% over 16 seeds (only engine 3, only greedy 3), +0.0 ±15.8 pts, sign test p 1.000: no better than greedy (level).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±0.0.
- Variants against the base (c2) at difficulty 1, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±12.9.
- Variants against the base (c2) at difficulty 2, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +12.5 ±12.5.
- Variants against the base (c2) at difficulty 3, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -18.8 ±13.6.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +12.5 ±12.5.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| c2 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (9.06) | 68.8 ±11.6 (12.69) | 100.0 ±0.0 (13.00) | +100.0 pts (+3.94 survival) |

| c2 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.94) | 81.2 ±9.8 (12.81) | 87.5 ±8.3 (12.88) | +87.5 pts (+4.94 survival) |

| c2 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.56) | 75.0 ±10.8 (11.75) | 50.0 ±12.5 (11.19) | +50.0 pts (+4.62 survival) |

| c2 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.44) | 62.5 ±12.1 (11.44) | 62.5 ±12.1 (11.31) | +62.5 pts (+5.88 survival) |

| c2 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.19) | 25.0 ±10.8 (7.44) | 25.0 ±10.8 (7.56) | +25.0 pts (+3.38 survival) |

| c3 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (9.06) | 56.2 ±12.4 (12.56) | 100.0 ±0.0 (13.00) | +100.0 pts (+3.94 survival) |

| c3 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.38) | 56.2 ±12.4 (12.56) | 87.5 ±8.3 (12.88) | +87.5 pts (+4.50 survival) |

| c3 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.00) | 50.0 ±12.5 (11.50) | 62.5 ±12.1 (11.62) | +62.5 pts (+4.62 survival) |

| c3 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.50) | 25.0 ±10.8 (10.50) | 43.8 ±12.4 (11.44) | +43.8 pts (+5.94 survival) |

| c3 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.12) | 12.5 ±8.3 (7.75) | 37.5 ±12.1 (9.00) | +37.5 pts (+4.88 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 random | 16 | 0.0 | 68.8 | 0 / 0 / 11 / 5 | -68.8 ±12.0 | <0.001 | worse than greedy |
| c2 d0 engine | 16 | 100.0 | 68.8 | 11 / 5 / 0 / 0 | +31.2 ±12.0 | 0.062 | no better than greedy (ahead, not significant) |
| c2 d1 random | 16 | 0.0 | 81.2 | 0 / 0 / 13 / 3 | -81.2 ±10.1 | <0.001 | worse than greedy |
| c2 d1 engine | 16 | 87.5 | 81.2 | 12 / 2 / 1 / 1 | +6.2 ±11.1 | 1.000 | no better than greedy (ahead, not significant) |
| c2 d2 random | 16 | 0.0 | 75.0 | 0 / 0 / 12 / 4 | -75.0 ±11.2 | <0.001 | worse than greedy |
| c2 d2 engine | 16 | 50.0 | 75.0 | 6 / 2 / 6 / 2 | -25.0 ±17.1 | 0.289 | no better than greedy (behind, not significant) |
| c2 d3 random | 16 | 0.0 | 62.5 | 0 / 0 / 10 / 6 | -62.5 ±12.5 | 0.002 | worse than greedy |
| c2 d3 engine | 16 | 62.5 | 62.5 | 7 / 3 / 3 / 3 | +0.0 ±15.8 | 1.000 | no better than greedy (level) |
| c2 d4 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c2 d4 engine | 16 | 25.0 | 25.0 | 1 / 3 / 3 / 9 | +0.0 ±15.8 | 1.000 | no better than greedy (level) |
| c3 d0 random | 16 | 0.0 | 56.2 | 0 / 0 / 9 / 7 | -56.2 ±12.8 | 0.004 | worse than greedy |
| c3 d0 engine | 16 | 100.0 | 56.2 | 9 / 7 / 0 / 0 | +43.8 ±12.8 | 0.016 | beats greedy |
| c3 d1 random | 16 | 0.0 | 56.2 | 0 / 0 / 9 / 7 | -56.2 ±12.8 | 0.004 | worse than greedy |
| c3 d1 engine | 16 | 87.5 | 56.2 | 8 / 6 / 1 / 1 | +31.2 ±15.1 | 0.125 | no better than greedy (ahead, not significant) |
| c3 d2 random | 16 | 0.0 | 50.0 | 0 / 0 / 8 / 8 | -50.0 ±12.9 | 0.008 | worse than greedy |
| c3 d2 engine | 16 | 62.5 | 50.0 | 5 / 5 / 3 / 3 | +12.5 ±18.0 | 0.727 | no better than greedy (ahead, not significant) |
| c3 d3 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c3 d3 engine | 16 | 43.8 | 25.0 | 2 / 5 / 2 / 7 | +18.8 ±16.4 | 0.453 | no better than greedy (ahead, not significant) |
| c3 d4 random | 16 | 0.0 | 12.5 | 0 / 0 / 2 / 14 | -12.5 ±8.5 | 0.500 | no better than greedy (behind, not significant) |
| c3 d4 engine | 16 | 37.5 | 12.5 | 0 / 6 / 2 / 8 | +25.0 ±17.1 | 0.289 | no better than greedy (ahead, not significant) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 9.06 → 9.06 |
| c3 | 0 | greedy | 68.8 → 56.2 | -12.5 ±20.2 | 0.754 | 7.69 → 9.19 |
| c3 | 0 | engine | 100.0 → 100.0 | +0.0 ±0.0 | 1.000 | 4.88 → 7.75 |
| c3 | 1 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 7.94 → 8.38 |
| c3 | 1 | greedy | 81.2 → 56.2 | -25.0 ±17.1 | 0.289 | 6.69 → 8.94 |
| c3 | 1 | engine | 87.5 → 87.5 | +0.0 ±12.9 | 1.000 | 6.31 → 8.38 |
| c3 | 2 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.56 → 7.00 |
| c3 | 2 | greedy | 75.0 → 50.0 | -25.0 ±17.1 | 0.289 | 6.94 → 8.44 |
| c3 | 2 | engine | 50.0 → 62.5 | +12.5 ±12.5 | 0.625 | 8.50 → 8.50 |
| c3 | 3 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.44 → 5.50 |
| c3 | 3 | greedy | 62.5 → 25.0 | -37.5 ±15.5 | 0.070 | 8.56 → 9.31 |
| c3 | 3 | engine | 62.5 → 43.8 | -18.8 ±13.6 | 0.375 | 7.88 → 9.38 |
| c3 | 4 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.19 → 4.12 |
| c3 | 4 | greedy | 25.0 → 12.5 | -12.5 ±15.5 | 0.688 | 6.12 → 7.12 |
| c3 | 4 | engine | 25.0 → 37.5 | +12.5 ±12.5 | 0.625 | 6.38 → 7.19 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 random | 9.06 (1.89, 6..12) | cities 7, deck_empty 2, supply 7 | – / – | 35.3 / 0.06 | 9.00 | 5.00 / 0.38 |
| c2 d0 greedy | 7.69 (3.44, 4..12) | deck_empty 5, purge 11 | 5.27 / 24.82 | 11.0 / 1.20 | 23.31 | 1.12 / 1.25 |
| c2 d0 engine | 4.88 (1.17, 4..8) | purge 16 | 6.12 / 24.88 | – / – | 24.88 | 0.00 / 3.06 |
| c2 d1 random | 7.94 (1.64, 5..12) | cities 6, deck_empty 1, supply 9 | – / – | 39.6 / 0.06 | 5.12 | 4.94 / 0.25 |
| c2 d1 greedy | 6.69 (2.82, 4..12) | deck_empty 3, purge 13 | 5.54 / 23.46 | 20.7 / 1.00 | 21.88 | 0.81 / 1.06 |
| c2 d1 engine | 6.31 (2.39, 4..12) | deck_empty 2, purge 14 | 5.50 / 23.93 | 7.5 / 2.00 | 23.62 | 0.50 / 2.44 |
| c2 d2 random | 6.56 (0.86, 5..8) | cities 4, supply 12 | – / – | 38.5 / 0.00 | 4.06 | 4.69 / 0.19 |
| c2 d2 greedy | 6.94 (2.54, 4..11) | deck_empty 4, purge 12 | 4.42 / 23.17 | 12.5 / 1.75 | 22.06 | 0.81 / 0.62 |
| c2 d2 engine | 8.50 (2.52, 4..11) | deck_empty 7, purge 7, supply 1, unity 1 | 3.38 / 25.38 | 7.4 / 1.75 | 22.38 | 1.56 / 2.12 |
| c2 d3 random | 5.44 (0.61, 5..7) | cities 1, supply 15 | – / – | 38.9 / 0.00 | 1.25 | 3.62 / 0.56 |
| c2 d3 greedy | 8.56 (2.29, 4..11) | deck_empty 5, purge 7, supply 1, unity 3 | 2.60 / 20.80 | 16.0 / 1.83 | 18.50 | 1.81 / 0.81 |
| c2 d3 engine | 7.88 (2.50, 4..11) | deck_empty 5, purge 8, supply 1, unity 2 | 3.50 / 22.60 | 9.8 / 1.83 | 21.75 | 1.19 / 1.00 |
| c2 d4 random | 4.19 (0.53, 3..5) | supply 16 | – / – | 26.6 / 0.06 | 0.19 | 3.12 / 0.25 |
| c2 d4 greedy | 6.12 (1.80, 3..10) | purge 2, supply 12, unity 2 | 3.25 / 14.50 | 11.2 / 1.08 | 4.12 | 2.06 / 0.44 |
| c2 d4 engine | 6.38 (2.20, 3..11) | deck_empty 1, purge 3, supply 11, unity 1 | 2.75 / 12.50 | 7.8 / 1.33 | 3.88 | 1.94 / 0.56 |
| c3 d0 random | 9.06 (1.39, 7..12) | cities 2, deck_empty 1, supply 13 | – / – | 59.8 / 0.00 | 5.88 | 6.50 / 0.94 |
| c3 d0 greedy | 9.19 (3.09, 4..12) | deck_empty 7, purge 9 | 4.00 / 35.44 | 6.1 / 1.86 | 35.75 | 1.00 / 2.56 |
| c3 d0 engine | 7.75 (1.79, 5..11) | purge 16 | 3.25 / 38.25 | – / – | 38.25 | 0.00 / 7.25 |
| c3 d1 random | 8.38 (1.45, 6..12) | cities 4, deck_empty 1, supply 11 | – / – | 60.9 / 0.00 | 6.56 | 7.12 / 0.56 |
| c3 d1 greedy | 8.94 (3.05, 4..12) | deck_empty 7, purge 9 | 4.44 / 35.22 | 10.7 / 1.86 | 34.31 | 1.25 / 2.31 |
| c3 d1 engine | 8.38 (2.18, 4..12) | deck_empty 2, purge 14 | 3.14 / 37.86 | 5.5 / 1.50 | 37.88 | 0.38 / 5.75 |
| c3 d2 random | 7.00 (0.79, 6..8) | cities 4, supply 12 | – / – | 54.0 / 0.06 | 5.38 | 6.56 / 0.56 |
| c3 d2 greedy | 8.44 (2.71, 5..11) | deck_empty 8, purge 8 | 4.12 / 32.75 | 6.6 / 1.88 | 31.88 | 1.56 / 1.25 |
| c3 d2 engine | 8.50 (2.18, 5..11) | deck_empty 6, purge 10 | 3.00 / 35.60 | 2.8 / 2.00 | 35.69 | 0.69 / 5.44 |
| c3 d3 random | 5.50 (1.00, 4..8) | supply 16 | – / – | 56.7 / 0.00 | 0.44 | 4.06 / 0.44 |
| c3 d3 greedy | 9.31 (1.89, 5..11) | deck_empty 7, purge 3, supply 5, unity 1 | 2.75 / 33.25 | 12.2 / 1.92 | 21.44 | 2.81 / 1.06 |
| c3 d3 engine | 9.38 (1.96, 5..11) | deck_empty 9, purge 7 | 2.71 / 34.71 | 7.0 / 2.00 | 33.50 | 1.56 / 3.12 |
| c3 d4 random | 4.12 (0.70, 3..5) | supply 16 | – / – | 42.1 / 0.00 | 0.62 | 3.88 / 0.44 |
| c3 d4 greedy | 7.12 (2.09, 3..11) | deck_empty 2, purge 2, supply 12 | 3.00 / 20.50 | 10.9 / 1.50 | 5.44 | 2.50 / 1.00 |
| c3 d4 engine | 7.19 (2.53, 3..11) | deck_empty 2, purge 4, supply 8, unity 2 | 2.83 / 25.00 | 11.8 / 1.30 | 12.38 | 2.00 / 2.12 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 random | 407.2 | 10.13 | 2% | – | – | 26.57 (78) | 10.44 | 13.81 | 3.44 | 6.19 / 25.38 (24%) |
| c2 d0 greedy | 266.2 | 23.54 | 3% | – | – | 12.47 (51) | 11.94 | 9.06 | 0.00 | 6.81 / 17.69 (39%) |
| c2 d0 engine | 188.2 | 16.66 | 3% | – | – | 7.19 (18) | 2.62 | 8.50 | 0.00 | 7.06 / 10.38 (68%) |
| c2 d1 random | 370.9 | 11.32 | 2% | – | – | 30.98 (96) | 8.25 | 23.81 | 6.44 | 7.69 / 34.56 (22%) |
| c2 d1 greedy | 252.6 | 20.31 | 3% | – | – | 16.69 (62) | 5.88 | 14.81 | 0.00 | 11.62 / 28.56 (41%) |
| c2 d1 engine | 257.6 | 18.82 | 3% | – | – | 11.99 (43) | 4.00 | 19.31 | 0.00 | 16.81 / 27.00 (62%) |
| c2 d2 random | 271.1 | 10.92 | 2% | – | – | 28.79 (84) | 6.12 | 16.62 | 4.44 | 5.38 / 26.44 (20%) |
| c2 d2 greedy | 266.9 | 20.02 | 3% | – | – | 18.89 (60) | 7.19 | 12.50 | 0.00 | 10.62 / 29.38 (36%) |
| c2 d2 engine | 370.1 | 22.14 | 2% | – | – | 16.90 (53) | 9.00 | 18.94 | 0.00 | 18.88 / 34.88 (54%) |
| c2 d3 random | 219.8 | 10.84 | 2% | – | – | 29.45 (80) | 7.88 | 16.38 | 4.31 | 5.19 / 21.31 (24%) |
| c2 d3 greedy | 345.8 | 20.68 | 3% | – | – | 28.96 (91) | 10.06 | 12.88 | 0.00 | 11.25 / 36.75 (31%) |
| c2 d3 engine | 358.2 | 20.30 | 2% | – | – | 23.18 (98) | 6.62 | 17.06 | 0.00 | 16.62 / 32.94 (50%) |
| c2 d4 random | 144.4 | 11.33 | 2% | – | – | 22.94 (52) | 3.88 | 10.12 | 2.12 | 3.38 / 14.81 (23%) |
| c2 d4 greedy | 235.8 | 18.09 | 3% | – | – | 25.04 (66) | 4.50 | 7.69 | 0.00 | 6.62 / 24.25 (27%) |
| c2 d4 engine | 262.5 | 18.28 | 2% | – | – | 22.69 (61) | 4.06 | 11.06 | 0.00 | 10.44 / 24.62 (42%) |
| c3 d0 random | 603.2 | 15.04 | 2% | – | – | 37.06 (95) | 31.12 | 24.31 | 5.81 | 9.50 / 32.06 (30%) |
| c3 d0 greedy | 447.5 | 32.03 | 2% | – | – | 20.98 (62) | 13.25 | 14.50 | 0.00 | 10.19 / 30.62 (33%) |
| c3 d0 engine | 440.8 | 30.53 | 2% | – | – | 12.27 (37) | 10.12 | 19.56 | 0.00 | 17.44 / 25.19 (69%) |
| c3 d1 random | 558.1 | 15.29 | 1% | – | – | 41.60 (127) | 25.44 | 29.69 | 7.19 | 11.50 / 40.88 (28%) |
| c3 d1 greedy | 462.4 | 31.46 | 2% | – | – | 25.29 (102) | 12.56 | 20.75 | 0.00 | 14.88 / 44.12 (34%) |
| c3 d1 engine | 514.2 | 29.84 | 2% | – | – | 16.69 (51) | 10.50 | 31.00 | 0.00 | 26.25 / 42.56 (62%) |
| c3 d2 random | 431.1 | 15.14 | 2% | – | – | 39.15 (92) | 16.56 | 22.75 | 6.62 | 7.88 / 33.12 (24%) |
| c3 d2 greedy | 436.6 | 29.95 | 2% | – | – | 25.96 (73) | 10.25 | 17.19 | 0.00 | 13.69 / 40.81 (34%) |
| c3 d2 engine | 527.6 | 29.96 | 2% | – | – | 19.32 (53) | 11.44 | 26.00 | 0.00 | 24.31 / 41.62 (58%) |
| c3 d3 random | 319.0 | 14.91 | 2% | – | – | 38.56 (96) | 14.19 | 20.00 | 7.00 | 6.50 / 25.44 (26%) |
| c3 d3 greedy | 499.1 | 27.79 | 2% | – | – | 38.83 (113) | 12.19 | 15.75 | 0.00 | 13.56 / 44.56 (30%) |
| c3 d3 engine | 601.9 | 28.34 | 2% | – | – | 29.21 (67) | 11.56 | 24.38 | 0.00 | 24.56 / 45.44 (54%) |
| c3 d4 random | 210.6 | 14.43 | 2% | – | – | 31.79 (63) | 9.00 | 12.81 | 4.50 | 3.31 / 17.44 (19%) |
| c3 d4 greedy | 364.1 | 26.12 | 2% | – | – | 34.75 (84) | 6.88 | 10.00 | 0.00 | 9.12 / 32.00 (29%) |
| c3 d4 engine | 443.2 | 26.21 | 2% | – | – | 30.98 (74) | 7.88 | 15.31 | 0.00 | 15.88 / 33.69 (47%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 10% / 71% / 19% | 10% / 72% / 18% |
| c2 d1 | 15% / 65% / 20% | 4% / 84% / 12% |
| c2 d2 | 20% / 40% / 40% | 5% / 85% / 10% |
| c2 d3 | 26% / 36% / 39% | 8% / 87% / 5% |
| c2 d4 | 30% / 10% / 60% | 28% / 28% / 45% |
| c3 d0 | 11% / 67% / 22% | 6% / 81% / 13% |
| c3 d1 | 17% / 53% / 31% | 6% / 84% / 10% |
| c3 d2 | 23% / 31% / 46% | 2% / 93% / 5% |
| c3 d3 | 29% / 18% / 53% | 6% / 86% / 9% |
| c3 d4 | 20% / 18% / 62% | 25% / 43% / 32% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 random | 16 of 16 | 0.85 | 0 of 0 | – |
| c2 d0 greedy | 5 of 16 | 0.43 | 4 of 11 | 0.38 |
| c2 d0 engine | 15 of 16 | 0.66 | 15 of 16 | 0.66 |
| c2 d1 random | 16 of 16 | 0.87 | 0 of 0 | – |
| c2 d1 greedy | 7 of 16 | 0.54 | 7 of 13 | 0.61 |
| c2 d1 engine | 15 of 16 | 0.68 | 13 of 14 | 0.69 |
| c2 d2 random | 16 of 16 | 0.86 | 0 of 0 | – |
| c2 d2 greedy | 8 of 16 | 0.53 | 8 of 12 | 0.60 |
| c2 d2 engine | 13 of 16 | 0.64 | 7 of 8 | 0.60 |
| c2 d3 random | 15 of 16 | 0.88 | 0 of 0 | – |
| c2 d3 greedy | 7 of 16 | 0.49 | 6 of 10 | 0.57 |
| c2 d3 engine | 13 of 16 | 0.63 | 10 of 10 | 0.65 |
| c2 d4 random | 15 of 16 | 0.89 | 0 of 0 | – |
| c2 d4 greedy | 6 of 16 | 0.45 | 3 of 4 | 0.51 |
| c2 d4 engine | 12 of 16 | 0.64 | 4 of 4 | 0.78 |
| c3 d0 random | 16 of 16 | 0.86 | 0 of 0 | – |
| c3 d0 greedy | 10 of 16 | 0.48 | 7 of 9 | 0.50 |
| c3 d0 engine | 16 of 16 | 0.61 | 16 of 16 | 0.61 |
| c3 d1 random | 16 of 16 | 0.84 | 0 of 0 | – |
| c3 d1 greedy | 9 of 16 | 0.46 | 5 of 9 | 0.34 |
| c3 d1 engine | 16 of 16 | 0.60 | 14 of 14 | 0.63 |
| c3 d2 random | 16 of 16 | 0.87 | 0 of 0 | – |
| c3 d2 greedy | 9 of 16 | 0.46 | 5 of 8 | 0.42 |
| c3 d2 engine | 16 of 16 | 0.61 | 10 of 10 | 0.60 |
| c3 d3 random | 16 of 16 | 0.88 | 0 of 0 | – |
| c3 d3 greedy | 9 of 16 | 0.47 | 3 of 4 | 0.33 |
| c3 d3 engine | 16 of 16 | 0.61 | 7 of 7 | 0.59 |
| c3 d4 random | 16 of 16 | 0.91 | 0 of 0 | – |
| c3 d4 greedy | 10 of 16 | 0.54 | 2 of 2 | 0.50 |
| c3 d4 engine | 16 of 16 | 0.63 | 6 of 6 | 0.66 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 5.31 / 1.06 / 13.2, – | 6.69 / 1.25 / 18.4, – |
| greedy | 2.00 / 0.06 / 11.7, 73% (73%) | 2.44 / 0.00 / 22.7, 89% (89%) |
| engine | 3.75 / 0.12 / 14.3, 100% (100%) | 5.00 / 1.38 / 40.8, 100% (100%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 5.00 / 0.44 / 12.0, – | 6.19 / 0.81 / 16.7, – |
| greedy | 1.94 / 0.00 / 11.4, 77% (77%) | 2.50 / 0.19 / 22.5, 89% (89%) |
| engine | 3.62 / 0.31 / 19.3, 100% (100%) | 4.88 / 1.06 / 45.8, 100% (100%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 4.19 / 0.31 / 9.4, – | 5.00 / 0.50 / 13.8, – |
| greedy | 1.94 / 0.00 / 13.1, 75% (75%) | 2.50 / 0.06 / 22.1, 88% (88%) |
| engine | 2.94 / 0.62 / 31.5, 88% (88%) | 4.12 / 1.31 / 47.2, 100% (100%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.88 / 0.19 / 7.3, – | 4.62 / 0.12 / 10.5, – |
| greedy | 1.94 / 0.00 / 17.6, 70% (70%) | 2.50 / 0.12 / 25.5, 75% (75%) |
| engine | 3.12 / 0.12 / 29.2, 90% (90%) | 4.25 / 1.31 / 54.8, 100% (100%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.44 / 0.06 / 5.6, – | 3.12 / 0.00 / 6.9, – |
| greedy | 1.81 / 0.00 / 12.2, 50% (50%) | 2.25 / 0.25 / 18.5, 100% (100%) |
| engine | 2.44 / 0.19 / 20.8, 75% (75%) | 3.62 / 0.75 / 40.5, 83% (83%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|---|
| antimatter_refinery | 2.5 (179) / 49% / 3% | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 2.4 (230) / 53% / 26% | 1.1 (185) / 100% / 0% | 1.1 (176) / 99% / 1% |
| civic_shelter | 2.4 (218) / 59% / 32% | 1.9 (160) / 100% / 0% | 1.9 (164) / 99% / 0% |
| contract_bid | 2.6 (246) / 59% / 48% | 3.9 (162) / 100% / 0% | 3.2 (769) / 84% / 89% |
| counter_intelligence | 2.5 (188) / 55% / 27% | – (0) / – / 0% | – (0) / – / 47% |
| data_mining | 2.4 (255) / 40% / 34% | 3.9 (163) / 100% / 0% | 2.8 (665) / 87% / 80% |
| emergency_fab | 2.7 (241) / 57% / 15% | 2.7 (159) / 100% / 0% | 2.6 (174) / 93% / 4% |
| evacuate | 2.3 (267) / 49% / 29% | 2.4 (364) / 100% / 67% | 2.3 (426) / 83% / 71% |
| fab_line | 2.4 (236) / 48% / 30% | 1.9 (164) / 98% / 0% | 1.9 (161) / 99% / 39% |
| firewall_node | 2.4 (232) / 49% / 34% | 1.9 (163) / 99% / 0% | 1.8 (159) / 100% / 0% |
| fork_process | 2.6 (257) / 50% / 40% | 2.2 (250) / 99% / 38% | 2.1 (296) / 92% / 49% |
| fusion_plant | 2.3 (183) / 48% / 0% | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | 2.5 (178) / 53% / 0% | – (0) / – / 0% | – (0) / – / 0% |
| grant | 2.7 (235) / 51% / 33% | 3.3 (170) / 99% / 0% | 2.7 (592) / 82% / 77% |
| grid_battery | 2.7 (225) / 46% / 40% | 2.0 (161) / 99% / 0% | 2.0 (161) / 96% / 11% |
| jam_signals | 2.7 (273) / 56% / 39% | 3.2 (163) / 98% / 0% | 2.4 (683) / 83% / 88% |
| liquidate_reserves | 2.5 (246) / 51% / 35% | 3.5 (171) / 100% / 0% | 3.4 (267) / 75% / 11% |
| logistics_ai | 2.6 (224) / 46% / 32% | 1.3 (180) / 100% / 0% | 1.4 (171) / 99% / 0% |
| megastructure | 2.3 (189) / 56% / 0% | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.5 (235) / 48% / 36% | 1.9 (158) / 100% / 0% | 1.9 (165) / 96% / 37% |
| modular_expansion | – (0) / – / 0.36/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| opposition_network | 2.3 (233) / 53% / 22% | 1.7 (161) / 100% / 0% | 1.7 (162) / 98% / 0% |
| orbital_lance | 2.5 (191) / 46% / 2% | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 2.5 (236) / 52% / 24% | 1.0 (215) / 100% / 0% | 1.0 (183) / 100% / 0% |
| point_defense | – (0) / – / 0.28/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| power_share | 2.3 (224) / 54% / 52% | 3.8 (164) / 98% / 0% | 3.5 (248) / 69% / 4% |
| precision_strike | 2.3 (261) / 55% / 24% | 1.9 (360) / 100% / 66% | 1.9 (483) / 95% / 75% |
| purge_protocol | 2.4 (207) / 50% / 14% | – (0) / – / 9% | – (0) / – / 42% |
| quantum_core | 2.5 (193) / 48% / 2% | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.3 (245) / 51% / 34% | 2.0 (608) / 100% / 86% | 2.0 (733) / 90% / 92% |
| rebuild | 2.5 (232) / 46% / 30% | 1.5 (463) / 100% / 79% | 1.6 (441) / 94% / 77% |
| recon_sweep | 2.5 (270) / 46% / 43% | 3.7 (170) / 98% / 0% | 3.4 (245) / 71% / 0% |
| reconstruction | 2.6 (184) / 47% / 0% | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 2.5 (207) / 55% / 29% | 1.1 (159) / 100% / 83% | 1.2 (161) / 100% / 84% |
| scavenge | 2.7 (245) / 50% / 38% | 3.8 (160) / 100% / 0% | 3.3 (293) / 77% / 38% |
| security_patch | 2.7 (240) / 44% / 34% | 3.3 (165) / 99% / 0% | 3.1 (203) / 79% / 0% |
| server_farm | 2.5 (231) / 47% / 23% | 1.6 (158) / 100% / 39% | 1.5 (162) / 98% / 42% |
| shelter_order | – (0) / – / 0.46/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shore_up | 2.6 (231) / 48% / 32% | 3.0 (158) / 100% / 0% | 2.9 (175) / 91% / 2% |
| solar_field | 2.5 (213) / 59% / 46% | 2.6 (159) / 100% / 0% | 2.5 (167) / 92% / 68% |
| strike_team | – (0) / – / 0.27/game | – (0) / – / 0.72/game | – (0) / – / 0.82/game |
| swarm_override | 2.5 (171) / 52% / 2% | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 2.4 (241) / 52% / 31% | 1.3 (748) / 100% / 93% | 1.4 (705) / 96% / 91% |
| trade_office | 2.3 (224) / 47% / 36% | 1.2 (172) / 100% / 0% | 1.3 (169) / 100% / 0% |
| turnkey_factory | – (0) / – / 0.47/game | – (0) / – / 0.97/game | – (0) / – / 0.97/game |
| venture_capital | – (0) / – / 0.56/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 2.4 (227) / 48% / 32% | 1.6 (159) / 100% / 0% | 1.6 (157) / 100% / 0% |
| zero_day_wipe | 2.6 (186) / 43% / 15% | – (0) / – / 0% | – (0) / – / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 random | 0.0 | 0.000 | 0.0 | 44.7 |
| c2 d0 greedy | 1390.8 | 5.224 | 0.0 | 1420.0 |
| c2 d0 engine | 810.2 | 4.305 | 0.0 | 826.5 |
| c2 d1 random | 0.0 | 0.000 | 0.0 | 42.2 |
| c2 d1 greedy | 1211.6 | 4.796 | 0.0 | 1239.6 |
| c2 d1 engine | 1446.6 | 5.615 | 0.0 | 1472.4 |
| c2 d2 random | 0.0 | 0.000 | 0.0 | 28.7 |
| c2 d2 greedy | 1316.3 | 4.932 | 0.0 | 1345.9 |
| c2 d2 engine | 2550.4 | 6.892 | 0.0 | 2591.4 |
| c2 d3 random | 0.0 | 0.000 | 0.0 | 21.5 |
| c2 d3 greedy | 1945.9 | 5.628 | 0.0 | 1989.8 |
| c2 d3 engine | 2518.1 | 7.029 | 0.0 | 2559.6 |
| c2 d4 random | 0.0 | 0.000 | 0.0 | 13.9 |
| c2 d4 greedy | 988.2 | 4.190 | 0.0 | 1013.5 |
| c2 d4 engine | 1448.6 | 5.519 | 0.0 | 1475.7 |
| c3 d0 random | 0.0 | 0.000 | 0.0 | 96.9 |
| c3 d0 greedy | 7112.1 | 15.893 | 0.0 | 7185.5 |
| c3 d0 engine | 9251.0 | 20.989 | 0.0 | 9316.5 |
| c3 d1 random | 0.0 | 0.000 | 0.0 | 91.1 |
| c3 d1 greedy | 7691.4 | 16.634 | 0.0 | 7767.9 |
| c3 d1 engine | 11450.6 | 22.267 | 0.0 | 11528.6 |
| c3 d2 random | 0.0 | 0.000 | 0.0 | 63.4 |
| c3 d2 greedy | 6683.3 | 15.307 | 0.0 | 6753.3 |
| c3 d2 engine | 10785.0 | 20.443 | 0.0 | 10864.2 |
| c3 d3 random | 0.0 | 0.000 | 0.0 | 42.8 |
| c3 d3 greedy | 7359.2 | 14.744 | 0.0 | 7441.4 |
| c3 d3 engine | 11886.7 | 19.747 | 0.0 | 11978.5 |
| c3 d4 random | 0.0 | 0.000 | 0.0 | 24.5 |
| c3 d4 greedy | 4912.8 | 13.492 | 0.0 | 4967.1 |
| c3 d4 engine | 6882.1 | 15.526 | 0.0 | 6944.6 |

Where the games' time went (summed over workers): agents 98.5%, probes 0.0%, apply 0.5%, legal 0.5%, replay check 0.5%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c29-baseline-c3-d4-engine-seed12.json`](records/close-loss-c29-baseline-c3-d4-engine-seed12.json): close loss at difficulty 4: deck_empty in round 11, 2 adversary piece(s) over the level-2 bar, supply 21 of 30, 1 cities corrupted
- [`records/close-loss-c28-baseline-c3-d4-greedy-seed11.json`](records/close-loss-c28-baseline-c3-d4-greedy-seed11.json): close loss at difficulty 4: deck_empty in round 11, 3 adversary piece(s) over the level-2 bar, supply 20 of 30, 1 cities corrupted

**early-lock-in**: none matched


**surprising-win**: none matched


