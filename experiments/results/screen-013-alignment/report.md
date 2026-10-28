# Cycle report: screen-013-alignment

013 quick screen of Alignment (rules r5): Unity and Discord tips per game and by round, win rates, and how often the +1 Discord antimatter action is taken, at 2 countries, d0 and d4. Variants: no_am switches the antimatter action off; off switches Alignment off (no Sway, no tips, bar 0) for reference.

384 games (12 cells × 32 seeds, 0..=31) on 12 threads in 45.3 s. Engine 0.1.0, rules r5, content 5bc9c388530e0342.

- Configs: configs/baseline.json
- Variants: base; no_am (alignment.antimatter_discord=0); off (alignment.variant=off)
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, base): win rates greedy 22%, engine 25%; skill gap +3 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, base): survival (rounds; wins count extra) greedy 8.91, engine 9.34; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, base): win rates greedy 3%, engine 9%; skill gap +6 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, base): survival (rounds; wins count extra) greedy 6.03, engine 6.31; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, no_am): win rates greedy 22%, engine 16%; skill gap -6 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, no_am): survival (rounds; wins count extra) greedy 8.91, engine 9.19; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, no_am): win rates greedy 3%, engine 3%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, no_am): survival (rounds; wins count extra) greedy 6.03, engine 6.19; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, off): win rates greedy 0%, engine 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, off): survival (rounds; wins count extra) greedy 9.53, engine 9.69; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, off): win rates greedy 0%, engine 3%; skill gap +3 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, off): survival (rounds; wins count extra) greedy 7.00, engine 7.22; the ladder is ordered.
- Skill gap from difficulty 0 to 4: -1 → +3 pts win rate, +0.29 → +0.22 survival (target: widens with difficulty).
- Game length for the strongest agent (engine): 8.5 rounds at difficulty 0, 6.0 at 4 (target 8–10).
- Games ended by: corruption 262, deck_empty 78, population 10, purge 34.
- Adversary operations: 13.9 per round on average across cells, at most 52 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, emergency_fab, fab_line, firewall_node, grid_battery, logistics_ai, microgrid, opposition_network, overclock_rig, precision_strike, rally, rebuild, recruitment_hub, server_farm, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 83% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 49% of threats countered.
- Strategy health at difficulty 0 (base): the top agent there (engine, 25% wins) plays 3.56 buildings and 3.91 tier-2 cards per game (of 42.9 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (base): the top agent there (engine, 9% wins) plays 2.44 buildings and 1.22 tier-2 cards per game (of 22.3 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (base): engine 9% vs greedy 3% over 32 seeds (only engine 2, only greedy 0), +6.2 ±4.3 pts, sign test p 0.500: no better than greedy (ahead, not significant).
- Variants against the base (base) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): no_am -9.4 ±8.2; **off -25.0 ±7.8**.
- Variants against the base (base) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): no_am -6.2 ±4.3; off -6.2 ±4.3.
- What a module is worth (base win rate minus with it off) at difficulty 0, engine: **alignment +25.0 ±7.8** pts.
- What a module is worth (base win rate minus with it off) at difficulty 4, engine: alignment +6.2 ±4.3 pts.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| base difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 21.9 ±7.3 (8.91) | 25.0 ±7.7 (9.34) | +3.1 pts (+0.44 survival) |

| base difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 3.1 ±3.1 (6.03) | 9.4 ±5.2 (6.31) | +6.2 pts (+0.28 survival) |

| no_am difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 21.9 ±7.3 (8.91) | 15.6 ±6.4 (9.19) | -6.2 pts (+0.28 survival) |

| no_am difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 3.1 ±3.1 (6.03) | 3.1 ±3.1 (6.19) | +0.0 pts (+0.16 survival) |

| off difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (9.53) | 0.0 ±0.0 (9.69) | +0.0 pts (+0.16 survival) |

| off difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.00) | 3.1 ±3.1 (7.22) | +3.1 pts (+0.22 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| base d0 engine | 32 | 25.0 | 21.9 | 3 / 5 / 4 / 20 | +3.1 ±9.5 | 1.000 | no better than greedy (ahead, not significant) |
| base d4 engine | 32 | 9.4 | 3.1 | 1 / 2 / 0 / 29 | +6.2 ±4.3 | 0.500 | no better than greedy (ahead, not significant) |
| no_am d0 engine | 32 | 15.6 | 21.9 | 2 / 3 / 5 / 22 | -6.2 ±8.9 | 0.727 | no better than greedy (behind, not significant) |
| no_am d4 engine | 32 | 3.1 | 3.1 | 0 / 1 / 1 / 30 | +0.0 ±4.5 | 1.000 | no better than greedy (level) |
| off d0 engine | 32 | 0.0 | 0.0 | 0 / 0 / 0 / 32 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| off d4 engine | 32 | 3.1 | 0.0 | 0 / 1 / 0 / 31 | +3.1 ±3.1 | 1.000 | no better than greedy (ahead, not significant) |

## Variants against the base

The first variant, `base`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| no_am | 0 | greedy | 21.9 → 21.9 | +0.0 ±0.0 | 1.000 | 8.06 → 8.06 |
| no_am | 0 | engine | 25.0 → 15.6 | -9.4 ±8.2 | 0.453 | 8.53 → 8.62 |
| no_am | 4 | greedy | 3.1 → 3.1 | +0.0 ±0.0 | 1.000 | 5.94 → 5.94 |
| no_am | 4 | engine | 9.4 → 3.1 | -6.2 ±4.3 | 0.500 | 6.03 → 6.12 |
| off | 0 | greedy | 21.9 → 0.0 | -21.9 ±7.4 | 0.016 | 8.06 → 9.53 |
| off | 0 | engine | 25.0 → 0.0 | -25.0 ±7.8 | 0.008 | 8.53 → 9.69 |
| off | 4 | greedy | 3.1 → 0.0 | -3.1 ±3.1 | 1.000 | 5.94 → 7.00 |
| off | 4 | engine | 9.4 → 3.1 | -6.2 ±4.3 | 0.500 | 6.03 → 7.03 |

## Removing a module

What a module is worth: the base's win rate minus the win rate with the module switched off, on the same seeds, ± the standard error of the per-seed difference. Positive: the module helps the players; negative: it holds them back.

| module (variant) | difficulty | agent | base win % | off win % | worth (pts) | p |
|---|---|---|---|---|---|---|
| alignment (off) | 0 | greedy | 21.9 | 0.0 | +21.9 ±7.4 | 0.016 |
| alignment (off) | 0 | engine | 25.0 | 0.0 | +25.0 ±7.8 | 0.008 |
| alignment (off) | 4 | greedy | 3.1 | 0.0 | +3.1 ±3.1 | 1.000 |
| alignment (off) | 4 | engine | 9.4 | 3.1 | +6.2 ±4.3 | 0.500 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| base d0 greedy | 8.06 (1.41, 5..10) | corruption 20, deck_empty 5, purge 7 | 1.86 / 14.71 | 13.9 / 1.52 | 3.94 | 4.69 |
| base d0 engine | 8.53 (1.30, 6..10) | corruption 19, deck_empty 5, purge 8 | 1.25 / 16.00 | 18.7 / 1.46 | 5.22 | 3.97 |
| base d4 greedy | 5.94 (1.58, 3..9) | corruption 29, deck_empty 1, population 1, purge 1 | 1.00 / 6.00 | 19.8 / 0.65 | 0.62 | 2.88 |
| base d4 engine | 6.03 (1.59, 4..9) | corruption 27, deck_empty 1, population 1, purge 3 | 1.00 / 7.67 | 25.6 / 0.41 | 0.94 | 2.38 |
| no_am d0 greedy | 8.06 (1.41, 5..10) | corruption 20, deck_empty 5, purge 7 | 1.86 / 14.71 | 13.9 / 1.52 | 3.94 | 4.69 |
| no_am d0 engine | 8.62 (1.39, 6..10) | corruption 21, deck_empty 6, purge 5 | 1.60 / 19.00 | 16.5 / 1.52 | 4.88 | 4.53 |
| no_am d4 greedy | 5.94 (1.58, 3..9) | corruption 29, deck_empty 1, population 1, purge 1 | 1.00 / 6.00 | 19.8 / 0.65 | 0.62 | 2.88 |
| no_am d4 engine | 6.12 (1.92, 3..9) | corruption 27, deck_empty 2, population 2, purge 1 | 0.00 / 4.00 | 25.0 / 0.35 | 0.78 | 2.09 |
| off d0 greedy | 9.53 (0.93, 7..10) | corruption 14, deck_empty 18 | – / – | 27.3 / 0.00 | 6.06 | 4.25 |
| off d0 engine | 9.69 (0.81, 7..10) | corruption 9, deck_empty 23 | – / – | 36.0 / 0.00 | 8.91 | 5.25 |
| off d4 greedy | 7.00 (1.56, 5..9) | corruption 25, deck_empty 5, population 2 | – / – | 26.0 / 0.00 | 1.34 | 1.38 |
| off d4 engine | 7.03 (1.61, 5..9) | corruption 22, deck_empty 6, population 3, purge 1 | 4.00 / 13.00 | 28.4 / 0.00 | 2.16 | 1.94 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| base d0 greedy | 238.4 | 11.19 | 4% | – | – | 13.88 (45) | 0.00 | 10.09 | 0.00 | 9.66 / 21.97 (44%) |
| base d0 engine | 314.2 | 10.91 | 3% | – | – | 14.84 (52) | 0.00 | 13.94 | 0.00 | 14.81 / 22.31 (66%) |
| base d4 greedy | 171.3 | 9.34 | 4% | – | – | 12.56 (49) | 0.00 | 6.75 | 0.00 | 4.97 / 18.81 (26%) |
| base d4 engine | 206.2 | 9.82 | 3% | – | – | 12.93 (46) | 0.00 | 9.50 | 0.00 | 7.16 / 19.31 (37%) |
| no_am d0 greedy | 238.4 | 10.69 | 4% | – | – | 13.88 (45) | 0.00 | 10.09 | 0.00 | 9.66 / 21.97 (44%) |
| no_am d0 engine | 318.1 | 10.28 | 3% | – | – | 14.68 (52) | 0.00 | 13.38 | 0.00 | 14.59 / 22.66 (64%) |
| no_am d4 greedy | 171.3 | 8.91 | 4% | – | – | 12.56 (49) | 0.00 | 6.75 | 0.00 | 4.97 / 18.81 (26%) |
| no_am d4 engine | 211.3 | 9.30 | 3% | – | – | 13.33 (40) | 0.00 | 8.62 | 0.00 | 6.91 / 19.75 (35%) |
| off d0 greedy | 253.0 | 11.33 | 4% | – | – | 14.01 (47) | 0.00 | 11.06 | 0.00 | 6.59 / 19.06 (35%) |
| off d0 engine | 351.8 | 10.90 | 4% | – | – | 16.03 (52) | 0.00 | 12.78 | 0.00 | 11.50 / 19.38 (59%) |
| off d4 greedy | 193.0 | 9.79 | 4% | – | – | 13.86 (40) | 0.00 | 8.22 | 0.00 | 4.16 / 21.00 (20%) |
| off d4 engine | 243.0 | 9.85 | 3% | – | – | 14.07 (46) | 0.00 | 9.31 | 0.00 | 6.91 / 21.09 (33%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| base d0 | 61% / 0% / 39% | 78% / 1% / 21% |
| base d4 | 73% / 2% / 25% | 86% / 1% / 14% |
| no_am d0 | 59% / 1% / 40% | 79% / 0% / 20% |
| no_am d4 | 48% / 0% / 52% | 84% / 0% / 15% |
| off d0 | – (no variance) | 79% / 1% / 21% |
| off d4 | 49% / 2% / 49% | 93% / 0% / 7% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| base d0 greedy | 3 of 32 | 0.35 | 2 of 7 | 0.38 |
| base d0 engine | 25 of 32 | 0.62 | 8 of 8 | 0.63 |
| base d4 greedy | 3 of 32 | 0.39 | 1 of 1 | – |
| base d4 engine | 15 of 32 | 0.54 | 3 of 3 | 0.53 |
| no_am d0 greedy | 3 of 32 | 0.35 | 2 of 7 | 0.38 |
| no_am d0 engine | 25 of 32 | 0.62 | 5 of 5 | 0.60 |
| no_am d4 greedy | 3 of 32 | 0.39 | 1 of 1 | – |
| no_am d4 engine | 14 of 32 | 0.54 | 1 of 1 | – |
| off d0 greedy | 3 of 32 | 0.35 | 0 of 0 | – |
| off d0 engine | 25 of 32 | 0.62 | 0 of 0 | – |
| off d4 greedy | 3 of 32 | 0.39 | 0 of 0 | – |
| off d4 engine | 16 of 32 | 0.54 | 1 of 1 | – |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | base | no_am | off |
|---|---|---|---|
| greedy | 1.94 / 2.50 / 23.9, 100% (100%) | 1.94 / 2.50 / 23.9, 100% (100%) | 1.97 / 4.06 / 23.3, – |
| engine | 3.56 / 3.91 / 42.9, 100% (100%) | 3.56 / 3.28 / 43.7, 100% (100%) | 3.75 / 4.78 / 50.1, – |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | base | no_am | off |
|---|---|---|---|
| greedy | 1.78 / 0.94 / 13.3, 100% (100%) | 1.78 / 0.94 / 13.3, 100% (100%) | 1.78 / 1.72 / 14.2, – |
| engine | 2.44 / 1.22 / 22.3, 100% (100%) | 2.41 / 1.06 / 23.1, 100% (100%) | 2.44 / 1.84 / 27.4, 100% (100%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (219) / 100% / 0% | 1.1 (199) / 99% / 4% |
| civic_shelter | 2.1 (193) / 98% / 0% | 2.1 (200) / 95% / 0% |
| contract_bid | 3.9 (196) / 98% / 0% | 3.4 (915) / 72% / 99% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 23% |
| data_mining | 3.5 (187) / 100% / 0% | 3.1 (618) / 86% / 77% |
| emergency_fab | 2.6 (191) / 100% / 0% | 2.5 (210) / 92% / 4% |
| evacuate | 2.9 (197) / 98% / 9% | 2.7 (291) / 86% / 46% |
| fab_line | 1.9 (193) / 98% / 0% | 1.8 (189) / 98% / 29% |
| firewall_node | 1.9 (195) / 98% / 0% | 2.0 (198) / 97% / 0% |
| fork_process | 2.2 (363) / 100% / 54% | 2.2 (408) / 88% / 59% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.7 (194) / 99% / 0% | 3.2 (681) / 80% / 82% |
| grid_battery | 1.7 (189) / 98% / 0% | 1.7 (190) / 96% / 4% |
| jam_signals | 3.3 (193) / 98% / 0% | 2.7 (677) / 86% / 84% |
| liquidate_reserves | 3.7 (192) / 100% / 0% | 3.5 (299) / 69% / 19% |
| logistics_ai | 1.3 (207) / 100% / 0% | 1.3 (203) / 98% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.5 (193) / 98% / 0% | 2.4 (194) / 96% / 44% |
| opposition_network | 1.5 (199) / 100% / 0% | 1.6 (192) / 99% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (234) / 100% / 0% | 1.0 (215) / 99% / 0% |
| power_share | 3.5 (195) / 98% / 0% | 3.4 (279) / 66% / 0% |
| precision_strike | 1.7 (731) / 98% / 92% | 1.9 (859) / 91% / 93% |
| purge_protocol | – (0) / – / 82% | – (0) / – / 87% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.3 (460) / 98% / 81% | 2.4 (511) / 92% / 80% |
| rebuild | 1.9 (254) / 100% / 42% | 1.6 (450) / 95% / 77% |
| recon_sweep | 3.5 (193) / 98% / 0% | 3.3 (268) / 69% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 0% |
| recruitment_hub | 1.6 (194) / 100% / 0% | 1.6 (195) / 98% / 0% |
| scavenge | 3.7 (196) / 97% / 0% | 3.5 (283) / 74% / 19% |
| server_farm | 1.4 (192) / 100% / 86% | 1.4 (200) / 94% / 50% |
| shore_up | 2.8 (242) / 100% / 34% | 2.6 (363) / 82% / 53% |
| solar_field | 2.8 (187) / 100% / 0% | 2.6 (198) / 95% / 73% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (606) / 99% / 89% | 1.4 (819) / 96% / 93% |
| trade_office | 1.2 (203) / 99% / 0% | 1.2 (200) / 98% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.8 (178) / 100% / 0% | 1.8 (189) / 97% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| base d0 greedy | 1054.6 | 4.424 | 0.0 | 1070.6 |
| base d0 engine | 2256.0 | 7.180 | 0.0 | 2276.5 |
| base d4 greedy | 625.2 | 3.649 | 0.0 | 636.3 |
| base d4 engine | 1151.7 | 5.584 | 0.0 | 1165.3 |
| no_am d0 greedy | 1032.8 | 4.333 | 0.0 | 1048.4 |
| no_am d0 engine | 2225.0 | 6.995 | 0.0 | 2246.0 |
| no_am d4 greedy | 630.9 | 3.682 | 0.0 | 641.7 |
| no_am d4 engine | 1239.7 | 5.867 | 0.0 | 1253.2 |
| off d0 greedy | 1068.1 | 4.222 | 0.0 | 1084.4 |
| off d0 engine | 2823.8 | 8.026 | 0.0 | 2846.7 |
| off d4 greedy | 731.5 | 3.790 | 0.0 | 744.0 |
| off d4 engine | 1522.6 | 6.265 | 0.0 | 1536.7 |

Where the games' time went (summed over workers): agents 98.9%, probes 0.0%, apply 0.5%, legal 0.2%, replay check 0.4%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c2-baseline-base-d4-greedy-seed20.json`](records/close-loss-c2-baseline-base-d4-greedy-seed20.json): close loss at difficulty 4: corruption in round 9, 1 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c6-baseline-no_am-d4-greedy-seed20.json`](records/close-loss-c6-baseline-no_am-d4-greedy-seed20.json): close loss at difficulty 4: corruption in round 9, 1 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-base-d0-greedy-seed2.json`](records/surprising-win-c0-baseline-base-d0-greedy-seed2.json): won by the weakest agent as measured (greedy)
- [`records/surprising-win-c4-baseline-no_am-d0-greedy-seed2.json`](records/surprising-win-c4-baseline-no_am-d0-greedy-seed2.json): won by the weakest agent as measured (greedy)

