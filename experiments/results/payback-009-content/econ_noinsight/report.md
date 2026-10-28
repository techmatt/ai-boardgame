# Cycle report: payback-009-c-econ_noinsight

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py econ_noinsight): economy buildings cost no insight. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 518.5 s. Engine 0.1.0, rules r3, content 6cb5394dd610f6da.

- Configs: configs/baseline.json
- Variants: econ_noinsight
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 96%, engine 97%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.85, engine 11.86; the ladder is ordered.
- Difficulty 4: win rates greedy 76%, engine 74%; skill gap -2 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.98, engine 9.92; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: +1 → -2 pts win rate, +0.01 → -0.06 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 49, deck_empty 6, opposition 329.
- Adversary operations: 10.6 per round on average across cells, at most 45 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 62% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 21% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine, 97% wins) plays 4.12 buildings and 2.83 tier-2 cards per game (of 35.6 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 76% wins) plays 1.82 buildings and 1.78 tier-2 cards per game (of 17.4 cards); 85% of its wins had 2+ buildings played (85% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 74% vs greedy 76% over 96 seeds (only engine 9, only greedy 11), -2.1 ±4.7 pts, sign test p 0.824: no better than greedy (behind, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.85) | 96.9 ±1.8 (11.86) | +1.0 pts (+0.01 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 76.0 ±4.4 (9.98) | 74.0 ±4.5 (9.92) | -2.1 pts (-0.06 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 96.9 | 95.8 | 89 / 4 / 3 / 0 | +1.0 ±2.8 | 1.000 | no better than greedy (ahead, not significant) |
| d4 engine | 96 | 74.0 | 76.0 | 62 / 9 / 11 / 14 | -2.1 ±4.7 | 0.824 | no better than greedy (behind, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.81 (0.88, 5..10) | corruption 3, deck_empty 1, opposition 92 | 2.22 / 16.37 | 9.0 / 2.00 | 15.76 | 5.43 |
| d0 engine | 8.16 (1.73, 2..10) | corruption 3, opposition 93 | 1.83 / 16.72 | 29.0 / 1.33 | 16.20 | 5.49 |
| d4 greedy | 7.15 (1.11, 4..9) | corruption 20, deck_empty 3, opposition 73 | 1.73 / 8.52 | 14.7 / 1.35 | 6.70 | 5.52 |
| d4 engine | 7.22 (1.73, 2..9) | corruption 23, deck_empty 2, opposition 71 | 1.65 / 9.66 | 18.5 / 1.28 | 7.33 | 5.54 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 216.6 | 10.12 | 4% | – | – | 9.66 (42) | 0.00 | 9.22 | 0.00 | 5.67 / 15.62 (36%) |
| d0 engine | 275.4 | 9.97 | 4% | – | – | 11.02 (45) | 0.00 | 10.98 | 0.00 | 9.59 / 16.31 (59%) |
| d4 greedy | 197.6 | 9.49 | 4% | – | – | 10.09 (39) | 0.00 | 4.19 | 0.00 | 0.77 / 14.29 (5%) |
| d4 engine | 238.0 | 9.76 | 4% | – | – | 11.44 (38) | 0.00 | 6.61 | 0.00 | 2.07 / 14.44 (14%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 48% / 0% / 52% | 48% / 0% / 52% |
| d4 | 72% / 0% / 28% | 76% / 0% / 24% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.37 | 3 of 92 | 0.36 |
| d0 engine | 56 of 96 | 0.63 | 54 of 93 | 0.63 |
| d4 greedy | 3 of 96 | 0.37 | 3 of 73 | 0.35 |
| d4 engine | 38 of 96 | 0.59 | 32 of 71 | 0.58 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.88 | 2.05 | 20.0 | 88% (88%) |
| engine | 4.12 | 2.83 | 35.6 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.82 | 1.78 | 17.4 | 85% (85%) |
| engine | 2.95 | 2.33 | 28.2 | 93% (93%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.2 (235) / 100% / 0% | 1.3 (210) / 99% / 7% |
| civic_shelter | 1.7 (192) / 100% / 0% | 1.6 (191) / 96% / 0% |
| contract_bid | 3.9 (192) / 99% / 0% | 3.3 (937) / 77% / 88% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 31% |
| data_mining | 3.6 (192) / 100% / 0% | 3.0 (651) / 79% / 72% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.4 (194) / 96% / 0% |
| evacuate | 3.0 (192) / 100% / 0% | 2.8 (259) / 89% / 23% |
| fab_line | 2.1 (192) / 100% / 0% | 2.2 (190) / 96% / 49% |
| firewall_node | 1.6 (192) / 100% / 0% | 1.7 (186) / 100% / 0% |
| fork_process | 2.2 (378) / 100% / 53% | 2.2 (435) / 92% / 57% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (190) / 99% / 0% | 3.1 (659) / 75% / 73% |
| grid_battery | 2.3 (192) / 100% / 0% | 2.3 (196) / 94% / 15% |
| jam_signals | 3.2 (190) / 100% / 0% | 2.7 (488) / 86% / 64% |
| liquidate_reserves | 3.6 (192) / 99% / 0% | 3.4 (272) / 74% / 12% |
| logistics_ai | 1.1 (203) / 100% / 0% | 1.1 (196) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.5 (190) / 100% / 0% | 2.4 (189) / 97% / 56% |
| opposition_network | 1.4 (192) / 100% / 0% | 1.4 (186) / 99% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (229) / 100% / 0% | 1.1 (215) / 99% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (282) / 65% / 0% |
| precision_strike | 1.7 (881) / 100% / 93% | 1.8 (916) / 94% / 92% |
| purge_protocol | – (0) / – / 94% | – (0) / – / 96% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (643) / 99% / 89% | 2.3 (720) / 87% / 90% |
| rebuild | 1.9 (245) / 99% / 28% | 1.6 (410) / 97% / 67% |
| recon_sweep | 3.6 (186) / 100% / 0% | 3.5 (249) / 72% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 5% |
| recruitment_hub | 1.4 (191) / 100% / 0% | 1.4 (192) / 97% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.4 (284) / 69% / 12% |
| server_farm | 1.4 (192) / 100% / 85% | 1.4 (188) / 99% / 66% |
| shore_up | 3.0 (238) / 99% / 22% | 2.8 (316) / 91% / 40% |
| solar_field | 2.7 (190) / 100% / 0% | 2.7 (196) / 93% / 67% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.5 (585) / 100% / 81% | 1.4 (784) / 96% / 88% |
| trade_office | 1.6 (211) / 100% / 0% | 1.6 (202) / 99% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.98/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.8 (191) / 100% / 0% | 1.8 (187) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 13472.4 | 62.207 | 0.0 | 13544.9 |
| d0 engine | 22797.7 | 82.769 | 0.0 | 22875.3 |
| d4 greedy | 10554.0 | 53.410 | 0.0 | 10607.6 |
| d4 engine | 16694.9 | 70.143 | 0.0 | 16755.9 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-econ_noinsight-d4-engine-seed300002.json`](records/close-loss-c3-baseline-econ_noinsight-d4-engine-seed300002.json): close loss at difficulty 4: deck_empty in round 9, 3 adversary piece(s) over the level-2 bar, pool 13 of 14
- [`records/close-loss-c3-baseline-econ_noinsight-d4-engine-seed300092.json`](records/close-loss-c3-baseline-econ_noinsight-d4-engine-seed300092.json): close loss at difficulty 4: deck_empty in round 9, 4 adversary piece(s) over the level-2 bar, pool 5 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-econ_noinsight-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-econ_noinsight-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-econ_noinsight-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-econ_noinsight-d4-greedy-seed300000.json): won by the weakest agent (greedy)

