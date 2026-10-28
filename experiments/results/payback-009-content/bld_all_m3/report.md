# Cycle report: payback-009-c-bld_all_m3

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py bld_all_m3): every building costs 3 less. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 398.4 s. Engine 0.1.0, rules r3, content bc339916a95cfffe.

- Configs: configs/baseline.json
- Variants: bld_all_m3
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 93%, engine 92%; skill gap -1 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.72, engine 11.69; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 73%, engine 69%; skill gap -4 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.84, engine 9.72; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: -1 → -4 pts win rate, -0.03 → -0.12 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.9 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 66, deck_empty 5, opposition 313.
- Adversary operations: 10.5 per round on average across cells, at most 48 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 69% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 22% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 93% wins) plays 2.86 buildings and 2.09 tier-2 cards per game (of 21.4 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 73% wins) plays 2.77 buildings and 1.70 tier-2 cards per game (of 18.3 cards); 97% of its wins had 2+ buildings played (97% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 69% vs greedy 73% over 96 seeds (only engine 10, only greedy 14), -4.2 ±5.1 pts, sign test p 0.541: no better than greedy (behind, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 92.7 ±2.7 (11.72) | 91.7 ±2.8 (11.69) | -1.0 pts (-0.03 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 72.9 ±4.5 (9.84) | 68.8 ±4.7 (9.72) | -4.2 pts (-0.12 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 91.7 | 92.7 | 83 / 5 / 6 / 2 | -1.0 ±3.5 | 1.000 | no better than greedy (behind, not significant) |
| d4 engine | 96 | 68.8 | 72.9 | 56 / 10 / 14 / 16 | -4.2 ±5.1 | 0.541 | no better than greedy (behind, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.95 (0.88, 6..10) | corruption 6, deck_empty 1, opposition 89 | 2.07 / 16.54 | 13.7 / 1.71 | 15.43 | 5.33 |
| d0 engine | 8.23 (1.45, 2..10) | corruption 8, opposition 88 | 1.77 / 17.60 | 14.9 / 1.75 | 16.14 | 5.48 |
| d4 greedy | 7.10 (1.12, 5..9) | corruption 23, deck_empty 3, opposition 70 | 1.76 / 8.11 | 13.4 / 1.50 | 6.06 | 5.39 |
| d4 engine | 7.22 (1.49, 2..9) | corruption 29, deck_empty 1, opposition 66 | 1.64 / 10.23 | 17.4 / 1.40 | 7.17 | 5.52 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 222.5 | 10.49 | 4% | – | – | 9.93 (44) | 0.00 | 10.55 | 0.00 | 6.07 / 15.90 (38%) |
| d0 engine | 287.1 | 9.79 | 4% | – | – | 10.76 (48) | 0.00 | 11.30 | 0.00 | 9.92 / 16.46 (60%) |
| d4 greedy | 198.8 | 9.69 | 4% | – | – | 10.35 (40) | 0.00 | 4.73 | 0.00 | 0.81 / 14.21 (6%) |
| d4 engine | 241.4 | 9.35 | 4% | – | – | 10.96 (43) | 0.00 | 6.86 | 0.00 | 2.21 / 14.44 (15%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 60% / 0% / 40% | 63% / 0% / 37% |
| d4 | 70% / 0% / 30% | 75% / 0% / 25% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 9 of 96 | 0.48 | 9 of 89 | 0.48 |
| d0 engine | 87 of 96 | 0.66 | 81 of 88 | 0.66 |
| d4 greedy | 12 of 96 | 0.50 | 12 of 70 | 0.50 |
| d4 engine | 62 of 96 | 0.63 | 48 of 66 | 0.63 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.86 | 2.09 | 21.4 | 100% (100%) |
| engine | 6.12 | 2.61 | 38.9 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.77 | 1.70 | 18.3 | 97% (97%) |
| engine | 4.42 | 2.17 | 30.0 | 98% (98%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (231) / 100% / 0% | 1.1 (207) / 99% / 4% |
| civic_shelter | 2.1 (189) / 100% / 0% | 2.0 (192) / 98% / 26% |
| contract_bid | 3.9 (189) / 100% / 0% | 3.3 (958) / 77% / 89% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 33% |
| data_mining | 3.5 (192) / 100% / 0% | 3.0 (661) / 86% / 73% |
| emergency_fab | 2.2 (192) / 100% / 0% | 2.2 (195) / 96% / 1% |
| evacuate | 2.9 (193) / 100% / 1% | 2.7 (241) / 91% / 17% |
| fab_line | 2.0 (192) / 100% / 0% | 2.0 (190) / 98% / 71% |
| firewall_node | 2.0 (192) / 100% / 0% | 1.9 (191) / 98% / 1% |
| fork_process | 2.2 (388) / 100% / 53% | 2.2 (448) / 94% / 58% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (190) / 100% / 0% | 3.0 (686) / 85% / 73% |
| grid_battery | 2.3 (192) / 100% / 0% | 2.3 (193) / 97% / 38% |
| jam_signals | 3.0 (191) / 99% / 0% | 2.6 (515) / 89% / 65% |
| liquidate_reserves | 3.5 (188) / 100% / 0% | 3.4 (264) / 81% / 16% |
| logistics_ai | 1.2 (208) / 100% / 0% | 1.3 (199) / 98% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.6 (190) / 100% / 43% | 2.6 (191) / 97% / 80% |
| modular_expansion | – (0) / – / 0.00/game | – (0) / – / 0.15/game |
| opposition_network | 1.5 (188) / 100% / 0% | 1.5 (195) / 97% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (222) / 100% / 0% | 1.0 (214) / 100% / 0% |
| point_defense | – (0) / – / 0.00/game | – (0) / – / 0.13/game |
| power_share | 3.5 (192) / 100% / 0% | 3.4 (247) / 77% / 0% |
| precision_strike | 1.6 (877) / 100% / 93% | 1.8 (908) / 94% / 93% |
| purge_protocol | – (0) / – / 93% | – (0) / – / 93% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.1 (640) / 99% / 89% | 2.3 (704) / 92% / 91% |
| rebuild | 1.6 (232) / 100% / 29% | 1.4 (389) / 99% / 66% |
| recon_sweep | 3.6 (185) / 100% / 0% | 3.5 (229) / 81% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.6 (193) / 100% / 0% | 1.6 (189) / 99% / 0% |
| scavenge | 3.5 (192) / 99% / 0% | 3.4 (262) / 78% / 12% |
| server_farm | 1.6 (192) / 100% / 94% | 1.6 (189) / 99% / 88% |
| shore_up | 2.9 (232) / 100% / 21% | 2.7 (311) / 91% / 38% |
| solar_field | 3.7 (190) / 100% / 44% | 3.6 (233) / 80% / 96% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.3 (588) / 100% / 82% | 1.3 (799) / 98% / 88% |
| trade_office | 1.2 (204) / 100% / 0% | 1.2 (205) / 98% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 2.1 (192) / 100% / 0% | 2.1 (192) / 97% / 3% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 6731.2 | 30.253 | 0.0 | 6763.2 |
| d0 engine | 27026.6 | 94.149 | 0.0 | 27117.9 |
| d4 greedy | 6282.5 | 31.604 | 0.0 | 6315.4 |
| d4 engine | 9048.9 | 37.483 | 0.0 | 9083.6 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-bld_all_m3-d4-engine-seed300082.json`](records/close-loss-c3-baseline-bld_all_m3-d4-engine-seed300082.json): close loss at difficulty 4: corruption in round 8, 2 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c3-baseline-bld_all_m3-d4-engine-seed300084.json`](records/close-loss-c3-baseline-bld_all_m3-d4-engine-seed300084.json): close loss at difficulty 4: corruption in round 7, 3 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-bld_all_m3-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-bld_all_m3-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-bld_all_m3-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-bld_all_m3-d4-greedy-seed300000.json): won by the weakest agent (greedy)

