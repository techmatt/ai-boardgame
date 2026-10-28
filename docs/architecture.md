# Architecture

How the engine is put together, and how to change the game without fighting it. The
rules themselves are in [rules.md](rules.md).

## Crates

- **`aic-core`**: the rules. Pure and deterministic; no rules live anywhere else.
- **`aic-sim`**: native CLI. Plays seeded games, compares configs, replay-checks
  everything. Agents live here, not in the core: the skill ladder (`agents/`, playing
  through the public API on determinized copies, never peeking at hidden cards) and
  the experiment harness (`experiment.rs`, `report.rs`) that measures play at graded
  skill. See the root README, *Agents* and *Experiments*.
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
  hold: tokens on hexes, resources on seat boards, exhausted cards, ordered card piles
  (decks, discards, hands, slots, building rows), a round and phase marker, numbered
  tracks and markers (`vars`). No hidden bookkeeping. If a module needs more than that,
  the design needs a look first. `rng` (the dice) and `stats` (sim observation, never
  read by rules) are not game components; `pending` (an open choice, below) is the step
  the table is in the middle of.

## API

```rust
let rules = Rules::new(&config)?;            // validate + compile once per config
let mut g = rules.new_game(seed);            // setup, then advance to the first decision or choice
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

A config is JSON. [`configs/baseline.json`](../configs/baseline.json) is the v0 game
(2 countries, difficulty 0); [`configs/shared_row.json`](../configs/shared_row.json)
differs only in the threat variant. Abridged:

```json
{
  "name": "baseline",
  "countries": 2,
  "modules": [
    { "module": "board", "variant": "pieces", "params": { "start_population": 1, ... } },
    { "module": "adversary", "variant": "militarist", "params": { "difficulty": 0, "stage_cards": [3, 4, 3], ... } },
    { "module": "threats", "variant": "countries", "params": { "per_country": 1 } },
    { "module": "opposition", "variant": "pushback", "params": { "per_player": 4, ... } },
    { "module": "economy", "variant": "standard", "params": { ... } },
    { "module": "ai", "variant": "boards", "params": { "boards": [0, 1], "start_credits": 12, ... } },
    { "module": "cards", "variant": "draft", "params": { "deal": 4, ... }, "cards": ["fab_line", ...] }
  ],
  "round": ["threats.deal", "cards.tier", "cards.deal", "cards.draft", "cards.keep", "ai.growth",
            "cards.play+threats.counter+economy.give", "threats.execute",
            "adversary.ravage", "adversary.build", "adversary.explore", "adversary.slide",
            "economy.upkeep", "board.cleanup"]
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
  to at most one module. In v0 the `cards` module lists every player card, AI uniques
  included; leaving a card out removes it from the game.
- **round**: the phase sequence, as `"<module>.<phase>"`:
  - a phase may appear more than once, or not at all;
  - `"a.play+b.build"` combines several modules' **decision** phases into one
    simultaneous phase (v0's action phase combines cards, threat counters and giving);
  - if `round` is empty, it defaults to every module's phases in module order.
- **Config hash**: `Rules::config_hash` is a 64-bit FNV-1a hash of the *resolved*
  config's JSON (round filled in, lists normalized, `name` excluded). Every
  `GameRecord` and every sim result also carries the content hash, the engine version
  and `RULES_REVISION`, so a content or rules change is never mistaken for the same
  experiment. Lists are hashed as written: `[1,2,3]` and `[1,2,3,3]` play the same
  schedule but hash differently.
- **Overrides**, for sweeps (`Config::apply_override`, the sim's `--set`):
  - `countries=4`;
  - `adversary.stage_cards=3,4,3` (`3` or `1,2,3`);
  - `adversary.difficulty=` unsets a parameter;
  - `opposition.variant=income` switches variant and drops the parameters the new
    variant doesn't take, so only its new ones need setting
    (`--set opposition.income_credits=2`);
  - `<module>=off` removes a module and its phases. In v0 every module is needed by
    another (`ModuleDef::requires`), so this is rejected with the reason. To measure a
    module's effect, use its `off` *variant* instead (`threats.variant=off`,
    `opposition.variant=off`, `cards.variant=off`): it keeps the module's phases and
    the parameters other modules read, and does nothing. See rules.md §2b.
  - Switching the adversary variant drops most of its parameters, so the spread
    variants have their own configs (`configs/clusters.json`, `configs/assets.json`).
- **`aic-sim show <config>`** prints the resolved config, its hash and every tunable
  (card-derived ones included), with its range.

## Modules

A **module** is one rules area (the adversary, threats, cards...). A **variant** is one
way of doing it. The difference matters for tuning:

| Change | Kind |
|---|---|
| A number inside a mechanic | **parameter**: config only, sweepable |
| Another way of doing the mechanic | **variant**: code, selected by config |
| A new mechanic | **module**: code, added to the config |
| A card, threat, adversary card, AI board or country piece made of existing primitives | **content**: data only (`content/`) |
| A new effect primitive or selector | code (`content.rs`, plus the module that runs it) |
| A new kind of player action | code (`Action`); a structural change |

The v0 modules are `board`, `adversary`, `threats`, `opposition`, `economy`, `ai` and
`cards` (`crates/aic-core/src/modules/`); `common.rs` (board queries, piece names) and
`effects.rs` (the player effect primitives) are shared code. Rules areas lean on each
other, so modules call each other's public functions (a card's damage calls
`adversary::damage`, which scores `opposition::for_kill`) and read each other's
parameters through `World::int("<module>.<param>")`. A module lists the modules it reads
in `ModuleDef::requires`; a config missing one is rejected.

A rule that depends on another module's *variant* reads `World::variant("<module>")`.
The one case so far: the adversary variants (`militarist`, `clusters`, `assets`) use
different pieces, so threats, opposition pushback and card effects ask
`adversary::style(w)` and go through adversary functions (`selects`, `kind_count`,
`reinforce`, `forward_base`, `pushback`, `under_bar`, `ravage`, `build_hex`, `cascade`)
that read the Militarist's piece names as the pieces in play. `common::ADVERSARY` lists
every variant's piece kinds; a game holds only its own.

Each variant is a `VariantDef` (in `module.rs`), holding:

- `params`: `ParamSpec { name, kind: Int | List, min, max, doc }`;
- `phases`: `PhaseSpec { name, kind: Auto | Decision, doc }`, the phases it contributes
  to the round;
- `rng`: the RNG purposes it draws from;
- `uses_cards`: whether it plays content cards. If it does, every `{"param": ...}` in a
  selected card's cost becomes a required integer parameter of this module (0–1000);
- `build`: turns validated params into a `Box<dyn Module>` with typed fields.

A `Module` implements whichever of these hooks it needs:

| Hook | When |
|---|---|
| `setup` | Once, at `new_game`, before round 1. |
| `round_start` | Start of every round, before its first phase. |
| `run(phase)` | An `Auto` phase is reached. May ask choices. |
| `begin(phase)` | A `Decision` phase is entered (seats un-passed). May pass seats with nothing to decide. |
| `legal(phase, seat)` | A `Decision` phase: append the seat's non-pass actions. |
| `can_pass(phase, seat)` | Whether the engine offers `Pass` (default yes; draft picks and growth say no, and the module passes the seat itself). |
| `owns(phase, action)` | Whether a non-pass action is this module's (routing in combined phases). |
| `act(phase, action)` | Validate and apply an owned action. May ask choices. On `Halt::Illegal` the engine restores the state. |
| `after` | When an action or automatic phase finishes resolving (after the answer that completes it): standing checks (opposition levels, win and loss). May ask choices. |

Modules change the state through `Cx`, which keeps the event log and counters
consistent:

- `add_tokens`, `add_resource`;
- `move_card` (between piles, or to and from the box), `set_pile` (a shuffle);
- `exhaust`, `refresh` (cards; the owning module refreshes its own cards);
- `set_var` (this module's track), `set_key` / `add_key` (any track by full key);
- `choose` (players choose, below), `note` (a log line);
- `adversary_ops`, `count` (this module's counter), `tally` (any counter by full key);
- `rng(purpose)`;
- `end`.

### Choices ("players choose")

Every point where the rules say players choose (a ravage's loss order, a cascade target,
the Momentum drone, a Panic destination, a card's target hex) is a choice in
`legal_actions`, made by the team agent like any other action:

- A module calls `cx.choose(seat, prompt, options)?` in the middle of resolving. With no
  options it gets `None`; with one, that option without asking. Otherwise, the first time
  through, the call stops resolution with `Halt::Ask`.
- The engine then stores `GameState::pending`: the question, the answers so far, what
  was being resolved (the phase, or the action) and a snapshot of the state before it.
  The state shown is the partly resolved one. `legal_actions` is only the options, as
  `Action::Choose { seat, option, target }` (a hex target makes it a hex pick in the
  viewer); passing is not allowed.
- An answer restores the snapshot and resolves the phase or action again from the start,
  replaying the answers so far. So resolution code is written straight through, never
  stores where it stopped, and must depend only on the state and the answers. Events
  for the replayed part are not logged twice.
- Answers are ordinary actions in the action log, so replay and records need nothing
  extra.

Counter and var names are namespaced `"<module>.<name>"`.

### The round

The engine starts a round (round counter, then the `round_start` hooks), then walks
`config.round`. It resolves each `Auto` phase immediately and stops at each `Decision`
phase until every seat has passed, and at every open choice. The game ends when a
module calls `Cx::end`, or as a `round_limit` loss after 1000 rounds.

**Decision phases are simultaneous.** Any seat may act in any order until all seats
pass. One team agent drives every seat, and `legal_actions` lists all seats' actions
together (seat by seat; within a seat, module by module in the order the round entry
lists them, then the pass if allowed). In v0, seats' actions may depend on each other
(pooled threat counters, giving insight, the shared board): the placeholder-era rule
that simultaneous actions commute was dropped, since the ruleset has players help each
other. Order matters the way it does at a table where the team talks it through.

### Adding a variant

1. In the module's file, add a `VariantDef` to its `DEF.variants`, with a build function
   and a `Module` impl. Shared behaviour can be one struct with an option, as
   `threats::Threats` (`shared`) and `opposition::Opposition` (`reward`) do.
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
4. If it needs player input, reuse an `Action` if one fits (with `owns` claiming it)
   and `cx.choose` for choices inside its resolution. Add an `Action` variant only if
   neither fits.
5. Make sure random games still replay (extend `variant_matrix` in `tests.rs`), and
   document it in `docs/rules.md`.
6. The viewer shows its tokens, resources, `vars` tracks and exhausted cards without
   any work. Add a `view` hook for derived values, card groups and piles (see
   [The generic view](#the-generic-view)).

### Adding content

Content is data in `content/`, composed from code primitives:

- **Player cards** (`cards.json`): `effects` ([`Effect`]: on play, or when built),
  `passives` ([`Passive`]: standing effects), an `ability` (once a round), `upkeep`,
  `tags`, `needs`, a `tier` (0 for an AI's unique card). List the id in the `cards`
  module's config `cards`. A cost written as `{"param": "name"}` becomes a tunable.
- **Threats** (`threats.json`): `counter` costs and `effects` ([`ThreatOp`], with hex
  `Selector`s).
- **Adversary cards** (`adversary.json`): stage, terrains, escalation.
- **AI boards** (`ais.json`): slots, innate passives, two tracks, growth options (lists of
  `Effect`), unique cards.
- **Country pieces** (`map.json`, `countries`): terrain rows, uplinks, setup hexes.

Ids are unique across all content, and actions refer to content by id, so reordering a
file does not break action logs. `Content::check` validates references, tags and
resources when the content loads.

## Map

`content/map.json` defines the piece shape (4×4 rhombus, 3 card slots), one layout per
country count (1–4), and the four printed country pieces. A placed piece covers
`anchor + i·axes[0] + j·axes[1]` for `i, j` in `0..4`:

- `Map::build` derives hexes in country-then-piece order, so `state.hexes[i]` is
  `map.hexes[i]`. It also derives neighbours (in `Dir::ALL` order), each country's hex
  list, and each hex's printed number (`i·4 + j + 1`), terrain and uplink flag from the
  country's piece (country *c* uses piece *c*).
- Geometry is not in the state. UIs get it from `board(config)` in wasm.
- The `layouts_match_the_reference_images` test pins the 3/4 layouts to the hex sets
  read off the reference images; `country_pieces_follow_the_ruleset` checks the piece
  constraints (terrain counts, uplinks on the outer edge, factories next to population)
  in every layout.

## The generic view

Modules come and go, so clients must not need code per module. `Rules::view(&state)`
(wasm `view(state)`, `view.rs`) turns any state into a fixed set of shapes, and the
web viewer draws only these:

| Part | Holds | Filled by the engine |
|---|---|---|
| `hexes[i]` | `hex` `[q, r]`, `country`, `piece_pos`, `neighbours`, `number`, `terrain`, `uplink`, `tokens` (`{kind, count}`) | every token stack on every hex |
| `countries[c]` | `colour`, `hexes`, `slots` (card strip), `values`, `cards` (card groups, such as its threat) | `tokens.<kind>` totals over the country |
| `seats[s]` | `passed`, `values`, `cards` (card groups) | `resource.<name>` per resource; exhausted cards no module shows, as group `engine.exhausted` |
| `globals` | `values` of the whole game | every `vars` track, by its key |
| `piles` | `{key, label, owner, count, visible}` | none: modules add them |
| `choice` | `{seat, prompt}` of the open choice | when `pending` is set |
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
- `out.country_cards(country, key, label, cards)`: a card group on a country.
- `out.pile(key, label, owner, count, visible)`: an ordered pile.

In v0:

- `adversary` adds the pipeline as piles (`deck` with any peeked cards, `explore`,
  `build`, `ravage`, `discard`) and the corruption pool, stage and difficulty.
- `threats` adds each country's threat (`threats.on`, status = counter progress such as
  `1/2 energy`), or the shared row pile, and the tier dealt next.
- `opposition` adds the track toward the next level, the level with its win condition,
  and population against the loss threshold.
- `ai` adds each seat's board, both tracks (current value; the doc lists what is under
  processors), tier-2 unlock and processor count.
- `economy` adds each seat's income per resource; `cards` adds hand, buildings
  (`n/slots`), draft hand and drafted cards with statuses, tags, and the decks.

The view is display data:

- The rules never read it.
- It is not in the state, the fingerprint or a replay.
- A hook must not draw from the RNG; it gets `&GameState`, so it can't.

Two tests check it: `the_view_shows_every_hex_seat_country_and_module_value`, and
`the_view_shows_state_no_module_describes` (unknown tokens, resources, vars, cards).

**What the client assumes.** It draws hexes from `[q, r]` by terrain, with the
country colour, and draws token kinds it knows with their own glyph and any other kind
as a counter. It offers actions as `legal_actions` lists them, grouped per seat:

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
  - `actions`: player actions applied, answered choices included;
  - `counters["<module>.<name>"]`: anything a module counts. v0 counts cards kept,
    played and used (per card), growth picks, threats dealt, countered and executed,
    adversary pieces destroyed, population and buildings lost, corruption added,
    processors placed, insight given and credit income.
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

Left out on purpose until something needs them:

- **`Event` is a closed enum**, and there are no interception hooks between modules
  (such as "before a population is lost"); modules call each other's functions instead
  (Civic Shelter is checked inside `adversary::lose_population`).
- **Timed-out games** (the action cap) still count toward the round statistics.
  They're visible as `ended: timeout`.
- **Old saved states**: `GameState` has no schema version (only `Stats`, `piles` and
  `pending` default), so a state saved by an older engine may fail to load. Records
  (config + seed + actions) are the durable format.
- **Snapshots cost**: every action and automatic phase clones the state first (the
  snapshot a choice replays from). Fine at v0 speeds; a hot spot if sims need more.
