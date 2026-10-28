# Cycle report: payback-009-c-insight_inc+keep0

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py insight_inc): the income buildings also give +1 insight income; with cards.keep_cost=0. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 522.0 s. Engine 0.1.0, rules r3, content 53c5b59a728530fb.

- Configs: configs/baseline.json
- Variants: insight_inc+keep0 (cards.keep_cost=0)
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 96%, engine 97%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.89, engine 11.94; the ladder is ordered.
- Difficulty 4: win rates greedy 82%, engine 89%; skill gap +6 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 10.32, engine 10.52; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +1 → +6 pts win rate, +0.05 → +0.20 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 7.8 rounds at difficulty 0, 7.0 at 4 (target 8–10).
- Games ended by: corruption 24, deck_empty 11, opposition 349.
- Adversary operations: 9.4 per round on average across cells, at most 42 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 56% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 40% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine, 97% wins) plays 5.05 buildings and 2.71 tier-2 cards per game (of 43.4 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine, 89% wins) plays 4.12 buildings and 2.42 tier-2 cards per game (of 35.0 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: engine 89% vs greedy 82% over 96 seeds (only engine 13, only greedy 7), +6.2 ±4.6 pts, sign test p 0.263: no better than greedy (ahead, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.89) | 96.9 ±1.8 (11.94) | +1.0 pts (+0.05 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 82.3 ±3.9 (10.32) | 88.5 ±3.3 (10.52) | +6.2 pts (+0.20 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 96.9 | 95.8 | 89 / 4 / 3 / 0 | +1.0 ±2.8 | 1.000 | no better than greedy (ahead, not significant) |
| d4 engine | 96 | 88.5 | 82.3 | 72 / 13 / 7 / 4 | +6.2 ±4.6 | 0.263 | no better than greedy (ahead, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.74 (0.97, 4..10) | corruption 2, deck_empty 2, opposition 92 | 2.33 / 17.24 | 8.2 / 2.00 | 16.73 | 5.54 |
| d0 engine | 7.80 (1.71, 2..10) | deck_empty 3, opposition 93 | 2.27 / 19.27 | 4.7 / 2.00 | 19.14 | 5.76 |
| d4 greedy | 7.14 (1.13, 4..9) | corruption 12, deck_empty 5, opposition 79 | 1.87 / 8.80 | 12.4 / 1.47 | 7.51 | 5.61 |
| d4 engine | 6.98 (1.35, 2..9) | corruption 10, deck_empty 1, opposition 85 | 2.00 / 10.73 | 11.9 / 1.55 | 9.64 | 5.82 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 217.4 | 10.80 | 4% | – | – | 8.93 (42) | 0.00 | 9.31 | 0.00 | 5.96 / 15.48 (38%) |
| d0 engine | 280.4 | 10.91 | 4% | – | – | 9.41 (41) | 0.00 | 10.71 | 0.00 | 9.86 / 15.60 (63%) |
| d4 greedy | 202.5 | 10.16 | 4% | – | – | 9.50 (35) | 0.00 | 4.19 | 0.00 | 0.86 / 14.27 (6%) |
| d4 engine | 244.7 | 10.50 | 4% | – | – | 9.68 (36) | 0.00 | 6.60 | 0.00 | 2.33 / 13.96 (17%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 48% / 0% / 52% | 48% / 0% / 51% |
| d4 | 58% / 1% / 41% | 64% / 0% / 36% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 2 of 96 | 0.34 | 2 of 92 | 0.34 |
| d0 engine | 46 of 96 | 0.60 | 44 of 93 | 0.60 |
| d4 greedy | 2 of 96 | 0.34 | 2 of 79 | 0.33 |
| d4 engine | 47 of 96 | 0.61 | 44 of 85 | 0.61 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.00 | 1.93 | 20.7 | 100% (100%) |
| engine | 5.05 | 2.71 | 43.4 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.00 | 1.71 | 18.9 | 100% (100%) |
| engine | 4.12 | 2.42 | 35.0 | 99% (99%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (233) / 100% / 0% | 1.1 (211) / 100% / 3% |
| civic_shelter | 1.8 (192) / 100% / 0% | 1.9 (182) / 100% / 0% |
| contract_bid | 4.0 (190) / 100% / 0% | 3.4 (917) / 100% / 91% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 31% |
| data_mining | 3.6 (190) / 100% / 0% | 3.0 (759) / 100% / 83% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.5 (186) / 100% / 1% |
| evacuate | 3.0 (192) / 100% / 0% | 2.9 (232) / 100% / 24% |
| fab_line | 1.9 (192) / 100% / 0% | 1.9 (186) / 100% / 77% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.8 (185) / 100% / 0% |
| fork_process | 2.2 (419) / 100% / 57% | 2.2 (493) / 100% / 66% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (190) / 100% / 0% | 3.1 (781) / 100% / 80% |
| grid_battery | 2.0 (192) / 100% / 0% | 2.0 (183) / 100% / 20% |
| jam_signals | 3.2 (191) / 100% / 0% | 2.8 (471) / 100% / 68% |
| liquidate_reserves | 3.6 (188) / 100% / 0% | 3.5 (205) / 100% / 11% |
| logistics_ai | 1.2 (205) / 100% / 0% | 1.2 (195) / 100% / 1% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (188) / 100% / 0% | 2.3 (184) / 100% / 88% |
| opposition_network | 1.5 (190) / 100% / 0% | 1.5 (182) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (218) / 100% / 0% | 1.0 (208) / 100% / 0% |
| power_share | 3.6 (192) / 100% / 0% | 3.6 (188) / 100% / 0% |
| precision_strike | 1.7 (926) / 100% / 93% | 1.9 (885) / 100% / 96% |
| purge_protocol | – (0) / – / 92% | – (0) / – / 98% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (646) / 100% / 89% | 2.4 (634) / 100% / 93% |
| rebuild | 2.0 (253) / 100% / 30% | 1.7 (478) / 100% / 80% |
| recon_sweep | 3.6 (185) / 100% / 0% | 3.7 (182) / 100% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 3% |
| recruitment_hub | 1.5 (193) / 100% / 0% | 1.5 (185) / 100% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.4 (213) / 100% / 17% |
| server_farm | 1.5 (192) / 100% / 100% | 1.5 (184) / 100% / 76% |
| shore_up | 3.0 (237) / 100% / 22% | 2.9 (272) / 100% / 33% |
| solar_field | 2.8 (190) / 100% / 0% | 2.7 (189) / 100% / 99% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (624) / 100% / 82% | 1.4 (804) / 100% / 92% |
| trade_office | 1.2 (208) / 100% / 0% | 1.2 (197) / 100% / 3% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (192) / 100% / 0% | 1.9 (186) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 6920.0 | 31.824 | 0.0 | 6952.7 |
| d0 engine | 34464.0 | 122.921 | 0.0 | 34590.8 |
| d4 greedy | 5706.9 | 28.187 | 0.0 | 5736.2 |
| d4 engine | 17408.2 | 71.151 | 0.0 | 17469.4 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c2-baseline-insight_inc_keep0-d4-greedy-seed300041.json`](records/close-loss-c2-baseline-insight_inc_keep0-d4-greedy-seed300041.json): close loss at difficulty 4: deck_empty in round 9, 4 adversary piece(s) over the level-2 bar, pool 13 of 14
- [`records/close-loss-c3-baseline-insight_inc_keep0-d4-engine-seed300043.json`](records/close-loss-c3-baseline-insight_inc_keep0-d4-engine-seed300043.json): close loss at difficulty 4: corruption in round 7, 4 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-insight_inc_keep0-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-insight_inc_keep0-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-insight_inc_keep0-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-insight_inc_keep0-d4-greedy-seed300000.json): won by the weakest agent (greedy)

