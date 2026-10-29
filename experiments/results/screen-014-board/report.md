# Cycle report: screen-014-board

014 quick screen of the new board model (rules r6): first new corruption, adversary ambassadors on the board by round, cities corrupted at the end, destructions per war round, Unity and Discord tips, win and loss rates and causes, standard-action use. 2 and 3 countries, d0 and d4, greedy and engine. No tuning: content is this prompt's own.

128 games (8 cells × 16 seeds, 0..=15) on 12 threads in 16.4 s. Engine 0.1.0, rules r6, content 1cd24a1f5d1941cc.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates greedy 100%, engine 100%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) greedy 12.00, engine 12.00; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c2): win rates greedy 0%, engine 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) greedy 3.44, engine 3.19; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, c3): win rates greedy 100%, engine 100%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) greedy 12.00, engine 12.00; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates greedy 6%, engine 6%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) greedy 4.06, engine 3.69; **the ladder is not ordered on survival**.
- Skill gap from difficulty 0 to 4: +0 → +0 pts win rate, +0.00 → -0.31 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 7.3 rounds at difficulty 0, 3.2 at 4 (target 8–10).
- Games ended by: purge 66, supply 62.
- Adversary operations: 16.7 per round on average across cells, at most 48 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 67% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 2.47 gifts per game; 45% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 100% wins) plays 4.19 buildings and 1.50 tier-2 cards per game (of 37.1 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (c2): the top agent there (engine, 0% wins) plays 1.31 buildings and 0.06 tier-2 cards per game (of 9.5 cards); – of its wins had 2+ buildings played (– with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 0% vs greedy 0% over 16 seeds (only engine 0, only greedy 0), +0.0 ±0.0 pts, sign test p 1.000: no better than greedy (level).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±0.0.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +6.2 ±6.2.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| c2 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 100.0 ±0.0 (12.00) | 100.0 ±0.0 (12.00) | +0.0 pts (+0.00 survival) |

| c2 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.44) | 0.0 ±0.0 (3.19) | +0.0 pts (-0.25 survival) |

| c3 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 100.0 ±0.0 (12.00) | 100.0 ±0.0 (12.00) | +0.0 pts (+0.00 survival) |

| c3 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 6.2 ±6.1 (4.06) | 6.2 ±6.1 (3.69) | +0.0 pts (-0.38 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 engine | 16 | 100.0 | 100.0 | 16 / 0 / 0 / 0 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c2 d4 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d0 engine | 16 | 100.0 | 100.0 | 16 / 0 / 0 / 0 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d4 engine | 16 | 6.2 | 6.2 | 0 / 1 / 1 / 14 | +0.0 ±9.1 | 1.000 | no better than greedy (level) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | greedy | 100.0 → 100.0 | +0.0 ±0.0 | 1.000 | 6.00 → 5.06 |
| c3 | 0 | engine | 100.0 → 100.0 | +0.0 ±0.0 | 1.000 | 7.31 → 6.75 |
| c3 | 4 | greedy | 0.0 → 6.2 | +6.2 ±6.2 | 1.000 | 3.44 → 3.81 |
| c3 | 4 | engine | 0.0 → 6.2 | +6.2 ±6.2 | 1.000 | 3.19 → 3.44 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 greedy | 6.00 (1.94, 4..10) | purge 16 | 4.00 / 17.94 | – / – | 17.94 | 0.00 / 1.25 |
| c2 d0 engine | 7.31 (1.69, 4..10) | purge 16 | 2.69 / 17.38 | – / – | 17.38 | 0.00 / 2.81 |
| c2 d4 greedy | 3.44 (0.61, 3..5) | supply 16 | – / – | 23.3 / 0.00 | 0.00 | 2.38 / 0.06 |
| c2 d4 engine | 3.19 (0.63, 2..4) | supply 16 | – / – | 22.9 / 0.00 | 0.00 | 2.50 / 0.12 |
| c3 d0 greedy | 5.06 (1.39, 3..8) | purge 16 | 4.94 / 29.25 | – / – | 29.25 | 0.00 / 0.38 |
| c3 d0 engine | 6.75 (1.39, 4..10) | purge 16 | 3.25 / 25.56 | – / – | 25.56 | 0.00 / 3.38 |
| c3 d4 greedy | 3.81 (1.01, 3..7) | purge 1, supply 15 | 2.00 / 24.00 | 33.4 / 0.00 | 1.50 | 2.94 / 0.06 |
| c3 d4 engine | 3.44 (1.06, 2..7) | purge 1, supply 15 | 2.00 / 24.00 | 32.6 / 0.00 | 1.50 | 3.12 / 0.38 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 greedy | 207.9 | 21.88 | 3% | – | – | 10.33 (23) | 3.62 | 9.00 | 0.00 | 5.81 / 18.62 (31%) |
| c2 d0 engine | 290.6 | 19.96 | 3% | – | – | 9.68 (22) | 4.00 | 14.62 | 0.00 | 13.56 / 23.94 (57%) |
| c2 d4 greedy | 93.2 | 12.56 | 3% | – | – | 17.18 (46) | 0.56 | 2.75 | 0.00 | 1.31 / 9.50 (14%) |
| c2 d4 engine | 96.2 | 13.19 | 3% | – | – | 17.73 (36) | 0.19 | 3.12 | 0.00 | 2.38 / 9.25 (26%) |
| c3 d0 greedy | 255.8 | 24.71 | 2% | – | – | 15.07 (28) | 2.56 | 12.38 | 0.00 | 7.50 / 20.50 (37%) |
| c3 d0 engine | 387.6 | 24.59 | 2% | – | – | 14.90 (31) | 5.06 | 22.12 | 0.00 | 19.06 / 28.31 (67%) |
| c3 d4 greedy | 163.6 | 17.01 | 2% | – | – | 25.41 (47) | 1.44 | 3.56 | 0.00 | 2.00 / 12.81 (16%) |
| c3 d4 engine | 152.0 | 17.11 | 2% | – | – | 23.40 (48) | 0.62 | 4.81 | 0.00 | 3.50 / 11.06 (32%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | – (no variance) | – (no variance) |
| c2 d4 | – (no variance) | 77% / 4% / 19% |
| c3 d0 | – (no variance) | – (no variance) |
| c3 d4 | 47% / 0% / 53% | 56% / 1% / 43% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 greedy | 6 of 16 | 0.62 | 6 of 16 | 0.62 |
| c2 d0 engine | 16 of 16 | 0.72 | 16 of 16 | 0.72 |
| c2 d4 greedy | 7 of 16 | 0.70 | 0 of 0 | – |
| c2 d4 engine | 8 of 16 | 0.78 | 0 of 0 | – |
| c3 d0 greedy | 11 of 16 | 0.52 | 11 of 16 | 0.52 |
| c3 d0 engine | 16 of 16 | 0.63 | 16 of 16 | 0.63 |
| c3 d4 greedy | 9 of 16 | 0.54 | 1 of 1 | – |
| c3 d4 engine | 14 of 16 | 0.75 | 1 of 1 | – |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| greedy | 1.69 / 0.81 / 17.6, 69% (69%) | 1.81 / 0.69 / 20.6, 81% (81%) |
| engine | 4.19 / 1.50 / 37.1, 100% (100%) | 5.06 / 3.69 / 46.1, 100% (100%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| greedy | 1.25 / 0.06 / 6.4, – | 1.75 / 0.19 / 11.2, 100% (100%) |
| engine | 1.31 / 0.06 / 9.5, – | 1.88 / 0.06 / 14.4, 100% (100%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.0 (69) / 100% / 0% | 1.0 (60) / 100% / 2% |
| civic_shelter | 2.2 (58) / 100% / 0% | 2.1 (57) / 100% / 0% |
| contract_bid | 3.9 (58) / 100% / 0% | 3.3 (193) / 94% / 88% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 58% |
| data_mining | 3.9 (51) / 100% / 0% | 3.2 (140) / 99% / 81% |
| emergency_fab | 2.7 (50) / 100% / 0% | 2.7 (48) / 100% / 0% |
| evacuate | 2.7 (60) / 100% / 20% | 2.6 (71) / 100% / 32% |
| fab_line | 1.7 (52) / 100% / 0% | 1.7 (50) / 100% / 33% |
| firewall_node | 2.0 (54) / 100% / 0% | 1.9 (54) / 100% / 0% |
| fork_process | 2.5 (71) / 100% / 36% | 2.2 (124) / 95% / 70% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.8 (59) / 100% / 0% | 3.3 (99) / 97% / 61% |
| grid_battery | 1.8 (55) / 100% / 0% | 1.7 (54) / 100% / 2% |
| jam_signals | 3.3 (56) / 100% / 0% | 2.7 (168) / 96% / 89% |
| liquidate_reserves | 3.6 (64) / 100% / 0% | 3.6 (69) / 99% / 10% |
| logistics_ai | 1.5 (61) / 100% / 0% | 1.5 (52) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.5 (56) / 100% / 0% | 2.6 (53) / 100% / 48% |
| opposition_network | 1.5 (62) / 100% / 0% | 1.4 (58) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (60) / 100% / 0% | 1.0 (56) / 100% / 0% |
| power_share | 3.5 (59) / 100% / 0% | 3.5 (59) / 98% / 0% |
| precision_strike | 2.0 (149) / 100% / 97% | 1.9 (189) / 98% / 97% |
| purge_protocol | – (0) / – / 68% | – (0) / – / 78% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.6 (124) / 100% / 77% | 2.4 (167) / 97% / 84% |
| rebuild | 1.6 (134) / 100% / 87% | 1.6 (156) / 97% / 89% |
| recon_sweep | 3.3 (54) / 100% / 0% | 3.4 (56) / 100% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.4 (60) / 100% / 97% | 1.4 (57) / 100% / 81% |
| scavenge | 3.7 (62) / 100% / 0% | 3.4 (90) / 97% / 33% |
| server_farm | 1.5 (53) / 100% / 82% | 1.5 (52) / 100% / 67% |
| shore_up | 2.9 (55) / 100% / 0% | 3.1 (54) / 100% / 0% |
| solar_field | 2.6 (54) / 100% / 0% | 2.8 (50) / 100% / 84% |
| strike_team | – (0) / – / 0.92/game | – (0) / – / 0.97/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.8 (129) / 100% / 89% | 1.5 (189) / 99% / 90% |
| trade_office | 1.3 (63) / 100% / 0% | 1.3 (57) / 100% / 0% |
| turnkey_factory | – (0) / – / 0.00/game | – (0) / – / 0.31/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (52) / 100% / 0% | 1.9 (53) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 greedy | 713.0 | 3.429 | 0.0 | 731.1 |
| c2 d0 engine | 1417.2 | 4.877 | 0.0 | 1444.2 |
| c2 d4 greedy | 160.1 | 1.717 | 0.0 | 168.1 |
| c2 d4 engine | 244.0 | 2.535 | 0.0 | 251.8 |
| c3 d0 greedy | 1979.2 | 7.737 | 0.0 | 2006.7 |
| c3 d0 engine | 3791.6 | 9.783 | 0.0 | 3833.4 |
| c3 d4 greedy | 797.1 | 4.871 | 0.0 | 813.4 |
| c3 d4 engine | 996.1 | 6.553 | 0.0 | 1010.6 |

Where the games' time went (summed over workers): agents 98.4%, probes 0.0%, apply 0.5%, legal 0.6%, replay check 0.5%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-c2-d4-engine-seed9.json`](records/close-loss-c3-baseline-c2-d4-engine-seed9.json): close loss at difficulty 4: supply in round 2, 20 adversary piece(s) over the level-0 bar, supply 0 of 16, 2 cities corrupted
- [`records/close-loss-c3-baseline-c2-d4-engine-seed12.json`](records/close-loss-c3-baseline-c2-d4-engine-seed12.json): close loss at difficulty 4: supply in round 4, 21 adversary piece(s) over the level-0 bar, supply 0 of 16, 3 cities corrupted

**early-lock-in**: none matched


**surprising-win**: none matched


