# aicomplete

**AI Complete**, a cooperative board game about AIs, being turned into a digital version.
The rules live in a pure, deterministic Rust core that simulates them. A native simulator
and a wasm build for a JS front end sit on top of the core, and neither holds any rules.

- [docs/principles.md](docs/principles.md): design pillars, difficulty targets, lenses,
  measures.
- [docs/rules.md](docs/rules.md): what the code implements: **rules revision 9** (018: the structural fixes and calibration, troops used up when they strike; 017: the adversary project deck, the Discord troop advance, one-shot Alignment cards; on 016: presence, the adversary ladder with nodes, factories and hardware, the production/war clock, the purge), on the 014
  board model, Militarist against Builder and Protector, 1–4 countries, with every
  default and deviation.
- [docs/ruleset-v0.md](docs/ruleset-v0.md): the v0 design draft as handed over.
- [docs/architecture.md](docs/architecture.md): modules, configs and tunables, the
  invariants, and how to add a module, variant or card.
- [CLAUDE.md](CLAUDE.md): the working contract for executors.

## Layout

| Path | What it is |
|---|---|
| `crates/aic-core` | The rules engine. No I/O, no clock, seeded per-purpose RNG streams in the state, serde state, exact replay. `Rules::new(config)` then `new_game` / `legal_actions_into` / `apply_mut`, plus the convenient free functions `new_game`, `legal_actions`, `apply`. |
| `crates/aic-core/src/modules/` | One file per rules area (`board`, `adversary`, `threats`, `alignment`, `economy`, `ai`, `cards`), shared code (`common`, `effects`), read-only forecasts for agents and clients (`plan`: the adversary's next build), the registry in `mod.rs`. |
| `crates/aic-sim` | Native CLI: seeded games over a seed range, one config or two side by side, with every game replay-checked; the agent skill ladder (`agents/`) and the experiment harness (`experiment.rs`, `report.rs`). |
| `experiments/` | Experiment specs, and `results/<name>/`: committed cycle reports. Game records are kept only where they still replay (`screen-014-board/records/`); `payback-009-records/` is r3 history and does not replay. `archive/`: specs the binary refuses today. [`history.md`](experiments/history.md): past sweeps, recipes and timings. |
| `crates/aic-wasm` | wasm-bindgen bindings, JSON strings in and out. |
| `content/` | Data embedded at compile time: `map.json` (piece shape, layouts for 1–4 countries, the four printed country pieces with their cities, ocean, setup buildings and megaproject), `cards.json`, `threats.json`, `adversary.json`, `ais.json`, `alignment.json` (the Unity and Discord decks), all built from code primitives. |
| `configs/` | Game configs: modules, variants, every parameter. `baseline.json` is the 016 board with 017's projects, r9 (2 countries, difficulty 0; 018 calibrated it on quick screens: every number still a candidate); `shared_row.json` differs only in the threat variant (and name): it is derived, so after editing the baseline run `python scripts/configs/shared_row.py`; a core test fails until you do. Other country counts and difficulties: `--set countries=N`, `--set adversary.difficulty=N`. Off-variants (rules.md §9): `--set threats.variant=off`, `alignment.variant=off`, `cards.variant=off`. Alignment's options (rules.md §5): `alignment.variant=still`, `alignment.momentum_every=2`, `alignment.antimatter_discord=0`. Optional modules switch off whole (016 trimmed `requires`): `--set threats=off`, `cards=off`, `economy=off`. Opt-in variants: `cards.variant=free_keep` (+ `cards.free_keeps=1`). The 013 configs (`v0_untuned`, `clusters`, `assets`) went with the old board. |
| `scripts/` | `wasm-smoke.mjs`: plays one game through the wasm package in Node (current). `configs/shared_row.py`: regenerates `configs/shared_row.json` from the baseline (current). `board/`: `screen_018.py` (018), the r9 calibration summary with the strategy-health gate (current); `screen_017.py` (017), the r8 screen summary (reads r9 too); `screen_016.py` (016), the r7 screen summary (reads r8 too); `screen_summary.py` (014), the board screen summary (current). `alignment/` (013): the Alignment screen summary (current: the alignment counters it reads still exist). `bench/` (012): `same_results.py`, compares two experiment outputs (current). `tune/` (007–008): `sweep.py`, `log_table.py` and `engine_sweep.py` (`sweep.py` still runs on r9: 018 used it). `payback/` (009): content patches and summaries for r3's payback sweep, not checked against r6 (see [experiments/history.md](experiments/history.md)). |
| `web/` | Browser viewer (Vite + TS on `aic-wasm`): play any config and seed, watch and step through sim game records, copy bug reports. See [web/README.md](web/README.md). |

Core source map: `rules.rs` (config compilation, round/phase loop, apply, choice
replay), `module.rs` (the module framework: `ModuleDef`, `VariantDef`, `Module`, `Cx`,
`World`), `config.rs`,
`content.rs`, `map.rs` + `hex.rs` (axial hexes), `state.rs`, `action.rs`, `event.rs`,
`rng.rs`, `hash.rs` (FNV-1a: config and content hashes, RNG stream ids, state
fingerprints), `driver.rs` (`play`/`replay` with an `Agent`, `GameRecord`), `view.rs`
(the generic view clients draw), `record.rs` (`SavedGame`, the game record format), `tests/` (`mod.rs`: helpers,
`variant_matrix`, `golden_game`; one file per area: config, map, setup, adversary,
plan, board, threats, alignment, cards, determinism, view).

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
- While Rust or content files are being edited, the dev server rebuilds the wasm and
  reloads mid-run, which breaks e2e (`Cannot read properties of undefined (reading
  'view')`). Check against a snapshot instead: `node scripts/build-wasm.mjs && npx vite
  build && npx vite preview --port $PORT --strictPort`.
- `node scripts/shots-017.mjs` (with `AIC_URL` and `AIC_SHOTS=<dir>`, ~30–90 s) plays
  baseline seeds and saves viewer shots of the adversary project row mid-fill and after
  a project resolved (017).
- e2e at 1366x768 (018): every check passes. `4 countries: no page scroll` has the board
  340 px tall against 307 needed (017: 260 to 281), `board fills its pane` 382 to 406
  against 346 (it varies with the game the run plays). Below 820 px of height
  (`@media (max-height: 820px)` in `web/src/style.css`) the strips around the board take
  tighter lines and the adversary cards one line of text.
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
cargo test --workspace                             # fast lane: ~4 s once built, ~13 s after an edit in aic-sim (rebuild included); after every change
cargo test-all                                     # full run, slow lane included: ~30 s once built, ~1 min after an edit (rebuild included); before every commit
cargo test-slow                                    # the slow lane alone: ~28 s (most of it the beam game, ~18 s, and the rollout game, ~10 s)
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
machine with nothing else heavy running (lanes remeasured in 015, after the beam's
plan check stopped replanning on every alignment-deck guess); the first build of the test profile takes about 40 s.

- The test profile is optimized (`[profile.test] opt-level = 1` in `Cargo.toml`;
  debug assertions and overflow checks stay on): unoptimized, the same tests took
  4 min. `opt-level = 2` built 6 s slower and ran no faster.
- `cargo test` runs test binaries one after another, so the fast lane's wall clock
  is roughly the sum of each binary's slowest test. Today that is
  `experiment_streams_games_and_resumes` and `experiment_reports_comparisons` (~1 s
  each).
- Per-test times: `RUSTC_BOOTSTRAP=1 target/debug/deps/<binary>.exe --include-ignored -Z
  unstable-options --report-time` (the binary paths are printed by `cargo test --no-run`).

`Cargo.lock` started as a copy of corelight-engine's lock, so dependency versions match a
known-good wasm build.

**Rules changes:** the `golden_game` test pins the final state of one fixed game
(baseline, seed `u64::MAX`, action *k* = legal action `(k × 7919) mod count`; "always the
first action" can now pay and withdraw a threat counter forever). If a rules change is
intended, bump `RULES_REVISION` in `crates/aic-core/src/lib.rs` and update the
fingerprint in both `crates/aic-core/src/tests/mod.rs` (`GOLDEN_FINGERPRINT`) and
`scripts/wasm-smoke.mjs`. The fingerprint covers the whole final state, config `name`
included. The revision is recorded with every sim result, so old and new results are
never mixed up.

Git Bash on this machine mangles backslashes inside heredocs, even with a quoted
delimiter. Write scripts with the editor, not with `cat <<'EOF'`.

## Simulator

Always run the sim in release.

```sh
S=target/release/aic-sim
$S run configs/baseline.json --games 2000                          # the 014 board, 2 countries, difficulty 0
for n in 1 2 3 4; do $S run configs/baseline.json --games 500 --set countries=$n; done
$S compare configs/baseline.json configs/shared_row.json --games 2000  # threat variants
$S compare configs/baseline.json configs/baseline.json --set-b threats.variant=off  # off-variants
$S run configs/baseline.json --set alignment.variant=still --set alignment.momentum_every=2
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
| `--set K=V` | | Override for every config. Repeatable. `countries=N`; `<module>.<param>=V` with `V` as `3` or `1,2,3`, or empty to unset; `<module>.variant=NAME` (keeps the parameters the new variant shares, drops the rest); `<module>=off` (removes the module and its phases; today every module is required by another, so the config is rejected: use `<module>.variant=off`). |
| `--set-a K=V` / `--set-b K=V` | | Override for config A or config B only. |
| `--agent SPEC` | `random` | The team agent (see [Agents](#agents)), e.g. `greedy`, `beam:width=4`. Shown in the table header and in `--json`. |
| `--max-actions N` | `100000` | Per-game action cap. A capped game ends as `timeout`. |
| `--threads N` | all cores | Results don't depend on it: agents are seeded per game. |
| `--json` | off | Print the summary as JSON: every field below, plus the diff, agent, `max_actions` and rules revision. |
| `--trace` | off | Print the first game of each config: every action (with its option count) and event. Goes to stderr with `--json`. |
| `--save DIR` | off | `run`/`compare`: write the selected games as game records into `DIR` (created if needed), named `<config name>-<run\|A\|B>-seed<S>.json`, so A and B never collide. The paths are printed after the table, or listed under `saved` in each `--json` result with the match counts. A write failure exits 2. |
| `--save-when W` | `loss` | Which games to save: `all`, `win`, `loss`, `timeout`, or an end reason code (`supply`, `cities`, `deck_empty`, `unity`, `purge`, ...). Repeatable; a game matching any is saved. With only `--save-seed` given, no default: just those seeds. A reason code no game ended with gets a warning on stderr (likely a typo). |
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

Measured on this machine (014 board, 2 countries, random agent, 12:05am): about 1.5 ms
per game, replay included (every action and automatic phase clones the state for choice
replay); 2,000 games take about 3 s. Random games last about 5 rounds and all lose,
almost all on the adversary's ambassador supply (1,984 of 2,000; the rest on every city
corrupted).

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
  a guess from its own RNG (`agents/determinize.rs`): the player-card, threat and
  Unity/Discord decks are shuffled (contents are public, order is not); each hidden
  adversary-deck position gets an unseen printed card of its stage (stages are stacked and
  public; next round's card is face up, and the top cards stay put under Recon Sweep);
  every RNG stream is reseeded. The guess depends only
  on the public table and the agent's RNG; a test changes only hidden information and
  checks that decisions stay the same.
- **Own randomness.** Each agent's RNG is the game seed on a stream named after the agent
  kind, apart from every engine stream. Games replay from seed + actions, and results
  don't depend on `--threads`.
- **Heuristic** (`agents/eval.rs`, 014 board): win/loss; corrupted cities (`corrupted`
  30, plus `cities_danger` 60 per city short of 2 not corrupted), aligned cities
  (`aligned` 8); adversary ambassadors (`adv_ambassador` 3 each, `on_piece` 2 more on a
  player piece, base or factory, `on_factory` 6 more on a factory), the pressure on cities
  not yet corrupted (`pressure` 0.7 × `corrupted` × (ambassadors / threshold)², the
  threshold and the next build read off the face-up card; the build is the core's
  `plan::next_build`), troops (`troop` 12); the
  supply against the next build (`supply` 5 per ambassador of slack short of 4,
  `supply_danger` 40 per ambassador short: starved loses); the bar's blockers
  (`blocker` 2); Alignment (Unity and Discord tips made, each bucket's fill as a share of
  its next tip, the bar level, and each visible adversary project's fill as a share of
  its cost, `project` 8, 017); the players' board (presence
  4, nodes −6, owned factories 4 (an adversary-owned one −4), hardware 2, cities/bases/stations in reach 0.5, owned stations 5 (a corrupted one −5),
  megaproject cubes 2 (adversary's −1), factories 0.5, hardening 1, player ambassadors
  5 in supply (018: was 3, below their 6-credit cost, so agents never bought one) / 2.5 on the board, and toward freeing a corrupted city `freeing` 0.8 ×
  `corrupted` × share of `free_city_at`); track income, resources, cards in hand and
  built; unpaid threats. Every weight is a parameter: `w.<name>=V` in an agent spec. The
  014 weights are hand-set, not swept.

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time: each scored by the heuristic after it, its own choices resolved best-first. A pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2 |
| `beam` | Beam search over the rest of the round's action sequences, scored at the start of the next round (the end-of-round preview); plans once, follows the plan while the table matches, replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 (levels of an action's own choices resolved before scoring it; with 1, placing presence, which asks a hex, looks worthless and beam stops growing its engine) |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each continued by `rollouts` greedy playouts for `horizon` more rounds on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode** (008): income (tracks, income buildings, rewards) less building upkeep, is worth `w.horizon` × the resource's weight × the rounds left, and board production (aligned cities' energy, less what adversary ambassadors divert; presence compute is track income) `w.production` of that (014: 1 won 65% at d0 against 37% with 0 on the checkpoint's map; level within noise at d1–d3 on the map since edited); the rounds left are the adversary deck's clock (deck plus the face-up card), cut short by when the team is on course to win (Unity Sway still needed for the outright Unity win at its rate so far, `w.unity_rate_start`, `w.win_lag`) and discounted by danger (`w.risk` per unpaid threat per country, supply slack ≤ 2 against the next build, 2 or fewer cities not corrupted); a building without income earns `w.ability` per round left; a card in hand counts by the chance it can be paid for before the end (credits and antimatter, from what the seat has and earns, factory credits included; `w.locked` for tier 2 not yet unlocked; `w.insight_wait` 1 also queues the hand for insight, but as written it made playing any card look costly and cut wins to 4%, so it is off); resources fade when fewer than `w.spend_horizon` rounds are left; and the **deck clock** (011, `w.clock`): a penalty per round the team, at its Unity rate so far, would still be short of the outright Unity win (`alignment.unity_win`, 8 tips; with it 0, the last bar drop) when the deck runs out. Searched like greedy by default; `width` > 0 makes it a beam over the round, `candidates` > 0 adds rollouts. | `width` 0, `samples` 1, `choices` 2; beam: `depth` 120, `branch` 0, `candidates` 0, `rollouts` 2, `horizon` 1. Weights: `horizon` 1, `hand` 1, `hand_cost` 0.2, `built` 1, `built_cost` 0, `risk` 0.1, `ability` 0.5, `spend_horizon` 2, `locked` 0.5, `unity_rate_start` 2 (`opp_rate` still accepted), `win_lag` 1, `production` 1, `insight_wait` 0, `clock` 30 |

Any agent can use horizon mode (`greedy:w.horizon=1,...`); `engine` only changes the
defaults. Agent sweeps with strategy health (win %, buildings and tier-2 cards played per
game, the top buildings) go through `scripts/tune/engine_sweep.py`; it logs to
`out/tune/engine.jsonl`, and `AIC_SIM` points it at another binary (an absolute Windows
path; build one with `CARGO_TARGET_DIR=target/alt` to sweep while editing). Measured: 48
games at difficulty 4 on 12 threads take about 45 s for greedy and `engine`, 2 min with
`samples=3`, 4 min for `engine:width=4`, and longer with rollouts.

Spec syntax: `kind[:key=value,...]`, for example `beam:width=12`,
`rollout:rollouts=8,horizon=3`, `greedy:w.corrupted=40`. `aic-sim run ... --agent SPEC` plays
any of them; an experiment lists several.

## Experiments

```sh
S=target/release/aic-sim
$S experiment experiments/ladder-v0.json                    # -> out/experiments/ladder-v0/
$S experiment experiments/smoke.json --out out/smoke        # a quick check (6 seeds)
$S experiment experiments/ladder-v0.json --games 20 --seed 1000 --threads 6
$S experiment experiments/screen-014-board.json --quick     # scoping: shallow agents, 16 seeds, no probes
$S experiment experiments/screen-014-board.json --resume    # continue a stopped run from its games.jsonl
```

**Tuning sweeps** (`scripts/tune/`, from the repo root): `sweep.py` runs `aic-sim run --json`
once per variant and agent and prints one row each (win %, rounds, adversary operations,
corruptions and buildings destroyed per game, how games ended), appending every result
to `out/tune/log.jsonl`. `log_table.py TAG...` turns logged rows into a markdown table.

```sh
python scripts/tune/sweep.py --agents "greedy;beam" --games 64 --tag try1 \
    --var "base=" --var "supply8=adversary.supply_per_country=8;cards.keep_cost=2"
```

**Past sweeps** (007 tuning, 008 engine agent, 009 payback and its content patches,
011 proposals), their recipes and timings, and the 012 speed-up are in
[experiments/history.md](experiments/history.md). Their write-ups are
`experiments/tuning-007.md`, `engine-008.md`, `payback-009.md` and `proposals-011.md`.

**Alignment screen (013)**, `experiments/screen-013-alignment.json` (base, `no_am` and
`off` × d0, d4 × greedy, engine × 32 seeds = 384 games): **45 s** alone on 12 threads
with `--quick --games 32`. `python scripts/alignment/screen_summary.py DIR` turns its
`games.jsonl` into the Alignment table (tips per game per side, the round of each tip,
antimatter actions, cards resolved); committed with the report in
`experiments/results/screen-013-alignment/` (`alignment.md`). Tips by round come from
the counters `alignment.tip_<side>.rNN`, so `aic-sim run` shows them too.

**Board screen (014)**, `experiments/screen-014-board.json` (`c2` and `c3`, that is 2
and 3 countries, × d0, d4 × greedy, engine): `aic-sim experiment
experiments/screen-014-board.json --quick` (16 seeds a cell, 128 games), then `python
scripts/board/screen_summary.py DIR` reads its `games.jsonl` and `results.json` and
prints markdown: per cell the win rate, rounds, the round of the first new corruption,
cities corrupted at the end, destructions per war round, Unity and Discord tips,
standard actions and how games ended; then adversary ambassadors on the board by round,
and standard actions by kind. Committed with the report and records in
`experiments/results/screen-014-board/` (`board.md`).

**Calibration screen (018, r9)**, `experiments/screen-018-calibration.json` (`c2`, `c3`
× d0–d4 × random, greedy, engine; 480 games): `aic-sim experiment
experiments/screen-018-calibration.json --quick --out out/screen-018` (**~3 min** wall on
12 threads: r9 games run longer, 6–12 rounds, and c3 engine games go the full 11),
then `python scripts/board/screen_018.py out/screen-018`: per cell the win rate,
rounds, what was left on the board at the end (corrupted cities, nodes, troops: the
purge's blockers), tips and ends, then the strategy-health gate per game (presence,
factories per side and owned at the end, hardware made and spent, nodes made and
removed, compute blocked by a player ambassador on a node, Security Patch, cities
freed, cards). `margin.nodes_end` / `troops_end` in `games.jsonl` are new in 018.
Committed in `experiments/results/screen-018-calibration/` (`calibration.md`).

**Projects screen (017, r8)**, `experiments/screen-017-projects.json` (the same cells):
`aic-sim experiment experiments/screen-017-projects.json --quick --out out/screen-017`
(128 games, **~8 s** wall on 12 threads: r8 games are short, ~4 rounds at d0), then
`python scripts/board/screen_017.py out/screen-017`: per cell the win rate, rounds,
tips and ends, then per game projects resolved (cheap, big), adversary compute made and
blocked, Discord and Unity tips, troop advances and strikes, and the round of the first
project; then standard actions by kind. Committed in
`experiments/results/screen-017-projects/` (`projects.md`). Gotcha: on Windows a
Python script printing an en dash to a redirected stdout writes cp1252, not UTF-8;
`screen_017.py` reconfigures stdout to UTF-8.

**Presence screen (016, r7)**, `experiments/screen-016-presence.json` (the same cells
as 014's): `aic-sim experiment experiments/screen-016-presence.json --quick` (128 games,
**20 s** wall on 12 threads; d0 engine at 3 countries is the slowest cell, ~5 s a
game per worker), then `python scripts/board/screen_016.py DIR`: per cell the win rate,
rounds, tips and ends, then per game presence placed, node peaks per board, nodes made
and removed, factories built, destroyed and owned per side, hardware made, bought and
spent, adversary compute made, and standard actions by kind. Committed in
`experiments/results/screen-016-presence/` (`presence.md`).

**Specs the binary refuses today** (the rules or configs they name are gone) are in
`experiments/archive/`, listed with why in
[experiments/history.md](experiments/history.md). Every spec left in `experiments/`
runs on the 014 board. Committed results from r5 and earlier are history: their game
records do not replay.

**Speed.** The agents are 99% of a run: search clones game states by the hundred
thousand. `aic-sim` uses **mimalloc** (on 12 threads Windows' heap serialized the
clones), and searches call `Rules::apply_to` (one state copy, not two). To see the
split: every run prints it (`time: ...` on stderr, and in the report's *Timing*);
`cargo build --release -p aic-sim --features profile` adds the search's own split
(apply, legal, heuristic, determinization) with call counts. `python
scripts/bench/same_results.py DIR_A DIR_B` compares two experiment output directories,
timing aside: the check that a speed-up changed no game.

**Results as they go.** Every finished game is appended to `DIR/games.jsonl` (a header
line with the spec, engine, rules revision and content hash, then one game per line,
actions included), and the whole report is rewritten from the games so far every
`--report-every` seconds (default 60; files are written whole and renamed into place),
marked **Partial: N of M games** at the top. A stopped or killed run leaves both.
`--resume` (same spec, same `--out`) reads the stream back, replays each game to rebuild
it, plays only the games missing and reports them all; a different spec, edited config
file, engine, rules revision or content is refused. Without `--resume`, an earlier
stream in the directory is moved to `games.jsonl.bak`. A partial report that can't be
written (say, `games.csv` open in a spreadsheet, which locks it on Windows) is skipped
with a warning; the run goes on. Games are handed out seed by seed across all cells, so
a stopped run has every cell's first seeds.

**Fast by default (`--quick`).** Until the rules are close to final, scoping runs use
shallow agents and short runs (`docs/principles.md`): `--quick` keeps only shallow agents
(random, greedy and engine at their defaults; beam, rollout, `samples`/`depth` > 1 and
`engine:width` > 0 are dropped, with a note), caps seeds at 16 per cell unless
`--games` is given, and turns the probes off unless `--winprob`/`--closeness` is given.

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
cycle report: findings, then the tables), `results.json` (`aic-experiment/2`: the
spec, every cell's and group's measures, the card table, the records), `games.csv` (one
row per game) and `cards.csv` (per agent, difficulty and card), and `records/`, plus
`games.jsonl` (`aic-games/2`, written as the games finish; see *Results as they go*).
Format 2 (016) renamed `margin.points` to `margin.unity_tips` (the `games.csv` column
too) and made each card's counts an object (`offered`, `drafted`, `kept`, `played`,
`pick_sum`, `drawn`) instead of a 6-number array; a format-1 stream can't be resumed.

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
- **Margin**: on a win, adversary cards left (deck plus the face-up card: rounds to
  spare) and the adversary's ambassador supply left; on a loss, the adversary pieces over
  the current bar (`margin.level` is the bar level and `margin.unity_tips` the Unity tip
  count); every game, `margin.cities_corrupted_end` and `margin.cities_aligned_end`
  (also columns of `games.csv`, with the core's `adversary.corruptions`,
  `adversary.first_corruption_round`, `board.std_actions` and `board.war_destroyed`
  counters; every counter is in `games.jsonl`).
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

**Timing on the 014 board** (baseline, release, 12 threads): `smoke.json --quick` (24
games; beam dropped) 0.8–1.3 s wall, greedy 0.3–0.6 s per game at d0 (at the old difficulty-4
ladder d4 games ended in round 1; rebuilt, rules.md §3); `screen-014-board.json --quick` (128 games) 11–13 s wall, per game greedy
0.7 s / engine 1.1–1.4 s with 2 countries, 1.9–2.3 s / 3.2–3.7 s with 3. Probes add to
that: win probability costs `rollouts` greedy playouts per round of each probed game
(the bulk of a probed game's time), closeness about one greedy decision per probed
decision. The per-cell split (agent, probes, total) is in each report's *Timing* table
and in `games.csv`. Timings on older rules (v0, the 007 baseline) are in
[experiments/history.md](experiments/history.md).

**Background runs:** the tool runner stops a background command after 10 minutes unless
it is given a longer timeout. A background waiter killed at its time limit can leave a
`bash.exe.stackdump` in the repo root: delete it. Windows can't replace `target/release/aic-sim.exe` while
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
