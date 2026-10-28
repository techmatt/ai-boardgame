# Rules as implemented

What the code implements: ruleset **v0 as tuned by the 007 tuning run, with 011's
rule changes and 013's Alignment** (rules revision 5), Militarist against Builder and
Protector, 1–4 countries. 007 changed only numbers. 011 changed three rules: the
clear-board win waits for stage II ([§3](#3-alignment-and-the-end-of-the-game-module-alignment)), and
difficulty levels 1 and 3 have new effects ([§2](#difficulty-difficulty-04-each-level-includes-the-ones-below)).
It also added variants that are not the default: `free_keep` and the `uplinks` spread
(its two opposition rewards went with opposition). **013 replaced human opposition with
Alignment** ([§3](#3-alignment-and-the-end-of-the-game-module-alignment)): the Sway
track, its Unity and Discord buckets that tip into cards, and a +1 Discord antimatter
action. Opposition, its levels and its reward variants are gone.

Each changed value gives the v0 value beside it, and
[`configs/v0_untuned.json`](../configs/v0_untuned.json) keeps the v0 numbers and rules. **Keep this current with every rules change.** Design intent lives in
[principles.md](principles.md); how the engine is put together lives in
[architecture.md](architecture.md).

Numbers in `code` are config tunables (`<module>.<param>`); the values given are those
in [`configs/baseline.json`](../configs/baseline.json). Card, threat, adversary-card and
AI-board numbers are content data in [`content/`](../content/). Where the v0 ruleset left
a gap or was ambiguous, the choice made is marked **Default**; where the implementation
departs from the ruleset text, **Deviation**. Both are collected in the
[last section](#defaults-and-deviations).

## Terms

- **Adjacent**: a neighbouring hex on the board, across country borders too.
- **Next to X**: holding X, or adjacent to a hex holding X. (**Default**: the ruleset uses
  "next to" for explore sources, factories and threat conditions; reading it as "on or
  adjacent" keeps a factory on a populated hex working and lets an outpost's own hex
  explore.)
- **Players choose**: the decision is an `Action::Choose` in `legal_actions`, made by the
  one team agent. Team choices in the adversary phase and in Unity and Discord cards are asked
  of seat 0; a choice inside a player's own card or ability is asked of that player.
- **Hex number**: each piece's hexes are printed 1–16 (row by row along the piece); ties
  go to the lower number.

## 1. Board and setup

### Country pieces (module `board`, variant `pieces`)

A game has 1–4 countries and as many players (seats). Country *i* is always printed piece
*i* ([`content/map.json`](../content/map.json), `countries`), placed by the layout for that
country count. **Default:** the four pieces below; each has 4 hexes of each terrain, two
Frontier uplinks on the board's outer edge in every layout it appears in (never touching
another country), its factories next to population, and a slightly different
arrangement. Letters are Urban, Industrial, Network, Frontier, rows of hex numbers 1–4,
5–8, 9–12, 13–16.

| # | Name | Terrain rows | Uplinks | Outpost | Population | Factories | Data centres |
|---|---|---|---|---|---|---|---|
| 0 | Coastal Republic | NFFI UUIF NUIF UINN | 3, 12 | 3 | 5 6 7 10 13 14 | 4, 11 | 1, 15 |
| 1 | Northern Union | IFNF UIUF NUIF UNIN | 2, 8 | 8 | 5 6 7 10 13 15 | 1, 11 | 3, 14 |
| 2 | Highland Federation | NUIN FUIU FIUN FINF | 5, 9 | 9 | 2 6 7 8 10 11 | 3, 14 | 4, 12 |
| 3 | Delta States | UNFI FUIN FIUF NUIN | 3, 9 | 9 | 1 6 7 10 11 14 | 4, 15 | 2, 16 |

Country 2's piece sits differently in the 3- and 4-country layouts; only hexes 5 and 9 are
on the outer edge in both, so its uplinks are adjacent. The 5-country layout was removed:
v0 has four pieces.

### Starting pieces

Each country starts with `start_population` (1) population on each printed population
hex (all 4 Urban hexes and 2 Industrial ones: 6), `start_factories` (1) factory on each
printed factory hex, `start_data_centres` (1) data centre on each printed data centre
hex, `start_drones` (1) drone on each uplink, and `start_outposts` (0; v0 had 1) outposts on
the printed outpost uplink. The starting population total is recorded
(`board.start_population`) for the loss condition.

The **corruption pool** holds `pool_per_country` (10; v0 had 2) per country + `pool_base`
(2): 22 at 2 countries.

### Players (module `ai`, variant `boards`)

Seat *s* plays AI board `boards[s]` (`[0, 1]`: Builder, Protector; the list repeats for
more seats, so 3 and 4 players repeat AIs). Each starts with `start_credits` (12) credits
and no other resources, and `start_processors` (1) processor on the lowest-numbered data
centre of country *s* (its starting country, which nothing else refers to), plus its AI's
unique cards in hand. **Default:** every card is unique, so a repeated AI's second seat
has no unique cards.

### Opening

Before round 1: reveal the first adversary card into the explore slot, explore, and slide
(so it builds in round 1 and ravages in round 2). Round 1 then starts with threats.

## 2. The adversary (module `adversary`, variant `militarist`)

### Pieces

| Piece | Health | Ravage damage |
|---|---|---|
| Drone | `drone_health` 1 | `drone_damage` 1 |
| Outpost | `outpost_health` 2 | `outpost_damage` 2 |
| Foundry | `foundry_health` 3 | `foundry_damage` 3 |

### The deck (the clock)

The printed adversary deck ([`content/adversary.json`](../content/adversary.json)) has
4 stage-I cards (one terrain each), 4 stage-II cards (one terrain each, with Escalation)
and 6 stage-III cards (each pair of terrains). **Default:** a game deals `stage_cards`
(4, 4, 2; v0 had 3, 4, 3) of them at random per stage, so a game's deck is 10 unique cards: stage I on
top, then II, then III, each stage shuffled on its own. Players **lose** if a card must be
revealed and the deck is empty.

### The pipeline

Three face-up slots: explore → build → ravage. In the adversary phase of each round:

1. **Ravage** (`adversary.ravage`) with the ravage-slot card;
2. **Build** (`adversary.build`) with the build-slot card;
3. **Explore** (`adversary.explore`): reveal the top card into the explore slot and
   explore; a stage-II card then escalates;
4. **Slide** (`adversary.slide`): the ravage card is discarded, build moves to ravage,
   explore to build.

Each step applies in every country to every hex of the card's terrain(s); the hexes are
fixed when the step starts.

- **Explore:** each such hex that is an uplink, or is next to an outpost or foundry, gains
  `explore_drones` (1) drone. (Firewall Node stops it on hexes with its owner's
  processor.)
- **Build:** each such hex holding any adversary piece gains `build_pieces` (1) foundry
  if it has more outposts than foundries, otherwise `build_pieces` outposts.
- **Ravage:** each such hex holding adversary pieces takes damage equal to their total
  ravage damage, minus defense: each defense token prevents `defense_value` (1), or its
  owner's Bulwark value. The damage destroys players' pieces one at a time: population
  (`population_health` 2), factories and data centres (`building_health` 3).
  **Default:** players choose each next piece among those that the remaining damage can
  destroy (needs its full health); damage left that can destroy nothing is lost. A
  choice with one option is taken without asking.
  If the damage (after defense) is `corruption_at` (3; v0 had 2) or more, add
  `corruption_per_ravage` (1) corruption from the pool to the hex. If the hex already had
  corruption, it **cascades**: add `cascade` (1) to an adjacent hex of the players'
  choice (**Default:** any adjacent hex, any country; a cascade does not cascade again).

**Corruption** stays until removed. Factories and data centres on a corrupted hex produce
nothing. Players **lose** when the pool is empty (**Default:** "runs out" read as
"reaches 0").

### Militarist innate rules

- **Momentum:** when a ravage destroys population in a hex, add `momentum_drones` (1)
  drone to an adjacent hex in the same country, of the players' choice. **Default:** once
  per ravaged hex, however many population it lost.
- **Escalation:** when a stage-II card is revealed (after its explore), each country's
  uplink with the most drones gains `escalation_outposts` (1) outpost; ties go to the
  lower hex number.

### Difficulty (`difficulty`, 0–4; each level includes the ones below)

| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules | |
| 1 | **An extra threat** (011): each round `extra_threats` (1) more threat is dealt in all, one per country in turn: country 0 (the first piece) in round 1, country 1 in round 2, and so on (tier as usual). Also available, off by default: every threat's first counter requirement higher (`counter_bonus`, r3: 2), or every requirement higher (`counter_bonus_all`) | `extra_threats` 1; `counter_bonus` 0 (r3: 2, v0: 1), `counter_bonus_all` 0 |
| 2 | Stage-I cards removed from the deck: stage II arrives sooner | `stage1_removed` 1 |
| 3 | **Foundry reach** (011): when a hex holding foundries is ravaged, each adjacent hex (any country) first takes `foundry_reach` (3) damage per foundry, less its own defense, destroying players' pieces as a ravage does (players choose each). The reach adds no corruption and no Momentum | `foundry_reach` 3; `foundry_bonus` 0 (r3: 1) |
| 4 | Tier-2 threats start sooner; the corruption pool is smaller per country | `tier2_early` 1 (rounds), `pool_reduction` 4 (v0: 1) |

Every option stays a tunable, so the r3 levels are one override away:
`counter_bonus` 2, `counter_bonus_all` 0, `extra_threats` 0, `foundry_bonus` 1,
`foundry_reach` 0 (and `alignment.clear_win_stage` 1 for r3's clear-board win).

**Why the new levels 1 and 3 (011).** The 007 run found level 1 barely moved a strong
team (`counter_bonus` raises only the first requirement, and greedy counters few
threats) and level 3 did nothing: foundries grow on uplinks and outpost hexes, which
hold no population or buildings, so `foundry_bonus` destroyed nothing (even at 5). The
new effects land where the players are. Screened with greedy (64 seeds a point,
`experiments/screen-011-difficulty*.json`, results in
`experiments/results/screen-011-*`), as the change in win rate from the level below:

| Level | Effect | Win % change from the level below (greedy) |
|---|---|---|
| 1 | r3: first requirement +2 (`counter_bonus` 2) | −7.8 (64 seeds), 007: 0 to −3 (200) |
| 1 | every requirement +1 / +2 / +3 (`counter_bonus_all`) | −3.1 / −4.7, −3.1, 0.0 / −4.7 |
| 1 | first +2 and every +1 | −1.0 (96 seeds) |
| 1 | one extra threat **per country** each round | −32.8 (64 seeds): far too strong |
| 1 | **one extra threat a round in all, countries in turn** (chosen) | **−11.5 ±3.9** (96 seeds, p 0.007) |
| 3 | r3: `foundry_bonus` 1 | 0 (64 seeds), 007: 0 even at 5 |
| 3 | foundry reach 1 / 2 | 0 / +1.5 |
| 3 | **foundry reach 3** (chosen) | **−6.2 ±3.0** and **−9.4** (two seed sets of 64) |
| 3 | foundry reach 4 | −9.4 (64 seeds) |

Raising counter requirements barely bites: greedy counters few threats and simply
lets more of them execute. Reach 1 and 2 kill nothing that matters often (population
needs 2, buildings 3); reach 3 takes a building or a population with each foundry.

`tier2_early` moved no win rate in 007; level 4's bite is `pool_reduction`. The five-level
curve at the new defaults is not yet measured (`experiments/proposals-011.md`).

Level 4's "a round sooner" is implemented as: tier-2 threats are dealt once any of the
top `tier2_early` deck cards is stage II or later (the stage boundary is public: the
stages are stacked).

## 2a. Spread variants (module `adversary`, variants `clusters` and `assets`)

Two other ways for the adversary to spread, adapted from design notes for comparison
runs ([`configs/clusters.json`](../configs/clusters.json),
[`configs/assets.json`](../configs/assets.json); each differs from the baseline only in
the adversary module). Both keep the Militarist's deck (the clock, the same cards, the
same deal and difficulty 1, 2 and 4), the three-slot pipeline and its four phases
(`ravage`, `build`, `explore`, `slide`), the corruption pool and every loss condition.
Neither has the Militarist's innate rules (Momentum, Escalation). Values below are
those in the two configs: untuned first guesses. Every adaptation is marked
**Adaptation** and listed [at the end](#defaults-and-deviations).

### Clusters and infection (`clusters`)

From Matt's 2020 notes. The adversary's only piece is the **cluster** (`cluster_health`
2 damage to destroy; `cluster_unity` 1 Sway toward Unity each). **Neutral clusters** are
the data centres; **friendly clusters** are processors (any player's).

- **Setup (Adaptation):** each starting drone and outpost the board places becomes one
  cluster (so each uplink starts with 1, the printed outpost uplink with 2). The
  opening is the Militarist's: reveal, place, slide.
- **Place** (explore slot, on reveal): each hex of the card's terrain that is an uplink,
  or next to a hex with `source_clusters` (2) or more clusters, gains `explore_clusters`
  (1). Firewall Node stops it as it stops explore. (**Adaptation**: the notes say only
  "the adversary places clusters"; this is v0's explore with a stack of clusters as the
  source.)
- **Hegemonize** (build slot): each hex of the card's terrain that holds a cluster gains
  `hegemonize_clusters` (1). (**Adaptation**: the notes add a cluster to *every* hex of
  the terrain; `hegemonize_all` 1 plays that reading. Off by default: 8 clusters per
  card at 2 countries swamps the board.)
- **Assault** (ravage slot), on every hex of the card's terrain:
  1. **Infection**, hex by hex in board order: offense = clusters on adjacent hexes;
     defense = processors on the hex + its defense tokens (the firewall; each prevents
     its owner's defense value). Each point of excess flips a data centre on the hex into
     a cluster (neutral first). **Adaptation:** processors never flip (losing one would
     need a way back onto an AI track, which v0 lacks); with `infection_spill` 1, excess
     left after that plants one cluster on a hex that has none, so infection spreads.
  2. **Combat**, on each hex of the terrain that now holds clusters: `cluster_damage` (1)
     per cluster, less defense, destroys players' pieces as a ravage does (players choose
     each piece): population `population_health` 1, factories and data centres
     `building_health` 2 (the notes' "clusters 2, factories 2, population 1"; a data
     centre is a neutral cluster). Corruption and cascades as in a ravage. **Adaptation:**
     the notes' military fights first and only leftover damage hits these; v0 has no
     military, so all of it does.
- **Difficulty 3** adds `damage_bonus` (1) to every assaulted hex.
- **Purge win (Adaptation):** at bar level *L* (from Unity tips) players win when at most
  *L* × `purge_allowance` (2) × countries clusters remain (level 0: none).
- **Pushback** (the Civil Defense Unity card) removes a cluster instead of a drone.

### Asset corruption (`assets`)

The adversary has no pieces of its own. It **corrupts** factories and data centres (the
assets): the asset's token is flipped to the adversary's side (`rogue_factory`,
`rogue_data_centre`) and one corruption from the pool goes with it. A corrupted asset
counts as an adversary piece (for Blockade, Sabotage, Panic, card targets) and produces
nothing. The pool is the limit: players lose when it is empty, so the pool size
(`pool_per_country` 3, `pool_base` 1) is how many assets the adversary may hold.

- **Setup (Adaptation):** the board's starting drones and outposts are not used. After
  the first reveal, a **beachhead** in each country: the asset nearest its printed
  outpost uplink is corrupted (ties: lower hex number; a factory before a data centre).
- **Corrupt** (explore slot, on reveal): in each country, one asset next to a hex of the
  card's terrain is corrupted; players choose which. Firewall Node protects assets on
  hexes holding its owner's processor (**Adaptation**: it stops explore).
- **Spread** (build slot): each corrupted asset (as the step starts) corrupts `spread`
  (1) assets on its hex or an adjacent one (any country), each next to a hex of the
  card's terrain; players choose each.
- **Strike** (ravage slot): each hex of the card's terrain next to a corrupted asset
  takes `rogue_damage` (1) per corrupted asset next to it (+ `damage_bonus` at difficulty
  3), less defense, destroying players' pieces as a ravage does. No corruption is added
  (corruption only comes with assets).
- **Adaptation, terrain:** factories and data centres sit on Industrial and Network
  hexes, so a card reaches an asset "next to" its terrain (on or adjacent): every card
  stays live, and Urban and Frontier cards strike where the population is.
- **Reclaiming:** player damage of `rogue_health` (2) reclaims a corrupted asset: it flips
  back and its corruption returns to the pool, swaying `reclaim_unity` (2) toward Unity. Purge
  Protocol reclaims every one in the hex. Rebuild and Reconstruction "return
  corruption" by reclaiming one corrupted asset each (no Sway). Pushback reclaims
  one per country (players choose).
- **Purge win (Adaptation):** at bar level *L*, at most *L* × `purge_allowance`
  (1) × countries corrupted assets remain (level 0: none).

### Explore from uplinks (`uplinks`, 011)

The Militarist with one change, for a spread that is easier to read: **explore adds
drones only on uplinks** of the card's terrain. Outposts no longer seed explore; they
still build (every hex holding adversary pieces builds) and ravage. With
`explore_foundries` 1, hexes next to a foundry explore too. Every other rule, parameter
and difficulty level is the Militarist's (`--set adversary.variant=uplinks --set
adversary.explore_foundries=0`). Drones still reach the interior by Momentum and by
threats (Conscription, Drone surge). Not the default.

### Threats and cards under the spread variants (Adaptation)

Threats and cards name the Militarist's pieces. Under `clusters`, a drone, outpost or
foundry they name reads as a cluster; under `assets`, as a corrupted asset, and adding
one means the corruption spreads from that hex (to an asset on it or adjacent, players
choose).

| Threat | `clusters` | `assets` |
|---|---|---|
| Conscription | Each hex with clusters gains 1 | Each hex with a corrupted asset spreads |
| Drone surge | Each uplink gains 1 cluster | Each uplink spreads |
| Forward base | The lowest-numbered uplink with a cluster gains 1 | The lowest-numbered uplink spreads |
| Fortify | Each hex with clusters gains 1 Fortify per cluster | Per corrupted asset |
| Patrol | Hexes with clusters and population | With a corrupted asset and population |
| Total war | Each hex with clusters fights (combat) | Each hex with a corrupted asset strikes |
| Cascade | Unchanged | Each hex with a corrupted asset spreads |
| Mass production | Each hex with clusters is hegemonized | Each hex with a corrupted asset spreads |
| Overrun | Unchanged (3+ clusters) | 3+ corrupted assets: the hex spreads |

- **Sweep, Swarm Override** ("remove every drone in a country"): `clusters`, each hex of
  the country loses one cluster; `assets`, never offered (nothing to remove).
- Cards that check for a destroyed foundry this round take their other branch.

## 2b. Off-variants

For measuring what a module adds, these variants remove its effect and keep the round:

| Variant | Removes |
|---|---|
| `threats.variant=off` | Threats: none are dealt, countered or executed. Jam Signals, Counter-intelligence and the `counter_threat` reward have no target. |
| `alignment.variant=off` | Alignment: no Sway moves, nothing tips, no antimatter action; the bar stays at 0 (winning and losing stay). |
| `cards.variant=off` | The draft and drawing: players have only their AI's unique cards. |

Not removable: `board` (the map), `adversary` (the clock and every loss), `alignment`
(winning and losing; only the track is), `economy` (all income: without it nothing can
be paid), `ai` (seats, insight, processors). Parts of them can be switched off by
parameters: Momentum and Escalation (`momentum_drones`, `escalation_outposts` 0),
giving (`economy.give_insight` 0), or by leaving `ai.growth` out of the round.

## 3. Alignment and the end of the game (module `alignment`)

Alignment (013) replaces human opposition. The AIs' world leans toward **Unity** (the
good AIs, the players) or **Discord** (the evil AI, the adversary). Everything on the
track is printed: Sway tokens on three spaces, two tip-count tracks, two small decks and
an ongoing row.

**The track.** Sway tokens sit in three places: **the Undecided** (the middle pool,
`alignment.undecided`), the **Unity bucket** and the **Discord bucket**. The Undecided
starts with `undecided_per_player` (4) × players Sway; both buckets start empty.

**Swaying** toward a side moves Sway one at a time from the Undecided into that side's
bucket. If the Undecided is empty:

| Variant | Sway with the Undecided empty |
|---|---|
| `tug` (default) | comes from the opposite bucket (a tug of war) |
| `still` | nothing happens |
| `off` | (for measuring Alignment, §2b) no Sway ever moves and nothing tips |

**Tipping.** A bucket holding `tip_per_player` (2) × players Sway **tips**, in this
order: one card of its side resolves; that side's tip count goes up by one; the bucket's
Sway returns to the Undecided. Unity tips and Discord tips are counted separately and
only count up; there is no net meter. **Default:** buckets are checked when an action or
automatic phase finishes resolving (with win and loss, below); a bucket may overfill
within one action, and all its Sway returns when it tips. If both are full, Discord tips
first; a tip's card may fill a bucket again, which then tips too.

**Momentum** (`momentum_every`, 0 = off, the default): a side's bucket holds 1 fewer Sway
for every `momentum_every` (2 when on) tips that side has made, down to 1 per player.

**Sources of Sway**

| Toward | From | Amount |
|---|---|---|
| Unity | destroying an outpost / a foundry (as opposition was) | `outpost` 1 / `foundry` 2; Unity Network +1 for its owner |
| Unity | Rally (2, if a hex holds population and adversary pieces), Global Mandate (4), Swarm Override (1 per 2 drones removed) | card text |
| Unity | each AI board, at the start of every round | `unity` on the board (`content/ais.json`; 0 for Builder and Protector) |
| Unity | ongoing Unity cards | card text |
| Discord | the adversary, at the start of every round | `adversary_discord` (1) |
| Discord | a threat that executes uncountered | `threat_discord` (1) each |
| Discord | the **antimatter action** (below) | `antimatter_discord` (1) |
| Discord | ongoing Discord cards | card text |

The round's Sway (adversary, then AI boards, then ongoing cards in the order they came
into play) is the phase `alignment.sway`, first in the round.

**The antimatter action.** In the action phase (`...+alignment.act`), any player may
sway `antimatter_discord` (1) toward Discord to produce 1 antimatter, up to
`antimatter_uses` (1) times a round each. `antimatter_discord` 0 removes the action.

**The cards.** Each side has a small deck in two tiers, shuffled at setup
(`content/alignment.json`). A tip draws from the stronger tier once that side's tip
count, before the tip, has reached its first threshold (`unity_ease[0]` /
`discord_escalate[0]`, both 2), else from tier 1; from the other tier if that one is
empty; with both empty, the side's resolved cards are shuffled back first. A one-shot
card resolves and goes face up on its side's resolved pile; an **ongoing** card goes into
the ongoing row and resolves at the start of every round from the next one on.

| Side | Tier | Card | Effect |
|---|---|---|---|
| Unity | 1 | Grassroots Funding | every player +2 credit income for the rest of the game |
| Unity | 1 | Open Market | the next 2 card purchases are free |
| Unity | 1 | Think Tank | every player +1 insight each round for the rest of the game |
| Unity | 1 | Ceasefire | counter one active threat (players choose) |
| Unity | 1 | Civil Defense | in each country, remove 1 drone (players choose) |
| Unity | 1 | Volunteer Corps | ongoing: each round, sway 1 toward Unity |
| Unity | 2 | Global Coalition | counter every active threat |
| Unity | 2 | New Deal | every player +2 credit income and 4 credits now |
| Discord | 1 | Incursion | deal one extra threat |
| Discord | 1 | Blight | in each country, the hex with the most adversary pieces gains 1 corruption |
| Discord | 1 | Buildup | the adversary builds now with the card in its build slot |
| Discord | 1 | Propaganda | ongoing: each round, sway 1 toward Discord |
| Discord | 2 | Offensive | the adversary ravages now with the card in its ravage slot |
| Discord | 2 | Two Fronts | deal two extra threats |
| Discord | 2 | Firestorm | in each country, the hex with the most adversary pieces gains 2 corruption |

The Unity cards are opposition's old rewards as one-shot cards; the Discord cards are
adversary effects the game already has. **Defaults:** extra threats come from the
current tier's deck (no reshuffle out of turn), one per country in turn from country 0,
and are countered or execute like any other; "most adversary pieces" ties go to the lower
hex number, and a country with none gains nothing; Buildup and Offensive act on the slot
cards as they are (empty slot, nothing), out of the pipeline's order.

**Unity eases the win, Discord escalates the adversary.** The clear-board **bar** is the
number of `unity_ease` thresholds (2 and 5) the Unity tip count has reached:

- bar 0: win when no adversary pieces remain (from stage II, below);
- bar 1 (2+ Unity tips): win when no outposts or foundries remain;
- bar 2 (5+ Unity tips): win when no foundries remain.

There is no instant win any more (opposition's top level was one). Each `discord_escalate`
threshold (2 and 5) the Discord tip count reaches raises the **difficulty in play** by
one level (`adversary.escalation`, up to 4): from then on the rules of that difficulty
level apply as in §2 (counter bonuses and extra threats from level 1, foundry reach from
3, early tier-2 threats at 4). Setup rules of a level (stage-I cards removed, a smaller
pool) are not applied mid-game, and at difficulty 4 escalation adds nothing beyond the
stronger Discord tier.

**Winning:** players win immediately when the adversary's presence is under the bar.
(The spread variants count clusters or corrupted assets instead: [§2a](#2a-spread-variants-module-adversary-variants-clusters-and-assets).)

**The bar-0 win waits for stage II** (011, the default): the bar-0 condition ("no
adversary pieces") counts only once an adversary card of stage `clear_win_stage` (2)
has been revealed. Before that, clearing the board wins nothing. With no starting
outpost the opening board is 4 drones, and Sweep or Watchtower Net could clear it for a
round-2 win (007: a random team did it; greedy in 3–4% of games). `clear_win_stage` 1
is the r3 rule (any time); `v0_untuned.json` keeps it. The stages are stacked and
public, so the table can always see whether it counts. The gate applies to every
adversary variant: under `clusters` and `assets`, the bar-0 purge win (none left) also
waits for stage II.

**Losing:** total population falls below `loss_population_percent` (50) % of the start;
the corruption pool is empty; or the adversary deck is empty when a card must be revealed.

**Default:** win and loss are checked whenever an action or automatic phase finishes
resolving (including after the answer that completes it), losses first; not in the
middle of a ravage or a card's resolution. (The ruleset says win is immediate and lists the check in
upkeep; checking everything immediately is the simplest reading and can only end a game
sooner.)

## 4. Resources and the round

### Resources

Each player has credits, compute, energy, antimatter and insight.

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit income (AI track, buildings, Unity cards) in upkeep; each working factory pays `factory_credits` (3; v0 had 1) to every player with a processor in its country | Keeping drafted cards; card costs; counters |
| Compute | `processor_compute` (1) per processor on a working data centre, in upkeep; buildings | Drawing (`draw_cost` 3 → 1 card); abilities; counters |
| Energy | Energy income (AI track, buildings); growth | Abilities; counters |
| Antimatter | Late track values; Scavenge; Antimatter Refinery | Tier-2 cards; counters |
| Insight | At the start of each round, set to the AI track's insight plus buildings (unspent insight is lost) | Card plays; giving |

- A **factory works** if its hex is uncorrupted and next to population (and not
  blockaded); each factory on a hex counts.
- A **data centre works** if its hex is uncorrupted (and not sabotaged). A processor
  produces compute only on a hex with a working data centre.
- **Giving insight:** a player may give 1 insight to another; the receiver gains
  `give_insight` (1).

### Damage and defense

- **Player damage** in a hex is first reduced by Fortify there (1 per Fortify token).
  The dealer then destroys adversary pieces one at a time, choosing each among those the
  remaining damage can destroy (full health needed); damage left over is lost. Destroyed
  pieces leave the board and sway toward Unity.
- **Defense** tokens belong to a player (`defense:<seat>`). Each prevents 1 ravage damage
  in its hex (the Protector's prevent 2). Defense and Fortify reduce every damage instance
  in the hex that round and are removed at the end of the round.

### The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities refresh; round markers clear |
| 1 | `threats.deal` | Deal `per_country` (1) threat face up onto each country (tier 1 until stage II has begun, then tier 2) |
| 2 | `cards.tier` | Players who have unlocked tier 2 choose the tier they are dealt from |
| 3 | `cards.deal` | Each player is dealt `deal` (4) cards from their tier |
| 4 | `cards.draft` | Take one card, pass the rest to the next seat until the hands are gone: to seat *s*+1 in odd rounds, *s*−1 in even rounds (as in Terraforming Mars). No passing |
| 5 | `cards.keep` | Keep any drafted cards at `keep_cost` (1; v0 had 3) credit each; the rest are discarded when the action phase begins |
| 6 | `ai.growth` | Each player picks one growth option (mandatory) |
| 7 | `cards.play+threats.counter+economy.give` | Actions, simultaneous: play cards, use abilities, draw, pay or withdraw counter payments, give insight. Each seat acts until it passes |
| 8 | `threats.execute` | Partial payments are refunded; uncountered threats execute in order and are discarded |
| 9–12 | `adversary.ravage`, `.build`, `.explore`, `.slide` | The pipeline |
| 13 | `economy.upkeep` | Income and production; building upkeep |
| 14 | `board.cleanup` | Remove defense and Fortify tokens and the round's country flags |

**Simultaneous actions** may now depend on each other (pooled counters, gifts, a shared
board): any seat acts in any order, as at a table where the team talks it through.

## 5. AI boards (module `ai`; content [`content/ais.json`](../content/ais.json))

Each AI board has two tracks covered by processors except the first value. Placing a
processor takes it from the left end of either track (players choose), uncovering the
next value; the current value is the highest uncovered one. A processor goes on any hex
within 1 of one of your processors, or on any data centre in a country where you already
have one. A track's last value may also give antimatter income; a marked value unlocks
tier 2 for good once uncovered.

**Builder** (6 building slots; innate *Economies of scale*: your buildings cost 2 less).
Insight 2 2 3 3 4 4; credit income +1 +2 +3 +4 (tier 2) +5 +6 (+1 antimatter). Growth:
place 1 processor and gain 3 credits; gain 8 credits; gain 2 energy and your next
building this round costs 4 less. Unique cards: Turnkey Factory, Venture Capital,
Modular Expansion.

**Protector** (3 slots; innate *Bulwark*: your defense tokens each prevent 2). Insight
1 2 2 3 3 4; energy income +1 +1 (tier 2) +2 +2 +3 +3 (+1 antimatter). Growth: place 1
processor and gain 1 energy; place 2 defense in any one country and gain 2 credits; gain
3 energy. Unique cards: Point Defense, Strike Team, Shelter Order.

## 6. Threats (module `threats`; content [`content/threats.json`](../content/threats.json))

- Variant **`countries`** (default): each threat sits on one country and acts there;
  only players with a processor in that country may pay toward it, and "each player"
  in its text means each player with a processor there.
- Variant **`shared_row`**: threats sit in one row, act in every country, and anyone may
  pay. **Default:** the row is still dealt `per_country` threats per country, so with
  more countries there are more threats and each acts everywhere.
- Variant **`off`**: no threats ([§2b](#2b-off-variants)).

**Countering (Default):** a threat is countered only when fully paid. Payments are
pooled across players (`Pay` 1 or as much as is useful at a time, `Withdraw` takes back
everything you put on it in one resource). The moment it is fully paid it is discarded
and the payments are spent. Anything still on a threat when the action phase ends is
refunded. Threat decks reshuffle their discards when they run out.

| Threat | Tier | If not countered | Counter |
|---|---|---|---|
| Conscription | 1 | Each hex with an outpost gains 1 drone | 2 energy |
| Supply raid | 1 | Each player loses 3 credits (down to 0) | 4 credits |
| Drone surge | 1 | Each uplink gains 1 drone | 2 compute |
| Blockade | 1 | Factories next to adversary pieces produce nothing this round | 2 compute |
| Forward base | 1 | One drone on the lowest-numbered uplink holding a drone becomes an outpost | 3 energy |
| Panic | 1 | One population next to adversary pieces flees to an adjacent hex in its country without them (players choose which and where); with nowhere to go, it is lost. **Default:** "in its country", from the ruleset's "in each country" | 1 insight |
| Sabotage | 1 | Processors on data centres next to adversary pieces produce nothing this round | 2 compute + 1 energy |
| Fortify | 1 | Each foundry gives its hex 1 Fortify this round | 3 credits |
| Requisition | 1 | Each player discards 1 card or pays 3 credits (their choice; with neither possible, they pay what credits they have) | 1 insight |
| Patrol | 1 | Each hex with a drone and population loses 1 population | 2 energy + 2 credits |
| Total war | 2 | Every hex with a foundry ravages | 2 antimatter |
| Cascade | 2 | Each corrupted hex cascades to one adjacent hex (players choose) | 1 antimatter + 3 energy |
| Orbital strike | 2 | Destroy one factory on the hex with the most population on adjacent hexes (ties: lower number) | 1 antimatter |
| Mass production | 2 | Every hex with an outpost builds | 6 energy |
| Purge protocols | 2 | Each player discards their most expensive card in hand (ties: they choose) | 1 antimatter |
| Overrun | 2 | Each hex with 3 or more adversary pieces gains 1 corruption (no cascade) | 2 antimatter |

Population lost to Patrol or Panic is protected by Shelter Order and Civic Shelter like
any population loss.

## 7. Player cards (module `cards`, variant `draft`; content [`content/cards.json`](../content/cards.json))

(Variant `off`: no draft and no drawing, [§2b](#2b-off-variants).)

**Variant `free_keep`** (011, not the default): the draft as above, but each player's
first `free_keeps` (1) kept cards each round cost nothing; each further keep costs
`keep_cost`. 007's proposal 6 reads "keep one card free each round, others at 2", so it
is measured with `keep_cost` 2 (`--set cards.variant=free_keep --set
cards.free_keeps=1 --set cards.keep_cost=2`). A round's free keep is used before an
Unity card's free purchase (Open Market), then credits.

- **Decks:** a tier-1 and a tier-2 deck of the cards listed in the config (one copy of
  each; a hard constraint the tests check). Discards reshuffle into a deck when it runs
  out. An AI's unique cards go back to the box when played (programs) or discarded.
- **Playing** a card costs its insight, its credits (buildings minus your discounts:
  Economies of scale, Assembly Cluster, a growth's next-building discount) and any
  antimatter. A tier-2 card needs your AI to have unlocked tier 2 and the listed tags.
  **Default:** tags are counted on your buildings (a program leaves no tags); the
  ruleset gives tier-2 cards no tags, so they have none.
- **Programs** resolve and are discarded. A program (or ability) is only offered when
  each of its targeted effects has a target.
- **Buildings** go into your building row. **Default:** playing a building into a full
  row discards one of your existing slot-taking buildings (your choice).
- **Abilities** ("once a round") are used with `Use` and refresh at the start of a round.
- **Drawing:** pay `draw_cost` (3) compute to draw the top card of the tier-1 deck, or of
  the tier-2 deck once unlocked.

Effects needing choices ask the playing player: hexes, countries, players, threats,
amounts ("up to N" offers 0..N), or "stop".

Card details beyond the ruleset text:

- **Strike Team:** "within 1 of your processor" is hex distance ≤ 1, any country.
- **Precision Strike, Orbital Lance, Purge Protocol:** any hex holding adversary pieces.
- **Watchtower Net, Point Defense:** a country where you have a processor.
- **Sweep, Swarm Override, Shore Up, Shelter Order:** any country.
- **Emergency Fab:** "empty" means no population, factory, data centre or adversary
  pieces. **Turnkey Factory:** any Industrial hex next to population, even one with a
  factory.
- **Evacuate:** up to 3 moves, one population at a time, anywhere within the country of
  the first one moved; stop at any time. **Shore Up / Point Defense / growth defense:**
  choose the country, then a hex for each token.
- **Recon Sweep:** the top 2 adversary cards are shown face up until revealed.
- **Jam Signals:** the chosen threat needs 1 less of each resource (not below 0) this
  round; if that leaves it fully paid (or needing nothing), it is countered at once. **Counter-intelligence:** partial payments on it are refunded.
- **Rebuild / Reconstruction:** return corruption to the pool one hex at a time
  (Reconstruction: up to 3, all in the first hex's country).
- **Overclock Rig:** its upkeep (1 energy) is paid automatically in upkeep if you can;
  if not, its effects are off next round. **Antimatter Refinery:** converts automatically
  when you have the energy.
- **Civic Shelter:** automatic, the first time population would be lost that round in a
  country where its owner has a processor.
- **Logistics AI (Deviation):** gain 2 credits when a threat you paid toward is countered
  by payment, not on every payment: with withdrawals allowed, a per-payment trigger
  would pay out without limit.
- **Modular Expansion (Deviation):** it takes no building slot itself, so it is a net
  +1 slot; taking a slot would make it do nothing.

## Defaults and deviations

**Defaults** (gaps in the ruleset, filled with the simplest reading that matches its intent):

1. Country pieces, hex numbering and the 4-country limit (§1).
2. One player per country; seat *s* starts in country *s*; the first processor goes on the
   lowest-numbered data centre there; `ai.boards` repeats for more seats, and a repeated
   AI has no unique cards (one copy each).
3. "Next to" = on or adjacent; "adjacent" = neighbouring, across borders unless "in the
   same country" is stated.
4. Ravage loss order: chosen piece by piece among pieces the remaining damage can
   destroy; leftover damage is lost. The same for player damage. Defense and Fortify
   reduce each damage instance that round.
5. Cascades do not chain and may cross borders. Momentum once per ravaged hex.
6. The corruption pool "runs out" when it reaches 0.
7. Win and loss are checked when an action or automatic phase finishes resolving: full
   buckets tip first (Discord before Unity), then losses, then the win.
8. The adversary deck deals 4/4/2 (`stage_cards`) of 4/4/6 printed cards; Escalation triggers on reveal.
9. Difficulty 4's "a round sooner" looks `tier2_early` cards ahead in the deck.
10. Pooled counters: countered only when fully paid; withdrawals until the action phase
    ends; partial payments refunded then.
11. A building into a full row discards one of your slot-taking buildings (your choice).
12. Insight is set at the start of each round; income arrives in upkeep.
13. Tags are counted on your buildings; tier-2 cards have no tags.
14. "Next K purchases" are keeps of drafted cards.
15. Unique AI cards return to the box when used up.
16. Requisition with neither a card nor 3 credits: pay what credits you have.
17. Team choices are asked of seat 0.
18. Building upkeep is automatic: Overclock Rig's energy is paid whenever you can, and
    Antimatter Refinery converts whenever you can (no choice is asked).
19. The draft passes alternate direction by round (Terraforming Mars).
20. The shared threat row is dealt `per_country` threats per country.
21. Panic's population flees within its own country.
22. A giving card (Power Share) is not offered without another player; Grant and
    Venture Capital are (their own gain still applies).
23. (011) Difficulty 1's extra threat goes to the countries in turn by round number;
    difficulty 3's foundry reach hits adjacent hexes in any country, before the
    foundry's own hex ravages, and adds no corruption or Momentum.
24. (011) "Stage II has begun" for the clear-board win means a stage-II card has been
    revealed into the explore slot (as for tier-2 threats).
25. (013) Alignment: a tip draws the stronger tier when the side's tip count *before*
    the tip has reached its first threshold; ongoing cards start the round after they
    come into play; the antimatter action is once a round per player; Discord
    escalation is the difficulty level in play (§3).

**Deviations** (the implementation departs from the ruleset text):

1. Logistics AI triggers on a countered threat you paid toward, not on every payment.
2. Modular Expansion takes no slot.
3. The 5-country layout is gone (v0 has four country pieces).

**Adaptations** (the spread variants, §2a, fitting design notes to the v0 game):

1. Both keep the Militarist's deck, pipeline slots, phases, pool and losses; no
   Momentum or Escalation.
2. Clusters: starting drones and outposts become one cluster each.
3. Clusters: placing is v0's explore, sourced from hexes with `source_clusters`.
4. Clusters: Hegemonize only hexes holding a cluster (`hegemonize_all` 1: every hex).
5. Clusters: data centres are the neutral clusters, processors the friendly ones;
   processors never flip; `infection_spill` plants one cluster with leftover excess.
6. Clusters: no military, so all combat damage hits clusters, factories and population;
   data centres and factories 2 health, population 1.
7. Assets: a corrupted asset is its token flipped plus one corruption from the pool;
   losing on an empty pool caps the assets the adversary may hold.
8. Assets: the board's starting adversary pieces are unused; a beachhead per country
   starts the corruption.
9. Assets: cards reach assets next to their terrain; strikes hit hexes of the terrain.
10. Assets: damage, Purge Protocol, Rebuild, Reconstruction and pushback reclaim.
11. Both: the purge win allows *L* × `purge_allowance` × countries pieces at bar *L*.
12. Both: threats and cards naming drones, outposts or foundries read them as clusters or
    corrupted assets (table in §2a).
