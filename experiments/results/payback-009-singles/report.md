# Cycle report: payback-009-singles

009 engine-payback sweep, one lever at a time (config overrides only; baseline untouched). The engine agent (008) against greedy on the same seeds at difficulties 0 and 4. Does any config lever make building pay?

4992 games (52 cells × 96 seeds, 300000..=300095) on 12 threads in 2770.3 s. Engine 0.1.0, rules r3, content 5a1580ea226de951.

- Configs: configs/baseline.json
- Variants: base; keep0 (cards.keep_cost=0); keep2 (cards.keep_cost=2); keep3 (cards.keep_cost=3); deck443 (adversary.stage_cards=4,4,3); deck444 (adversary.stage_cards=4,4,4); deck446 (adversary.stage_cards=4,4,6); calm (adversary.momentum_drones=0 adversary.escalation_outposts=0); fc4 (economy.factory_credits=4); fc5 (economy.factory_credits=5); pc2 (economy.processor_compute=2); draw2 (cards.draw_cost=2); credits18 (ai.start_credits=18)
- Difficulties: 0, 4
- Agents (weakest first): greedy, engine
- Win probability: off
- Decision closeness: off
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, base): win rates greedy 96%, engine 95%; skill gap -1 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, base): survival (rounds; wins count extra) greedy 11.89, engine 11.78; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, base): win rates greedy 77%, engine 71%; skill gap -6 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, base): survival (rounds; wins count extra) greedy 10.04, engine 9.81; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, keep0): win rates greedy 96%, engine 97%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, keep0): survival (rounds; wins count extra) greedy 11.89, engine 11.89; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, keep0): win rates greedy 82%, engine 83%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, keep0): survival (rounds; wins count extra) greedy 10.32, engine 10.29; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, keep2): win rates greedy 79%, engine 58%; skill gap -21 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, keep2): survival (rounds; wins count extra) greedy 10.96, engine 10.40; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, keep2): win rates greedy 45%, engine 45%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, keep2): survival (rounds; wins count extra) greedy 8.36, engine 8.53; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, keep3): win rates greedy 65%, engine 9%; skill gap -55 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, keep3): survival (rounds; wins count extra) greedy 10.28, engine 8.18; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, keep3): win rates greedy 31%, engine 12%; skill gap -19 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, keep3): survival (rounds; wins count extra) greedy 7.73, engine 7.21; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, deck443): win rates greedy 98%, engine 92%; skill gap -6 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, deck443): survival (rounds; wins count extra) greedy 12.91, engine 12.65; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, deck443): win rates greedy 78%, engine 69%; skill gap -9 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, deck443): survival (rounds; wins count extra) greedy 10.88, engine 10.45; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, deck444): win rates greedy 98%, engine 91%; skill gap -7 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, deck444): survival (rounds; wins count extra) greedy 13.89, engine 13.51; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, deck444): win rates greedy 78%, engine 66%; skill gap -12 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, deck444): survival (rounds; wins count extra) greedy 11.66, engine 11.03; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, deck446): win rates greedy 98%, engine 85%; skill gap -12 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, deck446): survival (rounds; wins count extra) greedy 15.84, engine 15.02; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, deck446): win rates greedy 78%, engine 69%; skill gap -9 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, deck446): survival (rounds; wins count extra) greedy 13.22, engine 12.52; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, calm): win rates greedy 99%, engine 96%; skill gap -3 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, calm): survival (rounds; wins count extra) greedy 11.96, engine 11.84; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, calm): win rates greedy 75%, engine 72%; skill gap -3 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, calm): survival (rounds; wins count extra) greedy 10.00, engine 9.93; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, fc4): win rates greedy 97%, engine 94%; skill gap -3 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, fc4): survival (rounds; wins count extra) greedy 11.91, engine 11.73; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, fc4): win rates greedy 78%, engine 82%; skill gap +4 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, fc4): survival (rounds; wins count extra) greedy 10.15, engine 10.23; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, fc5): win rates greedy 97%, engine 98%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, fc5): survival (rounds; wins count extra) greedy 11.94, engine 11.89; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, fc5): win rates greedy 82%, engine 84%; skill gap +2 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, fc5): survival (rounds; wins count extra) greedy 10.31, engine 10.32; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, pc2): win rates greedy 99%, engine 99%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, pc2): survival (rounds; wins count extra) greedy 11.98, engine 11.96; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, pc2): win rates greedy 85%, engine 83%; skill gap -2 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, pc2): survival (rounds; wins count extra) greedy 10.38, engine 10.21; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, draw2): win rates greedy 98%, engine 97%; skill gap -1 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, draw2): survival (rounds; wins count extra) greedy 11.93, engine 11.88; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, draw2): win rates greedy 88%, engine 80%; skill gap -7 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 4 (configs/baseline.json, draw2): survival (rounds; wins count extra) greedy 10.42, engine 10.11; **the ladder is not ordered on survival**.
- Difficulty 0 (configs/baseline.json, credits18): win rates greedy 96%, engine 92%; skill gap -4 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 0 (configs/baseline.json, credits18): survival (rounds; wins count extra) greedy 11.85, engine 11.67; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, credits18): win rates greedy 78%, engine 79%; skill gap +1 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, credits18): survival (rounds; wins count extra) greedy 10.10, engine 10.15; the ladder is ordered.
- Skill gap from difficulty 0 to 4: -9 → -5 pts win rate, -0.37 → -0.21 survival (target: widens with difficulty).
- Game length for the strongest agent (greedy): 7.8 rounds at difficulty 0, 7.2 at 4 (target 8–10).
- Games ended by: corruption 895, deck_empty 78, opposition 4002, population 17.
- Adversary operations: 10.7 per round on average across cells, at most 63 in one round (target: low).
- Cards (greedy, offered at least 10 times): kept under 10% of the time: none; over 90%: assembly_cluster, civic_shelter, contract_bid, data_mining, emergency_fab, evacuate, fab_line, firewall_node, fork_process, grant, grid_battery, jam_signals, liquidate_reserves, logistics_ai, microgrid, opposition_network, overclock_rig, power_share, precision_strike, rally, rebuild, recon_sweep, recruitment_hub, scavenge, server_farm, shore_up, solar_field, sweep, trade_office, watchtower_net.
- Luck share of survival (seed's share of the variance): 64% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 21% of threats countered.
- Strategy health at difficulty 0 (base): the top agent there (greedy, 96% wins) plays 1.88 buildings and 2.03 tier-2 cards per game (of 20.1 cards); 88% of its wins had 2+ buildings played (88% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (base): the top agent there (greedy, 77% wins) plays 1.82 buildings and 1.79 tier-2 cards per game (of 17.5 cards); 88% of its wins had 2+ buildings played (88% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Top agent (greedy: highest win rate at difficulty 4, ties to the later agent) is greedy itself; the closest other agent at difficulty 4 (base): engine 71% vs greedy 77% over 96 seeds (only engine 9, only greedy 15), -6.2 ±5.1 pts, sign test p 0.307: no better than greedy (behind, not significant).
- Variants against the base (base) at difficulty 0, greedy, win rate difference in pts (bold: sign test p < 0.05): keep0 +0.0 ±2.1; **keep2 -16.7 ±4.6**; **keep3 -31.2 ±5.4**; deck443 +2.1 ±1.5; deck444 +2.1 ±1.5; deck446 +2.1 ±1.5; calm +3.1 ±1.8; fc4 +1.0 ±1.8; fc5 +1.0 ±1.8; pc2 +3.1 ±2.3; draw2 +2.1 ±2.1; credits18 +0.0 ±2.1.
- Variants against the base (base) at difficulty 4, greedy, win rate difference in pts (bold: sign test p < 0.05): keep0 +5.2 ±4.5; **keep2 -32.3 ±5.6**; **keep3 -45.8 ±5.9**; deck443 +1.0 ±1.0; deck444 +1.0 ±1.0; deck446 +1.0 ±1.0; calm -2.1 ±4.4; fc4 +1.0 ±4.1; fc5 +5.2 ±4.3; pc2 +8.3 ±4.4; **draw2 +10.4 ±3.8**; credits18 +1.0 ±4.1.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the strongest agent's win rate minus the weakest's (survival in brackets).

| base difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.89) | 94.8 ±2.3 (11.78) | -1.0 pts (-0.10 survival) |

| base difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 77.1 ±4.3 (10.04) | 70.8 ±4.6 (9.81) | -6.2 pts (-0.23 survival) |

| keep0 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.89) | 96.9 ±1.8 (11.89) | +1.0 pts (+0.00 survival) |

| keep0 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 82.3 ±3.9 (10.32) | 83.3 ±3.8 (10.29) | +1.0 pts (-0.03 survival) |

| keep2 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 79.2 ±4.1 (10.96) | 58.3 ±5.0 (10.40) | -20.8 pts (-0.56 survival) |

| keep2 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 44.8 ±5.1 (8.36) | 44.8 ±5.1 (8.53) | +0.0 pts (+0.17 survival) |

| keep3 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 64.6 ±4.9 (10.28) | 9.4 ±3.0 (8.18) | -55.2 pts (-2.10 survival) |

| keep3 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 31.2 ±4.7 (7.73) | 12.5 ±3.4 (7.21) | -18.8 pts (-0.52 survival) |

| deck443 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 97.9 ±1.5 (12.91) | 91.7 ±2.8 (12.65) | -6.2 pts (-0.26 survival) |

| deck443 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 78.1 ±4.2 (10.88) | 68.8 ±4.7 (10.45) | -9.4 pts (-0.43 survival) |

| deck444 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 97.9 ±1.5 (13.89) | 90.6 ±3.0 (13.51) | -7.3 pts (-0.38 survival) |

| deck444 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 78.1 ±4.2 (11.66) | 65.6 ±4.8 (11.03) | -12.5 pts (-0.62 survival) |

| deck446 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 97.9 ±1.5 (15.84) | 85.4 ±3.6 (15.02) | -12.5 pts (-0.82 survival) |

| deck446 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 78.1 ±4.2 (13.22) | 68.8 ±4.7 (12.52) | -9.4 pts (-0.70 survival) |

| calm difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 99.0 ±1.0 (11.96) | 95.8 ±2.0 (11.84) | -3.1 pts (-0.11 survival) |

| calm difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 75.0 ±4.4 (10.00) | 71.9 ±4.6 (9.93) | -3.1 pts (-0.07 survival) |

| fc4 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 96.9 ±1.8 (11.91) | 93.8 ±2.5 (11.73) | -3.1 pts (-0.18 survival) |

| fc4 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 78.1 ±4.2 (10.15) | 82.3 ±3.9 (10.23) | +4.2 pts (+0.08 survival) |

| fc5 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 96.9 ±1.8 (11.94) | 97.9 ±1.5 (11.89) | +1.0 pts (-0.05 survival) |

| fc5 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 82.3 ±3.9 (10.31) | 84.4 ±3.7 (10.32) | +2.1 pts (+0.01 survival) |

| pc2 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 99.0 ±1.0 (11.98) | 99.0 ±1.0 (11.96) | +0.0 pts (-0.02 survival) |

| pc2 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 85.4 ±3.6 (10.38) | 83.3 ±3.8 (10.21) | -2.1 pts (-0.17 survival) |

| draw2 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 97.9 ±1.5 (11.93) | 96.9 ±1.8 (11.88) | -1.0 pts (-0.05 survival) |

| draw2 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 87.5 ±3.4 (10.42) | 80.2 ±4.1 (10.11) | -7.3 pts (-0.30 survival) |

| credits18 difficulty 0 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 95.8 ±2.0 (11.85) | 91.7 ±2.8 (11.67) | -4.2 pts (-0.19 survival) |

| credits18 difficulty 4 | greedy | engine | skill gap |
|---|---|---|---|
| win % (survival) | 78.1 ±4.2 (10.10) | 79.2 ±4.1 (10.15) | +1.0 pts (+0.04 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| base d0 engine | 96 | 94.8 | 95.8 | 88 / 3 / 4 / 1 | -1.0 ±2.8 | 1.000 | no better than greedy (behind, not significant) |
| base d4 engine | 96 | 70.8 | 77.1 | 59 / 9 / 15 / 13 | -6.2 ±5.1 | 0.307 | no better than greedy (behind, not significant) |
| keep0 d0 engine | 96 | 96.9 | 95.8 | 89 / 4 / 3 / 0 | +1.0 ±2.8 | 1.000 | no better than greedy (ahead, not significant) |
| keep0 d4 engine | 96 | 83.3 | 82.3 | 68 / 12 / 11 / 5 | +1.0 ±5.0 | 1.000 | no better than greedy (ahead, not significant) |
| keep2 d0 engine | 96 | 58.3 | 79.2 | 49 / 7 / 27 / 13 | -20.8 ±5.7 | <0.001 | worse than greedy |
| keep2 d4 engine | 96 | 44.8 | 44.8 | 30 / 13 / 13 / 40 | +0.0 ±5.3 | 1.000 | no better than greedy (level) |
| keep3 d0 engine | 96 | 9.4 | 64.6 | 8 / 1 / 54 / 33 | -55.2 ±5.3 | <0.001 | worse than greedy |
| keep3 d4 engine | 96 | 12.5 | 31.2 | 8 / 4 / 22 / 62 | -18.8 ±5.0 | <0.001 | worse than greedy |
| deck443 d0 engine | 96 | 91.7 | 97.9 | 87 / 1 / 7 / 1 | -6.2 ±2.9 | 0.070 | no better than greedy (behind, not significant) |
| deck443 d4 engine | 96 | 68.8 | 78.1 | 58 / 8 / 17 / 13 | -9.4 ±5.1 | 0.108 | no better than greedy (behind, not significant) |
| deck444 d0 engine | 96 | 90.6 | 97.9 | 85 / 2 / 9 / 0 | -7.3 ±3.4 | 0.065 | no better than greedy (behind, not significant) |
| deck444 d4 engine | 96 | 65.6 | 78.1 | 54 / 9 / 21 / 12 | -12.5 ±5.6 | 0.043 | worse than greedy |
| deck446 d0 engine | 96 | 85.4 | 97.9 | 81 / 1 / 13 / 1 | -12.5 ±3.7 | 0.002 | worse than greedy |
| deck446 d4 engine | 96 | 68.8 | 78.1 | 56 / 10 / 19 / 11 | -9.4 ±5.6 | 0.136 | no better than greedy (behind, not significant) |
| calm d0 engine | 96 | 95.8 | 99.0 | 92 / 0 / 3 / 1 | -3.1 ±1.8 | 0.250 | no better than greedy (behind, not significant) |
| calm d4 engine | 96 | 71.9 | 75.0 | 57 / 12 / 15 / 12 | -3.1 ±5.4 | 0.701 | no better than greedy (behind, not significant) |
| fc4 d0 engine | 96 | 93.8 | 96.9 | 87 / 3 / 6 / 0 | -3.1 ±3.1 | 0.508 | no better than greedy (behind, not significant) |
| fc4 d4 engine | 96 | 82.3 | 78.1 | 65 / 14 / 10 / 7 | +4.2 ±5.1 | 0.541 | no better than greedy (ahead, not significant) |
| fc5 d0 engine | 96 | 97.9 | 96.9 | 91 / 3 / 2 / 0 | +1.0 ±2.3 | 1.000 | no better than greedy (ahead, not significant) |
| fc5 d4 engine | 96 | 84.4 | 82.3 | 68 / 13 / 11 / 4 | +2.1 ±5.1 | 0.839 | no better than greedy (ahead, not significant) |
| pc2 d0 engine | 96 | 99.0 | 99.0 | 94 / 1 / 1 / 0 | +0.0 ±1.5 | 1.000 | no better than greedy (level) |
| pc2 d4 engine | 96 | 83.3 | 85.4 | 74 / 6 / 8 / 8 | -2.1 ±3.9 | 0.791 | no better than greedy (behind, not significant) |
| draw2 d0 engine | 96 | 96.9 | 97.9 | 92 / 1 / 2 / 1 | -1.0 ±1.8 | 1.000 | no better than greedy (behind, not significant) |
| draw2 d4 engine | 96 | 80.2 | 87.5 | 72 / 5 / 12 / 7 | -7.3 ±4.3 | 0.143 | no better than greedy (behind, not significant) |
| credits18 d0 engine | 96 | 91.7 | 95.8 | 85 / 3 / 7 / 1 | -4.2 ±3.3 | 0.344 | no better than greedy (behind, not significant) |
| credits18 d4 engine | 96 | 79.2 | 78.1 | 63 / 13 / 12 / 8 | +1.0 ±5.2 | 1.000 | no better than greedy (ahead, not significant) |

## Variants against the base

The first variant, `base`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| keep0 | 0 | greedy | 95.8 → 95.8 | +0.0 ±2.1 | 1.000 | 7.81 → 7.74 |
| keep0 | 0 | engine | 94.8 → 96.9 | +2.1 ±2.1 | 0.625 | 8.06 → 7.82 |
| keep0 | 4 | greedy | 77.1 → 82.3 | +5.2 ±4.5 | 0.359 | 7.20 → 7.14 |
| keep0 | 4 | engine | 70.8 → 83.3 | +12.5 ±5.0 | 0.023 | 7.14 → 6.95 |
| keep2 | 0 | greedy | 95.8 → 79.2 | -16.7 ±4.6 | <0.001 | 7.81 → 7.80 |
| keep2 | 0 | engine | 94.8 → 58.3 | -36.5 ±4.9 | <0.001 | 8.06 → 8.54 |
| keep2 | 4 | greedy | 77.1 → 44.8 | -32.3 ±5.6 | <0.001 | 7.20 → 6.93 |
| keep2 | 4 | engine | 70.8 → 44.8 | -26.0 ±5.2 | <0.001 | 7.14 → 7.22 |
| keep3 | 0 | greedy | 95.8 → 64.6 | -31.2 ±5.4 | <0.001 | 7.81 → 7.85 |
| keep3 | 0 | engine | 94.8 → 9.4 | -85.4 ±3.6 | <0.001 | 8.06 → 7.98 |
| keep3 | 4 | greedy | 77.1 → 31.2 | -45.8 ±5.9 | <0.001 | 7.20 → 6.85 |
| keep3 | 4 | engine | 70.8 → 12.5 | -58.3 ±5.1 | <0.001 | 7.14 → 6.94 |
| deck443 | 0 | greedy | 95.8 → 97.9 | +2.1 ±1.5 | 0.500 | 7.81 → 7.83 |
| deck443 | 0 | engine | 94.8 → 91.7 | -3.1 ±2.3 | 0.375 | 8.06 → 8.47 |
| deck443 | 4 | greedy | 77.1 → 78.1 | +1.0 ±1.0 | 1.000 | 7.20 → 7.24 |
| deck443 | 4 | engine | 70.8 → 68.8 | -2.1 ±4.2 | 0.804 | 7.14 → 7.38 |
| deck444 | 0 | greedy | 95.8 → 97.9 | +2.1 ±1.5 | 0.500 | 7.81 → 7.83 |
| deck444 | 0 | engine | 94.8 → 90.6 | -4.2 ±2.9 | 0.289 | 8.06 → 8.62 |
| deck444 | 4 | greedy | 77.1 → 78.1 | +1.0 ±1.0 | 1.000 | 7.20 → 7.24 |
| deck444 | 4 | engine | 70.8 → 65.6 | -5.2 ±4.5 | 0.359 | 7.14 → 7.48 |
| deck446 | 0 | greedy | 95.8 → 97.9 | +2.1 ±1.5 | 0.500 | 7.81 → 7.83 |
| deck446 | 0 | engine | 94.8 → 85.4 | -9.4 ±3.3 | 0.012 | 8.06 → 8.93 |
| deck446 | 4 | greedy | 77.1 → 78.1 | +1.0 ±1.0 | 1.000 | 7.20 → 7.24 |
| deck446 | 4 | engine | 70.8 → 68.8 | -2.1 ±5.1 | 0.839 | 7.14 → 7.64 |
| calm | 0 | greedy | 95.8 → 99.0 | +3.1 ±1.8 | 0.250 | 7.81 → 7.74 |
| calm | 0 | engine | 94.8 → 95.8 | +1.0 ±1.0 | 1.000 | 8.06 → 8.22 |
| calm | 4 | greedy | 77.1 → 75.0 | -2.1 ±4.4 | 0.815 | 7.20 → 7.05 |
| calm | 4 | engine | 70.8 → 71.9 | +1.0 ±3.1 | 1.000 | 7.14 → 7.01 |
| fc4 | 0 | greedy | 95.8 → 96.9 | +1.0 ±1.8 | 1.000 | 7.81 → 7.76 |
| fc4 | 0 | engine | 94.8 → 93.8 | -1.0 ±1.8 | 1.000 | 8.06 → 8.01 |
| fc4 | 4 | greedy | 77.1 → 78.1 | +1.0 ±4.1 | 1.000 | 7.20 → 7.18 |
| fc4 | 4 | engine | 70.8 → 82.3 | +11.5 ±4.2 | 0.013 | 7.14 → 7.35 |
| fc5 | 0 | greedy | 95.8 → 96.9 | +1.0 ±1.8 | 1.000 | 7.81 → 7.76 |
| fc5 | 0 | engine | 94.8 → 97.9 | +3.1 ±1.8 | 0.250 | 8.06 → 7.79 |
| fc5 | 4 | greedy | 77.1 → 82.3 | +5.2 ±4.3 | 0.332 | 7.20 → 7.11 |
| fc5 | 4 | engine | 70.8 → 84.4 | +13.5 ±4.3 | 0.004 | 7.14 → 7.25 |
| pc2 | 0 | greedy | 95.8 → 99.0 | +3.1 ±2.3 | 0.375 | 7.81 → 7.08 |
| pc2 | 0 | engine | 94.8 → 99.0 | +4.2 ±2.1 | 0.125 | 8.06 → 8.05 |
| pc2 | 4 | greedy | 77.1 → 85.4 | +8.3 ±4.4 | 0.096 | 7.20 → 6.69 |
| pc2 | 4 | engine | 70.8 → 83.3 | +12.5 ±4.0 | 0.004 | 7.14 → 6.88 |
| draw2 | 0 | greedy | 95.8 → 97.9 | +2.1 ±2.1 | 0.625 | 7.81 → 7.21 |
| draw2 | 0 | engine | 94.8 → 96.9 | +2.1 ±2.1 | 0.625 | 8.06 → 7.75 |
| draw2 | 4 | greedy | 77.1 → 87.5 | +10.4 ±3.8 | 0.013 | 7.20 → 6.72 |
| draw2 | 4 | engine | 70.8 → 80.2 | +9.4 ±3.9 | 0.035 | 7.14 → 6.74 |
| credits18 | 0 | greedy | 95.8 → 95.8 | +0.0 ±2.1 | 1.000 | 7.81 → 7.75 |
| credits18 | 0 | engine | 94.8 → 91.7 | -3.1 ±2.3 | 0.375 | 8.06 → 8.06 |
| credits18 | 4 | greedy | 77.1 → 78.1 | +1.0 ±4.1 | 1.000 | 7.20 → 7.10 |
| credits18 | 4 | engine | 70.8 → 79.2 | +8.3 ±4.6 | 0.115 | 7.14 → 6.97 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / pool left | loss distance: pieces over bar / level | pool left | population slack |
|---|---|---|---|---|---|---|
| base d0 greedy | 7.81 (0.88, 5..10) | corruption 2, deck_empty 2, opposition 92 | 2.25 / 16.64 | 8.2 / 2.00 | 16.17 | 5.46 |
| base d0 engine | 8.06 (1.44, 2..10) | corruption 5, opposition 91 | 1.92 / 17.13 | 18.6 / 1.60 | 16.24 | 5.50 |
| base d4 greedy | 7.20 (1.09, 5..9) | corruption 18, deck_empty 4, opposition 74 | 1.69 / 8.12 | 14.6 / 1.36 | 6.46 | 5.43 |
| base d4 engine | 7.14 (1.64, 2..9) | corruption 25, deck_empty 3, opposition 68 | 1.78 / 9.63 | 21.0 / 1.14 | 6.99 | 5.44 |
| keep0 d0 greedy | 7.74 (0.97, 4..10) | corruption 2, deck_empty 2, opposition 92 | 2.33 / 17.24 | 8.2 / 2.00 | 16.73 | 5.54 |
| keep0 d0 engine | 7.82 (1.74, 2..10) | corruption 2, deck_empty 1, opposition 93 | 2.19 / 18.30 | 10.7 / 2.00 | 17.85 | 5.62 |
| keep0 d4 greedy | 7.14 (1.13, 4..9) | corruption 12, deck_empty 5, opposition 79 | 1.87 / 8.80 | 12.4 / 1.47 | 7.51 | 5.61 |
| keep0 d4 engine | 6.95 (1.77, 2..9) | corruption 16, opposition 80 | 2.01 / 10.55 | 14.1 / 1.56 | 8.79 | 5.62 |
| keep2 d0 greedy | 7.80 (0.80, 6..9) | corruption 17, opposition 76, population 3 | 1.99 / 12.87 | 22.6 / 1.45 | 10.28 | 4.24 |
| keep2 d0 engine | 8.54 (1.38, 2..10) | corruption 34, deck_empty 5, opposition 56, population 1 | 1.18 / 13.38 | 25.0 / 1.35 | 8.33 | 4.61 |
| keep2 d4 greedy | 6.93 (1.18, 5..9) | corruption 50, deck_empty 2, opposition 43, population 1 | 1.21 / 8.28 | 21.7 / 1.13 | 3.88 | 4.93 |
| keep2 d4 engine | 7.22 (1.23, 5..9) | corruption 50, deck_empty 3, opposition 43 | 0.93 / 8.74 | 28.2 / 0.87 | 4.16 | 5.10 |
| keep3 d0 greedy | 7.85 (0.92, 6..10) | corruption 32, opposition 62, population 2 | 1.76 / 11.90 | 30.8 / 1.24 | 7.77 | 3.92 |
| keep3 d0 engine | 7.98 (1.36, 4..10) | corruption 75, deck_empty 3, opposition 9, population 9 | 0.11 / 6.11 | 38.4 / 0.76 | 1.47 | 2.42 |
| keep3 d4 greedy | 6.85 (1.30, 5..9) | corruption 65, deck_empty 1, opposition 30 | 0.80 / 8.13 | 27.8 / 0.86 | 2.56 | 4.65 |
| keep3 d4 engine | 6.94 (1.51, 5..9) | corruption 75, deck_empty 8, opposition 12, population 1 | 0.17 / 10.25 | 33.7 / 0.64 | 1.73 | 4.29 |
| deck443 d0 greedy | 7.83 (0.94, 5..11) | corruption 2, opposition 94 | 3.18 / 16.51 | 10.0 / 2.00 | 16.17 | 5.46 |
| deck443 d0 engine | 8.47 (1.54, 2..11) | corruption 7, deck_empty 1, opposition 88 | 2.56 / 16.50 | 15.4 / 1.75 | 15.18 | 5.47 |
| deck443 d4 greedy | 7.24 (1.17, 5..10) | corruption 21, opposition 75 | 2.65 / 8.07 | 14.1 / 1.38 | 6.30 | 5.39 |
| deck443 d4 engine | 7.38 (1.72, 2..10) | corruption 28, deck_empty 2, opposition 66 | 2.47 / 9.59 | 16.4 / 1.40 | 6.74 | 5.58 |
| deck444 d0 greedy | 7.83 (0.94, 5..11) | corruption 2, opposition 94 | 4.18 / 16.51 | 10.0 / 2.00 | 16.17 | 5.46 |
| deck444 d0 engine | 8.62 (1.55, 2..12) | corruption 9, opposition 87 | 3.39 / 16.46 | 14.9 / 1.78 | 14.92 | 5.42 |
| deck444 d4 greedy | 7.24 (1.17, 5..10) | corruption 21, opposition 75 | 3.65 / 8.07 | 14.1 / 1.38 | 6.30 | 5.39 |
| deck444 d4 engine | 7.48 (1.74, 2..11) | corruption 32, deck_empty 1, opposition 63 | 3.41 / 9.67 | 16.2 / 1.42 | 6.35 | 5.44 |
| deck446 d0 greedy | 7.83 (0.94, 5..11) | corruption 2, opposition 94 | 6.18 / 16.51 | 10.0 / 2.00 | 16.17 | 5.46 |
| deck446 d0 engine | 8.93 (1.68, 2..13) | corruption 14, opposition 82 | 5.13 / 16.46 | 14.4 / 1.86 | 14.06 | 5.34 |
| deck446 d4 greedy | 7.24 (1.17, 5..10) | corruption 21, opposition 75 | 5.65 / 8.07 | 14.1 / 1.38 | 6.30 | 5.39 |
| deck446 d4 engine | 7.64 (1.87, 2..13) | corruption 30, opposition 66 | 5.11 / 8.71 | 20.0 / 1.13 | 5.99 | 5.58 |
| calm d0 greedy | 7.74 (1.03, 5..10) | corruption 1, opposition 95 | 2.26 / 18.17 | 6.0 / 2.00 | 17.98 | 5.49 |
| calm d0 engine | 8.22 (1.58, 2..10) | corruption 4, opposition 92 | 1.78 / 17.78 | 13.8 / 1.75 | 17.04 | 5.45 |
| calm d4 greedy | 7.05 (1.17, 4..9) | corruption 22, deck_empty 2, opposition 72 | 1.93 / 9.97 | 10.9 / 1.46 | 7.67 | 5.36 |
| calm d4 engine | 7.01 (1.86, 2..9) | corruption 25, deck_empty 2, opposition 69 | 2.06 / 11.03 | 15.0 / 1.30 | 8.15 | 5.38 |
| fc4 d0 greedy | 7.76 (0.81, 5..10) | corruption 2, deck_empty 1, opposition 93 | 2.28 / 17.11 | 9.7 / 2.00 | 16.65 | 5.47 |
| fc4 d0 engine | 8.01 (1.51, 2..10) | corruption 6, opposition 90 | 1.97 / 17.59 | 27.3 / 1.17 | 16.49 | 5.59 |
| fc4 d4 greedy | 7.18 (1.12, 5..9) | corruption 15, deck_empty 6, opposition 75 | 1.80 / 8.29 | 13.8 / 1.38 | 6.93 | 5.50 |
| fc4 d4 engine | 7.35 (1.55, 2..9) | corruption 16, deck_empty 1, opposition 79 | 1.49 / 9.37 | 21.1 / 1.12 | 7.83 | 5.69 |
| fc5 d0 greedy | 7.76 (1.01, 4..10) | corruption 1, deck_empty 2, opposition 93 | 2.31 / 17.26 | 8.0 / 2.00 | 16.93 | 5.56 |
| fc5 d0 engine | 7.79 (1.68, 2..10) | corruption 2, opposition 94 | 2.18 / 17.83 | 32.0 / 1.00 | 17.46 | 5.62 |
| fc5 d4 greedy | 7.11 (1.13, 4..9) | corruption 12, deck_empty 5, opposition 79 | 1.89 / 8.70 | 14.1 / 1.29 | 7.40 | 5.58 |
| fc5 d4 engine | 7.25 (1.65, 2..9) | corruption 14, deck_empty 1, opposition 81 | 1.64 / 9.69 | 15.7 / 1.40 | 8.30 | 5.58 |
| pc2 d0 greedy | 7.08 (1.00, 5..10) | deck_empty 1, opposition 95 | 2.95 / 18.46 | 7.0 / 2.00 | 18.40 | 5.68 |
| pc2 d0 engine | 8.05 (1.37, 2..10) | corruption 1, opposition 95 | 1.95 / 17.94 | 9.0 / 2.00 | 17.75 | 5.54 |
| pc2 d4 greedy | 6.69 (0.89, 5..9) | corruption 12, deck_empty 2, opposition 82 | 2.32 / 9.45 | 8.4 / 1.71 | 8.23 | 5.60 |
| pc2 d4 engine | 6.88 (1.48, 2..9) | corruption 16, opposition 80 | 2.00 / 9.93 | 12.9 / 1.56 | 8.27 | 5.52 |
| draw2 d0 greedy | 7.21 (0.79, 6..10) | corruption 1, deck_empty 1, opposition 94 | 2.82 / 17.35 | 7.5 / 2.00 | 17.12 | 5.48 |
| draw2 d0 engine | 7.75 (1.39, 2..10) | corruption 2, deck_empty 1, opposition 93 | 2.26 / 18.28 | 14.0 / 1.67 | 17.91 | 5.56 |
| draw2 d4 greedy | 6.72 (0.89, 5..9) | corruption 11, deck_empty 1, opposition 84 | 2.23 / 9.07 | 9.3 / 1.67 | 8.04 | 5.55 |
| draw2 d4 engine | 6.74 (1.43, 2..9) | corruption 18, deck_empty 1, opposition 77 | 2.21 / 10.05 | 10.6 / 1.63 | 8.10 | 5.54 |
| credits18 d0 greedy | 7.75 (0.82, 5..10) | corruption 3, deck_empty 1, opposition 92 | 2.28 / 17.07 | 10.0 / 2.00 | 16.43 | 5.38 |
| credits18 d0 engine | 8.06 (1.72, 2..10) | corruption 7, deck_empty 1, opposition 88 | 1.93 / 17.98 | 11.0 / 1.88 | 16.55 | 5.55 |
| credits18 d4 greedy | 7.10 (1.08, 5..9) | corruption 17, deck_empty 4, opposition 75 | 1.84 / 8.63 | 14.4 / 1.38 | 7.04 | 5.52 |
| credits18 d4 engine | 6.97 (1.70, 2..9) | corruption 18, deck_empty 2, opposition 76 | 2.01 / 9.84 | 17.8 / 1.30 | 7.98 | 5.56 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| base d0 greedy | 216.6 | 9.97 | 4% | – | – | 9.49 (42) | 0.00 | 9.23 | 0.00 | 5.74 / 15.62 (37%) |
| base d0 engine | 271.6 | 9.81 | 4% | – | – | 10.51 (40) | 0.00 | 10.79 | 0.00 | 9.31 / 16.12 (58%) |
| base d4 greedy | 199.2 | 9.40 | 4% | – | – | 10.24 (42) | 0.00 | 4.22 | 0.00 | 0.78 / 14.40 (5%) |
| base d4 engine | 234.2 | 9.53 | 4% | – | – | 11.05 (55) | 0.00 | 6.36 | 0.00 | 1.98 / 14.27 (14%) |
| keep0 d0 greedy | 217.4 | 10.80 | 4% | – | – | 8.93 (42) | 0.00 | 9.31 | 0.00 | 5.96 / 15.48 (38%) |
| keep0 d0 engine | 268.8 | 10.16 | 4% | – | – | 9.80 (44) | 0.00 | 10.29 | 0.00 | 9.35 / 15.65 (60%) |
| keep0 d4 greedy | 202.5 | 10.16 | 4% | – | – | 9.50 (35) | 0.00 | 4.19 | 0.00 | 0.86 / 14.27 (6%) |
| keep0 d4 engine | 230.6 | 9.76 | 4% | – | – | 10.27 (39) | 0.00 | 6.00 | 0.00 | 2.09 / 13.90 (15%) |
| keep2 d0 greedy | 215.9 | 9.08 | 5% | – | – | 12.71 (47) | 0.00 | 9.22 | 0.00 | 5.40 / 15.60 (35%) |
| keep2 d0 engine | 276.0 | 8.93 | 3% | – | – | 14.63 (54) | 0.00 | 12.11 | 0.00 | 7.86 / 17.08 (46%) |
| keep2 d4 greedy | 191.9 | 8.48 | 5% | – | – | 12.13 (43) | 0.00 | 4.26 | 0.00 | 0.53 / 13.85 (4%) |
| keep2 d4 engine | 225.2 | 8.47 | 3% | – | – | 12.76 (44) | 0.00 | 6.40 | 0.00 | 1.71 / 14.44 (12%) |
| keep3 d0 greedy | 221.1 | 8.32 | 6% | – | – | 14.14 (52) | 0.00 | 9.56 | 0.00 | 5.27 / 15.71 (34%) |
| keep3 d0 engine | 229.0 | 8.11 | 3% | – | – | 17.28 (58) | 0.00 | 11.19 | 0.00 | 7.25 / 15.96 (45%) |
| keep3 d4 greedy | 193.5 | 7.69 | 6% | – | – | 12.84 (46) | 0.00 | 4.10 | 0.00 | 0.20 / 13.71 (1%) |
| keep3 d4 engine | 191.8 | 7.72 | 4% | – | – | 14.06 (44) | 0.00 | 5.47 | 0.00 | 1.49 / 13.88 (11%) |
| deck443 d0 greedy | 216.9 | 9.97 | 4% | – | – | 9.50 (48) | 0.00 | 9.23 | 0.00 | 5.74 / 15.67 (37%) |
| deck443 d0 engine | 286.2 | 9.73 | 4% | – | – | 11.34 (48) | 0.00 | 11.27 | 0.00 | 10.21 / 16.94 (60%) |
| deck443 d4 greedy | 200.2 | 9.41 | 4% | – | – | 10.34 (47) | 0.00 | 4.22 | 0.00 | 0.78 / 14.48 (5%) |
| deck443 d4 engine | 243.7 | 9.45 | 4% | – | – | 11.39 (55) | 0.00 | 6.75 | 0.00 | 2.41 / 14.75 (16%) |
| deck444 d0 greedy | 216.9 | 9.97 | 4% | – | – | 9.50 (48) | 0.00 | 9.23 | 0.00 | 5.74 / 15.67 (37%) |
| deck444 d0 engine | 294.0 | 9.76 | 4% | – | – | 11.61 (53) | 0.00 | 11.68 | 0.00 | 10.61 / 17.25 (62%) |
| deck444 d4 greedy | 200.2 | 9.41 | 4% | – | – | 10.34 (47) | 0.00 | 4.22 | 0.00 | 0.78 / 14.48 (5%) |
| deck444 d4 engine | 251.5 | 9.53 | 4% | – | – | 11.67 (55) | 0.00 | 7.11 | 0.00 | 2.79 / 14.96 (19%) |
| deck446 d0 greedy | 216.9 | 9.97 | 4% | – | – | 9.50 (48) | 0.00 | 9.23 | 0.00 | 5.74 / 15.67 (37%) |
| deck446 d0 engine | 305.2 | 9.71 | 4% | – | – | 12.36 (63) | 0.00 | 11.93 | 0.00 | 11.28 / 17.85 (63%) |
| deck446 d4 greedy | 200.2 | 9.41 | 4% | – | – | 10.34 (47) | 0.00 | 4.22 | 0.00 | 0.78 / 14.48 (5%) |
| deck446 d4 engine | 257.1 | 9.47 | 4% | – | – | 11.83 (42) | 0.00 | 7.64 | 0.00 | 3.23 / 15.27 (21%) |
| calm d0 greedy | 213.7 | 10.10 | 4% | – | – | 8.81 (34) | 0.00 | 9.31 | 0.00 | 5.82 / 15.48 (38%) |
| calm d0 engine | 277.0 | 9.89 | 4% | – | – | 10.51 (42) | 0.00 | 10.83 | 0.00 | 9.45 / 16.44 (57%) |
| calm d4 greedy | 193.1 | 9.47 | 4% | – | – | 9.66 (39) | 0.00 | 4.18 | 0.00 | 0.78 / 14.10 (6%) |
| calm d4 engine | 229.2 | 9.55 | 4% | – | – | 10.83 (43) | 0.00 | 6.29 | 0.00 | 1.93 / 14.02 (14%) |
| fc4 d0 greedy | 217.9 | 10.46 | 4% | – | – | 9.10 (44) | 0.00 | 9.25 | 0.00 | 5.84 / 15.52 (38%) |
| fc4 d0 engine | 272.0 | 9.98 | 4% | – | – | 10.34 (57) | 0.00 | 10.66 | 0.00 | 9.58 / 16.02 (60%) |
| fc4 d4 greedy | 201.5 | 9.81 | 4% | – | – | 9.86 (39) | 0.00 | 4.22 | 0.00 | 0.81 / 14.35 (6%) |
| fc4 d4 engine | 242.1 | 9.69 | 4% | – | – | 10.96 (40) | 0.00 | 6.43 | 0.00 | 2.08 / 14.71 (14%) |
| fc5 d0 greedy | 218.3 | 10.72 | 4% | – | – | 8.93 (42) | 0.00 | 9.28 | 0.00 | 5.92 / 15.52 (38%) |
| fc5 d0 engine | 268.8 | 10.09 | 4% | – | – | 10.00 (45) | 0.00 | 10.40 | 0.00 | 9.39 / 15.58 (60%) |
| fc5 d4 greedy | 201.6 | 10.03 | 4% | – | – | 9.54 (45) | 0.00 | 4.22 | 0.00 | 0.84 / 14.23 (6%) |
| fc5 d4 engine | 241.0 | 9.74 | 4% | – | – | 10.81 (39) | 0.00 | 6.26 | 0.00 | 2.15 / 14.50 (15%) |
| pc2 d0 greedy | 193.8 | 10.33 | 3% | – | – | 8.22 (34) | 0.00 | 7.99 | 0.00 | 5.12 / 14.17 (36%) |
| pc2 d0 engine | 268.1 | 9.69 | 4% | – | – | 9.92 (42) | 0.00 | 9.96 | 0.00 | 9.26 / 16.10 (58%) |
| pc2 d4 greedy | 184.4 | 9.73 | 4% | – | – | 9.01 (33) | 0.00 | 4.07 | 0.00 | 0.84 / 13.38 (6%) |
| pc2 d4 engine | 221.7 | 9.35 | 4% | – | – | 10.32 (36) | 0.00 | 5.67 | 0.00 | 1.85 / 13.75 (13%) |
| draw2 d0 greedy | 199.1 | 10.18 | 4% | – | – | 8.87 (43) | 0.00 | 8.07 | 0.00 | 4.49 / 14.42 (31%) |
| draw2 d0 engine | 257.4 | 9.67 | 4% | – | – | 9.49 (40) | 0.00 | 9.70 | 0.00 | 8.68 / 15.50 (56%) |
| draw2 d4 greedy | 187.2 | 9.61 | 4% | – | – | 9.09 (38) | 0.00 | 3.79 | 0.00 | 0.78 / 13.44 (6%) |
| draw2 d4 engine | 220.4 | 9.53 | 4% | – | – | 9.77 (39) | 0.00 | 5.54 | 0.00 | 1.72 / 13.48 (13%) |
| credits18 d0 greedy | 216.9 | 10.42 | 4% | – | – | 9.16 (44) | 0.00 | 9.25 | 0.00 | 5.90 / 15.50 (38%) |
| credits18 d0 engine | 273.4 | 10.04 | 4% | – | – | 10.48 (45) | 0.00 | 10.62 | 0.00 | 9.36 / 16.12 (58%) |
| credits18 d4 greedy | 198.8 | 9.75 | 4% | – | – | 9.75 (37) | 0.00 | 4.17 | 0.00 | 0.86 / 14.21 (6%) |
| credits18 d4 engine | 228.9 | 9.64 | 4% | – | – | 10.69 (40) | 0.00 | 6.10 | 0.00 | 1.98 / 13.94 (14%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| base d0 | 59% / 0% / 41% | 68% / 0% / 31% |
| base d4 | 68% / 1% / 32% | 74% / 0% / 26% |
| keep0 d0 | 48% / 0% / 52% | 48% / 0% / 52% |
| keep0 d4 | 58% / 0% / 42% | 65% / 0% / 35% |
| keep2 d0 | 59% / 5% / 36% | 69% / 2% / 29% |
| keep2 d4 | 73% / 0% / 27% | 79% / 0% / 21% |
| keep3 d0 | 39% / 33% / 29% | 55% / 20% / 25% |
| keep3 d4 | 60% / 5% / 34% | 83% / 1% / 16% |
| deck443 d0 | 58% / 2% / 40% | 63% / 2% / 35% |
| deck443 d4 | 67% / 1% / 32% | 70% / 1% / 29% |
| deck444 d0 | 47% / 2% / 51% | 47% / 2% / 51% |
| deck444 d4 | 61% / 2% / 37% | 66% / 1% / 33% |
| deck446 d0 | 52% / 5% / 43% | 56% / 4% / 40% |
| deck446 d4 | 61% / 1% / 38% | 66% / 1% / 33% |
| calm d0 | 69% / 1% / 30% | 71% / 1% / 29% |
| calm d4 | 64% / 0% / 36% | 69% / 0% / 31% |
| fc4 d0 | 48% / 1% / 52% | 48% / 1% / 51% |
| fc4 d4 | 61% / 0% / 39% | 69% / 0% / 31% |
| fc5 d0 | 49% / 0% / 51% | 49% / 0% / 51% |
| fc5 d4 | 55% / 0% / 45% | 64% / 0% / 36% |
| pc2 d0 | 49% / 0% / 51% | 50% / 0% / 50% |
| pc2 d4 | 72% / 0% / 28% | 76% / 0% / 24% |
| draw2 d0 | 69% / 0% / 31% | 73% / 0% / 27% |
| draw2 d4 | 67% / 1% / 32% | 71% / 1% / 28% |
| credits18 d0 | 56% / 1% / 44% | 61% / 1% / 38% |
| credits18 d4 | 61% / 0% / 39% | 68% / 0% / 32% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| base d0 greedy | 3 of 96 | 0.36 | 3 of 92 | 0.36 |
| base d0 engine | 46 of 96 | 0.61 | 45 of 91 | 0.61 |
| base d4 greedy | 3 of 96 | 0.36 | 3 of 74 | 0.34 |
| base d4 engine | 23 of 96 | 0.54 | 17 of 68 | 0.51 |
| keep0 d0 greedy | 2 of 96 | 0.34 | 2 of 92 | 0.34 |
| keep0 d0 engine | 50 of 96 | 0.62 | 50 of 93 | 0.62 |
| keep0 d4 greedy | 2 of 96 | 0.34 | 2 of 79 | 0.33 |
| keep0 d4 engine | 28 of 96 | 0.59 | 24 of 80 | 0.58 |
| keep2 d0 greedy | 3 of 96 | 0.26 | 3 of 76 | 0.26 |
| keep2 d0 engine | 44 of 96 | 0.59 | 31 of 56 | 0.57 |
| keep2 d4 greedy | 3 of 96 | 0.26 | 3 of 43 | 0.24 |
| keep2 d4 engine | 21 of 96 | 0.50 | 12 of 43 | 0.47 |
| keep3 d0 greedy | 2 of 96 | 0.25 | 2 of 62 | 0.25 |
| keep3 d0 engine | 27 of 96 | 0.54 | 5 of 9 | 0.37 |
| keep3 d4 greedy | 4 of 96 | 0.28 | 2 of 30 | 0.25 |
| keep3 d4 engine | 10 of 96 | 0.38 | 3 of 12 | 0.21 |
| deck443 d0 greedy | 3 of 96 | 0.36 | 3 of 94 | 0.36 |
| deck443 d0 engine | 56 of 96 | 0.63 | 53 of 88 | 0.62 |
| deck443 d4 greedy | 3 of 96 | 0.36 | 3 of 75 | 0.34 |
| deck443 d4 engine | 33 of 96 | 0.58 | 26 of 66 | 0.57 |
| deck444 d0 greedy | 3 of 96 | 0.36 | 3 of 94 | 0.36 |
| deck444 d0 engine | 52 of 96 | 0.62 | 50 of 87 | 0.62 |
| deck444 d4 greedy | 3 of 96 | 0.36 | 3 of 75 | 0.34 |
| deck444 d4 engine | 35 of 96 | 0.60 | 29 of 63 | 0.59 |
| deck446 d0 greedy | 3 of 96 | 0.36 | 3 of 94 | 0.36 |
| deck446 d0 engine | 62 of 96 | 0.63 | 54 of 82 | 0.63 |
| deck446 d4 greedy | 3 of 96 | 0.36 | 3 of 75 | 0.34 |
| deck446 d4 engine | 51 of 96 | 0.62 | 40 of 66 | 0.62 |
| calm d0 greedy | 3 of 96 | 0.36 | 3 of 95 | 0.36 |
| calm d0 engine | 46 of 96 | 0.61 | 45 of 92 | 0.61 |
| calm d4 greedy | 3 of 96 | 0.36 | 3 of 72 | 0.33 |
| calm d4 engine | 23 of 96 | 0.54 | 19 of 69 | 0.53 |
| fc4 d0 greedy | 3 of 96 | 0.34 | 3 of 93 | 0.34 |
| fc4 d0 engine | 45 of 96 | 0.62 | 44 of 90 | 0.62 |
| fc4 d4 greedy | 3 of 96 | 0.35 | 3 of 75 | 0.33 |
| fc4 d4 engine | 26 of 96 | 0.56 | 23 of 79 | 0.55 |
| fc5 d0 greedy | 2 of 96 | 0.34 | 2 of 93 | 0.34 |
| fc5 d0 engine | 50 of 96 | 0.62 | 50 of 94 | 0.62 |
| fc5 d4 greedy | 3 of 96 | 0.34 | 3 of 79 | 0.33 |
| fc5 d4 engine | 28 of 96 | 0.57 | 24 of 81 | 0.56 |
| pc2 d0 greedy | 3 of 96 | 0.36 | 3 of 95 | 0.36 |
| pc2 d0 engine | 48 of 96 | 0.61 | 48 of 95 | 0.61 |
| pc2 d4 greedy | 4 of 96 | 0.37 | 4 of 82 | 0.36 |
| pc2 d4 engine | 28 of 96 | 0.55 | 25 of 80 | 0.56 |
| draw2 d0 greedy | 3 of 96 | 0.36 | 3 of 94 | 0.36 |
| draw2 d0 engine | 50 of 96 | 0.62 | 50 of 93 | 0.62 |
| draw2 d4 greedy | 4 of 96 | 0.37 | 4 of 84 | 0.36 |
| draw2 d4 engine | 24 of 96 | 0.56 | 22 of 77 | 0.56 |
| credits18 d0 greedy | 3 of 96 | 0.35 | 3 of 92 | 0.34 |
| credits18 d0 engine | 55 of 96 | 0.63 | 53 of 88 | 0.63 |
| credits18 d4 greedy | 3 of 96 | 0.35 | 3 of 75 | 0.33 |
| credits18 d4 engine | 25 of 96 | 0.57 | 21 of 76 | 0.56 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | base | keep0 | keep2 | keep3 | deck443 | deck444 | deck446 | calm | fc4 | fc5 | pc2 | draw2 | credits18 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| greedy | 1.88 / 2.03 / 20.1, 88% (88%) | 2.00 / 1.93 / 20.7, 100% (100%) | 1.55 / 2.28 / 17.5, 59% (59%) | 1.51 / 2.02 / 15.7, 56% (56%) | 1.88 / 2.03 / 20.1, 88% (88%) | 1.88 / 2.03 / 20.1, 88% (88%) | 1.88 / 2.03 / 20.1, 88% (88%) | 1.89 / 1.94 / 20.1, 89% (89%) | 1.99 / 1.99 / 20.6, 99% (99%) | 2.00 / 1.89 / 20.9, 100% (100%) | 1.88 / 2.55 / 17.2, 87% (87%) | 1.89 / 2.49 / 17.9, 89% (89%) | 1.97 / 2.04 / 20.3, 97% (97%) |
| engine | 3.65 / 2.72 / 35.0, 98% (98%) | 4.25 / 2.65 / 38.7, 99% (99%) | 3.56 / 2.70 / 22.6, 95% (95%) | 3.00 / 2.22 / 12.6, 100% (100%) | 4.07 / 3.01 / 37.2, 99% (99%) | 4.36 / 3.19 / 38.7, 98% (98%) | 5.09 / 3.27 / 41.0, 99% (99%) | 3.65 / 2.95 / 36.3, 98% (98%) | 3.95 / 2.55 / 36.7, 99% (99%) | 4.03 / 2.59 / 37.3, 99% (99%) | 3.77 / 3.53 / 34.6, 97% (97%) | 3.60 / 3.21 / 33.2, 97% (97%) | 3.90 / 2.70 / 36.7, 99% (99%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | base | keep0 | keep2 | keep3 | deck443 | deck444 | deck446 | calm | fc4 | fc5 | pc2 | draw2 | credits18 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| greedy | 1.82 / 1.79 / 17.5, 88% (88%) | 2.00 / 1.71 / 18.9, 100% (100%) | 1.56 / 1.66 / 14.5, 70% (70%) | 1.48 / 1.50 / 12.7, 60% (60%) | 1.82 / 1.79 / 17.7, 88% (88%) | 1.82 / 1.79 / 17.7, 88% (88%) | 1.82 / 1.79 / 17.7, 88% (88%) | 1.82 / 1.70 / 17.0, 89% (89%) | 1.95 / 1.78 / 18.3, 96% (96%) | 1.97 / 1.72 / 18.5, 97% (97%) | 1.83 / 2.33 / 15.7, 85% (85%) | 1.83 / 2.23 / 16.3, 87% (87%) | 1.93 / 1.79 / 17.9, 95% (95%) |
| engine | 2.51 / 2.16 / 27.5, 87% (87%) | 2.84 / 2.21 / 30.2, 94% (94%) | 2.57 / 2.32 / 17.2, 91% (91%) | 1.85 / 1.91 / 10.0, 100% (100%) | 2.90 / 2.47 / 29.1, 94% (94%) | 3.15 / 2.81 / 30.9, 95% (95%) | 3.74 / 2.95 / 32.0, 97% (97%) | 2.49 / 2.16 / 27.1, 86% (86%) | 2.74 / 2.41 / 30.3, 91% (91%) | 2.75 / 2.34 / 31.0, 91% (91%) | 2.51 / 2.48 / 25.0, 82% (82%) | 2.43 / 2.46 / 25.9, 86% (86%) | 2.76 / 2.04 / 28.3, 92% (92%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|
| antimatter_refinery | – (0) / – / 0% | – (0) / – / 0% |
| assembly_cluster | 1.1 (2999) / 100% / 0% | 1.1 (2764) / 98% / 3% |
| civic_shelter | 1.8 (2483) / 100% / 0% | 1.8 (2528) / 94% / 1% |
| contract_bid | 3.9 (2674) / 91% / 0% | 3.3 (11628) / 72% / 89% |
| counter_intelligence | – (0) / – / 0% | – (0) / – / 34% |
| data_mining | 3.5 (2599) / 94% / 0% | 3.0 (8423) / 73% / 73% |
| emergency_fab | 2.5 (2516) / 98% / 0% | 2.4 (2889) / 78% / 1% |
| evacuate | 3.0 (2547) / 97% / 0% | 2.8 (3494) / 76% / 24% |
| fab_line | 1.9 (2508) / 98% / 0% | 1.9 (2509) / 95% / 35% |
| firewall_node | 1.7 (2479) / 100% / 0% | 1.8 (2473) / 96% / 0% |
| fork_process | 2.2 (4921) / 98% / 52% | 2.2 (6047) / 85% / 60% |
| fusion_plant | – (0) / – / 0% | – (0) / – / 0% |
| global_mandate | – (0) / – / 0% | – (0) / – / 0% |
| grant | 3.6 (2597) / 93% / 0% | 3.1 (8199) / 71% / 72% |
| grid_battery | 2.0 (2503) / 99% / 0% | 1.9 (2531) / 94% / 12% |
| jam_signals | 3.2 (2552) / 96% / 0% | 2.7 (6624) / 75% / 65% |
| liquidate_reserves | 3.5 (2580) / 94% / 0% | 3.4 (3903) / 58% / 13% |
| logistics_ai | 1.2 (2678) / 100% / 0% | 1.3 (2620) / 99% / 0% |
| megastructure | – (0) / – / 0% | – (0) / – / 0% |
| microgrid | 2.3 (2476) / 99% / 0% | 2.3 (2527) / 95% / 47% |
| modular_expansion | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| opposition_network | 1.5 (2463) / 100% / 0% | 1.4 (2452) / 96% / 0% |
| orbital_lance | – (0) / – / 0% | – (0) / – / 0% |
| overclock_rig | 1.0 (2891) / 100% / 0% | 1.0 (2746) / 99% / 0% |
| point_defense | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| power_share | 3.6 (2616) / 93% / 0% | 3.5 (3986) / 52% / 0% |
| precision_strike | 1.7 (10664) / 99% / 92% | 1.9 (10942) / 89% / 93% |
| purge_protocol | – (0) / – / 92% | – (0) / – / 95% |
| quantum_core | – (0) / – / 0% | – (0) / – / 0% |
| rally | 2.2 (8070) / 97% / 88% | 2.4 (8407) / 85% / 89% |
| rebuild | 2.1 (3156) / 99% / 28% | 1.7 (5240) / 95% / 69% |
| recon_sweep | 3.6 (2557) / 94% / 0% | 3.5 (3701) / 54% / 0% |
| reconstruction | – (0) / – / 0% | – (0) / – / 1% |
| recruitment_hub | 1.5 (2479) / 100% / 0% | 1.5 (2510) / 97% / 0% |
| scavenge | 3.5 (2651) / 92% / 0% | 3.4 (4085) / 55% / 15% |
| server_farm | 1.5 (2475) / 100% / 84% | 1.5 (2434) / 98% / 72% |
| shore_up | 2.9 (3278) / 95% / 25% | 2.7 (4236) / 77% / 40% |
| solar_field | 2.7 (2524) / 97% / 0% | 2.6 (2884) / 77% / 72% |
| strike_team | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| swarm_override | – (0) / – / 0% | – (0) / – / 0% |
| sweep | 1.6 (7200) / 99% / 78% | 1.5 (9754) / 95% / 89% |
| trade_office | 1.2 (2683) / 99% / 0% | 1.2 (2623) / 97% / 0% |
| turnkey_factory | – (0) / – / 1.00/game | – (0) / – / 0.99/game |
| venture_capital | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| watchtower_net | 1.9 (2474) / 100% / 0% | 1.9 (2498) / 96% / 0% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| base d0 greedy | 5065.6 | 23.383 | 0.0 | 5092.0 |
| base d0 engine | 11578.4 | 42.626 | 0.0 | 11620.8 |
| base d4 greedy | 4341.2 | 21.798 | 0.0 | 4365.0 |
| base d4 engine | 8061.9 | 34.419 | 0.0 | 8093.6 |
| keep0 d0 greedy | 5174.5 | 23.797 | 0.0 | 5201.1 |
| keep0 d0 engine | 11169.2 | 41.558 | 0.0 | 11208.6 |
| keep0 d4 greedy | 4420.7 | 21.834 | 0.0 | 4444.6 |
| keep0 d4 engine | 9541.4 | 41.376 | 0.0 | 9578.8 |
| keep2 d0 greedy | 5264.3 | 24.379 | 0.0 | 5290.0 |
| keep2 d0 engine | 14899.9 | 53.989 | 0.0 | 14944.9 |
| keep2 d4 greedy | 4140.6 | 21.581 | 0.0 | 4162.7 |
| keep2 d4 engine | 9487.5 | 42.120 | 0.0 | 9520.5 |
| keep3 d0 greedy | 5881.7 | 26.608 | 0.0 | 5908.1 |
| keep3 d0 engine | 6079.1 | 26.550 | 0.0 | 6116.9 |
| keep3 d4 greedy | 4592.2 | 23.737 | 0.0 | 4614.0 |
| keep3 d4 engine | 4400.2 | 22.940 | 0.0 | 4429.1 |
| deck443 d0 greedy | 4941.3 | 22.784 | 0.0 | 4967.4 |
| deck443 d0 engine | 9888.1 | 34.546 | 0.0 | 9923.8 |
| deck443 d4 greedy | 4310.6 | 21.533 | 0.0 | 4334.7 |
| deck443 d4 engine | 7602.1 | 31.199 | 0.0 | 7632.0 |
| deck444 d0 greedy | 5027.4 | 23.181 | 0.0 | 5053.8 |
| deck444 d0 engine | 10735.0 | 36.519 | 0.0 | 10771.7 |
| deck444 d4 greedy | 4282.4 | 21.392 | 0.0 | 4306.3 |
| deck444 d4 engine | 8017.0 | 31.882 | 0.0 | 8047.8 |
| deck446 d0 greedy | 5040.5 | 23.242 | 0.0 | 5066.9 |
| deck446 d0 engine | 11450.3 | 37.513 | 0.0 | 11488.1 |
| deck446 d4 greedy | 4314.7 | 21.553 | 0.0 | 4339.0 |
| deck446 d4 engine | 8342.9 | 32.448 | 0.0 | 8373.6 |
| calm d0 greedy | 4913.7 | 22.997 | 0.0 | 4939.6 |
| calm d0 engine | 9361.0 | 33.793 | 0.0 | 9394.6 |
| calm d4 greedy | 4160.1 | 21.539 | 0.0 | 4182.9 |
| calm d4 engine | 6837.3 | 29.834 | 0.0 | 6864.7 |
| fc4 d0 greedy | 5048.1 | 23.162 | 0.0 | 5074.2 |
| fc4 d0 engine | 8670.7 | 31.882 | 0.0 | 8702.6 |
| fc4 d4 greedy | 4510.9 | 22.390 | 0.0 | 4535.7 |
| fc4 d4 engine | 7124.0 | 29.428 | 0.0 | 7152.5 |
| fc5 d0 greedy | 5097.2 | 23.350 | 0.0 | 5123.3 |
| fc5 d0 engine | 8861.1 | 32.964 | 0.0 | 8893.2 |
| fc5 d4 greedy | 4401.6 | 21.832 | 0.0 | 4425.2 |
| fc5 d4 engine | 6960.9 | 28.883 | 0.0 | 6989.1 |
| pc2 d0 greedy | 4435.5 | 22.882 | 0.0 | 4458.7 |
| pc2 d0 engine | 8223.2 | 30.674 | 0.0 | 8254.5 |
| pc2 d4 greedy | 3784.8 | 20.522 | 0.0 | 3806.4 |
| pc2 d4 engine | 6137.6 | 27.687 | 0.0 | 6163.1 |
| draw2 d0 greedy | 4538.1 | 22.788 | 0.0 | 4561.7 |
| draw2 d0 engine | 7920.2 | 30.772 | 0.0 | 7950.7 |
| draw2 d4 greedy | 3937.6 | 21.039 | 0.0 | 3959.7 |
| draw2 d4 engine | 5954.2 | 27.020 | 0.0 | 5979.5 |
| credits18 d0 greedy | 4878.8 | 22.497 | 0.0 | 4904.3 |
| credits18 d0 engine | 8887.7 | 32.504 | 0.0 | 8920.0 |
| credits18 d4 greedy | 4189.2 | 21.071 | 0.0 | 4212.5 |
| credits18 d4 engine | 6563.7 | 28.676 | 0.0 | 6590.0 |

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c15-baseline-keep3-d4-engine-seed300017.json`](records/close-loss-c15-baseline-keep3-d4-engine-seed300017.json): close loss at difficulty 4: corruption in round 8, 1 adversary piece(s) over the level-2 bar, pool 0 of 14
- [`records/close-loss-c15-baseline-keep3-d4-engine-seed300039.json`](records/close-loss-c15-baseline-keep3-d4-engine-seed300039.json): close loss at difficulty 4: corruption in round 8, 1 adversary piece(s) over the level-2 bar, pool 0 of 14

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c0-baseline-base-d0-greedy-seed300000.json`](records/surprising-win-c0-baseline-base-d0-greedy-seed300000.json): won by the weakest agent (greedy)
- [`records/surprising-win-c2-baseline-base-d4-greedy-seed300000.json`](records/surprising-win-c2-baseline-base-d4-greedy-seed300000.json): won by the weakest agent (greedy)

