# History

What the README used to carry that is history rather than how to build, run and test
today: past sweeps and their recipes, timings measured on rules that have since
changed, and the specs the binary no longer accepts. Moved here in 016, otherwise
as written. Every number is as measured at the time, on the rules revision named; spec
paths are as they were then (the refused specs now live in [`archive/`](archive/)).

## Tuning run (007) and engine agent (008)

The 007 run's log, change log and proposals: `experiments/tuning-007.md`. The 008
engine agent, its sweep and the building-engine verdict: `experiments/engine-008.md`;
rerun `experiments/engine-agent-008.json` (~28 min alone on 12 threads, 2,400 games:
greedy-search agents take 6–16 s a game each with every core busy) to judge a payback
lever.

## Payback sweep (009)

Write-up `experiments/payback-009.md`. Config levers are plain specs:
`experiments/payback-009-singles.json` (13 variants × d0, d4 × greedy, engine × 96
seeds = 4,992 games, **46 min** alone on 12 threads) and
`experiments/payback-009-combos.json` (both refused today, archived). Levers that config
can't reach (card costs and income, tier-2 costs, the AI tracks) run as **content
patches** built into separate binaries from a copy of the repo under `out/`. Committed
content is never touched. The scripts are r3's and not checked against r6:

```sh
python scripts/payback/patch_content.py --list                      # the patches
bash scripts/payback/build_patches.sh insight_inc econ_half+insight_inc   # -> out/patch-bin/<patch>.exe, ~1 min each
bash scripts/payback/run_content.sh insight_inc insight_inc:insight_inc+keep0  # <binary>:<spec> in experiments/payback-009-content/
python scripts/payback/summarize.py out/payback-009-singles out/payback-009-content/*/ --picks 20
```

`scripts/payback/resources.py` (unspent resources by round from a `--trace` game) was
retired in 015 (it keyed on the phase `adversary.ravage`, removed in 014) and deleted in
016, as was `scripts/tune/trace_summary.py` (a round-by-round summary of one `--trace`
game, reading pre-014 token and track names).

One content experiment (384 games) takes about 5 minutes alone. Records from a patched
binary carry its content hash, so they replay only on a binary built from the same
patch (main's `aic-sim replay` fails on them, typically "unaffordable"). Running two or
three experiments at once gives the same total throughput (about 1.3 games/s for greedy
+ engine), and even `tail` on a log can take 30+ s under that load.

## Proposals (011)

Write-up `experiments/proposals-011.md`: 007's accepted proposals as variants and new
defaults (rules r4). The timings below are r4's, and `proposals-011.json` and two of the
screens are refused today. Screens (no probes, greedy):
`experiments/screen-011-difficulty*.json` (512–1,024 games, 9–16 min each with another
run beside it) chose the new level 1 and 3 effects; `screen-011-clock*.json` ran on
009's `t2_half+t2_noam` patch binary (`bash scripts/payback/build_patches.sh
t2_half+t2_noam`, then `out/patch-bin/t2_half+t2_noam.exe experiment ...`; 192–480
greedy + engine games, 7–12 min). The measurement, `experiments/proposals-011.json`
(10 variants × d0–d4 × greedy, engine × 64 seeds = 6,400 games, no probes), was
stopped unfinished: 2,109 games in 28 minutes alone on 12 threads (about 1.25 greedy +
engine games a second), so about 85 minutes for the whole grid. Since 012 the whole
grid ran in **10.6 minutes** (633 s, about 10 games a second; report 0.3 s; measured
before the last heuristic fixes, so somewhat less after), and a stopped run keeps
its games. `proposals-011 --quick` was 1,600 greedy + engine games, about 3 minutes (r4).

## Specs the binary refuses (archived in 016)

The rules or configs they name are gone; kept as history until rewritten, in
[`archive/`](archive/): `tune-before` (`configs/v0_untuned.json` is missing),
`variants-spread` (the `clusters` and `assets` configs), `proposals-011` and
`variants-modules` (the removed `opposition` module), `screen-011-difficulty` and
`screen-011-difficulty2` (`adversary.foundry_bonus`), `payback-009-singles`,
`payback-009-combos`, `payback-009-content/insight_inc+fc4.json` and
`payback-009-content/econ_half+insight_inc+d4tune.json` (removed parameters), and
`tune-agents` (removed agent weights). Committed results from r5 and earlier are
history: their game records do not replay.

## Speed (012)

Measured on r4, before 013 and 014 changed the game. A 400-game slice of
`proposals-011` (`--games 4`: every variant, difficulty and agent) took **263.8 s**
before 012 and **34.5 s** after, alone on 12 threads, with identical games (`python
scripts/bench/same_results.py DIR_A DIR_B` compares two output directories, timing
aside). Greedy then took about 0.6 s a game and the engine agent about 1.3 s, with every
core busy. Where the time went, and what fixed it (times for a 20-game slice):

- The agents are 99% of a run; rules apply, legal actions, the replay check and setup
  are each under 1%, and writing the report well under a second.
- Most of the agents' time was the system allocator: search clones game states by the
  hundred thousand, and on 12 threads Windows' heap serialized them (a state clone took
  134 µs on 12 threads, 24 µs on one). `aic-sim` uses **mimalloc** (19.6 s → 4.8 s on a
  20-game slice).
- A state shares its config, and a pending choice its snapshot, through `Arc` (they
  serialize as before), and searches call `Rules::apply_to`, which copies the state once
  where clone + `apply_mut` copied it twice (the action's snapshot): 4.8 s → 2.7 s.
- The heuristic (the engine agent's horizon mode most): the economy's production built
  token keys per hex and looked at the adversary's reach everywhere; each building in
  hand worked the seat's building discount out again; cards were found by id with a
  linear search (now an index in `Content`); the defense values and unpaid threats were
  each worked out twice per position. 2.7 s → 2.3 s, the heuristic 38 → 22 µs a call.
- Left: the heuristic (~50% of search time, ~22 µs a call) and apply with its state copy
  (~40%, ~17 µs). A state is ~200 small allocations (string keys and card ids); interning
  them is the next big step.

## Timing before 012 (v0 baseline)

These predate 012's speed-up (about 7.6× faster on 12 threads, 5× on one: 10 games of
`proposals-011` on one thread took 29.0 s, then 5.8 s).

v0 baseline, 2 countries, release build, this 12-thread machine; games last 3–5 rounds,
about 85–120 decisions. One game on one thread:

| Agent | ms/game | ms/decision |
|---|---|---|
| `random` | 1.4 | 0.02 |
| `greedy` | ~900 | ~7.5 |
| `beam` | ~1,100 | ~10 |
| `rollout` | ~2,500 | ~21 |

With all 12 threads busy each game takes 1.5–3× as long (shared cores), so an
experiment's throughput is about 4–5 greedy or 3–4 beam games a second. Probes add to
that: win probability costs `rollouts` playouts per round of each probed game (8
greedy playouts ≈ 8 × 0.5 s per round, the bulk of a probed game's time); closeness
costs about as much as a greedy decision per probed decision. Measured:
`experiments/ladder-v0.json` (600 games: 2 difficulties × random, greedy, beam × 100
seeds, 10 probed seeds per cell) took **3.7–4.3 minutes** (two runs); `experiments/smoke.json` (36
games) 30 s.

## Timing on the tuned baseline (007)

Games lasted 7–8 rounds, about 200 decisions; 12 threads busy, so per-game times are
under load:

| Agent | s/game under load | Throughput (12 threads) |
|---|---|---|
| `random` | 0.003 | ~300 games/s |
| `greedy` | 4–9 | ~1.5–2 games/s |
| `beam` | 45–60 | ~0.2–0.25 games/s |
| `rollout` | 60–80 | ~0.15–0.2 games/s |

- `experiments/tune-after.json` (2,000 games: random and greedy × d0–d4 × 200, 10 seeds
  per cell probed for win probability): **45 minutes**. The probes are most of it;
  without them it is about 12 minutes.
- `experiments/tune-after-search.json` (greedy, beam, rollout × d0, d4 × 24): 8 minutes.
- `experiments/variants-modules.json` (3,072 games, no probes): 11 minutes.
- `experiments/payback-009-singles.json` (4,992 greedy + engine games): 46 minutes;
  `payback-009-combos.json` (1,152): about 20 minutes alone. A content-patch spec (384
  games) takes 5–6 minutes alone. The engine agent takes 7–25 s a game under load,
  about twice greedy.
- `scripts/tune/sweep.py`: one `--set` point of 64 greedy games takes about 45–60 s.

## Slow test lane on 014

`cargo test -p aic-sim` with `--include-ignored` took 78–122 s on 014 (the whole-game
`rollout` and `beam` tests); 015's fix to the beam's plan check brought it to about
25 s.
