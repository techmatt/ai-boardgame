# Architecture

How the engine is put together, and how to change the game without fighting it. The
rules themselves are in [rules.md](rules.md).

## Crates

- **`aic-core`**: the rules. Pure and deterministic; no rules live anywhere else.
- **`aic-sim`**: native CLI. Plays seeded games, compares configs, replay-checks
  everything. Agents live here, not in the core.
- **`aic-wasm`**: wasm-bindgen adapter, JSON strings in and out. No rules.
- **`web/`**: the browser viewer (Vite + TypeScript) on `aic-wasm`. It draws the
  [generic view](#the-generic-view) and offers only `legal_actions`; see
  [web/README.md](../web/README.md).

## Core invariants

- **No I/O**: no files, network, env vars, stdout or logging. Content is embedded at
  compile time from `content/`; configs are handed in by the caller.
- **No clock**: no `std::time`, `Instant` or `SystemTime`.
- **No global RNG**: no `thread_rng`, `rand::random` or `getrandom`. Randomness comes
  only from per-purpose streams in `GameState::rng`:
  - Each variant declares its purposes (`VariantDef::rng`). `Cx::rng` panics if a
    module draws from a purpose it did not declare, and two modules may not declare the
    same one.
  - A stream is keyed by the game seed and a hash of the purpose name, so adding,
    removing or reordering modules never shifts another module's numbers.
- **Fixed-width draws**: `Stream` exposes only `below(n)` and `next_u32()`, written
  out on `u32`. rand samples `usize` from a `u64` on 64-bit targets and from a `u32` on
  wasm32, which made native and browser games differ. Never draw a `usize` through
  rand.
- **No hash-order leaks**: every map is a `BTreeMap`, and iteration follows seat, piece
  or `Dir::ALL` order. Never `HashMap`/`HashSet` in anything that reaches the state or
  results.
- **Serializable state**: `GameState` round-trips through JSON. The seed and the RNG
  positions are decimal strings, because they don't fit JavaScript numbers.
- **Exact replay**: config + seed + action list rebuild the identical state. The sim
  checks this for every game.
- **Native = wasm**: the `golden_game` test pins the final-state fingerprint of a fixed
  game, and `scripts/wasm-smoke.mjs` checks that the wasm build reaches the same
  fingerprint.
- **Printable state**: everything the rules read is something a physical copy could
  hold: tokens on hexes, resources on seat boards, exhausted cards, card slots, a round
  and phase marker, numbered tracks (`vars`). No hidden bookkeeping. If a module needs
  more than that, the design needs a look first. `rng` (the dice) and `stats` (sim
  observation, never read by rules) are the only parts that are not game components.

## API

```rust
let rules = Rules::new(&config)?;            // validate + compile once per config
let mut g = rules.new_game(seed);            // setup, then advance to the first decision
rules.legal_actions_into(&g, &mut buf);      // hot path: reuse a buffer
rules.apply_mut(&mut g, &action, None)?;     // hot path: in place, no event log
let (next, events) = rules.apply(&g, &action)?;  // copy + event log
rules.tunables();                            // every tunable of this config, with values
rules.view(&g);                              // the generic view, for clients (display only)
rules.check_state(&g)?;                      // does a (deserialized) state fit these rules?
```

The free functions `new_game(&config, seed)`, `legal_actions(&state)` and
`apply(&state, &action)` are the convenient path:

- They rebuild `Rules` from the config inside the state on every call, which is fine
  for a UI and wrong for a sweep.
- They run `check_state` first, so a mismatched state is an error, not a panic (in wasm,
  a panic is an `unreachable` trap).

`driver::play(&rules, seed, &mut agent, max_actions)` plays a whole game with an
`Agent` (one team agent for every seat) and returns `(final GameState, GameRecord)`.
`driver::replay` rebuilds a game from its seed and actions.

## Config

A config is JSON (examples in [`configs/`](../configs/)):

```json
{
  "name": "baseline",
  "countries": 3,
  "modules": [
    { "module": "income", "variant": "flat", "params": { "amount": 3 } },
    { "module": "actions", "variant": "open_cards", "cards": ["scrub", "invest", "purge"],
      "params": { "scrub_cost": 2, "scrub_amount": 1, "invest_cost": 2, "invest_gain": 3 } },
    { "module": "spread", "variant": "random", "params": { "start": 2, "per_country": [1, 2, 2, 3] } },
    { "module": "victory", "variant": "survive", "params": { "rounds": 8, "overrun": 13 } }
  ],
  "round": ["income.collect", "actions.play", "spread.advance", "victory.check"]
}
```

- **modules**: at most one instance per module id, each in one variant.
- **params**: named tunables. **Every** parameter must be given; there are no defaults
  in code. Unknown names, missing names, wrong kinds and out-of-range values are errors
  that name the parameter. A config is the whole experiment, and a typo can't silently
  test the default.
- **Integers only.** Every number has to be printable. A `list` parameter is a per-round
  schedule or a table (for schedules, the last entry repeats, via `module::scheduled`).
  A single integer is accepted for a list and normalized to a one-item list.
- **cards**: content ids, for variants that play cards (`uses_cards`). A card belongs
  to at most one module.
- **round**: the phase sequence, as `"<module>.<phase>"`:
  - a phase may appear more than once, or not at all;
  - `"a.play+b.build"` combines several modules' **decision** phases into one
    simultaneous phase, so a new player-facing module can add actions to the existing
    player turn without adding a second all-pass round trip;
  - if `round` is empty, it defaults to every module's phases in module order.
- **Config hash**: `Rules::config_hash` is a 64-bit FNV-1a hash of the *resolved*
  config's JSON (round filled in, lists normalized, `name` excluded). Every
  `GameRecord` and every sim result also carries the content hash, the engine version
  and `RULES_REVISION`, so a content or rules change is never mistaken for the same
  experiment. Lists are hashed as written: `[1,2,3]` and `[1,2,3,3]` play the same
  schedule but hash differently.
- **Overrides**, for sweeps (`Config::apply_override`, the sim's `--set`):
  - `countries=4`;
  - `spread.per_country=1,2,3` (`3` or `1,2,3`);
  - `income.base=` unsets a parameter;
  - `income.variant=clean_land` switches variant and drops the parameters the new
    variant doesn't take, so only its new ones need setting;
  - `spread=off` removes a module and its phases (the "effect of removing the module"
    measure).
- **`aic-sim show <config>`** prints the resolved config, its hash and every tunable
  (card-derived ones included), with its range.

## Modules

A **module** is one mechanic (for example income, the adversary's spread, or the end
condition). A **variant** is one way of doing that mechanic. The difference matters for
tuning:

| Change | Kind |
|---|---|
| A number inside a mechanic | **parameter**: config only, sweepable |
| Another way of doing the mechanic | **variant**: code, selected by config |
| A new mechanic | **module**: code, added to the config |
| A card made of existing effect primitives | **content**: data only (`content/cards.json`) |
| A new effect primitive or target rule | code (`content.rs`, plus the module that runs it) |
| A new kind of player action | code (`Action`); a structural change |

Each variant is a `VariantDef` (in `module.rs`), holding:

- `params`: `ParamSpec { name, kind: Int | List, min, max, doc }`;
- `phases`: `PhaseSpec { name, kind: Auto | Decision, doc }`, the phases it contributes
  to the round;
- `rng`: the RNG purposes it draws from;
- `uses_cards`: whether it plays content cards. If it does, every `{"param": ...}` in a
  selected card becomes a required integer parameter of this module (0–1000);
- `build`: turns validated params into a `Box<dyn Module>` with typed fields.

A `Module` implements whichever of these hooks it needs:

| Hook | When |
|---|---|
| `setup` | Once, at `new_game`, before round 1. |
| `round_start` | Start of every round, before its first phase. |
| `run(phase)` | An `Auto` phase is reached. |
| `legal(phase, seat)` | A `Decision` phase: append the seat's non-pass actions. |
| `owns(phase, action)` | Whether a non-pass action is this module's (routing in combined phases). |
| `act(phase, action)` | Validate and apply an owned action. On error, leave the state untouched. |

Modules change the state through `Cx`, which keeps the event log and counters
consistent:

- `add_tokens`, `add_resource`;
- `exhaust`, `refresh` (cards; the owning module refreshes its own cards, the
  placeholder in `round_start`);
- `set_var` (emits `VarChanged`);
- `adversary_ops`, `count`;
- `rng(purpose)`;
- `end`.

Counter and var names are namespaced `"<module>.<name>"` automatically.

### The round

The engine starts a round (round counter, then the `round_start` hooks), then walks
`config.round`. It resolves each `Auto` phase immediately and stops at each `Decision`
phase until every seat has passed. The game ends when a module calls `Cx::end`, or as a
`round_limit` loss after 1000 rounds.

**Decision phases are simultaneous.** Any seat may act in any order until all seats
pass. One team agent drives every seat, and `legal_actions` lists all seats' actions
together (seat by seat; within a seat, module by module in the order the round entry
lists them, then the pass).

A module's actions **must not create dependencies between seats**: one seat's action
never changes what another seat may do or what its action does. Placeholder cards
achieve this by targeting only the player's own country. The
`simultaneous_actions_commute` test checks it for every variant (whole-state equality
after x;y and y;x). A future help mechanic has to keep it, for example by having the
helper spend only their own resources.

### Adding a variant

1. In the module's file, add a `VariantDef` to its `DEF.variants`, with a build function
   and a `Module` impl. Shared behaviour can be one struct with an option, as
   `spread::Spread` does.
2. Add its parameters to a config in `configs/` and run
   `aic-sim compare configs/old.json configs/new.json`, or sweep it in place with
   `--set-b <module>.variant=<name> --set-b <module>.<new param>=...`.
3. Add it to `variant_matrix` in `tests.rs`, bump `RULES_REVISION` if existing games
   change, and document it in `docs/rules.md`.

### Adding a module

1. Create `crates/aic-core/src/modules/<id>.rs` with `pub static DEF: ModuleDef`.
2. Add `pub mod <id>;` and `&<id>::DEF` to `REGISTRY` in `modules/mod.rs`.
3. Add it to configs: a module entry, plus its phases in `round` (its own entries, or
   `+`-combined with an existing decision phase).
4. If it needs player input, reuse `Play`/`Pass` if it can (with `owns` claiming its
   actions). Add an `Action` variant only if it can't.
5. Make sure random games still replay and that simultaneous actions still commute
   (extend `variant_matrix`), and document it in `docs/rules.md`.
6. The viewer shows its tokens, resources, `vars` tracks and exhausted cards without
   any work. Add a `view` hook only for derived values or card status (see
   [The generic view](#the-generic-view)).

### Adding content

Add a card to `content/cards.json` using existing `op`s (`gain`, `remove_tokens`,
`place_tokens`) and target rules (`own_hex`). Write a number as `{"param": "name"}` to
make it tunable from config. A literal is fixed until someone edits the content. Then
list the card id in a module's `cards`. Actions and exhausted cards refer to cards by
id, so reordering `cards.json` does not break action logs.

## Map

`content/map.json` defines the piece (4×4 rhombus, 3 card slots) and one layout per
country count. A placed piece covers `anchor + i·axes[0] + j·axes[1]` for `i, j` in
`0..4`:

- `Map::build` derives hexes in country-then-piece order, so `state.hexes[i]` is
  `map.hexes[i]`. It also derives neighbours (in `Dir::ALL` order) and each country's
  hex list.
- Geometry is not in the state. UIs get it from `board(config)` in wasm.
- The `layouts_match_the_reference_images` test pins the 3/4/5 layouts to the hex sets
  read off the reference images.

## The generic view

Modules come and go, so clients must not need code per module. `Rules::view(&state)`
(wasm `view(state)`, `view.rs`) turns any state into a fixed set of shapes, and the
web viewer draws only these:

| Part | Holds | Filled by the engine |
|---|---|---|
| `hexes[i]` | `hex` `[q, r]`, `country`, `piece_pos`, `neighbours`, `tokens` (`{kind, count}`) | every token stack on every hex |
| `countries[c]` | `colour`, `hexes`, `slots` (card strip), `values` | `tokens.<kind>` totals over the country |
| `seats[s]` | `passed`, `values`, `cards` (card groups) | `resource.<name>` per resource; exhausted cards no module shows, as group `engine.exhausted` |
| `globals` | `values` of the whole game | every `vars` track, by its key |
| `piles` | `{key, label, owner, count, visible}` | none yet: no module has piles |
| `round`, `phase`, `round_order`, `outcome` | where the game is | always |

A **value** is `{key, label, value, max?, source, doc?}`. `max` is a limit to measure
against (a threshold or a round count); the viewer marks a value at 75% of its `max` or
more. `source` is `"engine"` or the module id. A **card** is `{id, name, text?, status,
cost?}`. `status` is a short word the module chooses; the viewer styles `ready`,
`used`, `unaffordable` and `no target`, and shows any other word as text.

**How a module exposes its state.** Most of it needs nothing. Tokens it places,
resources it gives and `vars` it sets all appear by default, labelled by their keys. A
module implements `Module::view(&self, g, w, out)` only to add what the engine can't
derive:

- `out.global(key, label, value, max, doc)`, `out.country(c, ...)`, `out.seat(s, ...)`:
  derived values such as a threshold, a projection or a schedule entry. Keys become
  `"<module>.<key>"`. A global with a `vars` key replaces that track's generic entry.
- `out.cards(seat, key, label, cards)`: a card group with each card's status. Cards it
  shows are no longer listed again under `engine.exhausted`.
- `out.pile(key, label, owner, count, visible)`: an ordered pile, for the first module
  that has one.

The placeholders show what this looks like:

- `victory` adds round *n*/`rounds`, and blight / `overrun` per country.
- `spread` adds this round's `per_country` (and `cap` for `cluster`).
- `income` adds each country's income if collected now.
- `actions` adds each seat's open cards as ready, used, unaffordable or no target,
  with costs.

The view is display data:

- The rules never read it.
- It is not in the state, the fingerprint or a replay.
- A hook must not draw from the RNG; it gets `&GameState`, so it can't.

Two tests check it: `the_view_shows_every_hex_seat_country_and_module_value`, and
`the_view_shows_state_no_module_describes` (unknown tokens, resources, vars, cards).

**What the client assumes.** It draws hexes from `[q, r]`, tints them by country
colour, and draws every token kind as a counter (blight has a set colour, other kinds
get a hashed one). It offers actions as `legal_actions` lists them, grouped per seat:

- Actions that differ only in a `target` hex become one button that arms a hex pick.
- Every other action is its own button, labelled from its fields.

A new `Action` variant with new fields therefore shows up as a button with no client
change. A hex-valued field under another name still gets a button per action, but no
board pick. Anything the view doesn't cover is in the viewer's *Raw state* tab.

**Records.** `SavedGame` (`record.rs`, format `aic-game-record/1`) holds the resolved
config, the seed and the action list, plus identity and the saved outcome and final
fingerprint. It is the one format for a game the sim saves (`aic-sim --save`), a
browser bug report and a game loaded into the viewer. `SavedGame::replay` rebuilds the
game and warns if the rules revision, content, config hash or final fingerprint
differ.

## Observability

- **Event log**: `Event`s from `apply` and `new_game_logged`, for UIs and the sim's
  `--trace`. Hot loops pass `None` to skip building it.
- **Counters** in `GameState::stats`, deterministic and replayed:
  - `adversary_ops[r-1]`: adversary operations in round `r`, a proxy for table time;
  - `setup_ops`: adversary operations during setup;
  - `actions`: player actions applied;
  - `counters["<module>.<name>"]`: anything a module counts. The placeholders count
    plays per card (pick/play rates), blight removed and placed (setup included), and
    credits earned.
- **Outcome**: `won`, the `round` the game ended, a stable `reason` code for aggregation,
  and a human `detail`.
- **Decisions** (`driver::play` → `GameRecord`): each applied action is one decision.
  `options_total`, `forced` (exactly one legal action) and `max_options` count the legal
  actions offered across all seats, passes included.
- **Identity**: every `GameRecord` carries `config_hash`, `content_hash`, the engine
  version, `rules_revision` and the final state's fingerprint.

The sim aggregates all of these. It averages every counter per game, without needing to
know the counter names in advance.

## Known gaps

Left out on purpose until a real module needs them:

- **No ordered card piles.** There are no decks, discard piles or adversary queues in
  the state, and nothing writes `Seat::slots` yet. The first module that draws cards
  should add module-namespaced piles plus `Cx` helpers (`draw`, `shuffle(purpose)`,
  `set_slot`) that emit events.
- **`Event` is a closed enum**, and there are no interception hooks between modules
  (such as "before blight is placed"). Modules interact only through shared state in
  round order.
- **Timed-out games** (the action cap) still count toward the round statistics.
  They're visible as `ended: timeout`.
- **Old saved states**: `GameState` has no schema version (only `Stats` is
  `#[serde(default)]`), so a state saved by an older engine may fail to load.
