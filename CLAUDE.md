# CLAUDE.md

The standing contract for every executor working in this repo.

## What this repo is

**AI Complete**, a board game being turned into a digital version.

- The rules must stay playable as a printed board game. A digital-only mechanic
  (hidden bookkeeping, timing a human can't track) is a design change, not an
  implementation detail: flag it rather than build it.
- Intended shape: a **Rust core** that simulates the rules, with clients that render
  it. The first client is **wasm + a JS front-end**. Clients render and take input;
  rules live in the core.
- Design is still being gathered. The engine skeleton exists (core, sim, wasm), but the
  game rules don't: the modules are placeholders. Don't describe or document code as
  if it does more than it does.

## Engine rules

- The core invariants (no I/O, no clock, no global RNG, no hash-order leaks, serde
  state, exact replay, native = wasm) and how to add a module, variant or card are in
  [docs/architecture.md](docs/architecture.md). Keep them.
- Tunable numbers live in configs, never in code. A new mechanic is a module; another
  way of doing one is a variant.
- Any change to how games play out: bump `RULES_REVISION` and update the golden
  fingerprint (`tests.rs` and `scripts/wasm-smoke.mjs`).
- Every rules change updates [docs/rules.md](docs/rules.md) in the same commit.

## Working rules

- **Commit to the branch you are on.** Leave nothing uncommitted.
- **Times are local, 12-hour clock** (2:45pm), in reports, docs and progress lines.
- **Background any step that takes more than ~30 seconds** (builds, tests, servers),
  and wait on the real process — the `cargo`/`node`/test binary — never on a
  launcher wrapper that returns before the work is done.
- **LF line endings.** `.gitattributes` sets `* text=auto eol=lf`; don't override it.
- **Operational facts go into `README.md`**: how to build, run and test, measured run
  times, ports, gotchas. Never leave one only in a report.

## Report

Write the report to the path conductor's preamble gives, starting with the header
it specifies:

```
status: done | partial | red
blocks: <what this blocks, or none>
carry: <what state.md should change, or none>
slow: none | <component> ~<time>, <why>, <what would fix it>
```

- Soft target about 60 lines.
- Findings, decisions and surprises only — no process narration.
- Mention tests only for a red you couldn't fix.
- Answer every amendment and message by number.
- End your final message with `Report written to <path>`.
