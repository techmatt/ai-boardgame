# Cycle report: payback-009-c-t2_half+t2_noam

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py t2_half+t2_noam): tier 2 costs half its credits and no antimatter. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 878.8 s. Engine 0.1.0, rules r3, content 2f31b40bb42daaae.

- Configs: configs/baseline.json
- Variants: t2_half+t2_noam
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 98%, engine 61%; skill gap -36 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.91, engine 11.09; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 68%, engine 35%; skill gap -32 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.68, engine 9.09; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: -36 → -32 pts win rate, -0.81 → -0.58 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 8.0 rounds at difficulty 0, 7.4 at 4 (target 8–10).
- Games ended by: corruption 84, deck_empty 48, opposition 252.
- Adversary operations: 11.5 per round on average across cells, at most 48 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 58% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 24% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 98% wins) plays 1.91 buildings and 1.80 tier-2 cards per game (of 21.4 cards); 91% of its wins had 2+ buildings played (91% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 68% wins) plays 1.84 buildings and 1.43 tier-2 cards per game (of 19.1 cards); 91% of its wins had 2+ buildings played (91% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 35% vs greedy 68% over 96 seeds (only engine 10, only greedy 41), -32.3 ±6.7 pts, sign test p <0.001: worse than greedy.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 97.9 ±1.5 (11.91) | 61.5 ±5.0 (11.09) | -36.5 pts (-0.81 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 67.7 ±4.8 (9.68) | 35.4 ±4.9 (9.09) | -32.3 pts (-0.58 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 61.5 | 97.9 | 58 / 1 / 36 / 1 | -36.5 ±5.2 | <0.001 | worse than greedy |
| d4 engine | 96 | 35.4 | 67.7 | 24 / 10 / 41 / 21 | -32.3 ±6.7 | <0.001 | worse than greedy |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.97 (0.97, 4..10) | corruption 2, opposition 94 | 2.02 / 16.76 | 21.0 / 1.50 | 16.41 | 5.48 |
| d0 engine | 9.19 (1.86, 2..10) | corruption 11, deck_empty 26, opposition 59 | 1.10 / 16.37 | 15.2 / 1.81 | 13.62 | 5.36 |
| d4 greedy | 7.42 (1.05, 5..9) | corruption 30, deck_empty 1, opposition 65 | 1.34 / 8.37 | 16.3 / 1.32 | 5.80 | 5.35 |
| d4 engine | 7.73 (1.88, 2..9) | corruption 41, deck_empty 21, opposition 34 | 1.85 / 9.03 | 15.6 / 1.48 | 5.07 | 5.43 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 220.8 | 10.13 | 4% | – | – | 9.52 (44) | 0.00 | 10.23 | 0.00 | 6.82 / 15.94 (43%) |
| d0 engine | 323.4 | 10.39 | 4% | – | – | 13.36 (48) | 0.00 | 14.17 | 0.00 | 12.69 / 18.38 (69%) |
| d4 greedy | 206.5 | 9.42 | 4% | – | – | 10.44 (45) | 0.00 | 4.26 | 0.00 | 0.79 / 14.83 (5%) |
| d4 engine | 263.4 | 9.84 | 4% | – | – | 12.80 (43) | 0.00 | 8.75 | 0.00 | 3.71 / 15.46 (24%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 40% / 21% / 39% | 50% / 15% / 35% |
| d4 | 47% / 10% / 43% | 66% / 2% / 31% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.36 | 3 of 94 | 0.36 |
| d0 engine | 53 of 96 | 0.63 | 39 of 59 | 0.62 |
| d4 greedy | 3 of 96 | 0.36 | 3 of 65 | 0.34 |
| d4 engine | 26 of 96 | 0.55 | 12 of 34 | 0.51 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.91 | 1.80 | 21.4 | 91% (91%) |
| engine | 3.79 | 2.33 | 46.0 | 98% (98%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.84 | 1.43 | 19.1 | 91% (91%) |
| engine | 2.65 | 2.17 | 33.7 | 82% (82%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (248) / 100% / 0% | 1.1 (213) / 99% / 2% |
| civic_shelter | 1.9 (188) / 100% / 0% | 1.8 (187) / 97% / 0% |
| contract_bid | 3.9 (187) / 99% / 0% | 3.3 (1094) / 78% / 94% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 80% |
| data_mining | 3.6 (190) / 100% / 0% | 2.9 (813) / 84% / 78% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.5 (200) / 94% / 1% |
| evacuate | 3.0 (190) / 100% / 1% | 2.8 (264) / 92% / 38% |
| fab_line | 1.9 (189) / 99% / 0% | 1.9 (187) / 98% / 29% |
| firewall_node | 1.8 (188) / 100% / 0% | 1.8 (180) / 99% / 0% |
| fork_process | 2.2 (375) / 100% / 54% | 2.2 (474) / 94% / 63% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 13% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (190) / 99% / 0% | 3.0 (791) / 83% / 80% |
| grid_battery | 2.0 (191) / 100% / 0% | 1.9 (189) / 98% / 10% |
| jam_signals | 3.2 (188) / 100% / 0% | 2.7 (491) / 90% / 65% |
| liquidate_reserves | 3.6 (187) / 99% / 0% | 3.5 (266) / 77% / 15% |
| logistics_ai | 1.2 (212) / 100% / 0% | 1.3 (198) / 99% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.4 (188) / 100% / 0% | 2.4 (190) / 99% / 41% |
| opposition_network | 1.5 (189) / 100% / 0% | 1.5 (180) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (251) / 100% / 0% | 1.0 (226) / 100% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (267) / 68% / 0% |
| precision_strike | 1.6 (894) / 100% / 96% | 1.9 (1048) / 93% / 94% |
| purge_protocol | – (0) / – / 97% | – (0) / – / 76% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (644) / 99% / 95% | 2.3 (835) / 91% / 91% |
| rebuild | 2.0 (257) / 100% / 35% | 1.6 (527) / 97% / 79% |
| recon_sweep | 3.6 (185) / 100% / 0% | 3.5 (242) / 74% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 1% |
| recruitment_hub | 1.5 (190) / 100% / 0% | 1.5 (193) / 98% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.5 (250) / 71% / 1% |
| server_farm | 1.5 (190) / 100% / 88% | 1.5 (183) / 100% / 66% |
| shore_up | 2.9 (244) / 98% / 25% | 2.8 (321) / 85% / 46% |
| solar_field | 2.7 (188) / 100% / 0% | 2.7 (193) / 94% / 69% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (549) / 100% / 82% | 1.4 (940) / 97% / 92% |
| trade_office | 1.2 (210) / 100% / 0% | 1.2 (199) / 99% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (188) / 100% / 0% | 1.8 (182) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 17544.5 | 79.454 | 0.0 | 17629.4 |
| d0 engine | 46112.1 | 142.578 | 0.0 | 46259.8 |
| d4 greedy | 15696.0 | 76.021 | 0.0 | 15775.8 |
| d4 engine | 28898.4 | 109.719 | 0.0 | 28995.3 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-t2_half_t2_noam-d4-engine-seed300085.json`](records/close-loss-c3-baseline-t2_half_t2_noam-d4-engine-seed300085.json): close loss at difficulty 4: deck_empty in round 9, 1 adversary piece(s) over the level-2 bar, pool 11 of 14
- [`records/close-loss-c2-baseline-t2_half_t2_noam-d4-greedy-seed300010.json`](records/close-loss-c2-baseline-t2_half_t2_noam-d4-greedy-seed300010.json): close loss at difficulty 4: deck_empty in round 9, 1 adversary piece(s) over the level-2 bar, pool 13 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-t2_half_t2_noam-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-t2_half_t2_noam-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-t2_half_t2_noam-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-t2_half_t2_noam-d4-greedy-seed300000.json): won by the weakest agent (greedy)

