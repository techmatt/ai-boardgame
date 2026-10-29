# Cycle report: screen-016-presence

016 quick screen of rules r7 (presence, the adversary ladder with nodes, factories and hardware, the production/war clock, the Unity-eased purge): win rates and length, the strategy-health gate, and per game presence placed, node peaks per board, factories built, destroyed and owned per side, hardware made and spent, adversary compute made. 2 and 3 countries, d0 and d4, greedy and engine. No tuning.

128 games (8 cells × 16 seeds, 0..=15) on 12 threads in 20.2 s. Engine 0.1.0, rules r7, content 628efb7adc22601f.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- **No agent won a game** (128 games). Targets: at difficulty 0 moderately skilled play essentially always wins; at the top difficulty the strongest agent wins narrowly. The ladder can only be read off survival (rounds, margins) until the rules are tuned.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) greedy 5.12, engine 6.56; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) greedy 2.62, engine 2.69; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) greedy 7.06, engine 7.69; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) greedy 2.69, engine 2.69; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +0 → +0 pts win rate, +1.03 → +0.03 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 6.6 rounds at difficulty 0, 2.7 at 4 (target 8–10).
- Games ended by: deck_empty 5, supply 123.
- Adversary operations: 19.8 per round on average across cells, at most 62 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 81% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 5.11 gifts per game; 43% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 0% wins) plays 3.88 buildings and 1.00 tier-2 cards per game (of 28.5 cards); – of its wins had 2+ buildings played (– with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (c2): the top agent there (engine, 0% wins) plays 1.75 buildings and 0.06 tier-2 cards per game (of 7.0 cards); – of its wins had 2+ buildings played (– with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 0% vs greedy 0% over 16 seeds (only engine 0, only greedy 0), +0.0 ±0.0 pts, sign test p 1.000: no better than greedy (level).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±0.0.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±0.0.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| c2 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.12) | 0.0 ±0.0 (6.56) | +0.0 pts (+1.44 survival) |

| c2 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.62) | 0.0 ±0.0 (2.69) | +0.0 pts (+0.06 survival) |

| c3 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.06) | 0.0 ±0.0 (7.69) | +0.0 pts (+0.62 survival) |

| c3 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.69) | 0.0 ±0.0 (2.69) | +0.0 pts (+0.00 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c2 d4 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d0 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d4 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | greedy | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.12 → 7.06 |
| c3 | 0 | engine | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.56 → 7.69 |
| c3 | 4 | greedy | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 2.62 → 2.69 |
| c3 | 4 | engine | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 2.69 → 2.69 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 greedy | 5.12 (1.93, 2..9) | supply 16 | – / – | 31.8 / 0.25 | 0.00 | 2.12 / 0.19 |
| c2 d0 engine | 6.56 (3.00, 2..11) | deck_empty 3, supply 13 | – / – | 29.2 / 0.44 | 3.00 | 2.19 / 0.81 |
| c2 d4 greedy | 2.62 (0.70, 2..4) | supply 16 | – / – | 27.9 / 0.00 | 0.00 | 2.12 / 0.00 |
| c2 d4 engine | 2.69 (0.77, 2..4) | supply 16 | – / – | 27.7 / 0.00 | 0.00 | 2.12 / 0.00 |
| c3 d0 greedy | 7.06 (1.82, 5..10) | supply 16 | – / – | 57.3 / 0.19 | 0.00 | 3.62 / 0.19 |
| c3 d0 engine | 7.69 (2.14, 4..11) | deck_empty 2, supply 14 | – / – | 49.8 / 0.44 | 3.00 | 3.44 / 1.38 |
| c3 d4 greedy | 2.69 (0.77, 2..5) | supply 16 | – / – | 39.6 / 0.00 | 0.00 | 3.12 / 0.06 |
| c3 d4 engine | 2.69 (0.77, 2..5) | supply 16 | – / – | 39.1 / 0.00 | 0.00 | 3.06 / 0.06 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 greedy | 167.1 | 17.55 | 3% | – | – | 16.27 (45) | 3.88 | 6.19 | 0.00 | 3.25 / 14.25 (23%) |
| c2 d0 engine | 254.1 | 21.42 | 3% | – | – | 14.57 (36) | 7.62 | 11.12 | 0.00 | 10.25 / 19.19 (53%) |
| c2 d4 greedy | 66.3 | 12.52 | 3% | – | – | 17.38 (39) | 0.31 | 2.19 | 0.00 | 1.50 / 6.56 (23%) |
| c2 d4 engine | 70.7 | 12.93 | 3% | – | – | 16.98 (33) | 0.25 | 2.81 | 0.00 | 1.81 / 6.62 (27%) |
| c3 d0 greedy | 318.0 | 25.26 | 2% | – | – | 24.72 (62) | 10.81 | 10.81 | 0.00 | 6.06 / 26.50 (23%) |
| c3 d0 engine | 404.9 | 25.82 | 2% | – | – | 23.09 (49) | 12.19 | 17.44 | 0.00 | 15.62 / 29.00 (54%) |
| c3 d4 greedy | 90.5 | 15.53 | 2% | – | – | 22.63 (46) | 0.50 | 2.75 | 0.00 | 1.81 / 7.44 (24%) |
| c3 d4 engine | 95.2 | 16.29 | 2% | – | – | 22.37 (44) | 0.38 | 3.19 | 0.00 | 2.62 / 7.38 (36%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | – (no variance) | 62% / 8% / 30% |
| c2 d4 | – (no variance) | 91% / 0% / 9% |
| c3 d0 | – (no variance) | 76% / 2% / 22% |
| c3 d4 | – (no variance) | 95% / 0% / 5% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 greedy | 7 of 16 | 0.46 | 0 of 0 | – |
| c2 d0 engine | 13 of 16 | 0.63 | 0 of 0 | – |
| c2 d4 greedy | 8 of 16 | 0.68 | 0 of 0 | – |
| c2 d4 engine | 11 of 16 | 0.75 | 0 of 0 | – |
| c3 d0 greedy | 9 of 16 | 0.37 | 0 of 0 | – |
| c3 d0 engine | 16 of 16 | 0.54 | 0 of 0 | – |
| c3 d4 greedy | 11 of 16 | 0.70 | 0 of 0 | – |
| c3 d4 engine | 13 of 16 | 0.74 | 0 of 0 | – |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| greedy | 2.19 / 0.00 / 10.7, – | 2.62 / 0.00 / 20.7, – |
| engine | 3.88 / 1.00 / 28.5, – | 5.06 / 1.75 / 42.7, – |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| greedy | 1.38 / 0.00 / 4.9, – | 1.69 / 0.00 / 5.9, – |
| engine | 1.75 / 0.06 / 7.0, – | 2.25 / 0.12 / 9.2, – |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.0 (60) / 100% / 0% | 1.0 (56) / 100% / 2% |
| civic_shelter | 2.1 (53) / 100% / 0% | 2.1 (52) / 100% / 0% |
| contract_bid | 3.9 (49) / 100% / 0% | 3.1 (161) / 98% / 99% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 64% |
| data_mining | 3.8 (44) / 100% / 0% | 3.1 (104) / 99% / 74% |
| emergency_fab | 2.8 (41) / 100% / 0% | 2.7 (42) / 100% / 5% |
| evacuate | 2.5 (54) / 100% / 30% | 2.6 (71) / 100% / 46% |
| fab_line | 1.8 (47) / 100% / 0% | 1.7 (47) / 100% / 50% |
| firewall_node | 1.9 (41) / 100% / 0% | 1.9 (45) / 100% / 0% |
| fork_process | 2.6 (55) / 100% / 33% | 2.6 (59) / 100% / 39% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.8 (51) / 100% / 0% | 3.2 (94) / 99% / 65% |
| grid_battery | 1.6 (49) / 100% / 0% | 1.6 (50) / 100% / 2% |
| jam_signals | 3.2 (45) / 100% / 0% | 2.4 (135) / 98% / 96% |
| liquidate_reserves | 3.7 (49) / 100% / 0% | 3.8 (51) / 100% / 2% |
| logistics_ai | 1.5 (48) / 100% / 0% | 1.6 (42) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.7 (46) / 100% / 0% | 2.7 (45) / 100% / 42% |
| opposition_network | 1.4 (54) / 100% / 0% | 1.5 (52) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (51) / 100% / 0% | 1.0 (49) / 100% / 0% |
| power_share | 3.4 (54) / 100% / 0% | 3.5 (54) / 100% / 0% |
| precision_strike | 2.0 (113) / 100% / 85% | 1.9 (140) / 100% / 88% |
| purge_protocol | – (0) / – / 0% | – (0) / – / 83% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.3 (112) / 100% / 81% | 2.4 (135) / 99% / 89% |
| rebuild | 1.7 (96) / 100% / 76% | 1.6 (108) / 99% / 78% |
| recon_sweep | 3.3 (42) / 100% / 0% | 3.2 (46) / 100% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.4 (49) / 100% / 88% | 1.4 (51) / 100% / 80% |
| scavenge | 3.7 (54) / 100% / 0% | 3.3 (79) / 100% / 39% |
| server_farm | 1.6 (45) / 100% / 64% | 1.6 (45) / 100% / 50% |
| shore_up | 3.0 (48) / 100% / 0% | 3.1 (48) / 100% / 2% |
| solar_field | 2.8 (50) / 100% / 0% | 2.8 (50) / 100% / 88% |
| strike_team | – (0) / – / 0.78/game | – (0) / – / 0.83/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (129) / 100% / 100% | 1.6 (155) / 99% / 99% |
| trade_office | 1.3 (51) / 100% / 0% | 1.3 (51) / 100% / 0% |
| turnkey_factory | – (0) / – / 0.84/game | – (0) / – / 0.83/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (46) / 100% / 0% | 1.9 (46) / 100% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 greedy | 511.7 | 3.063 | 0.0 | 528.1 |
| c2 d0 engine | 1400.0 | 5.509 | 0.0 | 1426.1 |
| c2 d4 greedy | 125.1 | 1.886 | 0.0 | 130.8 |
| c2 d4 engine | 164.8 | 2.332 | 0.0 | 170.5 |
| c3 d0 greedy | 2880.4 | 9.058 | 0.0 | 2931.0 |
| c3 d0 engine | 4956.4 | 12.242 | 0.0 | 5015.1 |
| c3 d4 greedy | 415.0 | 4.585 | 0.0 | 424.6 |
| c3 d4 engine | 536.1 | 5.629 | 0.0 | 545.9 |

Where the games' time went (summed over workers): agents 98.4%, probes 0.0%, apply 0.6%, legal 0.5%, replay check 0.5%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c2-baseline-c2-d4-greedy-seed0.json`](records/close-loss-c2-baseline-c2-d4-greedy-seed0.json): close loss at difficulty 4: supply in round 2, 23 adversary piece(s) over the level-0 bar, supply 0 of 16, 2 cities corrupted
- [`records/close-loss-c3-baseline-c2-d4-engine-seed0.json`](records/close-loss-c3-baseline-c2-d4-engine-seed0.json): close loss at difficulty 4: supply in round 2, 23 adversary piece(s) over the level-0 bar, supply 0 of 16, 2 cities corrupted

**early-lock-in**: none matched


**surprising-win**: none matched


