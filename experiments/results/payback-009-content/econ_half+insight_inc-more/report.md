# Cycle report: payback-009-c-econ_half+insight_inc-more

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py econ_half+insight_inc): econ_half+insight_inc. Engine agent against greedy, same seeds as payback-009-singles. Seeds 300096+ (144 more), to firm up the 96-seed result.

576 games (4 cells × 144 seeds, 300096..=300239) on 12 threads in 433.0 s. Engine 0.1.0, rules r3, content 25acdebeadca4472.

- Configs: configs/baseline.json
- Variants: econ_half+insight_inc
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 95%, engine 97%; skill gap +2 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.76, engine 11.90; the ladder is ordered.
- Difficulty 4: win rates greedy 69%, engine 81%; skill gap +12 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.53, engine 10.10; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +2 → +12 pts win rate, +0.14 → +0.57 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 8.1 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 81, deck_empty 2, opposition 492, population 1.
- Adversary operations: 10.9 per round on average across cells, at most 50 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grid_battery, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rally, rebuild, recruitment_hub, server_farm, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 66% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 39% of threats countered.
- Strategy health at difficulty 0: the top agent there (engine, 97% wins) plays 5.78 buildings and 3.32 tier-2 cards per game (of 43.0 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine, 81% wins) plays 5.01 buildings and 2.74 tier-2 cards per game (of 33.7 cards); 98% of its wins had 2+ buildings played (98% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: engine 81% vs greedy 69% over 144 seeds (only engine 23, only greedy 6), +11.8 ±3.6 pts, sign test p 0.002: beats greedy.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.1 ±1.8 (11.76) | 97.2 ±1.4 (11.90) | +2.1 pts (+0.14 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 68.8 ±3.9 (9.53) | 80.6 ±3.3 (10.10) | +11.8 pts (+0.57 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 144 | 97.2 | 95.1 | 134 / 6 / 3 / 1 | +2.1 ±2.1 | 0.508 | no better than greedy (ahead, not significant) |
| d4 engine | 144 | 80.6 | 68.8 | 93 / 23 / 6 / 22 | +11.8 ±3.6 | 0.002 | beats greedy |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.72 (1.04, 2..10) | corruption 5, deck_empty 1, opposition 137, population 1 | 2.25 / 16.34 | 12.4 / 1.71 | 15.69 | 5.24 |
| d0 engine | 8.10 (1.42, 2..10) | corruption 3, deck_empty 1, opposition 140 | 1.91 / 18.76 | 11.5 / 2.00 | 18.30 | 5.56 |
| d4 greedy | 7.06 (1.08, 2..9) | corruption 45, opposition 99 | 1.61 / 7.83 | 17.2 / 1.31 | 5.38 | 5.37 |
| d4 engine | 7.13 (1.24, 2..9) | corruption 28, opposition 116 | 1.69 / 10.06 | 20.5 / 1.14 | 8.10 | 5.63 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 220.2 | 10.51 | 4% | – | – | 10.20 (41) | 0.00 | 10.27 | 0.00 | 5.88 / 15.44 (38%) |
| d0 engine | 293.2 | 10.64 | 3% | – | – | 10.83 (46) | 0.00 | 11.51 | 0.00 | 10.05 / 16.21 (62%) |
| d4 greedy | 201.6 | 9.67 | 4% | – | – | 11.11 (41) | 0.00 | 4.55 | 0.00 | 0.78 / 14.11 (6%) |
| d4 engine | 249.9 | 10.11 | 3% | – | – | 11.34 (50) | 0.00 | 6.88 | 0.00 | 2.36 / 14.26 (17%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 57% / 0% / 42% | 57% / 1% / 42% |
| d4 | 73% / 2% / 25% | 75% / 2% / 23% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 12 of 144 | 0.50 | 12 of 137 | 0.50 |
| d0 engine | 102 of 144 | 0.64 | 99 of 140 | 0.64 |
| d4 greedy | 12 of 144 | 0.50 | 12 of 99 | 0.49 |
| d4 engine | 76 of 144 | 0.62 | 69 of 116 | 0.62 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.77 | 2.11 | 21.5 | 95% (95%) |
| engine | 5.78 | 3.32 | 43.0 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.71 | 1.69 | 18.8 | 91% (91%) |
| engine | 5.01 | 2.74 | 33.7 | 98% (98%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.9 (285) / 100% / 0% | 1.9 (293) / 98% / 13% |
| civic_shelter | 1.6 (285) / 100% / 0% | 1.6 (287) / 99% / 0% |
| contract_bid | 4.0 (283) / 100% / 0% | 3.6 (1400) / 82% / 91% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 41% |
| data_mining | 3.6 (286) / 100% / 0% | 3.1 (1092) / 87% / 80% |
| emergency_fab | 2.3 (288) / 100% / 0% | 2.3 (290) / 99% / 1% |
| evacuate | 2.9 (283) / 100% / 1% | 2.8 (378) / 93% / 29% |
| fab_line | 2.3 (283) / 100% / 0% | 2.3 (287) / 97% / 98% |
| firewall_node | 1.5 (288) / 100% / 0% | 1.5 (283) / 99% / 0% |
| fork_process | 2.0 (636) / 99% / 57% | 2.1 (711) / 93% / 61% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 6% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.7 (287) / 100% / 0% | 3.1 (1255) / 85% / 84% |
| grid_battery | 2.2 (289) / 99% / 0% | 2.2 (298) / 96% / 42% |
| jam_signals | 3.2 (279) / 100% / 0% | 2.8 (762) / 89% / 67% |
| liquidate_reserves | 3.7 (287) / 99% / 0% | 3.5 (442) / 81% / 26% |
| logistics_ai | 1.0 (364) / 100% / 0% | 1.0 (329) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 1% |
| microgrid | 2.6 (284) / 100% / 47% | 2.6 (294) / 96% / 98% |
| opposition_network | 1.2 (330) / 100% / 0% | 1.2 (301) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.9 (285) / 100% / 0% | 1.9 (275) / 100% / 0% |
| power_share | 3.6 (286) / 99% / 0% | 3.6 (353) / 79% / 0% |
| precision_strike | 1.7 (1293) / 100% / 93% | 1.9 (1342) / 96% / 96% |
| purge_protocol | – (0) / – / 95% | – (0) / – / 98% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (994) / 99% / 91% | 2.4 (1059) / 91% / 94% |
| rebuild | 1.7 (384) / 99% / 33% | 1.5 (681) / 97% / 75% |
| recon_sweep | 3.5 (285) / 100% / 0% | 3.5 (346) / 81% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 25% |
| recruitment_hub | 1.2 (323) / 100% / 0% | 1.2 (301) / 100% / 0% |
| scavenge | 3.7 (281) / 99% / 0% | 3.4 (436) / 81% / 32% |
| server_farm | 1.2 (306) / 100% / 81% | 1.2 (305) / 100% / 57% |
| shore_up | 2.8 (394) / 99% / 31% | 2.6 (507) / 89% / 43% |
| solar_field | 3.2 (282) / 100% / 41% | 3.1 (335) / 83% / 97% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.4 (934) / 100% / 83% | 1.3 (1230) / 97% / 92% |
| trade_office | 2.1 (286) / 100% / 0% | 2.1 (285) / 99% / 30% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.4 (284) / 100% / 0% | 1.5 (283) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 6759.4 | 30.692 | 0.0 | 6789.8 |
| d0 engine | 13612.2 | 46.423 | 0.0 | 13654.9 |
| d4 greedy | 5669.1 | 28.117 | 0.0 | 5696.1 |
| d4 engine | 9632.3 | 38.548 | 0.0 | 9664.7 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c2-baseline-econ_half_insight_inc-d4-greedy-seed300236.json`](records/close-loss-c2-baseline-econ_half_insight_inc-d4-greedy-seed300236.json): close loss at difficulty 4: corruption in round 7, 1 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c2-baseline-econ_half_insight_inc-d4-greedy-seed300132.json`](records/close-loss-c2-baseline-econ_half_insight_inc-d4-greedy-seed300132.json): close loss at difficulty 4: corruption in round 7, 3 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-econ_half_insight_inc-d0-greedy-seed300096.json`](records/surprising-win-c0-baseline-econ_half_insight_inc-d0-greedy-seed300096.json): won by the weakest agent (greedy)
- [`records/surprising-win-c0-baseline-econ_half_insight_inc-d0-greedy-seed300097.json`](records/surprising-win-c0-baseline-econ_half_insight_inc-d0-greedy-seed300097.json): won by the weakest agent (greedy)

