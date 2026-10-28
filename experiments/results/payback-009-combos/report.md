# Cycle report: payback-009-combos

009 engine-payback sweep, config-only combinations of the single levers that helped the engine most (keep cost 0, factory credits 4, a longer deck). Engine agent against greedy, same seeds as payback-009-singles.

1152 games (12 cells × 96 seeds, 300000..=300095) on 12 threads in 2212.2 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: keep0+fc4 (cards.keep_cost=0 economy.factory_credits=4); keep0+deck444 (cards.keep_cost=0 adversary.stage_cards=4,4,4); keep0+fc4+deck444 (cards.keep_cost=0 economy.factory_credits=4 adversary.stage_cards=4,4,4)
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, keep0+fc4): win rates greedy 95%, engine 97%; skill gap +2 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, keep0+fc4): survival (rounds; wins count extra) greedy 11.84, engine 11.89; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, keep0+fc4): win rates greedy 82%, engine 83%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, keep0+fc4): survival (rounds; wins count extra) greedy 10.30, engine 10.28; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, keep0+deck444): win rates greedy 98%, engine 98%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, keep0+deck444): survival (rounds; wins count extra) greedy 13.89, engine 13.91; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, keep0+deck444): win rates greedy 86%, engine 80%; skill gap -6 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, keep0+deck444): survival (rounds; wins count extra) greedy 12.15, engine 11.85; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, keep0+fc4+deck444): win rates greedy 97%, engine 98%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, keep0+fc4+deck444): survival (rounds; wins count extra) greedy 13.82, engine 13.88; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, keep0+fc4+deck444): win rates greedy 85%, engine 78%; skill gap -7 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, keep0+fc4+deck444): survival (rounds; wins count extra) greedy 12.08, engine 11.74; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: +1 → -4 pts win rate, +0.04 → -0.22 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 7.7 rounds at difficulty 0, 7.0 at 4 (target 8–10).
- Games ended by: corruption 109, deck_empty 8, opposition 1035.
- Adversary operations: 9.9 per round on average across cells, at most 54 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 58% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 40% of threats countered.
- Strategy health at difficulty 0 (keep0+fc4): the top agent there (engine, 97% wins) plays 4.39 buildings and 2.65 tier-2 cards per game (of 38.2 cards); 99% of its wins had 2+ buildings played (99% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (keep0+fc4): the top agent there (engine, 83% wins) plays 2.98 buildings and 2.27 tier-2 cards per game (of 30.4 cards); 95% of its wins had 2+ buildings played (95% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (keep0+fc4): engine 83% vs greedy 82% over 96 seeds (only engine 11, only greedy 10), +1.0 ±4.8 pts, sign test p 1.000: no better than greedy (ahead, not significant).
- Variants against the base (keep0+fc4) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): keep0+deck444 +1.0 ±1.0; keep0+fc4+deck444 +1.0 ±1.8.
- Variants against the base (keep0+fc4) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): keep0+deck444 -3.1 ±3.8; keep0+fc4+deck444 -5.2 ±3.7.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| keep0+fc4 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 94.8 ±2.3 (11.84) | 96.9 ±1.8 (11.89) | +2.1 pts (+0.04 survival) |

| keep0+fc4 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 82.3 ±3.9 (10.30) | 83.3 ±3.8 (10.28) | +1.0 pts (-0.02 survival) |

| keep0+deck444 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 97.9 ±1.5 (13.89) | 97.9 ±1.5 (13.91) | +0.0 pts (+0.02 survival) |

| keep0+deck444 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 86.5 ±3.5 (12.15) | 80.2 ±4.1 (11.85) | -6.2 pts (-0.29 survival) |

| keep0+fc4+deck444 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 96.9 ±1.8 (13.82) | 97.9 ±1.5 (13.88) | +1.0 pts (+0.05 survival) |

| keep0+fc4+deck444 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 85.4 ±3.6 (12.08) | 78.1 ±4.2 (11.74) | -7.3 pts (-0.34 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| keep0+fc4 d0 engine | 96 | 96.9 | 94.8 | 88 / 5 / 3 / 0 | +2.1 ±3.0 | 0.727 | no better than greedy (ahead, not significant) |
| keep0+fc4 d4 engine | 96 | 83.3 | 82.3 | 69 / 11 / 10 / 6 | +1.0 ±4.8 | 1.000 | no better than greedy (ahead, not significant) |
| keep0+deck444 d0 engine | 96 | 97.9 | 97.9 | 92 / 2 / 2 / 0 | +0.0 ±2.1 | 1.000 | no better than greedy (level) |
| keep0+deck444 d4 engine | 96 | 80.2 | 86.5 | 70 / 7 / 13 / 6 | -6.2 ±4.6 | 0.263 | no better than greedy (behind, not significant) |
| keep0+fc4+deck444 d0 engine | 96 | 97.9 | 96.9 | 91 / 3 / 2 / 0 | +1.0 ±2.3 | 1.000 | no better than greedy (ahead, not significant) |
| keep0+fc4+deck444 d4 engine | 96 | 78.1 | 85.4 | 69 / 6 / 13 / 8 | -7.3 ±4.5 | 0.167 | no better than greedy (behind, not significant) |

## Variants against the base

The first variant, `keep0+fc4`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| keep0+deck444 | 0 | greedy | 94.8 → 97.9 | +3.1 ±1.8 | 0.250 | 7.75 → 7.76 |
| keep0+deck444 | 0 | engine | 96.9 → 97.9 | +1.0 ±1.0 | 1.000 | 7.73 → 8.30 |
| keep0+deck444 | 4 | greedy | 82.3 → 86.5 | +4.2 ±2.1 | 0.125 | 7.11 → 7.20 |
| keep0+deck444 | 4 | engine | 83.3 → 80.2 | -3.1 ±3.8 | 0.581 | 6.96 → 7.47 |
| keep0+fc4+deck444 | 0 | greedy | 94.8 → 96.9 | +2.1 ±1.5 | 0.500 | 7.75 → 7.77 |
| keep0+fc4+deck444 | 0 | engine | 96.9 → 97.9 | +1.0 ±1.8 | 1.000 | 7.73 → 8.27 |
| keep0+fc4+deck444 | 4 | greedy | 82.3 → 85.4 | +3.1 ±1.8 | 0.250 | 7.11 → 7.17 |
| keep0+fc4+deck444 | 4 | engine | 83.3 → 78.1 | -5.2 ±3.7 | 0.267 | 6.96 → 7.38 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| keep0+fc4 d0 greedy | 7.75 (0.98, 4..10) | corruption 3, deck_empty 2, opposition 91 | 2.32 / 17.43 | 8.2 / 2.00 | 16.73 | 5.52 |
| keep0+fc4 d0 engine | 7.73 (1.83, 2..10) | corruption 2, deck_empty 1, opposition 93 | 2.29 / 18.49 | 12.7 / 2.00 | 18.02 | 5.67 |
| keep0+fc4 d4 greedy | 7.11 (1.12, 4..9) | corruption 13, deck_empty 4, opposition 79 | 1.87 / 8.92 | 12.6 / 1.41 | 7.57 | 5.60 |
| keep0+fc4 d4 engine | 6.96 (1.73, 2..9) | corruption 16, opposition 80 | 1.99 / 10.50 | 14.4 / 1.56 | 8.75 | 5.61 |
| keep0+deck444 d0 greedy | 7.76 (1.03, 4..11) | corruption 2, opposition 94 | 4.26 / 17.09 | 10.5 / 2.00 | 16.73 | 5.54 |
| keep0+deck444 d0 engine | 8.30 (2.01, 2..12) | corruption 2, opposition 94 | 3.72 / 18.01 | 29.5 / 1.50 | 17.64 | 5.65 |
| keep0+deck444 d4 greedy | 7.20 (1.26, 4..11) | corruption 13, opposition 83 | 3.72 / 8.53 | 14.2 / 1.31 | 7.38 | 5.60 |
| keep0+deck444 d4 engine | 7.47 (1.83, 2..11) | corruption 18, deck_empty 1, opposition 77 | 3.47 / 10.27 | 15.7 / 1.47 | 8.26 | 5.66 |
| keep0+fc4+deck444 d0 greedy | 7.77 (1.04, 4..11) | corruption 3, opposition 93 | 4.25 / 17.27 | 9.7 / 2.00 | 16.73 | 5.52 |
| keep0+fc4+deck444 d0 engine | 8.27 (1.87, 2..10) | corruption 2, opposition 94 | 3.72 / 18.15 | 21.5 / 1.50 | 17.77 | 5.77 |
| keep0+fc4+deck444 d4 greedy | 7.17 (1.23, 4..11) | corruption 14, opposition 82 | 3.76 / 8.71 | 14.0 / 1.29 | 7.44 | 5.59 |
| keep0+fc4+deck444 d4 engine | 7.38 (1.82, 2..11) | corruption 21, opposition 75 | 3.59 / 10.85 | 17.3 / 1.38 | 8.48 | 5.67 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| keep0+fc4 d0 greedy | 217.9 | 10.86 | 4% | – | – | 8.92 (42) | 0.00 | 9.31 | 0.00 | 5.96 / 15.50 (38%) |
| keep0+fc4 d0 engine | 265.2 | 10.15 | 4% | – | – | 9.69 (44) | 0.00 | 10.12 | 0.00 | 9.26 / 15.46 (60%) |
| keep0+fc4 d4 greedy | 202.2 | 10.25 | 4% | – | – | 9.44 (35) | 0.00 | 4.18 | 0.00 | 0.86 / 14.23 (6%) |
| keep0+fc4 d4 engine | 232.0 | 9.78 | 4% | – | – | 10.43 (39) | 0.00 | 6.05 | 0.00 | 2.12 / 13.92 (15%) |
| keep0+deck444 d0 greedy | 217.7 | 10.80 | 4% | – | – | 8.94 (48) | 0.00 | 9.31 | 0.00 | 5.96 / 15.52 (38%) |
| keep0+deck444 d0 engine | 287.6 | 10.31 | 4% | – | – | 10.81 (54) | 0.00 | 10.90 | 0.00 | 10.52 / 16.60 (63%) |
| keep0+deck444 d4 greedy | 203.8 | 10.18 | 4% | – | – | 9.59 (46) | 0.00 | 4.19 | 0.00 | 0.86 / 14.40 (6%) |
| keep0+deck444 d4 engine | 254.3 | 9.89 | 4% | – | – | 11.12 (49) | 0.00 | 6.71 | 0.00 | 2.91 / 14.94 (19%) |
| keep0+fc4+deck444 d0 greedy | 218.1 | 10.86 | 4% | – | – | 8.93 (48) | 0.00 | 9.31 | 0.00 | 5.96 / 15.54 (38%) |
| keep0+fc4+deck444 d0 engine | 286.6 | 10.30 | 4% | – | – | 10.58 (50) | 0.00 | 10.89 | 0.00 | 10.52 / 16.54 (64%) |
| keep0+fc4+deck444 d4 greedy | 203.4 | 10.27 | 4% | – | – | 9.53 (46) | 0.00 | 4.18 | 0.00 | 0.86 / 14.33 (6%) |
| keep0+fc4+deck444 d4 engine | 251.9 | 9.85 | 4% | – | – | 11.13 (49) | 0.00 | 6.71 | 0.00 | 2.98 / 14.75 (20%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| keep0+fc4 d0 | 48% / 0% / 52% | 48% / 0% / 52% |
| keep0+fc4 d4 | 62% / 0% / 38% | 67% / 0% / 33% |
| keep0+deck444 d0 | 49% / 0% / 51% | 49% / 0% / 51% |
| keep0+deck444 d4 | 62% / 1% / 37% | 68% / 0% / 32% |
| keep0+fc4+deck444 d0 | 49% / 0% / 51% | 49% / 0% / 51% |
| keep0+fc4+deck444 d4 | 67% / 1% / 32% | 69% / 1% / 30% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| keep0+fc4 d0 greedy | 2 of 96 | 0.34 | 2 of 91 | 0.34 |
| keep0+fc4 d0 engine | 48 of 96 | 0.62 | 48 of 93 | 0.62 |
| keep0+fc4 d4 greedy | 2 of 96 | 0.34 | 2 of 79 | 0.33 |
| keep0+fc4 d4 engine | 29 of 96 | 0.59 | 25 of 80 | 0.58 |
| keep0+deck444 d0 greedy | 2 of 96 | 0.34 | 2 of 94 | 0.34 |
| keep0+deck444 d0 engine | 46 of 96 | 0.61 | 46 of 94 | 0.61 |
| keep0+deck444 d4 greedy | 2 of 96 | 0.34 | 2 of 83 | 0.33 |
| keep0+deck444 d4 engine | 53 of 96 | 0.62 | 45 of 77 | 0.62 |
| keep0+fc4+deck444 d0 greedy | 2 of 96 | 0.34 | 2 of 93 | 0.34 |
| keep0+fc4+deck444 d0 engine | 47 of 96 | 0.61 | 47 of 94 | 0.61 |
| keep0+fc4+deck444 d4 greedy | 2 of 96 | 0.34 | 2 of 82 | 0.33 |
| keep0+fc4+deck444 d4 engine | 49 of 96 | 0.62 | 42 of 75 | 0.62 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | keep0+fc4 | keep0+deck444 | keep0+fc4+deck444 |
|---|---|---|---|
| greedy | 2.00 / 1.93 / 20.8, 100% (100%) | 2.00 / 1.93 / 20.7, 100% (100%) | 2.00 / 1.93 / 20.8, 100% (100%) |
| engine | 4.39 / 2.65 / 38.2, 99% (99%) | 4.91 / 3.08 / 43.2, 99% (99%) | 5.02 / 3.12 / 43.1, 99% (99%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | keep0+fc4 | keep0+deck444 | keep0+fc4+deck444 |
|---|---|---|---|
| greedy | 2.00 / 1.70 / 18.9, 100% (100%) | 2.00 / 1.71 / 19.1, 100% (100%) | 2.00 / 1.70 / 19.0, 100% (100%) |
| engine | 2.98 / 2.27 / 30.4, 95% (95%) | 3.82 / 2.75 / 35.5, 99% (99%) | 3.94 / 2.69 / 35.2, 99% (99%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (697) / 100% / 0% | 1.1 (630) / 100% / 5% |
| civic_shelter | 1.8 (576) / 100% / 0% | 1.8 (543) / 100% / 0% |
| contract_bid | 4.0 (570) / 100% / 0% | 3.2 (2943) / 100% / 90% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 49% |
| data_mining | 3.6 (570) / 100% / 0% | 2.9 (2112) / 100% / 79% |
| emergency_fab | 2.5 (573) / 100% / 0% | 2.5 (557) / 100% / 0% |
| evacuate | 3.0 (576) / 100% / 0% | 2.9 (685) / 100% / 24% |
| fab_line | 1.9 (576) / 100% / 0% | 1.9 (557) / 100% / 59% |
| firewall_node | 1.7 (576) / 100% / 0% | 1.8 (556) / 100% / 0% |
| fork_process | 2.2 (1255) / 100% / 57% | 2.2 (1450) / 100% / 64% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (570) / 100% / 0% | 3.1 (1771) / 100% / 72% |
| grid_battery | 2.0 (576) / 100% / 0% | 2.0 (551) / 100% / 15% |
| jam_signals | 3.2 (573) / 100% / 0% | 2.7 (1524) / 100% / 70% |
| liquidate_reserves | 3.6 (564) / 100% / 0% | 3.5 (574) / 100% / 6% |
| logistics_ai | 1.2 (615) / 100% / 0% | 1.2 (592) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (564) / 100% / 0% | 2.3 (546) / 100% / 66% |
| opposition_network | 1.5 (570) / 100% / 0% | 1.4 (548) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (658) / 100% / 0% | 1.0 (631) / 100% / 0% |
| power_share | 3.6 (576) / 100% / 0% | 3.6 (558) / 100% / 0% |
| precision_strike | 1.7 (2785) / 100% / 93% | 1.9 (2702) / 100% / 93% |
| purge_protocol | – (0) / – / 91% | – (0) / – / 96% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (1947) / 100% / 89% | 2.4 (1880) / 100% / 89% |
| rebuild | 2.0 (763) / 100% / 31% | 1.7 (1470) / 100% / 76% |
| recon_sweep | 3.6 (555) / 100% / 0% | 3.6 (541) / 100% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 2% |
| recruitment_hub | 1.5 (579) / 100% / 0% | 1.5 (552) / 100% / 0% |
| scavenge | 3.6 (576) / 100% / 0% | 3.4 (615) / 100% / 12% |
| server_farm | 1.5 (576) / 100% / 100% | 1.5 (547) / 100% / 88% |
| shore_up | 3.0 (713) / 100% / 23% | 2.9 (808) / 100% / 34% |
| solar_field | 2.8 (570) / 100% / 0% | 2.7 (562) / 100% / 92% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (1885) / 100% / 82% | 1.5 (2513) / 100% / 90% |
| trade_office | 1.2 (626) / 100% / 0% | 1.2 (584) / 100% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (576) / 100% / 0% | 1.9 (556) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| keep0+fc4 d0 greedy | 18448.0 | 84.660 | 0.0 | 18531.5 |
| keep0+fc4 d0 engine | 34020.8 | 128.280 | 0.0 | 34158.2 |
| keep0+fc4 d4 greedy | 15240.1 | 75.368 | 0.0 | 15315.6 |
| keep0+fc4 d4 engine | 23692.5 | 102.114 | 0.0 | 23783.9 |
| keep0+deck444 d0 greedy | 17531.6 | 80.547 | 0.0 | 17621.6 |
| keep0+deck444 d0 engine | 37435.7 | 130.145 | 0.0 | 37550.0 |
| keep0+deck444 d4 greedy | 16022.7 | 78.639 | 0.0 | 16115.2 |
| keep0+deck444 d4 engine | 27815.1 | 109.360 | 0.0 | 27926.8 |
| keep0+fc4+deck444 d0 greedy | 12751.4 | 58.462 | 0.0 | 12818.2 |
| keep0+fc4+deck444 d0 engine | 33738.5 | 117.727 | 0.0 | 33864.7 |
| keep0+fc4+deck444 d4 greedy | 10488.0 | 51.572 | 0.0 | 10547.3 |
| keep0+fc4+deck444 d4 engine | 26625.0 | 105.698 | 0.0 | 26737.2 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c2-baseline-keep0_fc4-d4-greedy-seed300041.json`](records/close-loss-c2-baseline-keep0_fc4-d4-greedy-seed300041.json): close loss at difficulty 4: deck_empty in round 9, 4 adversary piece(s) over the level-2 bar, pool 13 of 14
- [`records/close-loss-c3-baseline-keep0_fc4-d4-engine-seed300037.json`](records/close-loss-c3-baseline-keep0_fc4-d4-engine-seed300037.json): close loss at difficulty 4: corruption in round 8, 4 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-keep0_fc4-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-keep0_fc4-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-keep0_fc4-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-keep0_fc4-d4-greedy-seed300000.json): won by the weakest agent (greedy)

