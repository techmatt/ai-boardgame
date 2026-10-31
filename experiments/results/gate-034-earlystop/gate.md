# The 034 gate rerun with early stopping (040, r19)

`aic-sim experiment experiments/gate-034.json --threads 4`: all 26 checks at d2 against the
planner, 2 and 3 countries, capped at 32 seeds, stopped by `"gate": {"z": 2.0, "min_games": 8,
"every": 4}`. Same seeds as 034 (`first_seed` 0). Generated from the run's `report.md`; the
compare is against `../screen-034-clocks/r19-gate.md` and `r19-gate-rest-8-seeds.md`.

**Verdict reproduced: the gate does not pass.** All 52 check verdicts have the sign 034
recorded. The 7 checks that ran to 32 seeds have 034's exact gap. A stopped check's gap is over its own seeds, so it reads differently from 034's
(never_tag=6 at c2 is +50 over 8 seeds, +28 over 32).

| run | checks at 32 seeds | games | game time (summed over threads) |
|---|---|---|---|
| Every check to the cap (estimated from these games' means) | 52 | 1,728 | 7.1 h |
| 034's plan, 11 checks hand-cut to 8 seeds at each size (same estimate) | 30 | 1,200 | 5.5 h |
| This run, stopped by rule | 7 | 872 | 4.2 h |

42% less game time than the cap and 23% less than 034's own plan, with every check
now held to one stated rule and no hand cut. Wall time is not comparable: 034 took 45 min for
its first 1,024 games on a shared 12 threads (the 8-seed rest not timed); this ran 1h 4m on 4
below-normal threads, beside other work.

`python scripts/bench/gate_stop_rate.py` puts the chance of a stopped verdict differing from
the cap's at about 1% for a level check, under 0.5% for a clear one.

| variant | d | check | games | stopped | win % | gap to its agent | paired se |
|---|---|---|---|---|---|---|---|
| c2 | 2 | planner:rich_spend=1 | 8 | clears the noise | 12 | +50 ±21 | 19 |
| c2 | 2 | planner:science_rush=1 | 32 | cap | 56 | +9 ±12 | 12 |
| c2 | 2 | planner:tier2_rush=1 | 32 | cap | 75 | -9 ±11 ✗ | 10 |
| c2 | 2 | planner:never_tag=1 | 24 | sign settled | 83 | -17 ±12 ✗ | 10 |
| c2 | 2 | planner:never_tag=2 | 28 | sign settled | 57 | +11 ±13 | 11 |
| c2 | 2 | planner:never_tag=3 | 32 | cap | 66 | +0 ±12 ✗ | 10 |
| c2 | 2 | planner:never_tag=4 | 20 | clears the noise | 35 | +35 ±15 | 15 |
| c2 | 2 | planner:never_tag=5 | 8 | clears the noise | 12 | +50 ±21 | 19 |
| c2 | 2 | planner:never_tag=6 | 8 | clears the noise | 12 | +50 ±21 | 19 |
| c2 | 2 | planner:no_powers=1 | 32 | cap | 69 | -3 ±12 ✗ | 11 |
| c2 | 2 | planner:unity_farm=1 | 28 | sign settled | 54 | +14 ±13 | 14 |
| c2 | 2 | planner:keep_k=2 | 20 | sign settled | 40 | +30 ±15 | 15 |
| c2 | 2 | planner:no_counters=1 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:keep_all=1 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:tier1_only=1 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:no_presence=1 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:only_tag=1 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:only_tag=2 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:only_tag=3 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:only_tag=4 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:only_tag=5 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:only_tag=6 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:events_only=1 | 12 | clears the noise | 17 | +50 ±17 | 19 |
| c2 | 2 | planner:ignore_threats=1 | 8 | clears the noise | 12 | +50 ±21 | 19 |
| c2 | 2 | planner:turtle=1 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c2 | 2 | planner:free_only=1 | 8 | clears the noise | 0 | +62 ±17 | 18 |
| c3 | 2 | planner:rich_spend=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:science_rush=1 | 20 | sign settled | 10 | +25 ±13 | 12 |
| c3 | 2 | planner:tier2_rush=1 | 32 | cap | 41 | -3 ±12 ✗ | 10 |
| c3 | 2 | planner:never_tag=1 | 32 | cap | 47 | -9 ±12 ✗ | 9 |
| c3 | 2 | planner:never_tag=2 | 32 | cap | 44 | -6 ±12 ✗ | 8 |
| c3 | 2 | planner:never_tag=3 | 28 | sign settled | 25 | +14 ±12 | 10 |
| c3 | 2 | planner:never_tag=4 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:never_tag=5 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:never_tag=6 | 12 | clears the noise | 8 | +33 ±16 | 14 |
| c3 | 2 | planner:no_powers=1 | 28 | sign settled | 29 | +11 ±13 | 11 |
| c3 | 2 | planner:unity_farm=1 | 20 | sign settled | 10 | +25 ±13 | 12 |
| c3 | 2 | planner:keep_k=2 | 12 | clears the noise | 8 | +33 ±16 | 14 |
| c3 | 2 | planner:no_counters=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:keep_all=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:tier1_only=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:no_presence=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:only_tag=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:only_tag=2 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:only_tag=3 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:only_tag=4 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:only_tag=5 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:only_tag=6 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:events_only=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:ignore_threats=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:turtle=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
| c3 | 2 | planner:free_only=1 | 12 | clears the noise | 0 | +42 ±14 | 15 |
