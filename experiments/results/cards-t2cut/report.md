# Cycle report: cards-t2cut

007 card health: tier-2 costs cut ~30% (content edit, separate binary), same seeds as tune-agents-007.

640 games (4 cells × 160 seeds, 100000..=100159) on 12 threads in 473.7 s. Engine 0.1.0, rules r3, content 22f6d4c1b4fd46ab.

- Configs: configs/baseline.json
- Variants: base
- Difficulties: 0, 4
- Agents (weakest first): greedy, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 96%, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 93%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.86, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 11.79; the ladder is ordered.
- Difficulty 4: win rates greedy 70%, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 64%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.68, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 9.39; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +0 → +0 pts win rate, +0.00 → +0.00 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 105, deck_empty 17, opposition 518.
- Adversary operations: 10.5 per round on average across cells, at most 55 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 76% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.29 gifts per game; 22% of threats countered.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | skill gap |
|---|---|---|---|
| win % (survival) | 96.2 ±1.5 (11.86) | 93.1 ±2.0 (11.79) | +0.0 pts (+0.00 survival) |

| difficulty 4 | greedy | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | skill gap |
|---|---|---|---|
| win % (survival) | 70.0 ±3.6 (9.68) | 64.4 ±3.8 (9.39) | +0.0 pts (+0.00 survival) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.79 (1.04, 3..10) | corruption 4, deck_empty 2, opposition 154 | 2.23 / 16.64 | 11.8 / 1.83 | 16.18 | 5.38 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 7.94 (1.14, 3..10) | corruption 5, deck_empty 6, opposition 149 | 2.13 / 17.54 | 15.3 / 1.91 | 16.76 | 5.42 |
| d4 greedy | 7.11 (1.17, 3..9) | corruption 45, deck_empty 3, opposition 112 | 1.67 / 8.72 | 14.7 / 1.31 | 6.22 | 5.44 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 7.03 (1.13, 5..9) | corruption 51, deck_empty 6, opposition 103 | 1.68 / 9.46 | 18.4 / 1.25 | 6.41 | 5.46 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 215.4 | 10.04 | 4% | – | – | 9.70 (42) | 0.00 | 9.36 | 0.00 | 5.79 / 15.59 (37%) |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 274.8 | 8.41 | 4% | – | – | 10.96 (55) | 0.71 | 9.60 | 0.00 | 6.39 / 15.88 (40%) |
| d4 greedy | 197.5 | 9.23 | 4% | – | – | 10.10 (43) | 0.00 | 4.12 | 0.00 | 0.77 / 14.22 (5%) |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 238.4 | 8.11 | 4% | – | – | 11.07 (51) | 0.46 | 3.98 | 0.00 | 0.47 / 14.05 (3%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 60% / 0% / 40% | 75% / 0% / 25% |
| d4 | 70% / 0% / 30% | 77% / 0% / 22% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 160 | 0.36 | 3 of 154 | 0.36 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 160 of 160 | 0.66 | 149 of 149 | 0.66 |
| d4 greedy | 3 of 160 | 0.37 | 3 of 112 | 0.37 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 160 of 160 | 0.66 | 103 of 103 | 0.65 |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 8% |
| assembly_cluster | 1.1 (364) / 100% / 0% | 1.1 (343) / 100% / 55% |
| civic_shelter | 2.0 (316) / 100% / 0% | 2.0 (316) / 99% / 42% |
| contract_bid | 3.9 (317) / 100% / 0% | 2.8 (1610) / 98% / 88% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 0% |
| data_mining | 3.6 (316) / 99% / 0% | 2.8 (911) / 99% / 71% |
| emergency_fab | 2.5 (315) / 100% / 0% | 2.5 (316) / 100% / 0% |
| evacuate | 3.0 (316) / 99% / 1% | 2.9 (362) / 98% / 16% |
| fab_line | 1.7 (315) / 100% / 0% | 1.7 (314) / 100% / 56% |
| firewall_node | 1.8 (317) / 100% / 0% | 1.8 (315) / 100% / 56% |
| fork_process | 2.1 (637) / 99% / 55% | 2.0 (701) / 100% / 58% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 21% |
| global_mandate | – (0) / – / 0% | – (0) / – / 32% |
| grant | 3.6 (318) / 100% / 0% | 3.0 (720) / 100% / 58% |
| grid_battery | 2.1 (310) / 100% / 0% | 2.1 (313) / 99% / 42% |
| jam_signals | 3.3 (315) / 100% / 0% | 2.8 (563) / 98% / 44% |
| liquidate_reserves | 3.6 (322) / 99% / 0% | 3.6 (339) / 96% / 0% |
| logistics_ai | 1.2 (351) / 100% / 0% | 1.2 (337) / 100% / 45% |
| megastructure | – (0) / – / 0% | – (0) / – / 8% |
| microgrid | 2.4 (315) / 100% / 0% | 2.4 (311) / 100% / 39% |
| modular_expansion | – (0) / – / 0.00/game | – (0) / – / 1.00/game |
| opposition_network | 1.5 (322) / 100% / 0% | 1.5 (321) / 100% / 54% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 10% |
| overclock_rig | 1.0 (405) / 100% / 0% | 1.0 (369) / 100% / 45% |
| point_defense | – (0) / – / 0.00/game | – (0) / – / 0.79/game |
| power_share | 3.5 (310) / 100% / 0% | 3.5 (339) / 98% / 0% |
| precision_strike | 1.6 (1440) / 100% / 92% | 1.7 (1389) / 99% / 88% |
| purge_protocol | – (0) / – / 94% | – (0) / – / 89% |
| quantum_core | – (0) / – / 0% | – (0) / – / 12% |
| rally | 2.2 (1053) / 99% / 90% | 2.3 (926) / 99% / 83% |
| rebuild | 2.0 (404) / 100% / 31% | 1.8 (487) / 99% / 45% |
| recon_sweep | 3.6 (314) / 99% / 0% | 3.6 (330) / 99% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 36% |
| recruitment_hub | 1.4 (315) / 100% / 0% | 1.5 (323) / 99% / 47% |
| scavenge | 3.6 (316) / 99% / 0% | 3.6 (333) / 98% / 0% |
| server_farm | 1.5 (317) / 100% / 88% | 1.5 (315) / 100% / 70% |
| shore_up | 2.8 (415) / 99% / 26% | 2.7 (424) / 98% / 28% |
| solar_field | 2.7 (315) / 100% / 0% | 2.4 (421) / 99% / 60% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 38% |
| sweep | 1.5 (931) / 100% / 82% | 1.5 (942) / 99% / 74% |
| trade_office | 1.3 (339) / 100% / 0% | 1.3 (337) / 99% / 49% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (318) / 100% / 0% | 1.9 (319) / 99% / 51% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 8741.9 | 40.586 | 0.0 | 8783.4 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 12336.4 | 44.893 | 0.0 | 12379.2 |
| d4 greedy | 5390.6 | 27.294 | 0.0 | 5419.9 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 8281.9 | 34.738 | 0.0 | 8314.0 |

## Saved records

None matched.
