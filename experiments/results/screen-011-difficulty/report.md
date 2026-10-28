# Cycle report: screen-011-difficulty

011 screen (no probes): candidate effects for difficulty levels 1 and 3. Each variant pairs a level-1 candidate with a level-3 candidate; level 1's effect reads as d1 - d0, level 3's as d3 - d2. Greedy only (the top agent at r3).

1024 games (16 cells × 64 seeds, 500000..=500063) on 12 threads in 945.9 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: old; cba1_reach1 (adversary.counter_bonus=0 adversary.counter_bonus_all=1 adversary.foundry_bonus=0 adversary.foundry_reach=1); xt1_reach2 (adversary.counter_bonus=0 adversary.extra_threats=1 adversary.foundry_bonus=0 adversary.foundry_reach=2); cba2_reach3 (adversary.counter_bonus=0 adversary.counter_bonus_all=2 adversary.foundry_bonus=0 adversary.foundry_reach=3)
- Difficulties: 0, 1, 2, 3
- Agents (weakest first): greedy
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, old): win rates greedy 95%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, old): survival (rounds; wins count extra) greedy 11.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, old): win rates greedy 88%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, old): survival (rounds; wins count extra) greedy 11.41; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, old): win rates greedy 88%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, old): survival (rounds; wins count extra) greedy 10.59; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, old): win rates greedy 88%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, old): survival (rounds; wins count extra) greedy 10.59; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, cba1_reach1): win rates greedy 95%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, cba1_reach1): survival (rounds; wins count extra) greedy 11.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, cba1_reach1): win rates greedy 92%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, cba1_reach1): survival (rounds; wins count extra) greedy 11.67; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, cba1_reach1): win rates greedy 88%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, cba1_reach1): survival (rounds; wins count extra) greedy 10.55; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, cba1_reach1): win rates greedy 88%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, cba1_reach1): survival (rounds; wins count extra) greedy 10.55; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, xt1_reach2): win rates greedy 95%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, xt1_reach2): survival (rounds; wins count extra) greedy 11.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, xt1_reach2): win rates greedy 55%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, xt1_reach2): survival (rounds; wins count extra) greedy 9.70; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, xt1_reach2): win rates greedy 47%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, xt1_reach2): survival (rounds; wins count extra) greedy 8.66; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, xt1_reach2): win rates greedy 48%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, xt1_reach2): survival (rounds; wins count extra) greedy 8.53; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, cba2_reach3): win rates greedy 95%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, cba2_reach3): survival (rounds; wins count extra) greedy 11.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, cba2_reach3): win rates greedy 91%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, cba2_reach3): survival (rounds; wins count extra) greedy 11.55; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, cba2_reach3): win rates greedy 88%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, cba2_reach3): survival (rounds; wins count extra) greedy 10.59; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, cba2_reach3): win rates greedy 81%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, cba2_reach3): survival (rounds; wins count extra) greedy 10.27; the ladder is ordered.
- Skill gap from difficulty 0 to 3: +0 → +0 pts win rate, +0.00 → +0.00 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.9 rounds at difficulty 0, 7.5 at 3 (target 8–10).
- Games ended by: corruption 130, deck_empty 26, opposition 845, population 23.
- Adversary operations: 10.8 per round on average across cells, at most 61 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Help and counters (strongest agent): 0.00 gifts per game; 22% of threats countered.
- Strategy health at difficulty 0 (old): the top agent there (greedy, 95% wins) plays 1.92 buildings and 2.39 tier-2 cards per game (of 19.8 cards); 92% of its wins had 2+ buildings played (92% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (old): the top agent there (greedy, 88% wins) plays 1.91 buildings and 2.33 tier-2 cards per game (of 19.1 cards); 93% of its wins had 2+ buildings played (93% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 2 (old): the top agent there (greedy, 88% wins) plays 1.81 buildings and 2.00 tier-2 cards per game (of 18.5 cards); 86% of its wins had 2+ buildings played (86% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 3 (old): the top agent there (greedy, 88% wins) plays 1.81 buildings and 2.00 tier-2 cards per game (of 18.6 cards); 86% of its wins had 2+ buildings played (86% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Variants against the base (old) at difficulty 0, greedy, win rate difference in pts (bold: sign test p < 0.05): cba1_reach1 +0.0 ±0.0; xt1_reach2 +0.0 ±0.0; cba2_reach3 +0.0 ±0.0.
- Variants against the base (old) at difficulty 1, greedy, win rate difference in pts (bold: sign test p < 0.05): cba1_reach1 +4.7 ±4.1; **xt1_reach2 -32.8 ±6.7**; cba2_reach3 +3.1 ±2.2.
- Variants against the base (old) at difficulty 2, greedy, win rate difference in pts (bold: sign test p < 0.05): cba1_reach1 +0.0 ±2.2; **xt1_reach2 -40.6 ±7.3**; cba2_reach3 +0.0 ±0.0.
- Variants against the base (old) at difficulty 3, greedy, win rate difference in pts (bold: sign test p < 0.05): cba1_reach1 +0.0 ±2.2; **xt1_reach2 -39.1 ±7.3**; cba2_reach3 -6.2 ±3.0.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| old difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.3 ±2.6 (11.81) | +0.0 pts (+0.00 survival) |

| old difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 87.5 ±4.1 (11.41) | +0.0 pts (+0.00 survival) |

| old difficulty 2 | greedy | skill gap |
|---|---|---|
| win % (survival) | 87.5 ±4.1 (10.59) | +0.0 pts (+0.00 survival) |

| old difficulty 3 | greedy | skill gap |
|---|---|---|
| win % (survival) | 87.5 ±4.1 (10.59) | +0.0 pts (+0.00 survival) |

| cba1_reach1 difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.3 ±2.6 (11.81) | +0.0 pts (+0.00 survival) |

| cba1_reach1 difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 92.2 ±3.4 (11.67) | +0.0 pts (+0.00 survival) |

| cba1_reach1 difficulty 2 | greedy | skill gap |
|---|---|---|
| win % (survival) | 87.5 ±4.1 (10.55) | +0.0 pts (+0.00 survival) |

| cba1_reach1 difficulty 3 | greedy | skill gap |
|---|---|---|
| win % (survival) | 87.5 ±4.1 (10.55) | +0.0 pts (+0.00 survival) |

| xt1_reach2 difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.3 ±2.6 (11.81) | +0.0 pts (+0.00 survival) |

| xt1_reach2 difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 54.7 ±6.2 (9.70) | +0.0 pts (+0.00 survival) |

| xt1_reach2 difficulty 2 | greedy | skill gap |
|---|---|---|
| win % (survival) | 46.9 ±6.2 (8.66) | +0.0 pts (+0.00 survival) |

| xt1_reach2 difficulty 3 | greedy | skill gap |
|---|---|---|
| win % (survival) | 48.4 ±6.2 (8.53) | +0.0 pts (+0.00 survival) |

| cba2_reach3 difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.3 ±2.6 (11.81) | +0.0 pts (+0.00 survival) |

| cba2_reach3 difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 90.6 ±3.6 (11.55) | +0.0 pts (+0.00 survival) |

| cba2_reach3 difficulty 2 | greedy | skill gap |
|---|---|---|
| win % (survival) | 87.5 ±4.1 (10.59) | +0.0 pts (+0.00 survival) |

| cba2_reach3 difficulty 3 | greedy | skill gap |
|---|---|---|
| win % (survival) | 81.2 ±4.9 (10.27) | +0.0 pts (+0.00 survival) |

## Variants against the base

The first variant, `old`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| cba1_reach1 | 0 | greedy | 95.3 → 95.3 | +0.0 ±0.0 | 1.000 | 7.89 → 7.89 |
| cba1_reach1 | 1 | greedy | 87.5 → 92.2 | +4.7 ±4.1 | 0.453 | 7.62 → 7.86 |
| cba1_reach1 | 2 | greedy | 87.5 → 87.5 | +0.0 ±2.2 | 1.000 | 7.52 → 7.52 |
| cba1_reach1 | 3 | greedy | 87.5 → 87.5 | +0.0 ±2.2 | 1.000 | 7.53 → 7.52 |
| xt1_reach2 | 0 | greedy | 95.3 → 95.3 | +0.0 ±0.0 | 1.000 | 7.89 → 7.89 |
| xt1_reach2 | 1 | greedy | 87.5 → 54.7 | -32.8 ±6.7 | <0.001 | 7.62 → 7.42 |
| xt1_reach2 | 2 | greedy | 87.5 → 46.9 | -40.6 ±7.3 | <0.001 | 7.52 → 7.09 |
| xt1_reach2 | 3 | greedy | 87.5 → 48.4 | -39.1 ±7.3 | <0.001 | 7.53 → 6.94 |
| cba2_reach3 | 0 | greedy | 95.3 → 95.3 | +0.0 ±0.0 | 1.000 | 7.89 → 7.89 |
| cba2_reach3 | 1 | greedy | 87.5 → 90.6 | +3.1 ±2.2 | 0.500 | 7.62 → 7.62 |
| cba2_reach3 | 2 | greedy | 87.5 → 87.5 | +0.0 ±0.0 | 1.000 | 7.52 → 7.55 |
| cba2_reach3 | 3 | greedy | 87.5 → 81.2 | -6.2 ±3.0 | 0.125 | 7.53 → 7.50 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| old d0 greedy | 7.89 (0.75, 6..10) | corruption 1, deck_empty 1, opposition 61, population 1 | 2.11 / 16.51 | 10.3 / 2.00 | 15.92 | 5.33 |
| old d1 greedy | 7.62 (0.82, 5..9) | corruption 8, opposition 56 | 2.32 / 16.75 | 15.6 / 1.50 | 14.66 | 5.16 |
| old d2 greedy | 7.52 (0.79, 5..9) | corruption 5, deck_empty 3, opposition 56 | 1.52 / 14.86 | 8.0 / 1.88 | 13.73 | 5.56 |
| old d3 greedy | 7.53 (0.81, 5..9) | corruption 5, deck_empty 3, opposition 56 | 1.50 / 14.71 | 8.0 / 1.88 | 13.61 | 5.55 |
| cba1_reach1 d0 greedy | 7.89 (0.75, 6..10) | corruption 1, deck_empty 1, opposition 61, population 1 | 2.11 / 16.51 | 10.3 / 2.00 | 15.92 | 5.33 |
| cba1_reach1 d1 greedy | 7.86 (0.70, 7..10) | corruption 4, opposition 59, population 1 | 2.14 / 16.42 | 11.6 / 1.80 | 15.16 | 5.14 |
| cba1_reach1 d2 greedy | 7.52 (0.79, 5..9) | corruption 4, deck_empty 3, opposition 56, population 1 | 1.46 / 15.16 | 12.1 / 1.62 | 14.09 | 5.58 |
| cba1_reach1 d3 greedy | 7.52 (0.79, 5..9) | corruption 4, deck_empty 3, opposition 56, population 1 | 1.46 / 15.16 | 12.1 / 1.62 | 14.09 | 5.45 |
| xt1_reach2 d0 greedy | 7.89 (0.75, 6..10) | corruption 1, deck_empty 1, opposition 61, population 1 | 2.11 / 16.51 | 10.3 / 2.00 | 15.92 | 5.33 |
| xt1_reach2 d1 greedy | 7.42 (0.81, 6..9) | corruption 29, opposition 35 | 2.17 / 12.31 | 18.5 / 1.48 | 6.73 | 4.50 |
| xt1_reach2 d2 greedy | 7.09 (1.09, 5..9) | corruption 31, deck_empty 3, opposition 30 | 1.33 / 11.93 | 19.1 / 1.32 | 6.19 | 4.84 |
| xt1_reach2 d3 greedy | 6.94 (1.18, 5..9) | corruption 23, deck_empty 1, opposition 31, population 9 | 1.29 / 11.97 | 21.0 / 1.03 | 6.95 | 1.25 |
| cba2_reach3 d0 greedy | 7.89 (0.75, 6..10) | corruption 1, deck_empty 1, opposition 61, population 1 | 2.11 / 16.51 | 10.3 / 2.00 | 15.92 | 5.33 |
| cba2_reach3 d1 greedy | 7.62 (0.84, 5..10) | corruption 6, opposition 58 | 2.33 / 16.66 | 18.2 / 1.33 | 15.09 | 5.12 |
| cba2_reach3 d2 greedy | 7.55 (0.79, 5..9) | corruption 5, deck_empty 3, opposition 56 | 1.48 / 14.80 | 8.1 / 1.88 | 13.69 | 5.53 |
| cba2_reach3 d3 greedy | 7.50 (0.94, 4..9) | corruption 2, deck_empty 3, opposition 52, population 7 | 1.40 / 15.12 | 12.7 / 1.58 | 13.92 | 2.94 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| old d0 greedy | 217.5 | 10.08 | 4% | – | – | 9.53 (43) | 0.00 | 9.33 | 0.00 | 5.55 / 15.78 (35%) |
| old d1 greedy | 213.0 | 9.83 | 4% | – | – | 10.35 (34) | 0.00 | 7.22 | 0.00 | 1.94 / 15.25 (13%) |
| old d2 greedy | 208.2 | 9.58 | 4% | – | – | 10.44 (41) | 0.00 | 5.97 | 0.00 | 1.45 / 15.03 (10%) |
| old d3 greedy | 208.5 | 9.59 | 4% | – | – | 10.53 (43) | 0.00 | 5.97 | 0.00 | 1.45 / 15.06 (10%) |
| cba1_reach1 d0 greedy | 217.5 | 10.08 | 4% | – | – | 9.53 (43) | 0.00 | 9.33 | 0.00 | 5.55 / 15.78 (35%) |
| cba1_reach1 d1 greedy | 217.0 | 9.96 | 4% | – | – | 10.25 (41) | 0.00 | 8.66 | 0.00 | 3.62 / 15.72 (23%) |
| cba1_reach1 d2 greedy | 208.8 | 9.66 | 4% | – | – | 10.17 (41) | 0.00 | 7.28 | 0.00 | 2.89 / 15.03 (19%) |
| cba1_reach1 d3 greedy | 208.8 | 9.66 | 4% | – | – | 11.13 (60) | 0.00 | 7.28 | 0.00 | 2.89 / 15.03 (19%) |
| xt1_reach2 d0 greedy | 217.5 | 10.08 | 4% | – | – | 9.53 (43) | 0.00 | 9.33 | 0.00 | 5.55 / 15.78 (35%) |
| xt1_reach2 d1 greedy | 221.9 | 10.02 | 4% | – | – | 13.01 (60) | 0.00 | 16.47 | 0.00 | 9.22 / 29.69 (31%) |
| xt1_reach2 d2 greedy | 209.9 | 9.60 | 4% | – | – | 13.18 (53) | 0.00 | 13.58 | 0.00 | 7.42 / 28.38 (26%) |
| xt1_reach2 d3 greedy | 204.1 | 9.58 | 4% | – | – | 14.06 (61) | 0.00 | 13.33 | 0.00 | 7.30 / 27.75 (26%) |
| cba2_reach3 d0 greedy | 217.5 | 10.08 | 4% | – | – | 9.53 (43) | 0.00 | 9.33 | 0.00 | 5.55 / 15.78 (35%) |
| cba2_reach3 d1 greedy | 211.3 | 9.84 | 4% | – | – | 10.19 (34) | 0.00 | 6.38 | 0.00 | 1.83 / 15.25 (12%) |
| cba2_reach3 d2 greedy | 208.0 | 9.58 | 4% | – | – | 10.48 (41) | 0.00 | 5.06 | 0.00 | 1.38 / 15.09 (9%) |
| cba2_reach3 d3 greedy | 205.9 | 9.54 | 4% | – | – | 11.23 (44) | 0.00 | 5.06 | 0.00 | 1.38 / 15.00 (9%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| old d0 | – (no variance) | – (no variance) |
| old d1 | – (no variance) | – (no variance) |
| old d2 | – (no variance) | – (no variance) |
| old d3 | – (no variance) | – (no variance) |
| cba1_reach1 d0 | – (no variance) | – (no variance) |
| cba1_reach1 d1 | – (no variance) | – (no variance) |
| cba1_reach1 d2 | – (no variance) | – (no variance) |
| cba1_reach1 d3 | – (no variance) | – (no variance) |
| xt1_reach2 d0 | – (no variance) | – (no variance) |
| xt1_reach2 d1 | – (no variance) | – (no variance) |
| xt1_reach2 d2 | – (no variance) | – (no variance) |
| xt1_reach2 d3 | – (no variance) | – (no variance) |
| cba2_reach3 d0 | – (no variance) | – (no variance) |
| cba2_reach3 d1 | – (no variance) | – (no variance) |
| cba2_reach3 d2 | – (no variance) | – (no variance) |
| cba2_reach3 d3 | – (no variance) | – (no variance) |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| old d0 greedy | 3 of 64 | 0.35 | 3 of 61 | 0.35 |
| old d1 greedy | 3 of 64 | 0.35 | 3 of 56 | 0.34 |
| old d2 greedy | 3 of 64 | 0.35 | 3 of 56 | 0.33 |
| old d3 greedy | 3 of 64 | 0.35 | 3 of 56 | 0.33 |
| cba1_reach1 d0 greedy | 3 of 64 | 0.35 | 3 of 61 | 0.35 |
| cba1_reach1 d1 greedy | 3 of 64 | 0.35 | 3 of 59 | 0.34 |
| cba1_reach1 d2 greedy | 3 of 64 | 0.35 | 3 of 56 | 0.33 |
| cba1_reach1 d3 greedy | 3 of 64 | 0.35 | 3 of 56 | 0.33 |
| xt1_reach2 d0 greedy | 3 of 64 | 0.35 | 3 of 61 | 0.35 |
| xt1_reach2 d1 greedy | 3 of 64 | 0.34 | 3 of 35 | 0.28 |
| xt1_reach2 d2 greedy | 3 of 64 | 0.33 | 3 of 30 | 0.26 |
| xt1_reach2 d3 greedy | 3 of 64 | 0.34 | 3 of 31 | 0.29 |
| cba2_reach3 d0 greedy | 3 of 64 | 0.35 | 3 of 61 | 0.35 |
| cba2_reach3 d1 greedy | 3 of 64 | 0.35 | 3 of 58 | 0.33 |
| cba2_reach3 d2 greedy | 3 of 64 | 0.35 | 3 of 56 | 0.33 |
| cba2_reach3 d3 greedy | 3 of 64 | 0.35 | 3 of 52 | 0.33 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | old | cba1_reach1 | xt1_reach2 | cba2_reach3 |
|---|---|---|---|---|
| greedy | 1.92 / 2.39 / 19.8, 92% (92%) | 1.92 / 2.39 / 19.8, 92% (92%) | 1.92 / 2.39 / 19.8, 92% (92%) | 1.92 / 2.39 / 19.8, 92% (92%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | old | cba1_reach1 | xt1_reach2 | cba2_reach3 |
|---|---|---|---|---|
| greedy | 1.91 / 2.33 / 19.1, 93% (93%) | 1.91 / 2.22 / 19.6, 92% (92%) | 1.77 / 1.77 / 18.1, 91% (91%) | 1.91 / 2.36 / 19.0, 93% (93%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | old | cba1_reach1 | xt1_reach2 | cba2_reach3 |
|---|---|---|---|---|
| greedy | 1.81 / 2.00 / 18.5, 86% (86%) | 1.86 / 1.89 / 18.5, 88% (88%) | 1.75 / 1.48 / 16.5, 90% (90%) | 1.83 / 2.02 / 18.5, 88% (88%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | old | cba1_reach1 | xt1_reach2 | cba2_reach3 |
|---|---|---|---|---|
| greedy | 1.81 / 2.00 / 18.6, 86% (86%) | 1.86 / 1.89 / 18.5, 88% (88%) | 1.73 / 1.47 / 15.8, 84% (84%) | 1.81 / 2.08 / 18.2, 88% (88%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play |
|---|---|
| antimatter_refinery | – (0) / – / 0% |
| assembly_cluster | 1.0 (1198) / 100% / 0% |
| civic_shelter | 2.0 (1027) / 100% / 0% |
| contract_bid | 4.0 (1024) / 99% / 0% |
| counter_intelligence | – (0) / – / 0% |
| data_mining | 3.6 (1024) / 100% / 0% |
| emergency_fab | 2.5 (1024) / 100% / 0% |
| evacuate | 3.0 (1024) / 100% / 0% |
| fab_line | 1.8 (1021) / 100% / 0% |
| firewall_node | 2.0 (1024) / 99% / 0% |
| fork_process | 2.2 (1883) / 99% / 47% |
| fusion_plant | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% |
| grant | 3.6 (1021) / 100% / 0% |
| grid_battery | 2.1 (1027) / 100% / 0% |
| jam_signals | 3.1 (1033) / 99% / 0% |
| liquidate_reserves | 3.5 (1030) / 99% / 0% |
| logistics_ai | 1.3 (1100) / 100% / 0% |
| megastructure | – (0) / – / 0% |
| microgrid | 2.0 (1027) / 100% / 0% |
| opposition_network | 1.5 (1043) / 100% / 0% |
| orbital_lance | – (0) / – / 0% |
| overclock_rig | 1.0 (1280) / 100% / 0% |
| power_share | 3.7 (1027) / 100% / 0% |
| precision_strike | 1.7 (4594) / 100% / 91% |
| purge_protocol | – (0) / – / 95% |
| quantum_core | – (0) / – / 0% |
| rally | 2.1 (3448) / 99% / 89% |
| rebuild | 2.2 (1337) / 99% / 28% |
| recon_sweep | 3.6 (1041) / 98% / 0% |
| reconstruction | – (0) / – / 0% |
| recruitment_hub | 1.4 (1040) / 99% / 0% |
| scavenge | 3.5 (1027) / 100% / 0% |
| server_farm | 1.6 (1027) / 100% / 84% |
| shore_up | 2.8 (1283) / 100% / 24% |
| solar_field | 2.7 (1024) / 100% / 0% |
| strike_team | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% |
| sweep | 1.6 (3445) / 100% / 83% |
| trade_office | 1.3 (1034) / 100% / 0% |
| turnkey_factory | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 1.00/game |
| watchtower_net | 1.7 (1009) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| old d0 greedy | 8260.2 | 37.978 | 0.0 | 8300.2 |
| old d1 greedy | 7628.8 | 35.821 | 0.0 | 7669.4 |
| old d2 greedy | 6817.9 | 32.739 | 0.0 | 6853.6 |
| old d3 greedy | 7653.6 | 36.702 | 0.0 | 7692.2 |
| cba1_reach1 d0 greedy | 9410.1 | 43.265 | 0.0 | 9453.0 |
| cba1_reach1 d1 greedy | 13375.2 | 61.628 | 0.0 | 13444.7 |
| cba1_reach1 d2 greedy | 11729.3 | 56.176 | 0.0 | 11795.3 |
| cba1_reach1 d3 greedy | 12321.4 | 59.011 | 0.0 | 12380.9 |
| xt1_reach2 d0 greedy | 12517.3 | 57.551 | 0.0 | 12580.2 |
| xt1_reach2 d1 greedy | 12725.5 | 57.350 | 0.0 | 12786.3 |
| xt1_reach2 d2 greedy | 11426.2 | 54.426 | 0.0 | 11494.0 |
| xt1_reach2 d3 greedy | 10497.3 | 51.426 | 0.0 | 10555.8 |
| cba2_reach3 d0 greedy | 12810.7 | 58.900 | 0.0 | 12874.6 |
| cba2_reach3 d1 greedy | 12460.5 | 58.963 | 0.0 | 12535.6 |
| cba2_reach3 d2 greedy | 13644.3 | 65.608 | 0.0 | 13713.3 |
| cba2_reach3 d3 greedy | 11344.3 | 55.094 | 0.0 | 11409.6 |

## Saved records

None matched.
