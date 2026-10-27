# ai-boardgame

**AI Complete**, a cooperative board game about AIs, being turned into a digital version.
The rules live in a pure, deterministic Rust core that simulates them. A native simulator
and a wasm build for a JS front end sit on top of the core, and neither holds any rules.

- [docs/principles.md](docs/principles.md): design pillars, difficulty targets, lenses,
  measures.
- [docs/rules.md](docs/rules.md): what the code implements. It is currently a
  **placeholder loop**, not the game.
- [docs/architecture.md](docs/architecture.md): modules, configs and tunables, the
  invariants, and how to add a module, variant or card.
- [CLAUDE.md](CLAUDE.md): the working contract for executors.

## Layout

| Path | What it is |
|---|---|
| `crates/aic-core` | The rules engine. No I/O, no clock, seeded per-purpose RNG streams in the state, serde state, exact replay. `Rules::new(config)` then `new_game` / `legal_actions_into` / `apply_mut`, plus the convenient free functions `new_game`, `legal_actions`, `apply`. |
| `crates/aic-core/src/modules/` | One file per module (`income`, `spread`, `actions`, `victory`), with the registry in `mod.rs`. |
| `crates/aic-sim` | Native CLI: seeded random-agent games over a seed range, one config or two side by side, with every game replay-checked. |
| `crates/aic-wasm` | wasm-bindgen bindings, JSON strings in and out. |
| `content/` | Data embedded at compile time: `map.json` (country piece, layouts for 1–5 countries) and `cards.json` (cards built from effect primitives). |
| `configs/` | Game configs: modules, variants, every parameter. `baseline.json` and `clean_land.json` differ only in the income variant and one spread tunable. |
| `scripts/wasm-smoke.mjs` | Plays one game through the wasm package in Node. |

Core source map: `rules.rs` (config compilation, round/phase loop, apply), `module.rs`
(the module framework: `ModuleDef`, `VariantDef`, `Module`, `Cx`), `config.rs`,
`content.rs`, `map.rs` + `hex.rs` (axial hexes), `state.rs`, `action.rs`, `event.rs`,
`rng.rs`, `driver.rs` (`play`/`replay` with an `Agent`, `GameRecord`), `tests.rs`.

## Toolchain

- Rust stable (built with 1.98.1), target `wasm32-unknown-unknown`
  (`rustup target add wasm32-unknown-unknown`).
- wasm-pack (`cargo install wasm-pack --locked`); Node (built with 24) for the smoke test.

## Build and test

```sh
cargo build --workspace
cargo test --workspace                             # ~25 s incl. build; the tests ~14 s (debug)
cargo build --release -p aic-sim                   # ~12 s
wasm-pack build crates/aic-wasm --target web       # ~20 s cold, ~9 s warm -> crates/aic-wasm/pkg (git-ignored)
node scripts/wasm-smoke.mjs                        # needs the wasm build; checks native = wasm
```

`Cargo.lock` started as a copy of corelight-engine's lock, so dependency versions match a
known-good wasm build.

**Rules changes:** the `golden_game` test pins the final state of one fixed game. If a
rules change is intended, bump `RULES_REVISION` in `crates/aic-core/src/lib.rs` and
update the fingerprint in both `tests.rs` and `scripts/wasm-smoke.mjs`. The revision is
recorded with every sim result, so old and new results are never mixed up.

Git Bash on this machine mangles backslashes inside heredocs, even with a quoted
delimiter. Write scripts with the editor, not with `cat <<'EOF'`.

## Simulator

Always run the sim in release.

```sh
S=target/release/aic-sim
$S compare configs/baseline.json configs/clean_land.json --games 2000
$S run configs/baseline.json --games 500 --set spread.variant=cluster --set spread.cap=4
$S compare configs/baseline.json configs/baseline.json --set-b victory.overrun=14   # one tunable
$S compare configs/baseline.json configs/baseline.json --set-b spread=off           # remove a module
$S run configs/baseline.json --set income.variant=clean_land --set income.base=1 --set income.per_clean=4
$S run configs/baseline.json --games 1 --trace     # first game as JSON lines
$S run configs/baseline.json --json                # machine-readable summary
$S show configs/baseline.json                      # resolved config, hash, every tunable
$S modules                                         # registry: modules, variants, params, phases
```

| Option | Default | Meaning |
|---|---|---|
| `--games N` / `--seed S` | `200` / `0` | Seeds `S` to `S+N-1`, one game per seed per config (any `u64`, up to `u64::MAX`). |
| `--set K=V` | | Override for every config. Repeatable. `countries=N`; `<module>.<param>=V` with `V` as `3` or `1,2,3`, or empty to unset; `<module>.variant=NAME` (keeps the parameters the new variant shares, drops the rest); `<module>=off` (removes the module and its phases). |
| `--set-a K=V` / `--set-b K=V` | | Override for config A or config B only. |
| `--max-actions N` | `100000` | Per-game action cap. A capped game ends as `timeout`. |
| `--threads N` | all cores | Results don't depend on it: agents are seeded per game. |
| `--json` | off | Print the summary as JSON: every field below, plus the diff, agent, `max_actions` and rules revision. |
| `--trace` | off | Print the first game of each config: every action (with its option count) and event. Goes to stderr with `--json`. |

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

Measured on this machine: about 0.03–0.05 ms per game, replay included. 2 × 100,000
games take about 7 s.

The random agent picks uniformly among all seats' legal actions, passes included. Treat
it as a floor, not as a model of a weak team.
