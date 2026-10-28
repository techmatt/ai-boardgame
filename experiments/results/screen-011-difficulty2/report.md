# Cycle report: screen-011-difficulty2

011 second screen (no probes), fresh seeds: the finalists for difficulty 1 (every counter requirement +2 or +3) and 3 (foundry reach 3 or 4). Level 1 reads as d1 - d0, level 3 as d3 - d2. Greedy only.

512 games (8 cells × 64 seeds, 510000..=510063) on 12 threads in 523.3 s. Engine 0.1.0, rules r4, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: cba2_reach3 (adversary.counter_bonus=0 adversary.counter_bonus_all=2 adversary.foundry_bonus=0 adversary.foundry_reach=3); cba3_reach4 (adversary.counter_bonus=0 adversary.counter_bonus_all=3 adversary.foundry_bonus=0 adversary.foundry_reach=4)
- Difficulties: 0, 1, 2, 3
- Agents (weakest first): greedy
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, cba2_reach3): win rates greedy 95%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, cba2_reach3): survival (rounds; wins count extra) greedy 11.78; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, cba2_reach3): win rates greedy 92%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, cba2_reach3): survival (rounds; wins count extra) greedy 11.66; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, cba2_reach3): win rates greedy 89%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, cba2_reach3): survival (rounds; wins count extra) greedy 10.59; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, cba2_reach3): win rates greedy 80%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, cba2_reach3): survival (rounds; wins count extra) greedy 10.14; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, cba3_reach4): win rates greedy 95%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, cba3_reach4): survival (rounds; wins count extra) greedy 11.78; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, cba3_reach4): win rates greedy 91%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, cba3_reach4): survival (rounds; wins count extra) greedy 11.59; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, cba3_reach4): win rates greedy 88%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, cba3_reach4): survival (rounds; wins count extra) greedy 10.53; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, cba3_reach4): win rates greedy 78%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, cba3_reach4): survival (rounds; wins count extra) greedy 10.09; the ladder is ordered.
- Skill gap from difficulty 0 to 3: +0 → +0 pts win rate, +0.00 → +0.00 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.4 at 3 (target 8–10).
- Games ended by: corruption 37, deck_empty 7, opposition 453, population 15.
- Adversary operations: 11.1 per round on average across cells, at most 50 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Help and counters (strongest agent): 0.00 gifts per game; 15% of threats countered.
- Strategy health at difficulty 0 (cba2_reach3): the top agent there (greedy, 95% wins) plays 1.92 buildings and 2.14 tier-2 cards per game (of 19.8 cards); 92% of its wins had 2+ buildings played (92% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (cba2_reach3): the top agent there (greedy, 92% wins) plays 1.89 buildings and 2.47 tier-2 cards per game (of 19.5 cards); 92% of its wins had 2+ buildings played (92% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 2 (cba2_reach3): the top agent there (greedy, 89% wins) plays 1.81 buildings and 2.12 tier-2 cards per game (of 18.7 cards); 86% of its wins had 2+ buildings played (86% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 3 (cba2_reach3): the top agent there (greedy, 80% wins) plays 1.83 buildings and 1.98 tier-2 cards per game (of 18.2 cards); 86% of its wins had 2+ buildings played (86% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Variants against the base (cba2_reach3) at difficulty 0, greedy, win rate difference in pts (bold: sign test p < 0.05): cba3_reach4 +0.0 ±0.0.
- Variants against the base (cba2_reach3) at difficulty 1, greedy, win rate difference in pts (bold: sign test p < 0.05): cba3_reach4 -1.6 ±1.6.
- Variants against the base (cba2_reach3) at difficulty 2, greedy, win rate difference in pts (bold: sign test p < 0.05): cba3_reach4 -1.6 ±2.7.
- Variants against the base (cba2_reach3) at difficulty 3, greedy, win rate difference in pts (bold: sign test p < 0.05): cba3_reach4 -1.6 ±4.2.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| cba2_reach3 difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.3 ±2.6 (11.78) | +0.0 pts (+0.00 survival) |

| cba2_reach3 difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 92.2 ±3.4 (11.66) | +0.0 pts (+0.00 survival) |

| cba2_reach3 difficulty 2 | greedy | skill gap |
|---|---|---|
| win % (survival) | 89.1 ±3.9 (10.59) | +0.0 pts (+0.00 survival) |

| cba2_reach3 difficulty 3 | greedy | skill gap |
|---|---|---|
| win % (survival) | 79.7 ±5.0 (10.14) | +0.0 pts (+0.00 survival) |

| cba3_reach4 difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.3 ±2.6 (11.78) | +0.0 pts (+0.00 survival) |

| cba3_reach4 difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 90.6 ±3.6 (11.59) | +0.0 pts (+0.00 survival) |

| cba3_reach4 difficulty 2 | greedy | skill gap |
|---|---|---|
| win % (survival) | 87.5 ±4.1 (10.53) | +0.0 pts (+0.00 survival) |

| cba3_reach4 difficulty 3 | greedy | skill gap |
|---|---|---|
| win % (survival) | 78.1 ±5.2 (10.09) | +0.0 pts (+0.00 survival) |

## Variants against the base

The first variant, `cba2_reach3`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| cba3_reach4 | 0 | greedy | 95.3 → 95.3 | +0.0 ±0.0 | 1.000 | 7.77 → 7.77 |
| cba3_reach4 | 1 | greedy | 92.2 → 90.6 | -1.6 ±1.6 | 1.000 | 7.70 → 7.69 |
| cba3_reach4 | 2 | greedy | 89.1 → 87.5 | -1.6 ±2.7 | 1.000 | 7.52 → 7.53 |
| cba3_reach4 | 3 | greedy | 79.7 → 78.1 | -1.6 ±4.2 | 1.000 | 7.38 → 7.38 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| cba2_reach3 d0 greedy | 7.77 (1.01, 5..10) | corruption 2, opposition 61, population 1 | 2.21 / 16.11 | 9.3 / 2.00 | 15.53 | 5.14 |
| cba2_reach3 d1 greedy | 7.70 (0.82, 5..10) | corruption 3, deck_empty 1, opposition 59, population 1 | 2.29 / 15.22 | 15.6 / 1.60 | 14.28 | 5.08 |
| cba2_reach3 d2 greedy | 7.52 (0.87, 5..9) | corruption 6, deck_empty 1, opposition 57 | 1.46 / 13.88 | 16.4 / 1.57 | 12.61 | 5.28 |
| cba2_reach3 d3 greedy | 7.38 (1.01, 5..9) | corruption 6, deck_empty 1, opposition 51, population 6 | 1.47 / 14.94 | 19.8 / 1.31 | 13.05 | 3.11 |
| cba3_reach4 d0 greedy | 7.77 (1.01, 5..10) | corruption 2, opposition 61, population 1 | 2.21 / 16.11 | 9.3 / 2.00 | 15.53 | 5.14 |
| cba3_reach4 d1 greedy | 7.69 (0.81, 5..10) | corruption 4, deck_empty 1, opposition 58, population 1 | 2.31 / 14.84 | 16.2 / 1.67 | 13.70 | 5.02 |
| cba3_reach4 d2 greedy | 7.53 (0.87, 5..9) | corruption 7, deck_empty 1, opposition 56 | 1.43 / 13.68 | 20.9 / 1.50 | 12.22 | 5.11 |
| cba3_reach4 d3 greedy | 7.38 (0.99, 5..9) | corruption 7, deck_empty 2, opposition 50, population 5 | 1.48 / 14.80 | 22.1 / 1.29 | 12.89 | 2.92 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| cba2_reach3 d0 greedy | 216.4 | 9.97 | 4% | – | – | 9.85 (41) | 0.00 | 8.84 | 0.00 | 5.42 / 15.53 (35%) |
| cba2_reach3 d1 greedy | 213.4 | 9.77 | 4% | – | – | 10.58 (40) | 0.00 | 6.19 | 0.00 | 1.78 / 15.41 (12%) |
| cba2_reach3 d2 greedy | 209.4 | 9.54 | 4% | – | – | 11.51 (44) | 0.00 | 4.88 | 0.00 | 1.30 / 15.03 (9%) |
| cba2_reach3 d3 greedy | 206.2 | 9.45 | 4% | – | – | 12.06 (50) | 0.00 | 4.88 | 0.00 | 1.30 / 14.75 (9%) |
| cba3_reach4 d0 greedy | 216.4 | 9.97 | 4% | – | – | 9.85 (41) | 0.00 | 8.84 | 0.00 | 5.42 / 15.53 (35%) |
| cba3_reach4 d1 greedy | 213.3 | 9.65 | 4% | – | – | 10.87 (40) | 0.00 | 4.41 | 0.00 | 1.38 / 15.38 (9%) |
| cba3_reach4 d2 greedy | 208.1 | 9.39 | 4% | – | – | 11.72 (44) | 0.00 | 3.45 | 0.00 | 1.03 / 15.06 (7%) |
| cba3_reach4 d3 greedy | 203.7 | 9.30 | 4% | – | – | 12.09 (50) | 0.00 | 3.45 | 0.00 | 1.03 / 14.75 (7%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| cba2_reach3 d0 | – (no variance) | – (no variance) |
| cba2_reach3 d1 | – (no variance) | – (no variance) |
| cba2_reach3 d2 | – (no variance) | – (no variance) |
| cba2_reach3 d3 | – (no variance) | – (no variance) |
| cba3_reach4 d0 | – (no variance) | – (no variance) |
| cba3_reach4 d1 | – (no variance) | – (no variance) |
| cba3_reach4 d2 | – (no variance) | – (no variance) |
| cba3_reach4 d3 | – (no variance) | – (no variance) |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| cba2_reach3 d0 greedy | 3 of 64 | 0.35 | 3 of 61 | 0.35 |
| cba2_reach3 d1 greedy | 3 of 64 | 0.36 | 3 of 59 | 0.34 |
| cba2_reach3 d2 greedy | 4 of 64 | 0.37 | 4 of 57 | 0.36 |
| cba2_reach3 d3 greedy | 3 of 64 | 0.36 | 3 of 51 | 0.34 |
| cba3_reach4 d0 greedy | 3 of 64 | 0.35 | 3 of 61 | 0.35 |
| cba3_reach4 d1 greedy | 3 of 64 | 0.36 | 3 of 58 | 0.35 |
| cba3_reach4 d2 greedy | 4 of 64 | 0.37 | 4 of 56 | 0.36 |
| cba3_reach4 d3 greedy | 3 of 64 | 0.36 | 3 of 50 | 0.33 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | cba2_reach3 | cba3_reach4 |
|---|---|---|
| greedy | 1.92 / 2.14 / 19.8, 92% (92%) | 1.92 / 2.14 / 19.8, 92% (92%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | cba2_reach3 | cba3_reach4 |
|---|---|---|
| greedy | 1.89 / 2.47 / 19.5, 92% (92%) | 1.83 / 2.38 / 19.6, 84% (84%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | cba2_reach3 | cba3_reach4 |
|---|---|---|
| greedy | 1.81 / 2.12 / 18.7, 86% (86%) | 1.78 / 2.05 / 18.5, 84% (84%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | cba2_reach3 | cba3_reach4 |
|---|---|---|
| greedy | 1.83 / 1.98 / 18.2, 86% (86%) | 1.78 / 2.02 / 17.8, 82% (82%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play |
|---|---|
| antimatter_refinery | – (0) / – / 0% |
| assembly_cluster | 1.1 (596) / 100% / 0% |
| civic_shelter | 2.0 (512) / 100% / 0% |
| contract_bid | 3.9 (510) / 98% / 0% |
| counter_intelligence | – (0) / – / 0% |
| data_mining | 3.6 (517) / 97% / 0% |
| emergency_fab | 2.5 (507) / 100% / 0% |
| evacuate | 3.0 (520) / 100% / 4% |
| fab_line | 1.9 (512) / 100% / 0% |
| firewall_node | 1.7 (498) / 100% / 0% |
| fork_process | 2.2 (1048) / 100% / 54% |
| fusion_plant | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% |
| grant | 3.6 (512) / 100% / 0% |
| grid_battery | 2.1 (512) / 100% / 0% |
| jam_signals | 3.4 (510) / 98% / 0% |
| liquidate_reserves | 3.6 (507) / 98% / 0% |
| logistics_ai | 1.1 (527) / 100% / 0% |
| megastructure | – (0) / – / 0% |
| microgrid | 2.2 (507) / 98% / 0% |
| opposition_network | 1.4 (517) / 100% / 0% |
| orbital_lance | – (0) / – / 0% |
| overclock_rig | 1.0 (642) / 100% / 0% |
| power_share | 3.6 (515) / 99% / 0% |
| precision_strike | 1.7 (2331) / 99% / 92% |
| purge_protocol | – (0) / – / 97% |
| quantum_core | – (0) / – / 0% |
| rally | 2.2 (1706) / 98% / 88% |
| rebuild | 2.0 (632) / 99% / 22% |
| recon_sweep | 3.6 (506) / 100% / 0% |
| reconstruction | – (0) / – / 0% |
| recruitment_hub | 1.6 (509) / 100% / 0% |
| scavenge | 3.7 (512) / 100% / 0% |
| server_farm | 1.4 (512) / 100% / 85% |
| shore_up | 2.8 (687) / 99% / 27% |
| solar_field | 2.7 (512) / 100% / 0% |
| strike_team | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% |
| sweep | 1.5 (1641) / 100% / 80% |
| trade_office | 1.3 (525) / 100% / 0% |
| turnkey_factory | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (506) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| cba2_reach3 d0 greedy | 9407.7 | 43.466 | 0.0 | 9458.9 |
| cba2_reach3 d1 greedy | 13734.8 | 64.360 | 0.0 | 13802.6 |
| cba2_reach3 d2 greedy | 12818.0 | 61.220 | 0.0 | 12883.9 |
| cba2_reach3 d3 greedy | 12328.7 | 59.780 | 0.0 | 12388.4 |
| cba3_reach4 d0 greedy | 12526.9 | 57.877 | 0.0 | 12590.9 |
| cba3_reach4 d1 greedy | 12050.1 | 56.499 | 0.0 | 12112.8 |
| cba3_reach4 d2 greedy | 11521.1 | 55.373 | 0.0 | 11578.6 |
| cba3_reach4 d3 greedy | 11137.1 | 54.665 | 0.0 | 11196.0 |

## Saved records

None matched.
