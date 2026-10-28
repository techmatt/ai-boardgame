# Cycle report: payback-009-c-insight_inc+fc4

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py insight_inc): the income buildings also give +1 insight income; with economy.factory_credits=4. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 288.4 s. Engine 0.1.0, rules r3, content 53c5b59a728530fb.

- Configs: configs/baseline.json
- Variants: insight_inc+fc4 (economy.factory_credits=4)
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 97%, engine 99%; skill gap +2 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.91, engine 11.95; the ladder is ordered.
- Difficulty 4: win rates greedy 78%, engine 80%; skill gap +2 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 10.15, engine 10.19; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +2 → +2 pts win rate, +0.04 → +0.04 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 7.9 rounds at difficulty 0, 7.0 at 4 (target 8–10).
- Games ended by: corruption 36, deck_empty 8, opposition 340.
- Adversary operations: 9.8 per round on average across cells, at most 44 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grid_battery, jam_signals, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rally, rebuild, recruitment_hub, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 56% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 39% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine, 99% wins) plays 4.73 buildings and 2.82 tier-2 cards per game (of 40.3 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine, 80% wins) plays 3.79 buildings and 2.45 tier-2 cards per game (of 32.8 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: engine 80% vs greedy 78% over 96 seeds (only engine 15, only greedy 13), +2.1 ±5.5 pts, sign test p 0.851: no better than greedy (ahead, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 96.9 ±1.8 (11.91) | 99.0 ±1.0 (11.95) | +2.1 pts (+0.04 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 78.1 ±4.2 (10.15) | 80.2 ±4.1 (10.19) | +2.1 pts (+0.04 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 99.0 | 96.9 | 92 / 3 / 1 / 0 | +2.1 ±2.1 | 0.625 | no better than greedy (ahead, not significant) |
| d4 engine | 96 | 80.2 | 78.1 | 62 / 15 / 13 / 6 | +2.1 ±5.5 | 0.851 | no better than greedy (ahead, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.76 (0.81, 5..10) | corruption 2, deck_empty 1, opposition 93 | 2.28 / 17.11 | 9.7 / 2.00 | 16.65 | 5.47 |
| d0 engine | 7.89 (1.54, 2..10) | corruption 1, opposition 95 | 2.11 / 18.57 | 38.0 / 1.00 | 18.38 | 5.83 |
| d4 greedy | 7.18 (1.12, 5..9) | corruption 15, deck_empty 6, opposition 75 | 1.80 / 8.29 | 13.8 / 1.38 | 6.93 | 5.50 |
| d4 engine | 7.04 (1.34, 2..9) | corruption 18, deck_empty 1, opposition 77 | 1.92 / 10.81 | 21.6 / 1.21 | 8.81 | 5.71 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 217.9 | 10.46 | 4% | – | – | 9.10 (44) | 0.00 | 9.25 | 0.00 | 5.84 / 15.52 (38%) |
| d0 engine | 279.2 | 10.53 | 4% | – | – | 9.86 (44) | 0.00 | 10.79 | 0.00 | 9.75 / 15.77 (62%) |
| d4 greedy | 201.5 | 9.81 | 4% | – | – | 9.86 (39) | 0.00 | 4.22 | 0.00 | 0.81 / 14.35 (6%) |
| d4 engine | 244.5 | 10.17 | 4% | – | – | 10.39 (38) | 0.00 | 6.60 | 0.00 | 2.38 / 14.08 (17%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 49% / 1% / 51% | 49% / 0% / 51% |
| d4 | 56% / 0% / 44% | 63% / 0% / 37% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.34 | 3 of 93 | 0.34 |
| d0 engine | 48 of 96 | 0.61 | 48 of 95 | 0.61 |
| d4 greedy | 3 of 96 | 0.35 | 3 of 75 | 0.33 |
| d4 engine | 46 of 96 | 0.60 | 41 of 77 | 0.58 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.99 | 1.99 | 20.6 | 99% (99%) |
| engine | 4.73 | 2.82 | 40.3 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.95 | 1.78 | 18.3 | 96% (96%) |
| engine | 3.79 | 2.45 | 32.8 | 99% (99%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (235) / 100% / 0% | 1.1 (211) / 100% / 3% |
| civic_shelter | 1.8 (192) / 100% / 0% | 1.8 (185) / 99% / 0% |
| contract_bid | 4.0 (192) / 99% / 0% | 3.5 (901) / 85% / 91% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 35% |
| data_mining | 3.6 (191) / 100% / 0% | 3.1 (761) / 89% / 82% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.4 (185) / 99% / 1% |
| evacuate | 3.0 (193) / 100% / 1% | 2.8 (245) / 96% / 28% |
| fab_line | 1.9 (194) / 99% / 0% | 1.9 (189) / 100% / 66% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.8 (185) / 100% / 0% |
| fork_process | 2.2 (418) / 100% / 57% | 2.2 (494) / 96% / 64% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (192) / 99% / 0% | 3.1 (784) / 89% / 80% |
| grid_battery | 2.0 (192) / 100% / 0% | 2.0 (185) / 99% / 16% |
| jam_signals | 3.2 (190) / 100% / 0% | 2.7 (480) / 92% / 66% |
| liquidate_reserves | 3.6 (190) / 100% / 0% | 3.5 (263) / 79% / 16% |
| logistics_ai | 1.2 (206) / 100% / 0% | 1.2 (200) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (188) / 100% / 0% | 2.4 (189) / 99% / 84% |
| opposition_network | 1.5 (190) / 100% / 0% | 1.4 (183) / 99% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (222) / 100% / 0% | 1.0 (207) / 100% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (258) / 72% / 0% |
| precision_strike | 1.7 (896) / 100% / 93% | 1.9 (875) / 97% / 95% |
| purge_protocol | – (0) / – / 92% | – (0) / – / 97% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (637) / 99% / 89% | 2.4 (658) / 96% / 93% |
| rebuild | 2.0 (250) / 100% / 31% | 1.7 (444) / 99% / 75% |
| recon_sweep | 3.6 (185) / 100% / 0% | 3.5 (232) / 78% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 1% |
| recruitment_hub | 1.5 (193) / 100% / 0% | 1.5 (187) / 100% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.4 (286) / 78% / 23% |
| server_farm | 1.5 (192) / 100% / 97% | 1.5 (188) / 100% / 64% |
| shore_up | 3.0 (237) / 99% / 22% | 2.8 (311) / 91% / 36% |
| solar_field | 2.8 (190) / 100% / 0% | 2.7 (194) / 95% / 96% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (619) / 100% / 81% | 1.5 (792) / 98% / 91% |
| trade_office | 1.2 (208) / 100% / 0% | 1.2 (199) / 98% / 3% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (191) / 100% / 0% | 1.9 (186) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 6866.4 | 31.505 | 0.0 | 6899.2 |
| d0 engine | 13285.0 | 47.579 | 0.0 | 13330.2 |
| d4 greedy | 5544.3 | 27.519 | 0.0 | 5572.1 |
| d4 engine | 9863.3 | 40.342 | 0.0 | 9897.5 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-insight_inc_fc4-d4-engine-seed300080.json`](records/close-loss-c3-baseline-insight_inc_fc4-d4-engine-seed300080.json): close loss at difficulty 4: deck_empty in round 9, 2 adversary piece(s) over the level-2 bar, pool 14 of 14
- [`records/close-loss-c2-baseline-insight_inc_fc4-d4-greedy-seed300071.json`](records/close-loss-c2-baseline-insight_inc_fc4-d4-greedy-seed300071.json): close loss at difficulty 4: deck_empty in round 9, 3 adversary piece(s) over the level-2 bar, pool 12 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-insight_inc_fc4-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-insight_inc_fc4-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-insight_inc_fc4-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-insight_inc_fc4-d4-greedy-seed300000.json): won by the weakest agent (greedy)

