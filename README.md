# aicomplete

**AI Complete**, a cooperative board game about AIs, being turned into a digital version.
The rules live in a pure, deterministic Rust core that simulates them. A native simulator
and a wasm build for a JS front end sit on top of the core, and neither holds any rules.

- [docs/principles.md](docs/principles.md): design pillars, difficulty targets, lenses,
  measures.
- [docs/rules.md](docs/rules.md): what the code implements: ruleset **v0**, Militarist
  against Builder and Protector, 1–4 countries, with every default and deviation.
- [docs/ruleset-v0.md](docs/ruleset-v0.md): the v0 design draft as handed over.
- [docs/architecture.md](docs/architecture.md): modules, configs and tunables, the
  invariants, and how to add a module, variant or card.
- [CLAUDE.md](CLAUDE.md): the working contract for executors.

## Layout

| Path | What it is |
|---|---|
| `crates/aic-core` | The rules engine. No I/O, no clock, seeded per-purpose RNG streams in the state, serde state, exact replay. `Rules::new(config)` then `new_game` / `legal_actions_into` / `apply_mut`, plus the convenient free functions `new_game`, `legal_actions`, `apply`. |
| `crates/aic-core/src/modules/` | One file per rules area (`board`, `adversary`, `threats`, `opposition`, `economy`, `ai`, `cards`), shared code (`common`, `effects`), the registry in `mod.rs`. |
| `crates/aic-sim` | Native CLI: seeded games over a seed range, one config or two side by side, with every game replay-checked; the agent skill ladder (`agents/`) and the experiment harness (`experiment.rs`, `report.rs`). |
| `experiments/` | Experiment specs, and `results/<name>/`: committed cycle reports with their records. |
| `crates/aic-wasm` | wasm-bindgen bindings, JSON strings in and out. |
| `content/` | Data embedded at compile time: `map.json` (piece shape, layouts for 1–4 countries, the four printed country pieces), `cards.json`, `threats.json`, `adversary.json`, `ais.json`, all built from code primitives. |
| `configs/` | Game configs: modules, variants, every parameter. `baseline.json` is v0 as tuned by the 007 run with 011's rules, r4 (2 countries, difficulty 0); `v0_untuned.json` keeps v0's numbers and rules; `shared_row.json` differs only in the threat variant; `clusters.json` and `assets.json` only in the adversary (the two spread variants, rules.md §2a). Other country counts and difficulties: `--set countries=N`, `--set adversary.difficulty=N`. Off-variants (rules.md §2b): `--set threats.variant=off`, `opposition.variant=off`, `cards.variant=off`. 011's opt-in variants (rules.md): `opposition.variant=insight` (+ `opposition.insight_income=1`), `opposition.variant=counter_all`, `cards.variant=free_keep` (+ `cards.free_keeps=1`), `adversary.variant=uplinks` (+ `adversary.explore_foundries=0|1`). |
| `scripts/wasm-smoke.mjs` | Plays one game through the wasm package in Node. |
| `web/` | Browser viewer (Vite + TS on `aic-wasm`): play any config and seed, watch and step through sim game records, copy bug reports. See [web/README.md](web/README.md). |

Core source map: `rules.rs` (config compilation, round/phase loop, apply, choice
replay), `module.rs` (the module framework: `ModuleDef`, `VariantDef`, `Module`, `Cx`,
`World`), `config.rs`,
`content.rs`, `map.rs` + `hex.rs` (axial hexes), `state.rs`, `action.rs`, `event.rs`,
`rng.rs`, `driver.rs` (`play`/`replay` with an `Agent`, `GameRecord`), `view.rs` (the
generic view clients draw), `record.rs` (`SavedGame`, the game record format), `tests.rs`.

## Viewer

```sh
cd web && npm install && npm run dev               # builds the wasm (~9 s warm), serves http://localhost:5173/
npm run dev -- --port 61870                        # another port
```

The dev server rebuilds the wasm and reloads when `crates/` or `content/` change. Open a
record from `aic-sim --save` with *Open record…* or by dropping the file on the page.
Details, and the browser check (`npm run e2e`), are in [web/README.md](web/README.md).

- A fresh worktree has no `web/node_modules`: run `npm install` first.
- `npm run build` (release wasm, typecheck, vite) took 92 s cold; `wasm-pack build
  crates/aic-wasm --target web` plus `node scripts/wasm-smoke.mjs` about 21 s; `npm run
  e2e` against a running dev server about 15 s.
- On a fixed port: `npx vite --port $PORT --strictPort` from `web/`, then
  `AIC_URL=http://localhost:$PORT/ npm run e2e`.
- The view (008) names each country (`countries[].name`), shows each seat's AI board as a
  `board` (name, innate rule, tracks with every step, slots, unique cards), and keeps
  module bookkeeping vars (threat payments, discounts, flags, ...) out of the globals:
  each module declares them and shows what players need where it belongs.

## Toolchain

- Rust stable (built with 1.98.1), target `wasm32-unknown-unknown`
  (`rustup target add wasm32-unknown-unknown`).
- wasm-pack (`cargo install wasm-pack --locked`); Node (built with 24) for the smoke test.

## Build and test

```sh
cargo build --workspace
cargo test --workspace                             # fast lane: ~10 s once built, ~18 s after an edit in aic-core (rebuild included); after every change
cargo test-all                                     # full run, slow lane included: ~70 s; before every commit
cargo test-slow                                    # the slow lane alone: ~65 s
cargo build --release -p aic-sim                   # ~12 s
wasm-pack build crates/aic-wasm --target web       # ~20 s cold, ~9 s warm -> crates/aic-wasm/pkg (git-ignored)
node scripts/wasm-smoke.mjs                        # needs the wasm build; checks native = wasm
```

**Test lanes.** The slow lane is the tests marked `#[ignore = "slow lane: ..."]`: whole
search-agent games, the greedy experiment grid, and more seeds of the replay checks
whose first seed runs in the fast lane. `test-all` and `test-slow` are cargo aliases
(`.cargo/config.toml`) for `cargo test --workspace -- --include-ignored` and
`-- --ignored`; one slow test runs with `cargo test -p aic-sim -- --ignored <name>`. A
new test that takes more than about a second goes in the slow lane, ideally with a
cheaper fast-lane twin (fewer seeds, fewer actions). Times measured on this 12-thread
machine with nothing else heavy running (010); the first build of the test profile
takes about 40 s.

- The test profile is optimized (`[profile.test] opt-level = 1` in `Cargo.toml`;
  debug assertions and overflow checks stay on): unoptimized, the same tests took
  4 min. `opt-level = 2` built 6 s slower and ran no faster.
- `cargo test` runs test binaries one after another, so the fast lane's wall clock
  is roughly the sum of each binary's slowest test. Today that is
  `experiment_reports_comparisons` (~5 s, six greedy games).
- Per-test times: `RUSTC_BOOTSTRAP=1 target/debug/deps/<binary>.exe --include-ignored -Z
  unstable-options --report-time` (the binary paths are printed by `cargo test --no-run`).

`Cargo.lock` started as a copy of corelight-engine's lock, so dependency versions match a
known-good wasm build.

**Rules changes:** the `golden_game` test pins the final state of one fixed game
(baseline, seed `u64::MAX`, action *k* = legal action `(k × 7919) mod count`; "always the
first action" can now pay and withdraw a threat counter forever). If a rules change is
intended, bump `RULES_REVISION` in `crates/aic-core/src/lib.rs` and update the
fingerprint in both `tests.rs` and `scripts/wasm-smoke.mjs`. The fingerprint covers the
whole final state, config `name` included. The revision is recorded with every sim
result, so old and new results are never mixed up.

Git Bash on this machine mangles backslashes inside heredocs, even with a quoted
delimiter. Write scripts with the editor, not with `cat <<'EOF'`.

## Simulator

Always run the sim in release.

```sh
S=target/release/aic-sim
$S run configs/baseline.json --games 2000                          # tuned v0, 2 countries, difficulty 0
for n in 1 2 3 4; do $S run configs/baseline.json --games 500 --set countries=$n; done
$S compare configs/baseline.json configs/shared_row.json --games 2000  # threat variants
$S compare configs/baseline.json configs/clusters.json --games 2000    # spread variants
$S compare configs/baseline.json configs/assets.json --games 2000
$S compare configs/baseline.json configs/baseline.json --set-b threats.variant=off  # off-variants
$S compare configs/clusters.json configs/clusters.json --set-b opposition.variant=off
$S run configs/baseline.json --set opposition.variant=income --set opposition.income_credits=2
$S compare configs/baseline.json configs/baseline.json --set-b adversary.difficulty=4
$S run configs/baseline.json --games 1 --trace     # first game as JSON lines
$S run configs/baseline.json --json                # machine-readable summary
$S show configs/baseline.json                      # resolved config, hash, every tunable
$S modules                                         # registry: modules, variants, params, phases
$S run configs/baseline.json --games 2000 --save out/games              # first 20 losses as records
$S replay out/games/*.json                         # check records natively (also browser bug reports)
```

| Option | Default | Meaning |
|---|---|---|
| `--games N` / `--seed S` | `200` / `0` | Seeds `S` to `S+N-1`, one game per seed per config (any `u64`, up to `u64::MAX`). |
| `--set K=V` | | Override for every config. Repeatable. `countries=N`; `<module>.<param>=V` with `V` as `3` or `1,2,3`, or empty to unset; `<module>.variant=NAME` (keeps the parameters the new variant shares, drops the rest); `<module>=off` (removes the module and its phases). |
| `--set-a K=V` / `--set-b K=V` | | Override for config A or config B only. |
| `--agent SPEC` | `random` | The team agent (see [Agents](#agents)), e.g. `greedy`, `beam:width=4`. Shown in the table header and in `--json`. |
| `--max-actions N` | `100000` | Per-game action cap. A capped game ends as `timeout`. |
| `--threads N` | all cores | Results don't depend on it: agents are seeded per game. |
| `--json` | off | Print the summary as JSON: every field below, plus the diff, agent, `max_actions` and rules revision. |
| `--trace` | off | Print the first game of each config: every action (with its option count) and event. Goes to stderr with `--json`. |
| `--save DIR` | off | `run`/`compare`: write the selected games as game records into `DIR` (created if needed), named `<config name>-<run\|A\|B>-seed<S>.json`, so A and B never collide. The paths are printed after the table, or listed under `saved` in each `--json` result with the match counts. A write failure exits 2. |
| `--save-when W` | `loss` | Which games to save: `all`, `win`, `loss`, `timeout`, or an end reason code (`overrun`, `survived`, ...). Repeatable; a game matching any is saved. With only `--save-seed` given, no default: just those seeds. A reason code no game ended with gets a warning on stderr (likely a typo). |
| `--save-seed S` | | Always save this seed, whatever `--save-when` says and not counted against `--save-max`. Repeatable. A seed outside the range played gets a warning. |
| `--save-max N` | `20` | Per config, keep the first `N` games `--save-when` matched, in seed order. The output says how many matched, so a cap never truncates silently. |

The output shows each config's hash, content hash and engine version, plus an
`A -> B` line listing every variant and parameter difference. The rows:

- `win %` (± standard error);
- rounds (mean, min..max);
- games by end reason and by end round;
- decisions per game, options per decision, forced decisions %, and max options;
- adversary ops per round (mean, and the max in any one round), and at setup;
- every module counter, averaged per game;
- `replay ok`;
- `ms/game`.

**Replay check:** each game is replayed from seed + actions through `Rules` rebuilt from
the resolved config, and must equal the final state. The exit code is 1 on any
mismatch, and 2 on a bad config or bad arguments (including a seed range that
overflows `u64`).

Measured on this machine (v0, 2 countries): about 1.4 ms per game, replay included
(every action and automatic phase clones the state for choice replay); 2,000 games take
about 3 s. Untuned v0 (`configs/v0_untuned.json`): random-agent games end in 2–6 rounds,
almost all on the corruption pool. Tuned baseline: random games last about 6.5 rounds
(max 10) and still lose (0 of 1,000; 3 ms per game). The spread variants (2,000 games each, 2 / 3
countries): `clusters` 1.2 / 1.9 ms per game, 2.8 rounds, about a quarter lost on
population; `assets` 0.9 / 1.3 ms, 2.0 rounds, all on the pool. Random agents won none
of the three. (The placeholder loop ran at 0.03–0.05 ms per game.)

**Game records** (`aic-game-record/1`, `aic_core::SavedGame`) are JSON: the resolved
config, the seed and the action list (one action per line), plus a `note` saying why the
game was saved (command, outcome, seed, selector, overrides) and the outcome, rounds,
final-state fingerprint, hashes, engine version and rules revision for checking the
replay. A saved record is the game the summary counted, not a second play. Only
`config`, `seed` and `actions` are required, so a hand-written record loads too. `aic-sim
replay <record.json>...` prints each record's seed, outcome and warnings (a different
rules revision, content or config hash, or final fingerprint), and exits 1 if any record
fails to load or replay or has a warning. Browser bug reports are checked the same way.

Measured (v0, 2 countries): a record is about 9 KB (about 90 actions); saving 300
records with their games takes 0.8 s, and `replay` of 300 records 1.2 s. Saving every
game of a big run is slow: Windows file writes are the bottleneck (placeholder-era
figure: 200,000 records took 82 s). A shell glob over more than a few
hundred records hits Windows' command-line length limit ("Argument list too long"), so
replay big sets in batches.

The random agent picks uniformly among all seats' legal actions, passes and choice
answers included. Treat it as a floor, not as a model of a weak team.

## Agents

The skill ladder (`crates/aic-sim/src/agents/`). One team agent drives every seat, with
hands and drafts open, and answers the "players choose" questions too. Agents play
through the public API only (`legal_actions_into`, `apply_mut`, reading the state).

- **No peeking.** Before any search an agent replaces what the printed game hides with
  a guess from its own RNG (`agents/determinize.rs`): the player-card and threat decks
  are shuffled (contents are public, order is not); each hidden adversary-deck position
  gets an unseen printed card of its stage (stages are stacked and public; the top
  cards stay put under Recon Sweep); every RNG stream is reseeded. The guess depends only
  on the public table and the agent's RNG; a test changes only hidden information and
  checks that decisions stay the same.
- **Own randomness.** Each agent's RNG is the game seed on a stream named after the agent
  kind, apart from every engine stream. Games replay from seed + actions, and results
  don't depend on `--threads`.
- **Heuristic** (`agents/eval.rs`): win/loss, the corruption pool, population slack,
  adversary pieces and the opposition track, the players' engine (factories, data
  centres, processors, track income, resources, cards in hand and built), unpaid threats,
  and a forecast of the coming ravages (the ravage- and build-slot cards are public).
  Every weight is a parameter: `w.<name>=V` in an agent spec.
  The 007 tuning run re-fit two defaults to the tuned ruleset: `pool` 12 → 6 and
  `opposition` 3 → 6. With the old values, on the larger tuned pool, beam bought back
  corruption instead of purging and ran out the deck (58% won against greedy's 94%).

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time: each scored by the heuristic after it, its own choices resolved best-first. A pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2 |
| `beam` | Beam search over the rest of the round's action sequences, scored at the start of the next round (the end-of-round preview); plans once, follows the plan while the table matches, replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 (levels of an action's own choices resolved before scoring it; with 1, placing a processor, which asks a track and then a hex, looks worthless and beam stops growing its engine) |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each continued by `rollouts` greedy playouts for `horizon` more rounds on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode** (008): income (tracks, income buildings, rewards) less building upkeep, is worth `w.horizon` × the resource's weight × the rounds left, and production (working factories' credits, processors' compute) `w.production` of that; the rounds left are the adversary deck's clock, cut short by when the team is on course to win (opposition still needed at its rate so far, `w.opp_rate`, `w.win_lag`) and discounted by danger (`w.risk` per unpaid threat per country, pool ≤ 4, population near the loss line); a building without income earns `w.ability` per round left; a card in hand counts by the chance it can be paid for before the end (credits and antimatter, from what the seat has and earns, factory credits included; `w.locked` for tier 2 not yet unlocked; `w.insight_wait` 1 also queues the hand for insight, but as written it made playing any card look costly and cut wins to 4%, so it is off); resources fade when fewer than `w.spend_horizon` rounds are left; and the **deck clock** (011, `w.clock`): a penalty per round the team, at its opposition rate so far, would still be short of the top level when the deck runs out. Searched like greedy by default; `width` > 0 makes it a beam over the round, `candidates` > 0 adds rollouts. | `width` 0, `samples` 1, `choices` 2; beam: `depth` 120, `branch` 0, `candidates` 0, `rollouts` 2, `horizon` 1. Weights: `horizon` 1, `hand` 1, `hand_cost` 0.2, `built` 1, `built_cost` 0, `risk` 0.1, `ability` 0.5, `spend_horizon` 2, `locked` 0.5, `opp_rate` 2, `win_lag` 1, `production` 0, `insight_wait` 0, `clock` 30 |

Any agent can use horizon mode (`greedy:w.horizon=1,...`); `engine` only changes the
defaults. Agent sweeps with strategy health (win %, buildings and tier-2 cards played per
game, the top buildings) go through `scripts/tune/engine_sweep.py`; it logs to
`out/tune/engine.jsonl`, and `AIC_SIM` points it at another binary (an absolute Windows
path; build one with `CARGO_TARGET_DIR=target/alt` to sweep while editing). Measured: 48
games at difficulty 4 on 12 threads take about 45 s for greedy and `engine`, 2 min with
`samples=3`, 4 min for `engine:width=4`, and longer with rollouts.

Spec syntax: `kind[:key=value,...]`, for example `beam:width=12`,
`rollout:rollouts=8,horizon=3`, `greedy:w.pool=20`. `aic-sim run ... --agent SPEC` plays
any of them; an experiment lists several.

## Experiments

```sh
S=target/release/aic-sim
$S experiment experiments/ladder-v0.json                    # -> out/experiments/ladder-v0/
$S experiment experiments/smoke.json --out out/smoke        # a quick check (6 seeds)
$S experiment experiments/ladder-v0.json --games 20 --seed 1000 --threads 6
```

**Tuning sweeps** (`scripts/tune/`, from the repo root): `sweep.py` runs `aic-sim run --json`
once per variant and agent and prints one row each (win %, rounds, adversary operations,
corruption added, how games ended), appending every result to `out/tune/log.jsonl`.
`log_table.py TAG...` turns logged rows into a markdown table, and `trace_summary.py`
summarises one `--trace` game round by round (adversary pieces, pool, opposition, plays).

```sh
python scripts/tune/sweep.py --agents "greedy;beam" --games 64 --tag try1 \
    --var "base=" --var "pool8=adversary.pool_per_country=8;cards.keep_cost=2"
target/release/aic-sim run configs/baseline.json --games 1 --seed 3 --agent greedy --trace 2>/dev/null \
    | python scripts/tune/trace_summary.py
```

The 007 run's log, change log and proposals: `experiments/tuning-007.md`. The 008
engine agent, its sweep and the building-engine verdict: `experiments/engine-008.md`;
rerun `experiments/engine-agent-008.json` (~28 min alone on 12 threads, 2,400 games:
greedy-search agents take 6–16 s a game each with every core busy) to judge a payback
lever.

**Payback sweep (009)**, write-up `experiments/payback-009.md`. Config levers are
plain specs: `experiments/payback-009-singles.json` (13 variants × d0, d4 × greedy,
engine × 96 seeds = 4,992 games, **46 min** alone on 12 threads) and
`experiments/payback-009-combos.json`. Levers that config can't reach (card costs and
income, tier-2 costs, the AI tracks) run as **content patches** built into separate
binaries from a copy of the repo under `out/`. Committed content is never touched:

```sh
python scripts/payback/patch_content.py --list                      # the patches
bash scripts/payback/build_patches.sh insight_inc econ_half+insight_inc   # -> out/patch-bin/<patch>.exe, ~1 min each
bash scripts/payback/run_content.sh insight_inc insight_inc:insight_inc+keep0  # <binary>:<spec> in experiments/payback-009-content/
python scripts/payback/summarize.py out/payback-009-singles out/payback-009-content/*/ --picks 20
target/release/aic-sim run configs/baseline.json --games 1 --seed 300041 --agent engine --trace 2>&1 \
    | python scripts/payback/resources.py                           # unspent resources by round
```

One content experiment (384 games) takes about 5 minutes alone. Records from a patched
binary carry its content hash, so they replay only on a binary built from the same
patch (main's `aic-sim replay` fails on them, typically "unaffordable"). Running two or three experiments at once
gives the same total throughput (about 1.3 games/s for greedy + engine), and even
`tail` on a log can take 30+ s under that load. A background waiter killed at its time
limit can leave a `bash.exe.stackdump` in the repo root: delete it.

**Proposals (011)**, write-up `experiments/proposals-011.md`: 007's accepted proposals
as variants and new defaults (rules r4). Screens (no probes, greedy):
`experiments/screen-011-difficulty*.json` (512–1,024 games, 9–16 min each with another
run beside it) chose the new level 1 and 3 effects; `screen-011-clock*.json` ran on
009's `t2_half+t2_noam` patch binary (`bash scripts/payback/build_patches.sh
t2_half+t2_noam`, then `out/patch-bin/t2_half+t2_noam.exe experiment ...`; 192–480
greedy + engine games, 7–12 min). The measurement, `experiments/proposals-011.json`
(10 variants × d0–d4 × greedy, engine × 64 seeds = 6,400 games, no probes), was
stopped unfinished: 2,109 games in 28 minutes alone on 12 threads (about 1.25 greedy +
engine games a second), so about 85 minutes for the whole grid. An experiment writes
nothing until its grid finishes, so a stopped run leaves no results.

An experiment is configs × variants × difficulties × agents over a seed range, played
on every core (one game at a time per worker, results sorted, so nothing depends on the
thread count; every game is replay-checked). The spec (`experiments/*.json`):

| Field | Default | Meaning |
|---|---|---|
| `name`, `note` | | Report title and a line on what it is for. |
| `configs` | | Config files. |
| `variants` | `[{"name": "base", "set": []}]` | Named override sets in `--set` syntax, each applied to every config. |
| `difficulties` | `[0]` | `adversary.difficulty` levels. |
| `agents` | | Agent specs, weakest first. |
| `games`, `first_seed` | `first_seed` 0 | Seeds per cell. `--games`/`--seed` override. |
| `max_actions` | `5000` | Per-game cap (`timeout`). |
| `winprob` | `{"games": 0, "rollouts": 8, "agent": "greedy"}` | **Off unless set.** Win probability by round: for the first `games` seeds of each cell, `rollouts` playouts by `agent` from a guess of the table at the start of each round. `--winprob N` sets `games`. |
| `closeness` | `{"every": 0, "gap": 2.0}` | **Off unless set** (011; it was on, every 1). Probe every n-th real choice: 1-ply greedy scores of every option; close when the best two are within `gap` heuristic points (about credits). `--closeness N` sets `every`. |
| `records` | `4` | Records saved per category. |

**Expensive measures are opt-in: screen first, probe finalists.** Win-probability
probes cost about 5× an experiment (007's `tune-after`: 45 minutes probed, 9–12
without), and closeness about one greedy decision per probed decision. Both are off
unless a spec or the command line turns them on. The two-stage pattern:

```sh
$S experiment experiments/x.json                                     # 1. screen: every variant, no probes
$S experiment experiments/x.json --variants keep2,uplinks \
    --winprob 10 --closeness 1 --out out/experiments/x-finalists     # 2. probe only the finalists
```

`--variants A,B` keeps only those variants of the spec (its first variant, the base,
is always kept, first, so the comparisons still have their reference). The report's
*Run* section says which measures were off.

The output directory (`--out`, default `out/experiments/<name>/`) holds `report.md` (the
cycle report: findings, then the tables), `results.json` (`aic-experiment/1`: the
spec, every cell's and group's measures, the card table, the records), `games.csv` (one
row per game) and `cards.csv` (per agent, difficulty and card), and `records/`.

**Measures** (all read from the public state or the actions, so the engine records
nothing extra):

- **Win rate** by agent × difficulty (± standard error), and the **skill gap**: the top
  tier's win rate minus the bottom tier's (each tier by its best agent, so a budget sweep
  of one kind is one rung), also on **survival**: the round a loss came in; a win counts
  its rounds plus the adversary cards left plus 2, so any win ranks above any loss.
  Survival is the ladder's signal while few games are won. *Ordered* means no tier does
  worse than the one below it.
- **Decision closeness**: forced decisions (one legal action), and of the real choices
  the share whose best two options are within `gap`.
- **Pick and play rates** per card: offered (in a draft packet), mean pick position
  (every dealt card is drafted by someone, so how early it goes is the signal), kept per
  offer, played per card gained (kept or drawn; per game for unique AI cards, which
  start in hand).
- **Build variety**: distinct end-of-game builds (each seat's AI and buildings) and the
  mean Jaccard distance between two games' building sets, over all games and over wins.
- **Win probability by round** from the playouts; checkpoints that are uncertain
  (0.1 < p < 0.9); early lock-in (the result was decided, p ≤ 0.05 or ≥ 0.95 to the end,
  by round 2).
- **Margin**: on a win, adversary cards left (rounds to spare) and pool left; on a loss,
  the adversary pieces over the current opposition level's bar.
- **Game length**, ends by reason and round; **adversary operations** per round (mean,
  max) and at setup.
- **Help and counters**: gifts, payments and withdrawals per game; threats countered of
  those dealt.
- **Strategy health** (008), per cell × agent: buildings played per game (any card of
  kind `building`, AI uniques included), tier-2 cards played per game, cards played per
  game, and the share of wins with 2 or more buildings played (and standing at the
  end). A findings line per difficulty says whether the top agent plays buildings and
  tier 2. *Top agent*: the highest win rate at the spec's highest difficulty (ties to
  the later agent).
- **Head to head against greedy** (008): every agent against plain `greedy` (or the
  first greedy-kind agent) on the same seeds, per cell: won by both / only the agent /
  only greedy / neither, the win-rate difference ± the standard error of the per-seed
  difference, an exact two-sided sign test on the seeds only one of them won, and a
  verdict (*beats greedy* only when ahead with p < 0.05).
- **Variants against the base** and **removing a module** (008), when a spec has
  several variants: each variant's win rate against the first (the base), paired on
  seeds; for variants that switch a module off (`<module>=off`,
  `<module>.variant=off`), the base minus the off-variant: what the module is worth.
- **Luck share**: a two-way split of outcome variance over seeds × agents within each
  config/variant/difficulty: the seed's share (luck), the agent's (skill), the rest
  (interaction and the agents' own randomness). Raw sums-of-squares shares, no
  degrees-of-freedom correction. On win/loss and on survival.

**Timing** (v0 baseline, 2 countries, release build, this 12-thread machine; games last
3–5 rounds, about 85–120 decisions). One game on one thread:

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
games) 30 s. The per-cell split (agent, probes, total) is in each report's *Timing*
table and in `games.csv`.

**Timing on the tuned baseline** (007: games last 7–8 rounds, about 200 decisions; 12
threads busy, so per-game times are under load):

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

**Background runs:** the tool runner stops a background command after 10 minutes unless
it is given a longer timeout. Windows can't replace `target/release/aic-sim.exe` while
an experiment runs it: copy the exe aside to run one build while rebuilding another.

**Saved records** (open in the viewer, or `aic-sim replay`), up to `records` each, with
the reason in the record's `note`: *close losses* at the top difficulty (fewest pieces
over the bar, then the longest games); *early lock-ins* (decided by round 2 in games of
3+ rounds, from the win-probability playouts); *surprising wins* (by the weakest agent,
after the playouts gave them ≤ 0.25, or below the top agent at the top difficulty).
"Weakest", "top" and "stronger first" are by **measured** strength (011), not ladder
tier: agents ranked by win rate at the top difficulty on the base variant (the same
reading as the strategy-health *top agent*); equal win rates share a rank, so with all
agents tied nobody is "weakest".
Files are named `<category>-c<cell>-<config>-<variant>-d<difficulty>-<agent>-seed<S>.json`;
a rerun into the same directory replaces them.
