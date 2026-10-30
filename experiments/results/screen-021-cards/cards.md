# Card and resource screen (021, rules r12)

`aic-sim experiment experiments/screen-021-cards.json --quick --out out/screen-021`
(2 and 3 countries × d0-d4 × random, greedy, engine; 16 seeds a cell, 480 games,
about 3 min on 12 threads), then `python scripts/cards/screen_021.py out/screen-021`.
`report.md` is the harness's cycle report of the tuned run; `cards.csv` and `games.csv`
its tables. With 16 games a cell the noise is about ±12 points.

## Win rates: before and after

r11 is `main` before 021 (the same spec on the 020 binary); "r12 untuned" is 021's rules
with the prompt's starting values (neutral city energy 2 credits, `remove_energy` 2, base
insight 2, no movement cards or second tracks); "r12" is the committed baseline.

| countries | d | greedy: r11 / r12 untuned / r12 | engine: r11 / r12 untuned / r12 |
|---|---|---|---|
| 2 | 0 | 94 / 75 / 88 | 100 / 88 / 100 |
| 2 | 1 | 100 / 38 / 81 | 88 / 69 / 100 |
| 2 | 2 | 94 / 38 / 88 | 75 / 62 / 94 |
| 2 | 3 | 50 / 31 / 75 | 81 / 44 / 75 |
| 2 | 4 | 31 / 0 / 19 | 38 / 19 / 19 |
| 3 | 0 | 94 / 31 / 69 | 94 / 56 / 88 |
| 3 | 1 | 75 / 31 / 50 | 94 / 44 / 94 |
| 3 | 2 | 81 / 31 / 31 | 100 / 62 / 81 |
| 3 | 3 | 56 / 12 / 31 | 69 / 38 / 75 |
| 3 | 4 | 44 / 0 / 6 | 62 / 6 / 31 |

## Tuning (021 adaptations)

All at d0 (and d2, d4 where named), engine, 16 games a cell, `--set` on the baseline:

| Change | c3 d0 engine | Kept? |
|---|---|---|
| r12 untuned | 56% | |
| `board.city_energy_credits=1` / `=0` | 75% / 75% | no: back to 2 once the rest was in |
| `board.remove_energy=1` | 69% | yes |
| both | 81% | |
| `cards.keep_tier1=0` | 94% | no (insight was the bottleneck: raised base insight instead) |
| `cards.insight_credits=3` | no change | no |
| base insight 3 a board (content) with the two above | 81%; c2 94% | yes |
| + movement cards, second tracks, city energy 1 credit | 100% at d0-d3 (c3) | too easy |
| city energy back to 2 credits, remove 1 | c2 100 / 94 / 19, c3 88 / 81 / 31 at d0 / d2 / d4 | **yes** |
| city 2, remove 2 (the prompt's values) | c2 100 / 81 / 12, c3 81 / 62 / 31 | no: c3 d0 below 90 |
| agent: `w.hand_cap=8` (count only the best 8 cards in hand) | c2 d0 38% | no: off by default |

## The tuned run (r12)

| variant | d | agent | games | win % | rounds | ends |
|---|---|---|---|---|---|---|
| c2 | 0 | random | 16 | 0 | 8.5 | loss supply 11, loss cities 3, loss deck_empty 2 |
| c2 | 0 | greedy | 16 | 88 | 7.6 | win purge 14, loss deck_empty 2 |
| c2 | 0 | engine | 16 | 100 | 6.8 | win purge 16 |
| c2 | 1 | random | 16 | 0 | 7.2 | loss supply 10, loss cities 5, loss deck_empty 1 |
| c2 | 1 | greedy | 16 | 81 | 7.6 | win purge 13, loss deck_empty 3 |
| c2 | 1 | engine | 16 | 100 | 7.0 | win purge 16 |
| c2 | 2 | random | 16 | 0 | 6.2 | loss supply 12, loss cities 4 |
| c2 | 2 | greedy | 16 | 88 | 6.4 | win purge 14, loss deck_empty 2 |
| c2 | 2 | engine | 16 | 94 | 6.9 | win purge 15, loss deck_empty 1 |
| c2 | 3 | random | 16 | 0 | 5.4 | loss supply 15, loss cities 1 |
| c2 | 3 | greedy | 16 | 75 | 7.4 | win purge 12, loss deck_empty 2, loss supply 2 |
| c2 | 3 | engine | 16 | 75 | 7.2 | win purge 12, loss supply 3, loss deck_empty 1 |
| c2 | 4 | random | 16 | 0 | 3.9 | loss supply 16 |
| c2 | 4 | greedy | 16 | 19 | 5.4 | loss supply 13, win purge 3 |
| c2 | 4 | engine | 16 | 19 | 5.5 | loss supply 13, win purge 3 |
| c3 | 0 | random | 16 | 0 | 8.4 | loss supply 12, loss cities 3, loss deck_empty 1 |
| c3 | 0 | greedy | 16 | 69 | 7.9 | win purge 11, loss deck_empty 5 |
| c3 | 0 | engine | 16 | 88 | 6.1 | win purge 14, loss deck_empty 2 |
| c3 | 1 | random | 16 | 0 | 7.6 | loss supply 14, loss deck_empty 1, loss cities 1 |
| c3 | 1 | greedy | 16 | 50 | 9.1 | win purge 8, loss deck_empty 8 |
| c3 | 1 | engine | 16 | 94 | 6.6 | win purge 15, loss deck_empty 1 |
| c3 | 2 | random | 16 | 0 | 6.3 | loss supply 15, loss cities 1 |
| c3 | 2 | greedy | 16 | 31 | 9.7 | loss deck_empty 11, win purge 5 |
| c3 | 2 | engine | 16 | 81 | 7.1 | win purge 13, loss deck_empty 3 |
| c3 | 3 | random | 16 | 0 | 5.2 | loss supply 16 |
| c3 | 3 | greedy | 16 | 31 | 7.7 | loss supply 6, win purge 5, loss deck_empty 5 |
| c3 | 3 | engine | 16 | 75 | 6.9 | win purge 11, loss deck_empty 2, loss supply 2, win unity 1 |
| c3 | 4 | random | 16 | 0 | 4.1 | loss supply 16 |
| c3 | 4 | greedy | 16 | 6 | 5.4 | loss supply 15, win purge 1 |
| c3 | 4 | engine | 16 | 31 | 6.1 | loss supply 9, win purge 5, loss deck_empty 2 |

Card economy, per game (means over the cell): cards kept / played (tier 2 dealt); powers used / tags sacrificed; buildings replaced; tokens gained / spent; presence placed; gate.

| variant | d | agent | kept / played | tier-2 dealt | powers / tags | replaced | tokens + / - | presence | gate |
|---|---|---|---|---|---|---|---|---|---|
| c2 | 0 | random | 33.5 / 13.2 | 25.4 | 1.8 / 6.6 | 2.2 | 4.2 / 1.8 | 11.7 | – |
| c2 | 0 | greedy | 54.2 / 16.4 | 13.4 | 0.1 / 0.2 | 0.1 | 1.5 / 1.2 | 19.8 | pass |
| c2 | 0 | engine | 53.7 / 31.4 | 5.4 | 1.8 / 5.2 | 1.2 | 3.7 / 2.4 | 18.8 | pass |
| c2 | 1 | random | 30.5 / 11.0 | 21.4 | 1.5 / 5.6 | 2.4 | 2.2 / 0.9 | 9.6 | – |
| c2 | 1 | greedy | 55.6 / 17.9 | 12.8 | 0.0 / 0.0 | 0.2 | 1.9 / 1.4 | 20.1 | pass |
| c2 | 1 | engine | 55.2 / 32.4 | 4.8 | 1.2 / 3.8 | 1.0 | 3.7 / 2.6 | 18.4 | pass |
| c2 | 2 | random | 27.0 / 9.7 | 18.8 | 1.4 / 4.6 | 1.9 | 1.4 / 0.6 | 8.7 | – |
| c2 | 2 | greedy | 51.0 / 17.3 | 8.4 | 0.1 / 0.2 | 0.2 | 1.7 / 1.2 | 18.6 | pass |
| c2 | 2 | engine | 53.6 / 31.2 | 3.2 | 1.4 / 4.1 | 0.6 | 2.1 / 1.1 | 20.3 | pass |
| c2 | 3 | random | 22.8 / 9.4 | 14.3 | 1.3 / 4.5 | 2.2 | 1.2 / 0.4 | 6.8 | – |
| c2 | 3 | greedy | 56.0 / 19.4 | 10.8 | 0.3 / 0.9 | 0.1 | 2.2 / 1.6 | 19.8 | pass |
| c2 | 3 | engine | 53.7 / 31.3 | 4.5 | 1.2 / 3.8 | 0.6 | 2.2 / 1.8 | 20.0 | pass |
| c2 | 4 | random | 18.2 / 8.2 | 9.8 | 0.9 / 3.2 | 1.8 | 1.4 / 0.4 | 4.3 | – |
| c2 | 4 | greedy | 39.1 / 12.7 | 1.7 | 0.0 / 0.0 | 0.0 | 0.7 / 0.2 | 15.1 | pass |
| c2 | 4 | engine | 36.1 / 20.5 | 0.4 | 1.1 / 3.2 | 0.4 | 1.5 / 0.9 | 14.2 | pass |
| c3 | 0 | random | 52.0 / 18.4 | 39.4 | 1.6 / 5.8 | 2.1 | 7.5 / 4.3 | 17.7 | – |
| c3 | 0 | greedy | 63.0 / 23.9 | 17.9 | 0.2 / 0.8 | 0.1 | 5.8 / 4.2 | 30.9 | pass |
| c3 | 0 | engine | 63.6 / 39.9 | 11.3 | 2.2 / 6.7 | 0.8 | 5.1 / 3.8 | 24.6 | pass |
| c3 | 1 | random | 47.4 / 16.3 | 34.1 | 1.9 / 6.9 | 2.8 | 5.5 / 2.8 | 15.0 | – |
| c3 | 1 | greedy | 64.9 / 26.8 | 17.8 | 0.1 / 0.4 | 0.2 | 6.4 / 4.8 | 35.1 | pass |
| c3 | 1 | engine | 68.9 / 41.7 | 15.3 | 2.4 / 7.3 | 0.6 | 6.2 / 4.4 | 26.1 | pass |
| c3 | 2 | random | 41.2 / 15.7 | 26.1 | 1.6 / 6.0 | 2.6 | 5.1 / 2.9 | 11.5 | – |
| c3 | 2 | greedy | 68.0 / 29.6 | 18.8 | 0.5 / 1.5 | 0.2 | 7.9 / 6.5 | 37.8 | pass |
| c3 | 2 | engine | 72.1 / 45.1 | 16.7 | 1.6 / 4.9 | 0.4 | 5.9 / 4.1 | 28.1 | pass |
| c3 | 3 | random | 32.4 / 12.3 | 19.4 | 1.4 / 4.8 | 1.9 | 3.1 / 1.0 | 9.2 | – |
| c3 | 3 | greedy | 61.6 / 24.9 | 16.6 | 0.2 / 0.6 | 0.2 | 5.1 / 3.8 | 30.0 | pass |
| c3 | 3 | engine | 68.3 / 43.1 | 15.1 | 1.9 / 5.8 | 0.5 | 4.8 / 3.8 | 27.1 | pass |
| c3 | 4 | random | 26.2 / 10.9 | 13.1 | 1.1 / 3.6 | 1.9 | 1.8 / 0.3 | 6.8 | – |
| c3 | 4 | greedy | 51.5 / 18.0 | 11.2 | 0.2 / 0.6 | 0.1 | 3.4 / 2.1 | 20.2 | pass |
| c3 | 4 | engine | 57.1 / 35.9 | 8.6 | 1.9 / 5.7 | 0.4 | 3.8 / 2.6 | 22.5 | pass |

By round (rounds 1-6; both seats summed; means over the games that reached the round): hand size after keeping, cards kept, cards played, insight / credits left unspent at upkeep, powers used. Engine and greedy at d0, 2 countries.

| agent | measure | r1 | r2 | r3 | r4 | r5 | r6 |
|---|---|---|---|---|---|---|---|
| greedy | hand size | 17.3 | 21.1 | 26.7 | 32.6 | 38.9 | 43.2 |
| greedy | kept | 11.3 | 8.0 | 7.9 | 8.0 | 7.8 | 6.5 |
| greedy | played | 4.2 | 2.1 | 2.0 | 1.5 | 1.9 | 1.4 |
| greedy | insight unspent | 7.7 | 0.2 | 1.3 | 2.6 | 1.5 | 0.0 |
| greedy | credits unspent | 2.1 | 2.4 | 2.6 | 3.3 | 3.4 | 3.4 |
| greedy | powers | 0.0 | 0.0 | 0.0 | 0.0 | 0.1 | 0.0 |
| engine | hand size | 14.0 | 12.9 | 16.8 | 20.2 | 25.0 | 27.1 |
| engine | kept | 8.0 | 8.0 | 8.0 | 7.8 | 8.0 | 8.0 |
| engine | played | 9.1 | 3.5 | 3.7 | 3.1 | 4.9 | 4.2 |
| engine | insight unspent | 7.7 | 0.1 | 1.2 | 2.1 | 3.5 | 2.3 |
| engine | credits unspent | 3.6 | 5.2 | 4.8 | 9.5 | 9.3 | 8.4 |
| engine | powers | 0.1 | 0.7 | 0.2 | 0.1 | 0.4 | 0.1 |


## The untuned run (r12 untuned), for the card measures

Card economy, per game (means over the cell): cards kept / played (tier 2 dealt); powers used / tags sacrificed; buildings replaced; tokens gained / spent; presence placed; gate.

| variant | d | agent | kept / played | tier-2 dealt | powers / tags | replaced | tokens + / - | presence | gate |
|---|---|---|---|---|---|---|---|---|---|
| c2 | 0 | random | 34.0 / 14.3 | 24.4 | 1.8 / 6.2 | 2.8 | 2.6 / 0.8 | 10.6 | – |
| c2 | 0 | greedy | 58.5 / 20.8 | 17.6 | 0.2 / 0.6 | 0.4 | 4.8 / 3.8 | 18.8 | pass |
| c2 | 0 | engine | 55.2 / 32.1 | 9.8 | 1.6 / 4.9 | 1.5 | 4.9 / 3.9 | 15.1 | pass |
| c2 | 1 | random | 27.4 / 9.7 | 19.6 | 1.6 / 5.5 | 1.6 | 1.5 / 0.8 | 8.1 | – |
| c2 | 1 | greedy | 62.2 / 24.2 | 16.7 | 0.1 / 0.2 | 0.4 | 5.4 / 4.7 | 20.7 | pass |
| c2 | 1 | engine | 60.2 / 36.9 | 8.8 | 1.8 / 5.6 | 1.0 | 5.2 / 4.1 | 17.0 | pass |
| c2 | 2 | random | 26.6 / 10.1 | 18.1 | 1.6 / 5.6 | 1.8 | 1.5 / 0.6 | 7.7 | – |
| c2 | 2 | greedy | 60.5 / 24.2 | 14.6 | 0.2 / 0.6 | 0.4 | 4.8 / 3.6 | 20.8 | pass |
| c2 | 2 | engine | 53.2 / 32.4 | 5.9 | 1.2 / 3.8 | 1.1 | 4.3 / 3.3 | 16.6 | pass |
| c2 | 3 | random | 20.1 / 8.7 | 14.1 | 1.6 / 6.1 | 2.3 | 1.0 / 0.2 | 5.1 | – |
| c2 | 3 | greedy | 52.9 / 20.5 | 10.1 | 0.1 / 0.2 | 0.4 | 3.6 / 2.6 | 16.9 | pass |
| c2 | 3 | engine | 46.2 / 28.0 | 2.6 | 1.2 / 3.7 | 1.1 | 3.5 / 2.6 | 15.1 | pass |
| c2 | 4 | random | 15.2 / 6.9 | 7.6 | 1.2 / 4.8 | 1.9 | 0.4 / 0.1 | 3.9 | – |
| c2 | 4 | greedy | 35.6 / 12.9 | 0.6 | 0.0 / 0.0 | 0.4 | 2.1 / 1.2 | 11.2 | pass |
| c2 | 4 | engine | 30.6 / 18.4 | 0.1 | 0.7 / 2.1 | 0.8 | 2.2 / 1.6 | 11.9 | pass |
| c3 | 0 | random | 49.8 / 18.9 | 37.2 | 1.9 / 7.2 | 3.0 | 5.1 / 2.4 | 16.9 | – |
| c3 | 0 | greedy | 68.2 / 32.8 | 21.8 | 0.4 / 1.3 | 0.0 | 10.9 / 8.8 | 32.7 | pass |
| c3 | 0 | engine | 73.6 / 48.6 | 20.3 | 2.1 / 6.4 | 1.2 | 10.1 / 7.8 | 23.8 | pass |
| c3 | 1 | random | 47.9 / 16.1 | 35.6 | 1.5 / 5.9 | 3.1 | 4.5 / 2.4 | 15.4 | – |
| c3 | 1 | greedy | 66.4 / 30.0 | 19.9 | 0.7 / 2.1 | 0.0 | 9.7 / 7.9 | 30.3 | pass |
| c3 | 1 | engine | 83.9 / 57.6 | 24.4 | 2.5 / 7.6 | 0.6 | 10.4 / 8.0 | 28.6 | pass |
| c3 | 2 | random | 39.2 / 14.1 | 26.6 | 1.4 / 5.4 | 2.1 | 3.9 / 1.8 | 11.7 | – |
| c3 | 2 | greedy | 64.6 / 28.9 | 18.9 | 0.4 / 1.1 | 0.0 | 8.1 / 6.5 | 28.6 | pass |
| c3 | 2 | engine | 73.1 / 46.8 | 20.9 | 2.1 / 6.4 | 0.4 | 7.6 / 5.9 | 27.7 | pass |
| c3 | 3 | random | 32.9 / 12.3 | 19.6 | 1.2 / 4.2 | 2.2 | 1.8 / 0.5 | 7.8 | – |
| c3 | 3 | greedy | 63.4 / 28.9 | 17.8 | 0.5 / 1.5 | 0.0 | 8.2 / 6.1 | 26.9 | pass |
| c3 | 3 | engine | 66.8 / 43.8 | 17.9 | 1.8 / 5.6 | 0.4 | 5.9 / 4.6 | 24.1 | pass |
| c3 | 4 | random | 26.8 / 11.2 | 12.8 | 1.2 / 4.3 | 2.1 | 1.2 / 0.1 | 5.9 | – |
| c3 | 4 | greedy | 48.4 / 18.9 | 9.2 | 0.2 / 0.6 | 0.1 | 4.5 / 2.5 | 17.0 | pass |
| c3 | 4 | engine | 44.2 / 27.9 | 4.2 | 0.9 / 2.9 | 0.3 | 4.1 / 2.6 | 16.9 | pass |

By round (rounds 1-6; both seats summed; means over the games that reached the round): hand size after keeping, cards kept, cards played, insight / credits left unspent at upkeep, powers used. Engine and greedy at d0, 2 countries.

| agent | measure | r1 | r2 | r3 | r4 | r5 | r6 |
|---|---|---|---|---|---|---|---|
| greedy | hand size | 17.6 | 19.3 | 23.9 | 29.6 | 35.4 | 38.2 |
| greedy | kept | 11.6 | 6.3 | 7.3 | 7.4 | 7.5 | 5.6 |
| greedy | played | 4.6 | 2.6 | 1.8 | 1.6 | 2.6 | 1.8 |
| greedy | insight unspent | 5.7 | 0.1 | 0.1 | 0.2 | 0.9 | 0.0 |
| greedy | credits unspent | 3.3 | 2.1 | 3.2 | 3.2 | 4.0 | 2.7 |
| greedy | powers | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| engine | hand size | 14.0 | 10.8 | 14.1 | 18.9 | 23.1 | 25.3 |
| engine | kept | 8.0 | 6.1 | 6.9 | 7.7 | 8.0 | 7.1 |
| engine | played | 9.2 | 3.2 | 2.5 | 3.3 | 4.7 | 3.5 |
| engine | insight unspent | 5.7 | 0.1 | 0.2 | 0.3 | 1.3 | 0.3 |
| engine | credits unspent | 3.8 | 4.1 | 5.4 | 8.2 | 8.8 | 10.4 |
| engine | powers | 0.2 | 0.2 | 0.2 | 0.1 | 0.3 | 0.1 |

