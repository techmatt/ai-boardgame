# Cycle report: payback-009-c-income_x2

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py income_x2): economy buildings' income doubled. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 269.3 s. Engine 0.1.0, rules r3, content c69677007f7c54cd.

- Configs: configs/baseline.json
- Variants: income_x2
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 96%, engine 93%; skill gap -3 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.89, engine 11.71; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 77%, engine 72%; skill gap -5 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 10.04, engine 9.81; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: -3 → -5 pts win rate, -0.18 → -0.23 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.2 at 4 (target 8–10).
- Games ended by: corruption 51, deck_empty 9, opposition 324.
- Adversary operations: 10.4 per round on average across cells, at most 48 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 66% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 21% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 96% wins) plays 1.88 buildings and 2.03 tier-2 cards per game (of 20.1 cards); 88% of its wins had 2+ buildings played (88% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 77% wins) plays 1.82 buildings and 1.79 tier-2 cards per game (of 17.5 cards); 88% of its wins had 2+ buildings played (88% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 72% vs greedy 77% over 96 seeds (only engine 10, only greedy 15), -5.2 ±5.2 pts, sign test p 0.424: no better than greedy (behind, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.89) | 92.7 ±2.7 (11.71) | -3.1 pts (-0.18 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 77.1 ±4.3 (10.04) | 71.9 ±4.6 (9.81) | -5.2 pts (-0.23 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 92.7 | 95.8 | 86 / 3 / 6 / 1 | -3.1 ±3.1 | 0.508 | no better than greedy (behind, not significant) |
| d4 engine | 96 | 71.9 | 77.1 | 59 / 10 / 15 / 12 | -5.2 ±5.2 | 0.424 | no better than greedy (behind, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.81 (0.88, 5..10) | corruption 2, deck_empty 2, opposition 92 | 2.25 / 16.64 | 8.2 / 2.00 | 16.17 | 5.46 |
| d0 engine | 8.35 (1.38, 2..10) | corruption 6, deck_empty 1, opposition 89 | 1.62 / 17.08 | 12.9 / 1.86 | 15.92 | 5.44 |
| d4 greedy | 7.20 (1.09, 5..9) | corruption 18, deck_empty 4, opposition 74 | 1.69 / 8.12 | 14.6 / 1.36 | 6.46 | 5.43 |
| d4 engine | 7.40 (1.48, 2..9) | corruption 25, deck_empty 2, opposition 69 | 1.36 / 9.71 | 20.0 / 1.15 | 7.25 | 5.52 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 216.6 | 9.97 | 4% | – | – | 9.49 (42) | 0.00 | 9.23 | 0.00 | 5.74 / 15.62 (37%) |
| d0 engine | 283.7 | 9.56 | 4% | – | – | 10.95 (48) | 0.00 | 11.02 | 0.00 | 10.10 / 16.71 (60%) |
| d4 greedy | 199.2 | 9.40 | 4% | – | – | 10.24 (42) | 0.00 | 4.22 | 0.00 | 0.78 / 14.40 (5%) |
| d4 engine | 245.3 | 9.35 | 4% | – | – | 11.03 (43) | 0.00 | 6.92 | 0.00 | 2.78 / 14.79 (19%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 57% / 0% / 43% | 61% / 1% / 38% |
| d4 | 66% / 0% / 34% | 70% / 0% / 29% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.36 | 3 of 92 | 0.36 |
| d0 engine | 51 of 96 | 0.61 | 50 of 89 | 0.61 |
| d4 greedy | 3 of 96 | 0.36 | 3 of 74 | 0.34 |
| d4 engine | 50 of 96 | 0.63 | 44 of 69 | 0.63 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.88 | 2.03 | 20.1 | 88% (88%) |
| engine | 4.83 | 2.92 | 37.9 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.82 | 1.79 | 17.5 | 88% (88%) |
| engine | 4.14 | 2.45 | 30.3 | 96% (96%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (235) / 100% / 0% | 1.1 (218) / 100% / 8% |
| civic_shelter | 1.8 (192) / 100% / 0% | 1.8 (194) / 97% / 0% |
| contract_bid | 4.0 (192) / 99% / 0% | 3.3 (984) / 79% / 89% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 38% |
| data_mining | 3.6 (192) / 100% / 0% | 3.0 (659) / 82% / 71% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.5 (195) / 96% / 1% |
| evacuate | 3.0 (192) / 100% / 0% | 2.8 (245) / 93% / 24% |
| fab_line | 1.9 (194) / 99% / 0% | 1.9 (190) / 98% / 81% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.8 (195) / 96% / 0% |
| fork_process | 2.2 (380) / 100% / 53% | 2.2 (440) / 93% / 57% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 3% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (192) / 99% / 0% | 3.0 (670) / 79% / 72% |
| grid_battery | 2.0 (192) / 100% / 0% | 1.9 (192) / 98% / 21% |
| jam_signals | 3.2 (190) / 100% / 0% | 2.7 (533) / 87% / 64% |
| liquidate_reserves | 3.6 (192) / 99% / 0% | 3.3 (292) / 73% / 18% |
| logistics_ai | 1.2 (206) / 100% / 0% | 1.3 (203) / 99% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (188) / 100% / 0% | 2.4 (190) / 99% / 88% |
| opposition_network | 1.5 (190) / 100% / 0% | 1.5 (185) / 99% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (224) / 100% / 0% | 1.0 (209) / 100% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.4 (287) / 66% / 0% |
| precision_strike | 1.7 (892) / 100% / 93% | 1.8 (933) / 92% / 92% |
| purge_protocol | – (0) / – / 94% | – (0) / – / 96% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (654) / 99% / 88% | 2.3 (718) / 92% / 90% |
| rebuild | 2.0 (250) / 100% / 30% | 1.8 (420) / 98% / 68% |
| recon_sweep | 3.6 (186) / 100% / 0% | 3.5 (256) / 73% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 3% |
| recruitment_hub | 1.5 (192) / 100% / 0% | 1.5 (190) / 98% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.3 (319) / 64% / 14% |
| server_farm | 1.5 (192) / 100% / 85% | 1.5 (186) / 100% / 61% |
| shore_up | 3.0 (234) / 99% / 21% | 2.8 (306) / 84% / 33% |
| solar_field | 2.7 (191) / 100% / 0% | 2.7 (199) / 93% / 95% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (594) / 100% / 80% | 1.4 (823) / 96% / 90% |
| trade_office | 1.2 (209) / 100% / 0% | 1.2 (203) / 98% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.96/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (191) / 100% / 0% | 1.9 (187) / 99% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 6481.3 | 29.918 | 0.0 | 6513.5 |
| d0 engine | 12229.1 | 43.104 | 0.0 | 12275.0 |
| d4 greedy | 5233.4 | 26.278 | 0.0 | 5260.6 |
| d4 engine | 9203.5 | 37.516 | 0.0 | 9238.0 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-income_x2-d4-engine-seed300038.json`](records/close-loss-c3-baseline-income_x2-d4-engine-seed300038.json): close loss at difficulty 4: corruption in round 8, 2 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c3-baseline-income_x2-d4-engine-seed300055.json`](records/close-loss-c3-baseline-income_x2-d4-engine-seed300055.json): close loss at difficulty 4: deck_empty in round 9, 3 adversary piece(s) over the level-2 bar, pool 14 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-income_x2-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-income_x2-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-income_x2-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-income_x2-d4-greedy-seed300000.json): won by the weakest agent (greedy)

