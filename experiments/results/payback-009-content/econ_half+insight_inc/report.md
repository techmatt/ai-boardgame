# Cycle report: payback-009-c-econ_half+insight_inc

009 content lever (separate binary, committed content untouched; scripts/payback/patch_content.py econ_half+insight_inc): econ_half+insight_inc. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 290.1 s. Engine 0.1.0, rules r3, content 25acdebeadca4472.

- Configs: configs/baseline.json
- Variants: econ_half+insight_inc
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 99%, engine 94%; skill gap -5 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 11.98, engine 11.76; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 73%, engine 85%; skill gap +12 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.90, engine 10.41; the ladder is ordered.
- Skill gap from difficulty 0 to 4: -5 → +12 pts win rate, -0.22 → +0.51 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 7.7 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 39, deck_empty 8, opposition 337.
- Adversary operations: 10.0 per round on average across cells, at most 45 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grid_battery, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rally, rebuild, recruitment_hub, server_farm, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 59% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 40% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 99% wins) plays 2.73 buildings and 2.03 tier-2 cards per game (of 20.9 cards); 98% of its wins had 2+ buildings played (98% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (engine, 85% wins) plays 4.89 buildings and 2.78 tier-2 cards per game (of 33.9 cards); 96% of its wins had 2+ buildings played (96% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: engine 85% vs greedy 73% over 96 seeds (only engine 18, only greedy 6), +12.5 ±5.0 pts, sign test p 0.023: beats greedy.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 99.0 ±1.0 (11.98) | 93.8 ±2.5 (11.76) | -5.2 pts (-0.22 survival) |

| difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 72.9 ±4.5 (9.90) | 85.4 ±3.6 (10.41) | +12.5 pts (+0.51 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 engine | 96 | 93.8 | 99.0 | 89 / 1 / 6 / 0 | -5.2 ±2.7 | 0.125 | no better than greedy (behind, not significant) |
| d4 engine | 96 | 85.4 | 72.9 | 64 / 18 / 6 / 8 | +12.5 ±5.0 | 0.023 | beats greedy |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 7.68 (0.82, 5..10) | corruption 1, opposition 95 | 2.35 / 17.02 | 12.0 / 2.00 | 16.84 | 5.49 |
| d0 engine | 7.71 (1.96, 2..10) | corruption 5, deck_empty 1, opposition 90 | 2.32 / 19.16 | 9.0 / 2.00 | 18.09 | 5.60 |
| d4 greedy | 7.00 (1.05, 4..9) | corruption 22, deck_empty 4, opposition 70 | 1.97 / 8.57 | 12.6 / 1.58 | 6.51 | 5.43 |
| d4 engine | 7.14 (1.49, 2..9) | corruption 11, deck_empty 3, opposition 82 | 1.83 / 9.84 | 16.3 / 1.29 | 8.78 | 5.68 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 216.8 | 10.51 | 4% | – | – | 9.21 (42) | 0.00 | 10.25 | 0.00 | 6.06 / 15.35 (39%) |
| d0 engine | 275.0 | 10.34 | 3% | – | – | 10.18 (45) | 0.00 | 10.73 | 0.00 | 9.49 / 15.42 (62%) |
| d4 greedy | 198.9 | 9.74 | 4% | – | – | 10.06 (42) | 0.00 | 4.71 | 0.00 | 0.81 / 14.00 (6%) |
| d4 engine | 249.8 | 10.15 | 3% | – | – | 10.74 (36) | 0.00 | 7.00 | 0.00 | 2.54 / 14.27 (18%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 48% / 2% / 50% | 48% / 2% / 49% |
| d4 | 62% / 2% / 36% | 69% / 2% / 29% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 12 of 96 | 0.50 | 12 of 95 | 0.50 |
| d0 engine | 76 of 96 | 0.65 | 74 of 90 | 0.65 |
| d4 greedy | 12 of 96 | 0.50 | 12 of 70 | 0.48 |
| d4 engine | 56 of 96 | 0.62 | 50 of 82 | 0.62 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.73 | 2.03 | 20.9 | 98% (98%) |
| engine | 5.70 | 2.94 | 39.4 | 99% (99%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 2.64 | 1.73 | 18.4 | 94% (94%) |
| engine | 4.89 | 2.78 | 33.9 | 96% (96%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.8 (194) / 99% / 0% | 1.8 (187) / 99% / 14% |
| civic_shelter | 1.5 (192) / 100% / 0% | 1.5 (191) / 97% / 0% |
| contract_bid | 3.9 (191) / 99% / 0% | 3.6 (896) / 82% / 90% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 39% |
| data_mining | 3.6 (192) / 100% / 0% | 3.1 (753) / 86% / 81% |
| emergency_fab | 2.3 (191) / 100% / 0% | 2.3 (189) / 96% / 0% |
| evacuate | 2.9 (193) / 100% / 1% | 2.7 (259) / 91% / 28% |
| fab_line | 2.3 (192) / 100% / 0% | 2.3 (197) / 93% / 95% |
| firewall_node | 1.4 (195) / 100% / 0% | 1.4 (185) / 100% / 0% |
| fork_process | 2.2 (391) / 100% / 55% | 2.2 (505) / 95% / 65% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 9% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (190) / 98% / 0% | 3.1 (802) / 84% / 81% |
| grid_battery | 2.3 (192) / 100% / 0% | 2.3 (192) / 95% / 41% |
| jam_signals | 3.1 (189) / 100% / 0% | 2.7 (491) / 89% / 66% |
| liquidate_reserves | 3.6 (191) / 99% / 0% | 3.5 (262) / 84% / 21% |
| logistics_ai | 1.0 (247) / 100% / 0% | 1.0 (222) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 1% |
| microgrid | 2.6 (190) / 100% / 44% | 2.6 (191) / 95% / 96% |
| opposition_network | 1.2 (214) / 100% / 0% | 1.2 (196) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.8 (191) / 100% / 0% | 1.8 (192) / 98% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (259) / 71% / 0% |
| precision_strike | 1.6 (854) / 100% / 93% | 1.9 (867) / 95% / 94% |
| purge_protocol | – (0) / – / 95% | – (0) / – / 97% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (647) / 98% / 90% | 2.4 (681) / 91% / 92% |
| rebuild | 1.7 (249) / 100% / 31% | 1.5 (413) / 99% / 69% |
| recon_sweep | 3.6 (188) / 100% / 0% | 3.5 (234) / 77% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 20% |
| recruitment_hub | 1.2 (216) / 100% / 0% | 1.2 (198) / 99% / 0% |
| scavenge | 3.6 (193) / 99% / 0% | 3.4 (289) / 75% / 22% |
| server_farm | 1.2 (194) / 100% / 84% | 1.2 (192) / 98% / 64% |
| shore_up | 2.8 (247) / 100% / 26% | 2.7 (313) / 87% / 39% |
| solar_field | 3.3 (194) / 99% / 40% | 3.2 (216) / 84% / 99% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.4 (578) / 100% / 82% | 1.3 (766) / 98% / 90% |
| trade_office | 2.0 (192) / 100% / 0% | 2.0 (194) / 97% / 27% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.6 (193) / 100% / 0% | 1.6 (192) / 99% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 6448.4 | 29.740 | 0.0 | 6478.8 |
| d0 engine | 14181.4 | 51.563 | 0.0 | 14228.4 |
| d4 greedy | 5482.9 | 27.559 | 0.0 | 5510.0 |
| d4 engine | 9667.1 | 38.693 | 0.0 | 9700.7 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-econ_half_insight_inc-d4-engine-seed300066.json`](records/close-loss-c3-baseline-econ_half_insight_inc-d4-engine-seed300066.json): close loss at difficulty 4: deck_empty in round 9, 1 adversary piece(s) over the level-2 bar, pool 14 of 14
- [`records/close-loss-c2-baseline-econ_half_insight_inc-d4-greedy-seed300027.json`](records/close-loss-c2-baseline-econ_half_insight_inc-d4-greedy-seed300027.json): close loss at difficulty 4: deck_empty in round 9, 2 adversary piece(s) over the level-2 bar, pool 5 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-econ_half_insight_inc-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-econ_half_insight_inc-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-econ_half_insight_inc-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-econ_half_insight_inc-d4-greedy-seed300000.json): won by the weakest agent (greedy)

