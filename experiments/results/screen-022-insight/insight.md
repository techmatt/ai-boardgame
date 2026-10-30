# Insight costs, keeping by plan and the dominance checks (022, rules r13)

`aic-sim experiment experiments/screen-022-cards.json --quick --out out/screen-022-cards`
and `... experiments/screen-022-dominance.json --quick --out out/screen-022-dominance`
(run side by side on 12 threads: ~6 and ~8.5 min), then `python
scripts/cards/screen_022.py out/screen-022-cards out/screen-022-dominance`. 16 seeds a
cell (noise about ±12 points on a win rate, ±15-18 on a gap). `cards/` and
`dominance/` hold each run's report, results and tables. The card-by-card pricing
reasoning is in [pricing.md](pricing.md).

## Verdict

- **Engine at d0:** 94% at 2 and 3 countries (target ~90%): met.
- **Dominance gate: not met.** The checks lose clearly where play needs a decision the
  check refuses (keep-cheapest-k, no-presence, no-counters: 25-94 points behind at every
  difficulty below 4). **keep-all still matches the engine at 2 countries, d2** (here
  81% against 62%; 69% against 62% on 32 other seeds, 100-131), though it loses at 3
  countries (25% against 50%). **no-powers** is level with the engine (AI-board powers
  are used about once a game). **At d4 nothing separates**: the engine wins 25% (c2) and
  6% (c3), so every check sits at the floor with it; the d4 half of the gate can't
  pass until d4 is winnable.
- **Structural causes**, not fixable by one number: (1) *keep-all*: with base insight 4
  (+1 on the presence track) a seat can keep all four drafted cards and still afford the
  0-insight plays, and the engine's own credit use (ambassadors, 6 credits each, ~29 a
  game at c2 d2) is no better than turning spare credits into keeps at 5:1. Pricing the
  removal events at 1 insight (below) moved keep-all from 91% to 69% at c2 d2 (32
  seeds); going further needs either dearer keeps (Matt's 1 / 3 are fixed) or more
  insight-priced plays, which cost the engine its d0 rate. (2) *no-powers*: powers cost
  sacrificed tags and pay little; they are rarely worth it, so never using them costs
  little. (3) *d4*: too hard for any shallow agent at r13 (supply losses by round 5-6).

## Adaptations (each a candidate)

| Change | Why |
|---|---|
| Base insight 4 a board (was 3) | with plays paying insight, 3 cut the engine to 75% at c2 d0 and 19% at c3 d2 |
| Removal events (Precision Strike, Sweep, Strike Team) at 1 insight, not an event's 0 | at 0 keep-all beat the engine 91% to 59% (c2 d2): the cards that win were free once kept |
| Agents: `w.node` 25 (020's D9, Matt's call) | accepted; it moves results within noise at d2 (c3 d2 engine 25% → 41% with 6, 32 seeds, on the first cut) |
| Tried, not kept: `cards.insight_credits` 3 or 2 | no effect on the engine (75% at c2 d0 either way) |
| Tried, not kept: the keep plan holding 2 of each later round's insight back for that round's keeps | worse (58% at c2 d0): agents need the cards |
| Tried, not kept: `w.ambassador` 3 (stop the engine buying ambassadors) | worse (44% / 25% at c2 / c3 d2): ambassadors matter |
| Probe after the commit-ready run: `cards.insight_credits` 10 against keep-all (c2 d2, seeds 100-131) | keep-all 69% at 5 and at 10, the engine 62% / 59%: buying keep insight with credits is not keep-all's edge, so the price that would make it a mistake is a keep's insight (Matt's 1 / 3) or the insight the strong plays cost |
| Tried, not kept: `plan_rounds` 5 | level within noise (72 / 62% against 59 / 69%) |

## Win rates: before and after

r12 is 021's committed run; "first cut" is r13 with base insight 3 and seven events and
small programs at 1 (the removal events, Security Patch, Rally Point, Shore Up, Rally); "events 0" has every event at 0 and base insight 4; "tuned" is the committed
baseline (removal events 1, base insight 4).

| countries | d | greedy: r12 / r13 tuned | engine: r12 / r13 first cut / r13 events 0 / r13 tuned |
|---|---|---|---|
| 2 | 0 | 88 / 75 | 100 / 75 / 94 / 94 |
| 2 | 1 | 81 / 44 | 100 / – / 100 / 69 |
| 2 | 2 | 88 / 31 | 94 / 50 / 50 / 62 |
| 2 | 3 | 75 / 25 | 75 / – / 38 / 44 |
| 2 | 4 | 19 / 12 | 19 / 0 / 19 / 25 |
| 3 | 0 | 69 / 25 | 88 / 81 / 94 / 94 |
| 3 | 1 | 50 / 19 | 94 / – / 75 / 62 |
| 3 | 2 | 31 / 38 | 81 / 19 / 50 / 50 |
| 3 | 3 | 31 / 25 | 75 / – / 31 / 44 |
| 3 | 4 | 6 / 12 | 31 / 0 / 6 / 6 |

## The tuned run


| variant | d | agent | games | win % | rounds | ends |
|---|---|---|---|---|---|---|
| c2 | 0 | engine | 16 | 94 | 6.9 | win purge 15, loss deck_empty 1 |
| c2 | 0 | greedy | 16 | 75 | 8.1 | win purge 12, loss deck_empty 4 |
| c2 | 0 | random | 16 | 0 | 8.2 | loss supply 10, loss cities 4, loss deck_empty 2 |
| c2 | 1 | engine | 16 | 69 | 9.3 | win purge 11, loss deck_empty 5 |
| c2 | 1 | greedy | 16 | 44 | 9.0 | win purge 7, loss deck_empty 6, loss supply 3 |
| c2 | 1 | random | 16 | 0 | 6.8 | loss supply 13, loss cities 3 |
| c2 | 2 | engine | 16 | 62 | 8.0 | win purge 10, loss deck_empty 3, loss supply 3 |
| c2 | 2 | greedy | 16 | 31 | 8.6 | loss supply 6, loss deck_empty 5, win purge 5 |
| c2 | 2 | random | 16 | 0 | 5.8 | loss supply 14, loss cities 2 |
| c2 | 3 | engine | 16 | 44 | 8.4 | win purge 7, loss deck_empty 5, loss supply 4 |
| c2 | 3 | greedy | 16 | 25 | 7.4 | loss supply 10, win purge 4, loss deck_empty 2 |
| c2 | 3 | random | 16 | 0 | 5.1 | loss supply 16 |
| c2 | 4 | engine | 16 | 25 | 5.9 | loss supply 12, win purge 4 |
| c2 | 4 | greedy | 16 | 12 | 5.4 | loss supply 14, win purge 2 |
| c2 | 4 | random | 16 | 0 | 3.9 | loss supply 16 |
| c3 | 0 | engine | 16 | 94 | 6.1 | win purge 15, loss deck_empty 1 |
| c3 | 0 | greedy | 16 | 25 | 10.7 | loss deck_empty 12, win purge 4 |
| c3 | 0 | random | 16 | 0 | 7.9 | loss supply 13, loss cities 2, loss deck_empty 1 |
| c3 | 1 | engine | 16 | 62 | 8.9 | win purge 10, loss deck_empty 6 |
| c3 | 1 | greedy | 16 | 19 | 10.1 | loss deck_empty 11, win purge 3, loss supply 2 |
| c3 | 1 | random | 16 | 0 | 6.7 | loss supply 15, loss cities 1 |
| c3 | 2 | engine | 16 | 50 | 8.8 | loss deck_empty 8, win purge 8 |
| c3 | 2 | greedy | 16 | 38 | 8.6 | loss deck_empty 7, win purge 6, loss supply 3 |
| c3 | 2 | random | 16 | 0 | 5.9 | loss supply 15, loss deck_empty 1 |
| c3 | 3 | engine | 16 | 44 | 9.0 | win purge 7, loss deck_empty 7, loss supply 2 |
| c3 | 3 | greedy | 16 | 25 | 7.3 | loss supply 9, win purge 4, loss deck_empty 3 |
| c3 | 3 | random | 16 | 0 | 5.1 | loss supply 16 |
| c3 | 4 | engine | 16 | 6 | 5.4 | loss supply 15, win purge 1 |
| c3 | 4 | greedy | 16 | 12 | 5.4 | loss supply 14, win purge 2 |
| c3 | 4 | random | 16 | 0 | 3.8 | loss supply 16 |

## Card economy per game (credits unspent: summed over rounds, both seats)

| variant | d | agent | kept / played | insight: keeps / plays | credits unspent | presence |
|---|---|---|---|---|---|---|
| c2 | 0 | engine | 49.9 / 29.1 | 40.8 / 16.3 | 53.6 | 19.1 |
| c2 | 0 | greedy | 44.9 / 14.0 | 35.9 / 11.1 | 25.6 | 21.2 |
| c2 | 0 | random | 34.4 / 9.9 | 38.4 / 5.4 | 277.6 | 11.6 |
| c2 | 1 | engine | 61.1 / 38.1 | 53.4 / 22.3 | 82.0 | 24.9 |
| c2 | 1 | greedy | 47.8 / 17.2 | 38.8 / 14.0 | 29.9 | 23.4 |
| c2 | 1 | random | 29.6 / 8.1 | 31.1 / 4.7 | 206.2 | 8.7 |
| c2 | 2 | engine | 54.6 / 31.8 | 43.3 / 16.8 | 66.6 | 21.8 |
| c2 | 2 | greedy | 47.3 / 17.4 | 37.6 / 13.7 | 29.3 | 22.6 |
| c2 | 2 | random | 26.8 / 8.1 | 26.6 / 4.8 | 158.3 | 6.4 |
| c2 | 3 | engine | 56.1 / 33.6 | 44.9 / 18.1 | 68.7 | 23.2 |
| c2 | 3 | greedy | 43.0 / 14.6 | 31.7 / 10.0 | 21.9 | 18.7 |
| c2 | 3 | random | 22.0 / 6.6 | 18.9 / 3.6 | 119.1 | 6.0 |
| c2 | 4 | engine | 40.4 / 22.2 | 28.1 / 11.9 | 40.9 | 15.3 |
| c2 | 4 | greedy | 35.0 / 10.8 | 21.8 / 6.1 | 16.9 | 14.2 |
| c2 | 4 | random | 17.6 / 5.1 | 12.2 / 2.5 | 77.7 | 3.4 |
| c3 | 0 | engine | 61.3 / 37.9 | 49.2 / 25.0 | 50.8 | 25.3 |
| c3 | 0 | greedy | 59.9 / 26.9 | 50.0 / 26.9 | 50.6 | 38.6 |
| c3 | 0 | random | 47.8 / 13.1 | 49.6 / 8.9 | 404.3 | 14.6 |
| c3 | 1 | engine | 77.9 / 51.4 | 69.2 / 34.4 | 111.0 | 32.4 |
| c3 | 1 | greedy | 59.4 / 26.4 | 49.2 / 25.8 | 47.1 | 35.4 |
| c3 | 1 | random | 44.4 / 12.9 | 44.4 / 8.2 | 265.1 | 12.1 |
| c3 | 2 | engine | 76.5 / 50.5 | 68.2 / 35.1 | 113.1 | 36.1 |
| c3 | 2 | greedy | 58.6 / 25.8 | 47.3 / 24.5 | 41.5 | 32.2 |
| c3 | 2 | random | 39.6 / 11.9 | 37.6 / 7.1 | 223.6 | 10.8 |
| c3 | 3 | engine | 76.9 / 49.2 | 66.9 / 33.6 | 112.1 | 36.0 |
| c3 | 3 | greedy | 53.5 / 21.2 | 39.6 / 18.2 | 30.1 | 26.7 |
| c3 | 3 | random | 32.9 / 10.3 | 25.1 / 5.7 | 155.6 | 8.2 |
| c3 | 4 | engine | 51.0 / 30.4 | 33.8 / 16.9 | 52.4 | 19.8 |
| c3 | 4 | greedy | 45.8 / 16.2 | 28.7 / 10.6 | 20.9 | 19.2 |
| c3 | 4 | random | 25.7 / 8.7 | 15.6 / 4.9 | 101.8 | 6.4 |

## By round (c2, both seats summed: per seat, halve it)

| agent | d | measure | r1 | r2 | r3 | r4 | r5 | r6 |
|---|---|---|---|---|---|---|---|---|
| engine | 0 | hand size | 19.3 | 17.1 | 20.2 | 23.2 | 26.4 | 27.1 |
| engine | 0 | kept | 13.3 | 7.2 | 7.1 | 6.8 | 6.5 | 6.0 |
| engine | 0 | played | 9.2 | 3.7 | 3.1 | 3.2 | 4.5 | 4.4 |
| engine | 0 | insight on keeps | 0.0 | 7.2 | 7.1 | 6.8 | 6.7 | 7.4 |
| engine | 0 | insight on plays | 5.5 | 2.1 | 1.6 | 1.3 | 2.0 | 1.6 |
| engine | 0 | credits unspent | 2.1 | 5.1 | 6.2 | 8.2 | 9.2 | 11.9 |
| engine | 2 | hand size | 19.3 | 18.4 | 20.7 | 24.2 | 25.9 | 26.6 |
| engine | 2 | kept | 13.3 | 7.2 | 6.5 | 6.5 | 6.0 | 5.9 |
| engine | 2 | played | 7.8 | 3.7 | 2.8 | 3.4 | 3.8 | 3.7 |
| engine | 2 | insight on keeps | 0.0 | 7.2 | 6.5 | 6.5 | 6.0 | 6.3 |
| engine | 2 | insight on plays | 4.3 | 1.8 | 1.2 | 1.6 | 1.7 | 1.9 |
| engine | 2 | credits unspent | 2.9 | 7.1 | 7.8 | 12.6 | 9.5 | 7.7 |
| greedy | 0 | hand size | 19.3 | 21.4 | 26.4 | 30.9 | 35.2 | 37.6 |
| greedy | 0 | kept | 13.3 | 6.4 | 6.6 | 5.7 | 5.1 | 4.1 |
| greedy | 0 | played | 4.4 | 1.6 | 1.1 | 0.9 | 1.6 | 1.8 |
| greedy | 0 | insight on keeps | 0.0 | 6.4 | 6.6 | 5.7 | 5.3 | 5.3 |
| greedy | 0 | insight on plays | 1.8 | 1.2 | 0.8 | 0.4 | 1.3 | 1.9 |
| greedy | 0 | credits unspent | 2.9 | 2.2 | 4.1 | 4.0 | 4.2 | 3.1 |
| greedy | 2 | hand size | 19.3 | 21.6 | 26.6 | 31.7 | 34.5 | 36.0 |
| greedy | 2 | kept | 13.3 | 6.7 | 6.5 | 6.3 | 4.2 | 4.4 |
| greedy | 2 | played | 4.4 | 1.4 | 1.2 | 1.3 | 2.2 | 2.1 |
| greedy | 2 | insight on keeps | 0.0 | 6.7 | 6.5 | 6.3 | 4.6 | 5.1 |
| greedy | 2 | insight on plays | 1.8 | 1.1 | 0.9 | 0.8 | 1.6 | 1.9 |
| greedy | 2 | credits unspent | 3.2 | 4.8 | 5.0 | 3.1 | 3.1 | 4.1 |

## Dominance checks: card economy per game

| variant | d | agent | kept / played | insight: keeps / plays | credits unspent | presence |
|---|---|---|---|---|---|---|
| c2 | 0 | engine | 49.9 / 29.1 | 40.8 / 16.3 | 53.6 | 19.1 |
| c2 | 0 | keep-all | 61.9 / 31.1 | 54.7 / 13.7 | 63.4 | 19.2 |
| c2 | 0 | keep-cheapest-k | 41.0 / 28.2 | 36.9 / 9.9 | 53.8 | 27.6 |
| c2 | 0 | no-counters | 57.1 / 32.6 | 47.3 / 17.6 | 73.0 | 21.6 |
| c2 | 0 | no-powers | 53.6 / 31.6 | 45.5 / 17.4 | 57.7 | 20.2 |
| c2 | 0 | no-presence | 57.4 / 33.8 | 47.9 / 15.9 | 81.7 | 0.0 |
| c2 | 0 | tier-1-only | 51.4 / 30.5 | 42.4 / 17.1 | 62.8 | 19.8 |
| c2 | 0 | tier-2-rush | 21.2 / 15.2 | 15.8 / 14.9 | 20.1 | 14.9 |
| c2 | 2 | engine | 54.6 / 31.8 | 43.3 / 16.8 | 66.6 | 21.8 |
| c2 | 2 | keep-all | 63.9 / 32.2 | 55.8 / 13.1 | 63.4 | 21.6 |
| c2 | 2 | keep-cheapest-k | 30.0 / 19.7 | 26.0 / 6.2 | 39.0 | 21.5 |
| c2 | 2 | no-counters | 64.6 / 33.7 | 52.8 / 16.0 | 108.5 | 27.7 |
| c2 | 2 | no-powers | 55.3 / 32.1 | 45.2 / 18.0 | 73.6 | 22.9 |
| c2 | 2 | no-presence | 46.8 / 25.2 | 34.2 / 10.5 | 59.6 | 0.0 |
| c2 | 2 | tier-1-only | 56.5 / 33.1 | 45.9 / 17.8 | 80.1 | 23.9 |
| c2 | 2 | tier-2-rush | 25.1 / 15.8 | 22.0 / 16.9 | 58.4 | 20.3 |
| c2 | 4 | engine | 40.4 / 22.2 | 28.1 / 11.9 | 40.9 | 15.3 |
| c2 | 4 | keep-all | 48.7 / 23.2 | 34.4 / 9.3 | 44.4 | 15.4 |
| c2 | 4 | keep-cheapest-k | 16.2 / 13.1 | 12.2 / 4.2 | 15.6 | 11.6 |
| c2 | 4 | no-counters | 35.6 / 16.9 | 22.2 / 7.6 | 46.9 | 15.6 |
| c2 | 4 | no-powers | 39.1 / 21.5 | 26.1 / 11.0 | 39.8 | 14.8 |
| c2 | 4 | no-presence | 32.2 / 15.9 | 18.9 / 7.8 | 30.9 | 0.0 |
| c2 | 4 | tier-1-only | 40.2 / 21.6 | 27.9 / 11.3 | 43.4 | 15.4 |
| c2 | 4 | tier-2-rush | 17.6 / 12.4 | 9.2 / 10.6 | 22.6 | 11.3 |
| c3 | 0 | engine | 61.3 / 37.9 | 49.2 / 25.0 | 50.8 | 25.3 |
| c3 | 0 | keep-all | 88.9 / 54.4 | 78.3 / 32.4 | 183.2 | 34.4 |
| c3 | 0 | keep-cheapest-k | 66.0 / 47.1 | 60.1 / 19.4 | 90.9 | 46.0 |
| c3 | 0 | no-counters | 75.6 / 47.3 | 67.8 / 32.8 | 122.5 | 31.9 |
| c3 | 0 | no-powers | 63.6 / 39.7 | 52.9 / 26.2 | 70.2 | 25.6 |
| c3 | 0 | no-presence | 74.9 / 50.8 | 63.2 / 24.2 | 115.9 | 0.0 |
| c3 | 0 | tier-1-only | 62.8 / 38.8 | 51.2 / 25.4 | 52.6 | 25.6 |
| c3 | 0 | tier-2-rush | 55.3 / 33.9 | 49.6 / 31.6 | 74.9 | 33.3 |
| c3 | 2 | engine | 76.5 / 50.5 | 68.2 / 35.1 | 113.1 | 36.1 |
| c3 | 2 | keep-all | 89.1 / 50.9 | 78.6 / 29.3 | 148.4 | 36.7 |
| c3 | 2 | keep-cheapest-k | 47.6 / 32.2 | 41.6 / 12.1 | 63.9 | 32.2 |
| c3 | 2 | no-counters | 78.5 / 48.0 | 68.0 / 31.9 | 139.2 | 37.5 |
| c3 | 2 | no-powers | 75.0 / 49.6 | 65.9 / 34.4 | 110.5 | 35.4 |
| c3 | 2 | no-presence | 64.4 / 41.5 | 50.9 / 19.8 | 95.6 | 0.0 |
| c3 | 2 | tier-1-only | 78.9 / 53.5 | 70.7 / 37.7 | 124.4 | 38.4 |
| c3 | 2 | tier-2-rush | 48.4 / 28.4 | 40.4 / 26.7 | 81.7 | 32.8 |
| c3 | 4 | engine | 51.0 / 30.4 | 33.8 / 16.9 | 52.4 | 19.8 |
| c3 | 4 | keep-all | 54.1 / 19.4 | 33.0 / 4.6 | 39.4 | 15.8 |
| c3 | 4 | keep-cheapest-k | 22.5 / 16.6 | 16.5 / 5.1 | 23.9 | 15.8 |
| c3 | 4 | no-counters | 48.1 / 26.2 | 30.4 / 14.0 | 59.9 | 19.9 |
| c3 | 4 | no-powers | 49.7 / 29.5 | 32.4 / 16.2 | 49.2 | 19.2 |
| c3 | 4 | no-presence | 44.8 / 24.6 | 25.9 / 12.3 | 42.1 | 0.0 |
| c3 | 4 | tier-1-only | 51.4 / 30.7 | 34.4 / 17.3 | 52.1 | 20.1 |
| c3 | 4 | tier-2-rush | 26.3 / 17.1 | 11.0 / 13.5 | 30.9 | 15.4 |

## Dominance table

Gap = engine win % minus the check's, ± one standard error of the difference.

| variant | d | engine | keep-all | keep-cheapest-k | tier-1-only | tier-2-rush | no-powers | no-counters | no-presence |
|---|---|---|---|---|---|---|---|---|---|
| c2 | 0 | 94 | +0 ±9 | +69 ±12 | +12 ±11 | -6 ±7 | +0 ±9 | +12 ±11 | +94 ±7 |
| c2 | 2 | 62 | -19 ±16 ✗ | +62 ±12 | +19 ±17 | +12 ±17 | +6 ±17 | +38 ±16 | +62 ±12 |
| c2 | 4 | 25 | +6 ±15 ~ | +25 ±11 | +6 ±15 ~ | +0 ±15 ✗ | +6 ±15 ~ | +25 ±11 | +25 ±11 |
| c3 | 0 | 94 | +56 ±14 | +94 ±7 | +0 ±9 | +25 ±13 | +12 ±11 | +25 ±13 | +94 ±7 |
| c3 | 2 | 50 | +25 ±17 | +44 ±14 | +19 ±17 | +25 ±17 | -6 ±18 ✗ | +25 ±17 | +50 ±13 |
| c3 | 4 | 6 | +0 ±9 ✗ | +6 ±7 ~ | +0 ±9 ✗ | +6 ±7 ~ | +6 ±7 ~ | +6 ±7 ~ | +6 ±7 ~ |

Gate: FAIL: keep-all c2 d2: gap -19; keep-all c2 d4: gap +6 within noise ±15; tier-1-only c2 d4: gap +6 within noise ±15; tier-2-rush c2 d4: gap +0; no-powers c2 d4: gap +6 within noise ±15; no-powers c3 d2: gap -6; keep-all c3 d4: gap +0; keep-cheapest-k c3 d4: gap +6 within noise ±7; tier-1-only c3 d4: gap +0; tier-2-rush c3 d4: gap +6 within noise ±7; no-powers c3 d4: gap +6 within noise ±7; no-counters c3 d4: gap +6 within noise ±7; no-presence c3 d4: gap +6 within noise ±7

Before (d2 and d4 rows; first cut, then events 0):

| run | variant | d | engine | keep-all | keep-cheapest-k | tier-1-only | tier-2-rush | no-powers | no-counters | no-presence |
|---|---|---|---|---|---|---|---|---|---|---|
| first cut | c2 | 2 | 50 | +25 ±17 | +44 ±14 | -12 ±17 ✗ | +12 ±17 | +12 ±17 | +38 ±15 | +50 ±13 |
| first cut | c2 | 4 | 0 | +0 ±4 ✗ | +0 ±4 ✗ | +0 ±4 ✗ | -12 ±9 ✗ | -6 ±7 ✗ | -6 ±7 ✗ | +0 ±4 ✗ |
| first cut | c3 | 2 | 19 | +6 ±13 | +19 ±10 | -19 ±16 ✗ | +0 ±14 ✗ | +0 ±14 ✗ | +19 ±10 | +19 ±10 |
| first cut | c3 | 4 | 0 | +0 ±4 ✗ | +0 ±4 ✗ | +0 ±4 ✗ | +0 ±4 ✗ | +0 ±4 ✗ | +0 ±4 ✗ | +0 ±4 ✗ |
| events 0 | c2 | 2 | 50 | -50 ±13 ✗ | +44 ±14 | -6 ±18 ✗ | +6 ±18 | -25 ±17 ✗ | +12 ±17 | +50 ±13 |
| events 0 | c2 | 4 | 19 | -12 ±15 ✗ | +19 ±10 | -6 ±15 ✗ | -6 ±15 ✗ | -19 ±16 ✗ | +19 ±10 | +19 ±10 |
| events 0 | c3 | 2 | 50 | +12 ±17 | +50 ±13 | -6 ±18 ✗ | +25 ±17 | -6 ±18 ✗ | +31 ±16 | +50 ±13 |
| events 0 | c3 | 4 | 6 | -6 ±10 ✗ | +6 ±7 ~ | +0 ±9 ✗ | +6 ±7 ~ | +0 ±9 ✗ | +6 ±7 ~ | +6 ±7 ~ |

## For a new player: insight is shared

Each round your AI board gives you insight (4, plus what your presence track has
uncovered), and whatever you don't spend is gone at the round's end. Both keeping and
playing draw on it. After the draft, each card you keep costs 1 insight (3 for a tier-2
card), and most cards cost insight again to play: small programs and events nothing,
the removal events and most cards 1, big tier-2 cards 2. So a round with 4 or 5 insight
is a choice: keep two cards and play two, or keep a tier-2 card and play one. Keep a card
only if you can play it in the next round or two, because a card that sits in your hand
cost you a play. When you run short while keeping, credits buy keep insight at 5 credits
each, but never insight to play.
