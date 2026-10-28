# Cycle report: screen-011-clock

011 screen (no probes), run on the 009 content patch t2_half+t2_noam (cheap tier 2) built by scripts/payback/build_patches.sh: does the deck-clock weight w.clock (shortfall against the top opposition level) stop the engine agent running out the deck? 009's seeds; rules r4 (clear-board win from stage II).

480 games (10 cells × 48 seeds, 300000..=300047) on 12 threads in 681.4 s. Engine 0.1.0, rules r4, content 2f31b40bb42daaae.

- Configs: configs/baseline.json
- Variants: base
- Difficulties: 0, 4
- Agents (weakest first): greedy, greedy:w.clock=10, engine:w.clock=0, engine:w.clock=10, engine:w.clock=30
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates greedy 100%, greedy:w.clock=10 98%, engine:w.clock=0 60%, engine:w.clock=10 56%, engine:w.clock=30 62%; skill gap -38 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) greedy 12.00, greedy:w.clock=10 11.90, engine:w.clock=0 11.08, engine:w.clock=10 10.92, engine:w.clock=30 11.02; **the ladder is not ordered on survival**.
- Difficulty 4: win rates greedy 67%, greedy:w.clock=10 75%, engine:w.clock=0 31%, engine:w.clock=10 46%, engine:w.clock=30 46%; skill gap -29 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) greedy 9.58, greedy:w.clock=10 9.88, engine:w.clock=0 8.98, engine:w.clock=10 9.10, engine:w.clock=30 9.08; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: -38 → -29 pts win rate, -0.92 → -0.77 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy:w.clock=10): 7.9 rounds at difficulty 0, 7.4 at 4 (target 8–10).
- Games ended by: corruption 101, deck_empty 71, opposition 308.
- Adversary operations: 11.2 per round on average across cells, at most 47 in one round (target: low).
- Cards (greedy:w.clock=10, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 52% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 25% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 100% wins) plays 1.92 buildings and 1.92 tier-2 cards per game (of 21.8 cards); 92% of its wins had 2+ buildings played (92% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy:w.clock=10, 75% wins) plays 1.79 buildings and 1.58 tier-2 cards per game (of 18.9 cards); 94% of its wins had 2+ buildings played (94% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy:w.clock=10: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4: greedy:w.clock=10 75% vs greedy 67% over 48 seeds (only greedy:w.clock=10 6, only greedy 2), +8.3 ±5.8 pts, sign test p 0.289: no better than greedy (ahead, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | greedy | greedy:w.clock=10 | engine:w.clock=0 | engine:w.clock=10 | engine:w.clock=30 | skill gap |
|---|---|---|---|---|---|---|
| win % (survival) | 100.0 ±0.0 (12.00) | 97.9 ±2.1 (11.90) | 60.4 ±7.1 (11.08) | 56.2 ±7.2 (10.92) | 62.5 ±7.0 (11.02) | -37.5 pts (-0.92 survival) |

| difficulty 4 | greedy | greedy:w.clock=10 | engine:w.clock=0 | engine:w.clock=10 | engine:w.clock=30 | skill gap |
|---|---|---|---|---|---|---|
| win % (survival) | 66.7 ±6.8 (9.58) | 75.0 ±6.2 (9.88) | 31.2 ±6.7 (8.98) | 45.8 ±7.2 (9.10) | 45.8 ±7.2 (9.08) | -29.2 pts (-0.77 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 greedy:w.clock=10 | 48 | 97.9 | 100.0 | 47 / 0 / 1 / 0 | -2.1 ±2.1 | 1.000 | no better than greedy (behind, not significant) |
| d0 engine:w.clock=0 | 48 | 60.4 | 100.0 | 29 / 0 / 19 / 0 | -39.6 ±7.1 | <0.001 | worse than greedy |
| d0 engine:w.clock=10 | 48 | 56.2 | 100.0 | 27 / 0 / 21 / 0 | -43.8 ±7.2 | <0.001 | worse than greedy |
| d0 engine:w.clock=30 | 48 | 62.5 | 100.0 | 30 / 0 / 18 / 0 | -37.5 ±7.1 | <0.001 | worse than greedy |
| d4 greedy:w.clock=10 | 48 | 75.0 | 66.7 | 30 / 6 / 2 / 10 | +8.3 ±5.8 | 0.289 | no better than greedy (ahead, not significant) |
| d4 engine:w.clock=0 | 48 | 31.2 | 66.7 | 9 / 6 / 23 / 10 | -35.4 ±10.1 | 0.002 | worse than greedy |
| d4 engine:w.clock=10 | 48 | 45.8 | 66.7 | 17 / 5 / 15 / 11 | -20.8 ±8.9 | 0.041 | worse than greedy |
| d4 engine:w.clock=30 | 48 | 45.8 | 66.7 | 17 / 5 / 15 / 11 | -20.8 ±8.9 | 0.041 | worse than greedy |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 greedy | 8.12 (0.78, 7..10) | opposition 48 | 1.88 / 16.69 | – / – | 16.69 | 5.62 |
| d0 greedy:w.clock=10 | 7.90 (0.82, 6..9) | corruption 1, opposition 47 | 2.09 / 16.04 | 42.0 / 1.00 | 15.71 | 5.52 |
| d0 engine:w.clock=0 | 9.60 (0.67, 8..10) | corruption 4, deck_empty 15, opposition 29 | 0.45 / 17.93 | 15.8 / 1.84 | 14.73 | 5.50 |
| d0 engine:w.clock=10 | 9.54 (0.73, 8..10) | corruption 7, deck_empty 14, opposition 27 | 0.44 / 16.93 | 14.0 / 1.86 | 13.35 | 5.54 |
| d0 engine:w.clock=30 | 9.52 (0.79, 7..10) | corruption 6, deck_empty 12, opposition 30 | 0.40 / 16.43 | 13.4 / 1.83 | 13.79 | 5.52 |
| d4 greedy | 7.44 (1.08, 5..9) | corruption 15, deck_empty 1, opposition 32 | 1.22 / 8.91 | 17.8 / 1.25 | 6.21 | 5.38 |
| d4 greedy:w.clock=10 | 7.38 (1.15, 5..9) | corruption 12, opposition 36 | 1.33 / 8.22 | 17.8 / 1.33 | 6.17 | 5.31 |
| d4 engine:w.clock=0 | 8.17 (1.12, 5..9) | corruption 18, deck_empty 15, opposition 15 | 0.60 / 8.00 | 14.5 / 1.42 | 5.25 | 5.31 |
| d4 engine:w.clock=10 | 7.96 (1.22, 5..9) | corruption 19, deck_empty 7, opposition 22 | 0.50 / 9.36 | 17.7 / 1.35 | 5.23 | 5.44 |
| d4 engine:w.clock=30 | 7.94 (1.23, 5..9) | corruption 19, deck_empty 7, opposition 22 | 0.50 / 9.41 | 18.5 / 1.31 | 5.25 | 5.46 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 greedy | 224.2 | 10.23 | 4% | – | – | 9.43 (44) | 0.00 | 10.42 | 0.00 | 7.10 / 16.25 (44%) |
| d0 greedy:w.clock=10 | 220.1 | 10.11 | 4% | – | – | 9.44 (42) | 0.00 | 10.15 | 0.00 | 6.96 / 15.79 (44%) |
| d0 engine:w.clock=0 | 333.9 | 10.54 | 4% | – | – | 12.68 (47) | 0.00 | 14.92 | 0.00 | 13.48 / 19.21 (70%) |
| d0 engine:w.clock=10 | 333.8 | 10.51 | 4% | – | – | 12.79 (41) | 0.00 | 14.69 | 0.00 | 13.40 / 19.08 (70%) |
| d0 engine:w.clock=30 | 332.9 | 10.51 | 4% | – | – | 12.63 (42) | 0.00 | 14.69 | 0.00 | 13.29 / 19.04 (70%) |
| d4 greedy | 206.0 | 9.45 | 4% | – | – | 9.70 (36) | 0.00 | 4.35 | 0.00 | 0.90 / 14.88 (6%) |
| d4 greedy:w.clock=10 | 205.3 | 9.45 | 4% | – | – | 9.88 (41) | 0.00 | 4.27 | 0.00 | 0.90 / 14.75 (6%) |
| d4 engine:w.clock=0 | 277.1 | 10.01 | 4% | – | – | 12.01 (36) | 0.00 | 9.60 | 0.00 | 4.23 / 16.33 (26%) |
| d4 engine:w.clock=10 | 268.5 | 9.87 | 4% | – | – | 11.71 (37) | 0.00 | 9.21 | 0.00 | 3.67 / 15.92 (23%) |
| d4 engine:w.clock=30 | 267.9 | 9.87 | 4% | – | – | 11.70 (37) | 0.00 | 9.19 | 0.00 | 3.67 / 15.88 (23%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 34% / 20% / 45% | 42% / 16% / 42% |
| d4 | 42% / 10% / 48% | 62% / 3% / 35% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 greedy | 3 of 48 | 0.34 | 3 of 48 | 0.34 |
| d0 greedy:w.clock=10 | 3 of 48 | 0.34 | 3 of 47 | 0.34 |
| d0 engine:w.clock=0 | 31 of 48 | 0.60 | 20 of 29 | 0.60 |
| d0 engine:w.clock=10 | 31 of 48 | 0.60 | 20 of 27 | 0.59 |
| d0 engine:w.clock=30 | 31 of 48 | 0.60 | 21 of 30 | 0.61 |
| d4 greedy | 3 of 48 | 0.33 | 3 of 32 | 0.33 |
| d4 greedy:w.clock=10 | 3 of 48 | 0.33 | 3 of 36 | 0.31 |
| d4 engine:w.clock=0 | 20 of 48 | 0.54 | 9 of 15 | 0.48 |
| d4 engine:w.clock=10 | 20 of 48 | 0.54 | 13 of 22 | 0.52 |
| d4 engine:w.clock=30 | 20 of 48 | 0.54 | 13 of 22 | 0.52 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.92 | 1.92 | 21.8 | 92% (92%) |
| greedy:w.clock=10 | 1.90 | 1.60 | 21.2 | 91% (91%) |
| engine:w.clock=0 | 3.85 | 2.38 | 48.1 | 100% (100%) |
| engine:w.clock=10 | 3.81 | 2.19 | 47.4 | 100% (100%) |
| engine:w.clock=30 | 3.81 | 2.19 | 47.2 | 100% (100%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| greedy | 1.81 | 1.48 | 19.0 | 91% (91%) |
| greedy:w.clock=10 | 1.79 | 1.58 | 18.9 | 94% (94%) |
| engine:w.clock=0 | 2.83 | 2.48 | 35.5 | 93% (93%) |
| engine:w.clock=10 | 2.83 | 2.17 | 33.6 | 95% (95%) |
| engine:w.clock=30 | 2.83 | 2.17 | 33.5 | 95% (95%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | greedy:w.clock=10: pick (offered) / keep / play | engine:w.clock=0: pick (offered) / keep / play | engine:w.clock=10: pick (offered) / keep / play | engine:w.clock=30: pick (offered) / keep / play |
|---|---|---|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (125) / 100% / 0% | 1.1 (124) / 100% / 0% | 1.1 (106) / 98% / 2% | 1.1 (108) / 97% / 2% | 1.1 (109) / 97% / 2% |
| civic_shelter | 1.9 (93) / 100% / 0% | 1.9 (93) / 100% / 0% | 1.8 (95) / 96% / 0% | 1.8 (95) / 96% / 0% | 1.8 (95) / 96% / 0% |
| contract_bid | 3.9 (92) / 98% / 0% | 3.9 (92) / 98% / 0% | 3.2 (587) / 79% / 94% | 3.3 (571) / 78% / 92% | 3.3 (569) / 78% / 92% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 78% | – (0) / – / 81% | – (0) / – / 81% |
| data_mining | 3.7 (94) / 100% / 0% | 3.7 (94) / 100% / 0% | 2.9 (437) / 85% / 79% | 2.9 (434) / 85% / 80% | 2.9 (437) / 85% / 80% |
| emergency_fab | 2.4 (95) / 100% / 0% | 2.4 (95) / 100% / 0% | 2.4 (103) / 92% / 0% | 2.4 (103) / 92% / 0% | 2.4 (103) / 92% / 0% |
| evacuate | 2.9 (94) / 100% / 0% | 2.9 (95) / 100% / 1% | 2.8 (124) / 94% / 29% | 2.7 (125) / 96% / 34% | 2.7 (125) / 96% / 34% |
| fab_line | 1.9 (96) / 98% / 0% | 1.9 (96) / 98% / 0% | 1.9 (97) / 99% / 28% | 1.9 (97) / 99% / 26% | 1.9 (97) / 99% / 26% |
| firewall_node | 1.7 (94) / 100% / 0% | 1.7 (94) / 100% / 0% | 1.7 (90) / 100% / 0% | 1.7 (89) / 100% / 0% | 1.7 (89) / 100% / 0% |
| fork_process | 2.3 (198) / 99% / 57% | 2.3 (192) / 99% / 55% | 2.2 (257) / 95% / 65% | 2.2 (254) / 95% / 64% | 2.2 (253) / 95% / 64% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 13% | – (0) / – / 12% | – (0) / – / 12% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.7 (97) / 98% / 0% | 3.7 (97) / 98% / 0% | 3.0 (428) / 81% / 81% | 3.0 (419) / 83% / 79% | 3.0 (418) / 83% / 80% |
| grid_battery | 2.1 (96) / 100% / 0% | 2.1 (96) / 100% / 0% | 2.0 (97) / 99% / 10% | 2.0 (97) / 99% / 10% | 2.0 (97) / 99% / 10% |
| jam_signals | 3.2 (94) / 100% / 0% | 3.2 (94) / 100% / 0% | 2.6 (258) / 91% / 64% | 2.6 (259) / 92% / 65% | 2.6 (260) / 91% / 65% |
| liquidate_reserves | 3.6 (93) / 100% / 0% | 3.6 (93) / 100% / 0% | 3.5 (139) / 81% / 19% | 3.4 (140) / 79% / 19% | 3.4 (140) / 79% / 19% |
| logistics_ai | 1.2 (106) / 100% / 0% | 1.2 (103) / 100% / 0% | 1.3 (101) / 99% / 0% | 1.3 (97) / 100% / 0% | 1.3 (97) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (92) / 100% / 0% | 2.3 (92) / 100% / 0% | 2.3 (97) / 99% / 40% | 2.3 (98) / 97% / 40% | 2.3 (98) / 97% / 40% |
| opposition_network | 1.4 (96) / 100% / 0% | 1.4 (96) / 100% / 0% | 1.5 (92) / 100% / 0% | 1.5 (92) / 100% / 0% | 1.5 (92) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (125) / 100% / 0% | 1.0 (123) / 100% / 0% | 1.0 (119) / 99% / 0% | 1.0 (119) / 99% / 0% | 1.0 (119) / 99% / 0% |
| power_share | 3.6 (98) / 98% / 0% | 3.6 (98) / 98% / 0% | 3.5 (135) / 70% / 0% | 3.5 (138) / 68% / 0% | 3.5 (138) / 68% / 0% |
| precision_strike | 1.6 (452) / 100% / 97% | 1.6 (444) / 100% / 96% | 1.9 (554) / 92% / 93% | 1.8 (553) / 92% / 94% | 1.8 (552) / 93% / 94% |
| purge_protocol | – (0) / – / 97% | – (0) / – / 99% | – (0) / – / 77% | – (0) / – / 82% | – (0) / – / 84% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (307) / 99% / 94% | 2.1 (337) / 98% / 96% | 2.3 (419) / 91% / 92% | 2.3 (437) / 88% / 93% | 2.3 (436) / 88% / 93% |
| rebuild | 2.1 (124) / 100% / 34% | 2.1 (123) / 100% / 35% | 1.7 (254) / 98% / 77% | 1.7 (239) / 97% / 78% | 1.7 (243) / 96% / 79% |
| recon_sweep | 3.7 (92) / 100% / 0% | 3.7 (92) / 100% / 0% | 3.5 (122) / 76% / 0% | 3.5 (126) / 74% / 0% | 3.5 (126) / 74% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 1% | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.6 (93) / 100% / 0% | 1.6 (93) / 100% / 0% | 1.6 (100) / 96% / 0% | 1.6 (100) / 96% / 0% | 1.6 (100) / 96% / 0% |
| scavenge | 3.6 (96) / 100% / 0% | 3.6 (96) / 100% / 0% | 3.4 (131) / 71% / 2% | 3.4 (129) / 71% / 0% | 3.4 (129) / 71% / 0% |
| server_farm | 1.4 (95) / 100% / 86% | 1.4 (95) / 100% / 84% | 1.5 (95) / 100% / 72% | 1.4 (96) / 99% / 72% | 1.4 (96) / 99% / 72% |
| shore_up | 2.9 (126) / 98% / 28% | 2.9 (128) / 98% / 29% | 2.7 (170) / 84% / 48% | 2.8 (166) / 84% / 47% | 2.8 (166) / 84% / 46% |
| solar_field | 2.6 (95) / 100% / 0% | 2.6 (95) / 100% / 0% | 2.6 (97) / 97% / 71% | 2.6 (96) / 97% / 72% | 2.6 (96) / 97% / 72% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (276) / 100% / 82% | 1.6 (253) / 100% / 77% | 1.4 (494) / 97% / 92% | 1.4 (474) / 96% / 92% | 1.4 (471) / 96% / 92% |
| trade_office | 1.3 (104) / 100% / 0% | 1.3 (105) / 100% / 0% | 1.2 (103) / 100% / 0% | 1.3 (103) / 99% / 0% | 1.3 (103) / 99% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 0.99/game | – (0) / – / 0.99/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.8 (96) / 100% / 0% | 1.8 (96) / 100% / 0% | 1.7 (93) / 98% / 0% | 1.7 (95) / 97% / 0% | 1.7 (95) / 97% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 greedy | 8368.2 | 37.316 | 0.0 | 8410.2 |
| d0 greedy:w.clock=10 | 7807.3 | 35.464 | 0.0 | 7842.4 |
| d0 engine:w.clock=0 | 35186.0 | 105.393 | 0.0 | 35287.8 |
| d0 engine:w.clock=10 | 30156.1 | 90.333 | 0.0 | 30258.3 |
| d0 engine:w.clock=30 | 29617.0 | 88.979 | 0.0 | 29711.8 |
| d4 greedy | 5561.1 | 27.001 | 0.0 | 5588.7 |
| d4 greedy:w.clock=10 | 10330.0 | 50.314 | 0.0 | 10387.4 |
| d4 engine:w.clock=0 | 19304.3 | 69.670 | 0.0 | 19376.8 |
| d4 engine:w.clock=10 | 10443.1 | 38.900 | 0.0 | 10478.9 |
| d4 engine:w.clock=30 | 10512.3 | 39.234 | 0.0 | 10549.7 |

## Saved records

None matched.
