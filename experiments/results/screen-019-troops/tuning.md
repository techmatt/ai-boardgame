# 019 tuning variants (r10, `--quick`, 16 games a cell)

Each block is `python scripts/board/screen_019.py out/tune019/<x> --short` on an `aic-sim experiment` over `configs/baseline.json` with the overrides named. These runs walked troops by straight-line distance (ocean included); the final rule
walks over land, which changed the adopted grid by one or two games in a few cells
(`troops.md` is the rerun). Before-and-after for the full grid: `before-tuning.md` (r10 as first built) and `troops.md` (adopted).

## a: Unity thresholds at d0 (engine): `base` is r10 untuned (`unity_ease` 1, 3: nodes always count); `e123` ambassadors 1, troops 2, nodes 3 tips; `e122` / `e112` nodes at 2 tips (identical in play: the bar only moves at whole tips).

| variant | d | agent | games | win % | rounds | left: corrupted / nodes / troops | Unity / Discord tips | ends |
|---|---|---|---|---|---|---|---|---|
| base_c2 | 0 | engine | 16 | 19 | 11.6 | 0.2 / 2.1 / 2.1 | 2.5 / 5.1 | loss deck_empty 13, win purge 3 |
| base_c3 | 0 | engine | 16 | 0 | 12.0 | 0.6 / 6.3 / 4.1 | 2.4 / 7.7 | loss deck_empty 16 |
| e123_c2 | 0 | engine | 16 | 56 | 11.0 | 0.1 / 2.1 / 0.9 | 2.4 / 5.1 | win purge 9, loss deck_empty 7 |
| e123_c3 | 0 | engine | 16 | 31 | 11.4 | 0.5 / 6.4 / 2.9 | 2.2 / 7.4 | loss deck_empty 11, win purge 5 |
| e122_c2 | 0 | engine | 16 | 94 | 8.6 | 0.0 / 2.9 / 0.1 | 1.9 / 4.2 | win purge 15, loss deck_empty 1 |
| e122_c3 | 0 | engine | 16 | 62 | 10.2 | 0.4 / 6.2 / 1.8 | 1.9 / 6.1 | win purge 10, loss deck_empty 6 |
| e112_c2 | 0 | engine | 16 | 94 | 8.6 | 0.0 / 2.9 / 0.1 | 1.9 / 4.2 | win purge 15, loss deck_empty 1 |
| e112_c3 | 0 | engine | 16 | 62 | 10.2 | 0.4 / 6.2 / 1.8 | 1.9 / 6.1 | win purge 10, loss deck_empty 6 |

## b: 3 countries, d0 engine, with nodes at 2 tips (`e112`): `e111` everything but corrupted cities stops at 1 tip; `node2` removing a node sways 2; `troop2` removing a troop sways 2; `tip` Momentum 1 (`momentum_every`); `d2` a mislabelled repeat of `e112` (the cell sets d0).

| variant | d | agent | games | win % | rounds | left: corrupted / nodes / troops | Unity / Discord tips | ends |
|---|---|---|---|---|---|---|---|---|
| e111_c3 | 0 | engine | 16 | 100 | 6.8 | 0.0 / 5.5 / 0.0 | 1.1 / 3.6 | win purge 16 |
| e112_node2_c3 | 0 | engine | 16 | 75 | 9.6 | 0.2 / 6.0 / 1.2 | 1.9 / 5.9 | win purge 12, loss deck_empty 4 |
| e112_troop2_c3 | 0 | engine | 16 | 62 | 10.1 | 0.4 / 6.0 / 1.8 | 1.9 / 5.9 | win purge 10, loss deck_empty 6 |
| e112_tip_c3 | 0 | engine | 16 | 88 | 8.2 | 0.2 / 6.6 / 0.5 | 2.4 / 8.2 | win purge 13, loss deck_empty 2, win unity 1 |
| e112_d2_c3 | 0 | engine | 16 | 62 | 10.2 | 0.4 / 6.2 / 1.8 | 1.9 / 6.1 | win purge 10, loss deck_empty 6 |

## e: 3 countries, d0 engine, on `e112` + node 2: `free3` freeing a city sways 3; `troop2`, `base3` (`base_unity` 3), `node3`, `fu2` (`factory_unity` 2) moved nothing.

| variant | d | agent | games | win % | rounds | left: corrupted / nodes / troops | Unity / Discord tips | ends |
|---|---|---|---|---|---|---|---|---|
| free3 | 0 | engine | 16 | 100 | 6.3 | 0.0 / 4.9 / 0.1 | 2.1 / 3.4 | win purge 16 |
| troop2 | 0 | engine | 16 | 75 | 9.6 | 0.3 / 5.9 / 1.2 | 1.9 / 5.7 | win purge 12, loss deck_empty 4 |
| base3 | 0 | engine | 16 | 75 | 9.6 | 0.2 / 6.0 / 1.2 | 1.9 / 5.9 | win purge 12, loss deck_empty 4 |
| node3 | 0 | engine | 16 | 75 | 9.4 | 0.2 / 6.0 / 1.2 | 1.9 / 5.8 | win purge 12, loss deck_empty 4 |
| fu2 | 0 | engine | 16 | 75 | 9.6 | 0.2 / 6.0 / 1.2 | 1.9 / 5.9 | win purge 12, loss deck_empty 4 |

## c: Full grid, greedy and engine, `unity_ease` 1, 1, 1 (rejected: games end in 4–7 rounds and greedy wins 100% at d1–d2).

| variant | d | agent | games | win % | rounds | left: corrupted / nodes / troops | Unity / Discord tips | ends |
|---|---|---|---|---|---|---|---|---|
| c2 | 0 | greedy | 16 | 94 | 4.9 | 0.1 / 3.8 / 0.4 | 1.3 / 3.0 | win purge 15, loss deck_empty 1 |
| c2 | 0 | engine | 16 | 100 | 3.9 | 0.0 / 2.8 / 0.0 | 1.0 / 1.8 | win purge 16 |
| c2 | 1 | greedy | 16 | 100 | 4.5 | 0.0 / 4.7 / 0.1 | 1.2 / 3.7 | win purge 16 |
| c2 | 1 | engine | 16 | 81 | 6.8 | 0.4 / 5.4 / 1.3 | 2.0 / 6.4 | win purge 13, loss deck_empty 3 |
| c2 | 2 | greedy | 16 | 100 | 4.3 | 0.0 / 4.9 / 0.1 | 1.4 / 3.6 | win purge 16 |
| c2 | 2 | engine | 16 | 81 | 5.4 | 0.5 / 5.6 / 0.8 | 1.6 / 5.5 | win purge 13, loss deck_empty 2, loss supply 1 |
| c2 | 3 | greedy | 16 | 69 | 5.8 | 0.9 / 9.3 / 2.1 | 1.6 / 6.4 | win purge 11, loss supply 3, loss deck_empty 2 |
| c2 | 3 | engine | 16 | 81 | 5.9 | 0.4 / 8.8 / 1.2 | 1.8 / 6.0 | win purge 13, loss supply 2, loss deck_empty 1 |
| c2 | 4 | greedy | 16 | 44 | 4.4 | 1.4 / 8.1 / 1.7 | 0.8 / 4.6 | loss supply 9, win purge 7 |
| c2 | 4 | engine | 16 | 25 | 5.8 | 1.8 / 8.7 / 2.6 | 1.6 / 6.8 | loss supply 11, win purge 4, loss deck_empty 1 |
| c3 | 0 | greedy | 16 | 100 | 4.9 | 0.0 / 6.2 / 0.0 | 1.0 / 2.9 | win purge 16 |
| c3 | 0 | engine | 16 | 100 | 6.8 | 0.0 / 5.5 / 0.0 | 1.1 / 3.6 | win purge 16 |
| c3 | 1 | greedy | 16 | 69 | 6.7 | 1.1 / 9.4 / 3.0 | 0.9 / 6.8 | win purge 11, loss deck_empty 5 |
| c3 | 1 | engine | 16 | 88 | 6.1 | 0.4 / 7.2 / 1.6 | 1.2 / 5.2 | win purge 14, loss deck_empty 2 |
| c3 | 2 | greedy | 16 | 88 | 5.1 | 0.4 / 8.4 / 1.2 | 1.0 / 4.4 | win purge 14, loss deck_empty 2 |
| c3 | 2 | engine | 16 | 81 | 5.6 | 0.6 / 7.8 / 1.8 | 1.2 / 5.1 | win purge 13, loss deck_empty 3 |
| c3 | 3 | greedy | 16 | 69 | 5.5 | 0.9 / 12.2 / 2.1 | 1.0 / 5.9 | win purge 11, loss supply 4, loss deck_empty 1 |
| c3 | 3 | engine | 16 | 81 | 5.0 | 0.4 / 10.9 / 1.2 | 1.2 / 4.8 | win purge 13, loss supply 2, loss deck_empty 1 |
| c3 | 4 | greedy | 16 | 44 | 5.3 | 1.7 / 12.0 / 2.3 | 0.9 / 5.6 | loss supply 9, win purge 7 |
| c3 | 4 | engine | 16 | 56 | 4.9 | 1.2 / 10.8 / 1.6 | 0.8 / 4.6 | win purge 9, loss supply 7 |

## d: Full grid, `unity_ease` 1, 1, 2 and `alignment.node` 2 (3 countries d0 engine 75%).

| variant | d | agent | games | win % | rounds | left: corrupted / nodes / troops | Unity / Discord tips | ends |
|---|---|---|---|---|---|---|---|---|
| c2 | 0 | greedy | 16 | 94 | 7.1 | 0.1 / 4.8 / 0.4 | 2.1 / 4.4 | win purge 15, loss deck_empty 1 |
| c2 | 0 | engine | 16 | 94 | 8.1 | 0.0 / 2.8 / 0.1 | 1.9 / 3.9 | win purge 15, loss deck_empty 1 |
| c2 | 1 | greedy | 16 | 94 | 6.5 | 0.2 / 5.6 / 0.8 | 2.0 / 6.0 | win purge 15, loss deck_empty 1 |
| c2 | 1 | engine | 16 | 75 | 8.2 | 0.6 / 5.8 / 1.7 | 2.8 / 7.6 | win purge 12, loss deck_empty 4 |
| c2 | 2 | greedy | 16 | 100 | 6.1 | 0.0 / 5.9 / 0.4 | 2.3 / 5.8 | win purge 16 |
| c2 | 2 | engine | 16 | 81 | 6.2 | 0.4 / 6.3 / 0.8 | 2.1 / 6.4 | win purge 13, loss deck_empty 2, loss supply 1 |
| c2 | 3 | greedy | 16 | 50 | 7.3 | 1.3 / 10.3 / 3.8 | 2.4 / 8.5 | win purge 8, loss deck_empty 4, loss supply 4 |
| c2 | 3 | engine | 16 | 75 | 6.9 | 0.8 / 9.6 / 2.0 | 2.5 / 7.4 | win purge 12, loss deck_empty 2, loss supply 2 |
| c2 | 4 | greedy | 16 | 38 | 5.1 | 1.5 / 8.5 / 2.2 | 1.1 / 5.4 | loss supply 10, win purge 6 |
| c2 | 4 | engine | 16 | 25 | 5.9 | 1.8 / 8.7 / 3.0 | 1.6 / 6.9 | loss supply 11, win purge 4, loss deck_empty 1 |
| c3 | 0 | greedy | 16 | 62 | 9.1 | 0.8 / 8.7 / 2.3 | 1.8 / 7.6 | win purge 10, loss deck_empty 6 |
| c3 | 0 | engine | 16 | 75 | 9.6 | 0.2 / 6.0 / 1.2 | 1.9 / 5.9 | win purge 12, loss deck_empty 4 |
| c3 | 1 | greedy | 16 | 44 | 9.9 | 1.6 / 11.8 / 5.1 | 1.7 / 10.9 | loss deck_empty 9, win purge 7 |
| c3 | 1 | engine | 16 | 75 | 8.4 | 0.6 / 8.4 / 2.6 | 2.2 / 8.3 | win purge 12, loss deck_empty 4 |
| c3 | 2 | greedy | 16 | 38 | 9.0 | 1.6 / 11.5 / 4.9 | 1.8 / 10.2 | loss deck_empty 10, win purge 6 |
| c3 | 2 | engine | 16 | 62 | 8.4 | 1.2 / 10.2 / 2.9 | 1.9 / 9.1 | win purge 10, loss deck_empty 6 |
| c3 | 3 | greedy | 16 | 38 | 7.6 | 1.6 / 14.4 / 3.8 | 1.6 / 8.9 | win purge 6, loss supply 6, loss deck_empty 4 |
| c3 | 3 | engine | 16 | 62 | 7.5 | 0.9 / 13.6 / 3.0 | 1.9 / 8.4 | win purge 10, loss deck_empty 3, loss supply 3 |
| c3 | 4 | greedy | 16 | 25 | 6.2 | 2.0 / 12.8 / 3.1 | 1.0 / 7.0 | loss supply 12, win purge 4 |
| c3 | 4 | engine | 16 | 50 | 5.9 | 1.4 / 11.6 / 2.1 | 1.2 / 6.0 | loss supply 8, win purge 8 |

## f: Full grid, `unity_ease` 1, 1, 2, `alignment.node` 2, `alignment.free_city` 3: adopted (the final screen repeats it with random added, `troops.md`).

| variant | d | agent | games | win % | rounds | left: corrupted / nodes / troops | Unity / Discord tips | ends |
|---|---|---|---|---|---|---|---|---|
| c2 | 0 | greedy | 16 | 88 | 7.2 | 0.4 / 4.6 / 0.6 | 2.1 / 4.3 | win purge 14, loss deck_empty 2 |
| c2 | 0 | engine | 16 | 100 | 8.0 | 0.0 / 2.9 / 0.0 | 2.0 / 3.6 | win purge 16 |
| c2 | 1 | greedy | 16 | 94 | 6.3 | 0.2 / 5.7 / 0.6 | 2.2 / 5.8 | win purge 15, loss deck_empty 1 |
| c2 | 1 | engine | 16 | 81 | 7.8 | 0.5 / 5.4 / 1.3 | 2.5 / 6.9 | win purge 13, loss deck_empty 2, loss supply 1 |
| c2 | 2 | greedy | 16 | 94 | 5.9 | 0.2 / 5.6 / 0.5 | 2.3 / 5.5 | win purge 15, loss deck_empty 1 |
| c2 | 2 | engine | 16 | 81 | 6.4 | 0.4 / 6.4 / 0.8 | 2.1 / 6.2 | win purge 13, loss deck_empty 2, loss supply 1 |
| c2 | 3 | greedy | 16 | 56 | 6.9 | 1.2 / 9.8 / 3.1 | 2.6 / 8.0 | win purge 9, loss supply 4, loss deck_empty 3 |
| c2 | 3 | engine | 16 | 75 | 7.1 | 0.8 / 9.6 / 2.3 | 2.8 / 7.7 | win purge 12, loss deck_empty 2, loss supply 2 |
| c2 | 4 | greedy | 16 | 38 | 5.1 | 1.5 / 8.5 / 2.4 | 1.2 / 5.4 | loss supply 10, win purge 6 |
| c2 | 4 | engine | 16 | 25 | 5.9 | 1.8 / 8.3 / 3.1 | 1.8 / 6.8 | loss supply 11, win purge 4, loss deck_empty 1 |
| c3 | 0 | greedy | 16 | 88 | 7.1 | 0.4 / 6.9 / 0.4 | 2.1 / 5.4 | win purge 14, loss deck_empty 2 |
| c3 | 0 | engine | 16 | 100 | 6.3 | 0.0 / 4.9 / 0.1 | 2.1 / 3.4 | win purge 16 |
| c3 | 1 | greedy | 16 | 75 | 8.1 | 0.8 / 9.6 / 2.2 | 2.4 / 8.5 | win purge 12, loss deck_empty 4 |
| c3 | 1 | engine | 16 | 81 | 7.6 | 0.5 / 7.3 / 2.0 | 2.3 / 6.7 | win purge 13, loss deck_empty 3 |
| c3 | 2 | greedy | 16 | 69 | 6.8 | 0.9 / 9.4 / 3.0 | 2.1 / 6.9 | win purge 11, loss deck_empty 5 |
| c3 | 2 | engine | 16 | 88 | 6.2 | 0.4 / 8.2 / 0.5 | 2.2 / 5.8 | win purge 14, loss deck_empty 2 |
| c3 | 3 | greedy | 16 | 56 | 6.9 | 1.5 / 15.4 / 3.6 | 1.7 / 8.4 | win purge 9, loss supply 4, loss deck_empty 3 |
| c3 | 3 | engine | 16 | 62 | 7.2 | 0.9 / 13.1 / 2.8 | 2.4 / 7.6 | win purge 10, loss deck_empty 4, loss supply 2 |
| c3 | 4 | greedy | 16 | 50 | 5.6 | 1.4 / 12.4 / 2.2 | 1.6 / 6.1 | win purge 8, loss supply 8 |
| c3 | 4 | engine | 16 | 69 | 5.5 | 0.9 / 11.4 / 1.4 | 1.8 / 5.6 | win purge 11, loss supply 5 |
