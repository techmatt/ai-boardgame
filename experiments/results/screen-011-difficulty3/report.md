# Cycle report: screen-011-difficulty3

011 third screen (no probes), fresh seeds, difficulty 1 only: every requirement +2; one extra threat a round in all (countries in turn); and r3's first-requirement +2 with every requirement +1. Greedy only.

576 games (6 cells × 96 seeds, 520000..=520095) on 12 threads in 561.8 s. Engine 0.1.0, rules r4, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: cba2 (adversary.counter_bonus=0 adversary.counter_bonus_all=2); xt1_total (adversary.counter_bonus=0 adversary.extra_threats=1); cb2_cba1 (adversary.counter_bonus=2 adversary.counter_bonus_all=1)
- Difficulties: 0, 1
- Agents (weakest first): greedy
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, cba2): win rates greedy 96%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, cba2): survival (rounds; wins count extra) greedy 11.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, cba2): win rates greedy 96%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, cba2): survival (rounds; wins count extra) greedy 11.81; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, xt1_total): win rates greedy 96%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, xt1_total): survival (rounds; wins count extra) greedy 11.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, xt1_total): win rates greedy 84%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, xt1_total): survival (rounds; wins count extra) greedy 11.27; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, cb2_cba1): win rates greedy 96%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, cb2_cba1): survival (rounds; wins count extra) greedy 11.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, cb2_cba1): win rates greedy 95%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, cb2_cba1): survival (rounds; wins count extra) greedy 11.77; the ladder is ordered.
- Skill gap from difficulty 0 to 1: +0 → +0 pts win rate, +0.00 → +0.00 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.6 at 1 (target 8–10).
- Games ended by: corruption 26, deck_empty 2, opposition 540, population 8.
- Adversary operations: 9.8 per round on average across cells, at most 51 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Help and counters (strongest agent): 0.00 gifts per game; 28% of threats countered.
- Strategy health at difficulty 0 (cba2): the top agent there (greedy, 96% wins) plays 1.90 buildings and 2.22 tier-2 cards per game (of 19.6 cards); 89% of its wins had 2+ buildings played (89% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (cba2): the top agent there (greedy, 96% wins) plays 1.86 buildings and 2.27 tier-2 cards per game (of 19.4 cards); 86% of its wins had 2+ buildings played (86% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Variants against the base (cba2) at difficulty 0, greedy, win rate difference in pts (bold: sign test p < 0.05): xt1_total +0.0 ±0.0; cb2_cba1 +0.0 ±0.0.
- Variants against the base (cba2) at difficulty 1, greedy, win rate difference in pts (bold: sign test p < 0.05): **xt1_total -11.5 ±3.9**; cb2_cba1 -1.0 ±1.8.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| cba2 difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.81) | +0.0 pts (+0.00 survival) |

| cba2 difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.81) | +0.0 pts (+0.00 survival) |

| xt1_total difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.81) | +0.0 pts (+0.00 survival) |

| xt1_total difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 84.4 ±3.7 (11.27) | +0.0 pts (+0.00 survival) |

| cb2_cba1 difficulty 0 | greedy | skill gap |
|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.81) | +0.0 pts (+0.00 survival) |

| cb2_cba1 difficulty 1 | greedy | skill gap |
|---|---|---|
| win % (survival) | 94.8 ±2.3 (11.77) | +0.0 pts (+0.00 survival) |

## Variants against the base

The first variant, `cba2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| xt1_total | 0 | greedy | 95.8 → 95.8 | +0.0 ±0.0 | 1.000 | 7.77 → 7.77 |
| xt1_total | 1 | greedy | 95.8 → 84.4 | -11.5 ±3.9 | 0.007 | 7.65 → 7.80 |
| cb2_cba1 | 0 | greedy | 95.8 → 95.8 | +0.0 ±0.0 | 1.000 | 7.77 → 7.77 |
| cb2_cba1 | 1 | greedy | 95.8 → 94.8 | -1.0 ±1.8 | 1.000 | 7.65 → 7.64 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| cba2 d0 greedy | 7.77 (0.82, 6..10) | corruption 2, opposition 92, population 2 | 2.22 / 16.77 | 22.8 / 1.50 | 16.30 | 5.40 |
| cba2 d1 greedy | 7.65 (0.88, 4..10) | corruption 2, deck_empty 1, opposition 92, population 1 | 2.35 / 15.92 | 28.0 / 1.25 | 15.40 | 5.29 |
| xt1_total d0 greedy | 7.77 (0.82, 6..10) | corruption 2, opposition 92, population 2 | 2.22 / 16.77 | 22.8 / 1.50 | 16.30 | 5.40 |
| xt1_total d1 greedy | 7.80 (0.82, 6..10) | corruption 15, opposition 81 | 2.11 / 13.65 | 16.2 / 1.60 | 11.52 | 5.15 |
| cb2_cba1 d0 greedy | 7.77 (0.82, 6..10) | corruption 2, opposition 92, population 2 | 2.22 / 16.77 | 22.8 / 1.50 | 16.30 | 5.40 |
| cb2_cba1 d1 greedy | 7.64 (0.87, 4..10) | corruption 3, deck_empty 1, opposition 91, population 1 | 2.36 / 15.95 | 15.4 / 1.80 | 15.16 | 5.23 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| cba2 d0 greedy | 215.3 | 10.08 | 4% | – | – | 9.22 (48) | 0.00 | 9.39 | 0.00 | 5.70 / 15.54 (37%) |
| cba2 d1 greedy | 212.6 | 9.84 | 4% | – | – | 10.00 (42) | 0.00 | 6.24 | 0.00 | 1.83 / 15.29 (12%) |
| xt1_total d0 greedy | 215.3 | 10.08 | 4% | – | – | 9.22 (48) | 0.00 | 9.39 | 0.00 | 5.70 / 15.54 (37%) |
| xt1_total d1 greedy | 222.8 | 10.17 | 4% | – | – | 11.29 (51) | 0.00 | 13.14 | 0.00 | 8.02 / 23.41 (34%) |
| cb2_cba1 d0 greedy | 215.3 | 10.08 | 4% | – | – | 9.22 (48) | 0.00 | 9.39 | 0.00 | 5.70 / 15.54 (37%) |
| cb2_cba1 d1 greedy | 213.1 | 9.86 | 4% | – | – | 10.08 (45) | 0.00 | 6.18 | 0.00 | 1.42 / 15.27 (9%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| cba2 d0 | – (no variance) | – (no variance) |
| cba2 d1 | – (no variance) | – (no variance) |
| xt1_total d0 | – (no variance) | – (no variance) |
| xt1_total d1 | – (no variance) | – (no variance) |
| cb2_cba1 d0 | – (no variance) | – (no variance) |
| cb2_cba1 d1 | – (no variance) | – (no variance) |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| cba2 d0 greedy | 3 of 96 | 0.36 | 3 of 92 | 0.36 |
| cba2 d1 greedy | 3 of 96 | 0.37 | 3 of 92 | 0.37 |
| xt1_total d0 greedy | 3 of 96 | 0.36 | 3 of 92 | 0.36 |
| xt1_total d1 greedy | 3 of 96 | 0.37 | 3 of 81 | 0.35 |
| cb2_cba1 d0 greedy | 3 of 96 | 0.36 | 3 of 92 | 0.36 |
| cb2_cba1 d1 greedy | 3 of 96 | 0.37 | 3 of 91 | 0.37 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | cba2 | xt1_total | cb2_cba1 |
|---|---|---|---|
| greedy | 1.90 / 2.22 / 19.6, 89% (89%) | 1.90 / 2.22 / 19.6, 89% (89%) | 1.90 / 2.22 / 19.6, 89% (89%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | cba2 | xt1_total | cb2_cba1 |
|---|---|---|---|
| greedy | 1.86 / 2.27 / 19.4, 86% (86%) | 1.83 / 2.06 / 19.5, 85% (85%) | 1.80 / 2.29 / 19.4, 80% (80%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play |
|---|---|
| antimatter_refinery | – (0) / – / 0% |
| assembly_cluster | 1.1 (661) / 99% / 0% |
| civic_shelter | 2.0 (578) / 99% / 0% |
| contract_bid | 3.9 (590) / 97% / 0% |
| counter_intelligence | – (0) / – / 0% |
| data_mining | 3.7 (582) / 98% / 0% |
| emergency_fab | 2.6 (568) / 99% / 0% |
| evacuate | 3.1 (571) / 100% / 1% |
| fab_line | 1.8 (574) / 100% / 0% |
| firewall_node | 1.9 (575) / 99% / 0% |
| fork_process | 2.1 (1167) / 99% / 53% |
| fusion_plant | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% |
| grant | 3.6 (580) / 99% / 0% |
| grid_battery | 1.9 (567) / 100% / 0% |
| jam_signals | 3.1 (573) / 99% / 0% |
| liquidate_reserves | 3.5 (576) / 98% / 0% |
| logistics_ai | 1.3 (592) / 100% / 0% |
| megastructure | – (0) / – / 0% |
| microgrid | 2.3 (578) / 100% / 0% |
| opposition_network | 1.5 (571) / 100% / 0% |
| orbital_lance | – (0) / – / 0% |
| overclock_rig | 1.0 (684) / 100% / 0% |
| power_share | 3.6 (577) / 100% / 0% |
| precision_strike | 1.6 (2695) / 100% / 92% |
| purge_protocol | – (0) / – / 96% |
| quantum_core | – (0) / – / 0% |
| rally | 2.1 (2099) / 100% / 90% |
| rebuild | 2.0 (723) / 100% / 25% |
| recon_sweep | 3.6 (577) / 99% / 0% |
| reconstruction | – (0) / – / 0% |
| recruitment_hub | 1.4 (581) / 100% / 0% |
| scavenge | 3.6 (579) / 99% / 0% |
| server_farm | 1.5 (572) / 100% / 86% |
| shore_up | 2.9 (779) / 98% / 29% |
| solar_field | 2.7 (574) / 100% / 0% |
| strike_team | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% |
| sweep | 1.5 (1849) / 100% / 80% |
| trade_office | 1.2 (617) / 100% / 0% |
| turnkey_factory | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (570) / 99% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| cba2 d0 greedy | 10131.1 | 47.062 | 0.0 | 10180.5 |
| cba2 d1 greedy | 7150.3 | 33.627 | 0.0 | 7185.0 |
| xt1_total d0 greedy | 11422.6 | 53.061 | 0.0 | 11477.8 |
| xt1_total d1 greedy | 14678.5 | 65.875 | 0.0 | 14746.7 |
| cb2_cba1 d0 greedy | 12832.3 | 59.610 | 0.0 | 12895.3 |
| cb2_cba1 d1 greedy | 12437.0 | 58.356 | 0.0 | 12504.0 |

## Saved records

None matched.
