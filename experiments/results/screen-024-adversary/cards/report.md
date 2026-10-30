# Cycle report: screen-024-cards

024 quick screen (r15): the scarier adversary. 2 and 3 countries, d0 to d4, random, greedy and engine: win rates, pick and play rates per card, tags used by winning engines, hand size, insight on keeps versus plays.

480 games (30 cells × 16 seeds, 0..=15) on 12 threads in 276.0 s. Engine 0.1.0, rules r15, content 33d97fde6b75a021.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates random 0%, greedy 62%, engine 94%; skill gap +94 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) random 6.12, greedy 13.12, engine 13.56; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c2): win rates random 0%, greedy 69%, engine 81%; skill gap +81 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.69, greedy 12.56, engine 12.94; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c2): win rates random 0%, greedy 31%, engine 69%; skill gap +69 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) random 4.94, greedy 11.31, engine 11.81; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c2): win rates random 0%, greedy 19%, engine 38%; skill gap +38 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c2): survival (rounds; wins count extra) random 4.12, greedy 9.25, engine 10.88; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c2): win rates random 0%, greedy 25%, engine 38%; skill gap +38 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) random 3.94, greedy 8.06, engine 9.69; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): win rates random 0%, greedy 81%, engine 94%; skill gap +94 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) random 6.25, greedy 13.25, engine 13.62; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c3): win rates random 0%, greedy 81%, engine 100%; skill gap +100 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c3): survival (rounds; wins count extra) random 6.06, greedy 13.00, engine 13.44; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates random 0%, greedy 75%, engine 81%; skill gap +81 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.50, greedy 11.81, engine 12.00; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c3): win rates random 0%, greedy 31%, engine 38%; skill gap +38 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c3): survival (rounds; wins count extra) random 4.44, greedy 10.00, engine 10.38; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates random 0%, greedy 6%, engine 19%; skill gap +19 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) random 4.00, greedy 7.94, engine 8.50; the ladder is ordered.
- Skill gap from difficulty 0 to 4 (mean over 2 configs/variants): +94 → +28 pts win rate, +7.41 → +5.12 survival (target: widens with difficulty).
- Game length for the strongest agent (engine; configs/baseline.json, c2): 6.4 rounds at difficulty 0, 7.6 at 4 (target 8–10).
- Game length for the strongest agent (engine; configs/baseline.json, c3): 6.6 rounds at difficulty 0, 7.6 at 4 (target 8–10).
- Games ended by: cities 88, deck_empty 70, purge 174, supply 141, unity 7.
- Adversary operations: 47.9 per round on average across cells, at most 292 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: benchmark_suite, black_ice_citadel, data_licensing_deal, exabyte_archive, foundation_model_campus, hyperscale_botnet, liquidity_event, megacorp_ipo, recruit_operatives, recursive_self_improvement, summit_caucus, vendor_financing; over 90%: adaptive_malware, agency_field_station, amnesty_decree, baseload_contract, blackout_script, blackstart_protocol, bonded_warehouse, breach_and_clear, breakthrough_leak, bulk_silicon_procurement, ceasefire_monitors, citation_engine, collateralized_grid, commons_grid, compute_siphon, consular_pouch, consulate_tower, cryo_compute_burn, darknet_exchange, deep_state_ledger, development_loan, diplomatic_expulsion, embassy_compound, endowed_research_chair, eviction_worm, exodus_corridors, expansion_fund, faraday_yard, flash_exploit, fusion_startup_stake, grad_student_pool, grid_tied_smelter, hardened_conduits, hardpoint_lockdown, hardware_spot_market, honeypot_relay, intrusion_countermeasures, intrusion_lab, joint_fabrication_pact, joint_security_compact, lights_out_assembly, logic_bomb, materials_hedge_desk, neural_mirror, peace_dividend, peacekeeper_rotation, predictive_modelling, safehouse_network, security_contractor_hq, sentinel_mesh, shared_threat_intel, simulation_cluster, storm_hardening, supply_chain_treaty, telemetry_archive, tidal_barrage.
- Luck share of survival (seed's share of the variance): 9% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 86% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 94% wins) plays 4.81 buildings and 5.44 tier-2 cards per game (of 17.2 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (c2): the top agent there (engine, 81% wins) plays 4.00 buildings and 6.44 tier-2 cards per game (of 18.1 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 2 (c2): the top agent there (engine, 69% wins) plays 3.12 buildings and 5.50 tier-2 cards per game (of 16.2 cards); 91% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 3 (c2): the top agent there (engine, 38% wins) plays 2.88 buildings and 6.31 tier-2 cards per game (of 17.2 cards); 100% of its wins had 2+ buildings played (83% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (c2): the top agent there (engine, 38% wins) plays 2.69 buildings and 5.56 tier-2 cards per game (of 15.8 cards); 100% of its wins had 2+ buildings played (67% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 38% vs greedy 25% over 16 seeds (only engine 4, only greedy 2), +12.5 ±15.5 pts, sign test p 0.688: no better than greedy (ahead, not significant).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±9.1.
- Variants against the base (c2) at difficulty 1, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +18.8 ±10.1.
- Variants against the base (c2) at difficulty 2, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +12.5 ±15.5.
- Variants against the base (c2) at difficulty 3, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±12.9.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -18.8 ±16.4.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.

| c2 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.12) | 62.5 ±12.1 (13.12) | 93.8 ±6.1 (13.56) | +93.8 pts (+7.44 survival) |

| c2 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.69) | 68.8 ±11.6 (12.56) | 81.2 ±9.8 (12.94) | +81.2 pts (+7.25 survival) |

| c2 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.94) | 31.2 ±11.6 (11.31) | 68.8 ±11.6 (11.81) | +68.8 pts (+6.88 survival) |

| c2 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.12) | 18.8 ±9.8 (9.25) | 37.5 ±12.1 (10.88) | +37.5 pts (+6.75 survival) |

| c2 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.94) | 25.0 ±10.8 (8.06) | 37.5 ±12.1 (9.69) | +37.5 pts (+5.75 survival) |

| c3 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.25) | 81.2 ±9.8 (13.25) | 93.8 ±6.1 (13.62) | +93.8 pts (+7.38 survival) |

| c3 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.06) | 81.2 ±9.8 (13.00) | 100.0 ±0.0 (13.44) | +100.0 pts (+7.38 survival) |

| c3 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.50) | 75.0 ±10.8 (11.81) | 81.2 ±9.8 (12.00) | +81.2 pts (+6.50 survival) |

| c3 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.44) | 31.2 ±11.6 (10.00) | 37.5 ±12.1 (10.38) | +37.5 pts (+5.94 survival) |

| c3 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.00) | 6.2 ±6.1 (7.94) | 18.8 ±9.8 (8.50) | +18.8 pts (+4.50 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 random | 16 | 0.0 | 62.5 | 0 / 0 / 10 / 6 | -62.5 ±12.5 | 0.002 | worse than greedy |
| c2 d0 engine | 16 | 93.8 | 62.5 | 10 / 5 / 0 / 1 | +31.2 ±12.0 | 0.062 | no better than greedy (ahead, not significant) |
| c2 d1 random | 16 | 0.0 | 68.8 | 0 / 0 / 11 / 5 | -68.8 ±12.0 | <0.001 | worse than greedy |
| c2 d1 engine | 16 | 81.2 | 68.8 | 10 / 3 / 1 / 2 | +12.5 ±12.5 | 0.625 | no better than greedy (ahead, not significant) |
| c2 d2 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c2 d2 engine | 16 | 68.8 | 31.2 | 3 / 8 / 2 / 3 | +37.5 ±18.0 | 0.109 | no better than greedy (ahead, not significant) |
| c2 d3 random | 16 | 0.0 | 18.8 | 0 / 0 / 3 / 13 | -18.8 ±10.1 | 0.250 | no better than greedy (behind, not significant) |
| c2 d3 engine | 16 | 37.5 | 18.8 | 1 / 5 / 2 / 8 | +18.8 ±16.4 | 0.453 | no better than greedy (ahead, not significant) |
| c2 d4 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c2 d4 engine | 16 | 37.5 | 25.0 | 2 / 4 / 2 / 8 | +12.5 ±15.5 | 0.688 | no better than greedy (ahead, not significant) |
| c3 d0 random | 16 | 0.0 | 81.2 | 0 / 0 / 13 / 3 | -81.2 ±10.1 | <0.001 | worse than greedy |
| c3 d0 engine | 16 | 93.8 | 81.2 | 12 / 3 / 1 / 0 | +12.5 ±12.5 | 0.625 | no better than greedy (ahead, not significant) |
| c3 d1 random | 16 | 0.0 | 81.2 | 0 / 0 / 13 / 3 | -81.2 ±10.1 | <0.001 | worse than greedy |
| c3 d1 engine | 16 | 100.0 | 81.2 | 13 / 3 / 0 / 0 | +18.8 ±10.1 | 0.250 | no better than greedy (ahead, not significant) |
| c3 d2 random | 16 | 0.0 | 75.0 | 0 / 0 / 12 / 4 | -75.0 ±11.2 | <0.001 | worse than greedy |
| c3 d2 engine | 16 | 81.2 | 75.0 | 10 / 3 / 2 / 1 | +6.2 ±14.3 | 1.000 | no better than greedy (ahead, not significant) |
| c3 d3 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c3 d3 engine | 16 | 37.5 | 31.2 | 1 / 5 / 4 / 6 | +6.2 ±19.3 | 1.000 | no better than greedy (ahead, not significant) |
| c3 d4 random | 16 | 0.0 | 6.2 | 0 / 0 / 1 / 15 | -6.2 ±6.2 | 1.000 | no better than greedy (behind, not significant) |
| c3 d4 engine | 16 | 18.8 | 6.2 | 0 / 3 / 1 / 12 | +12.5 ±12.5 | 0.625 | no better than greedy (ahead, not significant) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.12 → 6.25 |
| c3 | 0 | greedy | 62.5 → 81.2 | +18.8 ±13.6 | 0.375 | 8.25 → 7.44 |
| c3 | 0 | engine | 93.8 → 93.8 | +0.0 ±9.1 | 1.000 | 6.44 → 6.62 |
| c3 | 1 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.69 → 6.06 |
| c3 | 1 | greedy | 68.8 → 81.2 | +12.5 ±12.5 | 0.625 | 8.69 → 7.38 |
| c3 | 1 | engine | 81.2 → 100.0 | +18.8 ±10.1 | 0.250 | 7.44 → 6.75 |
| c3 | 2 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.94 → 5.50 |
| c3 | 2 | greedy | 31.2 → 75.0 | +43.8 ±12.8 | 0.016 | 9.31 → 7.81 |
| c3 | 2 | engine | 68.8 → 81.2 | +12.5 ±15.5 | 0.688 | 7.62 → 7.25 |
| c3 | 3 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.12 → 4.44 |
| c3 | 3 | greedy | 18.8 → 31.2 | +12.5 ±18.0 | 0.727 | 8.00 → 7.94 |
| c3 | 3 | engine | 37.5 → 37.5 | +0.0 ±12.9 | 1.000 | 8.44 → 8.50 |
| c3 | 4 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 3.94 → 4.00 |
| c3 | 4 | greedy | 25.0 → 6.2 | -18.8 ±10.1 | 0.250 | 6.44 → 7.75 |
| c3 | 4 | engine | 37.5 → 18.8 | -18.8 ±16.4 | 0.453 | 7.62 → 7.62 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 random | 6.12 (1.05, 5..8) | cities 8, supply 8 | – / – | 52.9 / 0.25 | 19.12 | 5.38 / 0.12 |
| c2 d0 greedy | 8.25 (2.90, 6..12) | deck_empty 6, purge 10 | 5.80 / 46.10 | 2.3 / 3.00 | 43.31 | 0.88 / 2.25 |
| c2 d0 engine | 6.44 (1.46, 6..12) | deck_empty 1, purge 15 | 5.60 / 47.07 | 3.0 / 2.00 | 47.00 | 0.06 / 3.19 |
| c2 d1 random | 5.69 (0.98, 4..8) | cities 13, supply 3 | – / – | 51.6 / 0.00 | 26.62 | 5.75 / 0.00 |
| c2 d1 greedy | 8.69 (2.31, 6..12) | cities 2, deck_empty 3, purge 8, unity 3 | 3.64 / 41.36 | 4.2 / 3.00 | 38.38 | 1.62 / 1.50 |
| c2 d1 engine | 7.44 (1.84, 6..12) | cities 2, deck_empty 1, purge 13 | 4.77 / 46.31 | 4.7 / 3.00 | 44.19 | 0.88 / 2.50 |
| c2 d2 random | 4.94 (0.56, 4..6) | cities 12, supply 4 | – / – | 44.7 / 0.00 | 28.69 | 5.75 / 0.00 |
| c2 d2 greedy | 9.31 (2.28, 6..11) | cities 1, deck_empty 10, purge 5 | 4.40 / 39.80 | 3.7 / 3.00 | 34.56 | 2.56 / 1.12 |
| c2 d2 engine | 7.62 (2.03, 6..11) | cities 2, deck_empty 3, purge 11 | 4.09 / 46.18 | 7.2 / 2.80 | 42.94 | 1.19 / 2.06 |
| c2 d3 random | 4.12 (0.48, 3..5) | cities 9, supply 7 | – / – | 44.7 / 0.12 | 22.25 | 5.31 / 0.06 |
| c2 d3 greedy | 8.00 (1.84, 6..11) | cities 4, deck_empty 3, purge 3, supply 6 | 4.67 / 45.00 | 5.3 / 2.92 | 21.00 | 3.50 / 0.12 |
| c2 d3 engine | 8.44 (2.24, 6..11) | cities 1, deck_empty 6, purge 6, supply 3 | 4.50 / 45.00 | 5.7 / 2.90 | 30.94 | 2.44 / 0.25 |
| c2 d4 random | 3.94 (0.43, 3..5) | cities 3, supply 13 | – / – | 49.0 / 0.12 | 12.00 | 5.00 / 0.06 |
| c2 d4 greedy | 6.44 (1.37, 4..9) | purge 4, supply 12 | 4.50 / 33.75 | 6.0 / 2.75 | 11.50 | 2.81 / 0.00 |
| c2 d4 engine | 7.62 (2.29, 4..11) | deck_empty 3, purge 5, supply 7, unity 1 | 3.50 / 32.00 | 6.9 / 2.70 | 17.88 | 2.12 / 0.25 |
| c3 d0 random | 6.25 (0.90, 5..8) | cities 6, supply 10 | – / – | 92.9 / 0.00 | 25.25 | 8.00 / 0.25 |
| c3 d0 greedy | 7.44 (2.29, 6..12) | deck_empty 3, purge 13 | 5.15 / 67.92 | 1.7 / 3.00 | 66.19 | 0.31 / 3.00 |
| c3 d0 engine | 6.62 (1.58, 6..12) | deck_empty 1, purge 15 | 5.47 / 69.20 | 1.0 / 3.00 | 69.25 | 0.06 / 4.31 |
| c3 d1 random | 6.06 (1.20, 5..10) | cities 10, supply 6 | – / – | 85.7 / 0.00 | 29.19 | 8.19 / 0.00 |
| c3 d1 greedy | 7.38 (2.29, 6..12) | deck_empty 3, purge 13 | 4.92 / 67.23 | 4.3 / 3.00 | 63.56 | 0.81 / 2.12 |
| c3 d1 engine | 6.75 (1.09, 6..9) | purge 16 | 4.69 / 68.75 | – / – | 68.75 | 0.00 / 3.56 |
| c3 d2 random | 5.50 (0.71, 4..7) | cities 5, supply 11 | – / – | 96.8 / 0.00 | 17.81 | 7.81 / 0.19 |
| c3 d2 greedy | 7.81 (1.88, 6..11) | cities 1, deck_empty 2, purge 10, supply 1, unity 2 | 3.33 / 66.75 | 5.8 / 3.00 | 62.38 | 1.75 / 1.94 |
| c3 d2 engine | 7.25 (1.89, 6..11) | deck_empty 3, purge 13 | 3.85 / 63.54 | 1.7 / 3.00 | 63.50 | 0.31 / 2.50 |
| c3 d3 random | 4.44 (0.61, 4..6) | cities 8, supply 8 | – / – | 83.4 / 0.00 | 26.50 | 7.81 / 0.06 |
| c3 d3 greedy | 7.94 (2.51, 4..11) | deck_empty 6, purge 5, supply 5 | 4.60 / 65.00 | 27.4 / 2.27 | 38.25 | 3.56 / 0.19 |
| c3 d3 engine | 8.50 (2.29, 4..11) | deck_empty 6, purge 6, supply 4 | 3.00 / 64.17 | 23.5 / 2.40 | 46.88 | 3.12 / 0.56 |
| c3 d4 random | 4.00 (0.61, 3..5) | cities 1, supply 15 | – / – | 78.4 / 0.00 | 16.88 | 7.38 / 0.00 |
| c3 d4 greedy | 7.75 (2.63, 4..11) | deck_empty 5, supply 10, unity 1 | 1.00 / 47.00 | 24.7 / 2.20 | 21.25 | 4.81 / 0.06 |
| c3 d4 engine | 7.62 (2.64, 4..11) | deck_empty 5, purge 3, supply 8 | 2.67 / 51.00 | 23.0 / 2.15 | 26.69 | 3.38 / 0.56 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 random | 306.8 | 10.90 | 2% | – | – | 49.40 (162) | 7.19 | 15.31 | 3.56 | 4.50 / 17.31 (26%) |
| c2 d0 greedy | 378.4 | 15.28 | 2% | – | – | 18.80 (79) | 0.00 | 13.31 | 0.00 | 11.31 / 17.94 (63%) |
| c2 d0 engine | 298.2 | 13.89 | 2% | – | – | 13.48 (47) | 0.00 | 11.44 | 0.00 | 12.25 / 12.44 (98%) |
| c2 d1 random | 287.9 | 11.07 | 2% | – | – | 52.09 (154) | 6.62 | 20.19 | 5.19 | 4.75 / 22.38 (21%) |
| c2 d1 greedy | 462.1 | 15.05 | 2% | – | – | 29.56 (118) | 0.00 | 18.38 | 0.00 | 15.06 / 30.69 (49%) |
| c2 d1 engine | 369.2 | 13.77 | 2% | – | – | 21.06 (127) | 0.00 | 18.62 | 0.00 | 19.88 / 23.31 (85%) |
| c2 d2 random | 241.1 | 11.52 | 2% | – | – | 46.63 (143) | 5.94 | 17.44 | 4.31 | 4.06 / 18.44 (22%) |
| c2 d2 greedy | 487.1 | 15.48 | 2% | – | – | 32.46 (149) | 0.00 | 19.56 | 0.00 | 15.38 / 31.81 (48%) |
| c2 d2 engine | 372.8 | 15.17 | 2% | – | – | 21.25 (114) | 0.00 | 19.56 | 0.00 | 18.75 / 23.56 (80%) |
| c2 d3 random | 181.2 | 12.25 | 2% | – | – | 51.33 (100) | 5.62 | 10.75 | 2.44 | 2.88 / 14.56 (20%) |
| c2 d3 greedy | 451.6 | 14.42 | 1% | – | – | 53.84 (145) | 0.00 | 17.81 | 0.00 | 14.00 / 29.50 (47%) |
| c2 d3 engine | 452.6 | 13.91 | 1% | – | – | 43.71 (128) | 0.00 | 23.69 | 0.00 | 23.62 / 29.00 (81%) |
| c2 d4 random | 172.1 | 12.24 | 2% | – | – | 51.54 (113) | 5.69 | 10.44 | 2.12 | 2.75 / 13.88 (20%) |
| c2 d4 greedy | 344.8 | 13.82 | 1% | – | – | 52.33 (151) | 0.00 | 11.50 | 0.00 | 9.31 / 22.19 (42%) |
| c2 d4 engine | 409.8 | 13.63 | 1% | – | – | 42.77 (120) | 0.00 | 21.31 | 0.00 | 20.75 / 26.31 (79%) |
| c3 d0 random | 465.6 | 15.06 | 1% | – | – | 80.83 (243) | 15.19 | 24.38 | 5.62 | 6.56 / 22.38 (29%) |
| c3 d0 greedy | 525.1 | 21.40 | 1% | – | – | 25.26 (74) | 0.00 | 16.69 | 0.00 | 14.56 / 22.12 (66%) |
| c3 d0 engine | 466.6 | 19.64 | 1% | – | – | 21.87 (54) | 0.00 | 15.69 | 0.00 | 17.44 / 18.50 (94%) |
| c3 d1 random | 481.4 | 15.27 | 1% | – | – | 78.55 (215) | 16.31 | 28.25 | 6.88 | 9.19 / 27.50 (33%) |
| c3 d1 greedy | 552.8 | 20.58 | 1% | – | – | 34.64 (184) | 0.00 | 21.50 | 0.00 | 19.62 / 30.94 (63%) |
| c3 d1 engine | 494.1 | 18.82 | 1% | – | – | 25.62 (86) | 0.00 | 20.88 | 0.00 | 23.50 / 26.75 (88%) |
| c3 d2 random | 421.9 | 15.11 | 1% | – | – | 81.17 (292) | 13.19 | 26.69 | 7.56 | 7.38 / 24.19 (30%) |
| c3 d2 greedy | 599.6 | 21.27 | 1% | – | – | 41.00 (131) | 0.00 | 21.94 | 0.00 | 19.75 / 33.75 (59%) |
| c3 d2 engine | 556.9 | 19.42 | 1% | – | – | 33.88 (138) | 0.00 | 23.50 | 0.00 | 25.38 / 29.81 (85%) |
| c3 d3 random | 286.1 | 15.73 | 1% | – | – | 82.93 (190) | 11.06 | 15.25 | 3.81 | 4.62 / 17.62 (26%) |
| c3 d3 greedy | 650.8 | 19.77 | 1% | – | – | 71.35 (211) | 0.00 | 26.62 | 0.00 | 21.81 / 35.44 (62%) |
| c3 d3 engine | 684.8 | 19.19 | 1% | – | – | 66.22 (201) | 0.00 | 30.94 | 0.00 | 31.94 / 38.50 (83%) |
| c3 d4 random | 251.0 | 15.88 | 1% | – | – | 75.52 (161) | 8.12 | 13.88 | 3.31 | 3.69 / 16.31 (23%) |
| c3 d4 greedy | 638.8 | 19.70 | 1% | – | – | 75.52 (203) | 0.00 | 25.75 | 0.00 | 19.31 / 34.81 (55%) |
| c3 d4 engine | 604.7 | 18.83 | 1% | – | – | 63.68 (193) | 0.00 | 27.94 | 0.00 | 27.25 / 33.06 (82%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 17% / 61% / 23% | 2% / 94% / 4% |
| c2 d1 | 22% / 51% / 27% | 5% / 89% / 6% |
| c2 d2 | 19% / 36% / 46% | 2% / 90% / 8% |
| c2 d3 | 27% / 15% / 58% | 9% / 74% / 17% |
| c2 d4 | 33% / 15% / 53% | 24% / 50% / 27% |
| c3 d0 | 9% / 71% / 20% | 2% / 95% / 3% |
| c3 d1 | 7% / 79% / 14% | 2% / 94% / 4% |
| c3 d2 | 17% / 55% / 29% | 3% / 90% / 6% |
| c3 d3 | 21% / 15% / 63% | 17% / 61% / 23% |
| c3 d4 | 27% / 8% / 65% | 27% / 40% / 32% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 random | 12 of 16 | 0.88 | 0 of 0 | – |
| c2 d0 greedy | 12 of 16 | 0.52 | 8 of 10 | 0.55 |
| c2 d0 engine | 15 of 16 | 0.69 | 14 of 15 | 0.64 |
| c2 d1 random | 15 of 16 | 0.88 | 0 of 0 | – |
| c2 d1 greedy | 11 of 16 | 0.51 | 9 of 11 | 0.56 |
| c2 d1 engine | 16 of 16 | 0.75 | 13 of 13 | 0.79 |
| c2 d2 random | 15 of 16 | 0.87 | 0 of 0 | – |
| c2 d2 greedy | 12 of 16 | 0.51 | 4 of 5 | 0.29 |
| c2 d2 engine | 16 of 16 | 0.66 | 11 of 11 | 0.55 |
| c2 d3 random | 11 of 16 | 0.88 | 0 of 0 | – |
| c2 d3 greedy | 9 of 16 | 0.42 | 3 of 3 | 0.39 |
| c2 d3 engine | 15 of 16 | 0.74 | 6 of 6 | 0.78 |
| c2 d4 random | 13 of 16 | 0.89 | 0 of 0 | – |
| c2 d4 greedy | 9 of 16 | 0.41 | 4 of 4 | 0.50 |
| c2 d4 engine | 14 of 16 | 0.73 | 5 of 6 | 0.80 |
| c3 d0 random | 14 of 16 | 0.91 | 0 of 0 | – |
| c3 d0 greedy | 16 of 16 | 0.57 | 13 of 13 | 0.57 |
| c3 d0 engine | 16 of 16 | 0.75 | 15 of 15 | 0.75 |
| c3 d1 random | 14 of 16 | 0.91 | 0 of 0 | – |
| c3 d1 greedy | 16 of 16 | 0.54 | 13 of 13 | 0.52 |
| c3 d1 engine | 16 of 16 | 0.68 | 16 of 16 | 0.68 |
| c3 d2 random | 14 of 16 | 0.90 | 0 of 0 | – |
| c3 d2 greedy | 16 of 16 | 0.60 | 12 of 12 | 0.52 |
| c3 d2 engine | 16 of 16 | 0.72 | 13 of 13 | 0.70 |
| c3 d3 random | 14 of 16 | 0.90 | 0 of 0 | – |
| c3 d3 greedy | 15 of 16 | 0.54 | 5 of 5 | 0.57 |
| c3 d3 engine | 15 of 16 | 0.71 | 6 of 6 | 0.69 |
| c3 d4 random | 15 of 16 | 0.87 | 0 of 0 | – |
| c3 d4 greedy | 14 of 16 | 0.53 | 1 of 1 | – |
| c3 d4 engine | 15 of 16 | 0.72 | 3 of 3 | 0.85 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.44 / 0.31 / 8.4, – | 2.00 / 0.44 / 9.5, – |
| greedy | 2.38 / 5.38 / 13.2, 90% (100%) | 3.62 / 8.38 / 19.4, 100% (100%) |
| engine | 4.81 / 5.44 / 17.2, 100% (100%) | 6.38 / 8.69 / 26.1, 100% (100%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.12 / 0.06 / 6.6, – | 2.81 / 0.44 / 10.9, – |
| greedy | 2.19 / 6.81 / 15.1, 82% (91%) | 3.31 / 9.75 / 19.8, 100% (100%) |
| engine | 4.00 / 6.44 / 18.1, 100% (100%) | 6.19 / 8.75 / 25.6, 100% (100%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.00 / 0.12 / 6.0, – | 3.00 / 0.44 / 9.9, – |
| greedy | 2.19 / 7.06 / 15.6, 60% (100%) | 3.12 / 9.00 / 19.8, 100% (100%) |
| engine | 3.12 / 5.50 / 16.2, 91% (100%) | 4.88 / 9.19 / 24.6, 100% (100%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 1.88 / 0.00 / 6.1, – | 2.69 / 0.12 / 7.9, – |
| greedy | 1.88 / 5.44 / 12.8, 33% (100%) | 3.25 / 8.88 / 19.6, 100% (100%) |
| engine | 2.88 / 6.31 / 17.2, 100% (83%) | 4.19 / 10.19 / 26.5, 100% (100%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 1.88 / 0.00 / 6.2, – | 2.62 / 0.06 / 7.4, – |
| greedy | 1.75 / 4.12 / 10.6, 75% (100%) | 2.94 / 8.75 / 19.3, 100% (100%) |
| engine | 2.69 / 5.56 / 15.8, 100% (67%) | 3.81 / 8.81 / 23.5, 100% (100%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|---|
| adaptive_malware | 2.3 (39) / 38% / 20% | 3.8 (28) / 143% / 62% | 3.8 (27) / 144% / 87% |
| adversarial_inference_sweep | 2.3 (45) / 18% / 0% | 2.3 (190) / 23% / 7% | 2.3 (172) / 23% / 0% |
| aegis_dome_array | 2.4 (43) / 40% / 0% | 2.3 (196) / 18% / 0% | 2.3 (178) / 31% / 0% |
| aegis_grid_lockdown | 2.8 (44) / 43% / 0% | 2.3 (214) / 12% / 0% | 2.4 (188) / 25% / 0% |
| agency_directorate | 2.4 (55) / 51% / 0% | 2.3 (209) / 7% / 0% | 2.3 (201) / 10% / 0% |
| agency_field_station | 2.5 (53) / 58% / 35% | 3.7 (27) / 111% / 40% | 3.7 (29) / 114% / 33% |
| amnesty_decree | 2.8 (48) / 75% / 6% | 3.6 (17) / 71% / 125% | 3.7 (20) / 95% / 100% |
| antiproton_offtake_deal | 2.8 (52) / 46% / 0% | 2.2 (196) / 1% / 0% | 2.3 (165) / 15% / 0% |
| arc_furnace_complex | 3.0 (41) / 41% / 0% | 2.3 (204) / 1% / 0% | 2.2 (176) / 24% / 0% |
| arcology_foundry | 2.2 (76) / 51% / 0% | 2.3 (185) / 37% / 0% | 2.2 (179) / 41% / 10% |
| asteroid_salvage_rights | 2.6 (51) / 49% / 4% | 2.2 (204) / 50% / 88% | 2.1 (189) / 53% / 86% |
| back_channel_talks | 2.3 (19) / 174% / 6% | 3.5 (10) / 100% / 100% | 3.8 (8) / 75% / 133% |
| baseload_contract | 2.5 (31) / 77% / 17% | 3.9 (16) / 181% / 0% | 3.8 (18) / 183% / 39% |
| beamed_power_constellation | 2.5 (66) / 53% / 0% | 2.2 (208) / 9% / 47% | 2.3 (179) / 21% / 34% |
| benchmark_suite | 2.3 (26) / 112% / 0% | 3.6 (16) / 12% / 0% | 3.6 (17) / 6% / 100% |
| black_ice_citadel | 2.5 (67) / 39% / 0% | 2.4 (205) / 0% / 0% | 2.3 (201) / 1% / 0% |
| blackout_script | 2.2 (26) / 112% / 3% | 3.4 (7) / 214% / 0% | 3.9 (10) / 160% / 88% |
| blackstart_protocol | 2.4 (31) / 129% / 38% | 3.8 (19) / 337% / 0% | 3.2 (23) / 287% / 98% |
| bonded_warehouse | 2.5 (45) / 91% / 29% | 3.4 (18) / 267% / 0% | 3.7 (23) / 230% / 9% |
| bounty_board | 2.4 (24) / 62% / 27% | 3.5 (12) / 75% / 0% | 3.4 (9) / 89% / 112% |
| breach_and_clear | 2.3 (21) / 133% / 25% | 3.8 (12) / 283% / 24% | 3.9 (10) / 390% / 44% |
| breakthrough_leak | 2.3 (21) / 114% / 21% | 3.8 (16) / 288% / 100% | 3.5 (12) / 350% / 79% |
| bridge_loan | 2.2 (26) / 173% / 56% | 3.6 (10) / 440% / 0% | 3.5 (8) / 538% / 84% |
| brownout_relief | 3.1 (12) / 333% / 35% | 3.0 (3) / 1400% / 64% | 3.8 (4) / 1075% / 93% |
| bulk_silicon_procurement | 2.6 (48) / 75% / 33% | 3.8 (25) / 180% / 104% | 3.7 (29) / 166% / 90% |
| capacitor_discharge | 2.5 (17) / 76% / 31% | 3.5 (10) / 180% / 0% | 4.0 (7) / 200% / 100% |
| cascade_failure | 2.1 (64) / 47% / 7% | 2.3 (159) / 85% / 0% | 2.1 (149) / 85% / 8% |
| ceasefire_monitors | 2.2 (23) / 109% / 44% | 3.5 (13) / 254% / 109% | 3.6 (17) / 206% / 94% |
| citation_engine | 2.6 (32) / 159% / 31% | 3.9 (31) / 206% / 0% | 3.7 (19) / 305% / 33% |
| coalition_charter | 2.4 (33) / 58% / 0% | 2.2 (153) / 89% / 93% | 2.3 (143) / 85% / 93% |
| collateralized_grid | 2.2 (39) / 118% / 26% | 3.5 (20) / 270% / 0% | 3.8 (20) / 275% / 62% |
| commons_grid | 2.6 (25) / 132% / 33% | 3.6 (11) / 327% / 31% | 3.7 (16) / 256% / 15% |
| compute_siphon | 2.7 (30) / 100% / 37% | 3.7 (10) / 440% / 102% | 3.3 (17) / 300% / 84% |
| consular_pouch | 2.2 (26) / 154% / 32% | 4.0 (25) / 256% / 100% | 3.2 (27) / 244% / 103% |
| consulate_tower | 2.1 (43) / 119% / 22% | 3.6 (13) / 338% / 141% | 3.8 (22) / 245% / 69% |
| continental_supergrid | 2.5 (49) / 37% / 0% | 2.3 (199) / 1% / 0% | 2.2 (181) / 27% / 33% |
| cryo_compute_burn | 3.0 (22) / 95% / 38% | 3.4 (18) / 178% / 0% | 3.4 (14) / 179% / 0% |
| darknet_exchange | 2.6 (26) / 88% / 26% | 3.4 (17) / 188% / 19% | 3.6 (19) / 168% / 16% |
| data_licensing_deal | 2.8 (47) / 49% / 0% | 2.2 (204) / 1% / 100% | 2.2 (203) / 1% / 67% |
| dataset_scraper | 2.4 (11) / 236% / 0% | 3.2 (6) / 17% / 0% | 3.8 (4) / 0% / – |
| deep_state_ledger | 2.5 (71) / 41% / 21% | 2.3 (152) / 97% / 0% | 2.4 (147) / 92% / 0% |
| demand_response_grid | 2.7 (10) / 220% / 23% | 3.6 (7) / 514% / 0% | 4.0 (6) / 600% / 17% |
| development_loan | 2.1 (30) / 90% / 15% | 3.8 (19) / 284% / 107% | 3.5 (22) / 241% / 100% |
| diplomatic_expulsion | 3.1 (19) / 326% / 27% | 3.4 (10) / 610% / 113% | 3.2 (13) / 485% / 100% |
| dyson_swarm_tranche | 2.4 (63) / 44% / 0% | 2.1 (218) / 0% / – | 2.4 (184) / 15% / 4% |
| embassy_compound | 2.8 (20) / 260% / 19% | 3.9 (14) / 493% / 101% | 3.6 (14) / 493% / 100% |
| endowed_research_chair | 2.5 (34) / 109% / 30% | 3.8 (12) / 250% / 0% | 3.8 (10) / 290% / 0% |
| eviction_worm | 2.8 (33) / 100% / 12% | 3.6 (19) / 168% / 69% | 3.4 (24) / 162% / 82% |
| exabyte_archive | 2.7 (68) / 41% / 0% | 2.3 (206) / 3% / 0% | 2.1 (185) / 6% / 0% |
| exodus_corridors | 2.5 (82) / 55% / 9% | 2.0 (153) / 93% / 0% | 2.2 (158) / 94% / 74% |
| expansion_fund | 2.5 (30) / 133% / 25% | 3.7 (27) / 248% / 0% | 3.6 (25) / 248% / 21% |
| faraday_yard | 2.4 (17) / 265% / 24% | 3.8 (14) / 293% / 0% | 3.8 (13) / 323% / 48% |
| field_sensor_grid | 2.0 (21) / 62% / 46% | 3.0 (4) / 75% / 0% | 3.5 (6) / 100% / 0% |
| flash_exploit | 2.5 (30) / 83% / 12% | 3.8 (23) / 174% / 35% | 3.2 (19) / 174% / 85% |
| foundation_model_campus | 2.5 (66) / 35% / 0% | 2.3 (212) / 0% / – | 2.3 (198) / 1% / 0% |
| full_automation_mandate | 2.5 (70) / 41% / 0% | 2.4 (206) / 32% / 0% | 2.2 (184) / 34% / 56% |
| fusion_startup_stake | 2.8 (24) / 117% / 7% | 3.7 (19) / 237% / 0% | 3.7 (14) / 314% / 82% |
| grad_student_pool | 2.1 (31) / 77% / 38% | 3.4 (13) / 246% / 0% | 3.5 (13) / 246% / 59% |
| great_liberation | 2.5 (56) / 38% / 5% | 2.3 (190) / 76% / 83% | 2.3 (192) / 68% / 89% |
| grid_census | 2.1 (12) / 250% / 37% | 3.6 (9) / 322% / 0% | 3.9 (9) / 322% / 100% |
| grid_tied_smelter | 2.1 (20) / 130% / 42% | 3.8 (18) / 228% / 0% | 3.1 (16) / 256% / 51% |
| hardened_bunker_complex | 2.7 (25) / 108% / 37% | 3.5 (14) / 343% / 0% | 3.8 (9) / 478% / 60% |
| hardened_conduits | 2.4 (23) / 196% / 49% | 3.9 (14) / 350% / 0% | 3.9 (10) / 480% / 8% |
| hardpoint_lockdown | 2.1 (26) / 115% / 13% | 3.4 (15) / 113% / 0% | 3.3 (19) / 116% / 0% |
| hardware_spot_market | 2.7 (26) / 169% / 55% | 3.9 (18) / 189% / 0% | 3.6 (20) / 190% / 26% |
| honeypot_relay | 2.0 (22) / 145% / 25% | 3.9 (16) / 244% / 0% | 3.7 (13) / 277% / 3% |
| hyperscale_botnet | 2.4 (45) / 44% / 0% | 2.2 (211) / 0% / 0% | 2.4 (206) / 0% / 0% |
| interceptor_battery | 2.3 (12) / 300% / 36% | 3.8 (17) / 324% / 0% | 3.6 (8) / 588% / 2% |
| interconnector_treaty | 2.3 (67) / 45% / 0% | 2.3 (213) / 6% / 0% | 2.2 (193) / 17% / 6% |
| intrusion_countermeasures | 2.5 (27) / 148% / 22% | 3.8 (15) / 247% / 0% | 3.4 (10) / 320% / 53% |
| intrusion_lab | 2.8 (32) / 88% / 14% | 3.5 (12) / 283% / 0% | 3.6 (17) / 241% / 7% |
| joint_fabrication_pact | 2.5 (34) / 126% / 35% | 3.6 (22) / 214% / 102% | 3.7 (23) / 191% / 105% |
| joint_security_compact | 2.5 (20) / 155% / 39% | 3.6 (16) / 244% / 0% | 3.3 (23) / 196% / 24% |
| kill_team_roster | 2.9 (30) / 90% / 7% | 3.6 (25) / 44% / 0% | 3.3 (21) / 67% / 7% |
| killswitch_directive | 2.7 (85) / 40% / 0% | 2.2 (199) / 78% / 52% | 2.4 (174) / 80% / 48% |
| kinetic_interdiction | 2.6 (55) / 31% / 0% | 2.4 (224) / 84% / 94% | 2.3 (215) / 73% / 90% |
| liaison_corps | 3.2 (18) / 83% / 0% | 3.8 (6) / 300% / 0% | 3.3 (3) / 600% / 6% |
| liberation_accords | 2.6 (77) / 51% / 0% | 2.3 (196) / 32% / 78% | 2.2 (196) / 27% / 79% |
| lights_out_assembly | 2.6 (25) / 100% / 8% | 3.9 (8) / 275% / 0% | 3.7 (13) / 208% / 15% |
| liquidity_event | 2.8 (38) / 113% / 0% | 3.7 (22) / 0% / – | 3.5 (28) / 0% / – |
| literature_crawler | 2.7 (26) / 169% / 34% | 3.9 (15) / 100% / 127% | 3.6 (19) / 84% / 94% |
| logic_bomb | 2.1 (26) / 85% / 32% | 3.8 (16) / 225% / 108% | 3.5 (17) / 218% / 89% |
| manifesto_leak | 2.5 (24) / 112% / 0% | 3.8 (9) / 0% / – | 3.9 (7) / 0% / – |
| martial_cordon | 2.8 (64) / 41% / 8% | 2.2 (178) / 44% / 95% | 2.2 (173) / 44% / 96% |
| materials_hedge_desk | 2.3 (22) / 155% / 26% | 3.7 (17) / 271% / 0% | 3.5 (18) / 261% / 62% |
| megacorp_ipo | 2.6 (44) / 41% / 0% | 2.4 (216) / 0% / – | 2.2 (190) / 3% / 0% |
| mesh_intrusion_pact | 2.4 (58) / 57% / 9% | 2.1 (160) / 70% / 0% | 2.2 (149) / 74% / 28% |
| modular_expansion | – (0) / – / 0.33/game | – (0) / – / 0.44/game | – (0) / – / 0.49/game |
| mutual_aid_pact | 2.7 (14) / 286% / 22% | 3.6 (7) / 471% / 82% | 3.8 (5) / 580% / 103% |
| mutual_defense_pact | 2.7 (22) / 141% / 39% | 3.7 (16) / 206% / 0% | 3.2 (9) / 300% / 15% |
| neural_cartographer | 2.9 (13) / 192% / 16% | 3.6 (8) / 138% / 55% | 3.4 (7) / 171% / 42% |
| neural_mirror | 2.4 (25) / 124% / 19% | 3.5 (11) / 173% / 0% | 3.5 (12) / 150% / 0% |
| node_severance | 2.6 (61) / 49% / 10% | 2.2 (210) / 68% / 94% | 2.4 (190) / 65% / 95% |
| open_assembly | 2.1 (20) / 125% / 32% | 4.0 (7) / 100% / 0% | 3.5 (8) / 100% / 12% |
| open_preprint | 2.7 (26) / 54% / 29% | 3.5 (12) / 108% / 108% | 3.2 (5) / 160% / 100% |
| open_weights_release | 2.3 (61) / 38% / 13% | 2.5 (155) / 70% / 86% | 2.4 (160) / 65% / 16% |
| orbital_embassy | 2.4 (65) / 52% / 0% | 2.2 (186) / 14% / 0% | 2.5 (175) / 11% / 5% |
| orbital_mass_driver | 2.4 (104) / 45% / 0% | 2.5 (214) / 1% / 0% | 2.6 (191) / 16% / 0% |
| orbital_observatory_array | 2.3 (40) / 65% / 0% | 2.2 (209) / 29% / 0% | 2.3 (191) / 26% / 0% |
| payroll_automation | 2.3 (26) / 165% / 44% | 3.8 (12) / 67% / 0% | 3.6 (20) / 85% / 0% |
| peace_dividend | 2.2 (24) / 62% / 53% | 3.8 (16) / 88% / 100% | 3.5 (11) / 91% / 140% |
| peacekeeper_rotation | 2.4 (18) / 117% / 19% | 3.6 (7) / 200% / 79% | 3.2 (12) / 167% / 60% |
| peacekeeping_detail | 2.6 (11) / 191% / 71% | 3.5 (10) / 100% / 0% | 3.6 (7) / 86% / 17% |
| peaker_turbines | 2.5 (29) / 62% / 17% | 3.8 (16) / 88% / 0% | 3.7 (18) / 83% / 13% |
| planetary_assembly | 2.7 (45) / 40% / 0% | 2.2 (196) / 12% / 0% | 2.5 (198) / 14% / 0% |
| planetary_referendum | 2.7 (72) / 43% / 10% | 2.2 (223) / 76% / 94% | 2.0 (210) / 71% / 93% |
| point_defense | – (0) / – / 0.29/game | – (0) / – / 0.00/game | – (0) / – / 0.38/game |
| positron_storage_ring | 2.6 (73) / 40% / 0% | 2.3 (218) / 0% / – | 2.3 (197) / 19% / 0% |
| predictive_modelling | 2.5 (26) / 100% / 23% | 3.9 (12) / 308% / 0% | 3.7 (14) / 264% / 0% |
| prefab_plant_kit | 2.8 (11) / 173% / 21% | 3.6 (7) / 57% / 0% | 3.2 (4) / 0% / – |
| pylon_drop | 2.5 (30) / 117% / 37% | 4.0 (11) / 73% / 75% | 3.6 (12) / 33% / 100% |
| quantum_decryptor | 2.5 (22) / 45% / 0% | 3.5 (14) / 86% / 0% | 3.4 (12) / 83% / 0% |
| recruit_operatives | 1.6 (14) / 179% / 0% | 3.8 (10) / 0% / – | 3.1 (10) / 0% / – |
| recursive_self_improvement | 2.6 (71) / 31% / 0% | 2.3 (215) / 1% / 0% | 2.3 (198) / 2% / 0% |
| red_team_audit | 2.7 (15) / 213% / 38% | 3.6 (8) / 75% / 17% | 3.6 (8) / 88% / 0% |
| relay_mast_rollout | 2.9 (26) / 104% / 44% | 3.8 (16) / 31% / 120% | 3.7 (16) / 50% / 88% |
| retained_earnings | 2.6 (38) / 66% / 44% | 3.6 (28) / 82% / 0% | 3.7 (24) / 71% / 76% |
| robotic_foundry | 2.3 (23) / 204% / 4% | 3.8 (17) / 53% / 0% | 3.6 (17) / 53% / 11% |
| rootkit_cache | 2.1 (11) / 218% / 33% | 4.0 (3) / 100% / 0% | 4.0 (4) / 75% / 67% |
| safehouse_network | 2.7 (22) / 150% / 15% | 3.8 (17) / 76% / 0% | 3.8 (16) / 94% / 7% |
| security_contractor_hq | 2.4 (31) / 123% / 39% | 3.6 (20) / 215% / 33% | 3.8 (15) / 260% / 23% |
| sentinel_mesh | 2.7 (20) / 375% / 21% | 3.8 (13) / 554% / 0% | 3.4 (16) / 462% / 7% |
| sentry_drone_wing | 2.6 (12) / 142% / 18% | 3.8 (4) / 50% / 0% | 2.5 (2) / 100% / 0% |
| shadow_broker_feed | 2.2 (14) / 64% / 44% | 4.0 (8) / 100% / 138% | 3.3 (9) / 89% / 75% |
| shared_compute_grant | 3.1 (15) / 340% / 20% | 3.6 (9) / 78% / 86% | 3.2 (10) / 70% / 14% |
| shared_exploit_kit | 2.2 (21) / 119% / 16% | 3.8 (13) / 85% / 27% | 3.9 (13) / 77% / 100% |
| shared_grid_accord | 3.0 (44) / 77% / 47% | 3.7 (32) / 47% / 0% | 3.3 (40) / 55% / 55% |
| shared_threat_intel | 2.5 (31) / 71% / 27% | 3.8 (17) / 88% / 0% | 3.7 (22) / 91% / 30% |
| shelter_order | – (0) / – / 0.54/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shuttle_diplomacy | 2.9 (14) / 164% / 35% | 3.8 (6) / 267% / 69% | 3.8 (8) / 175% / 107% |
| signal_hijack | 1.9 (8) / 288% / 22% | 3.8 (5) / 80% / 50% | 4.0 (6) / 67% / 75% |
| simulation_cluster | 2.8 (32) / 38% / 0% | 3.6 (20) / 80% / 0% | 3.7 (14) / 93% / 0% |
| singularity_exploit | 2.6 (42) / 60% / 4% | 2.4 (175) / 33% / 86% | 2.5 (142) / 35% / 86% |
| sovereign_capital_fund | 2.4 (40) / 42% / 6% | 2.1 (182) / 30% / 0% | 2.2 (162) / 29% / 0% |
| spoofed_credentials | 2.6 (34) / 50% / 41% | 3.8 (24) / 83% / 85% | 3.2 (24) / 83% / 95% |
| station_prefab_order | 2.8 (30) / 153% / 11% | 3.4 (19) / 84% / 106% | 3.4 (16) / 88% / 114% |
| storm_hardening | 2.1 (21) / 71% / 13% | 3.6 (10) / 20% / 0% | 3.6 (14) / 93% / 0% |
| strike_team | – (0) / – / 0.24/game | – (0) / – / 0.70/game | – (0) / – / 0.95/game |
| substation_hub | 2.8 (17) / 100% / 12% | 3.5 (6) / 483% / 0% | 3.2 (8) / 412% / 79% |
| summit_caucus | 2.3 (32) / 84% / 7% | 3.8 (25) / 0% / – | 3.2 (22) / 0% / – |
| supply_chain_treaty | 3.0 (15) / 247% / 11% | 3.9 (15) / 93% / 0% | 3.6 (14) / 121% / 12% |
| surplus_auction | 2.0 (1) / 2100% / 57% | 2.0 (1) / 1000% / 0% | 3.0 (1) / 1100% / 73% |
| synthetic_data_forge | 2.3 (29) / 86% / 12% | 3.5 (18) / 83% / 0% | 3.5 (17) / 76% / 0% |
| telemetry_archive | 2.7 (46) / 63% / 34% | 3.7 (23) / 139% / 66% | 3.5 (26) / 127% / 61% |
| threat_forecast_engine | 2.1 (17) / 135% / 39% | 3.6 (12) / 75% / 89% | 3.9 (10) / 80% / 62% |
| threat_neutralization_mandate | 2.5 (60) / 38% / 0% | 2.2 (195) / 21% / 0% | 2.2 (185) / 25% / 20% |
| threat_triage | 1.7 (16) / 244% / 28% | 3.4 (5) / 40% / 0% | 4.0 (4) / 75% / 33% |
| tidal_barrage | 2.1 (32) / 153% / 43% | 3.4 (27) / 170% / 0% | 3.8 (29) / 176% / 92% |
| total_containment | 2.2 (46) / 57% / 8% | 2.4 (193) / 13% / 28% | 2.3 (179) / 12% / 23% |
| transmission_corridor | 2.6 (25) / 84% / 29% | 3.9 (15) / 60% / 0% | 3.6 (16) / 44% / 43% |
| treaty_grid_exchange | 3.0 (17) / 206% / 29% | 3.6 (8) / 450% / 0% | 3.6 (9) / 411% / 89% |
| turnkey_factory | – (0) / – / 0.46/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| vendor_financing | 2.4 (39) / 105% / 5% | 3.7 (20) / 0% / – | 3.5 (25) / 8% / 100% |
| venture_capital | – (0) / – / 0.44/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| zero_click_breach | 2.3 (26) / 123% / 6% | 3.8 (22) / 55% / 50% | 2.7 (16) / 75% / 75% |
| zero_knowledge_exploit | 2.1 (32) / 138% / 32% | 3.4 (20) / 60% / 83% | 3.8 (22) / 73% / 19% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 random | 0.0 | 0.000 | 0.0 | 86.8 |
| c2 d0 greedy | 3614.6 | 9.553 | 0.0 | 3712.2 |
| c2 d0 engine | 3261.4 | 10.937 | 0.0 | 3356.8 |
| c2 d1 random | 0.0 | 0.000 | 0.0 | 81.7 |
| c2 d1 greedy | 6832.0 | 14.786 | 0.0 | 6988.9 |
| c2 d1 engine | 4393.5 | 11.901 | 0.0 | 4505.6 |
| c2 d2 random | 0.0 | 0.000 | 0.0 | 63.3 |
| c2 d2 greedy | 6861.6 | 14.088 | 0.0 | 7023.4 |
| c2 d2 engine | 5382.7 | 14.441 | 0.0 | 5498.8 |
| c2 d3 random | 0.0 | 0.000 | 0.0 | 55.3 |
| c2 d3 greedy | 7236.4 | 16.025 | 0.0 | 7399.7 |
| c2 d3 engine | 6185.8 | 13.668 | 0.0 | 6352.5 |
| c2 d4 random | 0.0 | 0.000 | 0.0 | 42.8 |
| c2 d4 greedy | 5814.5 | 16.863 | 0.0 | 5940.1 |
| c2 d4 engine | 5806.4 | 14.171 | 0.0 | 5961.0 |
| c3 d0 random | 0.0 | 0.000 | 0.0 | 211.6 |
| c3 d0 greedy | 8874.6 | 16.902 | 0.0 | 9079.2 |
| c3 d0 engine | 10247.6 | 21.964 | 0.0 | 10407.8 |
| c3 d1 random | 0.0 | 0.000 | 0.0 | 224.4 |
| c3 d1 greedy | 9954.2 | 18.007 | 0.0 | 10179.7 |
| c3 d1 engine | 11029.9 | 22.325 | 0.0 | 11229.8 |
| c3 d2 random | 0.0 | 0.000 | 0.0 | 192.1 |
| c3 d2 greedy | 12144.7 | 20.256 | 0.0 | 12370.6 |
| c3 d2 engine | 12937.5 | 23.232 | 0.0 | 13135.3 |
| c3 d3 random | 0.0 | 0.000 | 0.0 | 118.2 |
| c3 d3 greedy | 14561.4 | 22.374 | 0.0 | 14853.3 |
| c3 d3 engine | 20184.0 | 29.474 | 0.0 | 20524.5 |
| c3 d4 random | 0.0 | 0.000 | 0.0 | 102.9 |
| c3 d4 greedy | 16536.5 | 25.889 | 0.0 | 16866.0 |
| c3 d4 engine | 16729.4 | 27.666 | 0.0 | 16957.9 |

Where the games' time went (summed over workers): agents 97.5%, probes 0.0%, apply 0.9%, legal 0.8%, replay check 0.8%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c14-baseline-c2-d4-engine-seed2.json`](records/close-loss-c14-baseline-c2-d4-engine-seed2.json): close loss at difficulty 4: deck_empty in round 11, 1 adversary piece(s) over the level-3 bar, supply 34 of 40, 1 cities corrupted
- [`records/close-loss-c29-baseline-c3-d4-engine-seed3.json`](records/close-loss-c29-baseline-c3-d4-engine-seed3.json): close loss at difficulty 4: deck_empty in round 11, 1 adversary piece(s) over the level-3 bar, supply 48 of 60, 1 cities corrupted

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c13-baseline-c2-d4-greedy-seed2.json`](records/surprising-win-c13-baseline-c2-d4-greedy-seed2.json): won at difficulty 4 by greedy, below the top agent
- [`records/surprising-win-c13-baseline-c2-d4-greedy-seed4.json`](records/surprising-win-c13-baseline-c2-d4-greedy-seed4.json): won at difficulty 4 by greedy, below the top agent

