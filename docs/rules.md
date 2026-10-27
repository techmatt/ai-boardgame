# Rules as implemented

What the code currently implements, and nothing else. **Keep this current with every
rules change.** Design intent lives in [principles.md](principles.md); how the engine is
put together lives in [architecture.md](architecture.md).

> Everything below the board section is a **placeholder loop** that exists to exercise
> the engine. None of it is a rule of AI Complete.

## Board

- Each country is one **4×4 rhombus piece** of 16 hexes, and is exactly one megacorp
  (one seat). A game has 1–5 countries.
- Pieces are placed around a shared centre by a fixed layout per country count
  ([`content/map.json`](../content/map.json)). The 3-, 4- and 5-country layouts are taken
  from the reference images; the 1- and 2-country layouts are the first one and two
  pieces of the 3-country layout (a stand-in until real ones are drawn).
- Each piece has a strip of **3 card slots** for per-country cards. Nothing uses them yet.
- Hexes use axial coordinates `[q, r]` (pointy-top; `q` east, `r` south-east). `[0, 0]`
  is the centre hex of the reference grid.

## Round structure

A round is a sequence of phases set by the config. In the placeholder configs:

1. `income.collect`
2. `actions.play` — the only player phase
3. `spread.advance`
4. `victory.check`

**Player phases are simultaneous.** Any seat may act, in any order, until every seat has
passed. One seat's actions never change what another seat may do. Hands and boards are
open. Several modules' player phases can be played as one phase (`a.play+b.play` in the
config); none of the placeholder configs do this.

## Placeholder modules

Numbers in `code` are parameters set by the config; the placeholder configs are in
[`configs/`](../configs/).

### Setup

- **spread** places `start` blight on each country, one at a time, by its variant's
  placement rule (below).

### income (variants)

- **flat:** every country gains `amount` credits.
- **clean_land:** every country gains `base` credits, plus 1 per `per_clean` blight-free
  hexes in it (rounded down).

### actions — `open_cards`

Every seat may play each configured card once per round, paying its cost in credits.
Targets are always hexes in the player's own country that hold blight.

| Card | Cost | Effect |
|---|---|---|
| Scrub | `scrub_cost` | Remove up to `scrub_amount` blight from a hex. |
| Invest | `invest_cost` | Gain `invest_gain` credits. |
| Purge | 4 | Remove up to 3 blight from a hex. |

Used cards refresh at the start of each round. Credits carry over between rounds.

### spread (variants)

In round N every country gets the Nth entry of `per_country` (counting from 1) as new
blight; after the last entry, the last entry repeats.

- **random:** each blight goes to a uniformly random hex of the country.
- **cluster:** each blight goes to the country's most-blighted hex that still holds fewer
  than `cap` (ties: lowest piece position). If the country has no blight, a random hex.
  If every blighted hex is full: the first clean neighbour in the country, checking E,
  NE, NW, W, SW, SE, of the full hex with the lowest piece position; else the clean
  hex with the lowest piece position. "Piece position" counts along the piece's first
  axis, then its second. If the country is completely full, the blight is
  not placed.

### victory — `survive`

Checked once per round: the team **loses** as soon as any country holds `overrun` or more
blight; otherwise it **wins** at the end of round `rounds`.

A round with no ending at all stops after 1000 rounds as a loss (`round_limit`).
