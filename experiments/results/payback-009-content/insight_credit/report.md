# Cycle report: payback-009-c-insight_credit

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py insight_credit): Fab Line, Trade Office and Megastructure also give +1 insight income. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 253.8 s. Engine 0.1.0, rules r3, content 14413c5f55b31a46.

- Configs: configs/baseline.json
- Variants: insight_credit
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 96%, engine 99%; skill gap +3 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.89, engine 11.96; the ladder is ordered.
- Difficulty 4: win rates greedy 77%, engine 79%; skill gap +2 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 10.04, engine 10.05; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +3 → +2 pts win rate, +0.07 → +0.01 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 8.1 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 40, deck_empty 7, opposition 337.
- Adversary operations: 10.1 per round on average across cells, at most 44 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grid_battery, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rebuild, recruitment_hub, server_farm, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 62% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 36% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine, 99% wins) plays 3.97 buildings and 2.91 tier-2 cards per game (of 36.8 cards); 98% of its wins had 2+ buildings played (98% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine, 79% wins) plays 2.82 buildings and 2.42 tier-2 cards per game (of 28.9 cards); 93% of its wins had 2+ buildings played (93% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: engine 79% vs greedy 77% over 96 seeds (only engine 11, only greedy 9), +2.1 ±4.7 pts, sign test p 0.824: no better than greedy (ahead, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.89) | 99.0 ±1.0 (11.96) | +3.1 pts (+0.07 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 77.1 ±4.3 (10.04) | 79.2 ±4.1 (10.05) | +2.1 pts (+0.01 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 99.0 | 95.8 | 91 / 4 / 1 / 0 | +3.1 ±2.3 | 0.375 | no better than greedy (ahead, not significant) |
| d4 engine | 96 | 79.2 | 77.1 | 65 / 11 / 9 / 11 | +2.1 ±4.7 | 0.824 | no better than greedy (ahead, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.81 (0.88, 5..10) | corruption 2, deck_empty 2, opposition 92 | 2.25 / 16.64 | 8.2 / 2.00 | 16.17 | 5.46 |
| d0 engine | 8.15 (1.36, 2..10) | corruption 1, opposition 95 | 1.85 / 17.31 | 13.0 / 2.00 | 17.12 | 5.61 |
| d4 greedy | 7.20 (1.09, 5..9) | corruption 18, deck_empty 4, opposition 74 | 1.69 / 8.12 | 14.6 / 1.36 | 6.46 | 5.43 |
| d4 engine | 7.12 (1.47, 2..9) | corruption 19, deck_empty 1, opposition 76 | 1.70 / 9.89 | 22.4 / 1.00 | 7.96 | 5.55 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 216.6 | 9.97 | 4% | – | – | 9.49 (42) | 0.00 | 9.23 | 0.00 | 5.74 / 15.62 (37%) |
| d0 engine | 277.2 | 10.00 | 4% | – | – | 10.29 (44) | 0.00 | 10.97 | 0.00 | 9.44 / 16.29 (58%) |
| d4 greedy | 199.2 | 9.40 | 4% | – | – | 10.24 (42) | 0.00 | 4.22 | 0.00 | 0.78 / 14.40 (5%) |
| d4 engine | 237.1 | 9.73 | 4% | – | – | 10.46 (44) | 0.00 | 6.29 | 0.00 | 2.04 / 14.25 (14%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 49% / 1% / 50% | 49% / 0% / 51% |
| d4 | 70% / 0% / 30% | 76% / 0% / 24% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.36 | 3 of 92 | 0.36 |
| d0 engine | 51 of 96 | 0.62 | 51 of 95 | 0.62 |
| d4 greedy | 3 of 96 | 0.36 | 3 of 74 | 0.34 |
| d4 engine | 30 of 96 | 0.58 | 28 of 76 | 0.57 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.88 | 2.03 | 20.1 | 88% (88%) |
| engine | 3.97 | 2.91 | 36.8 | 98% (98%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.82 | 1.79 | 17.5 | 88% (88%) |
| engine | 2.82 | 2.42 | 28.9 | 93% (93%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (235) / 100% / 0% | 1.1 (217) / 99% / 2% |
| civic_shelter | 1.8 (192) / 100% / 0% | 1.8 (192) / 96% / 0% |
| contract_bid | 4.0 (192) / 99% / 0% | 3.4 (930) / 77% / 90% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 27% |
| data_mining | 3.6 (192) / 100% / 0% | 3.0 (705) / 83% / 75% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.4 (203) / 94% / 1% |
| evacuate | 3.0 (192) / 100% / 0% | 2.8 (244) / 91% / 22% |
| fab_line | 1.9 (194) / 99% / 0% | 1.9 (188) / 98% / 60% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.8 (189) / 99% / 0% |
| fork_process | 2.2 (380) / 100% / 53% | 2.2 (482) / 93% / 60% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (192) / 99% / 0% | 3.0 (706) / 81% / 76% |
| grid_battery | 2.0 (192) / 100% / 0% | 1.9 (189) / 98% / 10% |
| jam_signals | 3.2 (190) / 100% / 0% | 2.8 (487) / 87% / 64% |
| liquidate_reserves | 3.6 (192) / 99% / 0% | 3.5 (274) / 76% / 12% |
| logistics_ai | 1.2 (206) / 100% / 0% | 1.3 (201) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (188) / 100% / 0% | 2.3 (191) / 99% / 42% |
| opposition_network | 1.5 (190) / 100% / 0% | 1.5 (184) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (224) / 100% / 0% | 1.0 (210) / 100% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (268) / 70% / 0% |
| precision_strike | 1.7 (892) / 100% / 93% | 1.9 (894) / 94% / 94% |
| purge_protocol | – (0) / – / 94% | – (0) / – / 97% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (654) / 99% / 88% | 2.4 (698) / 89% / 92% |
| rebuild | 2.0 (250) / 100% / 30% | 1.8 (397) / 97% / 68% |
| recon_sweep | 3.6 (186) / 100% / 0% | 3.5 (251) / 72% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 2% |
| recruitment_hub | 1.5 (192) / 100% / 0% | 1.5 (193) / 98% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.4 (290) / 72% / 17% |
| server_farm | 1.5 (192) / 100% / 85% | 1.5 (185) / 100% / 62% |
| shore_up | 3.0 (234) / 99% / 21% | 2.8 (298) / 89% / 35% |
| solar_field | 2.7 (191) / 100% / 0% | 2.7 (197) / 94% / 67% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (594) / 100% / 80% | 1.5 (798) / 97% / 89% |
| trade_office | 1.2 (209) / 100% / 0% | 1.2 (200) / 99% / 2% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.98/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (191) / 100% / 0% | 1.9 (185) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 6098.5 | 28.151 | 0.0 | 6128.4 |
| d0 engine | 11591.4 | 41.810 | 0.0 | 11633.8 |
| d4 greedy | 5143.0 | 25.824 | 0.0 | 5170.1 |
| d4 engine | 8441.7 | 35.608 | 0.0 | 8472.8 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c2-baseline-insight_credit-d4-greedy-seed300091.json`](records/close-loss-c2-baseline-insight_credit-d4-greedy-seed300091.json): close loss at difficulty 4: corruption in round 7, 3 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c3-baseline-insight_credit-d4-engine-seed300048.json`](records/close-loss-c3-baseline-insight_credit-d4-engine-seed300048.json): close loss at difficulty 4: corruption in round 7, 4 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-insight_credit-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-insight_credit-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-insight_credit-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-insight_credit-d4-greedy-seed300000.json): won by the weakest agent (greedy)

