# Cycle report: tune-before

007 tuning run, before: ruleset v0 as handed over (configs/v0_untuned.json), the whole ladder at every difficulty.

1000 games (20 cells × 50 seeds, 0..=49) on 12 threads in 592.4 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/v0_untuned.json
- Variants: base
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy, beam, rollout
- Win probability: off
- Decision closeness: every 4 real choice(s) probed by 1-ply greedy scoring; close = best two within 2 points
- Replay check: every game replays from seed + actions

## Findings

- **No agent won a game** (1000 games). Targets: at difficulty 0 moderately skilled play essentially always wins; at the top difficulty the strongest agent wins narrowly. The ladder can only be read off survival (rounds, margins) until the rules are tuned.
- Difficulty 0: survival (rounds; wins count extra) random 3.10, greedy 3.84, beam 4.00, rollout 4.00; the ladder is ordered.
- Difficulty 1: survival (rounds; wins count extra) random 3.04, greedy 3.78, beam 3.98, rollout 3.94; **the ladder is not ordered on survival**.
- Difficulty 2: survival (rounds; wins count extra) random 2.98, greedy 3.64, beam 3.76, rollout 3.74; **the ladder is not ordered on survival**.
- Difficulty 3: survival (rounds; wins count extra) random 2.98, greedy 3.64, beam 3.76, rollout 3.74; **the ladder is not ordered on survival**.
- Difficulty 4: survival (rounds; wins count extra) random 2.78, greedy 3.04, beam 3.12, rollout 3.10; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: +0 → +0 pts win rate, +0.90 → +0.32 survival (target: widens with difficulty).
- Game length for the strongest agent (rollout): 4.0 rounds at difficulty 0, 3.1 at 4 (target 8–10).
- Games ended by: corruption 1000.
- Adversary operations: 13.1 per round on average across cells, at most 33 in one round (target: low).
- Decisions (strongest agent): 7% forced, 73% of real choices close (best two within 2 points).
- Cards (rollout, offered at least 10 times): kept under 10% of the time: counter_intelligence; over 90%: assembly_cluster, megastructure, overclock_rig, reconstruction.
- Luck share of survival (seed's share of the variance): 70% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 2.34 gifts per game; 17% of threats countered.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| difficulty 0 | random | greedy | beam | rollout | skill gap |
|---|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.10) | 0.0 ±0.0 (3.84) | 0.0 ±0.0 (4.00) | 0.0 ±0.0 (4.00) | +0.0 pts (+0.90 survival) |

| difficulty 1 | random | greedy | beam | rollout | skill gap |
|---|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.04) | 0.0 ±0.0 (3.78) | 0.0 ±0.0 (3.98) | 0.0 ±0.0 (3.94) | +0.0 pts (+0.90 survival) |

| difficulty 2 | random | greedy | beam | rollout | skill gap |
|---|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.98) | 0.0 ±0.0 (3.64) | 0.0 ±0.0 (3.76) | 0.0 ±0.0 (3.74) | +0.0 pts (+0.76 survival) |

| difficulty 3 | random | greedy | beam | rollout | skill gap |
|---|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.98) | 0.0 ±0.0 (3.64) | 0.0 ±0.0 (3.76) | 0.0 ±0.0 (3.74) | +0.0 pts (+0.76 survival) |

| difficulty 4 | random | greedy | beam | rollout | skill gap |
|---|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.78) | 0.0 ±0.0 (3.04) | 0.0 ±0.0 (3.12) | 0.0 ±0.0 (3.10) | +0.0 pts (+0.32 survival) |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| d0 random | 3.10 (0.88, 2..5) | corruption 50 | – / – | 32.4 / 0.00 | 0.00 | 4.66 |
| d0 greedy | 3.84 (0.61, 3..5) | corruption 50 | – / – | 41.4 / 0.00 | 0.00 | 4.66 |
| d0 beam | 4.00 (0.66, 3..6) | corruption 50 | – / – | 38.2 / 0.00 | 0.00 | 5.30 |
| d0 rollout | 4.00 (0.66, 3..5) | corruption 50 | – / – | 40.3 / 0.00 | 0.00 | 5.12 |
| d1 random | 3.04 (0.82, 2..4) | corruption 50 | – / – | 32.3 / 0.00 | 0.00 | 4.54 |
| d1 greedy | 3.78 (0.70, 2..5) | corruption 50 | – / – | 41.7 / 0.00 | 0.00 | 4.72 |
| d1 beam | 3.98 (0.76, 2..6) | corruption 50 | – / – | 40.1 / 0.00 | 0.00 | 5.22 |
| d1 rollout | 3.94 (0.76, 2..5) | corruption 50 | – / – | 39.0 / 0.00 | 0.00 | 5.20 |
| d2 random | 2.98 (0.79, 2..4) | corruption 50 | – / – | 32.5 / 0.00 | 0.00 | 4.50 |
| d2 greedy | 3.64 (0.82, 2..5) | corruption 50 | – / – | 41.6 / 0.00 | 0.00 | 4.72 |
| d2 beam | 3.76 (0.84, 2..5) | corruption 50 | – / – | 39.0 / 0.00 | 0.00 | 5.50 |
| d2 rollout | 3.74 (0.82, 2..5) | corruption 50 | – / – | 38.9 / 0.00 | 0.00 | 5.36 |
| d3 random | 2.98 (0.79, 2..4) | corruption 50 | – / – | 32.5 / 0.00 | 0.00 | 4.50 |
| d3 greedy | 3.64 (0.82, 2..5) | corruption 50 | – / – | 41.6 / 0.00 | 0.00 | 4.72 |
| d3 beam | 3.76 (0.84, 2..5) | corruption 50 | – / – | 39.0 / 0.00 | 0.00 | 5.50 |
| d3 rollout | 3.74 (0.82, 2..5) | corruption 50 | – / – | 38.9 / 0.00 | 0.00 | 5.36 |
| d4 random | 2.78 (0.64, 2..4) | corruption 50 | – / – | 28.4 / 0.00 | 0.00 | 4.78 |
| d4 greedy | 3.04 (0.82, 2..5) | corruption 50 | – / – | 31.2 / 0.00 | 0.00 | 4.82 |
| d4 beam | 3.12 (0.89, 2..5) | corruption 50 | – / – | 29.0 / 0.00 | 0.00 | 5.56 |
| d4 rollout | 3.10 (0.88, 2..5) | corruption 50 | – / – | 29.7 / 0.00 | 0.00 | 5.50 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 random | 83.1 | 5.92 | 6% | 77% | 1.4 | 12.34 (27) | 3.02 | 4.02 | 1.04 | 1.40 / 6.20 (23%) |
| d0 greedy | 107.6 | 5.89 | 6% | 75% | 1.4 | 13.85 (32) | 0.00 | 5.58 | 0.00 | 2.92 / 7.68 (38%) |
| d0 beam | 114.8 | 6.06 | 7% | 73% | 1.7 | 13.38 (30) | 2.16 | 5.66 | 0.02 | 2.98 / 8.00 (37%) |
| d0 rollout | 114.8 | 6.08 | 7% | 74% | 1.6 | 13.52 (30) | 2.20 | 5.62 | 0.04 | 3.00 / 8.00 (38%) |
| d1 random | 80.8 | 5.97 | 6% | 78% | 1.3 | 12.50 (27) | 2.84 | 4.24 | 1.22 | 0.76 / 6.08 (12%) |
| d1 greedy | 106.1 | 5.89 | 6% | 74% | 1.4 | 13.98 (32) | 0.00 | 5.52 | 0.00 | 1.22 / 7.56 (16%) |
| d1 beam | 114.0 | 6.04 | 7% | 74% | 1.5 | 13.49 (31) | 2.38 | 5.50 | 0.04 | 1.52 / 7.96 (19%) |
| d1 rollout | 112.8 | 6.00 | 7% | 74% | 1.5 | 13.39 (31) | 2.46 | 5.50 | 0.02 | 1.52 / 7.88 (19%) |
| d2 random | 79.6 | 5.88 | 6% | 78% | 1.3 | 12.74 (28) | 2.50 | 3.40 | 1.10 | 0.52 / 5.96 (9%) |
| d2 greedy | 101.5 | 5.77 | 6% | 75% | 1.5 | 14.07 (33) | 0.00 | 3.82 | 0.00 | 0.68 / 7.28 (9%) |
| d2 beam | 110.0 | 5.89 | 7% | 73% | 1.6 | 13.73 (33) | 2.30 | 4.00 | 0.02 | 0.88 / 7.52 (12%) |
| d2 rollout | 108.5 | 5.87 | 7% | 73% | 1.6 | 13.70 (33) | 2.58 | 3.94 | 0.00 | 0.86 / 7.48 (11%) |
| d3 random | 79.6 | 5.88 | 6% | 78% | 1.3 | 12.74 (28) | 2.50 | 3.40 | 1.10 | 0.52 / 5.96 (9%) |
| d3 greedy | 101.5 | 5.77 | 6% | 75% | 1.5 | 14.07 (33) | 0.00 | 3.82 | 0.00 | 0.68 / 7.28 (9%) |
| d3 beam | 110.0 | 5.89 | 7% | 73% | 1.6 | 13.73 (33) | 2.30 | 4.00 | 0.02 | 0.88 / 7.52 (12%) |
| d3 rollout | 108.5 | 5.87 | 7% | 73% | 1.6 | 13.70 (33) | 2.58 | 3.94 | 0.00 | 0.86 / 7.48 (11%) |
| d4 random | 73.7 | 5.74 | 6% | 78% | 1.3 | 11.12 (27) | 2.58 | 2.36 | 0.88 | 0.14 / 5.56 (3%) |
| d4 greedy | 83.7 | 5.63 | 6% | 73% | 1.5 | 11.92 (29) | 0.00 | 2.12 | 0.00 | 0.32 / 6.08 (5%) |
| d4 beam | 90.6 | 5.68 | 7% | 71% | 1.6 | 11.51 (28) | 2.14 | 2.20 | 0.00 | 0.34 / 6.24 (5%) |
| d4 rollout | 88.7 | 5.72 | 7% | 72% | 1.6 | 11.63 (30) | 1.88 | 2.18 | 0.02 | 0.34 / 6.20 (5%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| d0 | – (no variance) | 55% / 22% / 23% |
| d1 | – (no variance) | 59% / 20% / 21% |
| d2 | – (no variance) | 74% / 13% / 13% |
| d3 | – (no variance) | 74% / 13% / 13% |
| d4 | – (no variance) | 88% / 3% / 9% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| d0 random | 37 of 50 | 0.76 | 0 of 0 | – |
| d0 greedy | 2 of 50 | 0.04 | 0 of 0 | – |
| d0 beam | 16 of 50 | 0.53 | 0 of 0 | – |
| d0 rollout | 14 of 50 | 0.55 | 0 of 0 | – |
| d1 random | 37 of 50 | 0.76 | 0 of 0 | – |
| d1 greedy | 4 of 50 | 0.24 | 0 of 0 | – |
| d1 beam | 17 of 50 | 0.57 | 0 of 0 | – |
| d1 rollout | 15 of 50 | 0.51 | 0 of 0 | – |
| d2 random | 40 of 50 | 0.77 | 0 of 0 | – |
| d2 greedy | 4 of 50 | 0.20 | 0 of 0 | – |
| d2 beam | 15 of 50 | 0.57 | 0 of 0 | – |
| d2 rollout | 9 of 50 | 0.49 | 0 of 0 | – |
| d3 random | 40 of 50 | 0.77 | 0 of 0 | – |
| d3 greedy | 4 of 50 | 0.20 | 0 of 0 | – |
| d3 beam | 15 of 50 | 0.57 | 0 of 0 | – |
| d3 rollout | 9 of 50 | 0.49 | 0 of 0 | – |
| d4 random | 35 of 50 | 0.75 | 0 of 0 | – |
| d4 greedy | 4 of 50 | 0.27 | 0 of 0 | – |
| d4 beam | 8 of 50 | 0.53 | 0 of 0 | – |
| d4 rollout | 13 of 50 | 0.57 | 0 of 0 | – |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | beam: pick (offered) / keep / play | rollout: pick (offered) / keep / play |
|---|---|---|---|---|
| antimatter_refinery | 2.1 (27) / 63% / 0% | – (0) / – / 0% | 2.0 (53) / 89% / 0% | 2.0 (43) / 81% / 0% |
| assembly_cluster | 2.5 (199) / 40% / 25% | 1.1 (226) / 94% / 0% | 1.4 (228) / 94% / 0% | 1.3 (226) / 95% / 0% |
| civic_shelter | 2.6 (196) / 42% / 37% | 2.1 (255) / 64% / 0% | 2.1 (246) / 58% / 3% | 2.1 (239) / 63% / 2% |
| contract_bid | 2.3 (191) / 35% / 74% | 4.0 (262) / 19% / 0% | 4.0 (241) / 21% / 75% | 4.0 (241) / 23% / 63% |
| counter_intelligence | 2.3 (40) / 38% / 0% | – (0) / – / 0% | 3.9 (68) / 6% / 0% | 4.0 (57) / 5% / 0% |
| data_mining | 2.5 (175) / 43% / 55% | 3.5 (248) / 38% / 0% | 3.5 (224) / 36% / 49% | 3.5 (236) / 36% / 38% |
| emergency_fab | 2.6 (173) / 42% / 3% | 2.6 (229) / 58% / 0% | 2.5 (231) / 50% / 0% | 2.5 (226) / 51% / 0% |
| evacuate | 2.5 (199) / 32% / 41% | 3.0 (240) / 53% / 4% | 3.0 (249) / 42% / 28% | 3.0 (234) / 48% / 30% |
| fab_line | 2.6 (172) / 41% / 31% | 1.8 (218) / 81% / 0% | 1.9 (218) / 76% / 0% | 1.8 (219) / 78% / 0% |
| firewall_node | 2.6 (176) / 33% / 27% | 1.9 (224) / 74% / 0% | 1.9 (221) / 71% / 0% | 2.0 (220) / 72% / 0% |
| fork_process | 2.6 (178) / 31% / 34% | 2.7 (244) / 50% / 47% | 2.7 (232) / 40% / 45% | 2.7 (238) / 41% / 43% |
| fusion_plant | 2.5 (27) / 11% / 0% | – (0) / – / 0% | 1.3 (51) / 92% / 0% | 1.2 (40) / 88% / 0% |
| global_mandate | 2.3 (39) / 23% / 0% | – (0) / – / 0% | 3.5 (48) / 42% / 0% | 3.5 (37) / 35% / 0% |
| grant | 2.5 (195) / 44% / 49% | 3.6 (250) / 42% / 0% | 3.7 (225) / 41% / 25% | 3.7 (229) / 41% / 26% |
| grid_battery | 2.5 (205) / 33% / 25% | 1.8 (246) / 61% / 0% | 1.8 (227) / 62% / 13% | 1.8 (242) / 67% / 7% |
| jam_signals | 2.4 (174) / 46% / 62% | 3.3 (245) / 42% / 0% | 3.2 (225) / 35% / 19% | 3.2 (232) / 31% / 22% |
| liquidate_reserves | 2.2 (195) / 41% / 57% | 3.6 (233) / 42% / 0% | 3.6 (232) / 38% / 11% | 3.7 (228) / 41% / 7% |
| logistics_ai | 2.5 (203) / 43% / 19% | 1.3 (235) / 86% / 0% | 1.4 (230) / 89% / 0% | 1.3 (232) / 81% / 0% |
| megastructure | 2.2 (15) / 13% / 0% | – (0) / – / 0% | 1.1 (38) / 100% / 0% | 1.0 (35) / 100% / 0% |
| microgrid | 2.7 (187) / 34% / 23% | 2.4 (242) / 52% / 0% | 2.3 (238) / 46% / 14% | 2.4 (244) / 46% / 21% |
| modular_expansion | – (0) / – / 0.56/game | – (0) / – / 0.00/game | – (0) / – / 0.02/game | – (0) / – / 0.01/game |
| opposition_network | 2.1 (199) / 50% / 38% | 1.6 (233) / 79% / 0% | 1.6 (226) / 84% / 0% | 1.6 (233) / 80% / 0% |
| orbital_lance | 2.8 (22) / 41% / 0% | – (0) / – / 0% | 1.9 (40) / 88% / 0% | 1.7 (40) / 90% / 0% |
| overclock_rig | 2.4 (175) / 39% / 16% | 1.0 (223) / 93% / 0% | 1.3 (219) / 90% / 0% | 1.3 (214) / 94% / 0% |
| point_defense | – (0) / – / 0.24/game | – (0) / – / 0.00/game | – (0) / – / 0.02/game | – (0) / – / 0.00/game |
| power_share | 2.6 (191) / 34% / 48% | 3.6 (250) / 34% / 0% | 3.6 (238) / 35% / 13% | 3.6 (236) / 36% / 11% |
| precision_strike | 2.7 (162) / 46% / 39% | 2.6 (243) / 49% / 89% | 2.5 (224) / 52% / 92% | 2.6 (221) / 49% / 85% |
| purge_protocol | 2.7 (18) / 50% / 0% | – (0) / – / 0% | 2.9 (66) / 76% / 1% | 3.1 (64) / 72% / 0% |
| quantum_core | 2.3 (37) / 22% / 0% | – (0) / – / 0% | 1.8 (44) / 75% / 0% | 1.8 (40) / 72% / 0% |
| rally | 2.7 (208) / 38% / 46% | 2.9 (272) / 46% / 91% | 2.9 (262) / 42% / 90% | 2.9 (262) / 45% / 93% |
| rebuild | 2.7 (204) / 30% / 5% | 2.2 (242) / 57% / 0% | 2.2 (227) / 48% / 17% | 2.2 (230) / 47% / 18% |
| recon_sweep | 2.1 (162) / 51% / 65% | 3.5 (233) / 37% / 0% | 3.5 (221) / 38% / 27% | 3.5 (218) / 39% / 28% |
| reconstruction | 3.5 (26) / 38% / 0% | – (0) / – / 0% | 2.4 (35) / 86% / 0% | 2.3 (33) / 97% / 0% |
| recruitment_hub | 2.3 (199) / 25% / 25% | 1.7 (242) / 77% / 0% | 1.7 (226) / 81% / 0% | 1.6 (230) / 82% / 0% |
| scavenge | 2.5 (200) / 38% / 58% | 3.7 (256) / 42% / 0% | 3.8 (241) / 41% / 9% | 3.8 (243) / 41% / 11% |
| server_farm | 2.6 (166) / 29% / 19% | 1.4 (217) / 85% / 5% | 1.5 (220) / 85% / 43% | 1.4 (221) / 77% / 33% |
| shelter_order | – (0) / – / 0.82/game | – (0) / – / 0.00/game | – (0) / – / 0.63/game | – (0) / – / 0.50/game |
| shore_up | 2.6 (182) / 26% / 35% | 3.0 (238) / 39% / 30% | 3.1 (241) / 32% / 56% | 3.1 (241) / 37% / 42% |
| solar_field | 2.5 (205) / 36% / 33% | 2.6 (227) / 69% / 0% | 2.6 (218) / 63% / 49% | 2.6 (223) / 64% / 47% |
| strike_team | – (0) / – / 0.51/game | – (0) / – / 1.00/game | – (0) / – / 0.98/game | – (0) / – / 1.00/game |
| swarm_override | 2.4 (29) / 55% / 0% | – (0) / – / 0% | 3.0 (43) / 67% / 0% | 3.2 (37) / 65% / 0% |
| sweep | 2.5 (205) / 46% / 25% | 2.5 (245) / 68% / 1% | 2.2 (268) / 58% / 60% | 2.4 (265) / 59% / 49% |
| trade_office | 2.5 (212) / 45% / 22% | 1.3 (234) / 88% / 0% | 1.5 (233) / 90% / 0% | 1.5 (233) / 82% / 0% |
| turnkey_factory | – (0) / – / 0.72/game | – (0) / – / 0.90/game | – (0) / – / 0.89/game | – (0) / – / 0.91/game |
| venture_capital | – (0) / – / 0.84/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 2.7 (184) / 45% / 20% | 1.8 (224) / 72% / 0% | 1.8 (221) / 68% / 2% | 1.8 (220) / 74% / 3% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| d0 random | 0.0 | 0.000 | 64.6 | 71.7 |
| d0 greedy | 1774.8 | 16.491 | 119.0 | 1906.7 |
| d0 beam | 12720.7 | 110.788 | 132.2 | 12865.3 |
| d0 rollout | 13934.9 | 121.405 | 122.8 | 14071.4 |
| d1 random | 0.0 | 0.000 | 67.3 | 74.3 |
| d1 greedy | 1657.8 | 15.625 | 117.2 | 1786.4 |
| d1 beam | 13112.0 | 114.997 | 123.7 | 13247.5 |
| d1 rollout | 14320.0 | 126.950 | 130.8 | 14464.7 |
| d2 random | 0.0 | 0.000 | 62.9 | 69.9 |
| d2 greedy | 1374.9 | 13.540 | 96.3 | 1480.7 |
| d2 beam | 15325.7 | 139.375 | 132.9 | 15469.5 |
| d2 rollout | 13800.5 | 127.241 | 125.3 | 13937.3 |
| d3 random | 0.0 | 0.000 | 63.4 | 70.5 |
| d3 greedy | 1348.7 | 13.283 | 95.2 | 1453.3 |
| d3 beam | 15062.1 | 136.978 | 131.0 | 15204.6 |
| d3 rollout | 13672.1 | 126.057 | 116.4 | 13800.8 |
| d4 random | 0.0 | 0.000 | 59.8 | 67.6 |
| d4 greedy | 1022.9 | 12.218 | 80.9 | 1111.7 |
| d4 beam | 11368.1 | 125.420 | 89.8 | 11467.0 |
| d4 rollout | 9245.4 | 104.208 | 93.5 | 9349.4 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c19-v0_untuned-base-d4-rollout-seed39.json`](records/close-loss-c19-v0_untuned-base-d4-rollout-seed39.json): close loss at difficulty 4: corruption in round 2, 16 adversary piece(s) over the level-0 bar, pool 0 of 4
- [`records/close-loss-c19-v0_untuned-base-d4-rollout-seed2.json`](records/close-loss-c19-v0_untuned-base-d4-rollout-seed2.json): close loss at difficulty 4: corruption in round 3, 18 adversary piece(s) over the level-0 bar, pool 0 of 4

**early-lock-in**: none matched


**surprising-win**: none matched


