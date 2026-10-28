# Cycle report: payback-009-c-insight_inc

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py insight_inc): the income buildings also give +1 insight income. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 526.1 s. Engine 0.1.0, rules r3, content 53c5b59a728530fb.

- Configs: configs/baseline.json
- Variants: insight_inc
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 96%, engine 96%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.89, engine 11.81; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 77%, engine 81%; skill gap +4 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 10.04, engine 10.12; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +0 → +4 pts win rate, -0.07 → +0.08 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 8.0 rounds at difficulty 0, 6.9 at 4 (target 8–10).
- Games ended by: corruption 42, deck_empty 6, opposition 336.
- Adversary operations: 10.0 per round on average across cells, at most 52 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, fab_line, firewall_node, fork_process, grid_battery, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rally, rebuild, recruitment_hub, server_farm, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 71% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 38% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine, 96% wins) plays 4.45 buildings and 2.94 tier-2 cards per game (of 38.0 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine, 81% wins) plays 3.64 buildings and 2.52 tier-2 cards per game (of 30.1 cards); 96% of its wins had 2+ buildings played (96% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: engine 81% vs greedy 77% over 96 seeds (only engine 12, only greedy 8), +4.2 ±4.7 pts, sign test p 0.503: no better than greedy (ahead, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.89) | 95.8 ±2.0 (11.81) | +0.0 pts (-0.07 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 77.1 ±4.3 (10.04) | 81.2 ±4.0 (10.12) | +4.2 pts (+0.08 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 95.8 | 95.8 | 89 / 3 / 3 / 1 | +0.0 ±2.6 | 1.000 | no better than greedy (level) |
| d4 engine | 96 | 81.2 | 77.1 | 66 / 12 / 8 / 10 | +4.2 ±4.7 | 0.503 | no better than greedy (ahead, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.81 (0.88, 5..10) | corruption 2, deck_empty 2, opposition 92 | 2.25 / 16.64 | 8.2 / 2.00 | 16.17 | 5.46 |
| d0 engine | 8.02 (1.15, 2..9) | corruption 4, opposition 92 | 1.96 / 17.79 | 16.0 / 1.75 | 17.05 | 5.51 |
| d4 greedy | 7.20 (1.09, 5..9) | corruption 18, deck_empty 4, opposition 74 | 1.69 / 8.12 | 14.6 / 1.36 | 6.46 | 5.43 |
| d4 engine | 6.94 (1.38, 2..9) | corruption 18, opposition 78 | 1.92 / 9.77 | 17.4 / 1.22 | 7.94 | 5.67 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 216.6 | 9.97 | 4% | – | – | 9.49 (42) | 0.00 | 9.23 | 0.00 | 5.74 / 15.62 (37%) |
| d0 engine | 279.4 | 10.20 | 4% | – | – | 10.14 (52) | 0.00 | 10.98 | 0.00 | 9.64 / 16.04 (60%) |
| d4 greedy | 199.2 | 9.40 | 4% | – | – | 10.24 (42) | 0.00 | 4.22 | 0.00 | 0.78 / 14.40 (5%) |
| d4 engine | 236.9 | 9.93 | 3% | – | – | 10.30 (35) | 0.00 | 6.58 | 0.00 | 2.27 / 13.88 (16%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 61% / 0% / 39% | 70% / 0% / 30% |
| d4 | 68% / 0% / 31% | 73% / 0% / 27% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 96 | 0.36 | 3 of 92 | 0.36 |
| d0 engine | 53 of 96 | 0.62 | 52 of 92 | 0.62 |
| d4 greedy | 3 of 96 | 0.36 | 3 of 74 | 0.34 |
| d4 engine | 43 of 96 | 0.60 | 38 of 78 | 0.59 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.88 | 2.03 | 20.1 | 88% (88%) |
| engine | 4.45 | 2.94 | 38.0 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.82 | 1.79 | 17.5 | 88% (88%) |
| engine | 3.64 | 2.52 | 30.1 | 96% (96%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (235) / 100% / 0% | 1.1 (215) / 99% / 3% |
| civic_shelter | 1.8 (192) / 100% / 0% | 1.8 (193) / 95% / 0% |
| contract_bid | 4.0 (192) / 99% / 0% | 3.6 (892) / 76% / 91% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 30% |
| data_mining | 3.6 (192) / 100% / 0% | 3.1 (750) / 81% / 80% |
| emergency_fab | 2.5 (191) / 100% / 0% | 2.4 (199) / 94% / 0% |
| evacuate | 3.0 (192) / 100% / 0% | 2.8 (253) / 89% / 22% |
| fab_line | 1.9 (194) / 99% / 0% | 1.9 (192) / 97% / 59% |
| firewall_node | 1.7 (192) / 100% / 0% | 1.8 (191) / 98% / 0% |
| fork_process | 2.2 (380) / 100% / 53% | 2.1 (498) / 92% / 63% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (192) / 99% / 0% | 3.1 (769) / 81% / 79% |
| grid_battery | 2.0 (192) / 100% / 0% | 2.0 (189) / 98% / 14% |
| jam_signals | 3.2 (190) / 100% / 0% | 2.8 (479) / 86% / 65% |
| liquidate_reserves | 3.6 (192) / 99% / 0% | 3.4 (323) / 73% / 24% |
| logistics_ai | 1.2 (206) / 100% / 0% | 1.3 (203) / 99% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (188) / 100% / 0% | 2.3 (192) / 98% / 79% |
| opposition_network | 1.5 (190) / 100% / 0% | 1.5 (187) / 99% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (224) / 100% / 0% | 1.0 (210) / 100% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (276) / 68% / 0% |
| precision_strike | 1.7 (892) / 100% / 93% | 1.9 (870) / 93% / 96% |
| purge_protocol | – (0) / – / 94% | – (0) / – / 98% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (654) / 99% / 88% | 2.4 (665) / 91% / 92% |
| rebuild | 2.0 (250) / 100% / 30% | 1.7 (417) / 97% / 69% |
| recon_sweep | 3.6 (186) / 100% / 0% | 3.5 (251) / 73% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 2% |
| recruitment_hub | 1.5 (192) / 100% / 0% | 1.5 (192) / 98% / 0% |
| scavenge | 3.6 (192) / 100% / 0% | 3.3 (319) / 69% / 27% |
| server_farm | 1.5 (192) / 100% / 85% | 1.5 (185) / 100% / 63% |
| shore_up | 3.0 (234) / 99% / 21% | 2.8 (322) / 82% / 36% |
| solar_field | 2.7 (191) / 100% / 0% | 2.7 (198) / 93% / 90% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (594) / 100% / 80% | 1.5 (765) / 96% / 90% |
| trade_office | 1.2 (209) / 100% / 0% | 1.2 (204) / 98% / 2% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.98/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (191) / 100% / 0% | 1.9 (186) / 99% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 12047.2 | 55.611 | 0.0 | 12105.4 |
| d0 engine | 23868.9 | 85.415 | 0.0 | 23947.3 |
| d4 greedy | 11490.2 | 57.695 | 0.0 | 11552.4 |
| d4 engine | 17277.6 | 72.920 | 0.0 | 17341.0 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-insight_inc-d4-engine-seed300068.json`](records/close-loss-c3-baseline-insight_inc-d4-engine-seed300068.json): close loss at difficulty 4: corruption in round 8, 2 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c3-baseline-insight_inc-d4-engine-seed300038.json`](records/close-loss-c3-baseline-insight_inc-d4-engine-seed300038.json): close loss at difficulty 4: corruption in round 7, 3 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-insight_inc-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-insight_inc-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-insight_inc-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-insight_inc-d4-greedy-seed300000.json): won by the weakest agent (greedy)

