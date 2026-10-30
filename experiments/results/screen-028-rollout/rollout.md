# A rollout agent, then rules revision 17 (028)

`aic-sim experiment experiments/screen-028.json --quick --games 32` and
`experiments/screen-028-dominance.json` with the same flags, then
`python scripts/cards/screen_028.py DIR` and `--gate DIR`. d2, 32 seeds a cell: noise about ±9
on a win rate, ±12 on a gap. Full tables: `r16-screen.md`, `r17-screen.md`,
`r16-dominance-planner.md`, `r17-gate.md`, `r17-curve.md`, `r17-draft.md` here. The r16 runs are the first commit of this
item, before any rule changed; the engine there reproduces 027's table exactly (59 / 47).

## Verdict

| Done when | Result |
|---|---|
| The rollout agent runs a 32-seed d2 screen at both sizes, within a stated runtime | **Met.** 128 games, 82 min of thread time: about 7 min on 12 free threads. It ran in 17 min on 5 threads of a busy machine; r16, measured free, is 4 min. |
| At d2, under the rollout agent, the presence loss ends some losing games | **Met.** 7 of 21 planner losses (c2 6 of 11, c3 1 of 10). |
| Some wins come from drawing out the Unity deck | **Met.** 10 of 43 planner wins (c2 3 of 21, c3 7 of 22); engine 5 of 21. |
| The harms ledger's checks pass | **Met.** Printed checks pass. Measured, Discord cards are 82% of presence destroyed (below). |

**The strategy-health gate does not pass**, as in 027: no check beats the planner by more than
noise and every exploit loses clearly, but tags remain level (below).

**Not met from the starting calibration:** presence is 55% of c2's planner losses and 10% of
c3's, against 10-30% (33% together). Winners lose 1.8 / 4.7 presence, so c3 sits over the 0-2.

## The planner

`planner` (`crates/aic-sim/src/agents/planner.rs`): the engine, with a shortlist of 2
alternatives a decision and 8 a round, in the action phase only, each played forward to the first
decision two rounds on with the engine as the policy; the best line on the heuristic wins. A
line runs on the real state, so it is exactly the game that follows. The line being followed is
reused from decision to decision, so a decision pays only for its alternatives.

**Better, not just slower (r16, d2, 32 seeds, win %):**

| | engine | planner | gain |
|---|---|---|---|
| c2, `hold_base` 2 | 59 | 88 | +28 |
| c3, `hold_base` 2 | 47 | 78 | +31 |
| c2, `hold_base` 3 | 31 | 66 | +34 |
| c3, `hold_base` 3 | 25 | 41 | +16 |

It beats the engine and manages a hold of 3: frees 1.8 / 2.5 a game there, against the engine's
1.4 / 2.0. Discord tips fall (c2 4.9 to 3.3), wins come a little earlier (c2 7.5 to 7.0).

**Speed (r16, d2, thread time a game, 12 threads free):**

| | engine | planner | games a minute on 12 threads |
|---|---|---|---|
| c2 | 2.9 s | 19 s | 250 / 37 |
| c3 | 8.6 s | 56 s | 84 / 13 |

**A trap worth keeping.** The first version scored decisions on open decks and won 6%. On open
decks the one-step heuristic itself won 19% (c2, 16 and 32 seeds) against the guessing engine's
56-59%, in 520 decisions a game against 420. My reading, not measured: it ranks cards by printed
cost, and seeing the real draft lets it chase them. Each decision inside a line now scores on a
guess. Related: `engine:samples=2` won 72% at c2 against 59% (32 seeds, a `run`, not kept here),
so averaging guesses may be a cheap gain.

**What made it fast enough:** seat keys as tables and reach worked out once a position (15% off
the engine, 2.25 to 1.91 s a game, same results), and `prune` 6: after a phase's first full scoring, each seat keeps
its 6 best actions for that phase of a rollout, which halves the cost. Win rate unchanged
within noise: 81 / 81 pruned, 81 / 94 not, 16 seeds.

## r16 under the planner: do slow tags still fail?

Gap to the planner, 32 seeds, ±8-11. Under the engine (027): ignore-Industry −9 / −6,
ignore-Cyber −6 / +3, ignore-Power +3 / −3, no-powers 0 / 0, tier-2-rush +9 / −6.

| check | c2 (planner 88) | c3 (planner 78) |
|---|---|---|
| ignore-Science | +0 ✗ | +0 ✗ |
| ignore-Industry | −3 ✗ | +9 |
| ignore-Cyber | +9 | +9 |
| ignore-Power | +3 | +19 |
| no-powers | +25 | +0 ✗ |
| tier-2-rush | +0 ✗ | +0 ✗ |
| mono-Industry, Cyber, Power | +88, +88, +84 | +78 each |

**Partly.** Planning moves ignore-Cyber and ignore-Power the right way, and Power clears the noise
at c3; no-powers clears it at c2. But Industry still doesn't earn its keep, and ignore-Science is
level at both sizes. r17 goes further at c3 (the gate, below).

## r17 at d2 (32 seeds)

| | engine c2 | engine c3 | planner c2 | planner c3 |
|---|---|---|---|---|
| win % | 47 | 19 | 66 | 69 |
| win rounds | 5:2 6:2 7:3 8:4 9:3 10:1 | 6:1 7:1 9:3 10:1 | 4:1 6:7 7:9 8:1 9:1 10:2 | 7:8 8:7 9:4 10:3 |
| loss rounds | 7:1 8:4 9:7 10:4 11:1 | 7:6 8:6 9:5 10:5 11:4 | 8:6 9:4 10:1 | 7:1 9:4 10:3 11:2 |
| wins: purge / Unity deck | 11 / 4 | 5 / 1 | 18 / 3 | 15 / 7 |
| losses: Discord / presence / deck-out | 10 / 6 / 1 | 14 / 8 / 4 | 5 / 6 / 0 | 7 / 1 / 2 |
| Unity / Discord tips a game | 3.8 / 5.0 | 3.7 / 5.9 | 4.2 / 4.1 | 4.6 / 4.8 |
| presence lost a game: wins / losses | 2.7 / 19 | 0.8 / 28 | 1.8 / 20 | 4.7 / 27 |
| cities freed a game | 1.4 | 1.9 | 1.6 | 2.4 |

r16 for comparison: engine 59 / 47, planner 88 / 78, no Unity-deck win, no presence loss.
Earliest loss round 7. One win at round 4 in 128 games.

**Presence destroyed, measured:** Discord cards 82%, tier-2 threats 9%, war 7%, corruption 2%.
That's above the "no source over half" 027 reported, because the last step is 7 tokens a
player. The ledger's check is on printed sources and passes (5 Discord cards of 12 kinds of
printing and rule).

## Tuning steps (d2, c2 / c3 win %)

Screening runs, most stopped early once they had said what they had to say: the first two rows
have 16 games a cell, the rest 5-13 (presence 5 credits: 3-6). Read them as directions, not rates.

| step | engine | planner | presence losses |
|---|---|---|---|
| `hold_base` 2, `unity_cards` 3, 2, hits of 1 at 3+ | 62 / 94 | 94 / 100 | 0 |
| `hold_base` 3, `unity_cards` 3, 3 | 50 / 31 | 69 / 75 | 0 |
| plus hits of 2 at 4+ and on Siege | 58 / 42 | 69 / 89 | 0 |
| plus one bought presence a round | 17 / 33 | 82 / 44 | 0 |
| hit of 5 on the last step | 55 / 36 | 70 / 100 | 0 |
| presence 5 credits and 2 energy | 0 / 17 | 40 / 67 | the first one; everyone else suffers too |
| **hit of 7 on the last step** | 55 / 18 | 64 / 80 | 5 of 20 |
| hit of 6, presence 4 credits | 86 / 14 | 57 / 60 | 0 |

**Why 7.** Teams fill both tracks by the late game, 9 tokens a player, less what recent hits took.
027's "5-6 tokens" was wrong or out of date: a token pays income for 1 energy or 3 credits, so
everyone buys every one (about two thirds by the standard action and project, a third by cards
and growth). A hit of 5 ended no game. At 7 a seat needs 8 or more to survive, and some don't
have them.

**d4 is different:** one fewer Discord card means the deck ends at the fifth tip, so the step at 5
never acts, and d4 had no presence loss in the curve. The hit of 2 at 4+ is its last hit.

## How each loop is capped

- **Lost presence → lost income → fewer counters → more uncountered threats → more lost
  presence.** Capped by when it can start: each-player hits come only at 4+ Discord tips and
  from two tier-2 threats, so stage I is untouched, the stage-I rule still guards a last
  presence, and hits of 7 only land on the last card of the Discord deck. The earliest planner
  loss is round 7, none before round 7 for the engine. Tokens are bought back at the old price.
  Winners keep 8-9 tokens a seat at the end, so the spiral doesn't reach them.
- **Unity → a smaller deck runs out → a win.** See `unity_farm` in the gate: a fixed policy
  that buys ambassadors and lands them on bases and factories before anything else. The cap is
  the price: each ambassador is 6 credits and 1 energy for at most 2 Sway, and a tip is 4-6.
  It loses 28 / 34 points to the planner (the gate, below).
  Volunteer Corps (sway 3) gives a head start on the next tip but can't chain one at 2 or 3
  countries, since a tip is 4 or more.

## The end conditions, for the rules review

**Purge.** Still the main win: 33 of 43 planner wins, rounds 4-10, mostly 6-8. The hold at
3 gives a real push, 1.6-2.4 frees a game; the engine lands fewer and its c3 falls to 19%, which
now measures the agent as much as the game.

**The last Unity card.** Now a real win, a quarter of the planner's wins and a third at c3,
rounds 8-10 there (6-7 at c2). Deck of 6, tips averaging 4-5, so it takes a strong game. A deck
of 5 ran with `hold_base` 2 at 94-100%; no run isolates the deck from the hold.

**Discord deck.** Still the main loss for the engine, around rounds 8-10.

**Presence.** Now live, but as a late check rather than a slow bleed: 7 of 21 planner losses and
14 of 43 engine losses, every one at the sixth Discord tip. It trades against the Discord-deck
loss one tip later, so it moves how a team loses more than whether. Planner c2 feels it far more
than c3 (55% against 10%); I don't know why, and 11 and 10 losses are few. It reads as "hold 8
tokens each by the sixth tip". A slower bite needs a token to be scarce, which it isn't: see
*Why 7*.

**Deck-out.** Rare for the planner, 2 of 21; the engine has 5 of 43, four of them at c3, all at round 11.

## Sanity: drafting

Drafting passes, for 10 minutes on 2 threads. Engine, r17, d2, 32 seeds, win %:
keeping by plan 47 / 19, keeping at random 13 / 3, keeping nothing 0 / 0. Keeping nothing has
0.2 Unity tips a game: the cards are the push. Spec: `experiments/screen-028-draft.json`.

## The curve (r17, 16 seeds, engine / greedy, %)

| countries | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| 2 | 50 / 12 | 44 / 12 | 44 / 12 | 6 / 0 | 19 / 6 |
| 3 | 75 / 0 | 56 / 0 | 12 / 0 | 6 / 0 | 0 / 0 |

Random wins 0 everywhere. **A cost to flag:** with the hold at 3, the one-step agents no longer
meet "moderately skilled play essentially always wins at d0" (`docs/principles.md`). The planner
is the agent these rules were set for, and its curve isn't measured. If the engine should stay a
fair stand-in for a table of new players, `hold_base` 2 at d0-d1 is the first thing to try.

## The gate (d2, 32 seeds)

Gap = the agent's win % minus the check's, points, ± one standard error, each check against its
own agent. ✗: not below it. 32 seeds a cell, 3,200 games. Base: engine 47 / 19, planner 66 / 69.

| check | engine c2 | engine c3 | planner c2 | planner c3 |
|---|---|---|---|---|
| keep-all | +47 ±9 | +19 ±7 | +66 ±8 | +69 ±8 |
| keep-cheapest-k | +38 ±10 | +16 ±8 | +19 ±12 | +38 ±12 |
| tier-1-only | +47 ±9 | +19 ±7 | +62 ±9 | +69 ±8 |
| tier-2-rush | +0 ±12 ✗ | -25 ±11 ✗ | +0 ±12 ✗ | +12 ±12 |
| no-powers | +6 ±12 | -9 ±11 ✗ | +9 ±12 | +0 ±12 ✗ |
| no-counters | +44 ±9 | +19 ±7 | +38 ±12 | +66 ±9 |
| no-presence | +47 ±9 | +19 ±7 | +66 ±8 | +69 ±8 |
| event-hoarder | +44 ±9 | +19 ±7 | +38 ±12 | +59 ±10 |
| ignore threats | +44 ±9 | +19 ±7 | +34 ±12 | +69 ±8 |
| turtle | +47 ±9 | +19 ±7 | +66 ±8 | +69 ±8 |
| free cities only | +47 ±9 | +19 ±7 | +66 ±8 | +69 ±8 |
| Unity farm | +41 ±10 | +12 ±8 | +28 ±12 | +34 ±12 |
| mono-Science | +47 ±9 | +19 ±7 | +66 ±8 | +69 ±8 |
| ignore-Science | +6 ±12 | -9 ±11 ✗ | -12 ±11 ✗ | +9 ±12 |
| mono-Industry | +47 ±9 | +19 ±7 | +66 ±8 | +69 ±8 |
| ignore-Industry | +16 ±12 | -16 ±11 ✗ | -9 ±11 ✗ | +6 ±12 |
| mono-Cyber | +47 ±9 | +19 ±7 | +66 ±8 | +69 ±8 |
| ignore-Cyber | -3 ±12 ✗ | -6 ±10 ✗ | +0 ±12 ✗ | +28 ±12 |
| mono-Power | +47 ±9 | +19 ±7 | +66 ±8 | +69 ±8 |
| ignore-Power | +38 ±10 | +9 ±9 | +6 ±12 | +31 ±12 |
| mono-Diplomacy | +12 ±12 | +16 ±8 | +6 ±12 | +56 ±10 |
| ignore-Diplomacy | +47 ±9 | +16 ±8 | +66 ±8 | +69 ±8 |
| mono-Security | +47 ±9 | +19 ±7 | +62 ±9 | +69 ±8 |
| ignore-Security | +28 ±11 | +19 ±7 | +25 ±12 | +47 ±11 |

**Verdict: no check beats the planner by more than noise, and every exploit loses clearly, but the
gate still doesn't pass.** Against the planner, c2 has 4 checks not below it (tier-2-rush, ignore-Science,
ignore-Industry, ignore-Cyber) and c3 has 1 (no-powers). The largest is ignore-Science at c2,
−12 ±11. All are within about one standard error of level.

**Do slow tags still fail when the agent can plan?** At c3 Cyber and Power no longer do: ignoring
them costs the planner 28 and 31 points (r16: 9 and 19), so a plan can use them, and the engine,
which can't, is level or ahead without Cyber. Science and Industry still fail at both sizes. Nothing
at c2 clears the noise. **The structural cause, as far as I can read it:** a two-round line sees
what pays in two rounds. Decisions come down to the push (Diplomacy, ignoring it costs all of
it) and counters (credits, Security, +25 / +47). Industry pays only over rounds, and the deciding rounds
are 6-8, so its payback has little time. I did not tune around this.

**tier-2-rush beats the engine at c3 by 25 ±11**, the one check clearly ahead of its agent. It trails
the planner by 12. So I read it as the engine's keep plan under-keeping tier 2 with the hold at
3, more than as a rule; the engine's c3 win rate of 19% is the same symptom. It wants a look,
since the keep plan is shared: the planner's draft and keeps follow the engine, so it may be
under-keeping too.

**Under the engine** the gate is weaker: at c3, with the engine at 19, five checks sit at or above it.
The engine can't manage these rules well enough to judge them there, which is the case for
judging by the planner.

**Unity farm.** Loses at both sizes under both agents: +41 / +12 for the engine, +28 / +34 for the
planner. It isn't dead, though: under the planner it won 23 of 64, every win by the Unity deck, at
rounds where the planner's own wins fall, and the rest of its games mostly end by presence (31 of
41 losses), because it skimps on the board. A focused Unity line is a real plan with a real
cost, which is what a second win should be.

## New-player summary of what changed

Three things changed. **Lost presence hurts more:** a token you lose goes back on your track and
covers the income under it until you place it again. **Late Discord cards hit every player:** the
fifth Discord tip usually makes every player destroy 2 of their own tokens, and the sixth makes
them destroy 7, so a team deep in Discord needs nearly full tracks or it loses at once. **The Unity
deck is short:** a game deals only 6 Unity cards, and drawing the last wins, so swaying toward
Unity is a second way to win as well as a way to ease the purge. Freeing a corrupted city now
takes 3 ambassadors plus 1 for each adversary ambassador on or next to it, so plan the push a
round ahead.
