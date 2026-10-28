# Cycle report: tune-agents-007

007: which greedy is strongest on the tuned baseline, same seeds. greedy (default weights) never plays buildings; built_cost 1.2 makes it build; hand 1 / hand_cost 0.2 also makes it play economy programs. Also the card-health data.

960 games (6 cells × 160 seeds, 100000..=100159) on 12 threads in 642.8 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: base
- Difficulties: 0, 4
- Agents (weakest first): greedy, greedy:w.built_cost=1.2, greedy:w.hand=1,w.hand_cost=0.2,w.built_cost=1.2
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 96%, greedy:w.built_cost=1.2 91%, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 95%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.84, greedy:w.built_cost=1.2 11.66, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 11.82; the ladder is ordered.
- Difficulty 4: win rates greedy 74%, greedy:w.built_cost=1.2 64%, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 69%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.82, greedy:w.built_cost=1.2 9.32, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 9.56; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +0 → +0 pts win rate, +0.00 → +0.00 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.7 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 159, deck_empty 17, opposition 783, population 1.
- Adversary operations: 10.6 per round on average across cells, at most 54 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 66% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.34 gifts per game; 20% of threats countered.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | greedy:w.built_cost=1.2 | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | skill gap |
|---|---|---|---|---|
| win % (survival) | 96.2 ±1.5 (11.84) | 91.2 ±2.2 (11.66) | 95.0 ±1.7 (11.82) | +0.0 pts (+0.00 survival) |

| difficulty 4 | greedy | greedy:w.built_cost=1.2 | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | skill gap |
|---|---|---|---|---|
| win % (survival) | 73.8 ±3.5 (9.82) | 63.7 ±3.8 (9.32) | 69.4 ±3.6 (9.56) | +0.0 pts (+0.00 survival) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.72 (1.01, 3..10) | corruption 5, deck_empty 1, opposition 154 | 2.29 / 16.94 | 11.8 / 1.83 | 16.34 | 5.40 |
| d0 greedy:w.built_cost=1.2 | 7.62 (1.10, 4..10) | corruption 10, deck_empty 3, opposition 146, population 1 | 2.42 / 16.06 | 14.9 / 1.86 | 14.91 | 5.07 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 7.85 (1.12, 3..10) | corruption 6, deck_empty 2, opposition 152 | 2.18 / 17.24 | 15.4 / 1.88 | 16.49 | 5.41 |
| d4 greedy | 7.13 (1.18, 3..9) | corruption 39, deck_empty 3, opposition 118 | 1.65 / 8.43 | 14.8 / 1.29 | 6.34 | 5.47 |
| d4 greedy:w.built_cost=1.2 | 6.94 (1.10, 5..9) | corruption 55, deck_empty 3, opposition 102 | 1.75 / 8.94 | 17.1 / 1.31 | 5.84 | 5.32 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 7.04 (1.15, 5..9) | corruption 44, deck_empty 5, opposition 111 | 1.63 / 8.88 | 17.5 / 1.29 | 6.38 | 5.46 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 213.8 | 10.06 | 4% | – | – | 9.55 (42) | 0.00 | 9.24 | 0.00 | 5.61 / 15.44 (36%) |
| d0 greedy:w.built_cost=1.2 | 222.2 | 8.96 | 4% | – | – | 11.09 (54) | 0.71 | 9.49 | 0.00 | 4.92 / 15.25 (32%) |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 270.6 | 8.33 | 4% | – | – | 10.94 (45) | 0.48 | 9.50 | 0.00 | 6.12 / 15.70 (39%) |
| d4 greedy | 197.3 | 9.30 | 4% | – | – | 10.02 (43) | 0.00 | 4.12 | 0.00 | 0.77 / 14.26 (5%) |
| d4 greedy:w.built_cost=1.2 | 206.7 | 8.62 | 4% | – | – | 10.92 (45) | 0.47 | 3.69 | 0.00 | 0.28 / 13.88 (2%) |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 238.8 | 8.07 | 4% | – | – | 11.02 (51) | 0.38 | 3.97 | 0.00 | 0.45 / 14.09 (3%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 49% / 1% / 50% | 59% / 1% / 40% |
| d4 | 64% / 1% / 35% | 73% / 1% / 26% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 160 | 0.36 | 3 of 154 | 0.36 |
| d0 greedy:w.built_cost=1.2 | 159 of 160 | 0.67 | 145 of 146 | 0.67 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 160 of 160 | 0.68 | 152 of 152 | 0.68 |
| d4 greedy | 3 of 160 | 0.37 | 3 of 118 | 0.36 |
| d4 greedy:w.built_cost=1.2 | 160 of 160 | 0.65 | 102 of 102 | 0.64 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 160 of 160 | 0.67 | 111 of 111 | 0.67 |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | greedy:w.built_cost=1.2: pick (offered) / keep / play | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2: pick (offered) / keep / play |
|---|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 12% | – (0) / – / 15% |
| assembly_cluster | 1.1 (365) / 100% / 0% | 1.1 (366) / 100% / 50% | 1.1 (348) / 100% / 53% |
| civic_shelter | 2.0 (317) / 100% / 0% | 2.0 (321) / 99% / 28% | 2.0 (320) / 99% / 42% |
| contract_bid | 4.0 (317) / 100% / 0% | 3.9 (331) / 99% / 0% | 2.8 (1595) / 97% / 87% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% |
| data_mining | 3.6 (316) / 99% / 0% | 3.6 (323) / 99% / 0% | 2.8 (909) / 99% / 69% |
| emergency_fab | 2.5 (316) / 100% / 0% | 2.5 (320) / 100% / 0% | 2.5 (316) / 100% / 0% |
| evacuate | 3.0 (316) / 99% / 1% | 3.0 (328) / 98% / 0% | 2.9 (359) / 97% / 13% |
| fab_line | 1.7 (316) / 100% / 0% | 1.7 (321) / 100% / 41% | 1.7 (313) / 100% / 56% |
| firewall_node | 1.8 (318) / 100% / 0% | 1.8 (319) / 100% / 45% | 1.8 (316) / 100% / 55% |
| fork_process | 2.1 (642) / 99% / 55% | 2.2 (500) / 100% / 41% | 2.0 (674) / 100% / 56% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 9% | – (0) / – / 27% |
| global_mandate | – (0) / – / 0% | – (0) / – / 23% | – (0) / – / 30% |
| grant | 3.6 (318) / 100% / 0% | 3.6 (321) / 99% / 0% | 3.0 (717) / 100% / 58% |
| grid_battery | 2.1 (310) / 100% / 0% | 2.1 (321) / 99% / 28% | 2.1 (314) / 99% / 45% |
| jam_signals | 3.3 (316) / 100% / 0% | 3.3 (325) / 98% / 0% | 2.8 (538) / 99% / 42% |
| liquidate_reserves | 3.6 (324) / 99% / 0% | 3.6 (329) / 97% / 0% | 3.6 (342) / 96% / 0% |
| logistics_ai | 1.2 (349) / 100% / 0% | 1.2 (350) / 99% / 41% | 1.2 (333) / 100% / 43% |
| megastructure | – (0) / – / 0% | – (0) / – / 4% | – (0) / – / 6% |
| microgrid | 2.4 (315) / 100% / 0% | 2.4 (321) / 99% / 22% | 2.4 (316) / 100% / 39% |
| modular_expansion | – (0) / – / 0.00/game | – (0) / – / 0.94/game | – (0) / – / 0.99/game |
| opposition_network | 1.5 (322) / 100% / 0% | 1.5 (323) / 100% / 51% | 1.5 (321) / 100% / 52% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 6% | – (0) / – / 13% |
| overclock_rig | 1.0 (396) / 100% / 0% | 1.0 (396) / 100% / 46% | 1.0 (370) / 100% / 44% |
| point_defense | – (0) / – / 0.00/game | – (0) / – / 0.66/game | – (0) / – / 0.78/game |
| power_share | 3.5 (311) / 100% / 0% | 3.5 (333) / 96% / 0% | 3.5 (341) / 98% / 0% |
| precision_strike | 1.6 (1427) / 100% / 92% | 1.7 (1219) / 99% / 89% | 1.7 (1373) / 99% / 87% |
| purge_protocol | – (0) / – / 95% | – (0) / – / 90% | – (0) / – / 88% |
| quantum_core | – (0) / – / 0% | – (0) / – / 17% | – (0) / – / 10% |
| rally | 2.2 (1051) / 99% / 89% | 2.2 (910) / 99% / 83% | 2.3 (924) / 99% / 82% |
| rebuild | 2.0 (397) / 99% / 28% | 2.0 (363) / 99% / 16% | 1.8 (475) / 99% / 42% |
| recon_sweep | 3.6 (316) / 99% / 0% | 3.6 (321) / 99% / 0% | 3.6 (332) / 99% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 23% | – (0) / – / 31% |
| recruitment_hub | 1.4 (315) / 100% / 0% | 1.4 (323) / 99% / 42% | 1.5 (324) / 99% / 47% |
| scavenge | 3.6 (317) / 99% / 0% | 3.6 (324) / 98% / 0% | 3.5 (338) / 98% / 1% |
| server_farm | 1.5 (317) / 100% / 88% | 1.5 (323) / 100% / 67% | 1.5 (316) / 100% / 69% |
| shore_up | 2.8 (419) / 99% / 25% | 2.9 (375) / 98% / 15% | 2.7 (429) / 98% / 27% |
| solar_field | 2.7 (316) / 100% / 0% | 2.6 (329) / 98% / 75% | 2.4 (424) / 99% / 64% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 0.99/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 41% | – (0) / – / 37% |
| sweep | 1.5 (935) / 100% / 82% | 1.6 (793) / 99% / 72% | 1.5 (933) / 100% / 74% |
| trade_office | 1.3 (340) / 100% / 0% | 1.3 (338) / 99% / 46% | 1.3 (333) / 99% / 49% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (317) / 100% / 0% | 1.9 (322) / 99% / 39% | 1.9 (319) / 99% / 51% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 7055.8 | 33.007 | 0.0 | 7090.8 |
| d0 greedy:w.built_cost=1.2 | 8541.7 | 38.447 | 0.0 | 8576.5 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 11038.7 | 40.796 | 0.0 | 11078.8 |
| d4 greedy | 5208.7 | 26.403 | 0.0 | 5237.4 |
| d4 greedy:w.built_cost=1.2 | 7207.3 | 34.875 | 0.0 | 7235.6 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 8444.5 | 35.363 | 0.0 | 8477.6 |

## Saved records

None matched.
