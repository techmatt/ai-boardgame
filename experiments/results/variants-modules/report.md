# Cycle report: variants-modules

007: opposition rewards, threat placement and the off-variants at the tuned baseline.

3072 games (32 cells × 96 seeds, 200000..=200095) on 12 threads in 654.0 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: pushback; income (opposition.variant=income opposition.income_credits=2); free_purchases (opposition.variant=free_purchases opposition.free_purchases=2); counter_threat (opposition.variant=counter_threat); opposition_off (opposition.variant=off); shared_row (threats.variant=shared_row); threats_off (threats.variant=off); cards_off (cards.variant=off)
- Difficulties: 0, 4
- Agents (weakest first): random, greedy
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, pushback): win rates random 1%, greedy 94%; skill gap +93 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, pushback): survival (rounds; wins count extra) random 6.47, greedy 11.78; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, pushback): win rates random 0%, greedy 66%; skill gap +66 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, pushback): survival (rounds; wins count extra) random 5.59, greedy 9.43; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, income): win rates random 1%, greedy 93%; skill gap +92 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, income): survival (rounds; wins count extra) random 6.47, greedy 11.73; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, income): win rates random 0%, greedy 64%; skill gap +64 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, income): survival (rounds; wins count extra) random 5.59, greedy 9.39; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, free_purchases): win rates random 1%, greedy 92%; skill gap +91 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, free_purchases): survival (rounds; wins count extra) random 6.47, greedy 11.69; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, free_purchases): win rates random 0%, greedy 64%; skill gap +64 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, free_purchases): survival (rounds; wins count extra) random 5.59, greedy 9.39; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, counter_threat): win rates random 1%, greedy 93%; skill gap +92 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, counter_threat): survival (rounds; wins count extra) random 6.47, greedy 11.73; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, counter_threat): win rates random 0%, greedy 69%; skill gap +69 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, counter_threat): survival (rounds; wins count extra) random 5.59, greedy 9.66; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, opposition_off): win rates random 1%, greedy 92%; skill gap +91 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, opposition_off): survival (rounds; wins count extra) random 6.47, greedy 11.68; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, opposition_off): win rates random 0%, greedy 64%; skill gap +64 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, opposition_off): survival (rounds; wins count extra) random 5.59, greedy 9.38; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, shared_row): win rates random 0%, greedy 75%; skill gap +75 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, shared_row): survival (rounds; wins count extra) random 6.04, greedy 10.73; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, shared_row): win rates random 0%, greedy 22%; skill gap +22 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, shared_row): survival (rounds; wins count extra) random 5.09, greedy 6.94; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, threats_off): win rates random 1%, greedy 99%; skill gap +98 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, threats_off): survival (rounds; wins count extra) random 7.26, greedy 11.98; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, threats_off): win rates random 1%, greedy 98%; skill gap +97 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, threats_off): survival (rounds; wins count extra) random 6.52, greedy 10.96; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, cards_off): win rates random 0%, greedy 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, cards_off): survival (rounds; wins count extra) random 6.27, greedy 6.96; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, cards_off): win rates random 0%, greedy 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, cards_off): survival (rounds; wins count extra) random 5.49, greedy 6.03; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +79 → +55 pts win rate, +4.54 → +3.26 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.7 rounds at difficulty 0, 6.9 at 4 (target 8–10).
- Games ended by: corruption 1941, deck_empty 29, opposition 1045, population 57.
- Adversary operations: 12.6 per round on average across cells, at most 58 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 31% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 22% of threats countered.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| pushback difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 1.0 ±1.0 (6.47) | 93.8 ±2.5 (11.78) | +92.7 pts (+5.31 survival) |

| pushback difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.59) | 65.6 ±4.8 (9.43) | +65.6 pts (+3.83 survival) |

| income difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 1.0 ±1.0 (6.47) | 92.7 ±2.7 (11.73) | +91.7 pts (+5.26 survival) |

| income difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.59) | 63.5 ±4.9 (9.39) | +63.5 pts (+3.79 survival) |

| free_purchases difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 1.0 ±1.0 (6.47) | 91.7 ±2.8 (11.69) | +90.6 pts (+5.22 survival) |

| free_purchases difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.59) | 63.5 ±4.9 (9.39) | +63.5 pts (+3.79 survival) |

| counter_threat difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 1.0 ±1.0 (6.47) | 92.7 ±2.7 (11.73) | +91.7 pts (+5.26 survival) |

| counter_threat difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.59) | 68.8 ±4.7 (9.66) | +68.8 pts (+4.06 survival) |

| opposition_off difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 1.0 ±1.0 (6.47) | 91.7 ±2.8 (11.68) | +90.6 pts (+5.21 survival) |

| opposition_off difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.59) | 63.5 ±4.9 (9.38) | +63.5 pts (+3.78 survival) |

| shared_row difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.04) | 75.0 ±4.4 (10.73) | +75.0 pts (+4.69 survival) |

| shared_row difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.09) | 21.9 ±4.2 (6.94) | +21.9 pts (+1.84 survival) |

| threats_off difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 1.0 ±1.0 (7.26) | 99.0 ±1.0 (11.98) | +97.9 pts (+4.72 survival) |

| threats_off difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 1.0 ±1.0 (6.52) | 97.9 ±1.5 (10.96) | +96.9 pts (+4.44 survival) |

| cards_off difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.27) | 0.0 ±0.0 (6.96) | +0.0 pts (+0.69 survival) |

| cards_off difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.49) | 0.0 ±0.0 (6.03) | +0.0 pts (+0.54 survival) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| pushback d0 random | 6.36 (1.13, 2..10) | corruption 92, opposition 1, population 3 | 8.00 / 22.00 | 59.5 / 0.01 | 0.30 | 2.07 |
| pushback d0 greedy | 7.74 (1.15, 3..10) | corruption 2, deck_empty 3, opposition 90, population 1 | 2.31 / 16.14 | 8.0 / 2.00 | 15.45 | 5.24 |
| pushback d4 random | 5.59 (1.39, 4..9) | corruption 96 | – / – | 43.9 / 0.00 | 0.00 | 3.93 |
| pushback d4 greedy | 6.94 (1.18, 3..9) | corruption 31, deck_empty 2, opposition 63 | 1.79 / 8.57 | 16.9 / 1.21 | 5.79 | 5.28 |
| income d0 random | 6.36 (1.13, 2..10) | corruption 92, opposition 1, population 3 | 8.00 / 22.00 | 59.5 / 0.01 | 0.30 | 2.07 |
| income d0 greedy | 7.74 (1.15, 3..10) | corruption 2, deck_empty 3, opposition 89, population 2 | 2.30 / 15.87 | 7.6 / 2.00 | 15.06 | 5.15 |
| income d4 random | 5.59 (1.39, 4..9) | corruption 96 | – / – | 43.9 / 0.00 | 0.00 | 3.93 |
| income d4 greedy | 6.95 (1.18, 3..9) | corruption 33, deck_empty 2, opposition 61 | 1.84 / 8.48 | 15.3 / 1.31 | 5.56 | 5.22 |
| free_purchases d0 random | 6.36 (1.13, 2..10) | corruption 92, opposition 1, population 3 | 8.00 / 22.00 | 59.5 / 0.01 | 0.30 | 2.07 |
| free_purchases d0 greedy | 7.74 (1.15, 3..10) | corruption 5, deck_empty 3, opposition 88 | 2.31 / 15.90 | 8.0 / 2.00 | 14.83 | 5.16 |
| free_purchases d4 random | 5.59 (1.39, 4..9) | corruption 96 | – / – | 43.9 / 0.00 | 0.00 | 3.93 |
| free_purchases d4 greedy | 6.98 (1.21, 3..9) | corruption 33, deck_empty 2, opposition 61 | 1.79 / 8.26 | 15.9 / 1.29 | 5.43 | 5.27 |
| counter_threat d0 random | 6.36 (1.13, 2..10) | corruption 92, opposition 1, population 3 | 8.00 / 22.00 | 59.5 / 0.01 | 0.30 | 2.07 |
| counter_threat d0 greedy | 7.68 (1.16, 3..10) | corruption 3, deck_empty 3, opposition 89, population 1 | 2.37 / 17.25 | 6.9 / 2.00 | 16.34 | 5.21 |
| counter_threat d4 random | 5.59 (1.39, 4..9) | corruption 96 | – / – | 43.9 / 0.00 | 0.00 | 3.93 |
| counter_threat d4 greedy | 7.06 (1.13, 3..9) | corruption 28, deck_empty 2, opposition 66 | 1.77 / 8.83 | 15.0 / 1.33 | 6.27 | 5.26 |
| opposition_off d0 random | 6.36 (1.13, 2..10) | corruption 92, opposition 1, population 3 | 8.00 / 22.00 | 59.5 / 0.01 | 0.30 | 2.07 |
| opposition_off d0 greedy | 7.74 (1.17, 3..10) | corruption 5, deck_empty 3, opposition 88 | 2.30 / 15.98 | 11.4 / 1.88 | 14.91 | 5.15 |
| opposition_off d4 random | 5.59 (1.39, 4..9) | corruption 96 | – / – | 43.9 / 0.00 | 0.00 | 3.93 |
| opposition_off d4 greedy | 6.97 (1.21, 3..9) | corruption 33, deck_empty 2, opposition 61 | 1.79 / 8.28 | 16.5 / 1.26 | 5.44 | 5.26 |
| shared_row d0 random | 6.04 (1.10, 3..9) | corruption 86, population 10 | – / – | 54.0 / 0.00 | 1.09 | 2.58 |
| shared_row d0 greedy | 7.56 (0.90, 6..10) | corruption 23, opposition 72, population 1 | 2.22 / 12.56 | 15.6 / 1.58 | 9.50 | 4.79 |
| shared_row d4 random | 5.09 (1.33, 3..8) | corruption 96 | – / – | 39.9 / 0.00 | 0.00 | 4.85 |
| shared_row d4 greedy | 6.11 (1.31, 4..9) | corruption 74, deck_empty 1, opposition 21 | 1.76 / 9.00 | 20.1 / 0.89 | 2.04 | 5.39 |
| threats_off d0 random | 7.17 (1.57, 3..10) | corruption 78, opposition 1, population 17 | 7.00 / 22.00 | 66.3 / 0.02 | 1.23 | 0.53 |
| threats_off d0 greedy | 7.72 (0.77, 4..10) | deck_empty 1, opposition 95 | 2.31 / 20.17 | 2.0 / 2.00 | 20.17 | 5.75 |
| threats_off d4 random | 6.44 (1.78, 3..9) | corruption 92, opposition 1, population 3 | 6.00 / 14.00 | 56.3 / 0.01 | 0.23 | 1.83 |
| threats_off d4 greedy | 7.40 (0.90, 5..9) | deck_empty 2, opposition 94 | 1.64 / 11.57 | 5.0 / 2.00 | 11.58 | 5.65 |
| cards_off d0 random | 6.27 (0.97, 5..9) | corruption 92, population 4 | – / – | 63.2 / 0.00 | 0.12 | 1.74 |
| cards_off d0 greedy | 6.96 (1.01, 4..10) | corruption 93, population 3 | – / – | 70.4 / 0.00 | 0.28 | 2.67 |
| cards_off d4 random | 5.49 (1.32, 4..8) | corruption 96 | – / – | 47.4 / 0.00 | 0.00 | 3.88 |
| cards_off d4 greedy | 6.03 (1.25, 4..9) | corruption 96 | – / – | 53.6 / 0.00 | 0.00 | 4.54 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| pushback d0 random | 197.6 | 6.87 | 5% | – | – | 15.91 (47) | 5.02 | 7.97 | 1.84 | 2.67 / 12.73 (21%) |
| pushback d0 greedy | 216.9 | 10.11 | 4% | – | – | 10.17 (42) | 0.00 | 9.21 | 0.00 | 5.71 / 15.48 (37%) |
| pushback d4 random | 171.9 | 6.84 | 5% | – | – | 12.65 (39) | 4.15 | 5.62 | 1.77 | 0.72 / 11.19 (6%) |
| pushback d4 greedy | 195.3 | 9.35 | 4% | – | – | 10.53 (38) | 0.00 | 3.96 | 0.00 | 0.83 / 13.88 (6%) |
| income d0 random | 197.6 | 6.87 | 5% | – | – | 15.91 (47) | 5.02 | 7.97 | 1.84 | 2.67 / 12.73 (21%) |
| income d0 greedy | 216.0 | 10.21 | 4% | – | – | 10.47 (38) | 0.00 | 9.21 | 0.00 | 5.70 / 15.48 (37%) |
| income d4 random | 171.9 | 6.84 | 5% | – | – | 12.65 (39) | 4.15 | 5.62 | 1.77 | 0.72 / 11.19 (6%) |
| income d4 greedy | 194.2 | 9.41 | 4% | – | – | 10.69 (39) | 0.00 | 3.97 | 0.00 | 0.83 / 13.90 (6%) |
| free_purchases d0 random | 197.6 | 6.87 | 5% | – | – | 15.91 (47) | 5.02 | 7.97 | 1.84 | 2.67 / 12.73 (21%) |
| free_purchases d0 greedy | 215.4 | 10.14 | 4% | – | – | 10.51 (39) | 0.00 | 9.21 | 0.00 | 5.70 / 15.48 (37%) |
| free_purchases d4 random | 171.9 | 6.84 | 5% | – | – | 12.65 (39) | 4.15 | 5.62 | 1.77 | 0.72 / 11.19 (6%) |
| free_purchases d4 greedy | 194.7 | 9.39 | 4% | – | – | 10.77 (39) | 0.00 | 3.98 | 0.00 | 0.83 / 13.96 (6%) |
| counter_threat d0 random | 197.6 | 6.87 | 5% | – | – | 15.90 (47) | 5.02 | 7.97 | 1.84 | 2.68 / 12.73 (21%) |
| counter_threat d0 greedy | 214.3 | 10.01 | 4% | – | – | 10.10 (36) | 0.00 | 8.54 | 0.00 | 7.25 / 15.35 (47%) |
| counter_threat d4 random | 171.9 | 6.84 | 5% | – | – | 12.65 (39) | 4.15 | 5.62 | 1.77 | 0.72 / 11.19 (6%) |
| counter_threat d4 greedy | 197.0 | 9.30 | 4% | – | – | 10.56 (38) | 0.00 | 3.96 | 0.00 | 2.50 / 14.12 (18%) |
| opposition_off d0 random | 197.6 | 6.87 | 5% | – | – | 15.91 (47) | 5.02 | 7.97 | 1.84 | 2.67 / 12.73 (21%) |
| opposition_off d0 greedy | 215.4 | 10.13 | 4% | – | – | 10.52 (36) | 0.00 | 9.23 | 0.00 | 5.70 / 15.48 (37%) |
| opposition_off d4 random | 171.9 | 6.84 | 5% | – | – | 12.65 (39) | 4.15 | 5.62 | 1.77 | 0.72 / 11.19 (6%) |
| opposition_off d4 greedy | 194.1 | 9.35 | 4% | – | – | 10.78 (39) | 0.00 | 3.97 | 0.00 | 0.83 / 13.94 (6%) |
| shared_row d0 random | 193.1 | 7.22 | 4% | – | – | 15.44 (44) | 4.76 | 11.38 | 2.34 | 4.34 / 12.08 (36%) |
| shared_row d0 greedy | 220.4 | 10.49 | 4% | – | – | 11.38 (58) | 0.00 | 12.49 | 0.00 | 7.47 / 15.12 (49%) |
| shared_row d4 random | 159.6 | 6.84 | 4% | – | – | 12.54 (42) | 3.89 | 8.99 | 3.17 | 1.03 / 10.19 (10%) |
| shared_row d4 greedy | 181.4 | 9.06 | 4% | – | – | 11.54 (38) | 0.00 | 5.98 | 0.00 | 1.31 / 12.23 (11%) |
| threats_off d0 random | 212.3 | 6.65 | 5% | – | – | 15.83 (49) | 5.66 | 0.00 | 0.00 | 0.00 / 0.00 (0%) |
| threats_off d0 greedy | 201.0 | 9.49 | 4% | – | – | 7.61 (28) | 0.00 | 0.00 | 0.00 | 0.00 / 0.00 (0%) |
| threats_off d4 random | 188.4 | 6.67 | 5% | – | – | 13.89 (48) | 4.65 | 0.00 | 0.00 | 0.00 / 0.00 (0%) |
| threats_off d4 greedy | 193.5 | 9.29 | 4% | – | – | 8.11 (37) | 0.00 | 0.00 | 0.00 | 0.00 / 0.00 (0%) |
| cards_off d0 random | 85.6 | 6.46 | 2% | – | – | 16.10 (46) | 7.85 | 8.47 | 1.93 | 2.84 / 12.54 (23%) |
| cards_off d0 greedy | 82.8 | 6.42 | 0% | – | – | 16.04 (57) | 0.00 | 9.84 | 0.00 | 6.40 / 13.92 (46%) |
| cards_off d4 random | 69.7 | 6.20 | 2% | – | – | 12.88 (45) | 6.92 | 5.39 | 1.84 | 0.67 / 10.98 (6%) |
| cards_off d4 greedy | 66.3 | 6.07 | 0% | – | – | 13.31 (54) | 0.00 | 4.04 | 0.00 | 0.85 / 12.06 (7%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| pushback d0 | 7% / 86% / 7% | 8% / 86% / 6% |
| pushback d4 | 26% / 49% / 26% | 35% / 51% / 15% |
| income d0 | 8% / 84% / 8% | 9% / 85% / 6% |
| income d4 | 27% / 47% / 27% | 35% / 50% / 15% |
| free_purchases d0 | 9% / 83% / 9% | 10% / 84% / 6% |
| free_purchases d4 | 27% / 47% / 27% | 35% / 50% / 15% |
| counter_threat d0 | 8% / 84% / 8% | 9% / 85% / 7% |
| counter_threat d4 | 24% / 52% / 24% | 30% / 56% / 13% |
| opposition_off d0 | 9% / 83% / 9% | 10% / 83% / 7% |
| opposition_off d4 | 27% / 47% / 27% | 35% / 50% / 15% |
| shared_row d0 | 20% / 60% / 20% | 24% / 64% / 12% |
| shared_row d4 | 44% / 12% / 44% | 65% / 18% / 17% |
| threats_off d0 | 2% / 96% / 2% | 9% / 81% / 9% |
| threats_off d4 | 3% / 94% / 3% | 13% / 75% / 13% |
| cards_off d0 | – (no variance) | 81% / 11% / 8% |
| cards_off d4 | – (no variance) | 91% / 4% / 5% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| pushback d0 random | 96 of 96 | 0.76 | 1 of 1 | – |
| pushback d0 greedy | 3 of 96 | 0.37 | 3 of 90 | 0.37 |
| pushback d4 random | 96 of 96 | 0.77 | 0 of 0 | – |
| pushback d4 greedy | 3 of 96 | 0.37 | 3 of 63 | 0.34 |
| income d0 random | 96 of 96 | 0.77 | 1 of 1 | – |
| income d0 greedy | 3 of 96 | 0.37 | 3 of 89 | 0.37 |
| income d4 random | 96 of 96 | 0.77 | 0 of 0 | – |
| income d4 greedy | 3 of 96 | 0.37 | 3 of 61 | 0.34 |
| free_purchases d0 random | 96 of 96 | 0.77 | 1 of 1 | – |
| free_purchases d0 greedy | 3 of 96 | 0.37 | 3 of 88 | 0.37 |
| free_purchases d4 random | 96 of 96 | 0.77 | 0 of 0 | – |
| free_purchases d4 greedy | 3 of 96 | 0.37 | 3 of 61 | 0.34 |
| counter_threat d0 random | 96 of 96 | 0.76 | 1 of 1 | – |
| counter_threat d0 greedy | 3 of 96 | 0.37 | 3 of 89 | 0.38 |
| counter_threat d4 random | 96 of 96 | 0.77 | 0 of 0 | – |
| counter_threat d4 greedy | 3 of 96 | 0.37 | 3 of 66 | 0.34 |
| opposition_off d0 random | 96 of 96 | 0.77 | 1 of 1 | – |
| opposition_off d0 greedy | 3 of 96 | 0.38 | 3 of 88 | 0.38 |
| opposition_off d4 random | 96 of 96 | 0.77 | 0 of 0 | – |
| opposition_off d4 greedy | 3 of 96 | 0.37 | 3 of 61 | 0.33 |
| shared_row d0 random | 96 of 96 | 0.76 | 0 of 0 | – |
| shared_row d0 greedy | 3 of 96 | 0.36 | 3 of 72 | 0.36 |
| shared_row d4 random | 96 of 96 | 0.78 | 0 of 0 | – |
| shared_row d4 greedy | 3 of 96 | 0.37 | 3 of 21 | 0.31 |
| threats_off d0 random | 96 of 96 | 0.75 | 1 of 1 | – |
| threats_off d0 greedy | 3 of 96 | 0.34 | 3 of 95 | 0.34 |
| threats_off d4 random | 96 of 96 | 0.77 | 1 of 1 | – |
| threats_off d4 greedy | 3 of 96 | 0.35 | 3 of 94 | 0.36 |
| cards_off d0 random | 4 of 96 | 0.10 | 0 of 0 | – |
| cards_off d0 greedy | 1 of 96 | 0.00 | 0 of 0 | – |
| cards_off d4 random | 7 of 96 | 0.27 | 0 of 0 | – |
| cards_off d4 greedy | 1 of 96 | 0.00 | 0 of 0 | – |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | 2.6 (662) / 37% / 8% | – (0) / – / 0% |
| assembly_cluster | 2.6 (1796) / 53% / 42% | 1.1 (1533) / 100% / 0% |
| civic_shelter | 2.5 (1785) / 46% / 49% | 2.1 (1318) / 100% / 0% |
| contract_bid | 2.5 (2341) / 47% / 79% | 3.9 (1344) / 100% / 0% |
| counter_intelligence | 2.5 (626) / 59% / 0% | – (0) / – / 0% |
| data_mining | 2.5 (2120) / 54% / 62% | 3.7 (1314) / 100% / 0% |
| emergency_fab | 2.6 (1800) / 48% / 16% | 2.4 (1348) / 99% / 0% |
| evacuate | 2.5 (2095) / 51% / 59% | 3.0 (1350) / 100% / 2% |
| fab_line | 2.5 (1706) / 55% / 54% | 1.8 (1310) / 100% / 0% |
| firewall_node | 2.6 (1730) / 52% / 54% | 1.9 (1322) / 100% / 0% |
| fork_process | 2.5 (2078) / 49% / 60% | 2.1 (2720) / 100% / 54% |
| fusion_plant | 2.5 (615) / 47% / 0% | – (0) / – / 0% |
| global_mandate | 2.3 (643) / 54% / 0% | – (0) / – / 0% |
| grant | 2.5 (2060) / 48% / 63% | 3.5 (1325) / 99% / 0% |
| grid_battery | 2.4 (1732) / 52% / 54% | 2.1 (1314) / 100% / 0% |
| jam_signals | 2.4 (1974) / 53% / 61% | 3.2 (1337) / 100% / 0% |
| liquidate_reserves | 2.5 (1999) / 49% / 58% | 3.6 (1330) / 100% / 0% |
| logistics_ai | 2.5 (1806) / 46% / 40% | 1.3 (1424) / 100% / 0% |
| megastructure | 2.4 (676) / 46% / 0% | – (0) / – / 0% |
| microgrid | 2.5 (1736) / 49% / 50% | 2.4 (1340) / 100% / 0% |
| modular_expansion | – (0) / – / 0.83/game | – (0) / – / 0.00/game |
| opposition_network | 2.5 (1868) / 48% / 47% | 1.5 (1337) / 100% / 0% |
| orbital_lance | 2.4 (693) / 45% / 2% | – (0) / – / 0% |
| overclock_rig | 2.4 (1919) / 48% / 40% | 1.0 (1593) / 100% / 0% |
| point_defense | – (0) / – / 0.68/game | – (0) / – / 0.00/game |
| power_share | 2.5 (2100) / 55% / 61% | 3.6 (1345) / 100% / 0% |
| precision_strike | 2.6 (2123) / 48% / 57% | 1.7 (5928) / 100% / 93% |
| purge_protocol | 2.5 (626) / 53% / 0% | – (0) / – / 93% |
| quantum_core | 2.4 (592) / 58% / 0% | – (0) / – / 0% |
| rally | 2.5 (2056) / 44% / 58% | 2.3 (4330) / 99% / 88% |
| rebuild | 2.5 (1933) / 49% / 43% | 2.0 (1692) / 100% / 28% |
| recon_sweep | 2.4 (2279) / 49% / 80% | 3.6 (1337) / 98% / 0% |
| reconstruction | 2.4 (633) / 54% / 1% | – (0) / – / 0% |
| recruitment_hub | 2.5 (1709) / 47% / 50% | 1.6 (1353) / 100% / 0% |
| scavenge | 2.6 (2096) / 55% / 61% | 3.5 (1335) / 100% / 0% |
| server_farm | 2.5 (1679) / 51% / 56% | 1.4 (1332) / 100% / 83% |
| shelter_order | – (0) / – / 0.96/game | – (0) / – / 0.00/game |
| shore_up | 2.5 (2069) / 50% / 58% | 2.8 (1857) / 100% / 31% |
| solar_field | 2.5 (1755) / 53% / 84% | 2.8 (1339) / 100% / 0% |
| strike_team | – (0) / – / 0.80/game | – (0) / – / 1.00/game |
| swarm_override | 2.5 (599) / 49% / 3% | – (0) / – / 0% |
| sweep | 2.4 (2104) / 45% / 55% | 1.5 (4063) / 99% / 83% |
| trade_office | 2.5 (1796) / 52% / 51% | 1.3 (1406) / 100% / 0% |
| turnkey_factory | – (0) / – / 0.87/game | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 0.96/game | – (0) / – / 1.00/game |
| watchtower_net | 2.4 (1823) / 48% / 56% | 1.7 (1335) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| pushback d0 random | 0.0 | 0.000 | 0.0 | 19.9 |
| pushback d0 greedy | 7554.1 | 34.830 | 0.0 | 7590.0 |
| pushback d4 random | 0.0 | 0.000 | 0.0 | 16.2 |
| pushback d4 greedy | 6602.6 | 33.813 | 0.0 | 6634.6 |
| income d0 random | 0.0 | 0.000 | 0.0 | 18.9 |
| income d0 greedy | 6667.1 | 30.868 | 0.0 | 6698.3 |
| income d4 random | 0.0 | 0.000 | 0.0 | 15.7 |
| income d4 greedy | 5265.2 | 27.115 | 0.0 | 5291.5 |
| free_purchases d0 random | 0.0 | 0.000 | 0.0 | 19.2 |
| free_purchases d0 greedy | 6245.3 | 28.996 | 0.0 | 6274.4 |
| free_purchases d4 random | 0.0 | 0.000 | 0.0 | 15.5 |
| free_purchases d4 greedy | 5140.8 | 26.408 | 0.0 | 5166.0 |
| counter_threat d0 random | 0.0 | 0.000 | 0.0 | 18.5 |
| counter_threat d0 greedy | 5898.1 | 27.517 | 0.0 | 5926.3 |
| counter_threat d4 random | 0.0 | 0.000 | 0.0 | 15.9 |
| counter_threat d4 greedy | 5072.1 | 25.750 | 0.0 | 5097.6 |
| opposition_off d0 random | 0.0 | 0.000 | 0.0 | 19.0 |
| opposition_off d0 greedy | 5776.8 | 26.819 | 0.0 | 5804.4 |
| opposition_off d4 random | 0.0 | 0.000 | 0.0 | 15.6 |
| opposition_off d4 greedy | 4803.4 | 24.748 | 0.0 | 4828.0 |
| shared_row d0 random | 0.0 | 0.000 | 0.0 | 18.4 |
| shared_row d0 greedy | 5806.5 | 26.343 | 0.0 | 5834.5 |
| shared_row d4 random | 0.0 | 0.000 | 0.0 | 14.8 |
| shared_row d4 greedy | 4128.2 | 22.754 | 0.0 | 4150.2 |
| threats_off d0 random | 0.0 | 0.000 | 0.0 | 19.4 |
| threats_off d0 greedy | 4375.9 | 21.770 | 0.0 | 4398.2 |
| threats_off d4 random | 0.0 | 0.000 | 0.0 | 16.7 |
| threats_off d4 greedy | 3992.6 | 20.636 | 0.0 | 4013.7 |
| cards_off d0 random | 0.0 | 0.000 | 0.0 | 9.6 |
| cards_off d0 greedy | 1930.8 | 23.325 | 0.0 | 1942.2 |
| cards_off d4 random | 0.0 | 0.000 | 0.0 | 8.4 |
| cards_off d4 greedy | 1277.7 | 19.265 | 0.0 | 1286.6 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c23-baseline-shared_row-d4-greedy-seed200064.json`](records/close-loss-c23-baseline-shared_row-d4-greedy-seed200064.json): close loss at difficulty 4: corruption in round 8, 1 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c23-baseline-shared_row-d4-greedy-seed200036.json`](records/close-loss-c23-baseline-shared_row-d4-greedy-seed200036.json): close loss at difficulty 4: corruption in round 6, 1 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-pushback-d0-random-seed200032.json`](records/surprising-win-c0-baseline-pushback-d0-random-seed200032.json): won by the weakest agent (random)
- [`records/surprising-win-c4-baseline-income-d0-random-seed200032.json`](records/surprising-win-c4-baseline-income-d0-random-seed200032.json): won by the weakest agent (random)

