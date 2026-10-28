# Cycle report: variants-spread

007: the three spread variants at the tuned numbers (the tuning was fitted to the Militarist; the others share its non-adversary numbers, so their results are indicative).

1152 games (12 cells × 96 seeds, 200000..=200095) on 12 threads in 146.8 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/baseline.json, configs/clusters.json, configs/assets.json
- Variants: base
- Difficulties: 0, 4
- Agents (weakest first): random, greedy
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, base): win rates random 1%, greedy 94%; skill gap +93 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, base): survival (rounds; wins count extra) random 6.47, greedy 11.78; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, base): win rates random 0%, greedy 66%; skill gap +66 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, base): survival (rounds; wins count extra) random 5.59, greedy 9.43; the ladder is ordered.
- Difficulty 0 (configs/clusters.json, base): win rates random 0%, greedy 1%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 0 (configs/clusters.json, base): survival (rounds; wins count extra) random 4.12, greedy 4.10; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/clusters.json, base): win rates random 0%, greedy 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/clusters.json, base): survival (rounds; wins count extra) random 3.74, greedy 3.80; the ladder is ordered.
- Difficulty 0 (configs/assets.json, base): win rates random 0%, greedy 4%; skill gap +4 pts; stronger agents never do worse.
- Difficulty 0 (configs/assets.json, base): survival (rounds; wins count extra) random 2.04, greedy 2.78; the ladder is ordered.
- Difficulty 4 (configs/assets.json, base): win rates random 0%, greedy 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/assets.json, base): survival (rounds; wins count extra) random 1.00, greedy 1.00; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +33 → +22 pts win rate, +2.01 → +1.30 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.7 rounds at difficulty 0, 6.9 at 4 (target 8–10).
- Games ended by: corruption 692, deck_empty 5, opposition 159, population 296.
- Adversary operations: 10.6 per round on average across cells, at most 50 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 47% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 26% of threats countered.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| configs/baseline.json difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 1.0 ±1.0 (6.47) | 93.8 ±2.5 (11.78) | +92.7 pts (+5.31 survival) |

| configs/baseline.json difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.59) | 65.6 ±4.8 (9.43) | +65.6 pts (+3.83 survival) |

| configs/clusters.json difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.12) | 1.0 ±1.0 (4.10) | +1.0 pts (-0.02 survival) |

| configs/clusters.json difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.74) | 0.0 ±0.0 (3.80) | +0.0 pts (+0.06 survival) |

| configs/assets.json difficulty 0 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.04) | 4.2 ±2.0 (2.78) | +4.2 pts (+0.74 survival) |

| configs/assets.json difficulty 4 | random | greedy | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (1.00) | 0.0 ±0.0 (1.00) | +0.0 pts (+0.00 survival) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| baseline d0 random | 6.36 (1.13, 2..10) | corruption 92, opposition 1, population 3 | 8.00 / 22.00 | 59.5 / 0.01 | 0.30 | 2.07 |
| baseline d0 greedy | 7.74 (1.15, 3..10) | corruption 2, deck_empty 3, opposition 90, population 1 | 2.31 / 16.14 | 8.0 / 2.00 | 15.45 | 5.24 |
| baseline d4 random | 5.59 (1.39, 4..9) | corruption 96 | – / – | 43.9 / 0.00 | 0.00 | 3.93 |
| baseline d4 greedy | 6.94 (1.18, 3..9) | corruption 31, deck_empty 2, opposition 63 | 1.79 / 8.57 | 16.9 / 1.21 | 5.79 | 5.28 |
| clusters d0 random | 4.12 (0.93, 2..6) | corruption 1, population 95 | – / – | 0.0 / 0.00 | 16.70 | -3.05 |
| clusters d0 greedy | 4.05 (0.98, 2..7) | opposition 1, population 95 | 3.00 / 5.00 | 0.0 / 0.26 | 18.15 | -3.32 |
| clusters d4 random | 3.74 (0.77, 2..6) | corruption 48, population 48 | – / – | 0.0 / 0.00 | 5.02 | -0.61 |
| clusters d4 greedy | 3.80 (0.84, 2..6) | corruption 42, population 54 | – / – | 0.0 / 0.15 | 5.50 | -0.96 |
| assets d0 random | 2.04 (0.41, 1..3) | corruption 96 | – / – | 0.0 / 0.00 | 0.00 | 5.43 |
| assets d0 greedy | 2.32 (0.85, 1..7) | corruption 92, opposition 4 | 9.00 / 7.00 | 0.0 / 0.01 | 0.29 | 5.01 |
| assets d4 random | 1.00 (0.00, 1..1) | corruption 96 | – / – | 0.0 / 0.00 | 0.00 | 6.00 |
| assets d4 greedy | 1.00 (0.00, 1..1) | corruption 96 | – / – | 0.0 / 0.00 | 0.00 | 6.00 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| baseline d0 random | 197.6 | 6.87 | 5% | – | – | 15.91 (47) | 5.02 | 7.97 | 1.84 | 2.67 / 12.73 (21%) |
| baseline d0 greedy | 216.9 | 10.11 | 4% | – | – | 10.17 (42) | 0.00 | 9.21 | 0.00 | 5.71 / 15.48 (37%) |
| baseline d4 random | 171.9 | 6.84 | 5% | – | – | 12.65 (39) | 4.15 | 5.62 | 1.77 | 0.72 / 11.19 (6%) |
| baseline d4 greedy | 195.3 | 9.35 | 4% | – | – | 10.53 (38) | 0.00 | 3.96 | 0.00 | 0.83 / 13.88 (6%) |
| clusters d0 random | 118.5 | 6.82 | 5% | – | – | 17.60 (49) | 2.89 | 5.97 | 1.32 | 2.18 / 8.25 (26%) |
| clusters d0 greedy | 117.2 | 8.02 | 4% | – | – | 15.05 (48) | 0.00 | 6.55 | 0.00 | 4.27 / 8.10 (53%) |
| clusters d4 random | 108.2 | 6.47 | 5% | – | – | 18.11 (50) | 2.69 | 3.84 | 1.21 | 0.55 / 7.48 (7%) |
| clusters d4 greedy | 111.5 | 7.29 | 4% | – | – | 17.16 (48) | 0.00 | 3.46 | 0.00 | 0.80 / 7.60 (11%) |
| assets d0 random | 59.8 | 6.15 | 5% | – | – | 5.08 (13) | 1.66 | 2.47 | 0.61 | 0.91 / 4.08 (22%) |
| assets d0 greedy | 69.2 | 6.27 | 4% | – | – | 5.29 (11) | 0.00 | 3.46 | 0.00 | 2.26 / 4.65 (49%) |
| assets d4 random | 0.0 | 0.00 | 0% | – | – | 0.00 (0) | 0.00 | 0.00 | 0.00 | 0.00 / 2.00 (0%) |
| assets d4 greedy | 0.0 | 0.00 | 0% | – | – | 0.00 (0) | 0.00 | 0.00 | 0.00 | 0.00 / 2.00 (0%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| configs/baseline.json d0 | 7% / 86% / 7% | 8% / 86% / 6% |
| configs/baseline.json d4 | 26% / 49% / 26% | 35% / 51% / 15% |
| configs/clusters.json d0 | 50% / 1% / 50% | 66% / 0% / 34% |
| configs/clusters.json d4 | – (no variance) | 78% / 0% / 22% |
| configs/assets.json d0 | 49% / 2% / 49% | 48% / 6% / 47% |
| configs/assets.json d4 | – (no variance) | – (no variance) |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| baseline d0 random | 96 of 96 | 0.76 | 1 of 1 | – |
| baseline d0 greedy | 3 of 96 | 0.37 | 3 of 90 | 0.37 |
| baseline d4 random | 96 of 96 | 0.77 | 0 of 0 | – |
| baseline d4 greedy | 3 of 96 | 0.37 | 3 of 63 | 0.34 |
| clusters d0 random | 94 of 96 | 0.80 | 0 of 0 | – |
| clusters d0 greedy | 3 of 96 | 0.32 | 1 of 1 | – |
| clusters d4 random | 90 of 96 | 0.79 | 0 of 0 | – |
| clusters d4 greedy | 3 of 96 | 0.30 | 0 of 0 | – |
| assets d0 random | 66 of 96 | 0.79 | 0 of 0 | – |
| assets d0 greedy | 4 of 96 | 0.32 | 1 of 4 | 0.00 |
| assets d4 random | 1 of 96 | 0.00 | 0 of 0 | – |
| assets d4 greedy | 1 of 96 | 0.00 | 0 of 0 | – |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | 2.6 (128) / 38% / 6% | – (0) / – / 0% |
| assembly_cluster | 2.6 (481) / 54% / 36% | 1.1 (459) / 100% / 0% |
| civic_shelter | 2.5 (494) / 47% / 42% | 2.1 (417) / 100% / 0% |
| contract_bid | 2.5 (581) / 45% / 75% | 4.0 (433) / 99% / 0% |
| counter_intelligence | 2.4 (120) / 62% / 0% | – (0) / – / 0% |
| data_mining | 2.6 (540) / 56% / 53% | 3.7 (419) / 98% / 0% |
| emergency_fab | 2.5 (472) / 51% / 12% | 2.5 (408) / 99% / 0% |
| evacuate | 2.6 (561) / 49% / 51% | 3.0 (439) / 100% / 1% |
| fab_line | 2.6 (486) / 56% / 48% | 1.8 (428) / 100% / 0% |
| firewall_node | 2.6 (479) / 52% / 48% | 1.9 (417) / 100% / 0% |
| fork_process | 2.5 (553) / 49% / 56% | 2.2 (708) / 100% / 54% |
| fusion_plant | 2.5 (116) / 50% / 0% | – (0) / – / 0% |
| global_mandate | 2.3 (118) / 54% / 0% | – (0) / – / 0% |
| grant | 2.5 (541) / 52% / 56% | 3.5 (433) / 98% / 0% |
| grid_battery | 2.4 (480) / 52% / 48% | 2.1 (414) / 100% / 0% |
| jam_signals | 2.4 (518) / 53% / 69% | 3.2 (414) / 99% / 0% |
| liquidate_reserves | 2.4 (522) / 49% / 51% | 3.6 (423) / 98% / 0% |
| logistics_ai | 2.5 (484) / 43% / 37% | 1.3 (437) / 100% / 0% |
| megastructure | 2.4 (138) / 47% / 0% | – (0) / – / 0% |
| microgrid | 2.6 (491) / 49% / 41% | 2.4 (439) / 100% / 0% |
| modular_expansion | – (0) / – / 0.59/game | – (0) / – / 0.00/game |
| opposition_network | 2.5 (525) / 48% / 37% | 1.5 (440) / 99% / 0% |
| orbital_lance | 2.4 (139) / 43% / 1% | – (0) / – / 0% |
| overclock_rig | 2.4 (525) / 47% / 33% | 1.0 (474) / 100% / 0% |
| point_defense | – (0) / – / 0.45/game | – (0) / – / 0.00/game |
| power_share | 2.5 (546) / 55% / 58% | 3.6 (433) / 100% / 0% |
| precision_strike | 2.6 (554) / 46% / 52% | 1.9 (1241) / 99% / 93% |
| purge_protocol | 2.6 (120) / 50% / 0% | – (0) / – / 87% |
| quantum_core | 2.4 (111) / 59% / 0% | – (0) / – / 0% |
| rally | 2.5 (533) / 44% / 51% | 2.4 (945) / 98% / 81% |
| rebuild | 2.6 (518) / 46% / 34% | 2.2 (475) / 99% / 25% |
| recon_sweep | 2.4 (561) / 50% / 76% | 3.6 (420) / 98% / 0% |
| reconstruction | 2.4 (123) / 56% / 1% | – (0) / – / 0% |
| recruitment_hub | 2.6 (478) / 47% / 40% | 1.6 (435) / 100% / 0% |
| scavenge | 2.6 (556) / 57% / 53% | 3.5 (439) / 99% / 0% |
| server_farm | 2.4 (463) / 51% / 48% | 1.4 (420) / 100% / 67% |
| shelter_order | – (0) / – / 0.75/game | – (0) / – / 0.00/game |
| shore_up | 2.4 (552) / 50% / 50% | 2.9 (512) / 100% / 17% |
| solar_field | 2.5 (476) / 54% / 76% | 2.8 (420) / 99% / 0% |
| strike_team | – (0) / – / 0.51/game | – (0) / – / 0.78/game |
| swarm_override | 2.6 (114) / 53% / 3% | – (0) / – / 0% |
| sweep | 2.4 (562) / 45% / 49% | 1.8 (803) / 100% / 58% |
| trade_office | 2.4 (493) / 52% / 47% | 1.3 (436) / 100% / 0% |
| turnkey_factory | – (0) / – / 0.63/game | – (0) / – / 0.83/game |
| venture_capital | – (0) / – / 0.73/game | – (0) / – / 0.83/game |
| watchtower_net | 2.4 (505) / 50% / 49% | 1.7 (429) / 99% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| baseline d0 random | 0.0 | 0.000 | 0.0 | 19.9 |
| baseline d0 greedy | 8473.8 | 39.071 | 0.0 | 8514.5 |
| baseline d4 random | 0.0 | 0.000 | 0.0 | 16.4 |
| baseline d4 greedy | 6167.8 | 31.586 | 0.0 | 6197.1 |
| clusters d0 random | 0.0 | 0.000 | 0.0 | 10.9 |
| clusters d0 greedy | 1595.7 | 13.618 | 0.0 | 1612.7 |
| clusters d4 random | 0.0 | 0.000 | 0.0 | 9.7 |
| clusters d4 greedy | 1350.4 | 12.107 | 0.0 | 1365.8 |
| assets d0 random | 0.0 | 0.000 | 0.0 | 4.7 |
| assets d0 greedy | 445.6 | 6.441 | 0.0 | 453.3 |
| assets d4 random | 0.0 | 0.000 | 0.0 | 0.2 |
| assets d4 greedy | 0.0 | 0.000 | 0.0 | 0.2 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c7-clusters-base-d4-greedy-seed200014.json`](records/close-loss-c7-clusters-base-d4-greedy-seed200014.json): close loss at difficulty 4: population in round 6, 0 adversary piece(s) over the level-1 bar, pool 2 of 14
- [`records/close-loss-c7-clusters-base-d4-greedy-seed200024.json`](records/close-loss-c7-clusters-base-d4-greedy-seed200024.json): close loss at difficulty 4: population in round 6, 0 adversary piece(s) over the level-2 bar, pool 1 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-base-d0-random-seed200032.json`](records/surprising-win-c0-baseline-base-d0-random-seed200032.json): won by the weakest agent (random)

