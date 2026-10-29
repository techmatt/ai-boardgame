# Rules as implemented

What the code implements: **rules revision 6, the 014 board model** (prompt A of three):
cities, research stations, factories, military bases and data centres, with
ambassadors as the adversary's main pressure, the Militarist against Builder and
Protector, 1–4 countries. 013's Alignment is kept as it was. Today's cards, AI unique
cards and AI boards are carried over as **placeholders**, remapped where they named a
removed piece (§8); the card economy is unchanged. Cards (B) and AI boards (C) come
later.

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
prompt left a gap, the choice made is marked **Default** and collected at the end.
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

Discord tips raise the difficulty in play (§5) as before; the setup rules of a level
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

Alignment is 013's, unchanged: the Undecided, the Unity and Discord buckets
(`tip_per_player` 2 × players), tips into the small Unity and Discord decks
([`content/alignment.json`](../content/alignment.json)), Discord escalation at 2 and 5
Discord tips, the antimatter action. Sources of Sway now:

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

Alignment cards remapped: Civil Defense removes 1 adversary ambassador per country;
Blight and Firestorm add 1 / 2 ambassadors to each country's clean city with the most
(ties: lower number); Buildup builds now; Offensive spreads now (with this round's card).

**Winning** (placeholders): no corrupted city, no adversary ambassador and no adversary
troop on the board. Unity tips ease the bar: after 2 (`unity_ease[0]`) ambassadors no
longer count; after 5 (`unity_ease[1]`) troops no longer count either. `unity_win` (8)
Unity tips win outright (that should be rare). `clear_win_stage` is 1: the clear-board
win counts at any time (the start board already holds corrupted cities).

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
| 7–10 | `cards.tier`, `.deal`, `.draft`, `.keep` | The draft, unchanged |
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
(players choose), uncovering the next value. Each board has a **credit production** of 8
credits a round (candidate), raised by cards: each AI's money source is part of its
flavour (placeholder for C).

**Builder** (6 slots; *Economies of scale*: buildings cost 2 less; discount: energy on
buildings 1:1). Insight 2 2 3 3 4 4; credit income +1 +2 +3 +4 (tier 2) +5 +6 (+1
antimatter). Growth: place 1 data centre and gain 3 credits; gain 8 credits; gain 2
energy and your next building this round costs 4 less.

**Protector** (3 slots; *Bulwark*: you may use the Harden standard action; discount:
compute on Cyber cards 1:1). Insight 1 2 2 3 3 4; energy income +1 +1 (tier 2) +2 +2 +3
+3 (+1 antimatter). Growth: place 1 data centre and gain 1 energy; harden 2 buildings in
any one country and gain 2 credits; gain 3 energy.

## 8. Threats and cards

### Threats (module `threats`; [`content/threats.json`](../content/threats.json))

Variants `countries` (default: each threat on one country; only players with a data
centre there pay toward it, and "each player" means those), `shared_row` and `off`, as
before. Countering is pooled and only when fully paid; partial payments come back.
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

An uncountered threat also sways `threat_discord` (1) Discord. With "pay or discard"
neither possible, the player pays what they have and discards their hand (**Default**).

### Player cards (module `cards`; [`content/cards.json`](../content/cards.json))

The draft, keeping, playing, buildings and abilities are unchanged. Remapped cards:

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

## Defaults

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
