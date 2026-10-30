# Cycle report: screen-027-cards

027 quick screen (r16): the endgame. 2 and 3 countries, d0 to d4, random, greedy and engine. Summary: python scripts/cards/screen_027.py DIR.

480 games (30 cells × 16 seeds, 0..=15) on 12 threads in 138.1 s. Engine 0.1.0, rules r16, content f6c822930a610633.

- Configs: configs/baseline.json
- Variants: c2; c3 (countries=3)
- Difficulties: 0, 1, 2, 3, 4
- Agents (weakest first): random, greedy, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (README, *Experiments*): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 0 (configs/baseline.json, c2): win rates random 0%, greedy 25%, engine 81%; skill gap +81 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.62, greedy 9.75, engine 12.81; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c2): win rates random 0%, greedy 25%, engine 38%; skill gap +38 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.38, greedy 9.06, engine 11.19; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c2): win rates random 0%, greedy 31%, engine 56%; skill gap +56 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c2): survival (rounds; wins count extra) random 5.19, greedy 8.44, engine 10.88; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c2): win rates random 0%, greedy 12%, engine 56%; skill gap +56 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c2): survival (rounds; wins count extra) random 4.88, greedy 7.25, engine 10.56; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c2): win rates random 0%, greedy 12%, engine 25%; skill gap +25 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c2): survival (rounds; wins count extra) random 4.31, greedy 6.00, engine 8.88; the ladder is ordered.
- Difficulty 0 (configs/baseline.json, c3): win rates random 0%, greedy 56%, engine 88%; skill gap +88 pts; stronger agents never do worse.
- Difficulty 0 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.44, greedy 11.06, engine 12.75; the ladder is ordered.
- Difficulty 1 (configs/baseline.json, c3): win rates random 0%, greedy 44%, engine 69%; skill gap +69 pts; stronger agents never do worse.
- Difficulty 1 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.38, greedy 9.81, engine 11.81; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, c3): win rates random 0%, greedy 19%, engine 50%; skill gap +50 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.06, greedy 8.38, engine 10.56; the ladder is ordered.
- Difficulty 3 (configs/baseline.json, c3): win rates random 0%, greedy 0%, engine 19%; skill gap +19 pts; stronger agents never do worse.
- Difficulty 3 (configs/baseline.json, c3): survival (rounds; wins count extra) random 5.12, greedy 6.88, engine 9.12; the ladder is ordered.
- Difficulty 4 (configs/baseline.json, c3): win rates random 0%, greedy 0%, engine 12%; skill gap +12 pts; stronger agents never do worse.
- Difficulty 4 (configs/baseline.json, c3): survival (rounds; wins count extra) random 4.81, greedy 5.69, engine 7.75; the ladder is ordered.
- Skill gap from difficulty 0 to 4 (mean over 2 configs/variants): +84 → +19 pts win rate, +7.25 → +3.75 survival (target: widens with difficulty).
- Game length for the strongest agent (engine; configs/baseline.json, c2): 8.9 rounds at difficulty 0, 7.6 at 4 (target 8–10).
- Game length for the strongest agent (engine; configs/baseline.json, c3): 9.2 rounds at difficulty 0, 7.2 at 4 (target 8–10).
- Games ended by: deck_empty 11, discord_deck 353, presence 1, purge 115.
- Adversary operations: 36.7 per round on average across cells, at most 219 in one round (target: low).
- Cards (engine, offered at least 10 times): kept under 10% of the time: aegis_grid_lockdown, benchmark_suite, black_ice_citadel, data_licensing_deal, dyson_swarm_tranche, exabyte_archive, foundation_model_campus, hyperscale_botnet, interconnector_treaty, manifesto_leak, megacorp_ipo, orbital_mass_driver, recruit_operatives, recursive_self_improvement, summit_caucus, vendor_financing; over 90%: adaptive_malware, agency_field_station, amnesty_decree, back_channel_talks, baseload_contract, blackout_script, blackstart_protocol, bonded_warehouse, bounty_board, breach_and_clear, breakthrough_leak, bridge_loan, bulk_silicon_procurement, capacitor_discharge, ceasefire_monitors, chip_spot_market, citation_engine, collateralized_grid, commons_grid, compute_siphon, consular_pouch, consulate_tower, cryo_compute_burn, darknet_exchange, development_loan, diplomatic_expulsion, embassy_compound, endowed_research_chair, eviction_worm, expansion_fund, faraday_yard, flash_exploit, fusion_startup_stake, grad_student_pool, grid_census, grid_tied_smelter, hardened_bunker_complex, hardened_conduits, hardpoint_lockdown, honeypot_relay, interceptor_battery, intrusion_countermeasures, intrusion_lab, joint_fabrication_pact, joint_security_compact, liaison_corps, lights_out_assembly, logic_bomb, materials_hedge_desk, mutual_defense_pact, neural_cartographer, neural_mirror, open_preprint, payroll_automation, peace_dividend, peacekeeper_rotation, predictive_modelling, red_team_audit, security_contractor_hq, sentinel_mesh, shared_exploit_kit, substation_hub, telemetry_archive, tidal_barrage, treaty_grid_exchange.
- Luck share of survival (seed's share of the variance): 11% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 0.00 gifts per game; 89% of threats countered.
- Strategy health at difficulty 0 (c2): the top agent there (engine, 81% wins) plays 4.62 buildings and 7.62 tier-2 cards per game (of 20.8 cards); 100% of its wins had 2+ buildings played (100% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 1 (c2): the top agent there (engine, 38% wins) plays 3.31 buildings and 6.94 tier-2 cards per game (of 19.4 cards); 83% of its wins had 2+ buildings played (67% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 2 (c2): the top agent there (engine, 56% wins) plays 3.38 buildings and 7.56 tier-2 cards per game (of 19.6 cards); 100% of its wins had 2+ buildings played (89% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 3 (c2): the top agent there (engine, 56% wins) plays 2.88 buildings and 6.50 tier-2 cards per game (of 18.2 cards); 89% of its wins had 2+ buildings played (67% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Strategy health at difficulty 4 (c2): the top agent there (engine, 25% wins) plays 2.38 buildings and 6.44 tier-2 cards per game (of 16.8 cards); 75% of its wins had 2+ buildings played (75% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Top agent (engine: highest win rate at difficulty 4, ties to the later agent) vs greedy at difficulty 4 (c2): engine 25% vs greedy 12% over 16 seeds (only engine 3, only greedy 1), +12.5 ±12.5 pts, sign test p 0.625: no better than greedy (ahead, not significant).
- Variants against the base (c2) at difficulty 0, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +6.2 ±14.3.
- Variants against the base (c2) at difficulty 1, engine, win rate difference in pts (bold: sign test p < 0.05): c3 +31.2 ±17.6.
- Variants against the base (c2) at difficulty 2, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -6.2 ±19.3.
- Variants against the base (c2) at difficulty 3, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -37.5 ±15.5.
- Variants against the base (c2) at difficulty 4, engine, win rate difference in pts (bold: sign test p < 0.05): c3 -12.5 ±12.5.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.

| c2 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.62) | 25.0 ±10.8 (9.75) | 81.2 ±9.8 (12.81) | +81.2 pts (+7.19 survival) |

| c2 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.38) | 25.0 ±10.8 (9.06) | 37.5 ±12.1 (11.19) | +37.5 pts (+5.81 survival) |

| c2 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.19) | 31.2 ±11.6 (8.44) | 56.2 ±12.4 (10.88) | +56.2 pts (+5.69 survival) |

| c2 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.88) | 12.5 ±8.3 (7.25) | 56.2 ±12.4 (10.56) | +56.2 pts (+5.69 survival) |

| c2 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.31) | 12.5 ±8.3 (6.00) | 25.0 ±10.8 (8.88) | +25.0 pts (+4.56 survival) |

| c3 difficulty 0 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.44) | 56.2 ±12.4 (11.06) | 87.5 ±8.3 (12.75) | +87.5 pts (+7.31 survival) |

| c3 difficulty 1 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.38) | 43.8 ±12.4 (9.81) | 68.8 ±11.6 (11.81) | +68.8 pts (+6.44 survival) |

| c3 difficulty 2 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.06) | 18.8 ±9.8 (8.38) | 50.0 ±12.5 (10.56) | +50.0 pts (+5.50 survival) |

| c3 difficulty 3 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (5.12) | 0.0 ±0.0 (6.88) | 18.8 ±9.8 (9.12) | +18.8 pts (+4.00 survival) |

| c3 difficulty 4 | random | greedy | engine | skill gap |
|---|---|---|---|---|
| win % (survival) | 0.0 ±0.0 (4.81) | 0.0 ±0.0 (5.69) | 12.5 ±8.3 (7.75) | +12.5 pts (+2.94 survival) |

## Head to head against greedy

Each agent and `greedy` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.

| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |
|---|---|---|---|---|---|---|---|
| c2 d0 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c2 d0 engine | 16 | 81.2 | 25.0 | 4 / 9 / 0 / 3 | +56.2 ±12.8 | 0.004 | beats greedy |
| c2 d1 random | 16 | 0.0 | 25.0 | 0 / 0 / 4 / 12 | -25.0 ±11.2 | 0.125 | no better than greedy (behind, not significant) |
| c2 d1 engine | 16 | 37.5 | 25.0 | 1 / 5 / 3 / 7 | +12.5 ±18.0 | 0.727 | no better than greedy (ahead, not significant) |
| c2 d2 random | 16 | 0.0 | 31.2 | 0 / 0 / 5 / 11 | -31.2 ±12.0 | 0.062 | no better than greedy (behind, not significant) |
| c2 d2 engine | 16 | 56.2 | 31.2 | 4 / 5 / 1 / 6 | +25.0 ±14.4 | 0.219 | no better than greedy (ahead, not significant) |
| c2 d3 random | 16 | 0.0 | 12.5 | 0 / 0 / 2 / 14 | -12.5 ±8.5 | 0.500 | no better than greedy (behind, not significant) |
| c2 d3 engine | 16 | 56.2 | 12.5 | 1 / 8 / 1 / 6 | +43.8 ±15.7 | 0.039 | beats greedy |
| c2 d4 random | 16 | 0.0 | 12.5 | 0 / 0 / 2 / 14 | -12.5 ±8.5 | 0.500 | no better than greedy (behind, not significant) |
| c2 d4 engine | 16 | 25.0 | 12.5 | 1 / 3 / 1 / 11 | +12.5 ±12.5 | 0.625 | no better than greedy (ahead, not significant) |
| c3 d0 random | 16 | 0.0 | 56.2 | 0 / 0 / 9 / 7 | -56.2 ±12.8 | 0.004 | worse than greedy |
| c3 d0 engine | 16 | 87.5 | 56.2 | 7 / 7 / 2 / 0 | +31.2 ±17.6 | 0.180 | no better than greedy (ahead, not significant) |
| c3 d1 random | 16 | 0.0 | 43.8 | 0 / 0 / 7 / 9 | -43.8 ±12.8 | 0.016 | worse than greedy |
| c3 d1 engine | 16 | 68.8 | 43.8 | 4 / 7 / 3 / 2 | +25.0 ±19.4 | 0.344 | no better than greedy (ahead, not significant) |
| c3 d2 random | 16 | 0.0 | 18.8 | 0 / 0 / 3 / 13 | -18.8 ±10.1 | 0.250 | no better than greedy (behind, not significant) |
| c3 d2 engine | 16 | 50.0 | 18.8 | 1 / 7 / 2 / 6 | +31.2 ±17.6 | 0.180 | no better than greedy (ahead, not significant) |
| c3 d3 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d3 engine | 16 | 18.8 | 0.0 | 0 / 3 / 0 / 13 | +18.8 ±10.1 | 0.250 | no better than greedy (ahead, not significant) |
| c3 d4 random | 16 | 0.0 | 0.0 | 0 / 0 / 0 / 16 | +0.0 ±0.0 | 1.000 | no better than greedy (level) |
| c3 d4 engine | 16 | 12.5 | 0.0 | 0 / 2 / 0 / 14 | +12.5 ±8.5 | 0.500 | no better than greedy (ahead, not significant) |

## Variants against the base

The first variant, `c2`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| c3 | 0 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.62 → 5.44 |
| c3 | 0 | greedy | 25.0 → 56.2 | +31.2 ±17.6 | 0.180 | 7.94 → 7.56 |
| c3 | 0 | engine | 81.2 → 87.5 | +6.2 ±14.3 | 1.000 | 8.88 → 9.19 |
| c3 | 1 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.38 → 5.38 |
| c3 | 1 | greedy | 25.0 → 43.8 | +18.8 ±18.8 | 0.508 | 7.31 → 7.06 |
| c3 | 1 | engine | 37.5 → 68.8 | +31.2 ±17.6 | 0.180 | 9.06 → 9.12 |
| c3 | 2 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 5.19 → 5.06 |
| c3 | 2 | greedy | 31.2 → 18.8 | -12.5 ±12.5 | 0.625 | 6.75 → 7.31 |
| c3 | 2 | engine | 56.2 → 50.0 | -6.2 ±19.3 | 1.000 | 8.12 → 8.75 |
| c3 | 3 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.88 → 5.12 |
| c3 | 3 | greedy | 12.5 → 0.0 | -12.5 ±8.5 | 0.500 | 6.50 → 6.88 |
| c3 | 3 | engine | 56.2 → 18.8 | -37.5 ±15.5 | 0.070 | 7.81 → 8.38 |
| c3 | 4 | random | 0.0 → 0.0 | +0.0 ±0.0 | 1.000 | 4.31 → 4.81 |
| c3 | 4 | greedy | 12.5 → 0.0 | -12.5 ±8.5 | 0.500 | 5.31 → 5.69 |
| c3 | 4 | engine | 25.0 → 12.5 | -12.5 ±12.5 | 0.625 | 7.56 → 7.19 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| c2 d0 random | 5.62 (0.48, 5..6) | discord_deck 16 | – / – | 34.8 / 0.00 | 32.88 | 3.25 / 0.31 |
| c2 d0 greedy | 7.94 (1.68, 4..10) | discord_deck 12, purge 4 | 5.25 / 39.25 | 5.7 / 2.25 | 41.44 | 1.25 / 1.06 |
| c2 d0 engine | 8.88 (2.00, 6..12) | deck_empty 3, purge 13 | 2.85 / 42.46 | 3.7 / 2.00 | 42.69 | 0.06 / 2.38 |
| c2 d1 random | 5.38 (0.60, 4..6) | discord_deck 16 | – / – | 30.1 / 0.00 | 37.69 | 3.94 / 0.19 |
| c2 d1 greedy | 7.31 (1.40, 5..11) | discord_deck 12, purge 4 | 5.00 / 41.25 | 7.0 / 2.25 | 42.44 | 1.81 / 0.75 |
| c2 d1 engine | 9.06 (1.82, 4..12) | deck_empty 1, discord_deck 9, purge 6 | 3.67 / 42.17 | 4.7 / 2.30 | 39.75 | 0.88 / 1.44 |
| c2 d2 random | 5.19 (0.73, 4..6) | discord_deck 16 | – / – | 36.3 / 0.00 | 35.44 | 4.94 / 0.19 |
| c2 d2 greedy | 6.75 (0.83, 5..8) | discord_deck 11, purge 5 | 3.40 / 40.40 | 8.5 / 2.09 | 39.06 | 1.88 / 0.56 |
| c2 d2 engine | 8.12 (1.69, 4..11) | deck_empty 1, discord_deck 6, purge 9 | 2.89 / 40.89 | 7.4 / 2.29 | 41.56 | 0.94 / 1.12 |
| c2 d3 random | 4.88 (0.48, 4..6) | discord_deck 16 | – / – | 40.1 / 0.00 | 33.12 | 5.19 / 0.19 |
| c2 d3 greedy | 6.50 (1.17, 5..9) | discord_deck 14, purge 2 | 4.00 / 41.50 | 21.7 / 1.43 | 36.25 | 3.38 / 0.38 |
| c2 d3 engine | 7.81 (1.63, 5..11) | deck_empty 1, discord_deck 6, purge 9 | 2.89 / 40.33 | 12.7 / 2.14 | 36.69 | 1.12 / 0.62 |
| c2 d4 random | 4.31 (0.46, 4..5) | discord_deck 16 | – / – | 39.0 / 0.00 | 31.81 | 4.62 / 0.12 |
| c2 d4 greedy | 5.31 (0.77, 4..7) | discord_deck 14, purge 2 | 3.50 / 37.00 | 27.1 / 1.07 | 37.00 | 3.75 / 0.19 |
| c2 d4 engine | 7.56 (1.22, 5..10) | discord_deck 12, purge 4 | 3.25 / 44.00 | 7.8 / 2.42 | 41.50 | 1.81 / 0.44 |
| c3 d0 random | 5.44 (0.50, 5..6) | discord_deck 16 | – / – | 53.4 / 0.00 | 47.75 | 4.94 / 0.44 |
| c3 d0 greedy | 7.56 (1.62, 5..12) | deck_empty 1, discord_deck 6, purge 9 | 4.22 / 53.89 | 3.0 / 2.57 | 60.50 | 0.50 / 0.88 |
| c3 d0 engine | 9.19 (1.42, 7..12) | deck_empty 1, discord_deck 1, purge 14 | 2.07 / 63.64 | 4.0 / 2.50 | 63.94 | 0.25 / 3.94 |
| c3 d1 random | 5.38 (0.48, 5..6) | discord_deck 16 | – / – | 50.1 / 0.00 | 51.19 | 5.38 / 0.25 |
| c3 d1 greedy | 7.06 (0.75, 6..8) | discord_deck 9, purge 7 | 4.29 / 50.43 | 9.1 / 2.22 | 58.19 | 1.81 / 0.38 |
| c3 d1 engine | 9.12 (0.99, 8..11) | discord_deck 5, purge 11 | 1.91 / 64.73 | 6.8 / 2.60 | 62.44 | 1.12 / 2.12 |
| c3 d2 random | 5.06 (0.24, 5..6) | discord_deck 15, presence 1 | – / – | 56.2 / 0.00 | 46.81 | 6.06 / 0.19 |
| c3 d2 greedy | 7.31 (0.98, 5..9) | discord_deck 13, purge 3 | 3.67 / 57.00 | 7.4 / 2.46 | 56.44 | 2.00 / 0.38 |
| c3 d2 engine | 8.75 (1.15, 7..11) | deck_empty 2, discord_deck 6, purge 8 | 1.62 / 63.75 | 4.1 / 2.75 | 63.94 | 1.12 / 1.38 |
| c3 d3 random | 5.12 (0.33, 5..6) | discord_deck 16 | – / – | 60.6 / 0.00 | 53.31 | 8.00 / 0.12 |
| c3 d3 greedy | 6.88 (0.93, 6..9) | discord_deck 16 | – / – | 26.3 / 1.88 | 58.19 | 4.38 / 0.06 |
| c3 d3 engine | 8.38 (0.78, 8..11) | deck_empty 1, discord_deck 12, purge 3 | 2.00 / 61.00 | 11.9 / 2.46 | 54.06 | 2.69 / 0.62 |
| c3 d4 random | 4.81 (0.39, 4..5) | discord_deck 16 | – / – | 66.9 / 0.00 | 42.62 | 7.25 / 0.06 |
| c3 d4 greedy | 5.69 (0.46, 5..6) | discord_deck 16 | – / – | 32.5 / 1.38 | 58.06 | 5.62 / 0.00 |
| c3 d4 engine | 7.19 (0.53, 6..8) | discord_deck 14, purge 2 | 2.50 / 63.00 | 20.8 / 1.93 | 60.88 | 3.56 / 0.56 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| c2 d0 random | 303.6 | 11.52 | 2% | – | – | 24.89 (79) | 6.94 | 15.62 | 3.00 | 4.88 / 15.50 (31%) |
| c2 d0 greedy | 400.1 | 16.22 | 2% | – | – | 20.43 (53) | 0.00 | 13.56 | 0.00 | 9.81 / 18.25 (54%) |
| c2 d0 engine | 454.5 | 15.53 | 2% | – | – | 15.53 (57) | 0.00 | 16.19 | 0.00 | 17.81 / 19.19 (93%) |
| c2 d1 random | 309.6 | 11.76 | 2% | – | – | 27.40 (89) | 7.38 | 21.06 | 5.12 | 6.75 / 20.19 (33%) |
| c2 d1 greedy | 389.2 | 15.65 | 2% | – | – | 26.01 (97) | 0.00 | 15.56 | 0.00 | 11.12 / 25.69 (43%) |
| c2 d1 engine | 467.0 | 15.75 | 2% | – | – | 18.18 (102) | 0.00 | 25.75 | 0.00 | 25.69 / 29.00 (89%) |
| c2 d2 random | 294.1 | 12.01 | 2% | – | – | 33.18 (100) | 8.12 | 20.88 | 5.00 | 5.81 / 20.38 (29%) |
| c2 d2 greedy | 360.2 | 15.23 | 2% | – | – | 27.65 (82) | 0.00 | 13.50 | 0.00 | 9.56 / 23.81 (40%) |
| c2 d2 engine | 425.9 | 14.76 | 2% | – | – | 20.50 (77) | 0.00 | 22.31 | 0.00 | 22.44 / 26.19 (86%) |
| c2 d3 random | 260.1 | 11.16 | 2% | – | – | 40.83 (118) | 5.56 | 18.19 | 4.31 | 5.25 / 20.31 (26%) |
| c2 d3 greedy | 356.9 | 14.74 | 2% | – | – | 35.75 (102) | 0.00 | 13.88 | 0.00 | 8.56 / 22.88 (37%) |
| c2 d3 engine | 422.8 | 14.18 | 2% | – | – | 26.11 (90) | 0.00 | 21.12 | 0.00 | 21.12 / 25.12 (84%) |
| c2 d4 random | 239.9 | 11.27 | 2% | – | – | 41.03 (141) | 4.81 | 15.88 | 4.06 | 4.69 / 17.44 (27%) |
| c2 d4 greedy | 309.8 | 14.39 | 1% | – | – | 40.31 (113) | 0.00 | 10.56 | 0.00 | 6.12 / 19.56 (31%) |
| c2 d4 engine | 424.5 | 14.00 | 2% | – | – | 27.36 (86) | 0.00 | 22.25 | 0.00 | 20.94 / 25.31 (83%) |
| c3 d0 random | 471.3 | 16.35 | 1% | – | – | 42.64 (135) | 21.75 | 25.56 | 5.12 | 7.25 / 20.38 (36%) |
| c3 d0 greedy | 583.4 | 21.81 | 1% | – | – | 32.40 (87) | 0.00 | 20.19 | 0.00 | 17.25 / 25.12 (69%) |
| c3 d0 engine | 734.2 | 22.17 | 1% | – | – | 26.87 (62) | 0.00 | 25.75 | 0.00 | 27.44 / 28.69 (96%) |
| c3 d1 random | 443.7 | 16.36 | 1% | – | – | 43.55 (161) | 15.62 | 25.56 | 5.19 | 8.69 / 25.31 (34%) |
| c3 d1 greedy | 555.6 | 21.25 | 1% | – | – | 38.00 (126) | 0.00 | 22.94 | 0.00 | 18.88 / 31.12 (61%) |
| c3 d1 engine | 735.1 | 22.05 | 1% | – | – | 29.95 (78) | 0.00 | 35.56 | 0.00 | 36.31 / 38.88 (93%) |
| c3 d2 random | 431.5 | 16.02 | 1% | – | – | 50.73 (127) | 14.50 | 27.31 | 6.62 | 9.12 / 24.44 (37%) |
| c3 d2 greedy | 586.7 | 21.30 | 1% | – | – | 43.62 (107) | 0.00 | 24.81 | 0.00 | 19.38 / 32.31 (60%) |
| c3 d2 engine | 707.9 | 21.08 | 1% | – | – | 33.39 (106) | 0.00 | 34.56 | 0.00 | 34.56 / 37.44 (92%) |
| c3 d3 random | 429.1 | 15.59 | 1% | – | – | 62.05 (178) | 12.88 | 26.69 | 7.19 | 8.19 / 25.81 (32%) |
| c3 d3 greedy | 588.7 | 20.48 | 1% | – | – | 56.26 (174) | 0.00 | 24.25 | 0.00 | 16.69 / 31.31 (53%) |
| c3 d3 engine | 682.8 | 19.75 | 1% | – | – | 45.37 (152) | 0.00 | 31.88 | 0.00 | 31.81 / 35.44 (90%) |
| c3 d4 random | 404.3 | 15.39 | 1% | – | – | 61.84 (219) | 13.25 | 27.50 | 6.94 | 7.38 / 22.62 (33%) |
| c3 d4 greedy | 493.0 | 18.97 | 1% | – | – | 60.57 (176) | 0.00 | 17.50 | 0.00 | 10.62 / 26.25 (40%) |
| c3 d4 engine | 619.0 | 18.70 | 1% | – | – | 48.63 (137) | 0.00 | 30.38 | 0.00 | 28.75 / 32.44 (89%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| c2 d0 | 21% / 50% / 28% | 6% / 85% / 9% |
| c2 d1 | 24% / 15% / 61% | 11% / 65% / 24% |
| c2 d2 | 33% / 26% / 42% | 15% / 65% / 20% |
| c2 d3 | 21% / 33% / 46% | 11% / 67% / 21% |
| c2 d4 | 37% / 10% / 54% | 19% / 52% / 29% |
| c3 d0 | 11% / 53% / 37% | 5% / 82% / 14% |
| c3 d1 | 17% / 34% / 49% | 9% / 65% / 26% |
| c3 d2 | 21% / 24% / 55% | 9% / 70% / 21% |
| c3 d3 | 29% / 13% / 58% | 15% / 70% / 15% |
| c3 d4 | 30% / 9% / 61% | 15% / 59% / 26% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| c2 d0 random | 13 of 16 | 0.86 | 0 of 0 | – |
| c2 d0 greedy | 15 of 16 | 0.65 | 4 of 4 | 0.81 |
| c2 d0 engine | 16 of 16 | 0.77 | 13 of 13 | 0.74 |
| c2 d1 random | 10 of 16 | 0.87 | 0 of 0 | – |
| c2 d1 greedy | 14 of 16 | 0.67 | 4 of 4 | 0.56 |
| c2 d1 engine | 16 of 16 | 0.86 | 6 of 6 | 0.87 |
| c2 d2 random | 13 of 16 | 0.92 | 0 of 0 | – |
| c2 d2 greedy | 12 of 16 | 0.67 | 5 of 5 | 0.81 |
| c2 d2 engine | 16 of 16 | 0.72 | 9 of 9 | 0.77 |
| c2 d3 random | 16 of 16 | 0.87 | 0 of 0 | – |
| c2 d3 greedy | 13 of 16 | 0.65 | 2 of 2 | 0.67 |
| c2 d3 engine | 14 of 16 | 0.76 | 8 of 9 | 0.74 |
| c2 d4 random | 11 of 16 | 0.84 | 0 of 0 | – |
| c2 d4 greedy | 13 of 16 | 0.65 | 2 of 2 | 0.50 |
| c2 d4 engine | 14 of 16 | 0.74 | 4 of 4 | 0.71 |
| c3 d0 random | 13 of 16 | 0.95 | 0 of 0 | – |
| c3 d0 greedy | 16 of 16 | 0.63 | 9 of 9 | 0.45 |
| c3 d0 engine | 16 of 16 | 0.75 | 14 of 14 | 0.75 |
| c3 d1 random | 15 of 16 | 0.95 | 0 of 0 | – |
| c3 d1 greedy | 16 of 16 | 0.63 | 7 of 7 | 0.54 |
| c3 d1 engine | 16 of 16 | 0.78 | 11 of 11 | 0.81 |
| c3 d2 random | 16 of 16 | 0.95 | 0 of 0 | – |
| c3 d2 greedy | 16 of 16 | 0.66 | 3 of 3 | 0.62 |
| c3 d2 engine | 16 of 16 | 0.77 | 8 of 8 | 0.83 |
| c3 d3 random | 15 of 16 | 0.92 | 0 of 0 | – |
| c3 d3 greedy | 16 of 16 | 0.60 | 0 of 0 | – |
| c3 d3 engine | 16 of 16 | 0.82 | 3 of 3 | 0.92 |
| c3 d4 random | 15 of 16 | 0.91 | 0 of 0 | – |
| c3 d4 greedy | 16 of 16 | 0.60 | 0 of 0 | – |
| c3 d4 engine | 16 of 16 | 0.85 | 2 of 2 | 1.00 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 0: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.12 / 0.19 / 6.8, – | 2.81 / 0.25 / 9.9, – |
| greedy | 2.38 / 5.88 / 14.2, 75% (100%) | 3.25 / 9.81 / 20.8, 100% (100%) |
| engine | 4.62 / 7.62 / 20.8, 100% (100%) | 7.00 / 12.44 / 33.3, 100% (100%) |

| difficulty 1: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 1.75 / 0.06 / 7.4, – | 2.94 / 0.25 / 10.6, – |
| greedy | 2.06 / 5.25 / 13.3, 75% (75%) | 3.38 / 10.06 / 20.9, 100% (100%) |
| engine | 3.31 / 6.94 / 19.4, 83% (67%) | 6.06 / 13.06 / 33.0, 100% (100%) |

| difficulty 2: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.00 / 0.06 / 7.6, – | 2.75 / 0.12 / 10.0, – |
| greedy | 2.00 / 5.44 / 13.4, 80% (100%) | 3.12 / 10.75 / 21.5, 100% (100%) |
| engine | 3.38 / 7.56 / 19.6, 100% (89%) | 4.75 / 11.44 / 28.2, 100% (100%) |

| difficulty 3: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 2.06 / 0.06 / 6.8, – | 2.81 / 0.06 / 9.3, – |
| greedy | 1.88 / 4.31 / 12.2, 50% (100%) | 3.12 / 9.06 / 20.3, – |
| engine | 2.88 / 6.50 / 18.2, 89% (67%) | 4.44 / 11.38 / 28.5, 100% (100%) |

| difficulty 4: buildings / tier 2 / cards per game, wins with 2+ buildings | c2 | c3 |
|---|---|---|
| random | 1.81 / 0.06 / 6.1, – | 2.75 / 0.19 / 8.9, – |
| greedy | 1.56 / 3.44 / 10.4, 50% (100%) | 2.38 / 6.56 / 16.0, – |
| engine | 2.38 / 6.44 / 16.8, 75% (75%) | 4.44 / 8.88 / 24.4, 100% (100%) |

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: kept (bought) per offer. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per difficulty in `cards.csv`.

| card | random: pick (offered) / keep / play | greedy: pick (offered) / keep / play | engine: pick (offered) / keep / play |
|---|---|---|---|
| adaptive_malware | 2.4 (50) / 46% / 20% | 3.8 (25) / 152% / 88% | 3.5 (32) / 144% / 92% |
| adversarial_inference_sweep | 2.2 (52) / 48% / 4% | 2.4 (171) / 23% / 3% | 2.4 (227) / 33% / 0% |
| aegis_dome_array | 2.7 (61) / 52% / 0% | 2.4 (191) / 13% / 0% | 2.3 (223) / 30% / 0% |
| aegis_grid_lockdown | 2.6 (66) / 47% / 13% | 2.3 (208) / 13% / 0% | 2.2 (249) / 9% / 0% |
| agency_directorate | 2.5 (69) / 48% / 0% | 2.4 (205) / 5% / 0% | 2.3 (246) / 10% / 0% |
| agency_field_station | 2.6 (69) / 57% / 18% | 3.8 (29) / 117% / 25% | 3.7 (37) / 103% / 26% |
| amnesty_decree | 2.7 (53) / 62% / 6% | 3.3 (15) / 93% / 83% | 3.6 (23) / 91% / 82% |
| antiproton_offtake_deal | 2.7 (66) / 42% / 0% | 2.2 (182) / 2% / 0% | 2.3 (227) / 11% / 0% |
| arc_furnace_complex | 2.1 (53) / 40% / 0% | 2.3 (187) / 0% / – | 2.3 (215) / 31% / 0% |
| arcology_foundry | 2.6 (83) / 41% / 0% | 2.2 (190) / 25% / 0% | 2.2 (212) / 43% / 1% |
| asteroid_salvage_rights | 2.4 (60) / 37% / 0% | 2.2 (207) / 43% / 80% | 2.2 (256) / 48% / 93% |
| back_channel_talks | 2.3 (19) / 216% / 2% | 4.0 (11) / 100% / 100% | 3.4 (12) / 92% / 91% |
| baseload_contract | 2.3 (46) / 54% / 8% | 3.9 (14) / 200% / 0% | 3.6 (28) / 139% / 24% |
| beamed_power_constellation | 2.7 (81) / 42% / 0% | 2.3 (196) / 3% / 0% | 2.2 (244) / 11% / 0% |
| benchmark_suite | 2.6 (37) / 81% / 0% | 3.9 (15) / 0% / 0% | 3.6 (21) / 0% / 0% |
| black_ice_citadel | 2.3 (82) / 46% / 0% | 2.3 (194) / 0% / – | 2.3 (251) / 1% / 0% |
| blackout_script | 2.2 (32) / 75% / 21% | 3.6 (10) / 140% / 0% | 3.6 (17) / 100% / 90% |
| blackstart_protocol | 2.4 (40) / 100% / 42% | 3.8 (19) / 332% / 0% | 3.0 (21) / 310% / 97% |
| bonded_warehouse | 2.4 (47) / 96% / 17% | 3.8 (19) / 253% / 0% | 3.9 (34) / 174% / 14% |
| bounty_board | 2.7 (32) / 56% / 50% | 3.8 (12) / 100% / 0% | 3.7 (15) / 93% / 100% |
| breach_and_clear | 2.5 (26) / 123% / 18% | 3.8 (8) / 425% / 79% | 3.4 (12) / 350% / 77% |
| breakthrough_leak | 2.5 (26) / 119% / 19% | 3.9 (12) / 350% / 100% | 3.8 (19) / 253% / 79% |
| bridge_loan | 2.5 (30) / 150% / 45% | 3.8 (9) / 489% / 0% | 3.5 (14) / 343% / 88% |
| brownout_relief | 2.3 (13) / 315% / 21% | 4.0 (3) / 1367% / 65% | 4.0 (4) / 1000% / 66% |
| bulk_silicon_procurement | 2.6 (55) / 73% / 24% | 3.8 (29) / 162% / 96% | 3.7 (34) / 150% / 93% |
| capacitor_discharge | 2.9 (26) / 58% / 44% | 3.6 (5) / 280% / 0% | 3.4 (12) / 183% / 92% |
| cascade_failure | 2.7 (85) / 44% / 5% | 2.4 (153) / 88% / 0% | 2.0 (169) / 82% / 7% |
| ceasefire_monitors | 2.6 (35) / 89% / 32% | 3.9 (14) / 236% / 91% | 3.5 (19) / 200% / 95% |
| chip_spot_market | 1.9 (39) / 138% / 51% | 3.8 (9) / 311% / 0% | 3.5 (24) / 175% / 40% |
| citation_engine | 2.3 (51) / 114% / 33% | 3.8 (23) / 261% / 0% | 3.8 (29) / 228% / 20% |
| coalition_charter | 2.5 (53) / 49% / 8% | 2.3 (157) / 87% / 93% | 2.3 (170) / 83% / 95% |
| collateralized_grid | 2.3 (45) / 109% / 25% | 3.6 (17) / 306% / 0% | 3.7 (23) / 243% / 38% |
| commons_grid | 2.0 (27) / 122% / 27% | 3.9 (9) / 378% / 15% | 3.5 (20) / 225% / 12% |
| compute_siphon | 2.8 (41) / 83% / 24% | 3.8 (12) / 367% / 84% | 3.4 (27) / 230% / 94% |
| consular_pouch | 2.2 (39) / 123% / 25% | 3.6 (23) / 265% / 94% | 3.2 (24) / 267% / 100% |
| consulate_tower | 2.5 (55) / 105% / 17% | 3.9 (10) / 440% / 100% | 3.7 (31) / 203% / 75% |
| continental_supergrid | 2.5 (62) / 47% / 0% | 2.3 (191) / 3% / 0% | 2.3 (237) / 12% / 17% |
| cryo_compute_burn | 2.2 (27) / 74% / 61% | 3.6 (17) / 188% / 0% | 3.5 (20) / 170% / 6% |
| darknet_exchange | 2.2 (36) / 89% / 15% | 3.6 (12) / 217% / 12% | 3.6 (18) / 167% / 9% |
| data_licensing_deal | 2.7 (70) / 44% / 0% | 2.1 (194) / 1% / 0% | 2.3 (241) / 2% / 40% |
| dataset_scraper | 2.8 (17) / 153% / 0% | 3.8 (5) / 0% / – | 2.9 (7) / 14% / 0% |
| deep_state_ledger | 2.6 (91) / 45% / 2% | 2.2 (156) / 92% / 0% | 2.3 (169) / 83% / 0% |
| demand_response_grid | 2.0 (12) / 167% / 55% | 3.8 (4) / 850% / 0% | 3.5 (4) / 825% / 21% |
| development_loan | 2.3 (45) / 69% / 42% | 3.8 (19) / 279% / 95% | 3.6 (24) / 242% / 100% |
| diplomatic_expulsion | 2.4 (24) / 246% / 46% | 4.0 (8) / 688% / 95% | 3.8 (17) / 353% / 100% |
| dyson_swarm_tranche | 2.5 (83) / 45% / 0% | 2.2 (192) / 0% / – | 2.2 (249) / 9% / 0% |
| embassy_compound | 2.7 (23) / 213% / 20% | 3.8 (12) / 550% / 100% | 3.7 (15) / 467% / 99% |
| endowed_research_chair | 2.8 (34) / 115% / 20% | 3.8 (12) / 267% / 0% | 3.7 (18) / 189% / 0% |
| eviction_worm | 2.5 (48) / 90% / 16% | 3.7 (20) / 220% / 98% | 3.6 (30) / 170% / 94% |
| exabyte_archive | 2.6 (75) / 48% / 0% | 2.3 (215) / 3% / 0% | 2.2 (247) / 4% / 0% |
| exodus_corridors | 2.5 (106) / 35% / 0% | 2.0 (158) / 92% / 18% | 2.3 (181) / 86% / 81% |
| expansion_fund | 2.9 (38) / 111% / 21% | 3.9 (21) / 290% / 0% | 3.6 (31) / 226% / 7% |
| faraday_yard | 2.6 (22) / 209% / 37% | 3.7 (14) / 307% / 0% | 3.6 (13) / 323% / 34% |
| field_sensor_grid | 2.1 (17) / 24% / 50% | 4.0 (2) / 100% / 0% | 3.6 (14) / 71% / 0% |
| flash_exploit | 2.7 (36) / 89% / 23% | 3.5 (17) / 182% / 35% | 3.7 (22) / 168% / 95% |
| foundation_model_campus | 2.7 (78) / 51% / 0% | 2.3 (200) / 0% / 0% | 2.3 (254) / 0% / – |
| full_automation_mandate | 2.4 (84) / 43% / 6% | 2.3 (194) / 26% / 0% | 2.2 (232) / 28% / 23% |
| fusion_startup_stake | 2.7 (38) / 97% / 3% | 3.8 (19) / 232% / 0% | 3.8 (26) / 200% / 54% |
| grad_student_pool | 2.2 (33) / 79% / 48% | 3.6 (15) / 233% / 0% | 3.5 (19) / 205% / 54% |
| great_liberation | 2.5 (63) / 40% / 8% | 2.3 (188) / 77% / 78% | 2.3 (233) / 70% / 86% |
| grid_census | 3.0 (20) / 165% / 36% | 3.9 (9) / 311% / 0% | 3.8 (10) / 300% / 97% |
| grid_tied_smelter | 2.4 (32) / 112% / 32% | 3.8 (13) / 285% / 0% | 3.3 (19) / 221% / 41% |
| hardened_bunker_complex | 2.5 (32) / 78% / 52% | 4.0 (8) / 525% / 0% | 3.6 (14) / 343% / 48% |
| hardened_conduits | 2.7 (30) / 163% / 35% | 3.8 (4) / 1050% / 0% | 3.9 (14) / 371% / 4% |
| hardpoint_lockdown | 2.3 (43) / 70% / 40% | 3.7 (11) / 155% / 0% | 3.2 (17) / 129% / 0% |
| honeypot_relay | 2.6 (31) / 152% / 17% | 3.8 (16) / 262% / 0% | 3.8 (18) / 233% / 9% |
| hyperscale_botnet | 2.4 (61) / 51% / 0% | 2.3 (204) / 0% / 0% | 2.3 (269) / 0% / 0% |
| interceptor_battery | 2.8 (17) / 241% / 34% | 3.9 (10) / 500% / 0% | 3.5 (13) / 400% / 4% |
| interconnector_treaty | 2.4 (90) / 43% / 3% | 2.3 (200) / 6% / 0% | 2.3 (252) / 7% / 22% |
| intrusion_countermeasures | 2.5 (36) / 106% / 8% | 3.6 (13) / 300% / 5% | 3.8 (20) / 215% / 39% |
| intrusion_lab | 2.4 (36) / 106% / 20% | 3.5 (12) / 300% / 0% | 3.6 (23) / 200% / 4% |
| joint_fabrication_pact | 2.4 (44) / 120% / 36% | 3.6 (21) / 190% / 98% | 3.6 (25) / 176% / 94% |
| joint_security_compact | 2.3 (22) / 132% / 35% | 3.5 (15) / 260% / 0% | 3.4 (22) / 209% / 19% |
| kill_team_roster | 2.2 (38) / 68% / 11% | 3.9 (20) / 35% / 0% | 3.5 (25) / 72% / 0% |
| killswitch_directive | 2.5 (106) / 38% / 0% | 2.4 (183) / 78% / 59% | 2.4 (198) / 78% / 46% |
| kinetic_interdiction | 2.5 (65) / 38% / 4% | 2.3 (204) / 80% / 96% | 2.4 (260) / 70% / 96% |
| liaison_corps | 2.4 (21) / 81% / 6% | 3.7 (3) / 600% / 0% | 3.9 (12) / 200% / 8% |
| liberation_accords | 2.5 (89) / 52% / 0% | 2.3 (185) / 34% / 84% | 2.2 (218) / 33% / 89% |
| lights_out_assembly | 2.5 (31) / 97% / 23% | 3.6 (10) / 250% / 0% | 3.4 (10) / 250% / 11% |
| liquidity_event | 2.4 (49) / 100% / 0% | 3.8 (26) / 0% / 0% | 3.5 (30) / 10% / 0% |
| literature_crawler | 2.5 (34) / 115% / 44% | 3.9 (20) / 85% / 89% | 3.7 (22) / 77% / 86% |
| logic_bomb | 2.2 (38) / 82% / 29% | 3.8 (14) / 279% / 96% | 3.6 (21) / 210% / 90% |
| manifesto_leak | 2.3 (30) / 97% / 0% | 3.8 (4) / 0% / 0% | 3.8 (15) / 0% / 0% |
| martial_cordon | 2.6 (81) / 36% / 3% | 2.2 (180) / 46% / 98% | 2.3 (219) / 43% / 99% |
| materials_hedge_desk | 2.4 (31) / 135% / 22% | 3.8 (17) / 265% / 0% | 3.7 (24) / 212% / 58% |
| megacorp_ipo | 2.2 (50) / 46% / 0% | 2.3 (187) / 0% / – | 2.2 (244) / 5% / 33% |
| mesh_intrusion_pact | 2.6 (59) / 37% / 5% | 2.2 (155) / 75% / 0% | 2.1 (194) / 69% / 35% |
| modular_expansion | – (0) / – / 0.24/game | – (0) / – / 0.57/game | – (0) / – / 0.49/game |
| mutual_aid_pact | 2.8 (19) / 221% / 29% | 4.0 (2) / 1550% / 100% | 3.8 (8) / 462% / 97% |
| mutual_defense_pact | 2.6 (31) / 113% / 23% | 3.9 (7) / 357% / 0% | 3.7 (15) / 213% / 34% |
| neural_cartographer | 2.6 (22) / 155% / 12% | 3.8 (9) / 133% / 0% | 3.8 (11) / 118% / 23% |
| neural_mirror | 2.6 (32) / 100% / 25% | 3.8 (11) / 173% / 0% | 3.4 (20) / 150% / 0% |
| node_severance | 2.8 (78) / 41% / 3% | 2.4 (201) / 73% / 95% | 2.3 (260) / 62% / 92% |
| open_assembly | 2.8 (25) / 76% / 32% | 3.4 (7) / 171% / 0% | 3.8 (9) / 156% / 39% |
| open_preprint | 2.5 (28) / 86% / 48% | 3.7 (11) / 91% / 92% | 3.9 (20) / 95% / 65% |
| open_weights_release | 2.3 (73) / 40% / 0% | 2.4 (159) / 72% / 69% | 2.4 (184) / 65% / 14% |
| orbital_embassy | 2.5 (67) / 42% / 0% | 2.3 (181) / 19% / 0% | 2.4 (226) / 16% / 19% |
| orbital_mass_driver | 2.5 (106) / 48% / 0% | 2.2 (199) / 2% / 0% | 2.4 (248) / 9% / 0% |
| orbital_observatory_array | 2.5 (70) / 34% / 0% | 2.3 (196) / 17% / 0% | 2.3 (235) / 27% / 0% |
| payroll_automation | 2.0 (32) / 138% / 50% | 3.6 (14) / 93% / 0% | 3.7 (21) / 95% / 0% |
| peace_dividend | 1.9 (24) / 42% / 60% | 3.8 (13) / 100% / 94% | 3.6 (16) / 94% / 84% |
| peacekeeper_rotation | 2.2 (19) / 105% / 15% | 4.0 (3) / 400% / 47% | 3.6 (14) / 150% / 83% |
| peacekeeping_detail | 1.8 (18) / 133% / 62% | 3.2 (6) / 100% / 0% | 3.4 (9) / 89% / 18% |
| peaker_turbines | 2.3 (45) / 67% / 17% | 3.8 (11) / 91% / 0% | 3.7 (18) / 67% / 0% |
| planetary_assembly | 2.6 (62) / 45% / 0% | 2.3 (188) / 15% / 0% | 2.3 (238) / 17% / 0% |
| planetary_referendum | 2.9 (90) / 39% / 3% | 2.2 (204) / 71% / 93% | 2.2 (261) / 67% / 95% |
| point_defense | – (0) / – / 0.35/game | – (0) / – / 0.00/game | – (0) / – / 0.44/game |
| positron_storage_ring | 2.5 (92) / 43% / 0% | 2.4 (208) / 0% / – | 2.3 (235) / 19% / 0% |
| predictive_modelling | 2.6 (30) / 113% / 26% | 3.9 (9) / 378% / 0% | 3.7 (15) / 260% / 0% |
| prefab_plant_kit | 2.9 (18) / 117% / 19% | 4.0 (3) / 33% / 0% | 3.4 (7) / 43% / 50% |
| pylon_drop | 2.6 (38) / 108% / 37% | 3.8 (11) / 27% / 0% | 3.7 (21) / 29% / 43% |
| quantum_decryptor | 2.8 (29) / 31% / 22% | 4.0 (8) / 50% / 0% | 3.7 (17) / 65% / 0% |
| recruit_operatives | 2.6 (29) / 100% / 0% | 3.8 (10) / 0% / 0% | 3.6 (12) / 0% / 0% |
| recursive_self_improvement | 2.5 (88) / 55% / 0% | 2.3 (192) / 0% / – | 2.4 (266) / 1% / 0% |
| red_team_audit | 2.2 (25) / 156% / 44% | 4.0 (2) / 100% / 17% | 3.6 (12) / 92% / 18% |
| relay_mast_rollout | 2.3 (31) / 87% / 24% | 4.0 (12) / 25% / 0% | 3.7 (21) / 24% / 17% |
| retained_earnings | 2.5 (43) / 70% / 44% | 3.8 (22) / 77% / 0% | 3.5 (31) / 74% / 67% |
| robotic_foundry | 2.7 (26) / 146% / 8% | 3.6 (14) / 57% / 0% | 3.5 (21) / 33% / 25% |
| rootkit_cache | 2.5 (22) / 123% / 7% | 3.8 (4) / 75% / 0% | 3.4 (7) / 57% / 75% |
| safehouse_network | 2.8 (26) / 85% / 14% | 3.9 (15) / 93% / 0% | 3.7 (10) / 60% / 7% |
| security_contractor_hq | 3.0 (34) / 124% / 38% | 3.9 (17) / 224% / 36% | 3.6 (21) / 210% / 27% |
| sentinel_mesh | 2.9 (24) / 292% / 39% | 3.8 (12) / 583% / 0% | 3.1 (15) / 493% / 4% |
| sentry_drone_wing | 2.5 (22) / 95% / 24% | 4.0 (1) / 100% / 0% | 3.8 (6) / 67% / 0% |
| shadow_broker_feed | 2.4 (13) / 100% / 25% | 3.7 (9) / 67% / 67% | 4.0 (7) / 71% / 50% |
| shared_compute_grant | 2.2 (18) / 289% / 23% | 3.7 (7) / 86% / 62% | 3.5 (10) / 80% / 25% |
| shared_exploit_kit | 2.5 (36) / 67% / 25% | 3.8 (12) / 92% / 12% | 3.5 (22) / 95% / 91% |
| shared_grid_accord | 2.6 (50) / 76% / 37% | 3.8 (28) / 32% / 0% | 3.1 (41) / 46% / 62% |
| shared_threat_intel | 2.7 (39) / 77% / 17% | 3.8 (20) / 85% / 0% | 3.6 (27) / 78% / 13% |
| shelter_order | – (0) / – / 0.50/game | – (0) / – / 0.00/game | – (0) / – / 0.00/game |
| shuttle_diplomacy | 1.8 (23) / 139% / 22% | 4.0 (6) / 267% / 89% | 3.4 (8) / 225% / 100% |
| signal_hijack | 2.9 (13) / 200% / 19% | 3.8 (4) / 100% / 50% | 3.8 (5) / 80% / 100% |
| simulation_cluster | 2.5 (35) / 43% / 7% | 3.9 (20) / 55% / 0% | 3.8 (17) / 65% / 0% |
| singularity_exploit | 2.7 (47) / 51% / 4% | 2.1 (172) / 35% / 84% | 2.2 (208) / 38% / 96% |
| sovereign_capital_fund | 2.0 (52) / 56% / 0% | 2.2 (172) / 28% / 0% | 2.1 (213) / 27% / 0% |
| spoofed_credentials | 2.5 (37) / 54% / 24% | 3.9 (22) / 86% / 91% | 3.5 (32) / 88% / 100% |
| station_prefab_order | 2.6 (32) / 116% / 8% | 3.8 (18) / 67% / 92% | 3.5 (22) / 82% / 91% |
| storm_hardening | 3.0 (28) / 57% / 11% | 3.7 (13) / 23% / 0% | 3.7 (15) / 80% / 0% |
| strike_team | – (0) / – / 0.38/game | – (0) / – / 0.96/game | – (0) / – / 0.98/game |
| substation_hub | 1.7 (22) / 114% / 12% | 4.0 (3) / 900% / 0% | 3.4 (10) / 340% / 83% |
| summit_caucus | 2.8 (42) / 62% / 0% | 3.6 (16) / 0% / 17% | 3.5 (29) / 0% / 0% |
| supply_chain_treaty | 3.2 (17) / 206% / 14% | 3.6 (12) / 67% / 0% | 3.7 (15) / 60% / 0% |
| surplus_auction | 3.0 (7) / 343% / 54% | – (0) / – / 0% | – (0) / – / 82% |
| synthetic_data_forge | 2.5 (48) / 71% / 15% | 3.8 (21) / 81% / 0% | 3.7 (23) / 52% / 0% |
| telemetry_archive | 2.6 (53) / 53% / 29% | 3.7 (27) / 130% / 58% | 3.6 (34) / 112% / 52% |
| threat_forecast_engine | 2.7 (18) / 139% / 18% | 3.8 (12) / 100% / 75% | 3.8 (13) / 85% / 75% |
| threat_neutralization_mandate | 2.4 (74) / 46% / 0% | 2.4 (196) / 13% / 0% | 2.3 (228) / 25% / 14% |
| threat_triage | 2.4 (24) / 167% / 25% | 3.5 (2) / 50% / 0% | 3.4 (8) / 25% / 0% |
| tidal_barrage | 2.7 (42) / 150% / 36% | 3.7 (26) / 173% / 0% | 3.6 (24) / 183% / 90% |
| total_containment | 2.5 (60) / 50% / 0% | 2.4 (171) / 10% / 71% | 2.2 (237) / 23% / 91% |
| transmission_corridor | 2.4 (33) / 61% / 18% | 3.8 (15) / 47% / 0% | 3.4 (18) / 28% / 20% |
| treaty_grid_exchange | 2.2 (24) / 167% / 10% | 3.3 (6) / 600% / 0% | 3.6 (11) / 327% / 86% |
| turnkey_factory | – (0) / – / 0.42/game | – (0) / – / 0.79/game | – (0) / – / 0.99/game |
| vendor_financing | 2.4 (44) / 86% / 0% | 3.7 (21) / 0% / 0% | 3.4 (30) / 0% / 0% |
| venture_capital | – (0) / – / 0.46/game | – (0) / – / 1.00/game | – (0) / – / 1.00/game |
| zero_click_breach | 2.4 (36) / 97% / 6% | 3.9 (20) / 35% / 50% | 3.5 (22) / 73% / 76% |
| zero_knowledge_exploit | 2.3 (41) / 129% / 20% | 3.8 (17) / 65% / 62% | 3.7 (23) / 83% / 84% |

## Timing

Per game on one thread (12 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| c2 d0 random | 0.0 | 0.000 | 0.0 | 46.5 |
| c2 d0 greedy | 1791.6 | 4.478 | 0.0 | 1849.5 |
| c2 d0 engine | 2895.9 | 6.372 | 0.0 | 2960.3 |
| c2 d1 random | 0.0 | 0.000 | 0.0 | 50.3 |
| c2 d1 greedy | 2083.4 | 5.353 | 0.0 | 2139.8 |
| c2 d1 engine | 3286.9 | 7.038 | 0.0 | 3358.8 |
| c2 d2 random | 0.0 | 0.000 | 0.0 | 52.3 |
| c2 d2 greedy | 1908.4 | 5.297 | 0.0 | 1958.4 |
| c2 d2 engine | 2766.6 | 6.495 | 0.0 | 2831.2 |
| c2 d3 random | 0.0 | 0.000 | 0.0 | 44.1 |
| c2 d3 greedy | 2229.0 | 6.246 | 0.0 | 2286.8 |
| c2 d3 engine | 2414.9 | 5.711 | 0.0 | 2477.6 |
| c2 d4 random | 0.0 | 0.000 | 0.0 | 37.2 |
| c2 d4 greedy | 1848.9 | 5.969 | 0.0 | 1904.4 |
| c2 d4 engine | 2569.1 | 6.052 | 0.0 | 2630.5 |
| c3 d0 random | 0.0 | 0.000 | 0.0 | 106.7 |
| c3 d0 greedy | 5192.8 | 8.901 | 0.0 | 5315.2 |
| c3 d0 engine | 11207.7 | 15.264 | 0.0 | 11364.5 |
| c3 d1 random | 0.0 | 0.000 | 0.0 | 103.2 |
| c3 d1 greedy | 5482.4 | 9.868 | 0.0 | 5603.4 |
| c3 d1 engine | 10190.6 | 13.864 | 0.0 | 10352.2 |
| c3 d2 random | 0.0 | 0.000 | 0.0 | 109.6 |
| c3 d2 greedy | 5911.7 | 10.076 | 0.0 | 6043.4 |
| c3 d2 engine | 9291.5 | 13.125 | 0.0 | 9453.2 |
| c3 d3 random | 0.0 | 0.000 | 0.0 | 103.9 |
| c3 d3 greedy | 6467.7 | 10.987 | 0.0 | 6612.1 |
| c3 d3 engine | 8668.9 | 12.697 | 0.0 | 8818.4 |
| c3 d4 random | 0.0 | 0.000 | 0.0 | 96.8 |
| c3 d4 greedy | 5486.0 | 11.128 | 0.0 | 5599.0 |
| c3 d4 engine | 7317.0 | 11.821 | 0.0 | 7452.2 |

Where the games' time went (summed over workers): agents 97.3%, probes 0.0%, apply 1.1%, legal 0.7%, replay check 0.9%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.

**close-loss**

- [`records/close-loss-c14-baseline-c2-d4-engine-seed4.json`](records/close-loss-c14-baseline-c2-d4-engine-seed4.json): close loss at difficulty 4: discord_deck in round 10, 1 adversary piece(s) over the level-3 bar, supply 44 of 48, 1 cities corrupted
- [`records/close-loss-c29-baseline-c3-d4-engine-seed2.json`](records/close-loss-c29-baseline-c3-d4-engine-seed2.json): close loss at difficulty 4: discord_deck in round 8, 1 adversary piece(s) over the level-3 bar, supply 57 of 72, 1 cities corrupted

**early-lock-in**: none matched


**surprising-win**

- [`records/surprising-win-c13-baseline-c2-d4-greedy-seed2.json`](records/surprising-win-c13-baseline-c2-d4-greedy-seed2.json): won at difficulty 4 by greedy, below the top agent
- [`records/surprising-win-c13-baseline-c2-d4-greedy-seed12.json`](records/surprising-win-c13-baseline-c2-d4-greedy-seed12.json): won at difficulty 4 by greedy, below the top agent

