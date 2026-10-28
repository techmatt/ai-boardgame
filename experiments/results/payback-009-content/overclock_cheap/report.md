# Cycle report: payback-009-c-overclock_cheap

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py overclock_cheap): Overclock Rig (+1 insight) costs 6, no upkeep. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 731.1 s. Engine 0.1.0, rules r3, content 94daaf2601cc78f1.

- Configs: configs/baseline.json
- Variants: overclock_cheap
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 97%, engine 94%; skill gap -3 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.91, engine 11.77; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 77%, engine 72%; skill gap -5 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 10.00, engine 9.81; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: -3 → -5 pts win rate, -0.14 → -0.19 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 53, deck_empty 5, opposition 326.
- Adversary operations: 10.5 per round on average across cells, at most 44 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 70% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 21% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 97% wins) plays 1.91 buildings and 2.03 tier-2 cards per game (of 20.0 cards); 91% of its wins had 2+ buildings played (91% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 77% wins) plays 1.82 buildings and 1.78 tier-2 cards per game (of 17.2 cards); 86% of its wins had 2+ buildings played (86% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 72% vs greedy 77% over 96 seeds (only engine 9, only greedy 14), -5.2 ±5.0 pts, sign test p 0.405: no better than greedy (behind, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 96.9 ±1.8 (11.91) | 93.8 ±2.5 (11.77) | -3.1 pts (-0.14 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 77.1 ±4.3 (10.00) | 71.9 ±4.6 (9.81) | -5.2 pts (-0.19 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 93.8 | 96.9 | 88 / 2 / 5 / 1 | -3.1 ±2.8 | 0.453 | no better than greedy (behind, not significant) |
| d4 engine | 96 | 71.9 | 77.1 | 60 / 9 / 14 / 13 | -5.2 ±5.0 | 0.405 | no better than greedy (behind, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.82 (0.91, 5..10) | corruption 2, deck_empty 1, opposition 93 | 2.22 / 16.40 | 8.0 / 2.00 | 16.03 | 5.46 |
| d0 engine | 8.32 (1.54, 2..10) | corruption 5, deck_empty 1, opposition 90 | 1.68 / 16.72 | 13.0 / 1.83 | 15.69 | 5.55 |
| d4 greedy | 7.11 (1.07, 5..9) | corruption 20, deck_empty 2, opposition 74 | 1.74 / 8.27 | 15.7 / 1.27 | 6.49 | 5.40 |
| d4 engine | 7.22 (1.60, 2..9) | corruption 26, deck_empty 1, opposition 69 | 1.61 / 9.46 | 19.7 / 1.26 | 6.93 | 5.43 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 216.4 | 10.01 | 4% | – | – | 9.49 (42) | 0.00 | 9.21 | 0.00 | 5.73 / 15.65 (37%) |
| d0 engine | 280.3 | 9.92 | 4% | – | – | 11.05 (40) | 0.00 | 10.88 | 0.00 | 9.33 / 16.65 (56%) |
| d4 greedy | 197.1 | 9.42 | 4% | – | – | 10.15 (40) | 0.00 | 4.20 | 0.00 | 0.78 / 14.23 (5%) |
| d4 engine | 235.6 | 9.52 | 4% | – | – | 11.13 (44) | 0.00 | 6.33 | 0.00 | 1.98 / 14.44 (14%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 59% / 1% / 40% | 65% / 1% / 34% |
| d4 | 68% / 0% / 31% | 75% / 0% / 24% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.36 | 3 of 93 | 0.36 |
| d0 engine | 43 of 96 | 0.62 | 42 of 90 | 0.62 |
| d4 greedy | 3 of 96 | 0.36 | 3 of 74 | 0.35 |
| d4 engine | 22 of 96 | 0.54 | 16 of 69 | 0.53 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.91 | 2.03 | 20.0 | 91% (91%) |
| engine | 3.53 | 2.81 | 36.5 | 97% (97%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.82 | 1.78 | 17.2 | 86% (86%) |
| engine | 2.47 | 2.17 | 27.8 | 83% (83%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.0 (234) / 100% / 0% | 1.0 (218) / 100% / 2% |
| civic_shelter | 1.8 (192) / 100% / 0% | 1.7 (191) / 95% / 0% |
| contract_bid | 4.0 (192) / 99% / 0% | 3.2 (966) / 79% / 89% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 28% |
| data_mining | 3.6 (192) / 100% / 0% | 3.0 (663) / 84% / 74% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.5 (203) / 93% / 1% |
| evacuate | 3.0 (192) / 100% / 0% | 2.8 (246) / 89% / 23% |
| fab_line | 1.8 (194) / 99% / 0% | 1.8 (190) / 97% / 27% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.7 (188) / 99% / 0% |
| fork_process | 2.2 (379) / 100% / 53% | 2.2 (440) / 95% / 57% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (192) / 99% / 0% | 3.1 (646) / 82% / 73% |
| grid_battery | 1.9 (192) / 100% / 0% | 1.9 (192) / 96% / 8% |
| jam_signals | 3.2 (192) / 100% / 0% | 2.8 (493) / 88% / 64% |
| liquidate_reserves | 3.6 (192) / 99% / 0% | 3.5 (257) / 80% / 11% |
| logistics_ai | 1.1 (217) / 100% / 0% | 1.2 (206) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (189) / 100% / 0% | 2.3 (188) / 99% / 40% |
| opposition_network | 1.4 (196) / 100% / 0% | 1.4 (189) / 99% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 2.1 (191) / 100% / 0% | 2.1 (188) / 99% / 1% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (273) / 68% / 0% |
| precision_strike | 1.7 (875) / 100% / 92% | 1.9 (929) / 93% / 92% |
| purge_protocol | – (0) / – / 93% | – (0) / – / 94% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (647) / 99% / 88% | 2.3 (716) / 89% / 91% |
| rebuild | 2.0 (245) / 100% / 29% | 1.7 (409) / 97% / 67% |
| recon_sweep | 3.6 (186) / 100% / 0% | 3.6 (228) / 79% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 2% |
| recruitment_hub | 1.4 (197) / 100% / 0% | 1.4 (190) / 98% / 0% |
| scavenge | 3.6 (193) / 99% / 0% | 3.4 (264) / 73% / 9% |
| server_farm | 1.4 (192) / 100% / 86% | 1.4 (189) / 98% / 61% |
| shore_up | 3.0 (231) / 99% / 20% | 2.7 (305) / 90% / 36% |
| solar_field | 2.7 (191) / 100% / 0% | 2.7 (194) / 95% / 66% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (580) / 100% / 80% | 1.4 (824) / 97% / 88% |
| trade_office | 1.1 (222) / 100% / 0% | 1.1 (208) / 99% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.8 (191) / 100% / 0% | 1.8 (187) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 18330.7 | 84.717 | 0.0 | 18417.8 |
| d0 engine | 28675.5 | 102.314 | 0.0 | 28781.4 |
| d4 greedy | 16167.9 | 82.027 | 0.0 | 16240.6 |
| d4 engine | 26275.5 | 111.514 | 0.0 | 26373.3 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-overclock_cheap-d4-engine-seed300068.json`](records/close-loss-c3-baseline-overclock_cheap-d4-engine-seed300068.json): close loss at difficulty 4: corruption in round 8, 3 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c3-baseline-overclock_cheap-d4-engine-seed300047.json`](records/close-loss-c3-baseline-overclock_cheap-d4-engine-seed300047.json): close loss at difficulty 4: corruption in round 7, 3 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-overclock_cheap-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-overclock_cheap-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-overclock_cheap-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-overclock_cheap-d4-greedy-seed300000.json): won by the weakest agent (greedy)

