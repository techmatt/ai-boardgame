# A scarier adversary (024, rules r15)

`aic-sim experiment experiments/screen-024-cards.json --quick --out out/screen-024-cards`
(480 games) and `aic-sim experiment experiments/screen-024-dominance.json --quick --games
32 --out out/screen-024-dominance` (1,344 games), side by side on 12 threads (~5 and ~12
min), then `python scripts/cards/screen_024.py out/screen-024-cards out/screen-024-dominance
--before out/before-cards`. 16 seeds a cell on the screen (noise about ±12 points), 32 on
the gate (±8, ±10-12 on a gap). `cards/` holds the final screen (`summary.md`: the script's
full output, per-card rates included), `dominance/` the gate, `before/` the same screen on
the r14 binary.

## Verdict

| Done when | Result |
|---|---|
| No agent wins before round 6 at any difficulty | **Met.** Earliest win 6 in every greedy and engine cell (9 at c3 d4 greedy); random never wins. r14: round 2-3 at c2. |
| d0 engine about 85%+ | **Met.** 94% at 2 countries, 94% at 3 (r14: 75, 81). |
| ignore-Science, -Industry, -Cyber, -Security each lose to engine at d2 | **Partly met.** Security loses at both sizes (+31, +62). At 3 countries Cyber (+16) and Industry (+9) lose and Science is level (+3). **At 2 countries Science (−9), Industry (−22) and Cyber (−9) do not lose** (±10-11). r14: −12 / −16 / −28. |
| Unspent credits at game end under about 15 a player | **Met.** 2-5 a player for the engine, 1 for greedy, after the last action phase played (r14: 7-11, 2). Over a whole game (each round's leftover, summed): 13-42 (r14: 70-160). |

## Win rates, earliest win, credits

Credits left: per player, after the last action phase played.

| countries | d | random | greedy: r14 / r15 | engine: r14 / r15 | earliest win, greedy / engine: r14 → r15 | mean win round, engine | credits left, engine: r14 / r15 |
|---|---|---|---|---|---|---|---|
| 2 | 0 | 0 | 56 / 62 | 75 / 94 | 2 / 2 → 6 / 6 | 6.1 | 8 / 2 |
| 2 | 1 | 0 | 44 / 69 | 44 / 81 | 2 / 3 → 6 / 6 | 6.7 | 11 / 3 |
| 2 | 2 | 0 | 25 / 31 | 31 / 69 | 2 / 3 → 6 / 6 | 6.5 | 9 / 4 |
| 2 | 3 | 0 | 31 / 19 | 19 / 38 | 2 / 2 → 6 / 6 | 6.0 | 8 / 5 |
| 2 | 4 | 0 | 6 / 25 | 19 / 38 | 2 / 2 → 6 / 6 | 6.8 | 7 / 4 |
| 3 | 0 | 0 | 56 / 81 | 81 / 94 | 4 / 5 → 6 / 6 | 6.3 | 11 / 4 |
| 3 | 1 | 0 | 31 / 81 | 31 / 100 | 4 / 3 → 6 / 6 | 6.8 | 11 / 3 |
| 3 | 2 | 0 | 6 / 75 | 25 / 81 | 8 / 4 → 6 / 6 | 6.4 | 11 / 4 |
| 3 | 3 | 0 | 12 / 31 | 25 / 38 | 4 / 5 → 6 / 6 | 7.3 | 8 / 5 |
| 3 | 4 | 0 | 6 / 6 | 6 / 19 | 6 / 8 → 9 / 6 | 7.3 | 7 / 4 |

**Win rates went up, though the adversary is harder, because the agent got better:** the
keep plan was the biggest thing wrong in 023's numbers (below). Read r15's curve against
the game as it now plays, not as the same players against a harder game. 1, 2 and 3-4 are
real steps: d1 costs a little, d3's bigger build halves the engine, and the supply ends
over a third of d3-4 games.

## The dominance table (d2, 32 seeds)

Gap = engine win % minus the check's, points, ± one standard error. ✗: not below the engine.

| check | c2 (engine 69) | c3 (engine 81) | r14: c2 / c3 |
|---|---|---|---|
| ignore-Science | −9 ±11 ✗ | +3 ±10 | −12 / −12 |
| ignore-Industry | −22 ±10 ✗ | +9 ±11 | −16 / −3 |
| ignore-Cyber | −9 ±11 ✗ | +16 ±11 | −28 / not in 023's write-up |
| ignore-Power | +16 ±12 | +25 ±11 | 0 / +16 |
| ignore-Diplomacy | +53 ±10 | +75 ±8 | +28 / +25 |
| ignore-Security | +31 ±12 | +62 ±10 | −9 / +6 |
| mono-Science, Industry, Cyber, Power | +69 ±8 | +81 ±7 | +25 to +31 |
| mono-Diplomacy | +9 ±12 | +44 ±11 | +3 / about +25 |
| mono-Security | +50 ±11 | +66 ±9 | about +25 |
| keep-all | +69 ±8 | +81 ±7 | +31 / +19 |
| keep-cheapest-k | +25 ±12 | +41 ±11 | +12 / +25 |
| tier-1-only | +25 ±12 | +38 ±11 | within noise |
| tier-2-rush | −6 ±11 ✗ | +6 ±10 | within noise |
| no-powers | +0 ±12 ✗ | +6 ±10 | within noise |
| no-counters | +6 ±12 | +38 ±11 | within noise |
| no-presence | +69 ±8 | +81 ±7 | +31 / +25 |
| event-hoarder | +25 ±12 | +56 ±10 | +28 / +25 |

(r14 figures are from 023's write-up, which gave the mono-tag and some c3 gaps as ranges.)

**Why the gate is still open at 2 countries.** Two independent causes:

1. **Most of "ignore-X beats the engine" was the agent, not the rules, and that part is
   fixed.** The keep plan paid 5 credits a card for opening keeps (all 20 starting credits),
   kept tier-2 buildings it could never build, and kept cards with no target. Dropping a
   tag happened to stop some of that. Charging for those credits alone took c2 d2 from 37%
   to 81% (`plan_credit`); skipping unplayable buildings took it from 19% to 37%. 023's "the
   win is made of Diplomacy" was partly this.
2. **What remains is structural: the game is decided by about round 6.** `purge_round` 7 and 8
   give the same win rates as 6 (c2 71, c3 81, with the same ignore-X results); only the
   win round moves. Once the last corrupted city is freed the adversary has no ambassador
   source left, so the board stays clear and the win waits for the gate. Slow engines
   (Science's draw, Industry's income) can't pay back that fast. 2 countries is probably
   worse because there are only 2 corrupted cities to free at the start. A stronger build doesn't fix it:
   `city_build` 3 cut c3 d0 to 62% and left c2 ignore-Cyber and ignore-Science ahead.

Agent caution: the engine plays one action at a time. Science's value is mostly in later
rounds, and it's the tag with the fewest plays (about 1 a win); a deeper agent might value
it more. Not tested: `--quick` runs shallow agents only.

## Insight, keeps and plays

No insight generation was added. Insight still splits about 3:2 toward keeps.

| countries | d | agent | insight on keeps / plays: r14 | r15 | cards kept / played: r14 | r15 | credits left over a game (sum of rounds): r14 / r15 |
|---|---|---|---|---|---|---|---|
| 2 | 0 | greedy | 26 / 12 | 28 / 16 | 28 / 12 | 24 / 13 | 23 / 8 |
| 2 | 0 | engine | 32 / 18 | 28 / 19 | 33 / 19 | 25 / 17 | 70 / 13 |
| 2 | 2 | greedy | 26 / 12 | 36 / 19 | 29 / 13 | 30 / 16 | 24 / 4 |
| 2 | 2 | engine | 35 / 18 | 31 / 17 | 35 / 19 | 27 / 16 | 78 / 22 |
| 3 | 0 | greedy | 44 / 19 | 41 / 23 | 46 / 19 | 37 / 19 | 38 / 12 |
| 3 | 0 | engine | 62 / 32 | 44 / 29 | 58 / 33 | 39 / 26 | 135 / 27 |
| 3 | 2 | greedy | 45 / 20 | 46 / 24 | 46 / 19 | 40 / 20 | 38 / 7 |
| 3 | 2 | engine | 57 / 28 | 47 / 28 | 56 / 29 | 40 / 25 | 160 / 42 |

Games are shorter at r15 (wins about 6.4 rounds), so per-game counts fall with them. Kept
cards still outnumber played ones by half: about a third of the insight spent on keeps buys
a card that is never played.

## Standard projects and actions per game (engine)

| countries | d | ambassador | presence (energy) | presence (credits) | harden (Protector) | harden (credits) | drain | energy | remove | move | align | r14 ambassadors |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 0 | 3.4 | 2.2 | 10.2 | 4.7 | 0.0 | 5.3 | 0.0 | 14.8 | 2.2 | 3.4 | 17.6 |
| 2 | 2 | 1.3 | 2.9 | 10.4 | 7.9 | 0.0 | 4.9 | 0.1 | 23.9 | 2.2 | 2.6 | 16.7 |
| 2 | 4 | 0.8 | 3.3 | 11.8 | 3.6 | 0.1 | 3.1 | 0.0 | 26.0 | 2.8 | 0.3 | 5.4 |
| 3 | 0 | 5.1 | 2.5 | 16.1 | 5.8 | 0.0 | 6.5 | 0.0 | 27.4 | 3.0 | 4.4 | 34.2 |
| 3 | 2 | 4.8 | 3.1 | 16.2 | 3.0 | 0.3 | 5.5 | 0.1 | 40.6 | 3.4 | 2.8 | 33.6 |
| 3 | 4 | 3.9 | 4.3 | 18.1 | 2.7 | 0.0 | 3.2 | 0.0 | 39.6 | 3.7 | 0.6 | 14.5 |

Credits now go to counters (19-26 a game at d2) and buying presence back after troops
strike. Buying energy is never used: the agent doesn't value it at 3 credits. The harden
project is almost unused. The standard remove (1 energy, unchanged) does much of the work
cards were meant to do, 15-40 a game; `remove_energy` 2 dropped d0 to 62% / 44%, so it stayed.

## Cards

| | 023 | 024 |
|---|---|---|
| Dead cards | 45 | 44 |
| Must-picks | 26 | 14 |

Must-picks fell by almost half, though about half are still Diplomacy cards (8 of 14
against 12 of 26):
Breakthrough Leak, Bulk Silicon Procurement, Ceasefire Monitors, Compute Siphon, Consular
Pouch, Consulate Tower, Development Loan, Diplomatic Expulsion, Embassy Compound, Joint
Fabrication Pact, Logic Bomb, Mutual Aid Pact, Open Preprint, Shuttle Diplomacy. Compute
Siphon and Logic Bomb are new: 023's project hacks were dead. The dead list is still long,
and it's the slow half of the set: the built-over tier-2 buildings (Exabyte Archive,
Foundation Model Campus, Hyperscale Botnet, Black Ice Citadel, Dyson Swarm Tranche, Orbital
Mass Driver, Orbital Embassy, Planetary Assembly), Science's insight converters (Benchmark
Suite, Field Sensor Grid, Cryo-Compute Burn), the income-cut cards, and the token engines
that pay late. Full list and per-card rates in `cards/summary.md`.

## Tuning steps (engine, d0 and d2, 16 seeds unless noted)

| step | c2 d0 | c3 d0 | c2 d2 | c3 d2 |
|---|---|---|---|---|
| r15 first cut (`city_build` 2, compute 2, supply 18, credit counters, `free_city` 3) | 75 | 75 | 6 | 0 |
| + `purge_round` 6 (earliest win was 4) | 91 | 90 | 0 | 0 |
| + supply 24, `extra_threats` 1 | 86 | 77 | 36 | 0 |
| + `free_city_at` 1, `free_city` 4 | 92 | 92 | 54 | 50 |
| + keep plan holds back insight | 88 | 100 | 44 | 44 |
| + Infiltration Drive corrupts cities | 94 | 94 | 19 | 44 |
| + keep plan skips unplayable buildings | | | 37 | 56 |
| + keep plan charges credits (`plan_credit` 3) | | | 81 | 88 |
| + `node_compute` 3, `w.project` 30 (final, 32 seeds at d2) | 94 | 94 | 69 | 81 |
| tried: `free_city_at` 2 | | | 44 | 31 |
| tried: `extra_threats` 2 | | | 38 | 75 |
| tried: `remove_energy` 2 | 62 | 44 | 44 | 0 |
| tried: `purge_round` 7, 8 | | | 71 | 81 |
| tried: `city_build` 3 | 88 | 62 | 38 | 44 |
| tried: `corrupt_at` 5 | 94 | 94 | 56 | 75 |
| tried: `counter_bonus` +1 on every counter | | | 42 | 50 |

## The end conditions, for the rules review

**Purge.** Driven now by freeing: 3 cities a game in wins, cleared by about round 6, with
2 Unity tips switching off everything else. `purge_round` 6 is the only reason no win
comes earlier; wins average round 6.4 and bunch hard against it, and rounds 7 and 8 change
who wins not at all. That's the main thing broken: the purge is a lock-in, not a race.
With the corrupted cities gone the adversary makes no ambassadors, so a clear board stays
clear. A rule that lets the adversary come back after a free (nodes building ambassadors,
say) would make the last rounds a fight rather than a wait. `free_city_at` 1 is also lower
than the "X ambassadors" Matt had in mind; 2 is one config line but the one-step agents
can't line it up. Nodes are 4 on the board at a win and don't matter: Unity switches them off.

**Supply.** It decides d3 and d4: 55 of 128 games there, by round 6.5. Below d3 it barely
exists (1 of 64 at d2), since doubling the build came with 24 a country and the engine
removes 15-40 ambassadors a game for 1 energy each. So it's a cliff: at d3 half as many more
ambassadors a round turns the supply from background into the main way to lose. The loss is
also hard to see coming, and players have only the removal to answer it. Worth a look as a
gentler track.

**Cities (every city corrupted).** Rare: 13 of 320 games, around round 9, up from almost
never now Overrun and Infiltration Drive can corrupt directly. It reads as the right kind of
loss, a slide you can watch, but it's a side path. Corrupted cities at a loss average 4.1 of
6-9, so most deck losses are this slide stopped by the clock.

**Deck.** The clock ends 70 of 320 games, at 11-12 rounds, with 17 nodes, 8 troops and 4
corrupted cities on the board and 3.5 Unity tips made. These are games the team was never
going to win, played out; by round 6-7 the result is usually clear, and the last four
rounds aren't interesting. An earlier "the adversary has won" check, or a deck that speeds up
when the adversary is ahead, would cut those rounds. Unity's outright win (8 tips) came 7 times,
at rounds 8-11, 6 of them greedy's: `purge_round` doesn't gate it, which is fine at that speed but
worth a decision. Discord tips run at 6.7 a game, far above Unity's 4: the tug is one-sided.
