# Cycle report: payback-009-c-insight_inc+deck444

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py insight_inc): the income buildings also give +1 insight income; with adversary.stage_cards=4,4,4. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 287.3 s. Engine 0.1.0, rules r3, content 53c5b59a728530fb.

- Configs: configs/baseline.json
- Variants: insight_inc+deck444 (adversary.stage_cards=4,4,4)
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 98%, engine 91%; skill gap -7 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 13.89, engine 13.49; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 78%, engine 77%; skill gap -1 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 11.66, engine 11.57; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: -7 → -1 pts win rate, -0.40 → -0.08 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.2 at 4 (target 8–10).
- Games ended by: corruption 54, opposition 330.
- Adversary operations: 10.6 per round on average across cells, at most 58 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 63% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 21% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 98% wins) plays 1.88 buildings and 2.03 tier-2 cards per game (of 20.1 cards); 88% of its wins had 2+ buildings played (88% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 78% wins) plays 1.82 buildings and 1.79 tier-2 cards per game (of 17.7 cards); 88% of its wins had 2+ buildings played (88% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 77% vs greedy 78% over 96 seeds (only engine 12, only greedy 13), -1.0 ±5.2 pts, sign test p 1.000: no better than greedy (behind, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 97.9 ±1.5 (13.89) | 90.6 ±3.0 (13.49) | -7.3 pts (-0.40 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 78.1 ±4.2 (11.66) | 77.1 ±4.3 (11.57) | -1.0 pts (-0.08 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 90.6 | 97.9 | 86 / 1 / 8 / 1 | -7.3 ±3.1 | 0.039 | worse than greedy |
| d4 engine | 96 | 77.1 | 78.1 | 62 / 12 / 13 / 9 | -1.0 ±5.2 | 1.000 | no better than greedy (behind, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.83 (0.94, 5..11) | corruption 2, opposition 94 | 4.18 / 16.51 | 10.0 / 2.00 | 16.17 | 5.46 |
| d0 engine | 8.50 (1.44, 2..12) | corruption 9, opposition 87 | 3.51 / 17.91 | 16.4 / 1.78 | 16.23 | 5.56 |
| d4 greedy | 7.24 (1.17, 5..10) | corruption 21, opposition 75 | 3.65 / 8.07 | 14.1 / 1.38 | 6.30 | 5.39 |
| d4 engine | 7.33 (1.53, 2..10) | corruption 22, opposition 74 | 3.50 / 9.91 | 23.9 / 1.05 | 7.64 | 5.65 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 216.9 | 9.97 | 4% | – | – | 9.50 (48) | 0.00 | 9.23 | 0.00 | 5.74 / 15.67 (37%) |
| d0 engine | 300.2 | 10.23 | 4% | – | – | 11.47 (48) | 0.00 | 11.65 | 0.00 | 10.61 / 17.00 (62%) |
| d4 greedy | 200.2 | 9.41 | 4% | – | – | 10.34 (47) | 0.00 | 4.22 | 0.00 | 0.78 / 14.48 (5%) |
| d4 engine | 253.7 | 9.94 | 3% | – | – | 11.11 (58) | 0.00 | 7.21 | 0.00 | 3.07 / 14.67 (21%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 57% / 2% / 41% | 60% / 2% / 38% |
| d4 | 63% / 0% / 37% | 67% / 0% / 33% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.36 | 3 of 94 | 0.36 |
| d0 engine | 56 of 96 | 0.62 | 53 of 87 | 0.62 |
| d4 greedy | 3 of 96 | 0.36 | 3 of 75 | 0.34 |
| d4 engine | 44 of 96 | 0.61 | 38 of 74 | 0.59 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.88 | 2.03 | 20.1 | 88% (88%) |
| engine | 5.03 | 3.48 | 42.1 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.82 | 1.79 | 17.7 | 88% (88%) |
| engine | 4.10 | 2.97 | 33.3 | 97% (97%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (235) / 100% / 0% | 1.1 (217) / 100% / 6% |
| civic_shelter | 1.8 (192) / 100% / 0% | 1.8 (188) / 98% / 1% |
| contract_bid | 4.0 (192) / 99% / 0% | 3.6 (976) / 70% / 91% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 53% |
| data_mining | 3.6 (192) / 100% / 0% | 3.1 (846) / 77% / 84% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.5 (199) / 95% / 1% |
| evacuate | 3.0 (192) / 100% / 0% | 2.8 (268) / 90% / 31% |
| fab_line | 1.9 (194) / 99% / 0% | 1.9 (188) / 99% / 75% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.8 (193) / 98% / 0% |
| fork_process | 2.2 (380) / 100% / 53% | 2.1 (527) / 92% / 64% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 2% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (192) / 99% / 0% | 3.0 (890) / 79% / 86% |
| grid_battery | 2.0 (192) / 100% / 0% | 1.9 (190) / 97% / 19% |
| jam_signals | 3.2 (190) / 100% / 0% | 2.7 (587) / 84% / 70% |
| liquidate_reserves | 3.6 (192) / 99% / 0% | 3.4 (321) / 74% / 28% |
| logistics_ai | 1.2 (206) / 100% / 0% | 1.2 (203) / 99% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (188) / 100% / 0% | 2.3 (190) / 99% / 87% |
| opposition_network | 1.5 (190) / 100% / 0% | 1.5 (187) / 98% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (224) / 100% / 0% | 1.0 (207) / 100% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (296) / 63% / 0% |
| precision_strike | 1.7 (896) / 100% / 92% | 1.9 (902) / 92% / 94% |
| purge_protocol | – (0) / – / 94% | – (0) / – / 97% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (656) / 99% / 88% | 2.4 (666) / 91% / 91% |
| rebuild | 2.0 (250) / 100% / 31% | 1.7 (446) / 98% / 74% |
| recon_sweep | 3.6 (186) / 100% / 0% | 3.4 (268) / 69% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 10% |
| recruitment_hub | 1.5 (192) / 100% / 0% | 1.5 (187) / 99% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.3 (341) / 65% / 34% |
| server_farm | 1.5 (192) / 100% / 85% | 1.5 (186) / 100% / 68% |
| shore_up | 3.0 (234) / 99% / 21% | 2.7 (332) / 84% / 44% |
| solar_field | 2.7 (191) / 100% / 0% | 2.7 (200) / 92% / 95% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (595) / 100% / 80% | 1.5 (816) / 96% / 92% |
| trade_office | 1.2 (209) / 100% / 0% | 1.2 (202) / 99% / 9% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (191) / 100% / 0% | 1.9 (188) / 99% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 6141.8 | 28.320 | 0.0 | 6172.2 |
| d0 engine | 14327.0 | 47.728 | 0.0 | 14373.1 |
| d4 greedy | 5271.3 | 26.332 | 0.0 | 5299.1 |
| d4 engine | 9701.8 | 38.240 | 0.0 | 9735.7 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c2-baseline-insight_inc_deck444-d4-greedy-seed300091.json`](records/close-loss-c2-baseline-insight_inc_deck444-d4-greedy-seed300091.json): close loss at difficulty 4: corruption in round 7, 3 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c2-baseline-insight_inc_deck444-d4-greedy-seed300085.json`](records/close-loss-c2-baseline-insight_inc_deck444-d4-greedy-seed300085.json): close loss at difficulty 4: corruption in round 10, 4 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-insight_inc_deck444-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-insight_inc_deck444-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-insight_inc_deck444-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-insight_inc_deck444-d4-greedy-seed300000.json): won by the weakest agent (greedy)

