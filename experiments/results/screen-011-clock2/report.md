# Cycle report: screen-011-clock2

011 screen (no probes) on the 009 content patch t2_half+t2_noam (cheap tier 2): the deck clock counting only the action phases after this one. Engine agent at w.clock 10 and 30, d0 and d4, 009's seeds; compare screen-011-clock (greedy 100 / 66.7, engine without the clock 60.4 / 31.2).

192 games (4 cells × 48 seeds, 300000..=300047) on 12 threads in 408.1 s. Engine 0.1.0, rules r4, content 2f31b40bb42daaae.

- Configs: configs/baseline.json
- Variants: base
- Difficulties: 0, 4
- Agents (weakest first): engine:w.clock=10, engine:w.clock=30
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates engine:w.clock=10 56%, engine:w.clock=30 62%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) engine:w.clock=10 10.92, engine:w.clock=30 11.02; the ladder is ordered.
- Difficulty 4: win rates engine:w.clock=10 46%, engine:w.clock=30 48%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) engine:w.clock=10 9.10, engine:w.clock=30 9.12; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +0 → +0 pts win rate, +0.00 → +0.00 survival (target: widens with difficulty).
- Game length for the strongest agent (engine:w.clock=30): 9.5 rounds at difficulty 0, 7.9 at 4 (target 8–10).
- Games ended by: corruption 51, deck_empty 39, opposition 102.
- Adversary operations: 12.2 per round on average across cells, at most 41 in one round (target: low).
- Cards (engine:w.clock=30, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grid_battery, jam_signals, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rebuild, recruitment_hub, server_farm, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 97% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 46% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine:w.clock=30, 62% wins) plays 3.81 buildings and 2.17 tier-2 cards per game (of 47.0 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine:w.clock=30, 48% wins) plays 2.83 buildings and 2.17 tier-2 cards per game (of 33.4 cards); 96% of its wins had 2+ buildings played (96% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | engine:w.clock=10 | engine:w.clock=30 | skill gap |
|---|---|---|---|
| win % (survival) | 56.2 ±7.2 (10.92) | 62.5 ±7.0 (11.02) | +0.0 pts (+0.00 survival) |

| difficulty 4 | engine:w.clock=10 | engine:w.clock=30 | skill gap |
|---|---|---|---|
| win % (survival) | 45.8 ±7.2 (9.10) | 47.9 ±7.2 (9.12) | +0.0 pts (+0.00 survival) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 engine:w.clock=10 | 9.52 (0.74, 8..10) | corruption 7, deck_empty 14, opposition 27 | 0.48 / 17.00 | 13.9 / 1.86 | 13.40 | 5.44 |
| d0 engine:w.clock=30 | 9.50 (0.79, 7..10) | corruption 6, deck_empty 12, opposition 30 | 0.43 / 16.73 | 13.6 / 1.83 | 13.98 | 5.44 |
| d4 engine:w.clock=10 | 7.96 (1.22, 5..9) | corruption 19, deck_empty 7, opposition 22 | 0.50 / 9.36 | 17.7 / 1.35 | 5.23 | 5.44 |
| d4 engine:w.clock=30 | 7.94 (1.23, 5..9) | corruption 19, deck_empty 6, opposition 23 | 0.48 / 9.30 | 18.7 / 1.28 | 5.38 | 5.46 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 engine:w.clock=10 | 333.8 | 10.51 | 4% | – | – | 12.80 (41) | 0.00 | 14.69 | 0.00 | 13.40 / 19.04 (70%) |
| d0 engine:w.clock=30 | 332.1 | 10.50 | 4% | – | – | 12.60 (41) | 0.00 | 14.65 | 0.00 | 13.27 / 19.00 (70%) |
| d4 engine:w.clock=10 | 268.5 | 9.87 | 4% | – | – | 11.71 (37) | 0.00 | 9.21 | 0.00 | 3.67 / 15.92 (23%) |
| d4 engine:w.clock=30 | 267.2 | 9.86 | 4% | – | – | 11.61 (37) | 0.00 | 9.17 | 0.00 | 3.67 / 15.88 (23%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 89% / 0% / 10% | 94% / 0% / 6% |
| d4 | 98% / 0% / 2% | 99% / 0% / 1% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 engine:w.clock=10 | 31 of 48 | 0.60 | 20 of 27 | 0.59 |
| d0 engine:w.clock=30 | 31 of 48 | 0.60 | 21 of 30 | 0.61 |
| d4 engine:w.clock=10 | 20 of 48 | 0.54 | 13 of 22 | 0.52 |
| d4 engine:w.clock=30 | 20 of 48 | 0.54 | 14 of 23 | 0.53 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| engine:w.clock=10 | 3.81 | 2.19 | 47.4 | 100% (100%) |
| engine:w.clock=30 | 3.81 | 2.17 | 47.0 | 100% (100%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| engine:w.clock=10 | 2.83 | 2.17 | 33.6 | 95% (95%) |
| engine:w.clock=30 | 2.83 | 2.17 | 33.4 | 96% (96%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | engine:w.clock=10: pick (offered) / keep / play | engine:w.clock=30: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (108) / 97% / 2% | 1.1 (109) / 97% / 2% |
| civic_shelter | 1.8 (95) / 96% / 0% | 1.8 (95) / 96% / 0% |
| contract_bid | 3.3 (570) / 78% / 92% | 3.3 (568) / 78% / 92% |
| counter_intelligence | – (0) / – / 81% | – (0) / – / 81% |
| data_mining | 2.9 (433) / 85% / 80% | 2.9 (436) / 85% / 80% |
| emergency_fab | 2.4 (103) / 92% / 0% | 2.4 (103) / 92% / 0% |
| evacuate | 2.7 (123) / 96% / 34% | 2.8 (124) / 96% / 34% |
| fab_line | 1.9 (97) / 99% / 26% | 1.9 (97) / 99% / 26% |
| firewall_node | 1.7 (89) / 100% / 0% | 1.7 (89) / 100% / 0% |
| fork_process | 2.2 (254) / 95% / 64% | 2.2 (253) / 95% / 64% |
| fusion_plant | – (0) / – / 12% | – (0) / – / 12% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.0 (420) / 83% / 79% | 3.0 (418) / 83% / 80% |
| grid_battery | 2.0 (97) / 99% / 10% | 2.0 (97) / 99% / 10% |
| jam_signals | 2.6 (259) / 92% / 65% | 2.6 (260) / 92% / 65% |
| liquidate_reserves | 3.4 (140) / 79% / 19% | 3.4 (140) / 79% / 19% |
| logistics_ai | 1.3 (97) / 100% / 0% | 1.3 (97) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (98) / 97% / 40% | 2.3 (98) / 97% / 40% |
| opposition_network | 1.5 (92) / 100% / 0% | 1.5 (92) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (119) / 99% / 0% | 1.0 (119) / 99% / 0% |
| power_share | 3.5 (138) / 68% / 0% | 3.5 (138) / 68% / 0% |
| precision_strike | 1.8 (553) / 92% / 94% | 1.8 (551) / 93% / 94% |
| purge_protocol | – (0) / – / 82% | – (0) / – / 85% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.3 (436) / 88% / 93% | 2.3 (435) / 89% / 94% |
| rebuild | 1.7 (239) / 97% / 78% | 1.7 (242) / 96% / 79% |
| recon_sweep | 3.5 (126) / 74% / 0% | 3.5 (126) / 74% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.6 (100) / 96% / 0% | 1.6 (100) / 96% / 0% |
| scavenge | 3.4 (129) / 71% / 0% | 3.4 (129) / 71% / 0% |
| server_farm | 1.4 (96) / 99% / 72% | 1.4 (96) / 99% / 72% |
| shore_up | 2.8 (169) / 85% / 47% | 2.8 (169) / 85% / 46% |
| solar_field | 2.6 (96) / 97% / 72% | 2.6 (96) / 97% / 72% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.4 (472) / 96% / 92% | 1.4 (468) / 96% / 91% |
| trade_office | 1.3 (103) / 99% / 0% | 1.3 (103) / 99% / 0% |
| turnkey_factory | – (0) / – / 0.99/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.7 (95) / 97% / 0% | 1.7 (95) / 97% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 engine:w.clock=10 | 34145.6 | 102.296 | 0.0 | 34255.0 |
| d0 engine:w.clock=30 | 30830.9 | 92.841 | 0.0 | 30928.4 |
| d4 engine:w.clock=10 | 22825.3 | 85.024 | 0.0 | 22905.6 |
| d4 engine:w.clock=30 | 12951.5 | 48.462 | 0.0 | 12997.6 |

## Saved records

None matched.
