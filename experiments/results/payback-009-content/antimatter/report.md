# Cycle report: payback-009-c-antimatter

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py antimatter): Refinery converts 2 energy, tracks' second-to-last value +1 antimatter. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 719.7 s. Engine 0.1.0, rules r3, content 040ff07eb4d389bb.

- Configs: configs/baseline.json
- Variants: antimatter
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 95%, engine 98%; skill gap +3 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.81, engine 11.92; the ladder is ordered.
- Difficulty 4: win rates greedy 72%, engine 73%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.83, engine 9.84; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +3 → +1 pts win rate, +0.10 → +0.01 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 8.2 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 55, deck_empty 4, opposition 324, population 1.
- Adversary operations: 10.4 per round on average across cells, at most 44 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, fab_line, firewall_node, fork_process, grid_battery, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rebuild, recruitment_hub, server_farm, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 73% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 38% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine, 98% wins) plays 3.65 buildings and 2.93 tier-2 cards per game (of 35.9 cards); 97% of its wins had 2+ buildings played (97% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine, 73% wins) plays 2.50 buildings and 2.47 tier-2 cards per game (of 27.7 cards); 86% of its wins had 2+ buildings played (86% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: engine 73% vs greedy 72% over 96 seeds (only engine 10, only greedy 9), +1.0 ±4.6 pts, sign test p 1.000: no better than greedy (ahead, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 94.8 ±2.3 (11.81) | 97.9 ±1.5 (11.92) | +3.1 pts (+0.10 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 71.9 ±4.6 (9.83) | 72.9 ±4.5 (9.84) | +1.0 pts (+0.01 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 97.9 | 94.8 | 90 / 4 / 1 / 1 | +3.1 ±2.3 | 0.375 | no better than greedy (ahead, not significant) |
| d4 engine | 96 | 72.9 | 71.9 | 60 / 10 / 9 / 17 | +1.0 ±4.6 | 1.000 | no better than greedy (ahead, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.76 (0.94, 4..10) | corruption 3, deck_empty 2, opposition 91 | 2.27 / 16.93 | 8.0 / 2.00 | 16.34 | 5.39 |
| d0 engine | 8.20 (1.50, 2..10) | corruption 2, opposition 94 | 1.80 / 16.79 | 12.0 / 2.00 | 16.44 | 5.49 |
| d4 greedy | 7.25 (1.09, 5..9) | corruption 24, deck_empty 2, opposition 69, population 1 | 1.59 / 8.51 | 13.7 / 1.52 | 6.21 | 5.32 |
| d4 engine | 7.11 (1.67, 2..9) | corruption 26, opposition 70 | 1.74 / 9.97 | 19.9 / 1.19 | 7.27 | 5.45 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 214.8 | 10.06 | 4% | – | – | 9.41 (41) | 0.00 | 9.62 | 0.00 | 6.12 / 15.52 (39%) |
| d0 engine | 276.1 | 9.84 | 4% | – | – | 10.62 (42) | 0.00 | 11.29 | 0.00 | 9.92 / 16.40 (60%) |
| d4 greedy | 200.6 | 9.46 | 4% | – | – | 10.34 (42) | 0.00 | 4.20 | 0.00 | 0.78 / 14.50 (5%) |
| d4 engine | 234.9 | 9.60 | 4% | – | – | 11.06 (44) | 0.00 | 6.86 | 0.00 | 2.21 / 14.23 (16%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 63% / 1% / 36% | 68% / 1% / 32% |
| d4 | 75% / 0% / 25% | 78% / 0% / 22% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.36 | 3 of 91 | 0.35 |
| d0 engine | 46 of 96 | 0.61 | 46 of 94 | 0.61 |
| d4 greedy | 3 of 96 | 0.36 | 3 of 69 | 0.33 |
| d4 engine | 23 of 96 | 0.53 | 18 of 70 | 0.51 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.90 | 2.08 | 19.8 | 91% (91%) |
| engine | 3.65 | 2.93 | 35.9 | 97% (97%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.83 | 1.75 | 17.7 | 88% (88%) |
| engine | 2.50 | 2.47 | 27.7 | 86% (86%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (237) / 100% / 0% | 1.1 (212) / 99% / 2% |
| civic_shelter | 1.8 (192) / 100% / 0% | 1.8 (194) / 96% / 0% |
| contract_bid | 4.0 (190) / 99% / 0% | 3.3 (938) / 77% / 88% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 39% |
| data_mining | 3.6 (192) / 100% / 0% | 3.0 (674) / 81% / 75% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.4 (199) / 95% / 1% |
| evacuate | 3.0 (192) / 100% / 0% | 2.9 (242) / 88% / 20% |
| fab_line | 1.9 (193) / 99% / 0% | 1.9 (189) / 98% / 29% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.8 (187) / 99% / 0% |
| fork_process | 2.3 (375) / 100% / 52% | 2.2 (450) / 92% / 58% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (192) / 99% / 0% | 3.0 (659) / 81% / 73% |
| grid_battery | 2.0 (192) / 100% / 0% | 1.9 (190) / 98% / 9% |
| jam_signals | 3.2 (190) / 100% / 0% | 2.7 (486) / 88% / 64% |
| liquidate_reserves | 3.6 (191) / 99% / 0% | 3.5 (267) / 75% / 10% |
| logistics_ai | 1.2 (208) / 100% / 0% | 1.3 (199) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (190) / 100% / 0% | 2.3 (190) / 99% / 40% |
| opposition_network | 1.4 (189) / 100% / 0% | 1.5 (186) / 99% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (225) / 100% / 0% | 1.0 (204) / 100% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (272) / 69% / 0% |
| precision_strike | 1.7 (882) / 100% / 93% | 1.8 (906) / 93% / 92% |
| purge_protocol | – (0) / – / 94% | – (0) / – / 96% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.1 (650) / 99% / 90% | 2.4 (703) / 89% / 89% |
| rebuild | 2.0 (249) / 99% / 29% | 1.7 (411) / 98% / 66% |
| recon_sweep | 3.6 (186) / 100% / 0% | 3.5 (248) / 73% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.5 (191) / 100% / 0% | 1.5 (193) / 97% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.4 (270) / 71% / 10% |
| server_farm | 1.5 (192) / 100% / 86% | 1.5 (185) / 100% / 66% |
| shore_up | 3.0 (235) / 99% / 21% | 2.8 (308) / 89% / 37% |
| solar_field | 2.7 (191) / 100% / 0% | 2.7 (196) / 95% / 67% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (594) / 100% / 81% | 1.5 (791) / 96% / 88% |
| trade_office | 1.2 (208) / 100% / 0% | 1.2 (201) / 99% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (191) / 100% / 0% | 1.9 (187) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 17553.4 | 81.727 | 0.0 | 17640.9 |
| d0 engine | 31696.1 | 114.793 | 0.0 | 31803.3 |
| d4 greedy | 15750.0 | 78.496 | 0.0 | 15835.5 |
| d4 engine | 23375.4 | 99.514 | 0.0 | 23467.3 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-antimatter-d4-engine-seed300037.json`](records/close-loss-c3-baseline-antimatter-d4-engine-seed300037.json): close loss at difficulty 4: corruption in round 7, 2 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c2-baseline-antimatter-d4-greedy-seed300091.json`](records/close-loss-c2-baseline-antimatter-d4-greedy-seed300091.json): close loss at difficulty 4: corruption in round 7, 3 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-antimatter-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-antimatter-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-antimatter-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-antimatter-d4-greedy-seed300000.json): won by the weakest agent (greedy)

