# Cycle report: payback-009-c-insight_inc-more

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py insight_inc): the income buildings also give +1 insight income. Engine agent against greedy, same seeds as payback-009-singles. Seeds 300096+ (144 more), to firm up the 96-seed result.

576 games (4 cells × 144 seeds, 300096..=300239) on 12 threads in 1203.2 s. Engine 0.1.0, rules r3, content 53c5b59a728530fb.

- Configs: configs/baseline.json
- Variants: insight_inc
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 94%, engine 97%; skill gap +3 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.78, engine 11.88; the ladder is ordered.
- Difficulty 4: win rates greedy 67%, engine 74%; skill gap +8 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.45, engine 9.83; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +3 → +8 pts win rate, +0.10 → +0.38 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 7.9 rounds at difficulty 0, 7.0 at 4 (target 8–10).
- Games ended by: corruption 90, deck_empty 8, opposition 478.
- Adversary operations: 11.0 per round on average across cells, at most 50 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, fab_line, firewall_node, fork_process, grid_battery, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rally, rebuild, recruitment_hub, server_farm, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 61% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 36% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine, 97% wins) plays 4.35 buildings and 3.01 tier-2 cards per game (of 38.0 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine, 74% wins) plays 3.53 buildings and 2.42 tier-2 cards per game (of 30.6 cards); 96% of its wins had 2+ buildings played (96% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: engine 74% vs greedy 67% over 144 seeds (only engine 23, only greedy 12), +7.6 ±4.1 pts, sign test p 0.090: no better than greedy (ahead, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 93.8 ±2.0 (11.78) | 97.2 ±1.4 (11.88) | +3.5 pts (+0.10 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 66.7 ±3.9 (9.45) | 74.3 ±3.6 (9.83) | +7.6 pts (+0.38 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 144 | 97.2 | 93.8 | 131 / 9 / 4 / 0 | +3.5 ±2.5 | 0.267 | no better than greedy (ahead, not significant) |
| d4 engine | 144 | 74.3 | 66.7 | 84 / 23 / 12 / 25 | +7.6 ±4.1 | 0.090 | no better than greedy (ahead, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.72 (1.11, 2..10) | corruption 5, deck_empty 4, opposition 135 | 2.33 / 16.03 | 14.4 / 1.78 | 15.19 | 5.13 |
| d0 engine | 7.93 (1.36, 2..10) | corruption 4, opposition 140 | 2.06 / 16.81 | 22.2 / 1.50 | 16.35 | 5.48 |
| d4 greedy | 7.03 (1.10, 2..9) | corruption 45, deck_empty 3, opposition 96 | 1.62 / 7.64 | 16.0 / 1.31 | 5.23 | 5.34 |
| d4 engine | 7.04 (1.22, 2..9) | corruption 36, deck_empty 1, opposition 107 | 1.76 / 9.42 | 18.5 / 1.24 | 7.08 | 5.60 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 216.0 | 10.09 | 4% | – | – | 10.38 (41) | 0.00 | 9.10 | 0.00 | 5.56 / 15.43 (36%) |
| d0 engine | 279.6 | 10.28 | 3% | – | – | 11.16 (50) | 0.00 | 10.79 | 0.00 | 9.23 / 15.86 (58%) |
| d4 greedy | 198.5 | 9.35 | 4% | – | – | 11.09 (41) | 0.00 | 4.01 | 0.00 | 0.72 / 14.07 (5%) |
| d4 engine | 242.0 | 9.92 | 4% | – | – | 11.31 (40) | 0.00 | 6.22 | 0.00 | 1.98 / 14.08 (14%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 48% / 1% / 52% | 48% / 0% / 52% |
| d4 | 71% / 1% / 29% | 74% / 1% / 26% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 144 | 0.35 | 3 of 135 | 0.35 |
| d0 engine | 58 of 144 | 0.60 | 57 of 140 | 0.60 |
| d4 greedy | 3 of 144 | 0.35 | 3 of 96 | 0.34 |
| d4 engine | 59 of 144 | 0.61 | 48 of 107 | 0.60 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.90 | 2.22 | 20.1 | 90% (90%) |
| engine | 4.35 | 3.01 | 38.0 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.82 | 1.72 | 17.6 | 82% (82%) |
| engine | 3.53 | 2.42 | 30.6 | 96% (96%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (331) / 100% / 0% | 1.2 (324) / 99% / 2% |
| civic_shelter | 2.1 (286) / 100% / 0% | 2.0 (292) / 97% / 0% |
| contract_bid | 4.0 (286) / 100% / 0% | 3.6 (1338) / 75% / 91% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 34% |
| data_mining | 3.6 (288) / 100% / 0% | 3.1 (1100) / 80% / 78% |
| emergency_fab | 2.5 (287) / 100% / 0% | 2.5 (302) / 95% / 1% |
| evacuate | 3.0 (285) / 100% / 1% | 2.8 (426) / 84% / 30% |
| fab_line | 1.9 (280) / 100% / 0% | 1.8 (285) / 99% / 66% |
| firewall_node | 1.8 (282) / 100% / 0% | 1.8 (281) / 99% / 0% |
| fork_process | 2.2 (560) / 99% / 52% | 2.1 (698) / 91% / 59% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.7 (286) / 100% / 0% | 3.1 (1197) / 79% / 81% |
| grid_battery | 1.9 (287) / 100% / 0% | 1.9 (292) / 98% / 9% |
| jam_signals | 3.2 (282) / 100% / 0% | 2.8 (703) / 85% / 63% |
| liquidate_reserves | 3.7 (287) / 99% / 0% | 3.4 (498) / 72% / 26% |
| logistics_ai | 1.2 (301) / 100% / 0% | 1.2 (297) / 99% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.2 (284) / 100% / 0% | 2.2 (286) / 98% / 71% |
| opposition_network | 1.5 (290) / 100% / 0% | 1.5 (295) / 98% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (356) / 100% / 0% | 1.1 (318) / 100% / 0% |
| power_share | 3.6 (286) / 99% / 0% | 3.5 (392) / 70% / 0% |
| precision_strike | 1.7 (1274) / 100% / 93% | 1.9 (1298) / 92% / 96% |
| purge_protocol | – (0) / – / 94% | – (0) / – / 97% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (980) / 100% / 89% | 2.4 (1034) / 92% / 94% |
| rebuild | 2.1 (359) / 99% / 29% | 1.8 (640) / 96% / 72% |
| recon_sweep | 3.6 (284) / 100% / 0% | 3.5 (352) / 81% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 2% |
| recruitment_hub | 1.5 (286) / 100% / 0% | 1.5 (285) / 100% / 0% |
| scavenge | 3.7 (282) / 99% / 0% | 3.4 (452) / 76% / 27% |
| server_farm | 1.6 (286) / 100% / 87% | 1.5 (288) / 98% / 57% |
| shore_up | 2.9 (383) / 100% / 28% | 2.7 (515) / 89% / 42% |
| solar_field | 2.7 (286) / 100% / 0% | 2.6 (305) / 93% / 92% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.5 (941) / 100% / 82% | 1.4 (1177) / 96% / 92% |
| trade_office | 1.2 (311) / 100% / 0% | 1.2 (293) / 99% / 1% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.98/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.8 (285) / 100% / 0% | 1.8 (286) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 18919.5 | 87.593 | 0.0 | 19015.0 |
| d0 engine | 38110.5 | 136.312 | 0.0 | 38229.5 |
| d4 greedy | 15374.2 | 77.455 | 0.0 | 15456.2 |
| d4 engine | 26465.0 | 109.375 | 0.0 | 26572.5 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-insight_inc-d4-engine-seed300230.json`](records/close-loss-c3-baseline-insight_inc-d4-engine-seed300230.json): close loss at difficulty 4: deck_empty in round 9, 2 adversary piece(s) over the level-2 bar, pool 12 of 14
- [`records/close-loss-c3-baseline-insight_inc-d4-engine-seed300170.json`](records/close-loss-c3-baseline-insight_inc-d4-engine-seed300170.json): close loss at difficulty 4: corruption in round 7, 2 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-insight_inc-d0-greedy-seed300096.json`](records/surprising-win-c0-baseline-insight_inc-d0-greedy-seed300096.json): won by the weakest agent (greedy)
- [`records/surprising-win-c0-baseline-insight_inc-d0-greedy-seed300097.json`](records/surprising-win-c0-baseline-insight_inc-d0-greedy-seed300097.json): won by the weakest agent (greedy)

