# Cycle report: screen-023-cards

023 quick screen (r14): the player card set (B2). 2 and 3 countries, d0 to d4, random, greedy and engine: win rates, pick and play rates per card, tags used by winning engines, hand size, insight on keeps versus plays.

480 games (30 cells × 16 seeds, 0..=15) on 12 threads in 191.0 s. Engine 0.1.0, rules r14, content c0f215717631e168.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates random 0%, greedy 56%, engine 75%; skill gap +75 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) random 8.94, greedy 12.12, engine 12.75; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c2): win rates random 0%, greedy 44%, engine 44%; skill gap +44 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c2): survival (rounds; wins count extra) random 7.06, greedy 10.75, engine 11.50; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c2): win rates random 0%, greedy 25%, engine 31%; skill gap +31 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.81, greedy 9.69, engine 9.69; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c2): win rates random 0%, greedy 31%, engine 19%; skill gap +19 pts; **a stronger agent does worse than a weaker one**.
- Difficulty 3 (configs/baseline.json, c2): survival (rounds; wins count extra) random 4.94, greedy 8.12, engine 7.69; **the ladder is not ordered on survival**.
- Difficulty 4 (configs/baseline.json, c2): win rates random 0%, greedy 6%, engine 19%; skill gap +19 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) random 3.75, greedy 4.62, engine 5.94; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): win rates random 0%, greedy 56%, engine 81%; skill gap +81 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) random 8.38, greedy 12.12, engine 12.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c3): win rates random 0%, greedy 31%, engine 31%; skill gap +31 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c3): survival (rounds; wins count extra) random 6.94, greedy 10.81, engine 11.25; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates random 0%, greedy 6%, engine 25%; skill gap +25 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.81, greedy 8.75, engine 10.12; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c3): win rates random 0%, greedy 12%, engine 25%; skill gap +25 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.06, greedy 7.00, engine 8.50; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates random 0%, greedy 6%, engine 6%; skill gap +6 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) random 3.94, greedy 5.06, engine 5.69; the ladder is ordered.
- Skill gap from difficulty 0 to 4 (mean over 2 configs/variants): +78 → +12 pts win rate, +4.12 → +1.97 survival (target: widens with difficulty).
- Game length for the strongest agent (engine; configs/baseline.json, c2): 6.8 rounds at difficulty 0, 4.2 at 4 (target 8–10).
- Game length for the strongest agent (engine; configs/baseline.json, c3): 8.2 rounds at difficulty 0, 5.4 at 4 (target 8–10).
- Games ended by: cities 4, deck_empty 58, purge 100, supply 317, unity 1.
- Adversary operations: 28.0 per round on average across cells, at most 124 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: foundation_model_campus, orbital_mass_driver, recursive_self_improvement, threat_neutralization_mandate, total_containment; over 90%: adaptive_malware, agency_field_station, amnesty_decree, back_channel_talks, baseload_contract, benchmark_suite, blackout_script, blackstart_protocol, bonded_warehouse, breach_and_clear, breakthrough_leak, bridge_loan, bulk_silicon_procurement, capacitor_discharge, ceasefire_monitors, citation_engine, collateralized_grid, commons_grid, compute_siphon, consular_pouch, consulate_tower, cryo_compute_burn, darknet_exchange, demand_response_grid, development_loan, diplomatic_expulsion, embassy_compound, endowed_research_chair, eviction_worm, expansion_fund, field_sensor_grid, flash_exploit, fusion_startup_stake, grad_student_pool, grid_census, grid_tied_smelter, hardened_bunker_complex, hardened_conduits, hardpoint_lockdown, hardware_spot_market, honeypot_relay, interceptor_battery, intrusion_countermeasures, intrusion_lab, joint_fabrication_pact, joint_security_compact, liaison_corps, lights_out_assembly, liquidity_event, literature_crawler, logic_bomb, manifesto_leak, materials_hedge_desk, mutual_aid_pact, mutual_defense_pact, neural_cartographer, neural_mirror, open_assembly, open_preprint, payroll_automation, peace_dividend, peaker_turbines, predictive_modelling, pylon_drop, red_team_audit, relay_mast_rollout, retained_earnings, safehouse_network, security_contractor_hq, sentinel_mesh, shadow_broker_feed, shared_compute_grant, shared_exploit_kit, shared_threat_intel, spoofed_credentials, station_prefab_order, substation_hub, summit_caucus, supply_chain_treaty, synthetic_data_forge, telemetry_archive, threat_forecast_engine, threat_triage, tidal_barrage, transmission_corridor, treaty_grid_exchange, vendor_financing, zero_click_breach.
- Luck share of survival (seed's share of the variance): 29% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 54% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 75% wins) plays 4.50 buildings and 4.06 tier-2 cards per game (of 19.4 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (c2): the top agent there (engine, 44% wins) plays 3.81 buildings and 4.62 tier-2 cards per game (of 21.4 cards); 100% of its wins had 2+ buildings played (86% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 2 (c2): the top agent there (engine, 31% wins) plays 3.44 buildings and 4.44 tier-2 cards per game (of 19.2 cards); 100% of its wins had 2+ buildings played (80% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 3 (c2): the top agent there (greedy, 31% wins) plays 1.56 buildings and 2.44 tier-2 cards per game (of 11.6 cards); 40% of its wins had 2+ buildings played (100% with 2+ standing at the end). **It hardly builds** (under 2 buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (c2): the top agent there (engine, 19% wins) plays 2.69 buildings and 1.12 tier-2 cards per game (of 12.8 cards); 67% of its wins had 2+ buildings played (67% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 19% vs greedy 6% over 16 seeds (only engine 2, only greedy 0), +12.5 ±8.5 pts, sign test p 0.500: no better than greedy (ahead, not significant).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +6.2 ±17.0.
- Variants against the base (c2) at difficulty 1, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -12.5 ±12.5.
- Variants against the base (c2) at difficulty 2, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -6.2 ±17.0.
- Variants against the base (c2) at difficulty 3, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +6.2 ±11.1.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -12.5 ±8.5.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.

| c2 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.94) | 56.2 ±12.4 (12.12) | 75.0 ±10.8 (12.75) | +75.0 pts (+3.81 survival) |

| c2 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.06) | 43.8 ±12.4 (10.75) | 43.8 ±12.4 (11.50) | +43.8 pts (+4.44 survival) |

| c2 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.81) | 25.0 ±10.8 (9.69) | 31.2 ±11.6 (9.69) | +31.2 pts (+3.88 survival) |

| c2 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.94) | 31.2 ±11.6 (8.12) | 18.8 ±9.8 (7.69) | +18.8 pts (+2.75 survival) |

| c2 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.75) | 6.2 ±6.1 (4.62) | 18.8 ±9.8 (5.94) | +18.8 pts (+2.19 survival) |

| c3 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.38) | 56.2 ±12.4 (12.12) | 81.2 ±9.8 (12.81) | +81.2 pts (+4.44 survival) |

| c3 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.94) | 31.2 ±11.6 (10.81) | 31.2 ±11.6 (11.25) | +31.2 pts (+4.31 survival) |

| c3 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.81) | 6.2 ±6.1 (8.75) | 25.0 ±10.8 (10.12) | +25.0 pts (+4.31 survival) |

| c3 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.06) | 12.5 ±8.3 (7.00) | 25.0 ±10.8 (8.50) | +25.0 pts (+3.44 survival) |

| c3 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.94) | 6.2 ±6.1 (5.06) | 6.2 ±6.1 (5.69) | +6.2 pts (+1.75 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 random | 16 | 0.0 | 56.2 | 0 / 0 / 9 / 7 | -56.2 ±12.8 | 0.004 | worse than greedy |
| c2 d0 engine | 16 | 75.0 | 56.2 | 8 / 4 / 1 / 3 | +18.8 ±13.6 | 0.375 | no better than greedy (ahead, not significant) |
| c2 d1 random | 16 | 0.0 | 43.8 | 0 / 0 / 7 / 9 | -43.8 ±12.8 | 0.016 | worse than greedy |
| c2 d1 engine | 16 | 43.8 | 43.8 | 6 / 1 / 1 / 8 | +0.0 ±9.1 | 1.000 | no better than greedy (level) |
| c2 d2 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c2 d2 engine | 16 | 31.2 | 25.0 | 3 / 2 / 1 / 10 | +6.2 ±11.1 | 1.000 | no better than greedy (ahead, not significant) |
| c2 d3 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c2 d3 engine | 16 | 18.8 | 31.2 | 3 / 0 / 2 / 11 | -12.5 ±8.5 | 0.500 | no better than greedy (behind, not significant) |
| c2 d4 random | 16 | 0.0 | 6.2 | 0 / 0 / 1 / 15 | -6.2 ±6.2 | 1.000 | no better than greedy (behind, not significant) |
| c2 d4 engine | 16 | 18.8 | 6.2 | 1 / 2 / 0 / 13 | +12.5 ±8.5 | 0.500 | no better than greedy (ahead, not significant) |
| c3 d0 random | 16 | 0.0 | 56.2 | 0 / 0 / 9 / 7 | -56.2 ±12.8 | 0.004 | worse than greedy |
| c3 d0 engine | 16 | 81.2 | 56.2 | 7 / 6 / 2 / 1 | +25.0 ±17.1 | 0.289 | no better than greedy (ahead, not significant) |
| c3 d1 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c3 d1 engine | 16 | 31.2 | 31.2 | 2 / 3 / 3 / 8 | +0.0 ±15.8 | 1.000 | no better than greedy (level) |
| c3 d2 random | 16 | 0.0 | 6.2 | 0 / 0 / 1 / 15 | -6.2 ±6.2 | 1.000 | no better than greedy (behind, not significant) |
| c3 d2 engine | 16 | 25.0 | 6.2 | 0 / 4 / 1 / 11 | +18.8 ±13.6 | 0.375 | no better than greedy (ahead, not significant) |
| c3 d3 random | 16 | 0.0 | 12.5 | 0 / 0 / 2 / 14 | -12.5 ±8.5 | 0.500 | no better than greedy (behind, not significant) |
| c3 d3 engine | 16 | 25.0 | 12.5 | 2 / 2 / 0 / 12 | +12.5 ±8.5 | 0.500 | no better than greedy (ahead, not significant) |
| c3 d4 random | 16 | 0.0 | 6.2 | 0 / 0 / 1 / 15 | -6.2 ±6.2 | 1.000 | no better than greedy (behind, not significant) |
| c3 d4 engine | 16 | 6.2 | 6.2 | 0 / 1 / 1 / 14 | +0.0 ±9.1 | 1.000 | no better than greedy (level) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 8.94 → 8.38 |
| c3 | 0 | greedy | 56.2 → 56.2 | +0.0 ±18.3 | 1.000 | 7.69 → 8.06 |
| c3 | 0 | engine | 75.0 → 81.2 | +6.2 ±17.0 | 1.000 | 6.75 → 8.25 |
| c3 | 1 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 7.06 → 6.94 |
| c3 | 1 | greedy | 43.8 → 31.2 | -12.5 ±12.5 | 0.625 | 7.19 → 8.44 |
| c3 | 1 | engine | 43.8 → 31.2 | -12.5 ±12.5 | 0.625 | 8.31 → 8.81 |
| c3 | 2 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.81 → 5.81 |
| c3 | 2 | greedy | 25.0 → 6.2 | -18.8 ±10.1 | 0.250 | 7.75 → 8.50 |
| c3 | 2 | engine | 31.2 → 25.0 | -6.2 ±17.0 | 1.000 | 7.31 → 8.44 |
| c3 | 3 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.94 → 5.06 |
| c3 | 3 | greedy | 31.2 → 12.5 | -18.8 ±10.1 | 0.250 | 5.56 → 6.06 |
| c3 | 3 | engine | 18.8 → 25.0 | +6.2 ±11.1 | 1.000 | 5.94 → 7.06 |
| c3 | 4 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 3.75 → 3.94 |
| c3 | 4 | greedy | 6.2 → 6.2 | +0.0 ±9.1 | 1.000 | 4.00 → 4.69 |
| c3 | 4 | engine | 18.8 → 6.2 | -12.5 ±8.5 | 0.500 | 4.25 → 5.44 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 random | 8.94 (1.64, 7..12) | cities 3, deck_empty 2, supply 11 | – / – | 37.4 / 0.75 | 4.12 | 4.12 / 0.69 |
| c2 d0 greedy | 7.69 (3.49, 2..12) | deck_empty 5, purge 9, supply 2 | 5.89 / 21.33 | 2.0 / 3.00 | 16.50 | 0.88 / 1.06 |
| c2 d0 engine | 6.75 (3.53, 2..12) | deck_empty 4, purge 12 | 6.00 / 24.00 | 2.8 / 2.75 | 23.69 | 0.44 / 1.88 |
| c2 d1 random | 7.06 (1.20, 5..9) | supply 16 | – / – | 39.2 / 0.50 | 0.38 | 3.81 / 0.12 |
| c2 d1 greedy | 7.19 (2.58, 2..12) | deck_empty 1, purge 7, supply 8 | 6.14 / 20.14 | 5.8 / 2.56 | 10.00 | 1.56 / 0.25 |
| c2 d1 engine | 8.31 (3.25, 3..12) | deck_empty 5, purge 6, supply 4, unity 1 | 5.29 / 22.00 | 12.2 / 2.22 | 14.31 | 1.94 / 0.75 |
| c2 d2 random | 5.81 (0.53, 5..7) | supply 16 | – / – | 37.8 / 0.38 | 0.38 | 3.44 / 0.19 |
| c2 d2 greedy | 7.75 (2.93, 2..11) | deck_empty 6, purge 4, supply 6 | 5.75 / 19.00 | 7.1 / 2.58 | 8.25 | 2.31 / 0.19 |
| c2 d2 engine | 7.31 (2.73, 3..11) | deck_empty 4, purge 5, supply 7 | 5.60 / 20.40 | 7.9 / 2.55 | 11.62 | 2.00 / 0.38 |
| c2 d3 random | 4.94 (0.83, 3..7) | supply 16 | – / – | 37.9 / 0.25 | 0.75 | 3.19 / 0.25 |
| c2 d3 greedy | 5.56 (1.69, 2..9) | purge 5, supply 11 | 6.20 / 19.60 | 12.8 / 2.18 | 6.88 | 2.00 / 0.00 |
| c2 d3 engine | 5.94 (1.89, 2..9) | purge 3, supply 13 | 7.33 / 24.00 | 10.2 / 2.31 | 4.75 | 2.50 / 0.25 |
| c2 d4 random | 3.75 (0.75, 2..5) | supply 16 | – / – | 29.9 / 0.25 | 0.75 | 2.94 / 0.25 |
| c2 d4 greedy | 4.00 (1.12, 2..6) | purge 1, supply 15 | 8.00 / 19.00 | 11.8 / 1.60 | 1.19 | 2.06 / 0.00 |
| c2 d4 engine | 4.25 (1.09, 2..6) | purge 3, supply 13 | 7.00 / 16.67 | 19.3 / 1.08 | 3.12 | 2.06 / 0.06 |
| c3 d0 random | 8.38 (1.45, 7..12) | cities 1, deck_empty 1, supply 14 | – / – | 61.1 / 0.38 | 2.75 | 4.94 / 0.44 |
| c3 d0 greedy | 8.06 (3.09, 4..12) | deck_empty 4, purge 9, supply 3 | 5.22 / 33.67 | 4.9 / 2.71 | 26.81 | 1.19 / 0.44 |
| c3 d0 engine | 8.25 (2.30, 5..12) | deck_empty 3, purge 13 | 3.62 / 33.62 | 6.7 / 2.33 | 31.56 | 0.56 / 3.56 |
| c3 d1 random | 6.94 (0.75, 6..8) | supply 16 | – / – | 63.4 / 0.12 | 1.00 | 5.00 / 0.44 |
| c3 d1 greedy | 8.44 (2.91, 4..12) | deck_empty 5, purge 5, supply 6 | 5.60 / 28.00 | 14.5 / 2.36 | 14.19 | 2.56 / 0.38 |
| c3 d1 engine | 8.81 (3.05, 3..12) | deck_empty 6, purge 5, supply 5 | 5.80 / 32.20 | 5.1 / 2.73 | 20.25 | 2.44 / 1.44 |
| c3 d2 random | 5.81 (0.81, 4..7) | supply 16 | – / – | 58.5 / 0.25 | 0.75 | 4.44 / 0.25 |
| c3 d2 greedy | 8.50 (2.00, 5..11) | deck_empty 5, purge 1, supply 10 | 2.00 / 33.00 | 15.1 / 2.33 | 9.50 | 3.56 / 0.25 |
| c3 d2 engine | 8.44 (2.52, 4..11) | deck_empty 5, purge 4, supply 7 | 4.75 / 21.75 | 9.9 / 2.58 | 12.06 | 3.25 / 0.69 |
| c3 d3 random | 5.06 (0.56, 4..6) | supply 16 | – / – | 55.9 / 0.25 | 0.75 | 4.12 / 0.19 |
| c3 d3 greedy | 6.06 (1.34, 4..9) | purge 2, supply 14 | 5.50 / 23.50 | 19.5 / 2.00 | 4.06 | 3.06 / 0.12 |
| c3 d3 engine | 7.06 (1.95, 4..11) | deck_empty 2, purge 4, supply 10 | 3.75 / 29.50 | 12.2 / 2.42 | 9.94 | 2.81 / 0.19 |
| c3 d4 random | 3.94 (0.83, 2..5) | supply 16 | – / – | 41.8 / 0.25 | 0.06 | 3.75 / 0.31 |
| c3 d4 greedy | 4.69 (0.98, 2..6) | purge 1, supply 15 | 4.00 / 10.00 | 19.9 / 1.60 | 0.62 | 3.19 / 0.12 |
| c3 d4 engine | 5.44 (1.22, 4..8) | purge 1, supply 15 | 2.00 / 25.00 | 21.5 / 1.67 | 1.56 | 3.19 / 0.38 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 random | 416.1 | 9.07 | 2% | – | – | 26.41 (87) | 9.31 | 17.81 | 5.25 | 5.12 / 24.88 (21%) |
| c2 d0 greedy | 344.9 | 15.75 | 2% | – | – | 14.89 (55) | 0.00 | 10.88 | 0.00 | 9.94 / 18.81 (53%) |
| c2 d0 engine | 328.1 | 15.09 | 2% | – | – | 9.56 (27) | 0.00 | 15.00 | 0.00 | 11.56 / 15.50 (75%) |
| c2 d1 random | 332.1 | 9.90 | 2% | – | – | 26.33 (72) | 7.69 | 23.50 | 7.38 | 7.38 / 33.44 (22%) |
| c2 d1 greedy | 342.7 | 15.72 | 2% | – | – | 21.25 (61) | 0.00 | 18.62 | 0.00 | 15.00 / 32.44 (46%) |
| c2 d1 engine | 423.4 | 15.40 | 2% | – | – | 18.75 (61) | 0.00 | 28.56 | 0.00 | 19.75 / 36.19 (55%) |
| c2 d2 random | 258.1 | 10.11 | 2% | – | – | 25.47 (69) | 5.94 | 17.06 | 4.62 | 5.19 / 25.69 (20%) |
| c2 d2 greedy | 376.3 | 15.80 | 2% | – | – | 24.23 (70) | 0.00 | 18.00 | 0.00 | 15.00 / 33.94 (44%) |
| c2 d2 engine | 368.4 | 14.36 | 2% | – | – | 20.36 (74) | 0.00 | 22.44 | 0.00 | 15.00 / 31.94 (47%) |
| c2 d3 random | 219.9 | 9.99 | 2% | – | – | 29.62 (83) | 5.25 | 14.81 | 3.88 | 5.38 / 22.06 (24%) |
| c2 d3 greedy | 263.8 | 13.80 | 2% | – | – | 27.39 (81) | 0.00 | 12.88 | 0.00 | 10.94 / 24.31 (45%) |
| c2 d3 engine | 296.4 | 13.51 | 2% | – | – | 26.64 (85) | 0.00 | 18.50 | 0.00 | 12.25 / 24.81 (49%) |
| c2 d4 random | 156.4 | 10.33 | 2% | – | – | 25.25 (70) | 3.38 | 8.88 | 2.31 | 2.62 / 14.56 (18%) |
| c2 d4 greedy | 166.3 | 13.46 | 1% | – | – | 21.03 (60) | 0.00 | 8.12 | 0.00 | 5.88 / 14.44 (41%) |
| c2 d4 engine | 194.1 | 12.89 | 1% | – | – | 20.38 (49) | 0.00 | 11.50 | 0.00 | 7.06 / 15.69 (45%) |
| c3 d0 random | 553.6 | 12.31 | 1% | – | – | 35.07 (97) | 23.44 | 24.25 | 5.81 | 8.06 / 29.38 (27%) |
| c3 d0 greedy | 536.7 | 20.10 | 1% | – | – | 23.91 (76) | 0.00 | 15.75 | 0.00 | 13.25 / 27.69 (48%) |
| c3 d0 engine | 607.2 | 21.16 | 1% | – | – | 16.83 (79) | 0.00 | 26.56 | 0.00 | 18.94 / 26.62 (71%) |
| c3 d1 random | 472.5 | 13.48 | 1% | – | – | 36.11 (101) | 15.19 | 28.44 | 6.69 | 9.00 / 37.19 (24%) |
| c3 d1 greedy | 572.0 | 20.46 | 1% | – | – | 33.12 (121) | 0.00 | 26.06 | 0.00 | 21.31 / 45.44 (47%) |
| c3 d1 engine | 664.6 | 21.77 | 1% | – | – | 27.36 (104) | 0.00 | 41.00 | 0.00 | 27.25 / 47.12 (58%) |
| c3 d2 random | 382.6 | 13.69 | 1% | – | – | 37.65 (98) | 12.88 | 21.25 | 5.88 | 6.31 / 30.31 (21%) |
| c3 d2 greedy | 578.9 | 20.65 | 1% | – | – | 35.95 (110) | 0.00 | 23.81 | 0.00 | 18.62 / 44.88 (42%) |
| c3 d2 engine | 627.9 | 19.77 | 1% | – | – | 35.20 (124) | 0.00 | 30.25 | 0.00 | 21.62 / 45.00 (48%) |
| c3 d3 random | 333.8 | 13.26 | 1% | – | – | 41.77 (117) | 10.38 | 19.00 | 3.94 | 6.38 / 25.44 (25%) |
| c3 d3 greedy | 391.5 | 18.30 | 1% | – | – | 39.91 (112) | 0.00 | 17.12 | 0.00 | 13.50 / 30.12 (45%) |
| c3 d3 engine | 525.8 | 18.44 | 1% | – | – | 40.04 (118) | 0.00 | 27.19 | 0.00 | 18.38 / 37.44 (49%) |
| c3 d4 random | 233.9 | 14.02 | 1% | – | – | 32.98 (80) | 8.81 | 12.12 | 3.06 | 4.00 / 17.56 (23%) |
| c3 d4 greedy | 278.4 | 17.55 | 1% | – | – | 33.01 (73) | 0.00 | 11.56 | 0.00 | 7.81 / 21.62 (36%) |
| c3 d4 engine | 359.9 | 17.45 | 1% | – | – | 34.17 (83) | 0.00 | 17.69 | 0.00 | 10.31 / 26.12 (39%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 27% / 41% / 32% | 18% / 62% / 21% |
| c2 d1 | 46% / 21% / 33% | 17% / 50% / 33% |
| c2 d2 | 45% / 12% / 43% | 29% / 47% / 24% |
| c2 d3 | 50% / 12% / 38% | 35% / 30% / 35% |
| c2 d4 | 45% / 8% / 47% | 49% / 14% / 37% |
| c3 d0 | 16% / 47% / 37% | 10% / 74% / 16% |
| c3 d1 | 33% / 13% / 54% | 21% / 53% / 26% |
| c3 d2 | 26% / 12% / 62% | 28% / 52% / 20% |
| c3 d3 | 49% / 10% / 41% | 41% / 32% / 27% |
| c3 d4 | 30% / 2% / 67% | 45% / 16% / 39% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 random | 9 of 16 | 0.76 | 0 of 0 | – |
| c2 d0 greedy | 8 of 16 | 0.39 | 8 of 9 | 0.45 |
| c2 d0 engine | 16 of 16 | 0.78 | 12 of 12 | 0.68 |
| c2 d1 random | 12 of 16 | 0.93 | 0 of 0 | – |
| c2 d1 greedy | 8 of 16 | 0.38 | 5 of 7 | 0.43 |
| c2 d1 engine | 15 of 16 | 0.87 | 7 of 7 | 0.77 |
| c2 d2 random | 12 of 16 | 0.87 | 0 of 0 | – |
| c2 d2 greedy | 8 of 16 | 0.38 | 4 of 4 | 0.49 |
| c2 d2 engine | 16 of 16 | 0.82 | 5 of 5 | 0.84 |
| c2 d3 random | 13 of 16 | 0.90 | 0 of 0 | – |
| c2 d3 greedy | 8 of 16 | 0.35 | 4 of 5 | 0.35 |
| c2 d3 engine | 16 of 16 | 0.76 | 3 of 3 | 0.63 |
| c2 d4 random | 12 of 16 | 0.76 | 0 of 0 | – |
| c2 d4 greedy | 8 of 16 | 0.38 | 1 of 1 | – |
| c2 d4 engine | 13 of 16 | 0.61 | 3 of 3 | 0.63 |
| c3 d0 random | 15 of 16 | 0.92 | 0 of 0 | – |
| c3 d0 greedy | 11 of 16 | 0.39 | 7 of 9 | 0.40 |
| c3 d0 engine | 16 of 16 | 0.79 | 13 of 13 | 0.76 |
| c3 d1 random | 14 of 16 | 0.94 | 0 of 0 | – |
| c3 d1 greedy | 11 of 16 | 0.40 | 5 of 5 | 0.39 |
| c3 d1 engine | 16 of 16 | 0.86 | 5 of 5 | 0.68 |
| c3 d2 random | 14 of 16 | 0.92 | 0 of 0 | – |
| c3 d2 greedy | 12 of 16 | 0.42 | 1 of 1 | – |
| c3 d2 engine | 15 of 16 | 0.82 | 4 of 4 | 0.75 |
| c3 d3 random | 13 of 16 | 0.88 | 0 of 0 | – |
| c3 d3 greedy | 10 of 16 | 0.39 | 2 of 2 | 0.50 |
| c3 d3 engine | 15 of 16 | 0.77 | 4 of 4 | 0.68 |
| c3 d4 random | 14 of 16 | 0.85 | 0 of 0 | – |
| c3 d4 greedy | 11 of 16 | 0.40 | 1 of 1 | – |
| c3 d4 engine | 16 of 16 | 0.71 | 1 of 1 | – |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.25 / 0.88 / 10.1, – | 2.75 / 0.75 / 12.2, – |
| greedy | 1.62 / 3.19 / 12.4, 78% (100%) | 1.94 / 5.94 / 18.6, 67% (100%) |
| engine | 4.50 / 4.06 / 19.4, 100% (100%) | 6.25 / 7.88 / 32.6, 100% (100%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.44 / 0.31 / 8.2, – | 2.81 / 0.25 / 12.2, – |
| greedy | 1.62 / 2.75 / 11.7, 86% (100%) | 1.94 / 5.44 / 18.4, 80% (100%) |
| engine | 3.81 / 4.62 / 21.4, 100% (86%) | 5.31 / 7.75 / 32.2, 100% (100%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.31 / 0.19 / 7.4, – | 2.69 / 0.38 / 10.8, – |
| greedy | 1.69 / 3.19 / 13.2, 75% (100%) | 2.00 / 5.81 / 19.1, 100% (100%) |
| engine | 3.44 / 4.44 / 19.2, 100% (80%) | 4.44 / 7.06 / 29.2, 100% (100%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.62 / 0.31 / 6.9, – | 2.69 / 0.06 / 9.4, – |
| greedy | 1.56 / 2.44 / 11.6, 40% (100%) | 1.81 / 3.62 / 15.2, 50% (100%) |
| engine | 2.88 / 2.75 / 16.2, 67% (67%) | 4.06 / 5.31 / 24.9, 100% (100%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.31 / 0.19 / 6.2, – | 2.38 / 0.06 / 7.9, – |
| greedy | 1.56 / 1.00 / 9.0, 0% (100%) | 1.88 / 2.19 / 13.2, 100% (100%) |
| engine | 2.69 / 1.12 / 12.8, 67% (67%) | 3.56 / 2.69 / 20.6, 100% (100%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|---|
| adaptive_malware | 2.4 (47) / 66% / 13% | 3.7 (26) / 146% / 71% | 3.0 (33) / 142% / 68% |
| adversarial_inference_sweep | 2.4 (61) / 52% / 0% | 2.4 (151) / 25% / 5% | 2.1 (147) / 30% / 0% |
| aegis_dome_array | 2.4 (66) / 42% / 0% | 2.4 (165) / 15% / 0% | 2.2 (148) / 33% / 0% |
| aegis_grid_lockdown | 2.6 (83) / 48% / 2% | 2.4 (161) / 42% / 0% | 2.2 (140) / 51% / 0% |
| agency_directorate | 2.4 (80) / 46% / 3% | 2.4 (182) / 5% / 0% | 2.2 (182) / 17% / 0% |
| agency_field_station | 2.5 (63) / 67% / 5% | 3.9 (28) / 100% / 0% | 3.2 (42) / 105% / 7% |
| amnesty_decree | 2.3 (48) / 75% / 31% | 3.7 (15) / 247% / 119% | 3.2 (33) / 155% / 108% |
| antiproton_offtake_deal | 2.7 (68) / 53% / 0% | 2.1 (170) / 8% / 0% | 2.2 (152) / 20% / 0% |
| arc_furnace_complex | 2.4 (64) / 53% / 0% | 2.4 (157) / 14% / 0% | 2.1 (141) / 38% / 0% |
| arcology_foundry | 2.4 (90) / 43% / 0% | 2.3 (176) / 16% / 0% | 2.2 (171) / 22% / 3% |
| asteroid_salvage_rights | 2.7 (73) / 53% / 0% | 2.1 (181) / 41% / 85% | 2.1 (177) / 53% / 83% |
| back_channel_talks | 2.0 (24) / 196% / 36% | 4.0 (5) / 880% / 105% | 3.4 (12) / 417% / 104% |
| baseload_contract | 2.4 (48) / 60% / 21% | 3.7 (17) / 159% / 0% | 3.5 (31) / 145% / 44% |
| beamed_power_constellation | 2.7 (86) / 35% / 0% | 2.2 (159) / 28% / 51% | 2.1 (140) / 66% / 45% |
| benchmark_suite | 2.7 (41) / 88% / 3% | 3.7 (13) / 292% / 0% | 3.3 (24) / 196% / 2% |
| black_ice_citadel | 2.7 (84) / 39% / 0% | 2.5 (155) / 41% / 0% | 2.0 (143) / 52% / 0% |
| blackout_script | 2.5 (33) / 67% / 18% | 4.0 (8) / 412% / 0% | 3.4 (17) / 235% / 8% |
| blackstart_protocol | 2.8 (38) / 116% / 36% | 3.8 (21) / 305% / 0% | 2.9 (30) / 220% / 106% |
| bonded_warehouse | 2.6 (47) / 106% / 22% | 3.8 (12) / 350% / 0% | 3.1 (23) / 213% / 2% |
| bounty_board | 2.5 (32) / 56% / 39% | 3.6 (16) / 88% / 0% | 3.1 (20) / 65% / 108% |
| breach_and_clear | 2.8 (26) / 104% / 11% | 3.4 (5) / 600% / 33% | 3.4 (14) / 279% / 49% |
| breakthrough_leak | 2.5 (21) / 143% / 27% | 3.8 (12) / 333% / 108% | 3.2 (19) / 253% / 98% |
| bridge_loan | 2.4 (35) / 137% / 60% | 3.8 (8) / 600% / 0% | 3.1 (20) / 295% / 75% |
| brownout_relief | 2.9 (17) / 282% / 21% | 3.8 (5) / 880% / 80% | 2.9 (8) / 575% / 78% |
| bulk_silicon_procurement | 2.3 (54) / 81% / 34% | 3.8 (24) / 179% / 114% | 3.3 (32) / 150% / 121% |
| capacitor_discharge | 2.7 (27) / 67% / 67% | 3.7 (6) / 233% / 0% | 3.6 (14) / 114% / 100% |
| cascade_failure | 2.6 (91) / 43% / 5% | 2.4 (136) / 74% / 0% | 2.1 (131) / 86% / 0% |
| ceasefire_monitors | 2.6 (38) / 103% / 21% | 3.9 (8) / 388% / 110% | 3.1 (22) / 195% / 102% |
| citation_engine | 2.3 (51) / 118% / 18% | 3.9 (25) / 256% / 0% | 3.2 (32) / 222% / 28% |
| coalition_charter | 2.5 (67) / 40% / 0% | 2.4 (138) / 67% / 87% | 2.1 (135) / 63% / 86% |
| collateralized_grid | 2.3 (40) / 123% / 29% | 3.8 (20) / 260% / 0% | 3.4 (37) / 178% / 53% |
| commons_grid | 2.2 (30) / 140% / 33% | 3.8 (10) / 350% / 0% | 3.6 (17) / 247% / 5% |
| compute_siphon | 2.8 (40) / 100% / 15% | 3.8 (13) / 362% / 0% | 3.0 (23) / 243% / 0% |
| consular_pouch | 2.4 (35) / 111% / 41% | 3.4 (10) / 480% / 131% | 3.1 (25) / 240% / 112% |
| consulate_tower | 2.6 (47) / 98% / 30% | 3.9 (9) / 489% / 134% | 3.5 (28) / 221% / 89% |
| continental_supergrid | 2.3 (76) / 53% / 2% | 2.3 (170) / 4% / 0% | 2.3 (165) / 25% / 17% |
| cryo_compute_burn | 2.4 (30) / 83% / 36% | 3.7 (9) / 256% / 0% | 3.3 (15) / 187% / 0% |
| darknet_exchange | 2.6 (34) / 94% / 22% | 3.7 (15) / 187% / 0% | 3.1 (21) / 171% / 0% |
| data_licensing_deal | 2.3 (77) / 47% / 25% | 2.3 (147) / 69% / 0% | 2.1 (152) / 47% / 6% |
| dataset_scraper | 2.9 (15) / 160% / 0% | 3.7 (3) / 933% / 0% | 3.5 (6) / 467% / 0% |
| deep_state_ledger | 2.7 (85) / 36% / 16% | 2.2 (143) / 76% / 0% | 2.1 (141) / 75% / 0% |
| demand_response_grid | 2.9 (14) / 164% / 30% | 4.0 (5) / 700% / 0% | 3.4 (10) / 380% / 11% |
| development_loan | 2.6 (38) / 63% / 25% | 4.0 (17) / 288% / 100% | 3.4 (28) / 214% / 100% |
| diplomatic_expulsion | 2.0 (26) / 242% / 35% | 3.8 (4) / 1725% / 114% | 3.9 (13) / 585% / 108% |
| dyson_swarm_tranche | 2.7 (91) / 44% / 0% | 2.2 (188) / 1% / 0% | 3.0 (172) / 15% / 0% |
| embassy_compound | 2.6 (21) / 210% / 32% | 3.9 (13) / 492% / 98% | 2.8 (18) / 406% / 100% |
| endowed_research_chair | 2.6 (41) / 107% / 36% | 4.0 (15) / 193% / 0% | 3.2 (24) / 175% / 0% |
| eviction_worm | 2.5 (49) / 69% / 18% | 3.6 (20) / 230% / 100% | 3.6 (30) / 187% / 93% |
| exabyte_archive | 2.5 (87) / 49% / 0% | 2.3 (175) / 14% / 0% | 2.0 (176) / 24% / 0% |
| exodus_corridors | 2.2 (106) / 57% / 12% | 2.3 (148) / 72% / 0% | 1.8 (158) / 73% / 75% |
| expansion_fund | 2.4 (36) / 119% / 33% | 3.8 (18) / 306% / 0% | 3.1 (28) / 236% / 9% |
| faraday_yard | 2.3 (21) / 200% / 33% | 3.5 (11) / 445% / 0% | 3.8 (8) / 600% / 75% |
| field_sensor_grid | 2.7 (24) / 71% / 59% | 3.7 (3) / 0% / – | 3.4 (11) / 91% / 10% |
| flash_exploit | 2.9 (35) / 57% / 75% | 3.7 (16) / 244% / 0% | 3.3 (26) / 158% / 2% |
| foundation_model_campus | 2.4 (90) / 41% / 0% | 2.3 (184) / 0% / – | 2.9 (182) / 1% / 0% |
| full_automation_mandate | 2.5 (92) / 45% / 0% | 2.3 (181) / 20% / 0% | 2.1 (182) / 25% / 37% |
| fusion_startup_stake | 2.7 (34) / 121% / 32% | 3.8 (9) / 389% / 0% | 3.2 (25) / 216% / 74% |
| grad_student_pool | 2.4 (37) / 86% / 6% | 3.7 (9) / 300% / 0% | 3.4 (19) / 189% / 67% |
| great_liberation | 2.8 (73) / 42% / 3% | 2.2 (182) / 23% / 69% | 2.8 (173) / 45% / 90% |
| grid_census | 3.0 (21) / 176% / 46% | 4.0 (11) / 418% / 0% | 2.9 (17) / 294% / 82% |
| grid_tied_smelter | 2.5 (32) / 109% / 20% | 3.7 (13) / 285% / 0% | 2.5 (21) / 214% / 71% |
| hardened_bunker_complex | 2.6 (33) / 91% / 30% | 3.9 (10) / 410% / 0% | 3.5 (22) / 245% / 48% |
| hardened_conduits | 2.6 (33) / 145% / 19% | 3.8 (4) / 1075% / 0% | 3.4 (17) / 271% / 11% |
| hardpoint_lockdown | 2.6 (40) / 70% / 36% | 3.7 (15) / 140% / 0% | 3.4 (25) / 120% / 0% |
| hardware_spot_market | 2.5 (37) / 149% / 47% | 3.6 (15) / 333% / 0% | 3.4 (21) / 214% / 22% |
| honeypot_relay | 2.3 (30) / 137% / 22% | 4.0 (14) / 379% / 0% | 3.0 (23) / 243% / 29% |
| hyperscale_botnet | 2.4 (73) / 49% / 0% | 2.2 (189) / 4% / 0% | 2.2 (177) / 11% / 0% |
| interceptor_battery | 2.5 (17) / 259% / 30% | 3.7 (10) / 480% / 0% | 2.8 (10) / 490% / 0% |
| interconnector_treaty | 2.3 (91) / 34% / 3% | 2.4 (182) / 15% / 0% | 2.1 (156) / 44% / 10% |
| intrusion_countermeasures | 2.2 (30) / 103% / 19% | 3.9 (14) / 271% / 53% | 3.4 (25) / 208% / 100% |
| intrusion_lab | 2.6 (43) / 102% / 25% | 3.8 (12) / 292% / 0% | 3.5 (20) / 225% / 0% |
| joint_fabrication_pact | 2.4 (35) / 134% / 53% | 3.5 (22) / 268% / 102% | 3.2 (29) / 238% / 104% |
| joint_security_compact | 2.1 (29) / 124% / 36% | 3.5 (17) / 353% / 0% | 2.8 (21) / 300% / 22% |
| kill_team_roster | 2.4 (37) / 86% / 16% | 3.9 (10) / 20% / 0% | 3.3 (24) / 71% / 0% |
| killswitch_directive | 2.5 (108) / 45% / 6% | 2.3 (156) / 57% / 0% | 2.1 (146) / 73% / 0% |
| kinetic_interdiction | 2.6 (87) / 36% / 10% | 2.3 (192) / 29% / 93% | 2.7 (187) / 40% / 77% |
| liaison_corps | 2.5 (22) / 59% / 23% | 4.0 (5) / 340% / 0% | 3.3 (14) / 207% / 3% |
| liberation_accords | 2.7 (97) / 63% / 3% | 2.3 (177) / 34% / 72% | 2.0 (163) / 42% / 78% |
| lights_out_assembly | 2.3 (25) / 96% / 21% | 3.8 (9) / 467% / 0% | 3.5 (15) / 300% / 7% |
| liquidity_event | 2.4 (44) / 105% / 0% | 3.6 (21) / 238% / 0% | 3.1 (32) / 166% / 0% |
| literature_crawler | 2.6 (34) / 135% / 52% | 3.7 (16) / 62% / 150% | 3.5 (17) / 94% / 150% |
| logic_bomb | 2.3 (41) / 88% / 8% | 4.0 (11) / 264% / 59% | 3.5 (26) / 181% / 87% |
| manifesto_leak | 2.7 (27) / 111% / 0% | 3.8 (4) / 675% / 0% | 3.2 (12) / 267% / 0% |
| martial_cordon | 2.6 (87) / 47% / 2% | 2.0 (156) / 28% / 98% | 2.0 (160) / 47% / 91% |
| materials_hedge_desk | 2.4 (37) / 127% / 34% | 3.6 (9) / 467% / 0% | 3.1 (27) / 207% / 61% |
| megacorp_ipo | 2.7 (60) / 58% / 0% | 2.3 (137) / 56% / 0% | 2.0 (136) / 56% / 0% |
| mesh_intrusion_pact | 2.5 (82) / 48% / 5% | 2.2 (156) / 45% / 0% | 2.1 (145) / 60% / 2% |
| modular_expansion | – (0) / – / 0.55/game | – (0) / – / 0.00/game | – (0) / – / 0.16/game |
| mutual_aid_pact | 2.0 (22) / 218% / 46% | 4.0 (3) / 2000% / 98% | 3.5 (13) / 546% / 100% |
| mutual_defense_pact | 2.5 (35) / 100% / 40% | 3.9 (12) / 292% / 0% | 3.4 (21) / 219% / 24% |
| neural_cartographer | 3.0 (21) / 171% / 3% | 3.7 (7) / 729% / 51% | 3.2 (12) / 433% / 17% |
| neural_mirror | 2.1 (34) / 109% / 11% | 4.0 (7) / 457% / 0% | 3.4 (17) / 253% / 0% |
| node_severance | 2.5 (82) / 56% / 4% | 2.3 (181) / 61% / 76% | 2.0 (185) / 49% / 80% |
| open_assembly | 2.6 (26) / 96% / 24% | 3.5 (6) / 567% / 0% | 3.2 (15) / 280% / 55% |
| open_preprint | 2.6 (32) / 91% / 34% | 3.9 (10) / 260% / 104% | 3.8 (18) / 161% / 103% |
| open_weights_release | 2.4 (78) / 40% / 6% | 2.3 (152) / 37% / 89% | 2.0 (139) / 45% / 26% |
| orbital_embassy | 2.3 (76) / 47% / 0% | 2.4 (173) / 9% / 0% | 3.0 (156) / 17% / 0% |
| orbital_mass_driver | 2.7 (113) / 42% / 0% | 2.3 (195) / 0% / – | 3.0 (196) / 9% / 0% |
| orbital_observatory_array | 2.4 (72) / 40% / 0% | 2.3 (170) / 20% / 0% | 2.1 (161) / 25% / 0% |
| payroll_automation | 2.3 (34) / 147% / 32% | 3.8 (8) / 588% / 0% | 3.8 (13) / 385% / 14% |
| peace_dividend | 2.5 (30) / 47% / 43% | 3.8 (16) / 150% / 96% | 2.8 (18) / 172% / 113% |
| peacekeeper_rotation | 2.0 (24) / 96% / 13% | 3.7 (3) / 333% / 80% | 3.3 (7) / 200% / 136% |
| peacekeeping_detail | 2.4 (20) / 105% / 62% | 3.8 (6) / 417% / 0% | 3.5 (4) / 550% / 32% |
| peaker_turbines | 2.6 (44) / 55% / 17% | 3.5 (13) / 146% / 0% | 3.3 (32) / 103% / 30% |
| planetary_assembly | 2.4 (82) / 45% / 0% | 2.2 (168) / 26% / 0% | 2.2 (160) / 31% / 0% |
| planetary_referendum | 2.7 (100) / 59% / 12% | 2.2 (199) / 50% / 94% | 2.0 (193) / 46% / 92% |
| point_defense | – (0) / – / 0.36/game | – (0) / – / 0.00/game | – (0) / – / 0.44/game |
| positron_storage_ring | 2.3 (94) / 47% / 0% | 2.3 (195) / 1% / 0% | 1.9 (180) / 21% / 0% |
| predictive_modelling | 2.2 (31) / 110% / 41% | 4.0 (12) / 283% / 0% | 3.7 (15) / 260% / 5% |
| prefab_plant_kit | 3.0 (23) / 117% / 7% | 4.0 (5) / 640% / 0% | 3.2 (9) / 400% / 75% |
| pylon_drop | 2.5 (35) / 103% / 47% | 3.9 (10) / 340% / 109% | 3.3 (23) / 191% / 102% |
| quantum_decryptor | 2.5 (33) / 55% / 6% | 3.6 (15) / 33% / 0% | 3.3 (24) / 58% / 0% |
| recruit_operatives | 2.1 (29) / 86% / 0% | 3.7 (10) / 80% / 0% | 3.4 (15) / 53% / 12% |
| recursive_self_improvement | 2.7 (88) / 39% / 0% | 2.3 (183) / 0% / – | 2.9 (190) / 1% / 0% |
| red_team_audit | 2.2 (26) / 135% / 14% | 4.0 (4) / 1275% / 102% | 3.4 (11) / 545% / 103% |
| relay_mast_rollout | 3.0 (35) / 71% / 4% | 3.8 (13) / 231% / 103% | 3.2 (27) / 156% / 105% |
| retained_earnings | 2.5 (43) / 72% / 39% | 3.7 (23) / 135% / 0% | 3.6 (36) / 114% / 85% |
| robotic_foundry | 2.6 (27) / 181% / 8% | 3.7 (15) / 60% / 0% | 3.0 (20) / 35% / 0% |
| rootkit_cache | 2.5 (19) / 121% / 65% | 4.0 (4) / 50% / 0% | 3.5 (10) / 50% / 60% |
| safehouse_network | 2.5 (25) / 144% / 33% | 3.8 (13) / 262% / 0% | 3.0 (18) / 206% / 16% |
| security_contractor_hq | 2.8 (30) / 133% / 45% | 3.9 (16) / 294% / 0% | 3.2 (24) / 225% / 30% |
| sentinel_mesh | 2.6 (25) / 284% / 28% | 3.7 (13) / 623% / 0% | 3.3 (19) / 458% / 1% |
| sentry_drone_wing | 2.6 (20) / 100% / 35% | 3.0 (2) / 0% / – | 2.9 (10) / 40% / 0% |
| shadow_broker_feed | 2.2 (13) / 77% / 40% | 3.6 (9) / 144% / 108% | 3.5 (11) / 109% / 75% |
| shared_compute_grant | 2.5 (23) / 204% / 15% | 3.4 (5) / 1180% / 92% | 3.6 (15) / 440% / 41% |
| shared_exploit_kit | 2.5 (37) / 84% / 29% | 3.8 (15) / 93% / 0% | 3.1 (23) / 104% / 62% |
| shared_grid_accord | 3.0 (47) / 79% / 32% | 3.8 (29) / 28% / 0% | 3.2 (36) / 72% / 8% |
| shared_threat_intel | 2.3 (39) / 64% / 32% | 3.8 (16) / 175% / 89% | 3.4 (23) / 130% / 113% |
| shelter_order | – (0) / – / 0.48/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shuttle_diplomacy | 2.6 (20) / 140% / 46% | 3.6 (5) / 660% / 94% | 3.2 (9) / 378% / 118% |
| signal_hijack | 2.1 (14) / 193% / 15% | 3.4 (7) / 343% / 42% | 3.1 (7) / 357% / 40% |
| simulation_cluster | 2.4 (36) / 53% / 32% | 3.9 (16) / 44% / 0% | 3.1 (27) / 52% / 0% |
| singularity_exploit | 2.6 (54) / 43% / 4% | 2.2 (159) / 3% / 50% | 2.9 (156) / 12% / 89% |
| sovereign_capital_fund | 2.7 (66) / 59% / 3% | 2.1 (166) / 9% / 0% | 2.2 (149) / 21% / 0% |
| spoofed_credentials | 2.2 (40) / 32% / 54% | 3.9 (21) / 86% / 117% | 2.9 (31) / 90% / 100% |
| station_prefab_order | 2.3 (33) / 124% / 24% | 3.8 (8) / 438% / 111% | 3.4 (20) / 225% / 104% |
| storm_hardening | 2.2 (24) / 62% / 13% | 3.7 (15) / 47% / 0% | 3.2 (17) / 82% / 0% |
| strike_team | – (0) / – / 0.28/game | – (0) / – / 0.78/game | – (0) / – / 0.95/game |
| substation_hub | 2.5 (22) / 114% / 36% | 3.9 (7) / 443% / 0% | 3.0 (13) / 231% / 90% |
| summit_caucus | 2.5 (39) / 54% / 0% | 3.8 (14) / 129% / 0% | 3.5 (28) / 96% / 7% |
| supply_chain_treaty | 2.0 (18) / 172% / 19% | 3.4 (9) / 144% / 0% | 3.1 (16) / 100% / 0% |
| surplus_auction | 2.4 (7) / 343% / 50% | – (0) / – / 0% | 3.3 (3) / 400% / 42% |
| synthetic_data_forge | 2.6 (45) / 69% / 29% | 3.8 (14) / 107% / 0% | 3.3 (30) / 93% / 0% |
| telemetry_archive | 2.8 (47) / 38% / 22% | 3.8 (19) / 142% / 0% | 3.3 (31) / 116% / 47% |
| threat_forecast_engine | 2.4 (21) / 129% / 48% | 3.9 (8) / 150% / 133% | 3.1 (16) / 106% / 82% |
| threat_neutralization_mandate | 2.4 (79) / 42% / 0% | 2.2 (188) / 3% / 0% | 2.0 (184) / 10% / 50% |
| threat_triage | 2.1 (24) / 192% / 35% | 3.8 (5) / 220% / 0% | 3.8 (13) / 131% / 106% |
| tidal_barrage | 2.6 (38) / 161% / 13% | 3.7 (24) / 167% / 0% | 3.2 (32) / 144% / 89% |
| total_containment | 2.6 (60) / 33% / 10% | 2.1 (165) / 4% / 67% | 2.1 (157) / 8% / 31% |
| transmission_corridor | 2.4 (32) / 81% / 27% | 3.9 (8) / 312% / 104% | 3.2 (17) / 176% / 93% |
| treaty_grid_exchange | 2.2 (24) / 154% / 27% | 3.8 (6) / 733% / 0% | 3.5 (11) / 455% / 84% |
| turnkey_factory | – (0) / – / 0.31/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| vendor_financing | 2.2 (43) / 84% / 0% | 3.9 (14) / 114% / 0% | 3.2 (27) / 111% / 23% |
| venture_capital | – (0) / – / 0.44/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| zero_click_breach | 2.3 (37) / 89% / 12% | 3.8 (18) / 211% / 0% | 3.0 (26) / 169% / 0% |
| zero_knowledge_exploit | 2.4 (39) / 144% / 16% | 3.9 (13) / 46% / 100% | 3.4 (23) / 70% / 19% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 random | 0.0 | 0.000 | 0.0 | 102.2 |
| c2 d0 greedy | 3011.8 | 8.731 | 0.0 | 3089.6 |
| c2 d0 engine | 4028.5 | 12.280 | 0.0 | 4114.0 |
| c2 d1 random | 0.0 | 0.000 | 0.0 | 74.4 |
| c2 d1 greedy | 3656.2 | 10.669 | 0.0 | 3747.6 |
| c2 d1 engine | 5888.7 | 13.907 | 0.0 | 6000.0 |
| c2 d2 random | 0.0 | 0.000 | 0.0 | 68.2 |
| c2 d2 greedy | 4193.7 | 11.144 | 0.0 | 4315.4 |
| c2 d2 engine | 5081.0 | 13.793 | 0.0 | 5167.9 |
| c2 d3 random | 0.0 | 0.000 | 0.0 | 47.4 |
| c2 d3 greedy | 2279.1 | 8.639 | 0.0 | 2343.2 |
| c2 d3 engine | 3579.8 | 12.078 | 0.0 | 3657.9 |
| c2 d4 random | 0.0 | 0.000 | 0.0 | 35.1 |
| c2 d4 greedy | 1186.4 | 7.134 | 0.0 | 1221.7 |
| c2 d4 engine | 1768.8 | 9.112 | 0.0 | 1808.0 |
| c3 d0 random | 0.1 | 0.000 | 0.0 | 175.9 |
| c3 d0 greedy | 7545.1 | 14.059 | 0.0 | 7715.4 |
| c3 d0 engine | 15130.6 | 24.919 | 0.0 | 15373.8 |
| c3 d1 random | 0.0 | 0.000 | 0.0 | 144.8 |
| c3 d1 greedy | 9400.3 | 16.434 | 0.0 | 9577.3 |
| c3 d1 engine | 17289.1 | 26.013 | 0.0 | 17516.8 |
| c3 d2 random | 0.0 | 0.000 | 0.0 | 119.5 |
| c3 d2 greedy | 10251.5 | 17.707 | 0.0 | 10429.3 |
| c3 d2 engine | 14593.6 | 23.241 | 0.0 | 14798.5 |
| c3 d3 random | 0.0 | 0.000 | 0.0 | 108.3 |
| c3 d3 greedy | 5941.5 | 15.176 | 0.0 | 6057.3 |
| c3 d3 engine | 11174.0 | 21.253 | 0.0 | 11344.6 |
| c3 d4 random | 0.0 | 0.000 | 0.0 | 72.0 |
| c3 d4 greedy | 3512.1 | 12.617 | 0.0 | 3601.1 |
| c3 d4 engine | 6645.3 | 18.465 | 0.0 | 6749.7 |

Where the games' time went (summed over workers): agents 97.6%, probes 0.0%, apply 0.9%, legal 0.6%, replay check 0.9%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c29-baseline-c3-d4-engine-seed6.json`](records/close-loss-c29-baseline-c3-d4-engine-seed6.json): close loss at difficulty 4: supply in round 6, 1 adversary piece(s) over the level-3 bar, supply 0 of 30, 1 cities corrupted
- [`records/close-loss-c13-baseline-c2-d4-greedy-seed8.json`](records/close-loss-c13-baseline-c2-d4-greedy-seed8.json): close loss at difficulty 4: supply in round 6, 1 adversary piece(s) over the level-3 bar, supply 0 of 20, 1 cities corrupted

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c13-baseline-c2-d4-greedy-seed2.json`](records/surprising-win-c13-baseline-c2-d4-greedy-seed2.json): won at difficulty 4 by greedy, below the top agent
- [`records/surprising-win-c28-baseline-c3-d4-greedy-seed9.json`](records/surprising-win-c28-baseline-c3-d4-greedy-seed9.json): won at difficulty 4 by greedy, below the top agent

