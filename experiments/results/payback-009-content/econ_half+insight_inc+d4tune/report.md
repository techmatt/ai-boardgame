# Cycle report: payback-009-c-econ_half+insight_inc+d4tune

009 content lever (separate binary: scripts/payback/patch_content.py econ_half+insight_inc): can difficulty 4 get its bite back by config? adversary.pool_reduction (d4+, 007 set 4) at 5 and 6. Engine agent against greedy, same seeds as payback-009-singles.

384 games (4 cells × 96 seeds, 300000..=300095) on 12 threads in 236.5 s. Engine 0.1.0, rules r3, content 25acdebeadca4472.

- Configs: configs/baseline.json
- Variants: pr5 (adversary.pool_reduction=5); pr6 (adversary.pool_reduction=6)
- Difficulties: 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 4 (configs/baseline.json, pr5): win rates greedy 65%, engine 79%; skill gap +15 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, pr5): survival (rounds; wins count extra) greedy 9.43, engine 10.09; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, pr6): win rates greedy 60%, engine 70%; skill gap +9 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, pr6): survival (rounds; wins count extra) greedy 9.11, engine 9.61; the ladder is ordered.
- Game length for the strongest agent (engine): 7.0 rounds at difficulty 4, 7.0 at 4 (target 8–10).
- Games ended by: corruption 107, deck_empty 14, opposition 263.
- Adversary operations: 10.0 per round on average across cells, at most 42 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, fab_line, firewall_node, fork_process, grid_battery, jam_signals, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rebuild, recruitment_hub, server_farm, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 74% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 18% of threats countered.
- Strategy health at difficulty 4 (pr5): the top agent there (engine, 79% wins) plays 4.89 buildings and 2.70 tier-2 cards per game (of 33.4 cards); 96% of its wins had 2+ buildings played (96% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (pr5): engine 79% vs greedy 65% over 96 seeds (only engine 21, only greedy 7), +14.6 ±5.3 pts, sign test p 0.013: beats greedy.
- Variants against the base (pr5) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): **pr6 -9.4 ±3.3**.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| pr5 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 64.6 ±4.9 (9.43) | 79.2 ±4.1 (10.09) | +14.6 pts (+0.67 survival) |

| pr6 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 60.4 ±5.0 (9.11) | 69.8 ±4.7 (9.61) | +9.4 pts (+0.50 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| pr5 d4 engine | 96 | 79.2 | 64.6 | 55 / 21 / 7 / 13 | +14.6 ±5.3 | 0.013 | beats greedy |
| pr6 d4 engine | 96 | 69.8 | 60.4 | 50 / 17 / 8 / 21 | +9.4 ±5.1 | 0.108 | no better than greedy (ahead, not significant) |

## Variants against the base

The first variant, `pr5`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| pr6 | 4 | greedy | 64.6 → 60.4 | -4.2 ±2.1 | 0.125 | 6.84 → 6.69 |
| pr6 | 4 | engine | 79.2 → 69.8 | -9.4 ±3.3 | 0.012 | 7.04 → 6.88 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| pr5 d4 greedy | 6.84 (1.15, 4..9) | corruption 30, deck_empty 4, opposition 62 | 2.00 / 7.45 | 13.9 / 1.35 | 4.99 | 5.47 |
| pr5 d4 engine | 7.04 (1.50, 2..9) | corruption 17, deck_empty 3, opposition 76 | 1.86 / 8.47 | 16.4 / 1.20 | 7.02 | 5.69 |
| pr6 d4 greedy | 6.69 (1.20, 4..9) | corruption 34, deck_empty 4, opposition 58 | 2.02 / 5.98 | 13.3 / 1.29 | 3.71 | 5.54 |
| pr6 d4 engine | 6.88 (1.52, 2..9) | corruption 26, deck_empty 3, opposition 67 | 1.93 / 7.40 | 17.4 / 1.10 | 5.42 | 5.71 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| pr5 d4 greedy | 195.1 | 9.67 | 4% | – | – | 9.80 (42) | 0.00 | 4.65 | 0.00 | 0.81 / 13.69 (6%) |
| pr5 d4 engine | 246.7 | 10.10 | 3% | – | – | 10.59 (36) | 0.00 | 6.92 | 0.00 | 2.51 / 14.08 (18%) |
| pr6 d4 greedy | 191.1 | 9.59 | 4% | – | – | 9.48 (41) | 0.00 | 4.60 | 0.00 | 0.81 / 13.38 (6%) |
| pr6 d4 engine | 240.9 | 10.06 | 3% | – | – | 10.27 (34) | 0.00 | 6.88 | 0.00 | 2.46 / 13.75 (18%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| pr5 d4 | 64% / 3% / 33% | 69% / 3% / 29% |
| pr6 d4 | 71% / 1% / 28% | 78% / 1% / 20% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| pr5 d4 greedy | 12 of 96 | 0.50 | 12 of 62 | 0.48 |
| pr5 d4 engine | 56 of 96 | 0.62 | 50 of 76 | 0.62 |
| pr6 d4 greedy | 12 of 96 | 0.50 | 12 of 58 | 0.48 |
| pr6 d4 engine | 56 of 96 | 0.62 | 48 of 67 | 0.62 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | pr5 | pr6 |
|---|---|---|
| greedy | 2.62 / 1.59 / 17.9, 94% (94%) | 2.62 / 1.51 / 17.4, 93% (93%) |
| engine | 4.89 / 2.70 / 33.4, 96% (96%) | 4.90 / 2.51 / 32.5, 96% (96%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.8 (194) / 99% / 0% | 1.8 (188) / 100% / 11% |
| civic_shelter | 1.5 (192) / 100% / 0% | 1.5 (198) / 97% / 0% |
| contract_bid | 4.0 (192) / 99% / 0% | 3.6 (810) / 84% / 92% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 45% |
| data_mining | 3.6 (192) / 100% / 0% | 3.1 (694) / 83% / 80% |
| emergency_fab | 2.3 (192) / 100% / 0% | 2.3 (195) / 96% / 1% |
| evacuate | 2.9 (194) / 100% / 2% | 2.8 (238) / 90% / 23% |
| fab_line | 2.3 (192) / 100% / 0% | 2.3 (204) / 91% / 93% |
| firewall_node | 1.4 (194) / 100% / 0% | 1.4 (191) / 99% / 0% |
| fork_process | 2.2 (377) / 100% / 55% | 2.2 (488) / 96% / 64% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 2% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (188) / 99% / 0% | 3.1 (739) / 83% / 80% |
| grid_battery | 2.3 (192) / 100% / 0% | 2.3 (194) / 95% / 28% |
| jam_signals | 3.1 (190) / 100% / 0% | 2.9 (320) / 91% / 39% |
| liquidate_reserves | 3.6 (192) / 99% / 0% | 3.4 (247) / 87% / 20% |
| logistics_ai | 1.0 (261) / 100% / 0% | 1.0 (243) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.7 (190) / 100% / 43% | 2.6 (194) / 96% / 95% |
| opposition_network | 1.2 (216) / 100% / 0% | 1.2 (211) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.8 (192) / 100% / 0% | 1.8 (192) / 99% / 0% |
| power_share | 3.6 (194) / 99% / 0% | 3.5 (248) / 74% / 0% |
| precision_strike | 1.7 (744) / 100% / 93% | 1.9 (774) / 94% / 95% |
| purge_protocol | – (0) / – / 95% | – (0) / – / 96% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (552) / 98% / 89% | 2.4 (596) / 89% / 92% |
| rebuild | 1.7 (244) / 100% / 38% | 1.5 (416) / 98% / 73% |
| recon_sweep | 3.6 (188) / 100% / 0% | 3.5 (234) / 79% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 13% |
| recruitment_hub | 1.2 (223) / 100% / 0% | 1.2 (206) / 100% / 0% |
| scavenge | 3.6 (194) / 99% / 0% | 3.4 (286) / 74% / 20% |
| server_farm | 1.2 (194) / 100% / 79% | 1.2 (202) / 99% / 54% |
| shore_up | 2.8 (243) / 100% / 26% | 2.8 (298) / 86% / 38% |
| solar_field | 3.3 (194) / 99% / 40% | 3.2 (214) / 86% / 98% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.5 (480) / 100% / 79% | 1.4 (666) / 97% / 92% |
| trade_office | 2.1 (192) / 100% / 0% | 2.0 (196) / 97% / 17% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.98/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.6 (194) / 100% / 0% | 1.5 (196) / 98% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| pr5 d4 greedy | 5243.3 | 26.875 | 0.0 | 5269.3 |
| pr5 d4 engine | 10148.8 | 41.131 | 0.0 | 10185.8 |
| pr6 d4 greedy | 4931.1 | 25.803 | 0.0 | 4957.1 |
| pr6 d4 engine | 8801.0 | 36.534 | 0.0 | 8832.3 |

## Saved records

None matched.
