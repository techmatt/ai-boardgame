# Cycle report: engine-agent-008

008: does a building engine beat no-building play? The engine agent (horizon valuation: income and buildings worth what they pay over the rounds left; weights from the 008 sweep) against greedy on the same seeds at the tuned baseline, difficulties 0 and 4. Also 007's building greedy, and the engine agent forbidden to build (w.built=-100) as a control that differs only in building. Strategy health for every agent.

2400 games (10 cells × 240 seeds, 300000..=300239) on 12 threads in 1329.5 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: base
- Difficulties: 0, 4
- Agents (weakest first): random, greedy, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2, engine:w.built=-100, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates random 0%, greedy 95%, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 93%, engine:w.built=-100 90%, engine 93%; skill gap +93 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0: survival (rounds; wins count extra) random 6.37, greedy 11.82, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 11.68, engine:w.built=-100 11.57, engine 11.70; **the ladder is not ordered on survival**.
- Difficulty 4: win rates random 0%, greedy 71%, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 60%, engine:w.built=-100 60%, engine 68%; skill gap +68 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4: survival (rounds; wins count extra) random 5.51, greedy 9.69, greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 9.10, engine:w.built=-100 9.26, engine 9.65; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: +93 → +68 pts win rate, +5.33 → +4.14 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.1 at 4 (target 8–10).
- Games ended by: corruption 848, deck_empty 28, opposition 1510, population 14.
- Adversary operations: 12.0 per round on average across cells, at most 60 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 22% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 21% of threats countered.
- Strategy health at difficulty 0: the top agent there (greedy, 95% wins) plays 1.89 buildings and 2.14 tier-2 cards per game (of 20.1 cards); 89% of its wins had 2+ buildings played (89% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4: the top agent there (greedy, 71% wins) plays 1.82 buildings and 1.75 tier-2 cards per game (of 17.6 cards); 85% of its wins had 2+ buildings played (85% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4: engine 68% vs greedy 71% over 240 seeds (only engine 25, only greedy 31), -2.5 ±3.1 pts, sign test p 0.504: no better than greedy (behind, not significant).

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | random | greedy | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | engine:w.built=-100 | engine | skill gap |
|---|---|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.37) | 94.6 ±1.5 (11.82) | 92.9 ±1.7 (11.68) | 89.6 ±2.0 (11.57) | 92.9 ±1.7 (11.70) | +92.9 pts (+5.33 survival) |

| difficulty 4 | random | greedy | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | engine:w.built=-100 | engine | skill gap |
|---|---|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.51) | 70.8 ±2.9 (9.69) | 60.4 ±3.2 (9.10) | 59.6 ±3.2 (9.26) | 68.3 ±3.0 (9.65) | +68.3 pts (+4.14 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| d0 random | 240 | 0.0 | 94.6 | 0 / 0 / 227 / 13 | -94.6 ±1.5 | <0.001 | worse than greedy |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 240 | 92.9 | 94.6 | 213 / 10 / 14 / 3 | -1.7 ±2.0 | 0.541 | no better than greedy (behind, not significant) |
| d0 engine:w.built=-100 | 240 | 89.6 | 94.6 | 206 / 9 / 21 / 4 | -5.0 ±2.3 | 0.043 | worse than greedy |
| d0 engine | 240 | 92.9 | 94.6 | 213 / 10 / 14 / 3 | -1.7 ±2.0 | 0.541 | no better than greedy (behind, not significant) |
| d4 random | 240 | 0.0 | 70.8 | 0 / 0 / 170 / 70 | -70.8 ±2.9 | <0.001 | worse than greedy |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 240 | 60.4 | 70.8 | 125 / 20 / 45 / 50 | -10.4 ±3.3 | 0.003 | worse than greedy |
| d4 engine:w.built=-100 | 240 | 59.6 | 70.8 | 121 / 22 / 49 / 48 | -11.2 ±3.4 | 0.002 | worse than greedy |
| d4 engine | 240 | 68.3 | 70.8 | 139 / 25 / 31 / 45 | -2.5 ±3.1 | 0.504 | no better than greedy (behind, not significant) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 random | 6.37 (0.96, 4..9) | corruption 229, population 11 | – / – | 59.3 / 0.00 | 0.19 | 2.12 |
| d0 greedy | 7.75 (1.03, 2..10) | corruption 7, deck_empty 6, opposition 227 | 2.30 / 16.28 | 12.5 / 1.85 | 15.58 | 5.26 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 7.88 (1.03, 5..10) | corruption 13, deck_empty 3, opposition 223, population 1 | 2.10 / 16.74 | 23.5 / 1.47 | 15.83 | 5.19 |
| d0 engine:w.built=-100 | 7.85 (1.97, 1..10) | corruption 22, deck_empty 2, opposition 215, population 1 | 2.15 / 16.21 | 18.0 / 1.68 | 14.66 | 5.16 |
| d0 engine | 8.12 (1.48, 2..10) | corruption 16, deck_empty 1, opposition 223 | 1.85 / 16.52 | 22.2 / 1.53 | 15.41 | 5.38 |
| d4 random | 5.51 (1.30, 3..9) | corruption 239, deck_empty 1 | – / – | 45.5 / 0.00 | 0.00 | 3.81 |
| d4 greedy | 7.10 (1.10, 2..9) | corruption 63, deck_empty 7, opposition 170 | 1.65 / 7.85 | 15.6 / 1.33 | 5.72 | 5.38 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 6.81 (1.22, 3..9) | corruption 93, deck_empty 2, opposition 145 | 1.79 / 8.95 | 21.3 / 1.08 | 5.49 | 5.35 |
| d4 engine:w.built=-100 | 7.03 (1.84, 1..9) | corruption 94, deck_empty 2, opposition 143, population 1 | 1.74 / 9.25 | 20.4 / 1.22 | 5.62 | 5.40 |
| d4 engine | 7.13 (1.52, 2..9) | corruption 72, deck_empty 4, opposition 164 | 1.68 / 9.27 | 19.7 / 1.25 | 6.42 | 5.44 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 random | 198.1 | 6.96 | 5% | – | – | 15.89 (47) | 4.53 | 7.70 | 1.73 | 2.75 / 12.74 (22%) |
| d0 greedy | 216.2 | 10.04 | 4% | – | – | 10.02 (42) | 0.00 | 9.15 | 0.00 | 5.63 / 15.51 (36%) |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 268.9 | 8.31 | 4% | – | – | 11.52 (47) | 0.40 | 9.42 | 0.00 | 6.24 / 15.75 (40%) |
| d0 engine:w.built=-100 | 259.2 | 10.22 | 4% | – | – | 11.63 (60) | 0.00 | 9.35 | 0.00 | 8.01 / 15.70 (51%) |
| d0 engine | 275.3 | 9.81 | 4% | – | – | 11.26 (47) | 0.00 | 10.76 | 0.00 | 9.21 / 16.25 (57%) |
| d4 random | 165.7 | 6.73 | 4% | – | – | 13.17 (50) | 4.01 | 5.33 | 1.80 | 0.62 / 11.02 (6%) |
| d4 greedy | 198.8 | 9.37 | 4% | – | – | 10.75 (42) | 0.00 | 4.10 | 0.00 | 0.75 / 14.20 (5%) |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 231.5 | 7.99 | 4% | – | – | 11.90 (44) | 0.33 | 3.69 | 0.00 | 0.40 / 13.62 (3%) |
| d4 engine:w.built=-100 | 226.1 | 9.63 | 4% | – | – | 11.92 (42) | 0.00 | 5.45 | 0.00 | 1.48 / 14.07 (11%) |
| d4 engine | 234.3 | 9.49 | 4% | – | – | 11.47 (55) | 0.00 | 6.10 | 0.00 | 1.83 / 14.27 (13%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 10% / 71% / 19% | 8% / 79% / 13% |
| d4 | 33% / 28% / 39% | 37% / 37% / 26% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 random | 240 of 240 | 0.74 | 0 of 0 | – |
| d0 greedy | 3 of 240 | 0.35 | 3 of 227 | 0.35 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 239 of 240 | 0.66 | 222 of 223 | 0.66 |
| d0 engine:w.built=-100 | 1 of 240 | 0.00 | 1 of 215 | 0.00 |
| d0 engine | 69 of 240 | 0.61 | 65 of 223 | 0.60 |
| d4 random | 240 of 240 | 0.77 | 0 of 0 | – |
| d4 greedy | 3 of 240 | 0.34 | 3 of 170 | 0.34 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 240 of 240 | 0.66 | 145 of 145 | 0.65 |
| d4 engine:w.built=-100 | 1 of 240 | 0.00 | 1 of 143 | 0.00 |
| d4 engine | 30 of 240 | 0.54 | 27 of 164 | 0.52 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| random | 8.32 | 0.01 | 19.9 | – |
| greedy | 1.89 | 2.14 | 20.1 | 89% (89%) |
| greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 11.01 | 3.26 | 36.8 | 100% (100%) |
| engine:w.built=-100 | 0.00 | 2.14 | 31.6 | 0% (0%) |
| engine | 3.58 | 2.75 | 35.5 | 96% (96%) |

| difficulty 4 | buildings played/game | tier 2 played/game | cards played/game | wins with 2+ buildings played (standing) |
|---|---|---|---|---|
| random | 7.00 | 0.02 | 16.7 | – |
| greedy | 1.82 | 1.75 | 17.6 | 85% (85%) |
| greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 10.53 | 2.34 | 30.7 | 100% (100%) |
| engine:w.built=-100 | 0.00 | 1.52 | 24.8 | 0% (0%) |
| engine | 2.43 | 2.10 | 27.5 | 85% (85%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2: pick (offered) / keep / play | engine: pick (offered) / keep / play | engine:w.built=-100: pick (offered) / keep / play |
|---|---|---|---|---|---|
| antimatter_refinery | 2.5 (192) / 51% / 2% | – (0) / – / 0% | – (0) / – / 10% | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 2.4 (631) / 54% / 46% | 1.1 (566) / 100% / 0% | 1.1 (530) / 100% / 46% | 1.1 (538) / 99% / 2% | 1.1 (513) / 100% / 0% |
| civic_shelter | 2.4 (608) / 50% / 49% | 2.0 (478) / 100% / 0% | 2.0 (479) / 100% / 42% | 1.9 (484) / 97% / 0% | 1.9 (463) / 99% / 0% |
| contract_bid | 2.5 (817) / 52% / 79% | 4.0 (478) / 99% / 0% | 2.8 (2335) / 98% / 87% | 3.3 (2350) / 78% / 90% | 3.2 (2290) / 81% / 90% |
| counter_intelligence | 2.6 (172) / 55% / 0% | – (0) / – / 0% | – (0) / – / 0% | – (0) / – / 29% | – (0) / – / 17% |
| data_mining | 2.5 (774) / 51% / 60% | 3.6 (480) / 100% / 0% | 2.8 (1218) / 99% / 64% | 3.0 (1599) / 83% / 71% | 3.0 (1513) / 86% / 71% |
| emergency_fab | 2.6 (631) / 52% / 13% | 2.5 (478) / 100% / 0% | 2.5 (475) / 100% / 0% | 2.5 (497) / 96% / 1% | 2.5 (477) / 97% / 0% |
| evacuate | 2.5 (731) / 48% / 52% | 3.0 (477) / 100% / 0% | 2.9 (549) / 98% / 15% | 2.9 (621) / 89% / 22% | 2.9 (581) / 92% / 21% |
| fab_line | 2.5 (636) / 52% / 49% | 1.9 (474) / 100% / 0% | 1.9 (476) / 99% / 55% | 1.8 (468) / 99% / 30% | 1.9 (458) / 99% / 0% |
| firewall_node | 2.5 (626) / 52% / 52% | 1.8 (474) / 100% / 0% | 1.8 (475) / 100% / 54% | 1.8 (468) / 99% / 0% | 1.8 (458) / 99% / 0% |
| fork_process | 2.4 (744) / 51% / 56% | 2.2 (940) / 100% / 52% | 2.1 (985) / 100% / 55% | 2.2 (1065) / 94% / 56% | 2.2 (1048) / 96% / 58% |
| fusion_plant | 2.5 (195) / 53% / 0% | – (0) / – / 0% | – (0) / – / 13% | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | 2.5 (183) / 53% / 1% | – (0) / – / 0% | – (0) / – / 22% | – (0) / – / 0% | – (0) / – / 0% |
| grant | 2.5 (735) / 49% / 60% | 3.7 (478) / 100% / 0% | 3.0 (1053) / 99% / 56% | 3.1 (1642) / 80% / 72% | 3.1 (1514) / 84% / 73% |
| grid_battery | 2.5 (651) / 50% / 49% | 2.0 (479) / 100% / 0% | 2.0 (477) / 100% / 46% | 1.9 (480) / 98% / 7% | 2.0 (463) / 100% / 0% |
| jam_signals | 2.4 (793) / 52% / 74% | 3.2 (472) / 100% / 0% | 2.7 (810) / 99% / 42% | 2.7 (1192) / 88% / 63% | 2.8 (1086) / 91% / 61% |
| liquidate_reserves | 2.5 (739) / 51% / 57% | 3.6 (479) / 99% / 0% | 3.6 (490) / 98% / 0% | 3.5 (722) / 76% / 18% | 3.5 (673) / 80% / 19% |
| logistics_ai | 2.6 (645) / 50% / 46% | 1.2 (507) / 100% / 0% | 1.2 (500) / 100% / 52% | 1.3 (491) / 99% / 0% | 1.2 (477) / 100% / 0% |
| megastructure | 2.4 (191) / 53% / 0% | – (0) / – / 0% | – (0) / – / 7% | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.4 (640) / 49% / 53% | 2.3 (472) / 100% / 0% | 2.3 (481) / 99% / 33% | 2.3 (472) / 99% / 35% | 2.3 (455) / 100% / 0% |
| modular_expansion | – (0) / – / 0.84/game | – (0) / – / 0.00/game | – (0) / – / 0.99/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| opposition_network | 2.5 (631) / 50% / 52% | 1.5 (480) / 100% / 0% | 1.5 (482) / 99% / 47% | 1.5 (475) / 99% / 0% | 1.5 (465) / 100% / 0% |
| orbital_lance | 2.5 (192) / 54% / 1% | – (0) / – / 0% | – (0) / – / 8% | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 2.5 (630) / 57% / 42% | 1.0 (580) / 100% / 0% | 1.0 (531) / 100% / 51% | 1.0 (525) / 100% / 0% | 1.0 (523) / 100% / 0% |
| point_defense | – (0) / – / 0.67/game | – (0) / – / 0.00/game | – (0) / – / 0.75/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| power_share | 2.6 (754) / 50% / 61% | 3.6 (480) / 99% / 0% | 3.5 (495) / 98% / 0% | 3.5 (631) / 74% / 0% | 3.5 (581) / 78% / 0% |
| precision_strike | 2.5 (741) / 48% / 56% | 1.7 (2166) / 100% / 93% | 1.7 (2027) / 99% / 87% | 1.8 (2249) / 93% / 93% | 1.9 (2230) / 94% / 94% |
| purge_protocol | 2.5 (191) / 46% / 1% | – (0) / – / 94% | – (0) / – / 85% | – (0) / – / 95% | – (0) / – / 92% |
| quantum_core | 2.5 (212) / 45% / 0% | – (0) / – / 0% | – (0) / – / 12% | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.5 (729) / 49% / 58% | 2.2 (1634) / 100% / 88% | 2.2 (1414) / 99% / 81% | 2.3 (1758) / 91% / 90% | 2.3 (1761) / 93% / 92% |
| rebuild | 2.6 (661) / 47% / 36% | 2.1 (609) / 100% / 30% | 1.9 (721) / 99% / 42% | 1.8 (1018) / 98% / 67% | 1.7 (1021) / 97% / 70% |
| recon_sweep | 2.5 (802) / 52% / 73% | 3.6 (470) / 100% / 0% | 3.5 (491) / 99% / 0% | 3.5 (597) / 78% / 0% | 3.5 (551) / 82% / 0% |
| reconstruction | 2.3 (190) / 51% / 0% | – (0) / – / 0% | – (0) / – / 33% | – (0) / – / 1% | – (0) / – / 0% |
| recruitment_hub | 2.6 (654) / 49% / 48% | 1.5 (478) / 100% / 0% | 1.5 (478) / 99% / 46% | 1.5 (472) / 99% / 0% | 1.5 (450) / 100% / 0% |
| scavenge | 2.5 (730) / 53% / 58% | 3.6 (474) / 100% / 0% | 3.6 (495) / 98% / 1% | 3.4 (644) / 77% / 10% | 3.4 (607) / 79% / 8% |
| server_farm | 2.5 (641) / 52% / 45% | 1.5 (478) / 100% / 86% | 1.5 (481) / 100% / 71% | 1.5 (468) / 99% / 62% | 1.5 (452) / 100% / 0% |
| shelter_order | – (0) / – / 0.97/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shore_up | 2.5 (758) / 48% / 54% | 2.9 (617) / 100% / 26% | 2.8 (617) / 98% / 25% | 2.8 (778) / 90% / 38% | 2.7 (764) / 92% / 40% |
| solar_field | 2.5 (631) / 50% / 78% | 2.7 (477) / 100% / 0% | 2.5 (565) / 99% / 70% | 2.7 (490) / 95% / 68% | 2.7 (468) / 97% / 0% |
| strike_team | – (0) / – / 0.73/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | 2.5 (189) / 51% / 1% | – (0) / – / 0% | – (0) / – / 47% | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 2.5 (757) / 47% / 52% | 1.5 (1535) / 100% / 81% | 1.5 (1377) / 100% / 74% | 1.4 (2000) / 96% / 88% | 1.4 (1988) / 97% / 90% |
| trade_office | 2.5 (643) / 51% / 47% | 1.2 (520) / 100% / 0% | 1.2 (500) / 100% / 52% | 1.2 (495) / 99% / 0% | 1.2 (485) / 99% / 0% |
| turnkey_factory | – (0) / – / 0.85/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 0.99/game | – (0) / – / 0.00/game |
| venture_capital | – (0) / – / 0.95/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 2.5 (626) / 54% / 53% | 1.8 (476) / 100% / 0% | 1.9 (476) / 100% / 60% | 1.8 (474) / 97% / 0% | 1.8 (459) / 99% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 random | 0.0 | 0.000 | 0.0 | 19.7 |
| d0 greedy | 5364.7 | 24.808 | 0.0 | 5391.8 |
| d0 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 8901.2 | 33.100 | 0.0 | 8934.9 |
| d0 engine:w.built=-100 | 14551.5 | 56.146 | 0.0 | 14603.7 |
| d0 engine | 10674.4 | 38.774 | 0.0 | 10712.4 |
| d4 random | 0.0 | 0.000 | 0.0 | 16.0 |
| d4 greedy | 4571.4 | 23.000 | 0.0 | 4595.5 |
| d4 greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2 | 7047.5 | 30.445 | 0.0 | 7075.4 |
| d4 engine:w.built=-100 | 7491.9 | 33.129 | 0.0 | 7520.5 |
| d4 engine | 7219.2 | 30.809 | 0.0 | 7247.6 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c8-baseline-base-d4-engine_w_built_-100-seed300207.json`](records/close-loss-c8-baseline-base-d4-engine_w_built_-100-seed300207.json): close loss at difficulty 4: corruption in round 8, 2 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c7-baseline-base-d4-greedy_w_built_cost_1_2_w_hand_1_w_hand_cost_0_2-seed300086.json`](records/close-loss-c7-baseline-base-d4-greedy_w_built_cost_1_2_w_hand_1_w_hand_cost_0_2-seed300086.json): close loss at difficulty 4: corruption in round 7, 3 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c6-baseline-base-d4-greedy-seed300000.json`](records/surprising-win-c6-baseline-base-d4-greedy-seed300000.json): won at difficulty 4 by greedy, below the top agent
- [`records/surprising-win-c7-baseline-base-d4-greedy_w_built_cost_1_2_w_hand_1_w_hand_cost_0_2-seed300000.json`](records/surprising-win-c7-baseline-base-d4-greedy_w_built_cost_1_2_w_hand_1_w_hand_cost_0_2-seed300000.json): won at difficulty 4 by greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2, below the top agent

