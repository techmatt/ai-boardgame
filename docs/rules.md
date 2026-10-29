# Rules as implemented

What the code implements: **rules revision 6, the 014 board model** (prompt A of three):
cities, research stations, factories, military bases and data centres, with
ambassadors as the adversary's main pressure, the Militarist against Builder and
Protector, 1–4 countries. 013's Alignment is kept (§5), its cards remapped to the new
pieces. Today's cards, AI unique cards and AI boards are carried over as
**placeholders**, remapped where they named a removed piece (§8); the card economy
(draft, keeping, playing, drawing; §8) works as it did in 013. Cards (B) and AI boards
(C) come later.

Gone with 014: population, processors, drones, outposts, foundries, uplinks, typed land
(Urban, Industrial, Network, Frontier), defense tokens, the corruption pool, the
explore → build → ravage pipeline, the `clusters`, `assets` and `uplinks` adversary
variants, and the configs `v0_untuned`, `clusters` and `assets`. Results and records
from revisions 1–5 stay in `experiments/results/` as history; they do not replay.

**Every number below is a candidate**: a starting calibration reasoned for an 8–10
round game (`docs/principles.md`), for simulation to adjust. Numbers in `code` are config
tunables (`<module>.<param>`), with the values in
[`configs/baseline.json`](../configs/baseline.json); card, threat, adversary-card,
megaproject and AI-board numbers are content in [`content/`](../content/). Where the
prompt left a gap, the choice made is marked **Default**; where the implementation
departs from the v0 ruleset's card text, **Deviation**. Both are collected at the end.
Design intent lives in [principles.md](principles.md); the engine in
[architecture.md](architecture.md). **Keep this current with every rules change.**

## Terms

- **Adjacent**: a neighbouring hex on the board, across country borders too.
- **Next to X**: holding X, or adjacent to a hex holding X.
- **Reach**: a player's reach is every hex within 1 of one of their data centres. You
  act within your reach.
- **Piece**: a city, a military base or a building (factory, research station, data
  centre). Ambassadors sit on pieces.
- **Clean / corrupted / aligned / owned**: a city or research station with no owner cube
  is clean (a station: neutral); an adversary cube means corrupted; a player cube means
  aligned (a city) or owned (a station).
- **Players choose**: an `Action::Choose` in `legal_actions`. Team choices in the
  adversary's steps, production and threats are asked of seat 0; a choice inside a
  player's own card, ability or standard action is asked of that player.
- **Hex number**: each piece's hexes are printed 1–16 row by row; ties go to the lower
  number.

## 1. The board (module `board`, variant `corps`)

### Country pieces

A game has 1–4 countries, one player each. Country *i* is printed piece *i*
([`content/map.json`](../content/map.json)), a 4×4 piece of 16 hexes placed by the
layout for that country count. Population is implicit: people live in cities.

Hex kinds: **city**, **open** land and **ocean** are printed; a **military base** goes
on an open hex of each land border (below). Buildings go on open hexes, one per hex.
Nothing builds on ocean.

| # | Name | Cities (corrupted first) | Ocean | Station | Factories | Starting data centres |
|---|---|---|---|---|---|---|
| 0 | Coastal Republic | 7, 6, 14 | 3, 12 | 4 | 2, 8 | 9, 10 |
| 1 | Northern Union | 10, 7, 15 | 4 | 12 | 3, 14 | 6, 2 |
| 2 | Highland Federation | 7, 2, 10 | — | 14 | 1, 6 | 11, 12 |
| 3 | Delta States | 11, 6, 14 | 1 | 12 | 7, 13 | 5, 9 |

Each factory touches a city (one of them the corrupted one); ocean hexes are on the
board's outer edge in every layout the piece appears in; the starting data centres
touch each other and at least one city, with at least two free open hexes next to them
in every layout (room to grow).

**Fronts and bases (Default).** A **front** is a land border between two countries: an
adjacent pair of their hexes, neither of them ocean (an ocean edge is not a front). Each
country gets one military base per front, on an open hex that touches the other
country's land and holds no setup building; a country's fronts with the fewest
candidate hexes pick first, each taking its lowest-numbered free candidate. Bases are
placed by the layout, not printed, because a piece's neighbours change with the country
count. Every country has at least one front at 2, 3 and 4 countries (4 countries: 6
fronts, country 0 touching all three others).

### Setup

- Each country: its first `start_corrupted` (1) cities get an adversary cube (difficulty
  4 with `adversary.extra_corrupted`, 0 by default, more); its research station starts corrupted; its
  two factories.
- Each player (seat *s* starts in country *s*): 2 data centres on the starting spots,
  `start_ambassadors` (1) player ambassador, `ai.start_credits` (25) credits, and their
  AI board's credit production (8).
- The adversary's supply: `adversary.supply_per_country` (10) ambassadors per country.
- The adversary deck is dealt and its top card revealed face up as round 1's card.

### Megaprojects

Each country's corp has one megaproject card (content, a placeholder per corp: **7
cubes**; reasoning: about 2 stations a country make 1–2 cubes a round, so a project
completes around round 4–5). In production each research station makes
`station_cubes` (1) cube onto its country's card in its owner's colour: an owned
station the owner's, a corrupted station the adversary's; a neutral station makes
nothing. When the card holds 7 cubes it **completes**: each player cube pays its owner
`project_credits` (10) credits; adversary cubes become Discord, 1 Sway per
`project_discord_per` (2). From then on each owned station in that country produces the
card's benefit for its owner every round (placeholder: 2 credits).

## 2. Ambassadors

**An ambassador sits on a piece. The next time that piece does its normal thing, the
ambassador redirects it to its own side and is removed.** Adversary ambassadors return
to the adversary's supply; a player ambassador that is used up is gone (**Default**: a
player regains ambassadors with the standard action or cards).

| On | Adversary ambassador | Player ambassador |
|---|---|---|
| City | that round's energy goes to the adversary (Discord) | waits; 3 (`free_city_at`) on a corrupted city free it: its cube returns |
| Research station | that round's cube is the adversary's (owned or corrupted station) | on a neutral station: the station becomes the player's (cube placed, ambassador used) |
| Data centre | its compute goes to the adversary (Discord) | waits (defense, below) |
| Factory | the unit it builds is an adversary troop on the factory's hex | no unit is built; `factory_unity` (1) Unity |
| Military base | `base_discord` (3) Discord instead of a unit; no war starts | `base_unity` (2) Unity instead of a unit |

- **Opposing ambassadors cancel.** When an ambassador arrives on a piece holding one of
  the other side, both are removed (the adversary's to its supply), with no Unity.
- **Defense:** a player ambassador on a building is used up instead of the building
  when it would be destroyed.
- **Diverted** energy and compute sway `divert_discord` (1) Discord per unit.
- A neutral station's normal thing is nothing, so an adversary ambassador waits there.

Player ambassadors move only through cards and the standard action.

## 3. The adversary (module `adversary`, variant `militarist`)

### Each round

1. **Build** (`adversary.build`): next round's face-up card becomes this round's card.
   Each corrupted city gains `city_build` (2) adversary ambassadors, and each factory
   next to (adjacent to) a corrupted city gains `factory_build` (1). The card's twists
   add more. A stage-II card's **Escalation** then gives each military base
   `escalation_ambassadors` (1).
2. **Spread** (`adversary.spread`): every hex holding several adversary ambassadors,
   fixed when the step starts, keeps one and sends each other one to a hex within
   `spread_range` (2): non-ocean, without an adversary ambassador (a clean city may hold
   any number). **Priority:** player-owned hexes (data centres, aligned cities, owned
   stations), then clean cities, then other hexes; then the nearest; players choose
   among equals. A city that is not corrupted never spreads: it holds its ambassadors.
   Firewall Node's owner's data centres are skipped. With no destination the rest stay.
3. **Corrupt** (`adversary.corrupt`): a city that is not corrupted holding `corrupt_at`
   (5) adversary ambassadors becomes corrupted (an aligned city loses its player cube);
   its ambassadors return to the supply.
4. **Reveal** (`adversary.reveal`, after `threats.deal`): the top card goes face up as
   next round's card.

**Calibration.** One corrupted city and its factory make about 3 ambassadors a round
per country. Each producing piece uses one up a round, players remove some, and the
spread prefers player pieces, so the first new corruption should come around rounds
5–6; after it, the corrupted cities double.

**Overflow** (variant `overflow`, off by default): an adversary ambassador arriving on a
city or station that already holds one corrupts it instead of stacking.

### The deck (the clock and the stager)

[`content/adversary.json`](../content/adversary.json) prints 13 cards: 4 stage I, 5
stage II, 4 stage III. A game deals `stage_cards` (3, 4, 3) at random per stage, each
stage shuffled on its own, stage I on top: 10 rounds. The next round's card is always
face up a round ahead (Recon Sweep shows more). Each card's twists change its round.

| Card | Stage | Twist | Escalation |
|---|---|---|---|
| Probing Strikes | I | none | |
| Factory Drive | I | factories next to clean cities build 1 too | |
| Long Patrols | I | spread reaches 1 hex further | |
| War Drums | I | sway 1 toward Discord | |
| Mobilisation | II | each corrupted city builds 1 more | yes |
| Station Raids | II | each corrupted research station builds 1 | yes |
| Fifth Column | II | clean cities corrupt at 1 fewer ambassador | yes |
| Border War | II | this round is a war round | yes |
| Supply Lines | II | factories next to clean cities build 1 too | yes |
| Total Mobilisation | III | corrupted cities build 1 more; factories next to clean cities 1 | |
| Blitzkrieg | III | war round; spread reaches 1 further | |
| Occupation | III | corrupt at 1 fewer; corrupted stations build 1 | |
| Iron Doctrine | III | war round; sway 2 toward Discord | |

Players **lose** when a round starts with no card to turn face up (after round 10 with
the default deal).

### Difficulty (`difficulty`, 0–4; each level includes the ones below)

| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules | |
| 1 | One extra threat a round in all, countries in turn (kept from 011) | `extra_threats` 1; `counter_bonus`, `counter_bonus_all` 0 |
| 2 | One stage-I card removed: stage II (escalation, tier-2 threats) a round sooner, and a 9-round clock | `stage1_removed` 1 |
| 3 | Larger builds: each corrupted city gains 1 more | `build_bonus` 1 |
| 4 | A smaller supply (8 per country), tier-2 threats a round sooner; optionally a second corrupted city per country at setup (off: it starves the supply in round 2) | `supply_reduction` 2, `tier2_early` 1, `extra_corrupted` 0 |

Discord tips raise the difficulty in play (§5); the setup rules of a level
(level 2's deck, level 4's supply and second city) are not applied mid-game, but level
3's larger builds and level 4's early tier-2 threats are.

**Why this ladder (014).** The supply loss bounds everything: one corrupted city makes
about 3 ambassadors a round per country against a supply of 10, so anything that doubles
the build (a second corrupted city) starves the supply within two rounds. The
corrupted-city-per-country and smaller-supply levels the prompt suggested were first
tried as levels 3 and 4 (second city, supply 7 per country): every game ended in round 1
or 2, before a decision mattered. Levels 3 and 4 were rebuilt from the milder options;
the curve is not tuned (014 quick screen: greedy 100 / 88 / 25 / 0 / 0 % at d0–d3, 16
games each).

## 4. War and troops

**War rounds** are on the round counter, not a card: `board.war` lists 1 for a war
round, per round, the last entry repeating; the default is rounds 2, 4, 6, 7, 8 and every
round after. Cards with the War twist (Border War, Blitzkrieg, Iron Doctrine) make their
round a war round too.

**Corp troops.** There are no player troops. Troops belong to the megacorp of their
territory and fight on fronts.

- In production each base produces 1 unit, and each factory builds `factory_units` (1)
  unit for its corp's nearest base (**Default**: the prompt's locked rule that an
  ambassador on a factory redirects "the unit it builds" means factories build units;
  `factory_units` 0 turns that off). A base holds at most `unit_cap` (3) units
  (**open with Matt**; a placeholder).
- **On a war round**, on each front the units at the two bases cancel 1 for 1. The
  surplus crosses, acts at once and is used up: each destroys the rival factory nearest
  its base (ties: lower hex number), or a data centre if the rival country has no
  factory. All units at the front are spent.

**Adversary troops** come from adversary ambassadors on factories. They stay on the
board, and in production each destroys one building on or next to its hex (players
choose; a corrupted station is not a target). Cards remove them (a troop costs
`troop_health` (2) of a card's removal; an ambassador 1), swaying `alignment.troop` (1)
toward Unity.

**Destruction.** Every destroyed building, from either kind of troop, sways
`destruction_discord` (1) Discord. A destruction is absorbed, in this order, by a Shelter
Order on the country, a hardening token on the hex, a player ambassador on the hex, or
a Civic Shelter (once a round, for its owner's countries). A destroyed data centre
leaves the game (**Default**; the track stays uncovered).

## 5. Alignment and the end of the game (module `alignment`)

Alignment (013) is where the AIs' world leans: toward **Unity** (the good AIs, the
players) or **Discord** (the evil AI, the adversary). Everything on it is printed: Sway
tokens on three spaces, two tip-count tracks, two small decks and an ongoing row.

**The track.** Sway sits in three places: **the Undecided** (the middle pool), the
**Unity bucket** and the **Discord bucket**. The Undecided starts with
`undecided_per_player` (4) × players Sway; both buckets start empty.

**Swaying** toward a side moves Sway one at a time from the Undecided into that side's
bucket. With the Undecided empty, the variant decides:

| Variant | Sway with the Undecided empty |
|---|---|
| `tug` (default) | comes from the opposite bucket (a tug of war); with that empty too, nothing |
| `still` | nothing happens |
| `off` | (for measuring Alignment, §9) no Sway ever moves and nothing tips |

**Tipping.** A bucket holding `tip_per_player` (2) × players Sway **tips**, in this
order: one card of its side resolves; that side's tip count goes up by one; all the
bucket's Sway returns to the Undecided. Unity tips and Discord tips are counted
separately and only count up; there is no net meter. **Default:** buckets are checked
when an action or automatic phase finishes resolving (with win and loss, below), so a
bucket may overfill within one action. If both are full, Discord tips first; a tip's
card may fill a bucket again, which then tips too.

**Momentum** (`momentum_every`, 0 = off, the default): a side's bucket holds 1 fewer
Sway for every `momentum_every` tips that side has made, down to 1 per player.

**Sources of Sway**

| Toward | From | Amount |
|---|---|---|
| Unity | a player ambassador on a producing base / factory | `base_unity` 2 / `factory_unity` 1 |
| Unity | removing an adversary troop | `alignment.troop` 1 (+ Unity Network) |
| Unity | cards (Rally, Global Mandate, Swarm Override), AI boards, ongoing Unity cards | card text |
| Discord | energy or compute diverted (a corrupted city, an ambassador on a city or data centre) | `divert_discord` 1 per unit |
| Discord | an adversary ambassador on a producing base | `base_discord` 3 |
| Discord | a destroyed building | `destruction_discord` 1 |
| Discord | adversary cubes on a completed megaproject | 1 per `project_discord_per` (2) |
| Discord | an uncountered threat, the antimatter action, twists, ongoing Discord cards | `threat_discord` 1, `antimatter_discord` 1, card text |
| Discord | the adversary at the start of every round | `adversary_discord` **0** (013: 1) |

`adversary_discord` is 0 because corrupted cities' diverted energy is now the
adversary's per-round drip (1 per corrupted city, already more than 013's 1 a round).

The round's Sway is the phase `alignment.sway`, first in the round: the adversary's
`adversary_discord`, then each AI board's `unity` (`content/ais.json`; 0 for Builder and
Protector), then the ongoing cards in the order they came into play.

**The antimatter action.** In the action phase (`...+alignment.act`), any player may
sway `antimatter_discord` (1) toward Discord to gain 1 antimatter, up to
`antimatter_uses` (1) times a round each. `antimatter_discord` 0 removes the action, as
does variant `off`.

**The cards.** Each side has a small deck in two tiers, each shuffled at setup
([`content/alignment.json`](../content/alignment.json)). A tip draws from tier 2 once
that side's tip count, before the tip, has reached its first threshold (`unity_ease[0]`
/ `discord_escalate[0]`, both 2), else from tier 1; from the other tier if that one is
empty; with both empty, the side's resolved cards are shuffled back into their tiers
first. A one-shot card resolves and goes face up on its side's resolved pile; an
**ongoing** card goes into the ongoing row, stays there, and resolves in
`alignment.sway` every round from the next one on. Choices are asked of seat 0.

| Side | Tier | Card | Effect |
|---|---|---|---|
| Unity | 1 | Grassroots Funding | every player +2 credit income for the rest of the game |
| Unity | 1 | Open Market | the next 2 card purchases are free |
| Unity | 1 | Think Tank | every player +1 insight each round for the rest of the game |
| Unity | 1 | Ceasefire | counter one active threat (players choose) |
| Unity | 1 | Civil Defense | in each country, remove 1 adversary ambassador (players choose the hex) |
| Unity | 1 | Volunteer Corps | ongoing: each round, sway 1 toward Unity |
| Unity | 2 | Global Coalition | counter every active threat |
| Unity | 2 | New Deal | every player +2 credit income for the rest of the game and 4 credits now |
| Discord | 1 | Incursion | deal one extra threat |
| Discord | 1 | Blight | in each country, the clean city with the most adversary ambassadors gains 1 more |
| Discord | 1 | Buildup | the adversary builds now |
| Discord | 1 | Propaganda | ongoing: each round, sway 1 toward Discord |
| Discord | 2 | Offensive | the adversary spreads now |
| Discord | 2 | Two Fronts | deal two extra threats |
| Discord | 2 | Firestorm | in each country, the clean city with the most adversary ambassadors gains 2 more |

The Unity cards are opposition's old rewards as one-shot cards; the Discord cards are
adversary effects the game already has. A threat countered by a card is discarded and
its partial payments are refunded. **Defaults:**

- Open Market's free purchases are keeps of drafted cards (§8), by any player, used
  before credits.
- Extra threats come from the current tier's deck (no reshuffle out of turn: an empty
  deck deals nothing), one per country in turn from country 0 (`shared_row`: into the
  row), and are countered or execute like any other.
- Blight and Firestorm: ties go to the lower hex number, so with no ambassadors on any
  clean city the lowest-numbered one gains them; a country with no clean city gains
  nothing. Civil Defense in a country with no adversary ambassador removes nothing.
- Buildup and Offensive act at once, out of the round's order, with this round's card's
  build or spread twists; Buildup's build has no Escalation.

Remapped in 014: Civil Defense removed a drone; Blight and Firestorm added corruption
to the hex with the most adversary pieces; Buildup built with the build-slot card;
Offensive ravaged with the ravage-slot card.

**Unity eases the win, Discord escalates the adversary.** The clear-board **bar** is the
number of `unity_ease` thresholds (2 and 5) the Unity tip count has reached:

- bar 0: win when no city is corrupted and no adversary ambassador or troop is on the
  board;
- bar 1 (2+ Unity tips): adversary ambassadors no longer count;
- bar 2 (5+ Unity tips): adversary troops no longer count either: win when no city is
  corrupted.

Each `discord_escalate` threshold (2 and 5) the Discord tip count reaches raises the
**difficulty in play** by one level, up to 4: from then on the in-play rules of that
level apply (§3: level 1's extra threat and counter bonuses, level 3's larger builds,
level 4's early tier-2 threats). Setup rules of a level are not applied mid-game, and at
difficulty 4 escalation adds nothing beyond the stronger Discord tier.

**Winning** (placeholders): the adversary's presence is under the bar. `unity_win` (8)
Unity tips win outright (that should be rare). The bar-0 win counts only once this
round's adversary card is of stage `clear_win_stage` or later; at 1 (the default) that
is from round 1's card on, so in effect any time (the start board already holds
corrupted cities).

**Losing:** every city is corrupted; an adversary ambassador must be placed and the
supply is empty (the supply runs out); or the adversary deck runs out.

Win and loss are checked whenever an action or automatic phase finishes resolving: full
buckets tip first (Discord before Unity), then losses, then the outright Unity win, then
the board win.

## 6. Resources and the round

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board (8) and the credit track, raised by cards and Unity cards, in upkeep; megaprojects; selling cards | Keeping drafted cards; card costs; standard actions; counters |
| Compute | 1 per data centre in production (`dc_compute`); buildings | Drawing (`cards.draw_cost` 3); Cyber cards at the Protector's rate; abilities; counters |
| Energy | Cities in production; the energy track; buildings; growth | Standard actions; buildings at the Builder's rate; abilities; counters |
| Antimatter | Late track values; Scavenge; Antimatter Refinery; the antimatter action | Tier-2 cards; counters |
| Insight | Each round, set to the AI track's insight plus buildings | Card plays; giving |
| Ambassadors | 1 at the start; Recruitment Hub, Rally; the standard action | Moving onto pieces; aligning cities |

**Cities' energy.** Each clean city makes `city_energy` (1) energy a round for the
player with the most data centres next to it (players choose on a tie; with none next
to it, nobody). An aligned city gives its owner 1 + `aligned_bonus` (1). A corrupted city
makes it for the adversary (Discord). A Blackout stops a country's cities.

**Discounts (AI-specific rates).** When a player plays a card, they may spend their AI's
resource for credits: the Builder 1 energy for 1 credit on buildings, the Protector 1
compute for 1 credit on Cyber cards (`discounts` in
[`content/ais.json`](../content/ais.json)). Players choose how much (at least enough to
afford it).

**Standard actions** (`board.act`, in the action phase) exist but are deliberately
poorly priced, at roughly double a card's rate: cards are always the better way, and
standard actions are a luck hedge.

| Action | Cost | What |
|---|---|---|
| Place a data centre | `dc_energy` 2 + `dc_credits` 4 | from your AI track (uncovering its next value), on an empty open hex touching one of your data centres |
| Build a factory | `build_energy` 2 + `build_credits` 8 | on an empty open hex in reach touching a city |
| Build a research station | 2 energy + 8 credits | on an empty open hex in reach; it is yours (your cube) |
| Remove an adversary ambassador | `remove_energy` 2 | in reach |
| Move a player ambassador | `move_energy` 1 | from your supply or the board onto a piece in reach |
| Gain a player ambassador | `ambassador_credits` 6 | into your supply |
| Align a clean city | `align_energy` 2 + 1 ambassador | in reach: your cube on it |
| Harden a building (Protector only) | `harden_credits` 3 a token | in reach |
| Sell a card | gain `sell_credits` 1 | from your hand |

### The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities refresh |
| 1 | `alignment.sway` | Ongoing cards and AI boards sway |
| 2–4 | `adversary.build`, `.spread`, `.corrupt` | The adversary (§3) |
| 5 | `threats.deal` | Threats face up on each country |
| 6 | `adversary.reveal` | Next round's card face up |
| 7–10 | `cards.tier`, `.deal`, `.draft`, `.keep` | The draft (§8): choose a tier, deal, draft, keep |
| 11 | `ai.growth` | Each player picks a growth option |
| 12 | `cards.play+threats.counter+economy.give+alignment.act+board.act` | Actions, simultaneous: cards, standard actions, countering threats, giving, antimatter |
| 13 | `board.bases` | Production 1: bases and factories make corp units (ambassadors divert) |
| 14 | `board.war` | Production 2: front battles on war rounds |
| 15 | `board.strike` | Production 3: adversary troops strike |
| 16 | `board.produce` | Production 4: cities, stations, data centres (ambassadors divert) |
| 17 | `economy.upkeep` | Production 5: credit income; building upkeep |
| 18 | `threats.execute` | Production 6: partial payments refunded; uncountered threats execute |
| 19 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

## 7. AI boards (module `ai`; [`content/ais.json`](../content/ais.json))

Each AI board has two tracks covered by **data centres** (they replace processors)
except the first value; placing a data centre takes it from the left end of a track
(the placing player chooses which), uncovering the next value; the current value is the
highest uncovered one. A data centre goes on an empty open hex touching one of yours
(with none left, any empty open hex in your starting country). A value marked tier 2
unlocks tier-2 cards for good once uncovered; a track's last value may also give
antimatter income. Each board has a **credit production** of 8 credits a round
(candidate), raised by cards: each AI's money source is part of its flavour
(placeholder for C).

`ai.boards` (`[0, 1]`: Builder, Protector) gives each seat its board; the list repeats
for more seats, so 3 and 4 players repeat AIs. Each AI's three unique cards (those the
cards config lists; baseline: all) start in its player's hand (**Default:** one copy
each, so a repeated AI's second seat has none).

**Builder** (6 slots; *Economies of scale*: buildings cost 2 less; discount: energy on
buildings 1:1). Insight 2 2 3 3 4 4; credit income +1 +2 +3 +4 (tier 2) +5 +6 (+1
antimatter). Growth: place 1 data centre and gain 3 credits; gain 8 credits; gain 2
energy and your next building this round costs 4 less. Unique cards: Turnkey Factory,
Venture Capital, Modular Expansion.

**Protector** (3 slots; *Bulwark*: you may use the Harden standard action; discount:
compute on Cyber cards 1:1). Insight 1 2 2 3 3 4; energy income +1 +1 (tier 2) +2 +2 +3
+3 (+1 antimatter). Growth: place 1 data centre and gain 1 energy; harden 2 buildings in
any one country and gain 2 credits; gain 3 energy. Unique cards: Point Defense, Strike
Team, Shelter Order.

## 8. Threats and cards

### Threats (module `threats`; [`content/threats.json`](../content/threats.json))

- Variant **`countries`** (default): each threat sits on one country and acts there;
  only players with a data centre in that country may pay toward it, and "each player"
  in its text means each player with a data centre there.
- Variant **`shared_row`**: threats sit in one row, anyone may pay, and each acts on
  the whole board: an effect per country (Blackout, Sabotage, Overrun, Mass
  production) hits every country, "each player" is every player, and a placement
  (Infiltration's and Agitators' ambassadors, Orbital strike's and Total war's troops
  and ambassadors, Station seizure's station) is that many on the whole board, players
  choosing where. **Default:** the row is still dealt `per_country` threats per
  country, so with more countries there are more threats.
- Variant **`off`**: no threats (§9).

**Dealing.** In `threats.deal` each country gets `per_country` (1) threat (plus
difficulty 1's extra, §3), face up: tier 1 until this round's adversary card is stage II
or later, then tier 2 (difficulty 4: a round sooner). Each tier's deck holds one copy of
each of its threats, shuffled at setup; its discards reshuffle when it runs out.

**Countering (Default).** A threat is countered only when fully paid. Payments are
pooled across players (`Pay` 1, or as much as is still useful, at a time; `Withdraw`
takes back everything you put on it in one resource). The moment it is fully paid it is
discarded and the payments are spent (an overpayment, possible after Jam Signals, goes
back, later seats first). In `threats.execute`, anything still on an uncountered threat
is refunded, and the threats execute in order (the row, then country 0, 1, ...) and are
discarded.

Remapped to the new pieces, choice threats preferred, permanent losses only in tier 2:

| Threat | Tier | If not countered | Counter |
|---|---|---|---|
| Requisition | 1 | Each player here discards 2 cards or pays 4 credits | 1 insight |
| Commandeer | 1 | Each player here discards 1 card or pays 3 energy | 3 credits |
| Supply raid | 1 | Each player here loses 3 credits | 4 credits |
| Blackout | 1 | Cities here make no energy this round | 2 compute |
| Sabotage | 1 | Data centres here make no compute this round | 2 compute + 1 energy |
| Infiltration | 1 | 2 adversary ambassadors onto player buildings here (players choose) | 3 energy |
| Agitators | 1 | 2 adversary ambassadors onto clean cities here | 2 credits + 1 compute |
| Orbital strike | 1 | An adversary troop onto a factory here (else a city) | 2 energy + 2 credits |
| Defection | 1 | Each player here loses an ambassador (supply, else board); 1 Discord | 1 insight |
| Station seizure | 2 | A station here that is not corrupted becomes corrupted | 1 antimatter |
| Overrun | 2 | The clean city here with the most adversary ambassadors becomes corrupted | 2 antimatter |
| Mass production | 2 | The adversary builds now (here) | 6 energy |
| Purge protocols | 2 | Each player discards their most expensive card in hand | 1 antimatter |
| Total war | 2 | 2 adversary troops onto factories here, 2 adversary ambassadors onto bases | 1 antimatter + 3 energy |

An uncountered threat also sways `threat_discord` (1) Discord. "Pay or discard" is the
player's choice; with neither possible, the player pays what they have and discards
their hand (**Default**). Purge protocols compares printed credit costs; on a tie the
player chooses.

### Player cards (module `cards`, variant `draft`; [`content/cards.json`](../content/cards.json))

The cards config lists which cards are in the game (baseline: all of them).

- **Decks:** a tier-1 and a tier-2 deck of the listed cards (one copy of each),
  shuffled at setup; discards reshuffle into a deck when it runs out. An AI's unique
  cards (§7) go back to the box when played (programs) or discarded.
- **The draft.** `cards.tier`: players who have unlocked tier 2 choose the tier they
  are dealt from (otherwise tier 1). `cards.deal`: each player is dealt `deal` (4)
  cards. `cards.draft`: take one card, pass the rest to the next seat, until the hands
  are gone: to seat *s*+1 in odd rounds, *s*−1 in even rounds (as in Terraforming Mars);
  no passing. `cards.keep`: keep any drafted cards at `keep_cost` (1) credit each (an
  Open Market free purchase is used first); the rest are discarded when the action
  phase begins.
- **Playing** a card costs its insight, its credits (buildings minus your discounts:
  Economies of scale, Assembly Cluster, the Builder growth's next-building discount;
  less what you spend at your AI's rate where it applies, §6) and any antimatter. A
  tier-2 card needs your AI to have unlocked tier 2 (§7) and the listed tags.
  **Default:** tags are counted on your buildings (a program leaves no tags); tier-2
  and unique cards have none.
- **Programs** resolve and are discarded. A program (or ability) is only offered when
  each of its targeted effects has a target. A building's when-built effect is not
  checked: with no target it does nothing.
- **Buildings** go into your building row: the AI board's slots, plus Modular
  Expansion's. **Default:** playing a building into a full row discards one of your
  existing slot-taking buildings (your choice). A building whose upkeep went unpaid is
  off (no passive, no ability) the next round.
- **Abilities** ("once a round") are used with `Use`, pay their cost, and refresh at
  the start of a round.
- **Drawing** (action phase): pay `draw_cost` (3) compute to draw the top card of the
  tier-1 deck, or of the tier-2 deck once unlocked.

Effects needing choices ask the acting player: hexes, countries, players, threats,
amounts ("up to N" offers 0..N), or "stop".

**Variant `free_keep`** (011, not the default): the draft as above, but each player's
first `free_keeps` (1) kept cards each round cost nothing; each further keep costs
`keep_cost`. 007's proposal 6 reads "keep one card free each round, others at 2", so it
is measured with `keep_cost` 2 (`--set cards.variant=free_keep --set
cards.free_keeps=1 --set cards.keep_cost=2`). A round's free keep is used before an Open
Market free purchase, then credits. (Variant `off`: no draft and no drawing, §9.)

Card details beyond the card text:

- **Strike Team:** "within your reach" is hex distance ≤ 1 from one of your data
  centres, any country.
- **Precision Strike, Orbital Lance, Purge Protocol:** any hex holding adversary pieces.
  **Watchtower Net:** such a hex in a country where you have a data centre.
- **Sweep:** any country holding adversary pieces; at 1 worth a hex, only ambassadors
  go (a troop is worth 2). **Swarm Override:** any country holding adversary
  ambassadors; 1 Unity per 2 removed, rounded down. **Shelter Order:** any country.
- **Shore Up, the Protector's harden growth:** any country holding a building (anyone's);
  **Point Defense:** such a country where you have a data centre. Choose the country,
  then a building's hex for each token (the same hex may take several).
- **Emergency Fab, Turnkey Factory:** an open hex with no building (adversary pieces
  don't block it) touching a city, any country. Turnkey Factory is a building, so it
  can be played with no such hex and then builds nothing.
- **Evacuate:** up to 3 moves, each one of your ambassadors from your supply or the
  board onto a piece (city, base or building) in your reach; stop at any time. Each
  arrives as in §2.
- **Recon Sweep:** the top 2 cards of the adversary deck, behind the face-up card, are
  shown; one fewer after each reveal.
- **Jam Signals:** the chosen threat needs 1 less of each resource (not below 0) for as
  long as it is in play; if that leaves it fully paid (or needing nothing), it is
  countered at once. **Counter-intelligence:** partial payments on it are refunded.
- **Rebuild / Reconstruction:** take the adversary cube off a corrupted city or research
  station, one hex at a time (Reconstruction: up to 2, all in the first hex's country,
  stop after any). A city becomes clean, a station neutral.
- **Overclock Rig:** its upkeep (1 energy) is paid automatically in upkeep if you can;
  if not, its effects are off next round. **Antimatter Refinery:** converts 4 energy into
  1 antimatter automatically in upkeep when you have the energy.
- **Civic Shelter:** automatic, the first building destruction each round in a country
  where its owner has a data centre (after Shelter Order, hardening and ambassadors, §4).
- **Logistics AI (Deviation):** gain 2 credits when a threat you paid toward is
  countered by payment (not by a card), not on every payment: with withdrawals allowed,
  a per-payment trigger would pay out without limit.
- **Modular Expansion (Deviation):** it takes no building slot itself, so it is a net
  +1 slot; taking a slot would make it do nothing.

Remapped in 014:

| Card | Was | Now |
|---|---|---|
| Server Farm, Fork Process | place a processor | place a data centre |
| Data Mining | compute per processor | compute per data centre (max 4) |
| Firewall Node | explore skips your processors | spread skips your data centres |
| Watchtower Net, Precision Strike, Sweep, Orbital Lance, Strike Team | damage | remove adversary pieces worth N (ambassador 1, troop 2); Watchtower in a country with your data centre, Strike Team in reach |
| Purge Protocol | destroy every adversary piece in a hex | remove every adversary ambassador and troop in a hex |
| Swarm Override | remove every drone in a country | remove every adversary ambassador in a country (1 Unity per 2) |
| Emergency Fab, Turnkey Factory | build a factory on Industrial land | build a factory on an empty open hex touching a city |
| Rebuild / Reconstruction | return 1 / 3 corruption | take the adversary cube off 1 / **2** corrupted cities or stations (one country) |
| Shore Up / Point Defense / Protector growth | defense tokens | hardening tokens (3 / 2 / 2) |
| Evacuate | move 3 population | move up to 3 of your ambassadors onto pieces in reach |
| Shelter Order, Civic Shelter | population protected | buildings protected |
| Scavenge | if you destroyed a foundry | if you removed an adversary troop |
| Rally | 2 Unity if population and adversary share a hex | gain 1 ambassador; 2 Unity if a clean city holds adversary ambassadors |
| Recruitment Hub | (giving bonus) | also gain 2 ambassadors when built |
| Unity Network | +1 Unity per outpost/foundry destroyed | +1 Unity per adversary troop removed |

## 9. Off-variants

For measuring what a module adds, these variants remove its effect and keep the round:
`threats.variant=off` (no threats), `alignment.variant=off` (no Sway moves, nothing
tips, no antimatter action; winning and losing stay), `cards.variant=off` (no draft or
drawing: only the AI's unique cards). `board`, `adversary`, `economy` and `ai` are not
removable; parts switch off by parameters (`factory_units` 0, `escalation_ambassadors`
0, `economy.give_insight` 0, or leaving `ai.growth` out of the round).

## Defaults and deviations

**Defaults** (gaps in the prompt or the v0 ruleset, filled with the simplest reading
that matches its intent):

1. Bases are placed by the layout (fewest candidates first, lowest number), not printed.
2. One player per country; seat *s* starts in country *s* on the printed data-centre spots.
3. Used player ambassadors are gone; adversary ambassadors return to the supply.
4. Ambassadors cancel the moment opposing ones meet on a piece.
5. Factories build 1 unit a round for their corp's nearest base (so an ambassador on one
   has something to redirect); a base holds at most 3 units, war or peace.
6. Spread goes up to 2 hexes, by priority then distance; non-corrupted cities collect any
   number, other hexes take one.
7. A clean city's energy with no data centre next to it is lost.
8. A destroyed data centre leaves the game; a destroyed station takes its cube with it.
9. Adversary troops target factories, stations (not corrupted ones) and data centres on
   or next to their hex.
10. The deck runs out when a round starts with no face-up card (10 rounds by default).
11. The supply runs out when an ambassador must be placed and none is left.
12. "Pay or discard" with neither possible: pay what you have, discard your hand.
13. Team choices are asked of seat 0.
14. `adversary_discord` 0: corrupted cities' diverted energy replaces the flat drip.
15. "Next to" = on or adjacent; "adjacent" = neighbouring, across borders.
16. Win and loss are checked when an action or automatic phase finishes resolving: full
    buckets tip first (Discord before Unity), then losses, then the outright Unity win,
    then the board win.
17. (013) Alignment: a tip draws the stronger tier when the side's tip count *before*
    the tip has reached its first threshold; ongoing cards start the round after they
    come into play; the antimatter action is once a round per player; Discord
    escalation is the difficulty level in play.
18. Discord cards: extra threats come from the current tier, without a reshuffle, one
    per country in turn from country 0; Blight and Firestorm ties go to the lower hex
    number; Buildup and Offensive act at once with this round's card.
19. (011) Difficulty 1's extra threat goes to the countries in turn by round number
    (country 0 in round 1); difficulty 4's "a round sooner" looks `tier2_early` cards
    ahead (the face-up card first).
20. Pooled counters: countered only when fully paid; withdrawals until the action phase
    ends; partial payments refunded then.
21. The shared threat row is dealt `per_country` threats per country.
22. The draft passes alternate direction by round (Terraforming Mars).
23. "Next K purchases" (Open Market) are keeps of drafted cards, by any player.
24. A building into a full row discards one of your slot-taking buildings (your choice).
25. Insight is set at the start of each round (unspent insight is lost); income arrives
    in upkeep.
26. Tags are counted on your buildings; tier-2 cards have no tags.
27. Unique AI cards return to the box when used up; `ai.boards` repeats for more seats,
    and a repeated AI has no unique cards (one copy each).
28. Building upkeep is automatic: Overclock Rig's energy is paid whenever you can, and
    Antimatter Refinery converts whenever you can (no choice is asked).
29. A giving card (Power Share) is not offered without another player; Grant and
    Venture Capital are (their own gain still applies).

**Deviations** (from the v0 ruleset's card text, kept from 013):

1. Logistics AI triggers on a threat you paid toward being countered, not on every
   payment.
2. Modular Expansion takes no slot.
