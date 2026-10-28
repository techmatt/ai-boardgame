# Cycle report: ladder-v0

First skill-ladder cycle on ruleset v0: baseline at difficulties 0 and 4, random, greedy and beam agents.

600 games (6 cells × 100 seeds, 0..=99) on 12 threads in 220.5 s. Engine 0.1.0, rules r2, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: base
- Difficulties: 0, 4
- Agents (weakest first): random, greedy, beam
- Win probability: first 10 seeds per cell, 8 `greedy` playouts at the start of each round
- Decision closeness: every 1 real choice(s) probed by 1-ply greedy scoring; close = best two within 2 points
- Replay check: every game replays from seed + actions

## Findings

- **No agent won a game** (600 games). Targets: at difficulty 0 moderately skilled play essentially always wins; at the top difficulty the strongest agent wins narrowly. The ladder can only be read off survival (rounds, margins) until the rules are tuned.
- Difficulty 0: survival (rounds; wins count extra) random 3.15, greedy 3.96, beam 4.16; the ladder is ordered.
- Difficulty 4: survival (rounds; wins count extra) random 2.81, greedy 3.12, beam 3.27; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +0 → +0 pts win rate, +1.01 → +0.46 survival (target: widens with difficulty).
- Game length for the strongest agent (beam): 4.2 rounds at difficulty 0, 3.3 at 4 (target 8–10).
- Games ended by: corruption 600.
- Adversary operations: 12.1 per round on average across cells, at most 35 in one round (target: low).
- Decisions (strongest agent): 8% forced, 83% of real choices close (best two within 2 points).
- Cards (beam, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, overclock_rig, trade_office.
- Luck share of survival (seed's share of the variance): 71% on average across groups (target: most variance from decisions).
- Win probability: 0% of checkpoints uncertain, 100% of games decided by round 2 (target: uncertain into the late game). Playouts use `greedy`, so they measure that agent's chances.
- Help and counters (strongest agent): 2.26 gifts per game; 27% of threats countered.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | random | greedy | beam | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.15) | 0.0 ±0.0 (3.96) | 0.0 ±0.0 (4.16) | +0.0 pts (+1.01 survival) |

| difficulty 4 | random | greedy | beam | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.81) | 0.0 ±0.0 (3.12) | 0.0 ±0.0 (3.27) | +0.0 pts (+0.46 survival) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 random | 3.15 (0.85, 2..5) | corruption 100 | – / – | 31.8 / 0.00 | 0.00 | 4.46 |
| d0 greedy | 3.96 (0.69, 3..6) | corruption 100 | – / – | 42.9 / 0.00 | 0.00 | 4.54 |
| d0 beam | 4.16 (0.81, 3..7) | corruption 100 | – / – | 38.4 / 0.00 | 0.00 | 5.32 |
| d4 random | 2.81 (0.64, 2..4) | corruption 100 | – / – | 27.7 / 0.00 | 0.00 | 4.84 |
| d4 greedy | 3.12 (0.90, 2..5) | corruption 100 | – / – | 30.8 / 0.00 | 0.00 | 5.03 |
| d4 beam | 3.27 (1.07, 2..6) | corruption 100 | – / – | 29.1 / 0.00 | 0.00 | 5.41 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 random | 84.6 | 5.95 | 7% | 77% | 1.5 | 12.01 (27) | 3.02 | 4.02 | 1.06 | 1.40 / 6.30 (22%) |
| d0 greedy | 111.1 | 5.94 | 6% | 79% | 1.5 | 13.82 (35) | 0.00 | 5.77 | 0.00 | 3.01 / 7.92 (38%) |
| d0 beam | 108.8 | 6.58 | 8% | 83% | 1.5 | 13.05 (35) | 2.43 | 6.01 | 0.03 | 3.54 / 8.32 (43%) |
| d4 random | 73.9 | 5.71 | 6% | 81% | 1.3 | 10.62 (27) | 2.53 | 2.22 | 0.78 | 0.18 / 5.62 (3%) |
| d4 greedy | 85.3 | 5.67 | 6% | 80% | 1.5 | 11.55 (29) | 0.00 | 2.31 | 0.00 | 0.41 / 6.24 (7%) |
| d4 beam | 83.8 | 6.20 | 8% | 84% | 1.4 | 11.25 (33) | 2.09 | 2.44 | 0.03 | 0.80 / 6.54 (12%) |

## Win probability by round

Mean estimated win probability at the start of each round (games still running in brackets). Uncertain: checkpoints with 0.1 < p < 0.9. Early lock-in: games whose result was already decided (p ≤ 0.05 for a loss, ≥ 0.95 for a win, to the end) by round 2.

| cell | uncertain | early lock-in | r1 | r2 | r3 | r4 | r5 |
|---|---|---|---|---|---|---|---|
| d0 random | 0% | 100% | 0.00 (10) | 0.00 (10) | 0.00 (6) | 0.00 (4) | |
| d0 greedy | 0% | 100% | 0.00 (10) | 0.00 (10) | 0.00 (10) | 0.00 (8) | 0.00 (1) |
| d0 beam | 0% | 100% | 0.00 (10) | 0.00 (10) | 0.00 (10) | 0.00 (8) | 0.00 (2) |
| d4 random | 0% | 100% | 0.00 (10) | 0.00 (10) | 0.00 (6) | 0.00 (1) | |
| d4 greedy | 0% | 100% | 0.00 (10) | 0.00 (10) | 0.00 (7) | 0.00 (4) | |
| d4 beam | 0% | 100% | 0.00 (10) | 0.00 (10) | 0.00 (7) | 0.00 (3) | 0.00 (1) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | – (no variance) | 60% / 24% / 16% |
| d4 | – (no variance) | 83% / 4% / 13% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 random | 68 of 100 | 0.76 | 0 of 0 | – |
| d0 greedy | 3 of 100 | 0.09 | 0 of 0 | – |
| d0 beam | 23 of 100 | 0.48 | 0 of 0 | – |
| d4 random | 66 of 100 | 0.78 | 0 of 0 | – |
| d4 greedy | 4 of 100 | 0.27 | 0 of 0 | – |
| d4 beam | 22 of 100 | 0.44 | 0 of 0 | – |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | beam: pick (offered) / keep / play |
|---|---|---|---|
| antimatter_refinery | 2.1 (19) / 53% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| assembly_cluster | 2.6 (166) / 39% / 24% | 1.1 (187) / 86% / 0% | 1.4 (195) / 93% / 0% |
| civic_shelter | 2.5 (155) / 43% / 30% | 2.1 (194) / 65% / 0% | 2.0 (194) / 62% / 4% |
| contract_bid | 2.4 (155) / 37% / 77% | 4.0 (193) / 27% / 0% | 3.9 (211) / 27% / 67% |
| counter_intelligence | 2.4 (25) / 32% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| data_mining | 2.4 (156) / 44% / 53% | 3.6 (197) / 42% / 0% | 3.6 (206) / 47% / 12% |
| emergency_fab | 2.5 (153) / 44% / 6% | 2.5 (187) / 59% / 0% | 2.5 (203) / 62% / 0% |
| evacuate | 2.6 (154) / 34% / 42% | 3.0 (189) / 52% / 2% | 3.0 (205) / 52% / 5% |
| fab_line | 2.6 (147) / 32% / 21% | 1.8 (174) / 78% / 0% | 1.8 (178) / 80% / 0% |
| firewall_node | 2.4 (147) / 37% / 24% | 1.9 (182) / 74% / 0% | 1.9 (187) / 80% / 2% |
| fork_process | 2.7 (148) / 35% / 38% | 2.7 (196) / 46% / 40% | 2.7 (202) / 44% / 4% |
| fusion_plant | 2.3 (17) / 29% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| global_mandate | 2.3 (24) / 25% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| grant | 2.4 (156) / 42% / 54% | 3.7 (200) / 40% / 0% | 3.6 (204) / 43% / 26% |
| grid_battery | 2.3 (153) / 39% / 35% | 1.9 (189) / 61% / 0% | 1.9 (191) / 72% / 18% |
| jam_signals | 2.5 (149) / 47% / 59% | 3.3 (195) / 42% / 0% | 3.2 (206) / 39% / 37% |
| liquidate_reserves | 2.4 (155) / 41% / 54% | 3.6 (193) / 41% / 0% | 3.6 (205) / 46% / 11% |
| logistics_ai | 2.5 (162) / 43% / 22% | 1.3 (190) / 81% / 0% | 1.5 (193) / 89% / 0% |
| megastructure | 2.3 (18) / 22% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| microgrid | 2.7 (151) / 34% / 24% | 2.3 (192) / 60% / 0% | 2.1 (199) / 58% / 27% |
| modular_expansion | – (0) / – / 0.56/game | – (0) / – / 0.00/game | – (0) / – / 0.01/game |
| opposition_network | 2.4 (162) / 43% / 21% | 1.6 (183) / 82% / 0% | 1.6 (190) / 84% / 0% |
| orbital_lance | 2.8 (14) / 43% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| overclock_rig | 2.3 (145) / 46% / 14% | 1.0 (175) / 91% / 0% | 1.4 (179) / 96% / 0% |
| point_defense | – (0) / – / 0.24/game | – (0) / – / 0.00/game | – (0) / – / 0.04/game |
| power_share | 2.6 (159) / 32% / 58% | 3.6 (197) / 43% / 0% | 3.5 (203) / 45% / 11% |
| precision_strike | 2.7 (144) / 43% / 49% | 2.6 (202) / 54% / 92% | 2.6 (215) / 53% / 71% |
| purge_protocol | 2.7 (15) / 40% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| quantum_core | 2.6 (20) / 35% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| rally | 2.5 (163) / 37% / 40% | 3.0 (201) / 58% / 0% | 3.0 (233) / 54% / 55% |
| rebuild | 2.5 (162) / 34% / 7% | 2.2 (189) / 66% / 0% | 2.0 (209) / 59% / 47% |
| recon_sweep | 2.1 (132) / 46% / 67% | 3.6 (188) / 29% / 0% | 3.7 (201) / 41% / 31% |
| reconstruction | 3.1 (22) / 41% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| recruitment_hub | 2.6 (156) / 36% / 20% | 1.6 (178) / 83% / 0% | 1.6 (183) / 89% / 0% |
| scavenge | 2.7 (155) / 41% / 52% | 3.7 (196) / 39% / 0% | 3.7 (199) / 48% / 11% |
| server_farm | 2.6 (134) / 32% / 24% | 1.5 (167) / 81% / 11% | 1.6 (170) / 84% / 9% |
| shelter_order | – (0) / – / 0.83/game | – (0) / – / 0.00/game | – (0) / – / 0.40/game |
| shore_up | 2.6 (145) / 27% / 34% | 3.0 (196) / 42% / 25% | 3.0 (203) / 48% / 11% |
| solar_field | 2.5 (157) / 36% / 40% | 2.6 (188) / 59% / 0% | 2.6 (194) / 57% / 54% |
| strike_team | – (0) / – / 0.57/game | – (0) / – / 0.99/game | – (0) / – / 0.82/game |
| swarm_override | 2.5 (22) / 36% / 0% | – (0) / – / 0% | – (0) / – / 0.00/game |
| sweep | 2.5 (163) / 40% / 26% | 2.4 (194) / 69% / 3% | 2.1 (228) / 69% / 72% |
| trade_office | 2.5 (146) / 45% / 13% | 1.3 (176) / 89% / 0% | 1.4 (180) / 93% / 0% |
| turnkey_factory | – (0) / – / 0.68/game | – (0) / – / 0.92/game | – (0) / – / 0.98/game |
| venture_capital | – (0) / – / 0.81/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 2.5 (142) / 43% / 21% | 1.7 (176) / 77% / 0% | 1.7 (175) / 77% / 8% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 random | 0.0 | 0.000 | 3230.0 | 3237.7 |
| d0 greedy | 1452.5 | 13.073 | 3530.4 | 4993.0 |
| d0 beam | 3196.1 | 29.370 | 5100.3 | 8308.4 |
| d4 random | 0.0 | 0.000 | 1784.2 | 1790.9 |
| d4 greedy | 989.7 | 11.603 | 2193.8 | 3191.3 |
| d4 beam | 2014.0 | 24.045 | 2092.6 | 4114.5 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c5-baseline-base-d4-beam-seed79.json`](records/close-loss-c5-baseline-base-d4-beam-seed79.json): close loss at difficulty 4: corruption in round 3, 11 adversary piece(s) over the level-0 bar, pool 0 of 4
- [`records/close-loss-c3-baseline-base-d4-random-seed79.json`](records/close-loss-c3-baseline-base-d4-random-seed79.json): close loss at difficulty 4: corruption in round 3, 15 adversary piece(s) over the level-0 bar, pool 0 of 4
- [`records/close-loss-c5-baseline-base-d4-beam-seed45.json`](records/close-loss-c5-baseline-base-d4-beam-seed45.json): close loss at difficulty 4: corruption in round 3, 16 adversary piece(s) over the level-0 bar, pool 0 of 4
- [`records/close-loss-c4-baseline-base-d4-greedy-seed79.json`](records/close-loss-c4-baseline-base-d4-greedy-seed79.json): close loss at difficulty 4: corruption in round 3, 16 adversary piece(s) over the level-0 bar, pool 0 of 4

**early-lock-in**

- [`records/early-lock-in-c2-baseline-base-d0-beam-seed2.json`](records/early-lock-in-c2-baseline-base-d0-beam-seed2.json): outcome (loss) decided from round 1 of 5: win probability by round 0.00 0.00 0.00 0.00 0.00
- [`records/early-lock-in-c5-baseline-base-d4-beam-seed3.json`](records/early-lock-in-c5-baseline-base-d4-beam-seed3.json): outcome (loss) decided from round 1 of 5: win probability by round 0.00 0.00 0.00 0.00 0.00
- [`records/early-lock-in-c2-baseline-base-d0-beam-seed6.json`](records/early-lock-in-c2-baseline-base-d0-beam-seed6.json): outcome (loss) decided from round 1 of 5: win probability by round 0.00 0.00 0.00 0.00 0.00
- [`records/early-lock-in-c1-baseline-base-d0-greedy-seed2.json`](records/early-lock-in-c1-baseline-base-d0-greedy-seed2.json): outcome (loss) decided from round 1 of 5: win probability by round 0.00 0.00 0.00 0.00 0.00

**surprising-win**: none matched


