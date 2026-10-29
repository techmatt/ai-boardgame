# Cycle report: screen-017-projects

017 quick screen of rules r8 (the adversary project deck filled by node and threat compute, the Discord troop advance, one-shot Alignment cards with printed Discord steps, more Unity sources, no Discord escalation): win rates and length, the strategy-health gate, and per game projects resolved (cheap, big), adversary compute made and blocked, Discord and Unity tips, troop advances and strikes, and the round of the first project. 2 and 3 countries, d0 and d4, greedy and engine. No tuning.

128 games (8 cells × 16 seeds, 0..=15) on 12 threads in 9.1 s. Engine 0.1.0, rules r8, content 428f15e336a3b070.

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
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) greedy 4.06, engine 3.75; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) greedy 2.25, engine 2.44; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) greedy 4.75, engine 4.81; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) greedy 2.31, engine 2.31; the ladder is ordered.
- Skill gap from difficulty 0 to 4: +0 → +0 pts win rate, -0.12 → +0.09 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 3.8 rounds at difficulty 0, 2.4 at 4 (target 8–10).
- Games ended by: supply 128.
- Adversary operations: 30.6 per round on average across cells, at most 92 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, security_patch, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 79% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.94 gifts per game; 43% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 0% wins) plays 2.62 buildings and 0.25 tier-2 cards per game (of 11.9 cards); – of its wins had 2+ buildings played (– with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 4 (c2): the top agent there (engine, 0% wins) plays 0.94 buildings and 0.00 tier-2 cards per game (of 5.9 cards); – of its wins had 2+ buildings played (– with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 0% vs greedy 0% over 16 seeds (only engine 0, only greedy 0), +0.0 ±0.0 pts, sign test p 1.000: no better than greedy (level).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±0.0.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±0.0.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| c2 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.06) | 0.0 ±0.0 (3.75) | +0.0 pts (-0.31 survival) |

| c2 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.25) | 0.0 ±0.0 (2.44) | +0.0 pts (+0.19 survival) |

| c3 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.75) | 0.0 ±0.0 (4.81) | +0.0 pts (+0.06 survival) |

| c3 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (2.31) | 0.0 ±0.0 (2.31) | +0.0 pts (+0.00 survival) |

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
| c3 | 0 | greedy | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.06 → 4.75 |
| c3 | 0 | engine | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 3.75 → 4.81 |
| c3 | 4 | greedy | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 2.25 → 2.31 |
| c3 | 4 | engine | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 2.44 → 2.31 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 greedy | 4.06 (1.43, 2..6) | supply 16 | – / – | 27.6 / 0.25 | 0.50 | 2.38 / 0.00 |
| c2 d0 engine | 3.75 (1.79, 2..9) | supply 16 | – / – | 23.2 / 0.38 | 0.25 | 2.12 / 0.38 |
| c2 d4 greedy | 2.25 (0.56, 1..3) | supply 16 | – / – | 25.8 / 0.00 | 0.31 | 2.12 / 0.12 |
| c2 d4 engine | 2.44 (0.70, 1..4) | supply 16 | – / – | 24.3 / 0.06 | 0.50 | 2.19 / 0.12 |
| c3 d0 greedy | 4.75 (1.09, 2..6) | supply 16 | – / – | 43.7 / 0.12 | 0.00 | 2.56 / 0.06 |
| c3 d0 engine | 4.81 (1.67, 2..8) | supply 16 | – / – | 36.2 / 0.38 | 0.00 | 3.00 / 1.06 |
| c3 d4 greedy | 2.31 (0.58, 1..3) | supply 16 | – / – | 37.8 / 0.00 | 0.12 | 3.06 / 0.06 |
| c3 d4 engine | 2.31 (0.68, 1..4) | supply 16 | – / – | 36.5 / 0.06 | 0.12 | 3.00 / 0.00 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 greedy | 144.8 | 14.28 | 2% | – | – | 26.03 (69) | 2.69 | 4.94 | 0.00 | 2.94 / 10.25 (29%) |
| c2 d0 engine | 134.4 | 14.55 | 2% | – | – | 23.48 (62) | 1.62 | 5.00 | 0.00 | 4.19 / 8.38 (50%) |
| c2 d4 greedy | 59.8 | 10.26 | 3% | – | – | 25.92 (77) | 0.06 | 1.88 | 0.00 | 1.25 / 5.44 (23%) |
| c2 d4 engine | 68.9 | 11.03 | 2% | – | – | 24.44 (56) | 0.06 | 2.50 | 0.00 | 2.00 / 6.12 (33%) |
| c3 d0 greedy | 225.2 | 19.03 | 2% | – | – | 37.57 (90) | 2.62 | 7.25 | 0.00 | 4.81 / 14.50 (33%) |
| c3 d0 engine | 246.9 | 19.57 | 2% | – | – | 35.94 (80) | 2.06 | 9.19 | 0.00 | 7.69 / 14.81 (52%) |
| c3 d4 greedy | 84.1 | 13.84 | 2% | – | – | 34.78 (73) | 0.06 | 2.81 | 0.00 | 2.06 / 6.69 (31%) |
| c3 d4 engine | 95.4 | 14.60 | 2% | – | – | 36.65 (92) | 0.00 | 3.12 | 0.00 | 2.56 / 6.94 (37%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | – (no variance) | 77% / 1% / 22% |
| c2 d4 | – (no variance) | 89% / 2% / 9% |
| c3 d0 | – (no variance) | 60% / 0% / 40% |
| c3 d4 | – (no variance) | 92% / 0% / 8% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 greedy | 6 of 16 | 0.49 | 0 of 0 | – |
| c2 d0 engine | 12 of 16 | 0.67 | 0 of 0 | – |
| c2 d4 greedy | 6 of 16 | 0.69 | 0 of 0 | – |
| c2 d4 engine | 7 of 16 | 0.67 | 0 of 0 | – |
| c3 d0 greedy | 9 of 16 | 0.48 | 0 of 0 | – |
| c3 d0 engine | 16 of 16 | 0.63 | 0 of 0 | – |
| c3 d4 greedy | 9 of 16 | 0.68 | 0 of 0 | – |
| c3 d4 engine | 15 of 16 | 0.78 | 0 of 0 | – |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| greedy | 1.88 / 0.00 / 8.0, – | 2.31 / 0.00 / 13.7, – |
| engine | 2.62 / 0.25 / 11.9, – | 4.00 / 0.69 / 22.9, – |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| greedy | 0.88 / 0.00 / 3.9, – | 1.38 / 0.00 / 5.6, – |
| engine | 0.94 / 0.00 / 5.9, – | 2.00 / 0.06 / 8.4, – |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (45) / 100% / 0% | 1.1 (45) / 100% / 2% |
| civic_shelter | 1.7 (40) / 100% / 0% | 1.7 (39) / 100% / 0% |
| contract_bid | 4.0 (44) / 100% / 0% | 3.5 (77) / 99% / 100% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 54% |
| data_mining | 3.9 (51) / 100% / 0% | 3.7 (59) / 100% / 39% |
| emergency_fab | 2.8 (40) / 100% / 0% | 2.8 (39) / 100% / 0% |
| evacuate | 2.9 (55) / 100% / 47% | 2.9 (51) / 98% / 38% |
| fab_line | 1.9 (40) / 100% / 0% | 1.9 (38) / 100% / 47% |
| firewall_node | 2.0 (38) / 100% / 0% | 1.9 (37) / 100% / 0% |
| fork_process | 2.4 (57) / 100% / 49% | 2.4 (56) / 100% / 61% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.3 (39) / 100% / 0% | 3.1 (56) / 100% / 64% |
| grid_battery | 1.9 (42) / 100% / 0% | 1.9 (41) / 100% / 10% |
| jam_signals | 3.2 (49) / 100% / 0% | 2.9 (70) / 99% / 90% |
| liquidate_reserves | 3.6 (41) / 100% / 0% | 3.6 (42) / 100% / 2% |
| logistics_ai | 1.4 (45) / 100% / 0% | 1.4 (44) / 100% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 1.7 (50) / 100% / 0% | 1.7 (50) / 100% / 30% |
| opposition_network | 1.8 (44) / 100% / 0% | 1.8 (45) / 100% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (46) / 100% / 0% | 1.0 (43) / 100% / 0% |
| power_share | 3.8 (38) / 100% / 0% | 3.8 (36) / 97% / 0% |
| precision_strike | 2.2 (82) / 100% / 79% | 2.3 (83) / 100% / 82% |
| purge_protocol | – (0) / – / 0% | – (0) / – / 75% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.8 (70) / 100% / 71% | 2.7 (74) / 99% / 76% |
| rebuild | 1.6 (62) / 100% / 68% | 1.6 (56) / 100% / 75% |
| recon_sweep | 3.8 (48) / 100% / 0% | 3.8 (48) / 100% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.2 (37) / 100% / 86% | 1.3 (35) / 100% / 43% |
| scavenge | 3.8 (48) / 100% / 0% | 3.7 (47) / 100% / 13% |
| security_patch | 3.3 (42) / 100% / 0% | 3.3 (40) / 100% / 0% |
| server_farm | 1.5 (38) / 100% / 53% | 1.5 (37) / 100% / 51% |
| shore_up | 3.1 (39) / 100% / 0% | 3.1 (36) / 100% / 0% |
| solar_field | 2.7 (39) / 100% / 0% | 2.7 (37) / 100% / 78% |
| strike_team | – (0) / – / 0.75/game | – (0) / – / 0.84/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.9 (74) / 100% / 99% | 1.8 (73) / 100% / 99% |
| trade_office | 1.3 (45) / 100% / 0% | 1.3 (44) / 100% / 0% |
| turnkey_factory | – (0) / – / 0.80/game | – (0) / – / 0.81/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.8 (45) / 100% / 0% | 1.8 (46) / 100% / 0% |
| zero_day_wipe | – (0) / – / 0% | – (0) / – / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 greedy | 507.4 | 3.505 | 0.0 | 523.4 |
| c2 d0 engine | 596.7 | 4.440 | 0.0 | 612.2 |
| c2 d4 greedy | 149.7 | 2.505 | 0.0 | 156.3 |
| c2 d4 engine | 204.1 | 2.964 | 0.0 | 212.2 |
| c3 d0 greedy | 1519.6 | 6.748 | 0.0 | 1557.1 |
| c3 d0 engine | 2171.7 | 8.794 | 0.0 | 2214.2 |
| c3 d4 greedy | 388.9 | 4.623 | 0.0 | 400.7 |
| c3 d4 engine | 635.3 | 6.661 | 0.0 | 650.0 |

Where the games' time went (summed over workers): agents 97.6%, probes 0.0%, apply 1.0%, legal 0.4%, replay check 0.9%, setup 0.1%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c3-baseline-c2-d4-engine-seed6.json`](records/close-loss-c3-baseline-c2-d4-engine-seed6.json): close loss at difficulty 4: supply in round 4, 10 adversary piece(s) over the level-1 bar, supply 0 of 16, 2 cities corrupted
- [`records/close-loss-c7-baseline-c3-d4-engine-seed6.json`](records/close-loss-c7-baseline-c3-d4-engine-seed6.json): close loss at difficulty 4: supply in round 4, 18 adversary piece(s) over the level-1 bar, supply 0 of 24, 2 cities corrupted

**early-lock-in**: none matched


**surprising-win**: none matched


