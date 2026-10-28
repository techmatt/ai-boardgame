# Cycle report: tune-after

007 tuning run, after: the tuned baseline on fresh seeds (100000+), random and greedy (today's strongest agent) at every difficulty. Win-probability probes off for the final run (an earlier probed run of the same games took 45 min: 15%/48% uncertain checkpoints at d0/d4).

2000 games (10 cells × 200 seeds, 100000..=100199) on 12 threads in 554.7 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: base
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy
- Win probability: off
- Decision closeness: every 4 real choice(s) probed by 1-ply greedy scoring; close = best two within 2 points
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0: win rates random 0%, greedy 96%; skill gap +96 pts; stronger agents never do worse.
- Difficulty 0: survival (rounds; wins count extra) random 6.50, greedy 11.81; the ladder is ordered.
- Difficulty 1: win rates random 0%, greedy 95%; skill gap +95 pts; stronger agents never do worse.
- Difficulty 1: survival (rounds; wins count extra) random 6.54, greedy 11.83; the ladder is ordered.
- Difficulty 2: win rates random 0%, greedy 86%; skill gap +86 pts; stronger agents never do worse.
- Difficulty 2: survival (rounds; wins count extra) random 6.48, greedy 10.55; the ladder is ordered.
- Difficulty 3: win rates random 0%, greedy 86%; skill gap +86 pts; stronger agents never do worse.
- Difficulty 3: survival (rounds; wins count extra) random 6.48, greedy 10.55; the ladder is ordered.
- Difficulty 4: win rates random 0%, greedy 73%; skill gap +73 pts; stronger agents never do worse.
- Difficulty 4: survival (rounds; wins count extra) random 5.79, greedy 9.81; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +96 → +73 pts win rate, +5.31 → +4.02 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.7 rounds at difficulty 0, 7.0 at 4 (target 8–10).
- Games ended by: corruption 1067, deck_empty 31, opposition 871, population 31.
- Adversary operations: 12.6 per round on average across cells, at most 54 in one round (target: low).
- Decisions (strongest agent): 4% forced, 68% of real choices close (best two within 2 points).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 15% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 15% of threats countered.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.50) | 95.5 ±1.5 (11.81) | +95.5 pts (+5.31 survival) |

| difficulty 1 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.54) | 95.0 ±1.5 (11.83) | +95.0 pts (+5.29 survival) |

| difficulty 2 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.48) | 86.0 ±2.5 (10.55) | +86.0 pts (+4.07 survival) |

| difficulty 3 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.48) | 86.0 ±2.5 (10.55) | +86.0 pts (+4.07 survival) |

| difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.79) | 73.0 ±3.1 (9.81) | +73.0 pts (+4.02 survival) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 random | 6.50 (1.03, 4..9) | corruption 185, population 15 | – / – | 58.1 / 0.01 | 0.42 | 2.10 |
| d0 greedy | 7.75 (0.94, 3..10) | corruption 7, deck_empty 1, opposition 191, population 1 | 2.26 / 17.06 | 13.9 / 1.78 | 16.38 | 5.37 |
| d1 random | 6.54 (0.91, 5..9) | corruption 191, population 9 | – / – | 59.7 / 0.00 | 0.25 | 2.32 |
| d1 greedy | 7.79 (0.94, 3..10) | corruption 6, deck_empty 4, opposition 190 | 2.25 / 15.58 | 14.8 / 1.70 | 15.00 | 5.37 |
| d2 random | 6.48 (1.34, 4..9) | corruption 197, population 3 | – / – | 59.6 / 0.01 | 0.07 | 3.06 |
| d2 greedy | 7.50 (1.00, 3..9) | corruption 17, deck_empty 11, opposition 172 | 1.56 / 14.52 | 12.6 / 1.79 | 13.11 | 5.46 |
| d3 random | 6.48 (1.34, 4..9) | corruption 197, population 3 | – / – | 59.6 / 0.01 | 0.07 | 3.06 |
| d3 greedy | 7.50 (1.00, 3..9) | corruption 17, deck_empty 11, opposition 172 | 1.55 / 14.48 | 12.6 / 1.79 | 13.08 | 5.47 |
| d4 random | 5.79 (1.33, 4..9) | corruption 200 | – / – | 45.1 / 0.00 | 0.00 | 4.10 |
| d4 greedy | 7.04 (1.22, 3..9) | corruption 50, deck_empty 4, opposition 146 | 1.81 / 8.64 | 14.7 / 1.33 | 6.45 | 5.53 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 random | 201.0 | 6.88 | 5% | 72% | 2.2 | 15.38 (45) | 4.79 | 8.06 | 1.88 | 2.77 / 13.00 (21%) |
| d0 greedy | 214.0 | 10.09 | 4% | 67% | 2.6 | 9.47 (42) | 0.00 | 9.25 | 0.00 | 5.69 / 15.49 (37%) |
| d1 random | 206.0 | 6.87 | 4% | 74% | 2.3 | 15.60 (47) | 4.97 | 9.16 | 3.08 | 1.45 / 13.07 (11%) |
| d1 greedy | 216.1 | 9.80 | 4% | 68% | 2.6 | 10.37 (41) | 0.00 | 7.01 | 0.00 | 1.93 / 15.59 (12%) |
| d2 random | 203.0 | 6.83 | 4% | 75% | 2.1 | 15.50 (54) | 5.03 | 8.05 | 2.85 | 1.06 / 12.96 (8%) |
| d2 greedy | 208.0 | 9.57 | 4% | 68% | 2.5 | 10.83 (47) | 0.00 | 5.78 | 0.00 | 1.36 / 14.99 (9%) |
| d3 random | 202.9 | 6.83 | 4% | 75% | 2.1 | 15.50 (54) | 4.98 | 8.05 | 2.85 | 1.06 / 12.96 (8%) |
| d3 greedy | 208.0 | 9.56 | 4% | 68% | 2.5 | 10.84 (47) | 0.00 | 5.78 | 0.00 | 1.36 / 15.00 (9%) |
| d4 random | 174.5 | 6.73 | 4% | 74% | 2.0 | 12.41 (49) | 4.21 | 5.88 | 2.12 | 0.66 / 11.58 (6%) |
| d4 greedy | 195.0 | 9.30 | 4% | 69% | 2.5 | 9.92 (43) | 0.00 | 4.14 | 0.00 | 0.78 / 14.07 (6%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | 4% / 91% / 4% | 7% / 88% / 5% |
| d1 | 5% / 90% / 5% | 5% / 90% / 5% |
| d2 | 12% / 75% / 12% | 16% / 72% / 12% |
| d3 | 12% / 75% / 12% | 16% / 72% / 12% |
| d4 | 21% / 57% / 21% | 29% / 58% / 13% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 random | 200 of 200 | 0.75 | 0 of 0 | – |
| d0 greedy | 3 of 200 | 0.36 | 3 of 191 | 0.36 |
| d1 random | 200 of 200 | 0.75 | 0 of 0 | – |
| d1 greedy | 3 of 200 | 0.37 | 3 of 190 | 0.37 |
| d2 random | 200 of 200 | 0.76 | 0 of 0 | – |
| d2 greedy | 3 of 200 | 0.37 | 3 of 172 | 0.36 |
| d3 random | 200 of 200 | 0.76 | 0 of 0 | – |
| d3 greedy | 3 of 200 | 0.37 | 3 of 172 | 0.36 |
| d4 random | 199 of 200 | 0.76 | 0 of 0 | – |
| d4 greedy | 3 of 200 | 0.37 | 3 of 146 | 0.36 |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | 2.4 (546) / 42% / 0% | – (0) / – / 0% |
| assembly_cluster | 2.4 (1368) / 50% / 45% | 1.1 (1141) / 100% / 0% |
| civic_shelter | 2.4 (1370) / 49% / 56% | 2.0 (993) / 100% / 0% |
| contract_bid | 2.5 (1769) / 51% / 82% | 3.9 (982) / 99% / 0% |
| counter_intelligence | 2.5 (535) / 50% / 0% | – (0) / – / 0% |
| data_mining | 2.5 (1667) / 50% / 62% | 3.6 (990) / 99% / 0% |
| emergency_fab | 2.5 (1405) / 50% / 16% | 2.5 (986) / 100% / 0% |
| evacuate | 2.5 (1593) / 52% / 60% | 3.1 (995) / 99% / 1% |
| fab_line | 2.5 (1364) / 51% / 53% | 1.7 (995) / 99% / 0% |
| firewall_node | 2.4 (1379) / 45% / 50% | 1.8 (994) / 100% / 0% |
| fork_process | 2.5 (1575) / 51% / 60% | 2.1 (2002) / 100% / 53% |
| fusion_plant | 2.5 (527) / 50% / 0% | – (0) / – / 0% |
| global_mandate | 2.5 (557) / 45% / 1% | – (0) / – / 0% |
| grant | 2.5 (1638) / 48% / 62% | 3.6 (992) / 100% / 0% |
| grid_battery | 2.5 (1387) / 46% / 52% | 2.2 (981) / 100% / 0% |
| jam_signals | 2.5 (1700) / 48% / 77% | 3.3 (982) / 100% / 0% |
| liquidate_reserves | 2.5 (1621) / 46% / 62% | 3.6 (1003) / 99% / 0% |
| logistics_ai | 2.5 (1356) / 50% / 46% | 1.2 (1079) / 100% / 0% |
| megastructure | 2.5 (529) / 46% / 0% | – (0) / – / 0% |
| microgrid | 2.6 (1388) / 45% / 57% | 2.3 (988) / 100% / 0% |
| modular_expansion | – (0) / – / 0.85/game | – (0) / – / 0.00/game |
| opposition_network | 2.4 (1382) / 50% / 51% | 1.5 (997) / 100% / 0% |
| orbital_lance | 2.5 (527) / 43% / 0% | – (0) / – / 0% |
| overclock_rig | 2.5 (1374) / 51% / 45% | 1.0 (1225) / 100% / 0% |
| point_defense | – (0) / – / 0.69/game | – (0) / – / 0.00/game |
| power_share | 2.6 (1654) / 50% / 61% | 3.6 (969) / 100% / 0% |
| precision_strike | 2.5 (1643) / 48% / 53% | 1.6 (4483) / 100% / 92% |
| purge_protocol | 2.4 (519) / 49% / 0% | – (0) / – / 95% |
| quantum_core | 2.5 (521) / 41% / 0% | – (0) / – / 0% |
| rally | 2.5 (1640) / 47% / 58% | 2.2 (3321) / 99% / 88% |
| rebuild | 2.5 (1474) / 48% / 47% | 2.0 (1267) / 100% / 28% |
| recon_sweep | 2.5 (1706) / 47% / 77% | 3.6 (991) / 99% / 0% |
| reconstruction | 2.4 (521) / 47% / 1% | – (0) / – / 0% |
| recruitment_hub | 2.6 (1430) / 47% / 51% | 1.4 (983) / 100% / 0% |
| scavenge | 2.5 (1608) / 49% / 65% | 3.6 (989) / 99% / 0% |
| server_farm | 2.4 (1397) / 48% / 52% | 1.5 (991) / 100% / 85% |
| shelter_order | – (0) / – / 0.96/game | – (0) / – / 0.00/game |
| shore_up | 2.5 (1637) / 49% / 57% | 2.9 (1273) / 99% / 24% |
| solar_field | 2.5 (1388) / 46% / 73% | 2.7 (991) / 99% / 0% |
| strike_team | – (0) / – / 0.74/game | – (0) / – / 1.00/game |
| swarm_override | 2.5 (517) / 44% / 2% | – (0) / – / 0% |
| sweep | 2.4 (1593) / 50% / 53% | 1.5 (3001) / 100% / 82% |
| trade_office | 2.5 (1439) / 48% / 47% | 1.3 (1061) / 100% / 0% |
| turnkey_factory | – (0) / – / 0.87/game | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 0.97/game | – (0) / – / 1.00/game |
| watchtower_net | 2.5 (1297) / 51% / 55% | 1.9 (997) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 random | 0.0 | 0.000 | 359.4 | 384.5 |
| d0 greedy | 6806.3 | 31.801 | 452.6 | 7292.1 |
| d1 random | 0.0 | 0.000 | 354.9 | 381.3 |
| d1 greedy | 5921.5 | 27.396 | 388.8 | 6340.3 |
| d2 random | 0.0 | 0.000 | 302.2 | 325.7 |
| d2 greedy | 5441.0 | 26.161 | 369.0 | 5838.7 |
| d3 random | 0.0 | 0.000 | 323.9 | 348.4 |
| d3 greedy | 5891.5 | 28.322 | 395.3 | 6318.1 |
| d4 random | 0.0 | 0.000 | 296.6 | 317.6 |
| d4 greedy | 5111.6 | 26.213 | 346.8 | 5486.8 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c9-baseline-base-d4-greedy-seed100019.json`](records/close-loss-c9-baseline-base-d4-greedy-seed100019.json): close loss at difficulty 4: corruption in round 8, 1 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c9-baseline-base-d4-greedy-seed100047.json`](records/close-loss-c9-baseline-base-d4-greedy-seed100047.json): close loss at difficulty 4: corruption in round 9, 3 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c9-baseline-base-d4-greedy-seed100158.json`](records/close-loss-c9-baseline-base-d4-greedy-seed100158.json): close loss at difficulty 4: corruption in round 6, 3 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c9-baseline-base-d4-greedy-seed100187.json`](records/close-loss-c9-baseline-base-d4-greedy-seed100187.json): close loss at difficulty 4: deck_empty in round 9, 4 adversary piece(s) over the level-2 bar, pool 9 of 14

**early-lock-in**: none matched


**surprising-win**: none matched


