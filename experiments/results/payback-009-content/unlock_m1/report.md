# Cycle report: payback-009-c-unlock_m1

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py unlock_m1): the AI tracks' tier-2 unlock one value earlier. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 708.7 s. Engine 0.1.0, rules r3, content 7b3aef411eccbe5b.

- Configs: configs/baseline.json
- Variants: unlock_m1
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 97%, engine 86%; skill gap -10 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.91, engine 11.59; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 69%, engine 66%; skill gap -3 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.76, engine 9.70; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: -10 → -3 pts win rate, -0.31 → -0.06 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 8.0 rounds at difficulty 0, 7.2 at 4 (target 8–10).
- Games ended by: corruption 67, deck_empty 12, opposition 305.
- Adversary operations: 10.7 per round on average across cells, at most 45 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 66% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 21% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 97% wins) plays 1.93 buildings and 1.98 tier-2 cards per game (of 20.7 cards); 94% of its wins had 2+ buildings played (94% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 69% wins) plays 1.85 buildings and 1.81 tier-2 cards per game (of 17.3 cards); 88% of its wins had 2+ buildings played (88% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 66% vs greedy 69% over 96 seeds (only engine 14, only greedy 17), -3.1 ±5.8 pts, sign test p 0.720: no better than greedy (behind, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 96.9 ±1.8 (11.91) | 86.5 ±3.5 (11.59) | -10.4 pts (-0.31 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 68.8 ±4.7 (9.76) | 65.6 ±4.8 (9.70) | -3.1 pts (-0.06 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 86.5 | 96.9 | 81 / 2 / 12 / 1 | -10.4 ±3.8 | 0.013 | worse than greedy |
| d4 engine | 96 | 65.6 | 68.8 | 49 / 14 / 17 / 16 | -3.1 ±5.8 | 0.720 | no better than greedy (behind, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.99 (1.06, 6..10) | corruption 2, deck_empty 1, opposition 93 | 2.04 / 15.86 | 10.3 / 2.00 | 15.44 | 5.43 |
| d0 engine | 8.27 (1.43, 2..10) | corruption 7, deck_empty 6, opposition 83 | 1.84 / 16.63 | 21.4 / 1.46 | 15.17 | 5.50 |
| d4 greedy | 7.16 (1.00, 5..9) | corruption 28, deck_empty 2, opposition 66 | 1.79 / 8.91 | 14.8 / 1.43 | 6.26 | 5.45 |
| d4 engine | 7.26 (1.60, 2..9) | corruption 30, deck_empty 3, opposition 63 | 1.71 / 9.97 | 23.0 / 1.09 | 6.82 | 5.34 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 229.9 | 9.95 | 4% | – | – | 10.15 (44) | 0.00 | 9.43 | 0.00 | 5.77 / 15.98 (36%) |
| d0 engine | 286.9 | 9.60 | 4% | – | – | 11.16 (45) | 0.00 | 11.02 | 0.00 | 9.54 / 16.54 (58%) |
| d4 greedy | 207.2 | 9.17 | 4% | – | – | 10.31 (38) | 0.00 | 4.22 | 0.00 | 0.78 / 14.31 (5%) |
| d4 engine | 245.6 | 9.42 | 4% | – | – | 11.29 (42) | 0.00 | 6.57 | 0.00 | 2.10 / 14.52 (14%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 52% / 4% / 44% | 59% / 3% / 38% |
| d4 | 63% / 0% / 37% | 72% / 0% / 28% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.36 | 3 of 93 | 0.35 |
| d0 engine | 46 of 96 | 0.61 | 42 of 83 | 0.61 |
| d4 greedy | 3 of 96 | 0.36 | 3 of 66 | 0.34 |
| d4 engine | 24 of 96 | 0.54 | 15 of 63 | 0.52 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.93 | 1.98 | 20.7 | 94% (94%) |
| engine | 3.55 | 2.57 | 35.3 | 98% (98%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.85 | 1.81 | 17.3 | 88% (88%) |
| engine | 2.46 | 2.15 | 27.6 | 79% (79%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (217) / 100% / 0% | 1.1 (206) / 99% / 2% |
| civic_shelter | 1.8 (191) / 100% / 0% | 1.8 (191) / 96% / 0% |
| contract_bid | 3.9 (194) / 99% / 0% | 3.3 (953) / 76% / 88% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 28% |
| data_mining | 3.6 (192) / 100% / 0% | 3.0 (662) / 81% / 72% |
| emergency_fab | 2.5 (192) / 100% / 0% | 2.4 (201) / 94% / 1% |
| evacuate | 3.0 (192) / 100% / 0% | 2.8 (256) / 89% / 22% |
| fab_line | 1.9 (194) / 99% / 0% | 1.9 (189) / 97% / 30% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.7 (188) / 99% / 0% |
| fork_process | 2.2 (400) / 100% / 53% | 2.2 (448) / 93% / 58% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (193) / 99% / 0% | 3.1 (685) / 80% / 73% |
| grid_battery | 2.0 (192) / 100% / 0% | 2.0 (191) / 97% / 8% |
| jam_signals | 3.2 (192) / 100% / 0% | 2.7 (496) / 86% / 63% |
| liquidate_reserves | 3.6 (194) / 98% / 0% | 3.4 (287) / 75% / 17% |
| logistics_ai | 1.3 (197) / 100% / 0% | 1.2 (197) / 99% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (189) / 100% / 0% | 2.3 (189) / 99% / 37% |
| opposition_network | 1.5 (189) / 100% / 0% | 1.5 (185) / 99% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (221) / 100% / 0% | 1.0 (199) / 100% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (273) / 68% / 0% |
| precision_strike | 1.6 (903) / 100% / 92% | 1.8 (923) / 93% / 91% |
| purge_protocol | – (0) / – / 85% | – (0) / – / 91% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (661) / 99% / 89% | 2.3 (711) / 89% / 90% |
| rebuild | 2.0 (259) / 99% / 35% | 1.7 (418) / 96% / 66% |
| recon_sweep | 3.6 (191) / 100% / 0% | 3.5 (249) / 74% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.5 (192) / 100% / 0% | 1.5 (191) / 98% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.4 (280) / 71% / 11% |
| server_farm | 1.5 (192) / 100% / 89% | 1.5 (188) / 99% / 65% |
| shore_up | 3.0 (243) / 98% / 22% | 2.7 (324) / 85% / 38% |
| solar_field | 2.8 (192) / 100% / 0% | 2.7 (200) / 94% / 63% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.5 (640) / 100% / 81% | 1.4 (819) / 94% / 87% |
| trade_office | 1.2 (199) / 100% / 0% | 1.2 (196) / 99% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.98/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (192) / 100% / 0% | 1.9 (190) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 20971.0 | 91.232 | 0.0 | 21081.1 |
| d0 engine | 24528.3 | 85.480 | 0.0 | 24620.4 |
| d4 greedy | 15955.7 | 77.019 | 0.0 | 16038.9 |
| d4 engine | 25329.0 | 103.116 | 0.0 | 25422.7 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-unlock_m1-d4-engine-seed300041.json`](records/close-loss-c3-baseline-unlock_m1-d4-engine-seed300041.json): close loss at difficulty 4: deck_empty in round 9, 2 adversary piece(s) over the level-1 bar, pool 14 of 14
- [`records/close-loss-c2-baseline-unlock_m1-d4-greedy-seed300002.json`](records/close-loss-c2-baseline-unlock_m1-d4-greedy-seed300002.json): close loss at difficulty 4: deck_empty in round 9, 5 adversary piece(s) over the level-2 bar, pool 9 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-unlock_m1-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-unlock_m1-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-unlock_m1-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-unlock_m1-d4-greedy-seed300000.json): won by the weakest agent (greedy)

