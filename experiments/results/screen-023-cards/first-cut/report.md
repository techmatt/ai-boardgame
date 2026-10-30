# Cycle report: screen-023-cards

023 quick screen (r14): the player card set (B2). 2 and 3 countries, d0 to d4, random, greedy and engine: win rates, pick and play rates per card, tags used by winning engines, hand size, insight on keeps versus plays.

480 games (30 cells × 16 seeds, 0..=15) on 12 threads in 327.6 s. Engine 0.1.0, rules r14, content fa0d3136e4cd45a3.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates random 0%, greedy 31%, engine 50%; skill gap +50 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) random 8.50, greedy 12.25, engine 12.50; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c2): win rates random 0%, greedy 6%, engine 6%; skill gap +6 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c2): survival (rounds; wins count extra) random 6.94, greedy 8.69, engine 11.06; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c2): win rates random 0%, greedy 0%, engine 12%; skill gap +12 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.62, greedy 7.50, engine 8.75; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c2): win rates random 0%, greedy 0%, engine 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c2): survival (rounds; wins count extra) random 4.75, greedy 6.38, engine 6.56; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c2): win rates random 0%, greedy 0%, engine 6%; skill gap +6 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) random 3.88, greedy 4.38, engine 5.06; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): win rates random 0%, greedy 0%, engine 25%; skill gap +25 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) random 7.81, greedy 10.25, engine 12.25; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c3): win rates random 0%, greedy 0%, engine 12%; skill gap +12 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c3): survival (rounds; wins count extra) random 7.00, greedy 8.12, engine 11.50; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates random 0%, greedy 0%, engine 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) random 6.06, greedy 7.50, engine 8.81; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c3): win rates random 0%, greedy 0%, engine 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.00, greedy 6.00, engine 6.44; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates random 0%, greedy 0%, engine 0%; skill gap +0 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) random 3.94, greedy 4.50, engine 4.62; the ladder is ordered.
- Skill gap from difficulty 0 to 4 (mean over 2 configs/variants): +38 → +3 pts win rate, +4.22 → +0.94 survival (target: widens with difficulty).
- Game length for the strongest agent (engine; configs/baseline.json, c2): 9.3 rounds at difficulty 0, 4.9 at 4 (target 8–10).
- Game length for the strongest agent (engine; configs/baseline.json, c3): 10.8 rounds at difficulty 0, 4.6 at 4 (target 8–10).
- Games ended by: cities 10, deck_empty 72, purge 24, supply 374.
- Adversary operations: 29.5 per round on average across cells, at most 123 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: none; over 90%: adaptive_malware, agency_field_station, amnesty_decree, back_channel_talks, baseload_contract, benchmark_suite, blackout_script, blackstart_protocol, bonded_warehouse, breach_and_clear, breakthrough_leak, bridge_loan, brownout_relief, bulk_silicon_procurement, ceasefire_monitors, citation_engine, collateralized_grid, commons_grid, compute_siphon, consular_pouch, consulate_tower, cryo_compute_burn, darknet_exchange, dataset_scraper, demand_response_grid, development_loan, diplomatic_expulsion, embassy_compound, endowed_research_chair, eviction_worm, expansion_fund, faraday_yard, flash_exploit, fusion_startup_stake, grad_student_pool, grid_census, grid_tied_smelter, hardened_bunker_complex, hardened_conduits, hardpoint_lockdown, hardware_spot_market, honeypot_relay, interceptor_battery, intrusion_countermeasures, intrusion_lab, joint_fabrication_pact, joint_security_compact, liaison_corps, lights_out_assembly, liquidity_event, logic_bomb, manifesto_leak, materials_hedge_desk, mutual_aid_pact, mutual_defense_pact, neural_cartographer, neural_mirror, open_assembly, payroll_automation, peaker_turbines, predictive_modelling, prefab_plant_kit, pylon_drop, red_team_audit, relay_mast_rollout, security_contractor_hq, sentinel_mesh, shared_compute_grant, shuttle_diplomacy, substation_hub, telemetry_archive, tidal_barrage, treaty_grid_exchange.
- Luck share of survival (seed's share of the variance): 27% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 54% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 50% wins) plays 6.00 buildings and 0.00 tier-2 cards per game (of 32.1 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 1 (c2): the top agent there (engine, 6% wins) plays 5.75 buildings and 0.00 tier-2 cards per game (of 35.4 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 2 (c2): the top agent there (engine, 12% wins) plays 4.69 buildings and 0.00 tier-2 cards per game (of 27.2 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 3 (c2): the top agent there (engine, 0% wins) plays 3.81 buildings and 0.00 tier-2 cards per game (of 22.4 cards); – of its wins had 2+ buildings played (– with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Strategy health at difficulty 4 (c2): the top agent there (engine, 6% wins) plays 3.31 buildings and 0.00 tier-2 cards per game (of 17.2 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); **it hardly plays tier 2** (under 0.5 per game).
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 6% vs greedy 0% over 16 seeds (only engine 1, only greedy 0), +6.2 ±6.2 pts, sign test p 1.000: no better than greedy (ahead, not significant).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -25.0 ±17.1.
- Variants against the base (c2) at difficulty 1, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +6.2 ±11.1.
- Variants against the base (c2) at difficulty 2, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -12.5 ±8.5.
- Variants against the base (c2) at difficulty 3, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +0.0 ±0.0.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -6.2 ±6.2.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.

| c2 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (8.50) | 31.2 ±11.6 (12.25) | 50.0 ±12.5 (12.50) | +50.0 pts (+4.00 survival) |

| c2 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.94) | 6.2 ±6.1 (8.69) | 6.2 ±6.1 (11.06) | +6.2 pts (+4.12 survival) |

| c2 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.62) | 0.0 ±0.0 (7.50) | 12.5 ±8.3 (8.75) | +12.5 pts (+3.12 survival) |

| c2 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.75) | 0.0 ±0.0 (6.38) | 0.0 ±0.0 (6.56) | +0.0 pts (+1.81 survival) |

| c2 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.88) | 0.0 ±0.0 (4.38) | 6.2 ±6.1 (5.06) | +6.2 pts (+1.19 survival) |

| c3 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.81) | 0.0 ±0.0 (10.25) | 25.0 ±10.8 (12.25) | +25.0 pts (+4.44 survival) |

| c3 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (7.00) | 0.0 ±0.0 (8.12) | 12.5 ±8.3 (11.50) | +12.5 pts (+4.50 survival) |

| c3 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (6.06) | 0.0 ±0.0 (7.50) | 0.0 ±0.0 (8.81) | +0.0 pts (+2.75 survival) |

| c3 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.00) | 0.0 ±0.0 (6.00) | 0.0 ±0.0 (6.44) | +0.0 pts (+1.44 survival) |

| c3 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (3.94) | 0.0 ±0.0 (4.50) | 0.0 ±0.0 (4.62) | +0.0 pts (+0.69 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c2 d0 engine | 16 | 50.0 | 31.2 | 3 / 5 / 2 / 6 | +18.8 ±16.4 | 0.453 | no better than greedy (ahead, not significant) |
| c2 d1 random | 16 | 0.0 | 6.2 | 0 / 0 / 1 / 15 | -6.2 ±6.2 | 1.000 | no better than greedy (behind, not significant) |
| c2 d1 engine | 16 | 6.2 | 6.2 | 0 / 1 / 1 / 14 | +0.0 ±9.1 | 1.000 | no better than greedy (level) |
| c2 d2 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c2 d2 engine | 16 | 12.5 | 0.0 | 0 / 2 / 0 / 14 | +12.5 ±8.5 | 0.500 | no better than greedy (ahead, not significant) |
| c2 d3 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c2 d3 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c2 d4 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c2 d4 engine | 16 | 6.2 | 0.0 | 0 / 1 / 0 / 15 | +6.2 ±6.2 | 1.000 | no better than greedy (ahead, not significant) |
| c3 d0 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d0 engine | 16 | 25.0 | 0.0 | 0 / 4 / 0 / 12 | +25.0 ±11.2 | 0.125 | no better than greedy (ahead, not significant) |
| c3 d1 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d1 engine | 16 | 12.5 | 0.0 | 0 / 2 / 0 / 14 | +12.5 ±8.5 | 0.500 | no better than greedy (ahead, not significant) |
| c3 d2 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d2 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d3 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d3 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d4 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d4 engine | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 8.50 → 7.81 |
| c3 | 0 | greedy | 31.2 → 0.0 | -31.2 ±12.0 | 0.062 | 10.38 → 10.25 |
| c3 | 0 | engine | 50.0 → 25.0 | -25.0 ±17.1 | 0.289 | 9.31 → 10.75 |
| c3 | 1 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.94 → 7.00 |
| c3 | 1 | greedy | 6.2 → 0.0 | -6.2 ±6.2 | 1.000 | 8.19 → 8.12 |
| c3 | 1 | engine | 6.2 → 12.5 | +6.2 ±11.1 | 1.000 | 10.50 → 10.81 |
| c3 | 2 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.62 → 6.06 |
| c3 | 2 | greedy | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 7.50 → 7.50 |
| c3 | 2 | engine | 12.5 → 0.0 | -12.5 ±8.5 | 0.500 | 7.88 → 8.81 |
| c3 | 3 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.75 → 5.00 |
| c3 | 3 | greedy | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.38 → 6.00 |
| c3 | 3 | engine | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 6.56 → 6.44 |
| c3 | 4 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 3.88 → 3.94 |
| c3 | 4 | greedy | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.38 → 4.50 |
| c3 | 4 | engine | 6.2 → 0.0 | -6.2 ±6.2 | 1.000 | 4.88 → 4.62 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 random | 8.50 (1.27, 7..12) | cities 3, deck_empty 1, supply 12 | – / – | 43.2 / 0.25 | 3.75 | 4.06 / 0.69 |
| c2 d0 greedy | 10.38 (2.76, 4..12) | deck_empty 10, purge 5, supply 1 | 4.00 / 20.40 | 6.6 / 1.91 | 16.88 | 1.25 / 1.00 |
| c2 d0 engine | 9.31 (3.40, 2..12) | deck_empty 8, purge 8 | 4.38 / 26.12 | 2.8 / 2.38 | 25.00 | 0.50 / 2.94 |
| c2 d1 random | 6.94 (0.97, 5..9) | cities 4, supply 12 | – / – | 45.3 / 0.00 | 3.56 | 4.62 / 0.00 |
| c2 d1 greedy | 8.19 (1.78, 5..12) | cities 1, deck_empty 1, purge 1, supply 13 | 6.00 / 25.00 | 22.3 / 1.13 | 3.75 | 2.69 / 0.38 |
| c2 d1 engine | 10.50 (2.21, 4..12) | deck_empty 8, purge 1, supply 7 | 7.00 / 28.00 | 11.4 / 2.20 | 9.31 | 2.62 / 0.75 |
| c2 d2 random | 5.62 (0.70, 4..6) | supply 16 | – / – | 43.4 / 0.00 | 0.12 | 3.38 / 0.19 |
| c2 d2 greedy | 7.50 (1.37, 6..11) | cities 1, deck_empty 1, supply 14 | – / – | 25.4 / 1.06 | 1.38 | 3.00 / 0.06 |
| c2 d2 engine | 7.88 (1.83, 5..11) | deck_empty 2, purge 2, supply 12 | 5.00 / 21.00 | 20.9 / 1.50 | 4.56 | 2.62 / 0.56 |
| c2 d3 random | 4.75 (0.66, 3..6) | supply 16 | – / – | 41.1 / 0.00 | 1.00 | 3.19 / 0.25 |
| c2 d3 greedy | 6.38 (0.86, 5..8) | supply 16 | – / – | 31.1 / 0.81 | 0.88 | 2.81 / 0.00 |
| c2 d3 engine | 6.56 (1.00, 5..9) | supply 16 | – / – | 24.4 / 1.12 | 0.88 | 3.06 / 0.38 |
| c2 d4 random | 3.88 (0.86, 2..5) | supply 16 | – / – | 32.5 / 0.00 | 0.31 | 2.88 / 0.31 |
| c2 d4 greedy | 4.38 (0.78, 3..6) | supply 16 | – / – | 23.1 / 0.62 | 0.00 | 2.31 / 0.00 |
| c2 d4 engine | 4.88 (1.32, 4..9) | purge 1, supply 15 | 1.00 / 5.00 | 26.2 / 0.53 | 0.56 | 2.44 / 0.19 |
| c3 d0 random | 7.81 (1.38, 6..12) | deck_empty 1, supply 15 | – / – | 68.4 / 0.00 | 1.31 | 4.75 / 0.75 |
| c3 d0 greedy | 10.25 (1.71, 8..12) | deck_empty 7, supply 9 | – / – | 41.1 / 0.75 | 6.56 | 3.19 / 0.12 |
| c3 d0 engine | 10.75 (2.51, 4..12) | deck_empty 12, purge 4 | 4.00 / 35.00 | 11.2 / 1.42 | 35.06 | 0.62 / 3.25 |
| c3 d1 random | 7.00 (1.12, 5..9) | cities 1, supply 15 | – / – | 66.6 / 0.12 | 1.56 | 5.25 / 0.62 |
| c3 d1 greedy | 8.12 (1.62, 6..12) | deck_empty 2, supply 14 | – / – | 40.3 / 0.88 | 2.88 | 3.50 / 0.19 |
| c3 d1 engine | 10.81 (2.13, 5..12) | deck_empty 11, purge 2, supply 3 | 3.50 / 28.00 | 15.9 / 2.07 | 19.38 | 3.38 / 1.00 |
| c3 d2 random | 6.06 (1.03, 4..8) | supply 16 | – / – | 65.0 / 0.12 | 0.62 | 5.00 / 0.50 |
| c3 d2 greedy | 7.50 (2.06, 4..11) | deck_empty 3, supply 13 | – / – | 48.4 / 0.62 | 2.44 | 4.38 / 0.06 |
| c3 d2 engine | 8.81 (1.67, 7..11) | deck_empty 4, supply 12 | – / – | 42.4 / 0.94 | 4.69 | 4.25 / 0.50 |
| c3 d3 random | 5.00 (0.71, 4..6) | supply 16 | – / – | 62.2 / 0.00 | 0.38 | 4.06 / 0.12 |
| c3 d3 greedy | 6.00 (1.00, 4..8) | supply 16 | – / – | 60.3 / 0.12 | 1.38 | 4.12 / 0.00 |
| c3 d3 engine | 6.44 (1.62, 4..11) | deck_empty 1, supply 15 | – / – | 57.6 / 0.25 | 2.44 | 4.00 / 0.12 |
| c3 d4 random | 3.94 (0.83, 2..5) | supply 16 | – / – | 44.6 / 0.12 | 0.19 | 3.81 / 0.12 |
| c3 d4 greedy | 4.50 (0.79, 2..5) | supply 16 | – / – | 42.6 / 0.12 | 0.25 | 3.62 / 0.00 |
| c3 d4 engine | 4.62 (0.99, 3..7) | supply 16 | – / – | 41.9 / 0.25 | 0.00 | 3.62 / 0.19 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 random | 404.2 | 9.41 | 2% | – | – | 25.71 (93) | 9.19 | 21.56 | 6.12 | 6.12 / 24.50 (25%) |
| c2 d0 greedy | 498.1 | 21.52 | 2% | – | – | 16.19 (59) | 0.00 | 16.31 | 0.00 | 11.94 / 25.12 (48%) |
| c2 d0 engine | 518.9 | 24.43 | 2% | – | – | 9.15 (43) | 0.00 | 22.81 | 0.00 | 16.75 / 20.62 (81%) |
| c2 d1 random | 332.1 | 10.10 | 2% | – | – | 27.00 (83) | 8.56 | 23.56 | 7.38 | 6.81 / 32.69 (21%) |
| c2 d1 greedy | 414.8 | 19.57 | 2% | – | – | 23.95 (75) | 0.00 | 23.44 | 0.00 | 15.31 / 37.00 (41%) |
| c2 d1 engine | 608.6 | 22.47 | 1% | – | – | 21.56 (68) | 0.00 | 42.06 | 0.00 | 26.56 / 46.88 (57%) |
| c2 d2 random | 254.1 | 10.26 | 2% | – | – | 25.68 (76) | 5.06 | 16.44 | 4.88 | 5.38 / 25.06 (21%) |
| c2 d2 greedy | 377.9 | 18.54 | 2% | – | – | 26.27 (77) | 0.00 | 18.81 | 0.00 | 11.88 / 33.88 (35%) |
| c2 d2 engine | 445.0 | 19.08 | 1% | – | – | 24.28 (80) | 0.00 | 27.56 | 0.00 | 16.62 / 35.19 (47%) |
| c2 d3 random | 212.6 | 10.02 | 2% | – | – | 29.43 (77) | 4.12 | 14.38 | 3.81 | 4.81 / 21.38 (23%) |
| c2 d3 greedy | 327.7 | 17.03 | 1% | – | – | 29.78 (83) | 0.00 | 16.06 | 0.00 | 11.06 / 27.75 (40%) |
| c2 d3 engine | 351.7 | 16.51 | 1% | – | – | 26.87 (80) | 0.00 | 21.69 | 0.00 | 13.25 / 27.31 (49%) |
| c2 d4 random | 149.6 | 10.50 | 2% | – | – | 23.63 (59) | 2.88 | 8.62 | 1.88 | 2.88 / 14.44 (20%) |
| c2 d4 greedy | 201.8 | 15.43 | 1% | – | – | 22.57 (55) | 0.00 | 9.25 | 0.00 | 6.12 / 16.50 (37%) |
| c2 d4 engine | 247.2 | 15.35 | 1% | – | – | 22.68 (78) | 0.00 | 14.06 | 0.00 | 8.50 / 18.69 (45%) |
| c3 d0 random | 518.9 | 12.68 | 1% | – | – | 33.96 (103) | 21.94 | 22.06 | 5.00 | 7.12 / 27.44 (26%) |
| c3 d0 greedy | 709.6 | 26.68 | 1% | – | – | 30.43 (96) | 0.00 | 22.62 | 0.00 | 15.69 / 36.12 (43%) |
| c3 d0 engine | 897.2 | 34.44 | 1% | – | – | 18.90 (79) | 0.00 | 37.44 | 0.00 | 26.00 / 34.81 (75%) |
| c3 d1 random | 490.2 | 13.74 | 1% | – | – | 37.78 (114) | 20.38 | 29.88 | 7.50 | 10.12 / 38.44 (26%) |
| c3 d1 greedy | 577.6 | 25.38 | 1% | – | – | 35.07 (102) | 0.00 | 27.94 | 0.00 | 18.81 / 42.81 (44%) |
| c3 d1 engine | 925.8 | 32.17 | 1% | – | – | 31.31 (95) | 0.00 | 54.19 | 0.00 | 34.62 / 59.25 (58%) |
| c3 d2 random | 421.8 | 13.57 | 1% | – | – | 39.46 (105) | 19.44 | 24.25 | 7.56 | 7.12 / 32.25 (22%) |
| c3 d2 greedy | 550.0 | 24.60 | 1% | – | – | 39.56 (100) | 0.00 | 24.25 | 0.00 | 15.31 / 40.50 (38%) |
| c3 d2 engine | 719.2 | 27.22 | 1% | – | – | 36.55 (92) | 0.00 | 36.44 | 0.00 | 21.19 / 46.88 (45%) |
| c3 d3 random | 323.9 | 13.46 | 1% | – | – | 42.02 (108) | 11.94 | 19.12 | 4.38 | 6.44 / 25.38 (25%) |
| c3 d3 greedy | 415.2 | 21.21 | 1% | – | – | 42.91 (123) | 0.00 | 18.00 | 0.00 | 11.62 / 30.12 (39%) |
| c3 d3 engine | 487.2 | 22.44 | 1% | – | – | 42.35 (117) | 0.00 | 24.56 | 0.00 | 14.94 / 31.81 (47%) |
| c3 d4 random | 227.1 | 14.19 | 1% | – | – | 33.54 (80) | 8.50 | 12.75 | 2.44 | 4.19 / 17.81 (24%) |
| c3 d4 greedy | 280.6 | 19.37 | 1% | – | – | 33.82 (79) | 0.00 | 11.12 | 0.00 | 7.00 / 20.56 (34%) |
| c3 d4 engine | 304.9 | 19.63 | 1% | – | – | 32.16 (91) | 0.00 | 14.75 | 0.00 | 8.25 / 20.44 (40%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 30% / 22% / 49% | 6% / 82% / 12% |
| c2 d1 | 30% / 2% / 67% | 18% / 55% / 27% |
| c2 d2 | 30% / 9% / 61% | 26% / 45% / 29% |
| c2 d3 | – (no variance) | 26% / 48% / 26% |
| c2 d4 | 32% / 4% / 64% | 39% / 12% / 49% |
| c3 d0 | 27% / 18% / 55% | 17% / 66% / 17% |
| c3 d1 | 30% / 9% / 61% | 16% / 63% / 21% |
| c3 d2 | – (no variance) | 33% / 32% / 35% |
| c3 d3 | – (no variance) | 30% / 21% / 49% |
| c3 d4 | – (no variance) | 61% / 10% / 29% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 random | 10 of 16 | 0.91 | 0 of 0 | – |
| c2 d0 greedy | 8 of 16 | 0.39 | 3 of 5 | 0.31 |
| c2 d0 engine | 16 of 16 | 0.88 | 8 of 8 | 0.79 |
| c2 d1 random | 12 of 16 | 0.87 | 0 of 0 | – |
| c2 d1 greedy | 10 of 16 | 0.43 | 1 of 1 | – |
| c2 d1 engine | 16 of 16 | 0.90 | 1 of 1 | – |
| c2 d2 random | 12 of 16 | 0.88 | 0 of 0 | – |
| c2 d2 greedy | 9 of 16 | 0.40 | 0 of 0 | – |
| c2 d2 engine | 16 of 16 | 0.88 | 2 of 2 | 0.75 |
| c2 d3 random | 12 of 16 | 0.86 | 0 of 0 | – |
| c2 d3 greedy | 10 of 16 | 0.41 | 0 of 0 | – |
| c2 d3 engine | 15 of 16 | 0.87 | 0 of 0 | – |
| c2 d4 random | 12 of 16 | 0.76 | 0 of 0 | – |
| c2 d4 greedy | 8 of 16 | 0.39 | 0 of 0 | – |
| c2 d4 engine | 16 of 16 | 0.78 | 1 of 1 | – |
| c3 d0 random | 16 of 16 | 0.93 | 0 of 0 | – |
| c3 d0 greedy | 10 of 16 | 0.26 | 0 of 0 | – |
| c3 d0 engine | 16 of 16 | 0.78 | 4 of 4 | 0.71 |
| c3 d1 random | 15 of 16 | 0.96 | 0 of 0 | – |
| c3 d1 greedy | 10 of 16 | 0.30 | 0 of 0 | – |
| c3 d1 engine | 16 of 16 | 0.79 | 2 of 2 | 1.00 |
| c3 d2 random | 15 of 16 | 0.94 | 0 of 0 | – |
| c3 d2 greedy | 11 of 16 | 0.35 | 0 of 0 | – |
| c3 d2 engine | 16 of 16 | 0.81 | 0 of 0 | – |
| c3 d3 random | 13 of 16 | 0.89 | 0 of 0 | – |
| c3 d3 greedy | 11 of 16 | 0.38 | 0 of 0 | – |
| c3 d3 engine | 16 of 16 | 0.74 | 0 of 0 | – |
| c3 d4 random | 14 of 16 | 0.80 | 0 of 0 | – |
| c3 d4 greedy | 11 of 16 | 0.42 | 0 of 0 | – |
| c3 d4 engine | 16 of 16 | 0.77 | 0 of 0 | – |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.62 / 0.88 / 11.1, – | 3.50 / 0.50 / 12.9, – |
| greedy | 2.50 / 0.00 / 18.4, 80% (100%) | 2.75 / 0.00 / 25.4, – |
| engine | 6.00 / 0.00 / 32.1, 100% (100%) | 9.06 / 0.06 / 57.8, 100% (100%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.69 / 0.25 / 8.0, – | 3.25 / 0.75 / 12.8, – |
| greedy | 2.38 / 0.00 / 17.4, 100% (100%) | 2.75 / 0.00 / 23.4, – |
| engine | 5.75 / 0.00 / 35.4, 100% (100%) | 7.31 / 0.00 / 52.4, 100% (100%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.38 / 0.25 / 7.3, – | 2.88 / 0.56 / 11.1, – |
| greedy | 2.19 / 0.00 / 16.0, – | 2.62 / 0.00 / 22.5, – |
| engine | 4.69 / 0.00 / 27.2, 100% (100%) | 6.25 / 0.00 / 39.2, – |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.31 / 0.06 / 5.9, – | 2.38 / 0.12 / 8.9, – |
| greedy | 2.06 / 0.00 / 15.8, – | 2.44 / 0.00 / 19.5, – |
| engine | 3.81 / 0.00 / 22.4, – | 4.81 / 0.00 / 28.7, – |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.00 / 0.00 / 4.9, – | 2.12 / 0.12 / 7.8, – |
| greedy | 1.88 / 0.00 / 11.9, – | 2.12 / 0.00 / 15.6, – |
| engine | 3.31 / 0.00 / 17.2, 100% (100%) | 3.81 / 0.00 / 20.7, – |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|---|
| adaptive_malware | 2.2 (45) / 53% / 25% | 2.0 (98) / 97% / 93% | 2.0 (105) / 110% / 83% |
| adversarial_inference_sweep | 2.5 (60) / 47% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| aegis_dome_array | 2.5 (70) / 33% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| aegis_grid_lockdown | 2.2 (77) / 44% / 6% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| agency_directorate | 2.4 (78) / 32% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| agency_field_station | 2.4 (68) / 41% / 11% | 1.6 (112) / 102% / 0% | 1.6 (119) / 100% / 3% |
| amnesty_decree | 2.5 (45) / 84% / 21% | 1.7 (103) / 111% / 102% | 1.6 (125) / 106% / 104% |
| antiproton_offtake_deal | 2.5 (71) / 56% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| arc_furnace_complex | 2.4 (63) / 41% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| arcology_foundry | 2.2 (91) / 47% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| asteroid_salvage_rights | 2.7 (72) / 50% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| back_channel_talks | 2.0 (23) / 187% / 28% | 2.7 (66) / 142% / 102% | 2.8 (70) / 151% / 108% |
| baseload_contract | 2.4 (43) / 44% / 11% | 1.7 (88) / 99% / 0% | 1.6 (90) / 109% / 40% |
| beamed_power_constellation | 2.8 (86) / 35% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| benchmark_suite | 2.6 (43) / 86% / 0% | 3.3 (91) / 107% / 0% | 3.2 (100) / 121% / 0% |
| black_ice_citadel | 2.6 (83) / 41% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| blackout_script | 2.5 (33) / 79% / 27% | 3.2 (78) / 112% / 0% | 3.2 (84) / 117% / 34% |
| blackstart_protocol | 3.2 (38) / 121% / 41% | 3.8 (78) / 135% / 0% | 3.8 (88) / 132% / 100% |
| bonded_warehouse | 2.6 (47) / 111% / 21% | 1.7 (84) / 120% / 0% | 1.5 (97) / 125% / 4% |
| bounty_board | 2.7 (30) / 27% / 38% | 4.0 (91) / 77% / 0% | 4.0 (120) / 67% / 114% |
| breach_and_clear | 2.6 (27) / 115% / 13% | 1.5 (78) / 91% / 51% | 1.5 (95) / 120% / 61% |
| breakthrough_leak | 2.4 (21) / 95% / 20% | 2.0 (81) / 106% / 100% | 2.0 (114) / 108% / 93% |
| bridge_loan | 2.4 (37) / 122% / 49% | 3.3 (78) / 144% / 0% | 3.2 (126) / 129% / 90% |
| brownout_relief | 2.1 (16) / 281% / 36% | 3.2 (70) / 139% / 85% | 3.2 (69) / 138% / 86% |
| bulk_silicon_procurement | 2.5 (53) / 79% / 29% | 3.5 (91) / 99% / 110% | 3.3 (120) / 102% / 108% |
| capacitor_discharge | 2.5 (29) / 72% / 48% | 3.9 (83) / 86% / 0% | 3.9 (136) / 74% / 111% |
| cascade_failure | 2.5 (89) / 47% / 10% | – (0) / – / 0.00/game | 3.0 (1) / 0% / – |
| ceasefire_monitors | 2.7 (34) / 91% / 16% | 2.2 (93) / 105% / 100% | 2.1 (101) / 110% / 105% |
| citation_engine | 2.6 (48) / 115% / 18% | 2.1 (81) / 138% / 0% | 1.9 (89) / 142% / 1% |
| coalition_charter | 2.6 (58) / 47% / 22% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| collateralized_grid | 2.5 (39) / 113% / 32% | 2.9 (87) / 122% / 0% | 3.0 (90) / 133% / 3% |
| commons_grid | 2.3 (28) / 136% / 34% | 1.8 (85) / 112% / 0% | 1.8 (90) / 121% / 12% |
| compute_siphon | 2.5 (41) / 98% / 22% | 1.8 (82) / 113% / 0% | 1.7 (85) / 126% / 1% |
| consular_pouch | 2.5 (30) / 130% / 56% | 3.4 (77) / 110% / 107% | 3.4 (85) / 125% / 107% |
| consulate_tower | 2.6 (50) / 96% / 33% | 1.7 (87) / 126% / 104% | 1.7 (88) / 135% / 98% |
| continental_supergrid | 2.3 (69) / 36% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| cryo_compute_burn | 2.2 (26) / 100% / 23% | 3.6 (94) / 83% / 0% | 3.6 (97) / 98% / 0% |
| darknet_exchange | 2.5 (35) / 94% / 15% | 1.7 (89) / 103% / 0% | 1.7 (91) / 113% / 12% |
| data_licensing_deal | 2.3 (74) / 58% / 21% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| dataset_scraper | 2.8 (16) / 181% / 0% | 3.7 (66) / 103% / 0% | 3.8 (80) / 96% / 3% |
| deep_state_ledger | 2.5 (82) / 41% / 15% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| demand_response_grid | 2.2 (12) / 158% / 21% | 2.4 (78) / 106% / 0% | 2.2 (80) / 123% / 0% |
| development_loan | 2.8 (36) / 64% / 26% | 3.3 (90) / 122% / 100% | 3.4 (91) / 119% / 99% |
| diplomatic_expulsion | 2.3 (26) / 250% / 43% | 3.5 (53) / 196% / 115% | 3.5 (73) / 164% / 111% |
| dyson_swarm_tranche | 2.7 (90) / 46% / 0% | – (0) / – / 0.00/game | 1.0 (1) / 100% / 0% |
| embassy_compound | 2.4 (22) / 209% / 35% | 1.7 (66) / 158% / 101% | 1.7 (81) / 158% / 98% |
| endowed_research_chair | 2.6 (36) / 122% / 30% | 1.0 (89) / 117% / 0% | 1.0 (98) / 111% / 0% |
| eviction_worm | 2.4 (45) / 71% / 19% | 2.3 (84) / 115% / 102% | 2.4 (98) / 123% / 93% |
| exabyte_archive | 2.6 (87) / 62% / 0% | – (0) / – / 0.00/game | 1.0 (1) / 0% / – |
| exodus_corridors | 2.4 (102) / 61% / 13% | – (0) / – / 0.00/game | 4.0 (1) / 100% / 100% |
| expansion_fund | 2.4 (37) / 114% / 24% | 2.1 (67) / 128% / 0% | 2.0 (83) / 139% / 0% |
| faraday_yard | 2.5 (24) / 208% / 36% | 1.3 (66) / 135% / 0% | 1.6 (69) / 148% / 24% |
| field_sensor_grid | 2.4 (24) / 50% / 67% | 2.9 (80) / 55% / 0% | 3.0 (99) / 78% / 0% |
| flash_exploit | 2.7 (40) / 60% / 54% | 3.2 (86) / 103% / 0% | 3.2 (89) / 116% / 28% |
| foundation_model_campus | 2.3 (90) / 43% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| full_automation_mandate | 2.4 (90) / 40% / 0% | – (0) / – / 0.00/game | 3.0 (1) / 0% / – |
| fusion_startup_stake | 2.9 (35) / 131% / 17% | 1.1 (94) / 89% / 0% | 1.1 (85) / 114% / 61% |
| grad_student_pool | 2.1 (38) / 68% / 19% | 2.7 (89) / 100% / 0% | 2.7 (101) / 100% / 0% |
| great_liberation | 2.7 (72) / 38% / 4% | – (0) / – / 0.00/game | 1.0 (1) / 0% / – |
| grid_census | 2.6 (20) / 180% / 44% | 2.7 (56) / 152% / 0% | 2.5 (75) / 124% / 86% |
| grid_tied_smelter | 2.2 (32) / 103% / 33% | 1.4 (81) / 102% / 0% | 1.4 (92) / 112% / 55% |
| hardened_bunker_complex | 3.1 (30) / 110% / 42% | 1.7 (78) / 127% / 0% | 1.9 (81) / 136% / 24% |
| hardened_conduits | 2.4 (33) / 133% / 20% | 3.7 (77) / 95% / 0% | 3.6 (91) / 112% / 14% |
| hardpoint_lockdown | 2.7 (43) / 74% / 34% | 3.0 (107) / 68% / 0% | 3.0 (109) / 95% / 0% |
| hardware_spot_market | 2.6 (39) / 136% / 42% | 3.9 (67) / 125% / 0% | 3.9 (98) / 98% / 14% |
| honeypot_relay | 2.4 (28) / 146% / 24% | 2.3 (83) / 105% / 0% | 2.2 (81) / 126% / 0% |
| hyperscale_botnet | 2.3 (69) / 39% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| interceptor_battery | 2.5 (17) / 253% / 35% | 1.4 (59) / 136% / 0% | 1.3 (73) / 134% / 0% |
| interconnector_treaty | 2.4 (94) / 55% / 2% | – (0) / – / 0.00/game | 2.0 (1) / 0% / – |
| intrusion_countermeasures | 2.5 (28) / 100% / 39% | 2.4 (82) / 98% / 35% | 2.4 (96) / 105% / 100% |
| intrusion_lab | 2.4 (43) / 121% / 35% | 1.2 (96) / 110% / 0% | 1.2 (93) / 117% / 0% |
| joint_fabrication_pact | 2.5 (36) / 125% / 49% | 3.9 (74) / 135% / 107% | 3.8 (97) / 128% / 107% |
| joint_security_compact | 1.9 (28) / 136% / 47% | 2.0 (66) / 121% / 0% | 2.2 (71) / 142% / 31% |
| kill_team_roster | 2.8 (36) / 83% / 13% | 1.4 (83) / 36% / 0% | 1.5 (90) / 73% / 0% |
| killswitch_directive | 2.2 (105) / 47% / 8% | – (0) / – / 0.00/game | 2.0 (1) / 0% / – |
| kinetic_interdiction | 2.4 (77) / 32% / 12% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| liaison_corps | 2.5 (23) / 70% / 19% | 2.3 (79) / 72% / 0% | 2.4 (75) / 105% / 0% |
| liberation_accords | 2.6 (93) / 55% / 2% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| lights_out_assembly | 2.2 (28) / 107% / 30% | 2.6 (78) / 91% / 0% | 2.5 (83) / 124% / 0% |
| liquidity_event | 2.1 (41) / 115% / 0% | 3.3 (73) / 110% / 0% | 3.4 (83) / 122% / 0% |
| literature_crawler | 2.5 (34) / 124% / 50% | 4.0 (71) / 55% / 123% | 3.9 (80) / 75% / 117% |
| logic_bomb | 2.1 (40) / 88% / 11% | 1.3 (87) / 75% / 65% | 1.3 (102) / 97% / 88% |
| manifesto_leak | 2.3 (32) / 75% / 0% | 3.9 (79) / 92% / 0% | 3.9 (99) / 98% / 0% |
| martial_cordon | 2.6 (89) / 53% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| materials_hedge_desk | 2.2 (34) / 129% / 32% | 2.6 (73) / 111% / 0% | 2.6 (87) / 126% / 0% |
| megacorp_ipo | 2.6 (65) / 45% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| mesh_intrusion_pact | 2.6 (78) / 49% / 11% | – (0) / – / 0.00/game | 4.0 (1) / 0% / – |
| modular_expansion | – (0) / – / 0.49/game | – (0) / – / 0.00/game | – (0) / – / 0.29/game |
| mutual_aid_pact | 2.9 (19) / 211% / 42% | 3.2 (60) / 140% / 108% | 3.1 (71) / 152% / 101% |
| mutual_defense_pact | 2.6 (32) / 131% / 31% | 1.9 (81) / 86% / 0% | 2.0 (91) / 112% / 30% |
| neural_cartographer | 2.4 (20) / 140% / 0% | 3.0 (52) / 154% / 65% | 2.8 (62) / 145% / 36% |
| neural_mirror | 2.5 (31) / 126% / 23% | 1.1 (74) / 118% / 0% | 1.1 (78) / 127% / 0% |
| node_severance | 2.3 (82) / 49% / 8% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| open_assembly | 2.6 (25) / 108% / 22% | 2.8 (85) / 99% / 0% | 2.7 (94) / 113% / 8% |
| open_preprint | 2.8 (29) / 86% / 40% | 3.7 (99) / 70% / 106% | 3.5 (123) / 80% / 109% |
| open_weights_release | 2.5 (76) / 30% / 4% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| orbital_embassy | 2.5 (72) / 49% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| orbital_mass_driver | 2.9 (109) / 34% / 0% | – (0) / – / 0.00/game | 1.0 (1) / 0% / – |
| orbital_observatory_array | 2.6 (73) / 55% / 2% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| payroll_automation | 2.4 (33) / 155% / 33% | 1.7 (63) / 113% / 0% | 1.9 (78) / 132% / 10% |
| peace_dividend | 2.4 (28) / 39% / 18% | 3.3 (92) / 64% / 93% | 3.2 (98) / 88% / 101% |
| peacekeeper_rotation | 2.2 (26) / 88% / 9% | 2.7 (88) / 68% / 90% | 2.9 (97) / 77% / 108% |
| peacekeeping_detail | 2.0 (24) / 112% / 30% | 3.5 (78) / 87% / 0% | 3.5 (100) / 90% / 30% |
| peaker_turbines | 2.6 (45) / 49% / 9% | 1.1 (97) / 93% / 0% | 1.1 (123) / 92% / 62% |
| planetary_assembly | 2.5 (79) / 39% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| planetary_referendum | 2.4 (102) / 43% / 5% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| point_defense | – (0) / – / 0.36/game | – (0) / – / 0.00/game | – (0) / – / 0.35/game |
| positron_storage_ring | 2.4 (90) / 38% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| predictive_modelling | 2.6 (32) / 94% / 43% | 2.8 (74) / 103% / 0% | 2.7 (91) / 98% / 1% |
| prefab_plant_kit | 2.1 (22) / 109% / 4% | 2.7 (77) / 82% / 0% | 2.7 (90) / 106% / 57% |
| pylon_drop | 2.8 (37) / 86% / 38% | 3.8 (95) / 84% / 92% | 3.8 (101) / 96% / 98% |
| quantum_decryptor | 2.6 (34) / 50% / 18% | 1.2 (107) / 44% / 0% | 1.1 (107) / 67% / 1% |
| recruit_operatives | 2.5 (29) / 103% / 0% | 3.8 (80) / 56% / 0% | 3.7 (101) / 73% / 0% |
| recursive_self_improvement | 2.7 (83) / 35% / 0% | – (0) / – / 0.00/game | 2.0 (1) / 0% / – |
| red_team_audit | 2.8 (28) / 121% / 26% | 2.2 (72) / 121% / 103% | 2.2 (82) / 133% / 105% |
| relay_mast_rollout | 2.7 (29) / 66% / 26% | 2.9 (79) / 73% / 93% | 2.8 (86) / 93% / 78% |
| retained_earnings | 2.6 (38) / 87% / 52% | 3.6 (90) / 82% / 0% | 3.5 (110) / 89% / 4% |
| robotic_foundry | 2.5 (27) / 163% / 0% | 1.0 (77) / 5% / 0% | 1.0 (93) / 51% / 0% |
| rootkit_cache | 2.3 (20) / 130% / 54% | 3.5 (85) / 42% / 0% | 3.3 (87) / 75% / 0% |
| safehouse_network | 2.8 (25) / 128% / 28% | 2.4 (76) / 68% / 0% | 2.4 (84) / 87% / 0% |
| security_contractor_hq | 2.6 (32) / 106% / 41% | 1.8 (75) / 103% / 0% | 1.9 (76) / 116% / 17% |
| sentinel_mesh | 2.6 (23) / 313% / 28% | 1.6 (56) / 188% / 0% | 1.5 (56) / 204% / 0% |
| sentry_drone_wing | 2.6 (21) / 81% / 24% | 2.1 (99) / 46% / 0% | 2.1 (89) / 78% / 0% |
| shadow_broker_feed | 2.3 (14) / 64% / 22% | 3.1 (80) / 38% / 87% | 3.1 (81) / 63% / 53% |
| shared_compute_grant | 2.1 (21) / 229% / 17% | 2.9 (71) / 96% / 101% | 2.8 (71) / 132% / 57% |
| shared_exploit_kit | 2.4 (38) / 84% / 38% | 3.1 (99) / 56% / 0% | 3.1 (106) / 79% / 62% |
| shared_grid_accord | 3.1 (49) / 92% / 27% | 3.6 (90) / 44% / 0% | 3.7 (91) / 77% / 29% |
| shared_threat_intel | 2.5 (39) / 59% / 26% | 3.7 (96) / 75% / 88% | 3.5 (102) / 85% / 106% |
| shelter_order | – (0) / – / 0.43/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shuttle_diplomacy | 2.6 (18) / 161% / 45% | 3.5 (57) / 107% / 97% | 3.4 (66) / 117% / 100% |
| signal_hijack | 2.1 (14) / 193% / 19% | 2.3 (69) / 57% / 79% | 2.0 (107) / 57% / 85% |
| simulation_cluster | 2.4 (38) / 55% / 10% | 1.3 (99) / 44% / 0% | 1.3 (105) / 67% / 0% |
| singularity_exploit | 2.5 (50) / 50% / 4% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| sovereign_capital_fund | 2.7 (58) / 48% / 0% | – (0) / – / 0.00/game | 3.0 (1) / 0% / – |
| spoofed_credentials | 2.5 (42) / 50% / 57% | 3.7 (92) / 62% / 104% | 3.8 (111) / 57% / 106% |
| station_prefab_order | 2.6 (34) / 124% / 33% | 2.4 (83) / 84% / 0% | 2.3 (107) / 83% / 111% |
| storm_hardening | 2.5 (25) / 48% / 25% | 2.3 (96) / 36% / 0% | 2.2 (106) / 53% / 0% |
| strike_team | – (0) / – / 0.33/game | – (0) / – / 0.96/game | – (0) / – / 0.96/game |
| substation_hub | 2.7 (21) / 110% / 26% | 1.4 (78) / 79% / 0% | 1.3 (93) / 92% / 76% |
| summit_caucus | 2.2 (36) / 69% / 0% | 2.3 (83) / 42% / 0% | 2.2 (85) / 67% / 19% |
| supply_chain_treaty | 2.2 (17) / 194% / 36% | 2.0 (73) / 48% / 0% | 1.9 (93) / 65% / 60% |
| surplus_auction | 3.0 (8) / 288% / 43% | 4.0 (64) / 69% / 5% | 3.9 (107) / 45% / 62% |
| synthetic_data_forge | 2.8 (46) / 70% / 28% | 1.4 (115) / 55% / 0% | 1.4 (120) / 65% / 0% |
| telemetry_archive | 2.5 (49) / 53% / 46% | 2.2 (97) / 88% / 0% | 2.1 (100) / 95% / 21% |
| threat_forecast_engine | 2.5 (22) / 123% / 41% | 2.6 (92) / 36% / 39% | 2.4 (98) / 61% / 15% |
| threat_neutralization_mandate | 2.4 (74) / 50% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| threat_triage | 2.4 (25) / 184% / 30% | 1.1 (89) / 31% / 0% | 1.2 (99) / 51% / 100% |
| tidal_barrage | 2.5 (41) / 151% / 11% | 1.2 (74) / 99% / 0% | 1.3 (85) / 112% / 74% |
| total_containment | 2.6 (58) / 38% / 0% | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| transmission_corridor | 2.7 (34) / 68% / 35% | 2.8 (88) / 82% / 74% | 2.7 (98) / 79% / 81% |
| treaty_grid_exchange | 2.1 (24) / 154% / 30% | 1.0 (66) / 117% / 0% | 1.1 (77) / 131% / 59% |
| turnkey_factory | – (0) / – / 0.34/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| vendor_financing | 2.5 (44) / 107% / 0% | 3.7 (97) / 56% / 0% | 3.7 (116) / 63% / 0% |
| venture_capital | – (0) / – / 0.41/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| zero_click_breach | 2.6 (36) / 94% / 15% | 1.0 (86) / 76% / 0% | 1.0 (93) / 87% / 1% |
| zero_knowledge_exploit | 2.7 (41) / 120% / 18% | 2.8 (101) / 33% / 55% | 2.7 (106) / 57% / 32% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 random | 0.0 | 0.000 | 0.0 | 104.6 |
| c2 d0 greedy | 5899.5 | 11.845 | 0.0 | 6024.0 |
| c2 d0 engine | 10937.2 | 21.076 | 0.0 | 11102.3 |
| c2 d1 random | 0.0 | 0.000 | 0.0 | 79.6 |
| c2 d1 greedy | 5619.6 | 13.547 | 0.0 | 5723.1 |
| c2 d1 engine | 14232.0 | 23.384 | 0.0 | 14424.0 |
| c2 d2 random | 0.0 | 0.000 | 0.0 | 68.3 |
| c2 d2 greedy | 4991.4 | 13.209 | 0.0 | 5105.5 |
| c2 d2 engine | 8408.6 | 18.896 | 0.0 | 8533.3 |
| c2 d3 random | 0.0 | 0.000 | 0.0 | 56.7 |
| c2 d3 greedy | 3736.0 | 11.401 | 0.0 | 3830.4 |
| c2 d3 engine | 5648.7 | 16.062 | 0.0 | 5749.6 |
| c2 d4 random | 0.0 | 0.000 | 0.0 | 26.9 |
| c2 d4 greedy | 1918.9 | 9.509 | 0.0 | 1967.3 |
| c2 d4 engine | 2853.3 | 11.540 | 0.0 | 2909.4 |
| c3 d0 random | 0.1 | 0.000 | 0.0 | 181.0 |
| c3 d0 greedy | 13479.8 | 18.996 | 0.0 | 13741.3 |
| c3 d0 engine | 38165.0 | 42.538 | 0.0 | 38514.8 |
| c3 d1 random | 0.0 | 0.000 | 0.0 | 159.4 |
| c3 d1 greedy | 12100.3 | 20.948 | 0.0 | 12303.5 |
| c3 d1 engine | 35609.0 | 38.465 | 0.0 | 35967.4 |
| c3 d2 random | 0.0 | 0.000 | 0.0 | 135.5 |
| c3 d2 greedy | 11967.1 | 21.758 | 0.0 | 12141.3 |
| c3 d2 engine | 23513.2 | 32.694 | 0.0 | 23783.5 |
| c3 d3 random | 0.0 | 0.000 | 0.0 | 97.8 |
| c3 d3 greedy | 7873.9 | 18.965 | 0.0 | 8008.6 |
| c3 d3 engine | 14577.7 | 29.922 | 0.0 | 14752.9 |
| c3 d4 random | 0.0 | 0.000 | 0.0 | 70.0 |
| c3 d4 greedy | 4229.6 | 15.072 | 0.0 | 4316.5 |
| c3 d4 engine | 6362.5 | 20.869 | 0.0 | 6467.5 |

Where the games' time went (summed over workers): agents 98.2%, probes 0.0%, apply 0.7%, legal 0.5%, replay check 0.6%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c13-baseline-c2-d4-greedy-seed4.json`](records/close-loss-c13-baseline-c2-d4-greedy-seed4.json): close loss at difficulty 4: supply in round 3, 4 adversary piece(s) over the level-2 bar, supply 0 of 20, 1 cities corrupted
- [`records/close-loss-c13-baseline-c2-d4-greedy-seed13.json`](records/close-loss-c13-baseline-c2-d4-greedy-seed13.json): close loss at difficulty 4: supply in round 4, 5 adversary piece(s) over the level-2 bar, supply 0 of 20, 2 cities corrupted

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c1-baseline-c2-d0-greedy-seed4.json`](records/surprising-win-c1-baseline-c2-d0-greedy-seed4.json): won by the weakest agent as measured (greedy)
- [`records/surprising-win-c1-baseline-c2-d0-greedy-seed6.json`](records/surprising-win-c1-baseline-c2-d0-greedy-seed6.json): won by the weakest agent as measured (greedy)

