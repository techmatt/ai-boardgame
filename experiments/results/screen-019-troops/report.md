# Cycle report: screen-019-troops

019 quick screen (r10): adversary troops march 2 steps a round toward player presence, stations and factories and destroy them; nodes removed only by tier-2 effects (Node Takedown, Decommission); Unity can make nodes stop counting. 2 and 3 countries, d0 to d4, random, greedy and engine: the strategy-health gate, troop steps and destructions, presence lost and replaced, nodes made, removed and peak, purge blocked only by nodes.

480 games (30 cells × 16 seeds, 0..=15) on 12 threads in 104.7 s. Engine 0.1.0, rules r10, content bca600e0fc1926fd.

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
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) random 8.38, greedy 12.88, engine 13.00; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c2): win rates random 0%, greedy 94%, engine 88%; skill gap +88 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 1 (configs/baseline.json, c2): survival (rounds; wins count extra) random 7.06, greedy 12.94, engine 12.88; **the ladder is not ordered on survival**.
- Difficulty 2 (configs/baseline.json, c2): win rates random 0%, greedy 94%, engine 81%; skill gap +81 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) random 6.44, greedy 11.94, engine 11.44; **the ladder is not ordered on survival**.
- Difficulty 3 (configs/baseline.json, c2): win rates random 0%, greedy 56%, engine 62%; skill gap +62 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.50, greedy 10.50, engine 10.94; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c2): win rates random 0%, greedy 38%, engine 25%; skill gap +25 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) random 3.81, greedy 7.56, engine 7.25; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, c3): win rates random 0%, greedy 88%, engine 100%; skill gap +100 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) random 9.19, greedy 12.88, engine 13.00; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c3): win rates random 0%, greedy 81%, engine 81%; skill gap +81 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c3): survival (rounds; wins count extra) random 8.19, greedy 12.81, engine 12.81; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates random 0%, greedy 69%, engine 88%; skill gap +88 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) random 7.56, greedy 11.69, engine 11.88; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c3): win rates random 0%, greedy 56%, engine 62%; skill gap +62 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.25, greedy 10.56, engine 11.12; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates random 0%, greedy 50%, engine 69%; skill gap +69 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) random 4.06, greedy 8.94, engine 10.00; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +100 → +47 pts win rate, +4.22 → +4.69 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.2 rounds at difficulty 0, 4.9 at 4 (target 8–10).
- Games ended by: cities 30, deck_empty 40, purge 235, supply 175.
- Adversary operations: 27.0 per round on average across cells, at most 126 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, security_patch, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 11% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 6.77 gifts per game; 36% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 100% wins) plays 4.19 buildings and 1.00 tier-2 cards per game (of 29.4 cards); 94% of its wins had 2+ buildings played (94% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (c2): the top agent there (greedy, 94% wins) plays 1.75 buildings and 0.00 tier-2 cards per game (of 10.2 cards); 67% of its wins had 2+ buildings played (67% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 2 (c2): the top agent there (greedy, 94% wins) plays 1.75 buildings and 0.00 tier-2 cards per game (of 11.2 cards); 67% of its wins had 2+ buildings played (67% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 3 (c2): the top agent there (engine, 62% wins) plays 3.12 buildings and 0.38 tier-2 cards per game (of 26.1 cards); 90% of its wins had 2+ buildings played (90% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 4 (c2): the top agent there (greedy, 38% wins) plays 1.69 buildings and 0.00 tier-2 cards per game (of 8.2 cards); 83% of its wins had 2+ buildings played (83% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4 (c2): engine 25% vs greedy 38% over 16 seeds (only engine 0, only greedy 2), -12.5 ±8.5 pts, sign test p 0.500: no better than greedy (behind, not significant).
- Variants against the base (c2) at difficulty 0, greedy, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±9.1.
- Variants against the base (c2) at difficulty 1, greedy, win rate difference in pts (bold: sign test p < 0.05): c3 -12.5 ±8.5.
- Variants against the base (c2) at difficulty 2, greedy, win rate difference in pts (bold: sign test p < 0.05): c3 -25.0 ±14.4.
- Variants against the base (c2) at difficulty 3, greedy, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±18.3.
- Variants against the base (c2) at difficulty 4, greedy, win rate difference in pts (bold: sign test p < 0.05): c3 +12.5 ±15.5.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| c2 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.38) | 87.5 ±8.3 (12.88) | 100.0 ±0.0 (13.00) | +100.0 pts (+4.62 survival) |

| c2 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.06) | 93.8 ±6.1 (12.94) | 87.5 ±8.3 (12.88) | +87.5 pts (+5.81 survival) |

| c2 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.44) | 93.8 ±6.1 (11.94) | 81.2 ±9.8 (11.44) | +81.2 pts (+5.00 survival) |

| c2 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.50) | 56.2 ±12.4 (10.50) | 62.5 ±12.1 (10.94) | +62.5 pts (+5.44 survival) |

| c2 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.81) | 37.5 ±12.1 (7.56) | 25.0 ±10.8 (7.25) | +25.0 pts (+3.44 survival) |

| c3 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (9.19) | 87.5 ±8.3 (12.88) | 100.0 ±0.0 (13.00) | +100.0 pts (+3.81 survival) |

| c3 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.19) | 81.2 ±9.8 (12.81) | 81.2 ±9.8 (12.81) | +81.2 pts (+4.62 survival) |

| c3 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.56) | 68.8 ±11.6 (11.69) | 87.5 ±8.3 (11.88) | +87.5 pts (+4.31 survival) |

| c3 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.25) | 56.2 ±12.4 (10.56) | 62.5 ±12.1 (11.12) | +62.5 pts (+5.88 survival) |

| c3 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.06) | 50.0 ±12.5 (8.94) | 68.8 ±11.6 (10.00) | +68.8 pts (+5.94 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 random | 16 | 0.0 | 87.5 | 0 / 0 / 14 / 2 | -87.5 ±8.5 | <0.001 | worse than greedy |
| c2 d0 engine | 16 | 100.0 | 87.5 | 14 / 2 / 0 / 0 | +12.5 ±8.5 | 0.500 | no better than greedy (ahead, not significant) |
| c2 d1 random | 16 | 0.0 | 93.8 | 0 / 0 / 15 / 1 | -93.8 ±6.2 | <0.001 | worse than greedy |
| c2 d1 engine | 16 | 87.5 | 93.8 | 13 / 1 / 2 / 0 | -6.2 ±11.1 | 1.000 | no better than greedy (behind, not significant) |
| c2 d2 random | 16 | 0.0 | 93.8 | 0 / 0 / 15 / 1 | -93.8 ±6.2 | <0.001 | worse than greedy |
| c2 d2 engine | 16 | 81.2 | 93.8 | 12 / 1 / 3 / 0 | -12.5 ±12.5 | 0.625 | no better than greedy (behind, not significant) |
| c2 d3 random | 16 | 0.0 | 56.2 | 0 / 0 / 9 / 7 | -56.2 ±12.8 | 0.004 | worse than greedy |
| c2 d3 engine | 16 | 62.5 | 56.2 | 6 / 4 / 3 / 3 | +6.2 ±17.0 | 1.000 | no better than greedy (ahead, not significant) |
| c2 d4 random | 16 | 0.0 | 37.5 | 0 / 0 / 6 / 10 | -37.5 ±12.5 | 0.031 | worse than greedy |
| c2 d4 engine | 16 | 25.0 | 37.5 | 4 / 0 / 2 / 10 | -12.5 ±8.5 | 0.500 | no better than greedy (behind, not significant) |
| c3 d0 random | 16 | 0.0 | 87.5 | 0 / 0 / 14 / 2 | -87.5 ±8.5 | <0.001 | worse than greedy |
| c3 d0 engine | 16 | 100.0 | 87.5 | 14 / 2 / 0 / 0 | +12.5 ±8.5 | 0.500 | no better than greedy (ahead, not significant) |
| c3 d1 random | 16 | 0.0 | 81.2 | 0 / 0 / 13 / 3 | -81.2 ±10.1 | <0.001 | worse than greedy |
| c3 d1 engine | 16 | 81.2 | 81.2 | 10 / 3 / 3 / 0 | +0.0 ±15.8 | 1.000 | no better than greedy (level) |
| c3 d2 random | 16 | 0.0 | 68.8 | 0 / 0 / 11 / 5 | -68.8 ±12.0 | <0.001 | worse than greedy |
| c3 d2 engine | 16 | 87.5 | 68.8 | 9 / 5 / 2 / 0 | +18.8 ±16.4 | 0.453 | no better than greedy (ahead, not significant) |
| c3 d3 random | 16 | 0.0 | 56.2 | 0 / 0 / 9 / 7 | -56.2 ±12.8 | 0.004 | worse than greedy |
| c3 d3 engine | 16 | 62.5 | 56.2 | 6 / 4 / 3 / 3 | +6.2 ±17.0 | 1.000 | no better than greedy (ahead, not significant) |
| c3 d4 random | 16 | 0.0 | 50.0 | 0 / 0 / 8 / 8 | -50.0 ±12.9 | 0.008 | worse than greedy |
| c3 d4 engine | 16 | 68.8 | 50.0 | 7 / 4 / 1 / 4 | +18.8 ±13.6 | 0.375 | no better than greedy (ahead, not significant) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 8.38 → 9.19 |
| c3 | 0 | greedy | 87.5 → 87.5 | +0.0 ±9.1 | 1.000 | 7.19 → 7.12 |
| c3 | 0 | engine | 100.0 → 100.0 | +0.0 ±0.0 | 1.000 | 8.00 → 6.31 |
| c3 | 1 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 7.06 → 8.19 |
| c3 | 1 | greedy | 93.8 → 81.2 | -12.5 ±8.5 | 0.500 | 6.25 → 8.00 |
| c3 | 1 | engine | 87.5 → 81.2 | -6.2 ±11.1 | 1.000 | 7.75 → 7.62 |
| c3 | 2 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.44 → 7.56 |
| c3 | 2 | greedy | 93.8 → 68.8 | -25.0 ±14.4 | 0.219 | 6.06 → 6.81 |
| c3 | 2 | engine | 81.2 → 87.5 | +6.2 ±11.1 | 1.000 | 6.50 → 6.25 |
| c3 | 3 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.50 → 5.25 |
| c3 | 3 | greedy | 56.2 → 56.2 | +0.0 ±18.3 | 1.000 | 7.00 → 6.94 |
| c3 | 3 | engine | 62.5 → 62.5 | +0.0 ±18.3 | 1.000 | 7.31 → 7.19 |
| c3 | 4 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 3.81 → 4.06 |
| c3 | 4 | greedy | 37.5 → 50.0 | +12.5 ±15.5 | 0.688 | 4.94 → 5.56 |
| c3 | 4 | engine | 25.0 → 68.8 | +43.8 ±15.7 | 0.039 | 5.75 → 5.50 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 random | 8.38 (0.93, 7..10) | cities 6, supply 10 | – / – | 43.4 / 0.00 | 6.44 | 4.94 / 0.44 |
| c2 d0 greedy | 7.19 (1.98, 5..12) | deck_empty 2, purge 14 | 4.50 / 21.86 | 3.0 / 3.00 | 22.25 | 0.38 / 1.19 |
| c2 d0 engine | 8.00 (1.70, 4..11) | purge 16 | 3.00 / 24.50 | – / – | 24.50 | 0.00 / 2.56 |
| c2 d1 random | 7.06 (0.90, 5..8) | cities 7, supply 9 | – / – | 41.8 / 0.00 | 6.00 | 4.56 / 0.12 |
| c2 d1 greedy | 6.25 (2.14, 4..12) | deck_empty 1, purge 15 | 5.13 / 22.40 | 15.0 / 2.00 | 22.31 | 0.25 / 0.56 |
| c2 d1 engine | 7.75 (2.17, 5..12) | deck_empty 2, purge 14 | 3.86 / 23.07 | 2.5 / 3.00 | 22.81 | 0.31 / 1.31 |
| c2 d2 random | 6.44 (1.06, 4..8) | cities 7, supply 9 | – / – | 42.1 / 0.00 | 7.19 | 5.12 / 0.12 |
| c2 d2 greedy | 6.06 (2.01, 4..11) | deck_empty 1, purge 15 | 4.27 / 21.73 | 3.0 / 3.00 | 21.31 | 0.19 / 0.44 |
| c2 d2 engine | 6.50 (2.15, 4..11) | deck_empty 2, purge 13, supply 1 | 4.08 / 22.31 | 10.0 / 2.33 | 20.75 | 0.44 / 1.25 |
| c2 d3 random | 5.50 (0.61, 5..7) | cities 2, supply 14 | – / – | 40.9 / 0.12 | 1.81 | 3.75 / 0.31 |
| c2 d3 greedy | 7.00 (2.18, 5..11) | deck_empty 3, purge 9, supply 4 | 4.22 / 19.67 | 7.4 / 2.57 | 14.12 | 1.19 / 0.19 |
| c2 d3 engine | 7.31 (2.05, 5..11) | deck_empty 3, purge 10, supply 3 | 3.80 / 21.40 | 6.0 / 2.67 | 15.69 | 1.19 / 0.19 |
| c2 d4 random | 3.81 (0.73, 2..5) | supply 16 | – / – | 32.6 / 0.00 | 0.25 | 2.75 / 0.25 |
| c2 d4 greedy | 4.94 (0.75, 3..6) | purge 6, supply 10 | 5.00 / 13.33 | 21.6 / 1.00 | 5.25 | 1.50 / 0.06 |
| c2 d4 engine | 5.75 (1.64, 3..9) | purge 4, supply 12 | 4.00 / 12.75 | 17.7 / 1.42 | 3.44 | 1.88 / 0.19 |
| c3 d0 random | 9.19 (1.29, 8..12) | cities 4, deck_empty 2, supply 10 | – / – | 71.8 / 0.00 | 7.94 | 6.81 / 0.56 |
| c3 d0 greedy | 7.12 (2.96, 3..12) | deck_empty 2, purge 14 | 4.57 / 35.14 | 3.0 / 3.00 | 35.06 | 0.38 / 1.31 |
| c3 d0 engine | 6.31 (1.72, 4..11) | purge 16 | 4.69 / 36.50 | – / – | 36.50 | 0.00 / 3.56 |
| c3 d1 random | 8.19 (1.38, 6..12) | cities 3, deck_empty 1, supply 12 | – / – | 70.9 / 0.00 | 5.38 | 6.81 / 0.44 |
| c3 d1 greedy | 8.00 (2.98, 3..12) | deck_empty 3, purge 13 | 3.92 / 36.46 | 6.0 / 2.67 | 35.00 | 0.69 / 1.06 |
| c3 d1 engine | 7.62 (3.04, 4..12) | deck_empty 3, purge 13 | 4.38 / 36.46 | 6.7 / 2.67 | 36.44 | 0.50 / 2.31 |
| c3 d2 random | 7.56 (1.58, 4..11) | cities 1, deck_empty 1, supply 14 | – / – | 75.1 / 0.00 | 2.56 | 6.38 / 0.75 |
| c3 d2 greedy | 6.81 (2.94, 3..11) | deck_empty 5, purge 11 | 5.09 / 32.91 | 5.8 / 2.80 | 33.00 | 0.88 / 0.62 |
| c3 d2 engine | 6.25 (2.28, 4..11) | deck_empty 2, purge 14 | 4.43 / 34.50 | 3.5 / 3.00 | 34.88 | 0.44 / 2.06 |
| c3 d3 random | 5.25 (0.75, 4..7) | supply 16 | – / – | 60.4 / 0.00 | 0.50 | 4.56 / 0.19 |
| c3 d3 greedy | 6.94 (2.49, 3..11) | deck_empty 3, purge 9, supply 4 | 4.44 / 31.89 | 19.0 / 2.14 | 23.06 | 1.50 / 0.12 |
| c3 d3 engine | 7.19 (2.46, 4..11) | deck_empty 4, purge 10, supply 2 | 4.30 / 30.20 | 6.8 / 2.67 | 27.69 | 0.88 / 0.38 |
| c3 d4 random | 4.06 (0.75, 2..5) | supply 16 | – / – | 47.9 / 0.00 | 0.25 | 3.94 / 0.25 |
| c3 d4 greedy | 5.56 (1.32, 3..7) | purge 8, supply 8 | 4.75 / 17.75 | 17.5 / 1.88 | 8.88 | 1.44 / 0.12 |
| c3 d4 engine | 5.50 (1.37, 4..9) | purge 11, supply 5 | 4.55 / 21.09 | 26.6 / 1.20 | 14.50 | 0.94 / 0.31 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 random | 343.7 | 10.27 | 2% | – | – | 25.21 (75) | 8.38 | 11.50 | 2.19 | 4.94 / 23.25 (21%) |
| c2 d0 greedy | 244.1 | 22.68 | 3% | – | – | 12.57 (42) | 8.94 | 9.31 | 0.00 | 7.06 / 16.12 (44%) |
| c2 d0 engine | 325.8 | 23.46 | 3% | – | – | 8.73 (22) | 10.88 | 13.19 | 0.00 | 13.38 / 17.94 (75%) |
| c2 d1 random | 288.6 | 10.89 | 2% | – | – | 26.47 (76) | 7.75 | 20.44 | 5.00 | 6.88 / 29.81 (23%) |
| c2 d1 greedy | 236.5 | 20.72 | 3% | – | – | 16.76 (50) | 4.69 | 16.31 | 0.00 | 12.44 / 26.88 (46%) |
| c2 d1 engine | 341.1 | 20.74 | 3% | – | – | 15.90 (49) | 7.81 | 24.56 | 0.00 | 21.31 / 33.00 (65%) |
| c2 d2 random | 269.1 | 10.48 | 2% | – | – | 28.40 (72) | 6.50 | 18.06 | 4.44 | 5.69 / 26.62 (21%) |
| c2 d2 greedy | 239.4 | 19.48 | 3% | – | – | 19.86 (56) | 3.75 | 12.56 | 0.00 | 9.94 / 26.12 (38%) |
| c2 d2 engine | 282.2 | 18.85 | 2% | – | – | 17.88 (73) | 5.81 | 17.81 | 0.00 | 15.12 / 27.75 (55%) |
| c2 d3 random | 216.9 | 10.88 | 2% | – | – | 29.67 (74) | 6.69 | 17.06 | 5.12 | 5.25 / 20.81 (25%) |
| c2 d3 greedy | 276.4 | 19.19 | 3% | – | – | 29.04 (77) | 6.06 | 12.06 | 0.00 | 9.44 / 29.19 (32%) |
| c2 d3 engine | 330.8 | 19.13 | 2% | – | – | 27.94 (82) | 6.31 | 18.81 | 0.00 | 16.62 / 30.50 (55%) |
| c2 d4 random | 139.4 | 10.97 | 2% | – | – | 23.69 (64) | 3.81 | 10.38 | 3.12 | 3.12 / 13.69 (23%) |
| c2 d4 greedy | 192.2 | 15.85 | 2% | – | – | 25.68 (65) | 2.56 | 7.38 | 0.00 | 4.75 / 18.88 (25%) |
| c2 d4 engine | 239.9 | 17.13 | 2% | – | – | 26.45 (62) | 4.06 | 12.19 | 0.00 | 9.44 / 22.25 (42%) |
| c3 d0 random | 610.2 | 15.16 | 2% | – | – | 39.48 (104) | 32.25 | 21.50 | 5.75 | 7.69 / 32.62 (24%) |
| c3 d0 greedy | 345.8 | 31.14 | 2% | – | – | 21.00 (52) | 10.50 | 13.19 | 0.00 | 8.81 / 23.50 (38%) |
| c3 d0 engine | 350.4 | 28.59 | 2% | – | – | 14.89 (41) | 7.25 | 17.19 | 0.00 | 15.25 / 20.31 (75%) |
| c3 d1 random | 515.4 | 14.91 | 2% | – | – | 40.39 (108) | 22.94 | 27.94 | 6.94 | 9.75 / 40.56 (24%) |
| c3 d1 greedy | 410.7 | 30.76 | 2% | – | – | 27.38 (81) | 11.19 | 21.31 | 0.00 | 16.06 / 40.50 (40%) |
| c3 d1 engine | 456.3 | 29.14 | 2% | – | – | 21.55 (63) | 9.12 | 30.44 | 0.00 | 25.56 / 38.12 (67%) |
| c3 d2 random | 473.5 | 14.60 | 2% | – | – | 43.74 (126) | 20.75 | 23.69 | 5.88 | 8.31 / 35.25 (24%) |
| c3 d2 greedy | 354.9 | 28.35 | 2% | – | – | 29.21 (78) | 8.19 | 15.81 | 0.00 | 12.00 / 33.06 (36%) |
| c3 d2 engine | 371.1 | 26.23 | 2% | – | – | 23.17 (61) | 6.25 | 23.31 | 0.00 | 20.00 / 31.19 (64%) |
| c3 d3 random | 285.8 | 14.20 | 2% | – | – | 39.32 (92) | 13.19 | 15.88 | 4.38 | 5.38 / 24.38 (22%) |
| c3 d3 greedy | 358.8 | 26.91 | 2% | – | – | 39.68 (103) | 7.50 | 15.06 | 0.00 | 10.50 / 33.69 (31%) |
| c3 d3 engine | 441.4 | 25.77 | 2% | – | – | 34.37 (89) | 7.00 | 23.75 | 0.00 | 21.00 / 35.00 (60%) |
| c3 d4 random | 208.9 | 14.49 | 2% | – | – | 32.52 (73) | 9.56 | 12.06 | 3.81 | 3.81 / 17.19 (22%) |
| c3 d4 greedy | 280.8 | 23.64 | 2% | – | – | 36.62 (97) | 4.31 | 9.44 | 0.00 | 7.12 / 25.88 (28%) |
| c3 d4 engine | 325.9 | 23.62 | 2% | – | – | 32.09 (68) | 3.44 | 17.00 | 0.00 | 13.50 / 26.50 (51%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 5% / 84% / 10% | 2% / 93% / 5% |
| c2 d1 | 7% / 77% / 16% | 1% / 96% / 3% |
| c2 d2 | 9% / 71% / 20% | 5% / 82% / 13% |
| c2 d3 | 25% / 33% / 42% | 13% / 68% / 19% |
| c2 d4 | 49% / 15% / 36% | 34% / 27% / 39% |
| c3 d0 | 5% / 84% / 10% | 5% / 84% / 11% |
| c3 d1 | 10% / 59% / 30% | 5% / 87% / 8% |
| c3 d2 | 11% / 57% / 32% | 6% / 81% / 13% |
| c3 d3 | 25% / 33% / 42% | 10% / 74% / 16% |
| c3 d4 | 30% / 35% / 34% | 25% / 50% / 25% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 random | 16 of 16 | 0.86 | 0 of 0 | – |
| c2 d0 greedy | 6 of 16 | 0.45 | 6 of 14 | 0.46 |
| c2 d0 engine | 15 of 16 | 0.66 | 15 of 16 | 0.66 |
| c2 d1 random | 16 of 16 | 0.90 | 0 of 0 | – |
| c2 d1 greedy | 6 of 16 | 0.51 | 6 of 15 | 0.53 |
| c2 d1 engine | 12 of 16 | 0.63 | 10 of 14 | 0.62 |
| c2 d2 random | 16 of 16 | 0.90 | 0 of 0 | – |
| c2 d2 greedy | 7 of 16 | 0.57 | 7 of 15 | 0.58 |
| c2 d2 engine | 14 of 16 | 0.68 | 11 of 13 | 0.68 |
| c2 d3 random | 16 of 16 | 0.88 | 0 of 0 | – |
| c2 d3 greedy | 6 of 16 | 0.47 | 5 of 9 | 0.56 |
| c2 d3 engine | 15 of 16 | 0.68 | 10 of 10 | 0.72 |
| c2 d4 random | 13 of 16 | 0.88 | 0 of 0 | – |
| c2 d4 greedy | 7 of 16 | 0.53 | 4 of 6 | 0.67 |
| c2 d4 engine | 11 of 16 | 0.63 | 4 of 4 | 0.76 |
| c3 d0 random | 16 of 16 | 0.85 | 0 of 0 | – |
| c3 d0 greedy | 8 of 16 | 0.47 | 8 of 14 | 0.49 |
| c3 d0 engine | 16 of 16 | 0.62 | 16 of 16 | 0.62 |
| c3 d1 random | 16 of 16 | 0.87 | 0 of 0 | – |
| c3 d1 greedy | 7 of 16 | 0.49 | 7 of 13 | 0.53 |
| c3 d1 engine | 16 of 16 | 0.63 | 13 of 13 | 0.61 |
| c3 d2 random | 16 of 16 | 0.83 | 0 of 0 | – |
| c3 d2 greedy | 8 of 16 | 0.51 | 7 of 11 | 0.55 |
| c3 d2 engine | 15 of 16 | 0.64 | 14 of 14 | 0.65 |
| c3 d3 random | 16 of 16 | 0.90 | 0 of 0 | – |
| c3 d3 greedy | 10 of 16 | 0.54 | 7 of 9 | 0.61 |
| c3 d3 engine | 16 of 16 | 0.63 | 10 of 10 | 0.61 |
| c3 d4 random | 15 of 16 | 0.91 | 0 of 0 | – |
| c3 d4 greedy | 10 of 16 | 0.48 | 7 of 8 | 0.45 |
| c3 d4 engine | 16 of 16 | 0.68 | 11 of 11 | 0.64 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 5.31 / 0.62 / 12.2, – | 6.88 / 1.62 / 19.9, – |
| greedy | 1.94 / 0.00 / 10.3, 79% (79%) | 2.25 / 0.00 / 15.9, 79% (79%) |
| engine | 4.19 / 1.00 / 29.4, 94% (94%) | 4.81 / 1.12 / 31.5, 100% (100%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 3.50 / 0.31 / 8.4, – | 5.94 / 0.81 / 16.2, – |
| greedy | 1.75 / 0.00 / 10.2, 67% (67%) | 2.38 / 0.00 / 19.6, 92% (92%) |
| engine | 3.69 / 0.88 / 28.7, 100% (100%) | 4.62 / 1.44 / 40.6, 100% (100%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 3.31 / 0.31 / 8.3, – | 5.19 / 0.50 / 14.8, – |
| greedy | 1.75 / 0.00 / 11.2, 67% (67%) | 2.31 / 0.00 / 17.2, 91% (91%) |
| engine | 3.00 / 0.31 / 22.1, 100% (100%) | 4.00 / 0.62 / 31.6, 100% (100%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 3.06 / 0.25 / 7.6, – | 3.56 / 0.19 / 9.1, – |
| greedy | 1.88 / 0.00 / 13.9, 78% (78%) | 2.25 / 0.00 / 17.0, 78% (78%) |
| engine | 3.12 / 0.38 / 26.1, 90% (90%) | 4.00 / 0.94 / 38.1, 100% (100%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.44 / 0.06 / 6.0, – | 2.88 / 0.06 / 7.4, – |
| greedy | 1.69 / 0.00 / 8.2, 83% (83%) | 2.44 / 0.00 / 13.9, 100% (100%) |
| engine | 2.75 / 0.25 / 17.9, 100% (100%) | 3.75 / 0.19 / 28.8, 100% (100%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|---|
| antimatter_refinery | 2.7 (168) / 41% / 1% | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 2.4 (236) / 42% / 28% | 1.1 (191) / 100% / 0% | 1.2 (181) / 100% / 0% |
| civic_shelter | 2.5 (204) / 56% / 40% | 1.8 (161) / 100% / 0% | 1.8 (165) / 97% / 0% |
| contract_bid | 2.5 (231) / 53% / 47% | 3.9 (167) / 99% / 0% | 3.3 (671) / 82% / 84% |
| counter_intelligence | 2.5 (190) / 47% / 20% | – (0) / – / 0% | – (0) / – / 36% |
| data_mining | 2.4 (258) / 42% / 37% | 3.9 (161) / 100% / 0% | 2.9 (586) / 87% / 75% |
| emergency_fab | 2.6 (240) / 53% / 6% | 2.7 (159) / 100% / 0% | 2.7 (165) / 96% / 1% |
| evacuate | 2.4 (241) / 50% / 29% | 2.6 (298) / 100% / 62% | 2.5 (375) / 85% / 67% |
| fab_line | 2.6 (211) / 56% / 29% | 1.9 (168) / 99% / 0% | 1.9 (163) / 97% / 37% |
| firewall_node | 2.4 (222) / 43% / 34% | 1.9 (161) / 99% / 0% | 1.8 (160) / 100% / 0% |
| fork_process | 2.6 (254) / 53% / 42% | 2.3 (233) / 99% / 33% | 2.1 (278) / 95% / 44% |
| fusion_plant | 2.5 (181) / 45% / 1% | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | 2.6 (179) / 53% / 0% | – (0) / – / 0% | – (0) / – / 0% |
| grant | 2.6 (227) / 51% / 34% | 3.3 (168) / 100% / 0% | 2.8 (526) / 79% / 71% |
| grid_battery | 2.7 (231) / 43% / 34% | 2.0 (160) / 100% / 0% | 2.0 (165) / 96% / 10% |
| jam_signals | 2.8 (242) / 62% / 50% | 3.2 (160) / 100% / 0% | 2.4 (618) / 82% / 86% |
| liquidate_reserves | 2.6 (235) / 46% / 32% | 3.6 (169) / 99% / 0% | 3.4 (265) / 76% / 15% |
| logistics_ai | 2.5 (228) / 38% / 30% | 1.3 (181) / 100% / 0% | 1.3 (168) / 99% / 0% |
| megastructure | 2.4 (175) / 50% / 0% | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.4 (230) / 47% / 40% | 1.9 (158) / 100% / 0% | 1.9 (161) / 98% / 46% |
| modular_expansion | – (0) / – / 0.41/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| node_takedown | 2.3 (167) / 47% / 21% | – (0) / – / 0% | – (0) / – / 67% |
| opposition_network | 2.6 (210) / 52% / 24% | 1.7 (163) / 100% / 0% | 1.7 (163) / 99% / 0% |
| orbital_lance | 2.7 (149) / 52% / 1% | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 2.4 (225) / 58% / 22% | 1.0 (225) / 100% / 0% | 1.0 (187) / 100% / 0% |
| point_defense | – (0) / – / 0.23/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| power_share | 2.2 (224) / 50% / 38% | 3.8 (169) / 98% / 0% | 3.6 (226) / 73% / 4% |
| precision_strike | 2.4 (264) / 59% / 26% | 2.1 (286) / 100% / 50% | 2.0 (397) / 93% / 67% |
| purge_protocol | 2.5 (186) / 50% / 22% | – (0) / – / 0% | – (0) / – / 15% |
| quantum_core | 2.5 (167) / 46% / 0% | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (243) / 52% / 32% | 2.1 (497) / 100% / 90% | 2.1 (653) / 91% / 94% |
| rebuild | 2.5 (212) / 42% / 25% | 1.6 (334) / 99% / 75% | 1.6 (367) / 93% / 80% |
| recon_sweep | 2.6 (264) / 46% / 43% | 3.7 (163) / 100% / 0% | 3.5 (252) / 70% / 0% |
| reconstruction | 2.3 (179) / 46% / 0% | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 2.3 (229) / 55% / 22% | 1.1 (159) / 100% / 80% | 1.2 (160) / 100% / 88% |
| scavenge | 2.6 (242) / 43% / 31% | 3.7 (162) / 100% / 0% | 3.4 (283) / 76% / 26% |
| security_patch | 2.7 (235) / 41% / 38% | 3.3 (165) / 99% / 0% | 3.0 (210) / 74% / 0% |
| server_farm | 2.5 (220) / 38% / 26% | 1.6 (159) / 100% / 34% | 1.5 (165) / 98% / 34% |
| shelter_order | – (0) / – / 0.53/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shore_up | 2.6 (233) / 51% / 42% | 3.0 (158) / 100% / 0% | 2.9 (168) / 93% / 0% |
| solar_field | 2.5 (211) / 55% / 39% | 2.6 (161) / 99% / 0% | 2.5 (177) / 88% / 75% |
| strike_team | – (0) / – / 0.29/game | – (0) / – / 0.41/game | – (0) / – / 0.71/game |
| swarm_override | 2.5 (170) / 45% / 1% | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 2.5 (240) / 54% / 31% | 1.4 (582) / 100% / 91% | 1.4 (601) / 97% / 91% |
| trade_office | 2.4 (225) / 49% / 28% | 1.2 (185) / 100% / 0% | 1.3 (162) / 99% / 0% |
| turnkey_factory | – (0) / – / 0.45/game | – (0) / – / 0.93/game | – (0) / – / 0.93/game |
| venture_capital | – (0) / – / 0.56/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 2.4 (229) / 43% / 31% | 1.6 (159) / 100% / 0% | 1.6 (163) / 98% / 0% |
| zero_day_wipe | 2.6 (164) / 52% / 8% | – (0) / – / 0% | – (0) / – / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 random | 0.0 | 0.000 | 0.0 | 35.9 |
| c2 d0 greedy | 1293.1 | 5.297 | 0.0 | 1318.6 |
| c2 d0 engine | 3082.3 | 9.462 | 0.0 | 3117.8 |
| c2 d1 random | 0.0 | 0.000 | 0.0 | 29.9 |
| c2 d1 greedy | 1225.6 | 5.182 | 0.0 | 1249.7 |
| c2 d1 engine | 2570.0 | 7.534 | 0.0 | 2606.4 |
| c2 d2 random | 0.0 | 0.000 | 0.0 | 27.8 |
| c2 d2 greedy | 1214.2 | 5.072 | 0.0 | 1239.4 |
| c2 d2 engine | 1853.8 | 6.569 | 0.0 | 1881.9 |
| c2 d3 random | 0.0 | 0.000 | 0.0 | 21.6 |
| c2 d3 greedy | 1338.1 | 4.841 | 0.0 | 1368.9 |
| c2 d3 engine | 2418.4 | 7.311 | 0.0 | 2454.9 |
| c2 d4 random | 0.0 | 0.000 | 0.0 | 12.7 |
| c2 d4 greedy | 698.3 | 3.633 | 0.0 | 718.0 |
| c2 d4 engine | 1308.1 | 5.453 | 0.0 | 1333.0 |
| c3 d0 random | 0.1 | 0.000 | 0.0 | 100.3 |
| c3 d0 greedy | 5609.7 | 16.225 | 0.0 | 5658.2 |
| c3 d0 engine | 6526.5 | 18.624 | 0.0 | 6575.0 |
| c3 d1 random | 0.0 | 0.000 | 0.0 | 79.5 |
| c3 d1 greedy | 6646.8 | 16.185 | 0.0 | 6710.0 |
| c3 d1 engine | 8347.8 | 18.294 | 0.0 | 8414.2 |
| c3 d2 random | 0.0 | 0.000 | 0.0 | 74.5 |
| c3 d2 greedy | 4881.1 | 13.754 | 0.0 | 4933.8 |
| c3 d2 engine | 6066.2 | 16.348 | 0.0 | 6116.9 |
| c3 d3 random | 0.0 | 0.000 | 0.0 | 37.4 |
| c3 d3 greedy | 4538.3 | 12.650 | 0.0 | 4595.3 |
| c3 d3 engine | 7166.2 | 16.234 | 0.0 | 7227.8 |
| c3 d4 random | 0.0 | 0.000 | 0.0 | 26.4 |
| c3 d4 greedy | 3011.8 | 10.725 | 0.0 | 3050.2 |
| c3 d4 engine | 4798.7 | 14.726 | 0.0 | 4839.8 |

Where the games' time went (summed over workers): agents 98.3%, probes 0.0%, apply 0.6%, legal 0.5%, replay check 0.5%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c14-baseline-c2-d4-engine-seed9.json`](records/close-loss-c14-baseline-c2-d4-engine-seed9.json): close loss at difficulty 4: supply in round 8, 1 adversary piece(s) over the level-3 bar, supply 0 of 20, 1 cities corrupted
- [`records/close-loss-c14-baseline-c2-d4-engine-seed12.json`](records/close-loss-c14-baseline-c2-d4-engine-seed12.json): close loss at difficulty 4: supply in round 8, 2 adversary piece(s) over the level-3 bar, supply 0 of 20, 2 cities corrupted

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c14-baseline-c2-d4-engine-seed0.json`](records/surprising-win-c14-baseline-c2-d4-engine-seed0.json): won at difficulty 4 by engine, below the top agent
- [`records/surprising-win-c29-baseline-c3-d4-engine-seed0.json`](records/surprising-win-c29-baseline-c3-d4-engine-seed0.json): won at difficulty 4 by engine, below the top agent

