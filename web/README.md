# web: the AI Complete viewer

A debug-first browser client on top of `aic-wasm`. Vite + TypeScript, no UI framework.
It renders the engine's **generic view** and offers only what `legal_actions` returns;
it knows no rules. See [../docs/architecture.md](../docs/architecture.md#the-generic-view)
for how a module shows up in it.

## Build and run

Needs Node (built with 24), Rust with the `wasm32-unknown-unknown` target, and
wasm-pack (see the root README).

```sh
cd web
npm install                     # once
npm run dev                     # builds the wasm, then serves http://localhost:5173/
npm run dev -- --port 61870     # another port
npm run wasm                    # only rebuild web/src/wasm/ from crates/aic-wasm
npm run check                   # typecheck
npm run build                   # wasm + typecheck + static site in web/dist/
npm run e2e -- <record.json>    # browser check, dev server must be running (see below)
```

- **The wasm build step:** `scripts/build-wasm.mjs` runs `wasm-pack build
  crates/aic-wasm --target web --release` into `web/src/wasm/`, which is git-ignored.
  It takes about 20 s cold and 9 s warm. Set `AIC_WASM_PROFILE=dev` for an unoptimized
  build.
- **Engine changes while the dev server runs:** editing anything under `crates/` or
  `content/` rebuilds the wasm and reloads the page. Editing or adding a file in
  `configs/` reloads the page. The configs are read at build time
  (`import.meta.glob`), so a new config file appears in the picker after the reload.
- The page keeps the last good engine if a rebuild fails; the error is in the dev
  server's terminal.

## Using it

- **Play:** pick a config (any file in `configs/`) and a seed, then *New game*. Each
  seat's panel lists that seat's legal actions. Any seat may act in any order during a
  simultaneous phase, and one person plays every seat. An action with a hex target
  arms a hex pick: click a highlighted hex, or press Esc to cancel. *Pass every seat*
  passes all the seats still to act. *Random action* (R) and *Random to end* are for
  quick debugging.
- **Undo** (Ctrl+Z) drops the last action. Every state is a replay of config + seed +
  actions, so undo never has to reverse anything.
- **Timeline:** ⏮ ◀ ▶ ⏭, the slider, ← → Home End, and Space to auto-play. Clicking an
  event-log entry jumps to that step. Acting from an earlier step drops the later
  actions, after asking first.
- **Hover** a hex, token, value, card or action button for its details. Hovering a log
  entry outlines the hexes it touched. Hexes changed by the shown step have an amber
  outline.
- **Data tabs:** *Event log*, *Raw state* (the engine's JSON, for anything the board
  doesn't draw), *View* (the generic view JSON) and *Config* (resolved config and hash).
- **Saved session:** the current game (config, seed, actions, cursor) is kept in
  `localStorage` and replayed on load.

## Game records

A record is the config, seed and action list, plus identity (hashes, engine version,
rules revision) and the saved outcome and final fingerprint. The engine writes it
(`aic_core::SavedGame`, format `aic-game-record/1`, one action per line):

- **Watch a simulated game:** `aic-sim run ... --save DIR` writes records (see the root
  README). Use *Open record…* or drop the file on the page. The game opens at step 0;
  step through it with the timeline. A record whose replay differs from what was
  saved (another rules revision or content) loads with a warning.
- **Bug report:** *Record / bug report…* holds the current game as a record, with an
  optional note. *Copy this game* copies it; *Copy up to shown step* stops at the
  cursor. Paste a record there and *Load* it to replay it. `aic-sim replay <file>`
  checks a record natively.

## Browser check

`npm run e2e -- <record.json>` (`scripts/e2e.mjs`) drives headless Edge or Chrome
over the DevTools protocol, with no extra packages. Point it at the dev server with
`AIC_URL` (default `http://localhost:5173/`), and at a browser with `AIC_BROWSER` if it
isn't at the usual path. It plays a game to the end by clicking the UI, then checks
the following:

- undo;
- the restored session;
- the record dialog;
- hover details;
- no page scroll at 1366×768, 1920×1080 and 2560×1440;
- loading the record and stepping through it.

Screenshots go to `web/e2e-out/` (git-ignored). A run takes about 10 s.

## Source

| File | What it does |
|---|---|
| `src/engine.ts` | Typed wrapper over the wasm API: view, record and event shapes. |
| `src/session.ts` | `Timeline`: config + seed + actions with every replayed state, cursor, undo, localStorage. |
| `src/board.ts` | SVG hex board drawn from the view. |
| `src/format.ts` | Generic labels for actions and events, and token colours. |
| `src/main.ts` | Panels, action buttons, log, timeline, hover card, record dialog. |
