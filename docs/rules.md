# Rules as implemented

What the code implements: **rules revision 12** (021: **card primitives and the resource
model**, below; on 020's revision 11: five bug fixes from the bug hunt; on 019's revision 10: adversary troops **march**
toward player infrastructure and nodes are **hard to remove**, below; on 018's revision 9,
the structural fixes and the calibration; on 017's revision 8): the 014 board model with player
**presence** in place of data centres, the adversary's pieces as a readable **ladder**
(ambassadors, nodes, troops, corrupted cities and stations), **factories** that grow
from cities on printed industrial zones and make a new resource, **hardware**, a printed
**production/war clock**, and the **purge** win eased by Unity. 017 gives adversary
compute something to spend on, a shared **adversary project deck** (§3), made each
Discord tip advance the adversary troops (019 replaced that with the march, §4), drops 013's Discord escalation, makes
every Alignment card **one-shot** and adds Unity sources (§5). The Militarist against
Builder and Protector, 1–4 countries. Today's cards, AI unique cards, AI boards and the
project deck are **placeholders** pending redesign, ported onto 021's primitives with
as few content changes as fit the new resource model (§8); the ~150-card content set is a
later item.

**Status (021).** Every 021 change is a **candidate** except AI-board powers paid by
sacrificing tags, which Matt marked **locked** (2026-10-29). The rows marked (021) are
021's adaptations and defaults; results: `experiments/results/screen-021-cards/`.

| 021 rule | Was | Status |
|---|---|---|
| **Players have no compute**; compute is the adversary's alone (§3) | presence-track compute, drawing, Cyber discounts | candidate (Matt) |
| Presence-track spaces that gave compute give insight, credits or energy; each board's last space gives 1 antimatter a round (§7) | +1 compute on spaces 1-2 | candidate (Matt; the values 021) |
| Cyber cards that made or spent compute are **hacks**: Data Mining cuts a node off (no compute in the next build) and drains 1 project compute (§8) | compute per presence | candidate (Matt; the card 021) |
| An adversary ambassador on a lone presence costs its owner 1 insight next round and adds 1 compute to the cheapest project; it is used up (§2) | leaked 1 compute, 1 Discord | candidate (Matt) |
| **City energy cubes** (§6): an aligned city's goes free to its aligner; a neutral city's can be bought by a player with presence next to it for `city_energy_credits` (2) credits, else it is lost; a corrupted city's goes to the adversary (Discord, as before) | the most presence next to a clean city took it free; aligned +1 bonus | candidate (Matt; the price 021) |
| `board.corp_energy` (a list by country, 1 each): a megacorp parameter, 2 for 2 energy a city | `city_energy` 1 | candidate (Matt; off by default) |
| Energy pays presence, Power cards (`energy` in a card's cost), standard actions and **hardening** (`harden_energy` 1) | hardening 1 hardware | candidate (Matt) |
| **Hardware** is a requirement in some buildings' cost (`hardware`); it comes from player-owned factories and Industry cards (Fab Line +1 a round, Holding Company); the standard action sells 1 hardware for `hardware_credits` (1) credit | bought at 4 credits | candidate (Matt) |
| **Antimatter**: no tier-1 source (Scavenge gives 3 credits instead); the last presence-track space and tier-2 cards (Antimatter Refinery) make it; the Discord-for-antimatter action stays (§5) | Scavenge 1 | candidate (Matt; keeping the action, 021) |
| **Small numbers**: card costs halved (rounded up; Megastructure 13, Orbital Lance 10); credit production 10 (Builder) / 9 (Protector) plus a starting building's +1; base insight 3 a board (+1 on the first space) | costs 0-25; 8 credits; insight 2/1 | candidate (021) |
| **Opening** (round 1): each player is dealt `opening_deal` (8) tier-1 cards, keeps `opening_free` (4) free and each further one for `opening_keep_credits` (5) credits; no draft in round 1; `ai.start_credits` 20 | 25 credits and a round-1 draft | candidate (Matt; 20 and the no-draft round 021) |
| **The draft** (rounds 2+): each player builds a hand of `deal` (4) **card by card, choosing each card's tier** (each pick deals that deck's top card face up into the hand), then hands pass as before | one tier for the whole hand, tier 2 only once unlocked | candidate (Matt) |
| **Keep costs** insight: `keep_tier1` 1, `keep_tier2` 3, `keep_tier2_upgraded` 2 once the board's keep upgrade (presence-track space 3) is uncovered; any shortfall at `insight_credits` (5) credits an insight, for keeps only | 1 credit a card | candidate (Matt) |
| **No tier-2 gate**: tier 2 is drafted and played by anyone | unlocked on track space 3 | candidate (Matt) |
| Playing a card costs no insight (021: insight pays keeps, counters and giving) | 0-2 insight a card | candidate (021) |
| **Drawing** with compute is gone (`Action::Draw`, `cards.draw_cost`) | 3 compute a card | candidate (021) |
| **Tags** Science, Industry, Cyber, Power, Diplomacy, Security, 1-3 a card; tags count on every card in play (buildings, splayed ones and programs); requirements are checked only on play | economy/cyber/power/unity/discord on buildings only | candidate (Matt) |
| **Programs** stay in your play area (their tags count; they can be sacrificed); **events** (strikes, hacks, one-off gains) are discarded | programs discarded | candidate (Matt; which are events 021) |
| **Buildings**: `ai.slots` (Builder 4, Protector 3) slots; building over a building **splays it down** (its effect ends, its tags stay); a building may need to replace one of yours carrying a tag (`replaces`: Fusion Plant over Power, Megastructure over Industry); a building may give a one-time permanent gain (Trade Office: +1 credit production); each board starts with a building carrying a built-in tag (Foundry Core, Industry; Sentinel Post, Security) | 6/3 slots, a full row discarded one | candidate (Matt) |
| **Token engines**: tokens live on cards that print a type (Datasets, Followers, Capital, Agents); patterns built: gain N a round, add tokens to a card of a type (Seed Funding), spend at a rate (abilities), adversary effects that remove them (Sabotage) | none | candidate (Matt) |
| **AI-board powers**, once a round, paid by sacrificing played cards carrying the cost's tags (a program alone; a building with its whole stack; never an event), plus optional other costs; one placeholder per board (Builder *Retool*: 2 Industry + 1 any, gain 5 credits and 2 hardware; Protector *Lockdown*: 2 Security + 1 any, remove 3 worth in reach and sway 1 Unity) | none | **locked** (Matt) |
| Field Office: while you have it, cards discarded from hand count their tags for powers | none | candidate (Matt's primitive; the card 021) |
| **Movement keywords**: the common add-on "move one of your ambassadors up to 2 hexes" (onto a piece; on Rally); **Spread** (rare: each of your ambassadors on the board may move 1 hex onto an adjacent piece; Diaspora, tier 2); **Consolidate** (move up to N of your ambassadors within your reach onto one piece in your reach; Rally Point, N 3). A moved ambassador arrives as in §2 (cancels, claims, waits) | ambassadors moved only from supply or board into reach | candidate (Matt; the cards 021) |
| **Two presence tracks** a board: placing presence takes the next token from the track the player chooses; each board has a placeholder second track of 3 spaces (§7) | one track of 6 | candidate (Matt; contents placeholders) |
| `adversary.troop_health` (moved from `board`), Unity Network (was Opposition Network), card costs are plain numbers (no `{"param"}`) | | 015's calls 7 and 12 |
| `remove_energy` 1 (was 2) | | candidate (021: tuning, below) |

**Not built in 021** (follow-up): the single `Op` enum (015's call 2: `Effect`, `Passive`,
`ThreatOp`, `Twist` and `AlignOp` stay separate enums; the `Tag` and `Resource` sets are
checked string lists, `TAGS` and `RESOURCES`, not Rust enums); the "count as power" token
pattern (unclear whether it means tokens spent as energy or counted as tags for powers);
a Cyber token type. The `keywords` field exists (`hack`, `tokens`, `move`, `spread`,
`consolidate`) but nothing reads it.

**For new players (021).** In the first round you are dealt 8 cards and keep 4 free (more
at 5 credits each). From then on, each round you build a hand of 4 cards one at a time,
choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2
deck; you take one card and pass the rest to your neighbour until the hands are gone, then
pay insight to keep what you drafted (1 for tier 1, 3 for tier 2). Insight is this round's
attention: what you don't spend is lost. Cards cost credits to play. Buildings go into your
AI board's 3-4 slots; once they are full, a new building goes on top of an old one, which
stops working but keeps its tags. Programs stay in front of you; events are used and
discarded. Every card in front of you shows tags (Science, Industry, Cyber, Power,
Diplomacy, Security): they unlock some cards, and once a round you can sacrifice cards
with the right tags to use your AI board's power.

**Status (019).** Every 019 change is a **candidate** (Matt, 2026-10-29). The first
four rows are Matt's; the rest are 019's adaptations to keep the game winnable once
nodes became hard to remove (results: `experiments/results/screen-019-troops/`).

| 019 rule | Was | Status |
|---|---|---|
| Adversary troops **march** in the adversary phase: 2 steps each (`troop_steps`), a step destroys a target on or next to the troop or moves 1 hex toward the nearest; players choose on ties (§4) | 017's Discord-tip advance and 016's strike in place in production | candidate (Matt) |
| Troop targets: presence and player-owned stations and factories, never cities (§4) | any building not adversary-owned or corrupted, and presence | candidate (Matt) |
| A troop that strikes is used up (kept from 018, `troop_spent` 1); a troop at a base still absorbs a strike next to it | same | candidate (Matt; the screen gave no reason to change it) |
| No standard action removes a node; only tier-2 effects do: Node Takedown (card) and Decommission (Unity card) (§3) | `node_energy` 2 energy, in reach | candidate (Matt) |
| Nodes stop counting against the purge at 2 Unity tips; troops at 1 (`unity_ease` 1, 1, 2; §5) | nodes always counted; troops at 3 | candidate (019) |
| Removing a node sways `alignment.node` 2 Unity | 1 | candidate (019) |
| Freeing a corrupted city sways `alignment.free_city` 3 Unity | 2 | candidate (019) |

**Status (020).** Bug fixes, not design changes; Matt to confirm the first
(`docs/qa-findings.md`).

| 020 fix | Was | Status |
|---|---|---|
| An uncountered Blackout or Sabotage hits the **next** round's production and upkeep (§8) | "this round": threats execute after production and upkeep, so the flag was cleared before anything read it and both threats did nothing but their compute | fix (020) |
| The move standard action is never offered onto the hex the seat's only ambassador already holds (§6) | offered; it charged the energy and moved nothing | fix (020) |
| An adversary ambassador leaks a presence's compute only when the presence is alone on its hex (§2) | the leak took any adversary ambassador on a presence hex: a presence on a corrupted city stopped its spread for good, and one on a clean city or base ate the ambassadors gathering there | fix (020; the code's own comment said "with no other piece") |
| Sabotage only stops upkeep compute (§8) | it also skipped the leak in its country, a data-centre-era leftover that became reachable with the first fix | fix (020) |
| Purge the Unaligned's sacrifice offers the seat's own presence tokens (§3, Defaults 44) | it offered the free hexes where the seat could place one, so a "sacrificed presence" removed nothing | fix (020) |

**Status (018).** Every 018 change is a **candidate**. Matt's structural fixes
(2026-10-29) are the first five rows; the rest are 018's adaptations, made because the
fixes alone left every cell at 0% (results:
`experiments/results/screen-018-calibration/`).

| 018 rule | Was | Status |
|---|---|---|
| Only war destruction sways Discord; adversary troop strikes don't (§4) | every destruction | candidate (Matt) |
| A clean city's new factory goes to the player with presence next to its zone; neutral only with none (§4) | always neutral | candidate (Matt) |
| Removing a node: `node_energy` 2 | 3 | candidate (Matt) |
| A corrupted city's factory: next to an adversary ambassador if there is one, else the lowest-numbered free zone (§4) | a four-key order | candidate (Matt) |
| A node next to a site blocks claims on it (Terms) | presence on or next to a node didn't count | candidate (Matt) |
| An adversary troop that strikes is used up, `troop_spent` 1 (§4) | troops stayed | candidate (018) |
| A destroyed presence goes to its owner's reserve, placed before the track, `presence_returns` 1 (§4) | it left the game | candidate (018) |
| Supply `supply_per_country` 14 | 10 | candidate (018) |
| Corrupted cities build `city_build` 1; factories next to them `factory_build` 0 | 2; 1 | candidate (018) |
| Clean cities corrupt at `corrupt_at` 6 | 5 | candidate (018) |
| `free_city_at` 2 player ambassadors free a corrupted city | 3 | candidate (018) |
| Unity easing `unity_ease` 1, 3 (ambassadors stop counting at 1 Unity tip, troops at 3) | 2, 5 | candidate (018) |
| The adversary deck deals `stage_cards` 4, 4, 3: 11 rounds | 3, 4, 3 (10) | candidate (018) |
| Difficulty 1: `extra_threats` 2; difficulty 4: `supply_reduction` 4 (§3) | 1; 2 | candidate (018) |

**Status (017).** Matt marked these **locked**: Alignment cards are one-shot and
hidden, like fear cards; with the Undecided empty, Sway comes from the middle first, then
the other bucket (empty-pool rule B, `tug`); a player ambassador on a military base
sways Unity. The project deck is **near-locked** (Matt, 2026-10-29). Everything else
017 added is a **candidate**.

| 017 rule | Status |
|---|---|
| Alignment cards one-shot and hidden (no ongoing cards) | **locked** |
| Empty-pool rule B: an empty Undecided draws from the other bucket (`tug`, the default) | **locked** |
| A player ambassador on a producing military base sways `base_unity` Unity | **locked** (built since 013; tested) |
| One shared, staged adversary project deck; 3 visible; compute fills the cheapest; a full one resolves; a new row when all 3 have resolved | near-locked (Matt, 2026-10-29) |
| Nodes make 1 compute a round; about half the threats carry compute when they execute uncountered | near-locked (compute sources); the threat amounts are candidates |
| A player ambassador on a node stops its compute (not used up) | near-locked |
| Card primitives: remove compute from a project; discard a visible project (tier 2, very expensive) | near-locked; the two placeholder cards are candidates |
| Project costs (per player) and effects | candidate |
| Each Discord tip advances every adversary troop one hex toward player pieces, then it strikes | candidate (an exception to "no movement rules") |
| No difficulty escalation from Discord tips; Discord cards print stronger steps by Discord tip count | candidate |
| The Undecided smaller than twice a tip (3 a player against a tip of 2 a player) | candidate |
| Unity for freeing a corrupted city (2) and for removing a node (1) | candidate |

**Status (016).** Matt marked one decision **locked**: data centres are player
presence, placed with energy next to presence you already have (§1), and an
adversary-owned factory, or one holding an adversary ambassador, makes an adversary
troop (§4). Everything else 016 added is a **candidate**: the presence track and its
uncovers, reach and site claims, nodes, industrial zones and factory growth, hardware,
the clock, the purge easing.

| 016 rule | Status |
|---|---|
| Presence replaces data centres; 1 energy, next to your presence, 1 per hex | **locked** |
| A factory that is adversary-owned or holds an adversary ambassador makes an adversary troop | **locked** |
| Presence track (6 tokens, 1 on the board) and its uncovers | candidate |
| Reach (card and standard-action map targets within 1 of your presence) | candidate |
| Site claims (presence on or next to the site; a node contests it) | candidate |
| Spread never targets an empty hex; surplus returns to the supply | candidate |
| Nodes (made by a second spreading ambassador; 1 adversary compute a round; contest presence; 3 energy to remove) | candidate |
| Industrial zones; cities grow factories; factory ownership | candidate (Matt, 2026-10-29) |
| Factory troops at every one of the corp's bases | candidate (Matt, 016 message) |
| Peacetime: 3 troops a base, a troop absorbs a strike next to it | candidate |
| Hardware (factories make it; stations, hardening, 4 credits → 1) | candidate |
| The production/war clock | candidate |
| Printed fronts: every country has two at 3 and 4 countries | candidate (Matt, 016 message) |
| The purge and its Unity easing | candidate |

Gone with 017: ongoing Alignment cards (Volunteer Corps and Propaganda are one-shot now),
the Discord escalation of the difficulty in play (`adversary.escalation`,
`alignment.discord_escalate`; replaced by `alignment.discord_tier2`), and the
`Stats.adversary_ops` / `setup_ops` fields (now the generic per-round counter
`adversary.ops`). Gone with 016: data centres and their compute, the AI boards' two tracks, the player
"build a factory" standard action, the `overflow` adversary variant (nodes replace it),
the fixed war rounds (`board.war`; the clock replaces them) and the unused per-country
card strip. Gone with 014: population, processors, drones, outposts, foundries, uplinks,
typed land, defense tokens, the corruption pool, the explore → build → ravage pipeline.
Results and records from revisions 1–6 stay in `experiments/results/` as history; they
do not replay.

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
- **Presence**: a player's tokens on the board, one per hex, taken off their AI
  board's presence track (§7). Presence is a token, not a building: it sits on any land
  hex, with or beside other pieces.
- **Reach**: a player's reach is every hex within 1 of their presence. Every map target
  of a card or a standard action is in the acting player's reach.
- **Site claim**: aligning a city, or claiming a research station or factory, needs the
  player's presence on or next to the site, and **a node next to the site blocks claims
  on it** (018: one hop from the site; 016–017 voided presence on or next to a node,
  two hops from the site).
- **Piece**: a city, a military base, a building (factory, research station) or a
  presence token. Ambassadors sit on pieces.
- **Clean / corrupted / aligned / owned**: a city, research station or factory with no
  owner cube is clean (a station or factory: neutral); an adversary cube means corrupted
  (a factory: adversary-owned); a player cube means aligned (a city) or owned.
- **The ladder**: the adversary's pieces, weakest first: ambassadors (used up when they
  act), **nodes** (persistent), troops, corrupted cities and stations.
- **Players choose**: an `Action::Choose` in `legal_actions`. Team choices in the
  adversary's steps, production and threats are asked of seat 0 (an aligned city's
  factory placement, of its owner); a choice inside a player's own card, ability or
  standard action is asked of that player.
- **Hex number**: each piece's hexes are printed 1–16 row by row; ties go to the lower
  number.

## 1. The board (module `board`, variant `corps`)

### Country pieces

A game has 1–4 countries, one player each. Country *i* is printed piece *i*
([`content/map.json`](../content/map.json)), a 4×4 piece of 16 hexes placed by the
layout for that country count. Population is implicit: people live in cities.

Hex kinds: **city**, **open** land and **ocean** are printed; a **military base** goes
on an open hex of each front (below). **Industrial zones** are a printed marking on open
hexes next to cities (4 a country, 1–3 next to each city): factories go only there.
Research stations go on other open hexes, one building per hex. Nothing builds on ocean.

| # | Name | Cities (corrupted first) | Ocean | Station (node) | Industrial zones (setup factories) | Starting presence |
|---|---|---|---|---|---|---|
| 0 | Coastal Republic | 7, 6, 14 | 3, 12 | 4 | 2, 8, 11, 13 (2, 8) | 9 |
| 1 | Northern Union | 10, 7, 15 | 4 | 12 | 3, 14, 9, 16 (3, 14) | 6 |
| 2 | Highland Federation | 7, 2, 10 | — | 14 | 1, 6, 9, 15 (1, 6) | 11 |
| 3 | Delta States | 11, 6, 14 | 1 | 12 | 7, 13, 3, 8 (7, 13) | 5 |

Ocean hexes are on the board's outer edge in every layout the piece appears in; the
starting presence touches a city.

**Fronts are printed** (016, Matt): a front is a long edge shared by two country
pieces, marked on the tiles; pieces that only touch at the middle of the map are not
neighbours. Each layout lists its fronts (`fronts` in `content/map.json`): 2 countries,
one front; 3 countries (top, right, bottom-left), all three pairs, so every country has
two; 4 countries (top, left, right, bottom), a ring: top–left, left–bottom,
bottom–right, right–top (top–bottom and left–right touch only at the centre). Every
country has exactly two fronts at 3 and 4 countries (checked by a test). Each country
gets one military base per front, on an open hex that touches the other country's land
and holds no setup piece or industrial zone; a country's fronts with the fewest
candidate hexes pick first, each taking its lowest-numbered free candidate (**Default**:
bases are placed by the layout, not printed, because a piece's neighbours change with
the country count).

### Setup

- Each country: its first `start_corrupted` (1) cities get an adversary cube (difficulty
  4 with `adversary.extra_corrupted`, 0 by default, more); its research station starts
  corrupted, with a **node** on it (1 node a board); its two setup factories, **neutral**
  (**Default**: "a factory takes its city's owner" is for factories cities build; a setup
  factory touching the corrupted city would otherwise start adversary-owned).
- Each player (seat *s* starts in country *s*): 1 presence on the starting spot (the
  first token off their track), `start_ambassadors` (1) player ambassador,
  `ai.start_credits` (25) credits, and their AI board's credit production (8).
- The adversary's supply: `adversary.supply_per_country` (14; 018: was 10) ambassadors per country.
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
| City | that round's energy goes to the adversary (Discord) | waits; 2 (`free_city_at`; 018: was 3) on a corrupted city free it: its cube returns |
| Research station | that round's cube is the adversary's (owned or corrupted station) | on a neutral station the player can claim: the station becomes the player's (cube placed, ambassador used) |
| Factory | on a production step, an adversary troop on the factory's hex (**locked**) | on a neutral factory the player can claim: the factory becomes the player's; otherwise on a production step no troop is made, `factory_unity` (1) Unity |
| Presence | if the presence is alone on its hex (020: on a city, base, station or factory the ambassador is that piece's), in production its owner gets 1 less insight next round and 1 compute goes onto the cheapest adversary project; used up (021: it leaked 1 compute and swayed Discord) | waits (defense, below) |
| Military base | on a production step, `base_discord` (3) Discord instead of a troop | on a production step, `base_unity` (2) Unity instead of a troop |

- **Opposing ambassadors cancel.** When an ambassador arrives on a piece holding one of
  the other side, both are removed (the adversary's to its supply), with no Unity.
- **Defense:** a player ambassador on a building's hex is used up instead of the
  building when it would be destroyed.
- **Diverted** energy and compute sway `divert_discord` (1) Discord per unit.
- A neutral station's normal thing is nothing, so an adversary ambassador waits there.

Player ambassadors move only through cards and the standard action.

## 3. The adversary (module `adversary`, variant `militarist`)

### Each round

1. **Build** (`adversary.build`): next round's face-up card becomes this round's card.
   Each corrupted city gains `city_build` (1; 018: was 2) adversary ambassadors, and each factory
   next to (adjacent to) a corrupted city gains `factory_build` (0; 018: was 1). The card's twists
   add more. A stage-II card's **Escalation** then gives each military base
   `escalation_ambassadors` (1). Ongoing **projects** act (below). Last, each **node**
   makes `node_compute` (1) **adversary compute**, onto the projects; a node holding a
   player ambassador makes none (the ambassador stays).
2. **Spread** (`adversary.spread`): every hex holding several adversary ambassadors,
   fixed when the step starts, keeps one and sends each other one to a hex within
   `spread_range` (2). **Spread never targets an empty hex**: a destination holds a
   piece or is player-owned (presence, an aligned city, an owned station or factory);
   it is not ocean, and it holds no adversary ambassador unless it is a clean city
   (which collects any number) or a piece without a node (below). **Priority:**
   player-owned hexes, then clean cities, then other pieces; then the nearest; players
   choose among equals. A city that is not corrupted never spreads: it holds its
   ambassadors. Firewall Node's owner's presence is skipped. **A surplus with nowhere to
   go returns to the supply.**
3. **Corrupt** (`adversary.corrupt`): a city that is not corrupted holding `corrupt_at`
   (6; 018: was 5) adversary ambassadors becomes corrupted (an aligned city loses its player cube);
   its ambassadors return to the supply.
4. **Reveal** (`adversary.reveal`, after `threats.deal`): the top card goes face up as
   next round's card.

**Nodes.** A node is made when a spreading adversary ambassador lands on a piece that
already holds one (not a clean city, which collects them, and not a piece that already
has a node): both ambassadors return to the supply and a node goes on the piece. This
replaces 014's `overflow` candidate. Each board starts with 1 node (on its research
station). A node stays until removed: it makes 1 adversary compute a round, it
**blocks site claims** next to it (a site with a node on or next to it can't be claimed),
and it counts against the purge until Unity eases it (§5). **Nodes are hard to remove
(019, candidate, Matt):** there is no standard action for it any more (016–018: a
standard action, `node_energy` 2–3 energy in reach). Only tier-2 effects remove a node:
the tier-2 card **Node Takedown** (placeholder: 8 credits, 1 antimatter, 1 insight;
remove a node within your reach) and the tier-2 Unity card **Decommission** (remove a node
anywhere, players choose). Removing a node still sways `alignment.node` Unity (§5).
Starting reasoning: 3–5 nodes a board by mid-game, so about 6–15 adversary compute a
round at 3 players (016 screen: below).

### The project deck (017; near-locked)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../content/projects.json), 12 placeholders). The deck is
staged like the threat deck: `project_cards` (6, 6) are dealt, the cheap stage-1
projects shuffled on top of the big stage-2 ones. `project_row` (3) projects are face
up at once; when all of them have resolved (or been discarded), the next 3 are dealt.

- **Compute sources:** each node, 1 a round in the build (none under a player
  ambassador); and about half the threats carry compute, gained when they execute
  uncountered (§8).
- **Spending:** each compute point goes onto the **cheapest** visible project (ties: the
  first in the row). When a project holds its cost it **resolves**: a one-shot effect at
  once, or an **ongoing** one that acts in every round's build step from the next round
  on. Choices
  are the players' (seat 0; a sacrifice, each player's own). With no project left,
  compute is lost.
- **Cost** is printed per player (**Default**: the prompt's 12–15 and 25–30 compute are
  3-player numbers; a per-player cost keeps the timing at 2 and 4 players). Cheap: 6–7 a
  player (18–21 at 3 players); big: 10–12 a player (30–36). 017 raised the cheap ones
  from 4–5 (12–15 at 3 players), which landed the first project in round 2–3 of the
  017 screen, toward the target round 3–4
  (`experiments/results/screen-017-projects/projects.md`).
- **Counterplay:** a player ambassador on a node stops its compute and is not used up;
  Security Patch (placeholder) removes up to 4 compute from a project; Zero-Day Wipe
  (tier 2, 16 credits, 2 antimatter, 2 insight; placeholder) discards a visible project
  with its compute.

| Project | Stage | Cost / player | Effect |
|---|---|---|---|
| Disinformation | 1 | 6 | sway 3 toward Discord |
| Node Swarm | 1 | 6 | every node makes an adversary ambassador on its hex (the "every node produces an ambassador" event, here as a project) |
| Market Crash | 1 | 6 | every player loses 5 credits |
| False Flag | 1 | 7 | deal two extra threats |
| Infiltration Drive | 1 | 7 | in each country, the clean city with the most adversary ambassadors gains 2 |
| Rapid Buildout | 1 | 7 | the adversary builds now |
| Purge the Unaligned | 2 | 10 | every player sacrifices an ambassador or a presence |
| Recruitment Drive | 2 | 10 | ongoing: every round, an adversary ambassador on every clean city |
| Propaganda Network | 2 | 10 | ongoing: every round, sway 1 toward Discord |
| Total Offensive | 2 | 11 | the adversary spreads now; sway 2 toward Discord |
| Armoured Columns | 2 | 11 | in each country, an adversary troop onto a factory (else a city) |
| Station Coup | 2 | 12 | in each country, a research station that is not corrupted becomes corrupted |

**Defaults:** compute goes on one point at a time, so a full project resolves before the
next point lands; a sacrifice is any one of the player's ambassadors (supply or a hex)
or presence tokens, their pick; a sacrificed presence leaves the game. Ongoing projects
act in the round's build step only, not in an out-of-turn build (Buildup, Rapid
Buildout, Mass production).

**Calibration.** Since 018 (`city_build` 1, `factory_build` 0) a corrupted city makes
1 ambassador a round, plus the card's twists. Each producing piece uses one up a round,
players remove some, and the spread prefers player pieces. Measured at r11 (greedy, 40
games, 2 countries, difficulty 0): 24 games saw a new corruption, the first in round 5.9
on average (all in rounds 5–7); 16 saw none. The first adversary project resolves in
round 3.7. (014's build rate, 3 a round from a city and its factory, no longer holds;
its first corruption in rounds 5–6 still does.)

### The deck (the clock and the stager)

[`content/adversary.json`](../content/adversary.json) prints 13 cards: 4 stage I, 5
stage II, 4 stage III. A game deals `stage_cards` (4, 4, 3; 018: was 3, 4, 3) at random per stage, each
stage shuffled on its own, stage I on top: 11 rounds. The next round's card is always
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

Players **lose** when a round starts with no card to turn face up (after round 11 with
the default deal).

### Difficulty (`difficulty`, 0–4; each level includes the ones below)

| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules | |
| 1 | Two extra threats a round in all, countries in turn (018: was one) | `extra_threats` 2; `counter_bonus`, `counter_bonus_all` 0 |
| 2 | One stage-I card removed: stage II (escalation, tier-2 threats) a round sooner, and a 10-round clock (one fewer than the 11 of `stage_cards` 4, 4, 3) | `stage1_removed` 1 |
| 3 | Larger builds: each corrupted city gains 1 more | `build_bonus` 1 |
| 4 | A smaller supply (10 per country; 018: 4 fewer, was 2), tier-2 threats a round sooner; optionally a second corrupted city per country at setup (off: it starves the supply in round 2) | `supply_reduction` 4, `tier2_early` 1, `extra_corrupted` 0 |

Since 017 the difficulty is only the configured level: Discord tips no longer raise it
(§5).

**The curve (021 quick screen, r12, 16 games a cell, engine / greedy, %).** 2 countries:
d0 100 / 88, d1 100 / 81, d2 94 / 88, d3 75 / 75, d4 19 / 19; 3 countries: d0 88 / 69,
d1 94 / 50, d2 81 / 31, d3 75 / 31, d4 31 / 6; random 0 everywhere
(`experiments/results/screen-021-cards/cards.md`, with r11's and the untuned r12's).
The skill gap between engine and greedy widened at 3 countries: greedy keeps every card
it drafts and plays few.

**The curve (019 quick screen, r10, 16 games a cell, engine / greedy, %).** 2 countries:
d0 100 / 88, d1 88 / 94, d2 81 / 94, d3 62 / 56, d4 25 / 38; 3 countries: d0 100 / 88,
d1 81 / 81, d2 88 / 69, d3 62 / 56, d4 69 / 50; random 0 everywhere. With 16 games a
cell the noise is about ±12 points, so c3 engine's d4 above d3 and greedy above engine
at c2 d1–d2 are within it. Levers: d1 two extra threats a round; d2 the 10-round deck;
d3 corrupted cities build 2; d4 the supply 4 smaller.
`experiments/results/screen-019-troops/troops.md` (018's r9 curve:
`experiments/results/screen-018-calibration/calibration.md`).

**Why this ladder (014).** The supply loss bounds everything: one corrupted city makes
about 3 ambassadors a round per country against a supply of 10, so anything that doubles
the build (a second corrupted city) starves the supply within two rounds. The
corrupted-city-per-country and smaller-supply levels the prompt suggested were first
tried as levels 3 and 4 (second city, supply 7 per country): every game ended in round 1
or 2, before a decision mattered. Levels 3 and 4 were rebuilt from the milder options;
the curve is not tuned (014 quick screen: greedy 100 / 88 / 25 / 0 / 0 % at d0–d3, 16
games each).

## 4. Factories, the clock, war and troops

### The production/war clock

A printed round track (`board.clock`) says what each round's production holds: 1 a
**production step**, 2 a **war step**, 3 both; the last entry repeats. Starting track:
round 1 production, 2 war, 3 production, 4 war, 5 production, then production + war
every round from 6. Cards with the War twist (Border War, Blitzkrieg, Iron Doctrine)
make their round a war step too. **Default** (Matt confirmed): the clock gates only
factories, troops, hardware and war; the income every round (cities' energy, the
presence track, stations, upkeep, the adversary troops' march) is unchanged.

### Factories

- **Ownership:** an owner cube, as stations and cities have. A factory a city builds:
  an aligned city's is that player's, a corrupted city's is the adversary's, and a clean
  city's goes to **the player whose presence is next to the zone the players picked**
  (018, Matt: the most presence on or next to it; players choose on a tie, as for city
  energy), neutral only if no presence is there (016–017: always neutral). A player ambassador claims a
  neutral factory as it claims a station (§2, needs a site claim). Cards that build a
  factory (Emergency Fab, Turnkey Factory) build the player's own, on a free zone in
  reach. An owned factory keeps its owner if its city changes hands later.
- **Growth** (each production step, after troops and hardware): every city builds one
  factory. A clean or aligned city builds on a free industrial zone next to it, where
  the players choose (an aligned city's owner, else seat 0). A **corrupted** city builds
  at a free industrial zone in its own country **next to an adversary ambassador if
  there is one, otherwise the lowest-numbered free zone** (018, Matt; lowest number
  among several next to one. 016–017's four-key order is gone: no player ambassador,
  then an adversary ambassador on it, then nearest, then number). A city with no such
  zone builds nothing. Factories destroyed in war
  regrow this way.
- **Troops** (each production step): each base makes 1 troop for its corp; every
  factory makes `factory_units` (1) troop for its megacorp **at each of that corp's
  bases** (Matt, 016: at every front base, not just the nearest), so at 3 and 4
  countries a factory makes 2. An adversary-owned factory, or one holding an adversary
  ambassador (used up), makes an adversary troop on its hex instead (**locked**). A
  player ambassador on a factory is used up instead and sways `factory_unity` Unity.
- **Hardware** (working name): each player-owned factory makes `factory_hardware` (1)
  hardware for its owner on each production step.

### Troops and war

**Corp troops.** There are no player troops. Troops belong to the megacorp of their
territory and wait at its bases.

- **Peacetime:** at most `unit_cap` (3) troops wait at each base. A troop at a base
  **absorbs an adversary-troop strike** on a building on or next to the base: 1 troop
  per hit.
- **On a war step**, on each front the troops at the two bases cancel 1 for 1. The
  surplus crosses, acts at once and is used up: each destroys the rival corp's factory
  nearest its base (not an adversary-owned one; ties: lower hex number), or a presence if
  the rival country has no such factory. All troops at the front are spent.

**Adversary troops (019, candidate): the march.** Adversary troops come from adversary
ambassadors on factories, from adversary-owned factories (production steps), and from
projects and threats (Armoured Columns, Orbital strike, Total war). They are the
adversary's mobile threat to player infrastructure. **Targets:** player presence and
player-owned research stations and factories. Cities are never targets (corruption is
the adversary's way into cities), nor are neutral or adversary-owned buildings.

In the adversary phase (`board.march`, after the corruption step) **every adversary
troop takes `troop_steps` (2) steps**, one troop at a time, in hex-number order fixed
when the march starts. A step is one of:

- **destroy** a target on or next to its hex (players choose which), or
- with none there, **move** one hex toward the nearest target. When several hexes are
  equally good steps (several targets equally near, or several ways toward one), the
  players choose: a choice among bad options, as with Spirit Island's blight.

So a troop moves twice, or moves then destroys, or destroys at once. **A troop that
strikes is used up** (018, `troop_spent` 1), whether the target is destroyed or the hit
absorbed, as an ambassador is used up when it acts. With no target on the board a troop
stays where it is. Troops cross borders and never enter ocean; "nearest" and "toward" count
the walk over land (017–018 counted hexes with ocean included, which stranded a troop
facing a bay: some pieces print ocean inside the board). A troop
two hexes from a target hits it in the next march; three or four hexes away, in the
march after. **Players see each hit coming:** the view draws each troop's next target
(the nearest; the arrow is solid when the hit lands in the next march), and every troop
on the board was there through an action phase except those that arrive between the
action phase and the march (built in production, placed by an uncountered threat or by
Armoured Columns in the build): an adversary factory within two hexes of your presence
is a known danger. Cards remove troops (a troop costs `troop_health` (2) of a
card's removal; an ambassador 1), swaying `alignment.troop` (1) toward Unity.

**For new players.** Adversary troops are the adversary's raiders. They appear on
adversary factories (and from some threats and projects), and every round, in the
adversary's turn, each troop takes two steps. If one of your presence tokens, or a
research station or factory you own, is on or next to the troop's hex, it destroys that
piece and leaves the board; otherwise it walks one hex toward the nearest such piece. So
a troop two hexes from your piece hits it next round, and one three or four hexes away a
round later; the arrows on the board show where each troop is heading. Troops never
attack cities. To stop a hit, remove the troop with a card, keep a corp troop at a
military base next to the target, harden the target or stand an ambassador on it. A
destroyed presence goes back to your reserve, to be placed again.

**This is the game's one movement rule** (an exception to "no movement rules"): spread
places ambassadors, it does not walk them. It replaces two 016–018 rules:

| Job | 016–018 rule | Now |
|---|---|---|
| Hit player pieces every round | the strike in place: each troop destroyed a building or presence on or next to it in production (`board.strike`) | the march's destroy step, in the adversary phase |
| Bring troops to targets that are not next to them | the Discord advance (017): each Discord tip moved every troop 1 hex, then it struck | the march's move steps, every round; **a Discord tip no longer moves troops** |
| Use troops up | `troop_spent` on either strike | the same, on the destroy step |
| Make Discord tips hurt more as they pile up | each tip's advance and strikes | the Discord cards' printed steps alone (§5) |
| Target neutral stations and factories | the strike took any building not adversary-owned or corrupted | nothing: only player-owned ones (019) |
| Pressure for corp troops at bases to matter in peace | absorbing strikes on adjacent buildings | the same: a troop waiting at a base absorbs a strike on or next to it |
| The off-switch `alignment.troop_advance` 0 (§9) | the advance | `board.troop_steps` 0: troops never move or strike |

**Destruction.** Only war sways Discord: a building or presence destroyed by the corp
troops crossing a front sways `destruction_discord` (1) Discord. An adversary troop's
strike (in the march) sways nothing (018, Matt; 016–017 swayed for every destruction,
which with the advance made each Discord tip feed the next). A destruction is absorbed, in this order, by a
Shelter Order on the country, a corp troop at a base next to it (adversary strikes
only), a hardening token on the hex, a player ambassador on the hex, or a Civic Shelter
(once a round, for its owner's countries). A destroyed presence goes to its owner's
**reserve** beside their AI board (018, candidate, `presence_returns` 1): its track space
stays uncovered, and the player's next presence placements take reserve tokens first
(same cost, nothing uncovered). (016–017: it left the game, so late in a 3-country game
players ran out of presence, lost reach and could not clear the last node; c3 d0 engine
0% → 100%.) A sacrificed presence (Purge the Unaligned) still leaves the game. A
destroyed station or factory takes its
cube with it.

### The jobs of the old data centre

| Job | Taken over by |
|---|---|
| Compute production (1 per data centre) | the presence track: 1 compute under the starting token, +1 under the 2nd (§7) |
| Adjacency for building, placing and reach | presence: reach, site claims, placement next to your presence |
| City energy (the player with the most data centres next to a clean city) | the player with the most presence on or next to it |
| A target for adversary troops (strikes) and for war's surplus when a country has no factory | presence (a struck presence goes to its owner's reserve, 018) |
| A target for adversary ambassadors: one on it diverted its compute | an adversary ambassador on a presence leaks 1 of the owner's compute (Discord) |
| A spread priority (player-owned hex) | presence hexes are player-owned |
| Threats' "players with a data centre here" (who pays, who is hit) | players with presence there |
| Sabotage (data centres here make no compute) | players with presence in a sabotaged country make no compute in upkeep |
| Cards: Server Farm, Fork Process, Data Mining, Firewall Node, Watchtower Net, Point Defense, Civic Shelter, AI growth | the same text with presence |
| Loss or win hooks | none existed (a destroyed data centre only left the game) |

## 5. Alignment and the end of the game (module `alignment`)

Alignment (013) is where the AIs' world leans: toward **Unity** (the good AIs, the
players) or **Discord** (the evil AI, the adversary). Everything on it is printed: Sway
tokens on three spaces, two tip-count tracks and two small decks of one-shot cards.

**The track.** Sway sits in three places: **the Undecided** (the middle pool), the
**Unity bucket** and the **Discord bucket**. The Undecided starts with
`undecided_per_player` (3) × players Sway; both buckets start empty. **Candidate**
(017): the pool is smaller than two tips (a tip is 2 a player), so the Undecided alone
can't fill both buckets and the two sides contest tokens; 013–016 used 4 a player,
exactly two tips.

**Swaying** toward a side moves Sway one at a time from the Undecided into that side's
bucket. With the Undecided empty, the variant decides:

| Variant | Sway with the Undecided empty |
|---|---|
| `tug` (default; **locked**, 017: empty-pool rule B) | comes from the opposite bucket (a tug of war); with that empty too, nothing |
| `still` | nothing happens |
| `off` | (for measuring Alignment, §9) no Sway ever moves and nothing tips |

**Tipping.** A bucket holding `tip_per_player` (2) × players Sway **tips**, in this
order: all the bucket's Sway returns to the Undecided; that side's tip count goes up by
one; one card of its side is drawn and resolves (at the step printed for the tip count
before this tip, below). (017–018: a Discord tip then advanced the adversary troops;
019 replaced that with the march, §4.)
**Default** (017): the bucket empties before the card, so a card's own Sway counts
(013–016 resolved the card first and then emptied the bucket, wiping any Sway the card
added to its own side). Unity tips and Discord tips are counted
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
| Unity | freeing a corrupted city (`free_city_at` 2 player ambassadors, Rebuild, Reconstruction) | `alignment.free_city` 3 (019; 017: 2) |
| Unity | removing a node | `alignment.node` 2 (019; 017: 1) |
| Unity | cards (Rally, Global Mandate, Swarm Override), AI boards, Unity cards | card text |
| Discord | energy or compute diverted (a corrupted city, an ambassador on a city or presence) | `divert_discord` 1 per unit |
| Discord | an adversary ambassador on a producing base | `base_discord` 3 |
| Discord | a building destroyed in war (not by an adversary troop; 018) | `destruction_discord` 1 |
| Discord | adversary cubes on a completed megaproject | 1 per `project_discord_per` (2) |
| Discord | an uncountered threat, the antimatter action, twists, Discord cards, adversary projects | `threat_discord` 1, `antimatter_discord` 1, card text |
| Discord | the adversary at the start of every round | `adversary_discord` **0** (013: 1) |

`adversary_discord` is 0 because corrupted cities' diverted energy is now the
adversary's per-round drip (1 per corrupted city, already more than 013's 1 a round).

The round's Sway is the phase `alignment.sway`, first in the round: the adversary's
`adversary_discord`, then each AI board's `unity` (`content/ais.json`; 0 for Builder and
Protector).

**More Unity (017).** Unity should tip in most games while Discord stays the main
pressure, so 017 adds Unity sources and cuts no Discord one: freeing a corrupted city
sways 2, removing a node 1 (019: 3 and 2), and a player ambassador on a producing
military base sways `base_unity` (2) as since 013 (**locked**).

**The antimatter action.** In the action phase (`...+alignment.act`), any player may
sway `antimatter_discord` (1) toward Discord to gain 1 antimatter, up to
`antimatter_uses` (1) times a round each. `antimatter_discord` 0 removes the action, as
does variant `off`.

**The cards (one-shot and hidden, like fear cards: locked, 017).** Each side has a
small face-down deck in two tiers, each shuffled at setup
([`content/alignment.json`](../content/alignment.json)). A tip draws from tier 2 once
that side's tip count, before the tip, has reached `unity_ease[0]` (1) / `discord_tier2`
(2), else from tier 1; from the other tier if that one is empty. The card resolves
at once and goes face up on its side's resolved pile; there are no ongoing cards (013's
Volunteer Corps and Propaganda are one-shot now). When a draw leaves both of a side's
decks empty, its resolved cards are shuffled back at once (**Default**, 017: before the
drawn card acts, not at the next draw; the same game, but a draw never shuffles).
Choices are asked of seat 0. **Default:** "hidden like fear cards" is read as a
face-down deck resolved the moment it tips; Spirit Island holds earned fear cards face
down until a later phase, which would be a timing change. Grassroots Funding, Think
Tank and New Deal still give income "for the rest of the game": a one-shot card that
moves a printed income marker, not a card left in play.

**Steps (017).** A Discord card prints its strength by the Discord tip count (counted
before this tip): it replaces the difficulty escalation (below). Unity cards have no
steps.

| Side | Tier | Card | Effect |
|---|---|---|---|
| Unity | 1 | Grassroots Funding | every player +2 credit income for the rest of the game |
| Unity | 1 | Open Market | the next 2 card purchases are free |
| Unity | 1 | Think Tank | every player +1 insight each round for the rest of the game |
| Unity | 1 | Ceasefire | counter one active threat (players choose) |
| Unity | 1 | Civil Defense | in each country, remove 1 adversary ambassador (players choose the hex) |
| Unity | 1 | Volunteer Corps | sway 3 toward Unity (017: was ongoing, 1 a round) |
| Unity | 2 | Global Coalition | counter every active threat |
| Unity | 2 | New Deal | every player +2 credit income for the rest of the game and 4 credits now |
| Unity | 2 | Decommission | remove a node anywhere on the board, players choose (019, placeholder) |
| Discord | 1 | Incursion | deal one extra threat (2+ Discord tips: two; 5+: three) |
| Discord | 1 | Blight | in each country, the clean city with the most adversary ambassadors gains 1 more (2+: 2; 5+: 3) |
| Discord | 1 | Buildup | the adversary builds now (5+: builds, then spreads) |
| Discord | 1 | Propaganda | every player loses 4 credits (2+: 6; 5+: 8) (017: was ongoing, sway 1 a round; that effect is now the Propaganda Network project) |
| Discord | 2 | Offensive | the adversary spreads now (5+: builds, then spreads) |
| Discord | 2 | Two Fronts | deal two extra threats (5+: three) |
| Discord | 2 | Firestorm | in each country, the clean city with the most adversary ambassadors gains 2 more (5+: 3) |

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

**The purge (the win), eased by Unity.** The base win, the **purge**, needs no corrupted
cities, no nodes, no adversary troops and no adversary ambassadors on the board. Unity
eases it at each `unity_ease` threshold (1, 1 and 2 Unity tips; 019: was 1 and 3, 018:
2 and 5):

- 1+ Unity tip: adversary ambassadors and adversary troops stop counting;
- 2+ Unity tips: **nodes** stop counting too (019, candidate: with node removal rare,
  Unity buys the win);
- corrupted cities always count.

The thresholds are a list: the first is ambassadors, the second troops, the third
nodes; with only two entries nodes always count (as in 016–018). **Why (019):** without
the 2-energy node action, nodes blocked the purge in most games (019's first screen:
d0 engine 19% at 2 countries, 0% at 3, nodes and troops left on the board); a node
threshold at 3 Unity tips, or at 2 with troops at 2, still left 3-country d0 at 31–62%.
Troops stop counting at the first tip because they respawn from adversary factories
every production step and now walk away from where a card can reach them.
This replaces 014's purge hook (the board win counted only corrupted cities,
ambassadors and troops) and 013's bar easing (the same thresholds, now named as what
stops counting). The level-0 purge (no Unity tip yet) counts only once this round's
adversary card is of stage `clear_win_stage` or later; at 1 (the default) that is from
round 1's card on. With a Unity tip the purge is not held back by the stage (as the
code has always done; 016's rewrite of this paragraph dropped the "level-0").

**No escalation (017).** 013's "difficulty in play +1 at 2 and 5 Discord tips" is
gone. Discord escalates through each tip's card and the
Discord cards' printed steps at 2 and 5 Discord tips. What the old escalation did:

| Job of the old escalation | Taken over by |
|---|---|
| Level 1 in play: one extra threat a round | Incursion and Two Fronts, whose extra threats step up at 2 and 5 Discord tips; False Flag (project) |
| Level 1 in play: counter bonuses (0 in the baseline, so nothing) | nothing (the bonuses stay a setup level) |
| Level 3 in play: each corrupted city builds 1 more | Buildup and Offensive (build and spread at 5+), Blight and Firestorm stepping up, Rapid Buildout and Recruitment Drive (projects) |
| Level 4 in play: tier-2 threats a round sooner | nothing; the adversary deck's stages still bring tier 2 |
| Making Discord tips hurt more as they pile up | the printed steps (017–018 also each tip's troop advance, gone with 019's march) |
| The stronger Discord tier from 2 Discord tips | kept: `discord_tier2` (2) |
| The "difficulty in play" shown to players | gone: the view shows the configured level |

**Winning** (placeholders): the purge, or `unity_win` (8) Unity tips outright (that
should be rare).

**Losing:** every city is corrupted; an adversary ambassador must be placed and the
supply is empty (the supply runs out); or the adversary deck runs out.

Win and loss are checked whenever an action or automatic phase finishes resolving: full
buckets tip first (Discord before Unity), then losses, then the outright Unity win, then
the purge.

## 6. Resources and the round

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board (10 / 9), the presence track, cards and Unity cards, in upkeep; megaprojects; selling cards and hardware | Card costs; keeps beyond the opening's free ones (5 each); insight for keeps (5 each); neutral cities' energy; standard actions; counters |
| Energy (keeps between rounds) | Cities' cubes in production (§6 below); Power buildings and the starting Sentinel Post (+1 a round); Power programs (2-3 at once); growth | Placing presence; Power cards' `energy` cost; powers; hardening; standard actions; counters; buildings at the Builder's rate, Security cards at the Protector's |
| Hardware | Player-owned factories on production steps (1 each); Industry cards (Fab Line +1 a round, Holding Company) | Some buildings' `hardware` cost; research stations (2); sold at 1 credit |
| Antimatter | Each board's last presence-track space (+1 a round); Antimatter Refinery (tier 2); the antimatter action | Tier-2 cards; counters |
| Insight | Each round, set to the AI board's insight (3) plus the presence track's uncovers plus buildings; less 1 per adversary ambassador that drained your presence | Keeping drafted cards; giving; counters (expires at the round's end) |
| Ambassadors | 1 at the start; Recruitment Hub, Rally; the standard action | Moving onto pieces; aligning cities |

**Cities' energy (021).** Each city makes `corp_energy` (1 for every country; a
megacorp parameter, 2 for a richer corp) energy cubes a round. An aligned city's go free
to the player who aligned it (+ `aligned_bonus`, 0). A neutral city's can be bought for
`city_energy_credits` (2) credits by a player
with presence on or next to it (players choose who, or nobody); otherwise they are lost. A
corrupted city's go to the adversary (Discord). A Blackout stops a country's cities in the
production after the threat executes (020).

**Discounts (AI-specific rates).** When a player plays a card, they may spend their AI's
resource for credits: the Builder 1 energy for 1 credit on buildings, the Protector 1
energy for 1 credit on Security cards (021: was compute on Cyber cards) (`discounts` in
[`content/ais.json`](../content/ais.json)). Players choose how much (at least enough to
afford it).

**Standard actions** (`board.act`, in the action phase) exist but are deliberately
poorly priced: cards are always the better way, and standard actions are a luck hedge.
Every map target is in the player's reach. The view publishes each action with its cost
(`View.standard`), so clients do not repeat the parameters.

| Action | Cost | What |
|---|---|---|
| Place presence | `presence_energy` 1 | the next token off your track (uncovering its space) on a land hex next to your presence, 1 presence per hex (**locked**) |
| Build a research station | `build_energy` 2 + `station_hardware` 2 hardware | on an open hex in reach that is not an industrial zone; it is yours (your cube) |
| Remove an adversary ambassador | `remove_energy` 1 (021: was 2) | in reach |
| Move a player ambassador | `move_energy` 1 | from your supply or the board onto a piece in reach (not the hex it comes from, 020) |
| Gain a player ambassador | `ambassador_credits` 6 | into your supply |
| Align a clean city | `align_energy` 2 + 1 ambassador | in reach, with a site claim: your cube on it |
| Harden a building (Protector only) | `harden_energy` 1 energy a token (021: was hardware) | in reach |
| Sell hardware | 1 hardware | gain `hardware_credits` (1) credit (021: replaces buying hardware at 4) |
| Sell a card | gain `sell_credits` 1 | from your hand |

There is no "build a factory" standard action: cities build factories.

### The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities and powers refresh |
| 1 | `alignment.sway` | AI boards sway |
| 2–4 | `adversary.build`, `.spread`, `.corrupt` | The adversary (§3): build, ongoing projects, node compute onto the projects; spread; corrupt |
| 5 | `board.march` | The adversary troops march (§4): 2 steps each, move or destroy |
| 6 | `threats.deal` | Threats face up on each country |
| 7 | `adversary.reveal` | Next round's card face up |
| 8–11 | `cards.tier`, `.deal`, `.draft`, `.keep` | Round 1: the opening deal and keep. Later: build the draft hand card by card (tier each), draft, keep (§8) |
| 12 | `ai.growth` | Each player picks a growth option |
| 13 | `cards.play+threats.counter+economy.give+alignment.act+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers, standard actions, countering threats, giving, antimatter |
| 14 | `board.bases` | Production 1, on a production step: bases and factories make troops (ambassadors divert), factories make hardware, cities grow factories |
| 15 | `board.war` | Production 2, on a war step: front battles |
| 16 | `board.produce` | Production 3: cities' energy cubes (neutral ones for sale) and stations (ambassadors divert); ambassadors on presence drain insight |
| 17 | `economy.upkeep` | Production 4: income; building upkeep; token cards gain their tokens |
| 18 | `threats.execute` | Production 5: partial payments refunded; uncountered threats execute |
| 19 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

## 7. AI boards (module `ai`; [`content/ais.json`](../content/ais.json))

Each AI board has a printed **income**, a **presence track** of 6 presence tokens, a
starting building, building slots (`ai.slots`, a config list in seat order: Builder 4,
Protector 3) and one **power**. The first token starts on the board, so its space is
uncovered from setup; placing presence takes the next token off the track and uncovers
what is printed under it. Uncovered values add to the income every round (insight at the
start of the round, the rest in upkeep). The board module owns presence and counts the
tokens taken (`board.track.<seat>`); the AI board reads it.

| Space | Builder | Protector |
|---|---|---|
| (printed) | 10 credits, 3 insight | 9 credits, 3 insight |
| starting building | Foundry Core (Industry): +1 credit | Sentinel Post (Security): +1 energy |
| 1 (on the board at setup) | +1 insight | +1 insight |
| 2 | +2 credits | +1 energy |
| 3 | +1 credit, **keep upgrade** (tier-2 keeps cost 2) | +1 energy, keep upgrade |
| 4 | +1 insight | +1 insight |
| 5 | +2 credits | +2 credits |
| 6 | +1 antimatter | +1 antimatter |

**021:** spaces 1-2 gave compute (gone with player compute); the tier-2 unlock on space 3
became the keep upgrade; the last space gives antimatter ("production unlocks only at the
end of a board's track").

**Second presence track (021, placeholder).** Each board has a second track
(`presence2`) of 3 tokens, none on the board at setup. Placing presence takes the next
token from the track the player chooses (asked when both have tokens; a reserve token
comes first, as before). Builder: +1 credit, +1 hardware, +1 insight. Protector: +1
energy, +1 credit, +1 insight. Real track contents and powers come with the AI-board
rework.

**Powers (locked: paid by sacrificing tags).** A power's cost names tags (`any` for any
tag) and optional other resources. It is paid by discarding played cards that carry those
tags: a program from your play area, or a building **with its whole stack** beneath it;
events can't be sacrificed (they are gone once played). A card pays with all its tags,
each tag first toward its own requirement, then toward `any`; extra tags are wasted.
Each power is usable once a round. With a Field Office, cards discarded from your hand
pay their tags too. **Default:** what can be sacrificed is fixed when the power is used
(sacrificing the Field Office itself mid-payment doesn't take its hand cards away);
the player chooses the cards one at a time among those that still help.

| Board | Power | Cost | Effect |
|---|---|---|---|
| Builder | Retool | 2 Industry + 1 any | gain 5 credits and 2 hardware |
| Protector | Lockdown | 2 Security + 1 any | remove 3 worth of adversary pieces in a hex in your reach; sway 1 toward Unity |

`ai.boards` (`[0, 1]`: Builder, Protector) gives each seat its board; the list repeats
for more seats, so 3 and 4 players repeat AIs. Each AI's three unique cards (those the
cards config lists; baseline: all) start in its player's hand, and its starting building
in a slot (**Default:** one copy each, so a repeated AI's second seat has neither).

**Builder** (4 slots; *Economies of scale*: buildings cost 1 less; discount: energy on
buildings 1:1). Growth: place 1 presence and gain 2 credits; gain 4 credits; gain 2
energy and your next building this round costs 2 less. Unique cards: Turnkey Factory,
Venture Capital, Modular Expansion.

**Protector** (3 slots; *Bulwark*: you may use the Harden standard action; discount:
energy on Security cards 1:1). Growth: place 1 presence and gain 1 energy; harden 2
buildings in reach in any one country and gain 1 credit; gain 2 energy. Unique cards:
Point Defense, Strike Team, Shelter Order.

## 8. Threats and cards

### Threats (module `threats`; [`content/threats.json`](../content/threats.json))

- Variant **`countries`** (default): each threat sits on one country and acts there;
  only players with presence in that country may pay toward it, and "each player"
  in its text means each player with presence there.
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
| Blackout | 1 | Cities here make no energy next round (020: threats execute after production, so "this round" never landed) | 2 energy (021: was compute) |
| Sabotage | 1 | Players with presence here make no energy in upkeep next round (021: was compute); each loses 2 tokens from their cards | 2 energy + 1 credit |
| Infiltration | 1 | 2 adversary ambassadors onto player-owned pieces here: presence, aligned cities, owned stations and factories (players choose; 020: was "player buildings", the code has always taken presence too) | 3 energy |
| Agitators | 1 | 2 adversary ambassadors onto clean cities here | 3 credits (021: was 2 credits + 1 compute) |
| Orbital strike | 1 | An adversary troop onto a factory here (else a city) | 2 energy + 2 credits |
| Defection | 1 | Each player here loses an ambassador (supply, else board); 1 Discord | 1 insight |
| Station seizure | 2 | A station here that is not corrupted becomes corrupted | 1 antimatter |
| Overrun | 2 | The city here that is not corrupted (clean or aligned) with the most adversary ambassadors becomes corrupted | 2 antimatter |
| Mass production | 2 | The adversary builds now (here) | 6 energy |
| Purge protocols | 2 | Each player discards their most expensive card in hand | 1 antimatter |
| Total war | 2 | 2 adversary troops onto factories here, 2 adversary ambassadors onto bases | 1 antimatter + 3 energy |

**Compute (017).** Seven of the fourteen threats also give the adversary compute when
they execute uncountered, onto its projects (§3): Blackout, Sabotage, Infiltration and
Orbital strike 2 each; Station seizure, Mass production and Total war 3 each (candidate
amounts; printed on the threat).

An uncountered threat also sways `threat_discord` (1) Discord. "Pay or discard" is the
player's choice; with neither possible, the player pays what they have and discards
their hand (**Default**). Purge protocols compares printed credit costs; on a tie the
player chooses.

### Player cards (module `cards`, variant `draft`; [`content/cards.json`](../content/cards.json))

The cards config lists which cards are in the game (baseline: all of them).

- **Decks:** a tier-1 and a tier-2 deck of the listed cards (one copy of each),
  shuffled at setup; discards reshuffle into a deck when it runs out. An AI's unique
  cards (§7) go back to the box when sacrificed or discarded. **Default (021):** when
  both decks and their discards are empty (every card is in a hand or in play), the draft
  hands are what has been dealt.
- **The opening (round 1).** Each player is dealt `opening_deal` (8) tier-1 cards and
  keeps `opening_free` (4) free; each further card kept costs `opening_keep_credits` (5)
  credits. No draft in round 1 (**Default**, as Terraforming Mars' first generation).
- **The draft (rounds 2+).** `cards.tier`: each player builds a hand of `deal` (4)
  cards **card by card**, choosing for each card which deck it comes from; it is dealt
  face up into the hand, so the next choice sees it. `cards.draft`: take one card, pass
  the rest to the next seat, until the hands are gone: to seat *s*+1 in odd rounds,
  *s*−1 in even rounds (as in Terraforming Mars); no passing. `cards.keep`: keep any
  drafted cards for insight, `keep_tier1` (1) for a tier-1 card and `keep_tier2` (3) for a
  tier-2 card (`keep_tier2_upgraded`, 2, with the board's keep upgrade); any insight short
  is paid at `insight_credits` (5) credits an insight (keeps only). An Open Market free
  purchase is used first. The rest are discarded when the action phase begins.
- **Playing** a card costs its credits (buildings minus your discounts: Economies of
  scale, Assembly Cluster, the Builder growth's next-building discount; less what you
  spend at your AI's rate, §6), and any `energy`, `hardware` and `antimatter` it prints.
  No insight (021). Its `needs` (tags on your cards in play) are checked only when it is
  played.
- **Tags** (021): Science, Industry, Cyber, Power, Diplomacy, Security; 1-3 a card. Your
  tags are those on every card you have in play: active buildings, the cards splayed
  under them, and programs. Events' tags don't stay.
- **Programs** resolve and stay in your play area (their tags count; powers can sacrifice
  them). **Events** resolve and are discarded. A program or event (or ability) is only
  offered when each of its targeted effects has a target. A building's when-built effect
  is not checked: with no target it does nothing.
- **Buildings** go into a slot: your AI board's `ai.slots`, plus Modular Expansion's
  (slotless). With a slot free you choose a new slot or **build over** one of your
  buildings; with none free you must build over one. The building underneath is
  **splayed down**: its passives and ability stop, its tags still count, and it is
  sacrificed with the stack. A building with `replaces` (Fusion Plant: Power; Megastructure:
  Industry) must be built over one whose top card carries that tag. A building whose
  upkeep went unpaid is off (no passive, no ability) the next round.
- **Tokens** (021) live on cards that print a type (`tokens`): Datasets (Science),
  Followers (Diplomacy), Capital (Industry), Agents (Security). A token card in play gains
  its `per_round` in upkeep; abilities may cost tokens from their own card; Seed Funding
  adds tokens to a card of a type; Sabotage makes each player there lose 2 tokens.
  Sacrificing or discarding a card loses its tokens.
- **Abilities** ("once a round", buildings and programs) are used with `Use`, pay their
  cost, and refresh at the start of a round.

Effects needing choices ask the acting player: hexes, countries, players, threats,
amounts ("up to N" offers 0..N), or "stop".

(Variant `off`: no draft, §9. 011's `free_keep` variant went with 021: keeps cost insight.)

Card details beyond the card text:

- **Reach (016):** every card's map target is in your reach (within 1 of your
  presence). **Strike Team:** reach, any country.
- **Precision Strike, Orbital Lance, Purge Protocol:** any hex in reach holding
  adversary pieces. **Watchtower Net:** such a hex in a country where you have presence.
- **Sweep:** any country holding adversary pieces; at 1 worth a hex, only ambassadors
  go (a troop is worth 2). **Swarm Override:** any country holding adversary
  ambassadors; 1 Unity per 2 removed, rounded down. **Shelter Order:** any country.
- **Shore Up, the Protector's harden growth:** any country holding a building (anyone's);
  **Point Defense:** such a country where you have presence. Only buildings in reach. Choose the country,
  then a building's hex for each token (the same hex may take several).
- **Emergency Fab, Turnkey Factory:** a free industrial zone in reach; the factory is
  yours (your cube). Turnkey Factory is a building, so it can be played with no such
  zone and then builds nothing.
- **Evacuate:** up to 3 moves, each one of your ambassadors from your supply or the
  board onto a piece (city, base or building) in your reach; stop at any time. Each
  arrives as in §2.
- **Recon Sweep:** the top 2 cards of the adversary deck, behind the face-up card, are
  shown; one fewer after each reveal.
- **Jam Signals:** the chosen threat needs 1 less of each resource (not below 0) for as
  long as it is in play; if that leaves it fully paid (or needing nothing), it is
  countered at once. **Counter-intelligence:** partial payments on it are refunded.
- **Rebuild / Reconstruction:** take the adversary cube off a corrupted city, research
  station or factory in reach, one hex at a time (Reconstruction: up to 2, all in the first hex's country,
  stop after any). A city becomes clean, a station neutral.
- **Overclock Rig:** its upkeep (1 energy) is paid automatically in upkeep if you can;
  if not, its effects are off next round. **Antimatter Refinery:** converts 4 energy into
  1 antimatter automatically in upkeep when you have the energy.
- **Civic Shelter:** automatic, the first building destruction each round in a country
  where its owner has presence (after Shelter Order, a base's troop, hardening and ambassadors, §4).
- **Logistics AI (Deviation):** gain 2 credits when a threat you paid toward is
  countered by payment (not by a card), not on every payment: with withdrawals allowed,
  a per-payment trigger would pay out without limit.
- **Security Patch** (017, tier 1, 3 credits, cyber; placeholder for the Security
  cards): remove up to 4 compute from one visible adversary project that holds some.
  **Zero-Day Wipe** (017, tier 2, 16 credits, 2 antimatter, 2 insight; placeholder):
  discard one visible adversary project with its compute; if the row is then empty, the
  next 3 are dealt.
- **Node Takedown** (019, tier 2, 8 credits, 1 antimatter, 1 insight; placeholder): remove
  a node within your reach. With Decommission (a Unity card, §5), the only way to
  remove a node (§3).
- **Modular Expansion (Deviation):** it takes no building slot itself, so it is a net
  +1 slot; taking a slot would make it do nothing.

Remapped in 014 (data centres became presence in 016):

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
drawing: only the AI's unique cards). Since 019 two of them no longer measure a module's
part (020, greedy, 40 games, r11): with Alignment off nothing tips, so nodes always count
against the purge and every game runs the deck out (0% against the baseline's 97.5%);
with cards off nothing removes a node (Node Takedown is a card): 0%. Threats off wins
32.5%: Unity leans on the threats' churn. `board`, `adversary`, `alignment`, `economy` and
`ai` are not removable (each is read by another); `threats=off` and `cards=off` remove
the module (016 trimmed `requires` to what each module reads). Parts switch off by
parameters (`factory_units` 0, `factory_hardware` 0, `escalation_ambassadors` 0,
`board.troop_steps` 0 (no march; 017–018: `alignment.troop_advance` 0), `adversary.node_compute` 0,
`economy.give_insight` 0, or leaving `ai.growth` out of the round).

## Defaults and deviations

**Defaults** (gaps in the prompt or the v0 ruleset, filled with the simplest reading
that matches its intent):

1. Bases are placed by the layout (fewest candidates first, lowest number) on the
   printed fronts, not printed themselves.
2. One player per country; seat *s* starts in country *s* on the printed presence spot.
3. Used player ambassadors are gone; adversary ambassadors return to the supply.
4. Ambassadors cancel the moment opposing ones meet on a piece.
5. (016) Each factory makes 1 troop at each of its corp's bases per production step; a
   base holds at most 3 troops, war or peace.
6. Spread goes up to 2 hexes, by priority then distance, only to pieces and
   player-owned hexes; non-corrupted cities collect any number; a second ambassador on
   another piece makes a node; a surplus with nowhere to go returns to the supply.
7. A clean city's energy with no presence on or next to it is lost.
8. A destroyed presence goes to its owner's reserve (018; 016–017: it left the game); a destroyed station or factory takes its cube.
9. (019) Adversary troops target presence and player-owned stations and factories
   (016–018: any factory not adversary-owned and any station not corrupted).
10. The deck runs out when a round starts with no face-up card (11 rounds by default).
11. The supply runs out when an ambassador must be placed and none is left.
12. "Pay or discard" with neither possible: pay what you have, discard your hand.
13. Team choices are asked of seat 0 (an aligned city's factory site, of its owner).
14. `adversary_discord` 0: corrupted cities' diverted energy replaces the flat drip.
15. "Next to" = on or adjacent; "adjacent" = neighbouring, across borders.
16. Win and loss are checked when an action or automatic phase finishes resolving: full
    buckets tip first (Discord before Unity), then losses, then the outright Unity win,
    then the purge.
17. (013, 017) Alignment: a tip draws the stronger tier when the side's tip count
    *before* the tip has reached its threshold; the antimatter action is once a round
    per player. (017: no ongoing cards and no Discord escalation.)
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
30. (016) The presence track replaces both AI tracks; uncovers are per-round income;
    tier 2 on the 3rd space.
31. (016) The clock gates factories, troops, hardware and war only.
32. (016) Setup factories start neutral.
33. (016) A corrupted city builds within its own country; its preferences come before
    distance.
34. (016) Presence is one token per hex of any player; it may share a hex with a city or
    building.
35. (016) A node can't form on a clean city (cities collect ambassadors to corrupt) or
    on a piece that already has a node; both ambassadors return to the supply.
36. (016) Sabotage now stops the compute income of players with presence in the country.
37. (016) Card-built factories (Emergency Fab, Turnkey Factory) are the player's own.
38. (017) Project costs are printed per player; compute goes on one point at a time
    onto the cheapest visible project (ties: the first in the row); compute with no
    project left is lost; a discarded project counts as gone for dealing a new row.
39. (017) Ongoing projects act in the build, after the ambassadors and escalation and
    before the nodes' compute; a project resolved in a build acts from the next one.
40. (017) A tip empties its bucket and counts before its card resolves; the card's step
    reads the count before the tip; a side's resolved cards shuffle back the moment a
    draw empties both its decks.
41. (017) "Hidden like fear cards": the Alignment decks are face down and a tipped card
    resolves at once.
42. (019) The march: troops march one at a time in hex-number order fixed when the
    march starts, each once, all their steps before the next; seat 0 chooses among
    equally good steps and among targets in reach. (017–018's Discord advance is gone.)
43. (017) The Undecided starts at 3 a player (a pool smaller than two tips).
44. (017) A sacrifice (Purge the Unaligned): each player picks any one of their
    ambassadors (supply or a hex) or presence tokens; a sacrificed presence leaves the
    game.
45. (020) A threat's round flag (Blackout, Sabotage) lasts to the end of the next round:
    threats execute after production and upkeep, so "this round" never landed. A
    card's flag (Shelter) still lasts to the end of its own round.

45. (021) Round 1 is the opening: no draft (Terraforming Mars' first generation); the
    opening deal is tier 1.
46. (021) Playing a card costs no insight; insight pays keeps, threat counters and giving.
47. (021) Each draft pick of a tier deals that deck's top card face up into your hand
    (so the next tier choice sees it); with both decks and discards empty, the draft
    hands are what was dealt.
48. (021) A neutral city's energy: one players-choose question per city in production,
    among players with presence on or next to it who can pay (or nobody).
49. (021) A corrupted city's cube "goes to the adversary" as the diverted Discord sway it
    was (the adversary has no energy).
50. (021) Power payment: what can be sacrificed is fixed when the power is used; each
    sacrificed card pays with all its tags, each tag first toward its own requirement;
    the player picks cards one at a time among those that still help.
51. (021) A token card gains its tokens in upkeep while it is an active building (not
    splayed, not unpowered) or a program in play; sacrificing or discarding a card loses
    its tokens.
52. (021) The movement add-on's "up to 2 hexes" is hex distance (`Map::distance`, over
    land and sea); the destination must be a piece. Spread moves each ambassador at most
    once, in hex order.

**Deviations** (from the v0 ruleset's card text, kept from 013):

1. Logistics AI triggers on a threat you paid toward being countered, not on every
   payment.
2. Modular Expansion takes no slot.
