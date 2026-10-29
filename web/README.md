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
npm run e2e [-- <record.json>] # browser check, dev server must be running (see below)
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

- **Play:** pick a config (any file in `configs/`; `baseline.json` is the default) and
  a seed, then *New game*. Each seat's panel lists that seat's legal actions. Any seat
  may act in any order during a simultaneous phase, and one person plays every seat.
  Growth and draft phases offer no pass: pick one of the buttons.
- **Choices:** when the rules wait for a choice, its question shows in the amber bar
  above the board. A choice answered with a hex highlights its hexes at once: click
  one. Any other answer is a button in that seat's panel. *Pass every seat* passes
  all the seats still to act. *Random action* (R) and *Random to end* are for quick
  debugging.
- **Reading the board:** hexes are filled by what they are (`kind`: city grey, military
  base khaki, open land pale green, ocean blue; key at the bottom left), with a band in
  the country's colour along each country border and the printed hex number on top.
  A military base carries a pennant in the colour of the country across its front
  (`facing`). Fronts are drawn in dark red: a dashed line along each land border
  between two countries and a dashed link between the two bases facing each other; on
  a war round (`board.war` = 1) they turn solid bright red and pulse, and the adversary row
  shows a **war round** badge. Each seat's colour is its country's.
  Players' pieces sit on the upper row of a hex: owner cube in the seat's colour (an
  aligned city or owned station), research station (teal dome), factory (brown),
  data centre (server rack in the seat's colour), hardening token (grey shield), corp
  units at a base (chevrons in the base country's colour) and player ambassadors (pawn
  in the seat's colour). The adversary's sit on the lower row, in purple: its owner cube
  (a corrupted city or station), its ambassadors (pawn) and its troops (dark
  spearhead). A number by a glyph is its count.
- **Around the board:** the Alignment track (the Unity bucket, blue, filling leftward
  and the Discord bucket, red, filling rightward on either side of the Undecided; each
  side's tip count with its next threshold; the Unity tips that win outright; the
  clear-board bar level and its win condition; adversary ambassadors and troops on the
  board), the Unity, Discord and ongoing cards resolved so far (hover a chip for its
  text) with what is left in each deck, and the loss conditions (adversary ambassador
  supply, corrupted cities out of all cities, adversary cards left); the adversary row
  (stage, difficulty, escalation, threat tier and fronts, then deck →
  next round's card, face up → this round's card → discard, each card with its stage,
  text and escalation/war marks); the countries strip with each country's name, its
  cities (clean, aligned, corrupted), adversary pieces and corp units, its megaproject
  card (one box per cube place, filled by owner: purple for the adversary, the seat's
  colour for a player; "completed" once done), its round flags (blackout, sabotage,
  shelter), its threat and counter progress (or the shared threat row). Country names
  (the printed pieces', from the view's `countries[].name`) also label the board and the
  hover text.
- **Actions:** besides cards, threats and gifts, each seat panel offers the board's
  standard actions (`standard`: place a data centre, build a factory or research
  station, remove an adversary ambassador, move an ambassador, gain an ambassador,
  align a city, harden a building) with their costs from the config, and one *sell*
  button per card in hand. A standard action that needs a hex asks for it next, as a
  hex pick.
- **Seat panels:** the AI board's name (hover it for its text, innate rule, slots and
  unique cards), resources with income, then the board's two tracks with every printed
  step: solid steps are uncovered, dashed ones still under data centres, the boxed one
  is the value in effect, and green **T2** / pink **+n AM** marks are the tier-2 unlock
  and antimatter income. Hover a step for its details. This is drawn from the generic
  `seats[].board` shape, not from AI-specific keys.
- **Globals** hold only game-wide tracks: modules keep per-seat and per-card
  bookkeeping out of them (`ViewOut::bookkeeping`). A `vars` track no module labels is
  folded under *Unlabelled tracks* below the piles; in the baseline there are none.
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

`npm run e2e [-- <record.json>]` (`scripts/e2e.mjs`; the record is optional) drives headless Edge or Chrome
over the DevTools protocol, with no extra packages. Point it at the dev server with
`AIC_URL` (default `http://localhost:5173/`), and at a browser with `AIC_BROWSER` if it
isn't at the usual path. It checks the board drawing (fills by hex kind, borders, base
pennants, fronts, every token stack drawn, owner colours distinct), the adversary row
with the face-up next card's text and the ambassador supply, the megaproject cards,
Alignment track and threats, country names, each seat's AI board with two tracks, and
that no bookkeeping var is among the globals, then plays a baseline game (seed 1) to
the end by clicking any action button (and a highlighted hex for hex choices), then
checks the following:

- hex choices highlight their hexes without a click;
- standard actions and selling a card were offered as buttons;
- war rounds are marked on the adversary row and the fronts;
- undo;
- the restored session;
- the record dialog;
- *Random to end* plays a new baseline game (seed 5) to an outcome;
- hover details;
- no page scroll at 1366×768, 1920×1080 and 2560×1440;
- a 4-country game (64 hexes) still fits at 1366×768 and 1920×1080;
- the shared threat row (`configs/shared_row.json`), if that config exists;
- loading the record and stepping through it, if one was given.

Screenshots go to `web/e2e-out/` (git-ignored): the start, draft, growth, first
choice, mid-game and end of the game, the layouts, four countries and the shared row.
With `AIC_SHOTS=<dir>` the mid-game and end-of-game shots are also saved there as
`viewer-mid.png` and `viewer-end.png`. A run takes about 15 s (12-20 s measured).

- **Against a built site:** `npm run build`, then `npx vite preview --port <p>
  --strictPort` serves `web/dist/` with no engine watcher, so the run can't be reloaded
  under it (see the first gotcha). After changing only `web/src/`, `npx vite build`
  (about 1 s, reuses `web/src/wasm/`) refreshes `dist/`.

- **Gotcha:** the dev server rebuilds the wasm and reloads the page whenever anything
  under `crates/` or `content/` changes. An e2e run during such a rebuild fails with
  `Cannot read properties of undefined`; run it again once the build is done. Any
  `.rs` edit under `crates/` counts, tests and `aic-sim` included, so while someone
  else edits the engine, run the e2e against `vite preview` instead.
- **Gotcha:** stopping a backgrounded `npm run dev` wrapper can leave the `vite`
  process serving the port; `--strictPort` then fails on the next start. Kill the
  process that holds the port.

## Source

| File | What it does |
|---|---|
| `src/engine.ts` | Typed wrapper over the wasm API: view, record and event shapes. |
| `src/session.ts` | `Timeline`: config + seed + actions with every replayed state, cursor, undo, localStorage. |
| `src/board.ts` | SVG hex board drawn from the view: hex kinds, borders, base pennants, fronts, token glyphs. |
| `src/format.ts` | Labels for actions (content names for cards, threats, growth options; standard actions with costs) and events; hex-kind and token colours. |
| `src/panels.ts` | Alignment track, adversary row, countries strip (megaprojects, threats), seat panel contents, piles. |
| `src/main.ts` | Wiring: choice prompt, action buttons, log, timeline, hover card, record dialog. |
