# Three loss clocks, Spirit Island presence, earlier Science and Industry (034, r19)

The review called this r18; 033 took the number, so it is rules revision 19. Judged by the
`planner`, d2, 32 seeds a cell (±9-12 points on a win rate): `aic-sim experiment
experiments/screen-034-dominance.json --quick --games 32`, then `python scripts/cards/screen_034.py
DIR` and `--gate DIR`.

## The keep plan, before any rule changed (r18, engine, d2, 32 seeds)

| agent | c2 win % | c3 win % |
|---|---|---|
| engine, `plan_rounds` 3, `plan_reserve` 2 (r17's) | 38 | 31 |
| `tier2_rush` | 31 | 47 |
| `samples=2` | 25 | 44 |
| `plan_rounds` 4 | 47 | 38 |
| `plan_reserve` 0 | 44 | 41 |
| **`plan_rounds` 4, `plan_reserve` 0 (kept)** | **59** | **50** |
| those, plus `tier2_rush` | 66 | 53 |

The under-keeping was the plan looking too few rounds ahead and holding insight back for keeps
it then didn't make. Fixed, tier-2-rush's lead is 7 / 3 points, inside the noise. `plan_credit` 1
(9 / 25) and `w.packet_keep` 0 or 0.5 are worse or level. **`engine:samples=2` did not hold**
(-13 at c2, +13 at c3, within noise of nothing): dropped. Spec:
`experiments/archive/screen-034-keep.json`, which needs the r18 config.

## Calibration (d2, 32 seeds, planner; c2 / c3)

| step | win % | Discord / presence / cap losses | tokens left a seat | Discord share of presence lost |
|---|---|---|---|---|
| hits 1, 2, 3, 13 tokens, cap 12 and 11 (16 seeds, old keep plan) | 50 / 25 | 5 / 15 / 0 | 1.0 / 1.9 | 63% |
| last hit 2, keep plan fixed | 78 / 50 | 20 / 3 / 0 | 0.5 / 1.0 | 49% with the next row |
| `corruption_reach` and `war_reach` 1 | 53 / 34 | 13 / 23 / 0 | 0.9 / 1.4 | 49% |
| `corruption_reach` 1 alone, `energy_credits` 4, stage-III sway 1 | 75 / 44 | 13 / 12 / 1 | 0.7 / 1.0 | 47% |
| `war_reach` 1 alone, same | 78 / 44 | 10 / 15 / 0 | 0.6 / 1.0 | 47% |
| corruption alone, steeper extras | 81 / 53 | 11 / 10 / 0 | 0.7 / 0.7 | 48% |
| plus `unity_cards` 4, 3 | 69 / 31 | 18 / 14 / 0 | 0.3 / 1.1 | 48% |
| 14 tokens, cap 11 and 10 by `stage_cards` 4, 4, 3 | 75 / 28 | 13 / 16 / 2 | 1.0 / 1.8 | 44% |
| by `stage_cards` 4, 5, 2 | 66 / 28 | 16 / 15 / 3 | 1.1 / 2.0 | 48% |
| **plus `free_city` 5 (r19)** | **66 / 38** | **12 / 16 / 3** | **1.0 / 1.9** | **48%** |

The first rows predate a bug that charged a presence's rise and extra twice; the fix is in every
row from `energy_credits` 4 on.

## r19 at d2 (32 seeds, planner)

| | c2 | c3 |
|---|---|---|
| win % | 66 | 38 |
| win rounds | 6:1 7:2 8:6 9:6 10:6 | 8:1 9:8 10:3 |
| loss rounds | 5:1 7:2 8:2 9:2 10:2 11:2 | 5:1 6:3 7:4 8:4 9:4 10:3 11:1 |
| wins: purge / last Unity card | 9 / 12 | 3 / 9 |
| losses: Discord / presence / cap | 3 / 6 / 2 | 9 / 10 / 1 |
| share of losses | 27 / 54 / 18 % | 45 / 50 / 5 % |
| presence losses by round | 5:1 7:2 8:1 9:1 10:1 | 5:1 6:3 7:2 9:3 10:1 |
| tokens left a seat at the end | 1.0 | 1.9 |
| rich actions a game | 0.1 | 0.8 |
| presence lost: wins / losses | 10.5 / 20.5 | 22 / 26 |

Together, 31 losses: Discord 39%, presence 52%, cap 10%. **Every loss condition ends games and none
is over 60%, at each size and together.** Presence losses fall in rounds 5-10, not at one tip; the
5 of 16 before round 7 are earlier than the 7-11 asked for: a seat that placed little when stage II
hits starts. Unity-deck wins are 21 of 33 wins, more than "may be rare": `free_city` 5 feeds it.

**Measured presence destroyed, all gate games:** Discord cards 46%, stage-III twists 17%, tier-2
threats 15%, corruption 14%, war 5%, tier-1 threats 3%. **No source is over half** (r17: Discord
82%). Tier-1 threats are small for two reasons: stage I is short at d2 and the planner counters
them while it has few tokens.

**c3 is far below c2.** 38 against 66. c3's wins are late, rounds 9-10, against a cap of 10, and at
`stage_cards` 4, 5, 3 it won 53% (with 13 tokens). A cap that bites c3 first is a real cost of
getting cap losses at all.

## The gate (d2, 32 seeds; gap = planner minus check, ± one standard error)

Base 66 / 38. Full table: `r19-gate.md`, with `r19-gate-detail.md`. New exploit checks first.

| check | c2 | c3 |
|---|---|---|
| **rich-spend** (every spare credit on the rich action) | +44 ±11 | +34 ±9 |
| **Science rush** (draft, keep and play Science first) | +9 ±12 | +19 ±11 |
| tier-2-rush | -9 ±11 ✗ | -3 ±12 ✗ |
| keep-cheapest-2 | +25 ±12 | +16 ±11 |
| no-powers | -3 ±12 ✗ | +9 ±12 |
| no-counters | +66 ±8 | +38 ±9 |
| ignore threats | +62 ±9 | +38 ±9 |
| Unity farm | +16 ±12 | +31 ±10 |
| mono-Diplomacy | +44 ±11 | +38 ±9 |
| ignore-Science | **-16 ±11 ✗** | -9 ±12 ✗ |
| ignore-Industry | +12 ±12 | -6 ±12 ✗ |
| ignore-Cyber | +0 ±12 ✗ | +12 ±12 |
| ignore-Power | +28 ±12 | +34 ±9 |
| ignore-Diplomacy | +53 ±10 | +38 ±9 |
| ignore-Security | +28 ±12 | +25 ±10 |

The other 11 checks (keep-all, tier-1-only, no-presence, the other monos, events-only, turtle, free cities) ran at 8 seeds for time: `r19-gate-rest-8-seeds.md`.

**Verdict: the gate does not pass.** Both new exploits lose. tier-2-rush is level at c3 (r17: -25
for the engine). Ignore-Power and ignore-Security clear the noise at both sizes, as ignore-Diplomacy
always has. **Industry moved:** ignore-Industry costs 12 at c2 (r17: -9) and is level at c3. **Science
did not:** ignoring it is ahead at both sizes, -16 at c2, more than a standard error.

**The structural cause, as far as I can read it.** A Science card pays in insight and cards, and
insight only becomes board change through other tags' cards, so a Science card competes with the
card it would pay for. The tag's tier-1 discount does nothing: tier-1 keeps cost 1 and the floor is 1,
and most plays cost 1, so `science_every` acts only on tier-2 keeps and 2-insight plays. Meanwhile
wins hinge on the push (Diplomacy) and the counters (credits, Security). `never_tag` also bars every
multi-tag card carrying Science, and the seat's slots go to what the rollout rewards, which is
credits, within two rounds. I did not tune around this. Cheapest next test: `science_every` 1, or a
discount on the 1-insight keep with floor 0 for Science cards alone.

## What the agents can and can't do here

- **A presence as a board location: partly.** The heuristic pays a flat value a token and a value for
  each city, base and station in reach; it doesn't value a hex for what it will be near next round,
  and doesn't price the marker a loss leaves. The new danger (from 4 tokens down) helps them hold
  tokens. Judge "location matters" by `corruption_reach` and losses by round, not by placement.
- **The rich action: yes,** when it pays: 0.1 and 0.8 uses a game, more at c3 where seats are short
  of energy. `rich_spend` shows the other end. It fits "worth it only with spare credits".
- **Placement across rounds: no.** Rollouts see two rounds, and a token's cost is mostly today's. Most
  tokens come free from growth and cards (70%), so the agents rarely face the choice the extras set.
  Judge the extras by tokens left, not by what agents would pay. **The extras price only the last
  third of tokens; if tokens should be a real tradeoff, growth is the first thing to change.**

## The loops and their caps

- **Lost presence → marker → corruption → lost presence.** Cap confirmed: a marker adds nothing to a
  corrupted site, a node or a marker, and takes no presence itself. Measured in the base cells: 10.7 markers a game, 1.0 corruption a game
  and no marker ever removed, so the loop is not running away, and the Remove option goes unused. `corruption_reach` does
  open a second path, a city's fall leaving markers on its neighbours: a marker on a neighbouring
  city counts toward corrupting it. Its size is one a corruption at most.
- **Placement → income → placement.** Cap replaced in part: the rich action's rise and the extras
  hold, but growth and cards skip the extras, so the real cap is the 14 tokens.
- **Science tags → cheaper insight → more Science.** Floor of 1, confirmed; the problem is the
  opposite one, above. The Industry floor holds for all building discounts now.

## New-player summary of r19

You lose in three different ways now. **The Discord deck** is the political clock: tips hurt, and a
tip with no card left loses. **Your presence** is the board clock: threats, late adversary cards,
Discord cards, war and falling cities destroy your tokens, and a token destroyed is gone for good,
though its income stays. It leaves a corruption marker, and you can't place on that hex until
someone removes the marker. If any player has none on the board, everyone loses. **The round cap** is
a quiet backstop: 11 rounds, 10 from difficulty 2. You win by purging the board or drawing the last
Unity card. Your two tracks hold 14 tokens: the first ones are cheap, the last ones cost up to 5
energy more. From stage II you can pay credits for a token instead, at a steep price that rises each
time in a round. Science and Industry now pay quickly: cheap cards with a gain up front, and every
two tags of them cut your costs a little, never below 1.

**Rules that resisted the summary:** the corruption marker's two effects and its exceptions need a
sentence each, and which placements pay the step extra (the action does, a card doesn't) reads as a
wart. Both are candidates to simplify.

## The curve (r19, planner, 12 seeds a cell, win %, ±14)

| countries | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| 2 | 100 | 100 | 67 | 17 | 17 |
| 3 | 100 | 58 | 42 | 0 | 0 |

Losses at d3-d4 are mostly presence (c2 d3 9 of 10, c3 d4 11 of 12) and early, rounds 4-6 at d4:
the fix for d4's steps means hits start a tip sooner there. The step from d2 to d3 is steep. d0
never loses, so nothing about d0 measures the rules.
