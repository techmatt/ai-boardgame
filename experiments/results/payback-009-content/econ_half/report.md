# Cycle report: payback-009-c-econ_half

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py econ_half): economy buildings cost half. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 413.0 s. Engine 0.1.0, rules r3, content a0f3f9b1268105da.

- Configs: configs/baseline.json
- Variants: econ_half
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 98%, engine 94%; skill gap -4 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.93, engine 11.76; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 74%, engine 70%; skill gap -4 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.92, engine 9.76; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: -4 → -4 pts win rate, -0.17 → -0.16 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.9 rounds at difficulty 0, 7.0 at 4 (target 8–10).
- Games ended by: corruption 53, deck_empty 9, opposition 322.
- Adversary operations: 10.4 per round on average across cells, at most 45 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 59% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 22% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 98% wins) plays 2.76 buildings and 2.04 tier-2 cards per game (of 20.9 cards); 98% of its wins had 2+ buildings played (98% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 74% wins) plays 2.64 buildings and 1.74 tier-2 cards per game (of 18.1 cards); 94% of its wins had 2+ buildings played (94% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 70% vs greedy 74% over 96 seeds (only engine 12, only greedy 16), -4.2 ±5.5 pts, sign test p 0.572: no better than greedy (behind, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 97.9 ±1.5 (11.93) | 93.8 ±2.5 (11.76) | -4.2 pts (-0.17 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 74.0 ±4.5 (9.92) | 69.8 ±4.7 (9.76) | -4.2 pts (-0.16 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 93.8 | 97.9 | 88 / 2 / 6 / 0 | -4.2 ±2.9 | 0.289 | no better than greedy (behind, not significant) |
| d4 engine | 96 | 69.8 | 74.0 | 55 / 12 / 16 / 13 | -4.2 ±5.5 | 0.572 | no better than greedy (behind, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.85 (0.85, 5..10) | corruption 2, opposition 94 | 2.16 / 16.31 | 9.5 / 2.00 | 15.97 | 5.42 |
| d0 engine | 7.91 (1.97, 2..10) | corruption 4, deck_empty 2, opposition 90 | 2.11 / 17.92 | 23.3 / 1.33 | 17.08 | 5.59 |
| d4 greedy | 7.04 (1.09, 4..9) | corruption 21, deck_empty 4, opposition 71 | 1.89 / 8.32 | 14.4 / 1.40 | 6.44 | 5.43 |
| d4 engine | 7.23 (1.67, 2..9) | corruption 26, deck_empty 3, opposition 67 | 1.63 / 9.64 | 17.5 / 1.34 | 6.92 | 5.65 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 219.5 | 10.26 | 4% | – | – | 9.67 (43) | 0.00 | 10.45 | 0.00 | 6.08 / 15.71 (39%) |
| d0 engine | 270.5 | 9.66 | 4% | – | – | 10.53 (40) | 0.00 | 10.99 | 0.00 | 9.71 / 15.81 (61%) |
| d4 greedy | 198.1 | 9.56 | 4% | – | – | 10.10 (42) | 0.00 | 4.69 | 0.00 | 0.79 / 14.08 (6%) |
| d4 engine | 241.1 | 9.34 | 4% | – | – | 11.35 (45) | 0.00 | 7.01 | 0.00 | 2.29 / 14.46 (16%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 48% / 1% / 51% | 48% / 1% / 51% |
| d4 | 64% / 0% / 36% | 70% / 0% / 30% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 12 of 96 | 0.49 | 12 of 94 | 0.49 |
| d0 engine | 67 of 96 | 0.64 | 65 of 90 | 0.64 |
| d4 greedy | 12 of 96 | 0.50 | 12 of 71 | 0.48 |
| d4 engine | 60 of 96 | 0.64 | 46 of 67 | 0.63 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.76 | 2.04 | 20.9 | 98% (98%) |
| engine | 5.02 | 2.44 | 36.7 | 98% (98%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.64 | 1.74 | 18.1 | 94% (94%) |
| engine | 4.07 | 2.20 | 29.8 | 96% (96%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.8 (194) / 99% / 0% | 1.8 (185) / 99% / 10% |
| civic_shelter | 1.5 (192) / 100% / 0% | 1.5 (189) / 98% / 0% |
| contract_bid | 3.9 (191) / 99% / 0% | 3.3 (927) / 83% / 88% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 33% |
| data_mining | 3.6 (192) / 100% / 0% | 3.1 (629) / 86% / 74% |
| emergency_fab | 2.3 (191) / 100% / 0% | 2.3 (190) / 96% / 0% |
| evacuate | 2.9 (191) / 100% / 0% | 2.8 (248) / 92% / 24% |
| fab_line | 2.3 (192) / 100% / 0% | 2.3 (194) / 93% / 83% |
| firewall_node | 1.4 (196) / 100% / 0% | 1.4 (186) / 100% / 0% |
| fork_process | 2.2 (378) / 100% / 53% | 2.2 (445) / 95% / 59% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 2% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (190) / 98% / 0% | 3.1 (631) / 82% / 72% |
| grid_battery | 2.3 (192) / 100% / 0% | 2.2 (190) / 96% / 34% |
| jam_signals | 3.1 (189) / 100% / 0% | 2.7 (490) / 90% / 65% |
| liquidate_reserves | 3.6 (191) / 99% / 0% | 3.4 (257) / 83% / 16% |
| logistics_ai | 1.0 (249) / 100% / 0% | 1.0 (221) / 99% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.6 (190) / 100% / 44% | 2.6 (191) / 95% / 77% |
| opposition_network | 1.2 (216) / 100% / 0% | 1.2 (197) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.8 (191) / 100% / 0% | 1.8 (192) / 98% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (254) / 73% / 0% |
| precision_strike | 1.6 (858) / 100% / 92% | 1.8 (888) / 94% / 92% |
| purge_protocol | – (0) / – / 93% | – (0) / – / 93% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.1 (644) / 99% / 90% | 2.3 (687) / 92% / 88% |
| rebuild | 1.7 (241) / 100% / 29% | 1.5 (418) / 98% / 67% |
| recon_sweep | 3.6 (188) / 100% / 0% | 3.5 (227) / 79% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 4% |
| recruitment_hub | 1.2 (219) / 100% / 0% | 1.2 (204) / 100% / 0% |
| scavenge | 3.6 (193) / 99% / 0% | 3.5 (257) / 74% / 10% |
| server_farm | 1.2 (194) / 100% / 86% | 1.2 (191) / 98% / 63% |
| shore_up | 2.8 (240) / 100% / 23% | 2.7 (300) / 88% / 33% |
| solar_field | 3.3 (194) / 99% / 40% | 3.2 (221) / 83% / 96% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.4 (579) / 100% / 81% | 1.3 (802) / 97% / 88% |
| trade_office | 2.0 (192) / 100% / 0% | 2.0 (193) / 97% / 6% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.98/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.6 (193) / 100% / 0% | 1.6 (190) / 99% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 12888.9 | 58.731 | 0.0 | 12951.5 |
| d0 engine | 12889.4 | 47.654 | 0.0 | 12937.9 |
| d4 greedy | 10140.8 | 51.195 | 0.0 | 10194.4 |
| d4 engine | 14821.2 | 61.469 | 0.0 | 14881.1 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c2-baseline-econ_half-d4-greedy-seed300027.json`](records/close-loss-c2-baseline-econ_half-d4-greedy-seed300027.json): close loss at difficulty 4: deck_empty in round 9, 2 adversary piece(s) over the level-2 bar, pool 7 of 14
- [`records/close-loss-c3-baseline-econ_half-d4-engine-seed300092.json`](records/close-loss-c3-baseline-econ_half-d4-engine-seed300092.json): close loss at difficulty 4: deck_empty in round 9, 3 adversary piece(s) over the level-2 bar, pool 13 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-econ_half-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-econ_half-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-econ_half-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-econ_half-d4-greedy-seed300000.json): won by the weakest agent (greedy)

