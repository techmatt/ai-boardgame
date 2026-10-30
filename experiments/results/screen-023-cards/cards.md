# The player card set (023, rules r14)

`aic-sim experiment experiments/screen-023-cards.json --quick --out out/screen-023-cards`
(480 games) and `aic-sim experiment experiments/screen-023-dominance.json --quick --games
32 --out out/screen-023-dominance` (1,344 games, d2 only), side by side on 12 threads
(~5 and ~12 min), then `python scripts/cards/screen_023.py out/screen-023-cards
out/screen-023-dominance`. 16 seeds a cell on the screen (noise about ±12 points on a
win rate), 32 on the gate (about ±8, ±8-12 on a gap). `cards/` holds the final screen
(`summary.md` is the script's full output: per-card keep and play rates, tags, by-round
tables, the dominance tables), `first-cut/` the set as designed, `dominance/` the gate,
`records/` the seeds below. The cards are [docs/cards.md](../../../docs/cards.md); the
pricing reasoning [docs/card-pricing.md](../../../docs/card-pricing.md).

## Verdict

- **Engine at d0: 75% (2 countries) and 81% (3): below the ~90% target.** r13's 52
  placeholder cards gave 94% / 94%. The first cut of the set gave 50% / 25%.
- **Dominance gate (d2, 32 seeds): not met, and the failures are informative.**
  Losing clearly, as they should: keep-all (+31 / +19 points behind the engine),
  no-presence (+31 / +25), event-hoarder (+28 / +25), every mono-tag check (+25 to +31,
  except mono-Diplomacy at c2: +3 ±11), ignore-Diplomacy (+28 / +25), keep-cheapest-k
  (+12 / +25). Not losing: **ignoring Science, Industry, Cyber or Security beats the
  engine** (ignore-Cyber 59% against 31% at c2, gap −28 ±12; ignore-Science −12 at both
  sizes; ignore-Industry −16 / −3; ignore-Security −9 / +6); ignore-Power is level at c2
  and loses at c3 (+16). tier-1-only, tier-2-rush, no-counters and no-powers sit within
  noise of the engine (the tier choice and the AI powers barely matter).
- **Structural cause:** the purge win is made of Diplomacy (freeing cities, Unity) plus
  removal and presence. Science's draw and insight, Industry's credits (the engine leaves
  70-170 credits unspent a game), Cyber's project hacks and Security's hardening cost
  the scarce resource, insight (keeps ~6 a round for both seats against ~2 on plays), and
  don't move the purge. A seat that never keeps them spends that insight on Diplomacy.
  Not an agent artefact: with projects weighted 2.5× (`w.project=20`) the engine reaches
  41% at c2 d2 and ignore-Cyber still 59%. The fix is content and rules work for the next
  pass: give each tag a part of the win (Cyber's "expel ambassadors" and node cuts
  counting toward the purge, Security's troop kills, Industry's stations and factories
  as Unity sources), or make the adversary's projects hurt enough that draining them is
  needed. Matt's call; noted, not built.

## Win rates: before and after

r13 is 022's committed run (the 52 placeholders); "first cut" is the 150 cards as
designed (r14 with `free_city` 3, no agent changes); "final" is the committed state.

| countries | d | greedy: r13 / first cut / final | engine: r13 / first cut / final | engine tier-2 plays a game: first cut / final |
|---|---|---|---|---|
| 2 | 0 | 75 / 31 / 56 | 94 / 50 / 75 | 0.0 / 4.1 |
| 2 | 1 | 44 / 6 / 44 | 69 / 6 / 44 | 0.0 / 4.6 |
| 2 | 2 | 31 / 0 / 25 | 62 / 12 / 31 | 0.0 / 4.4 |
| 2 | 3 | 25 / 0 / 31 | 44 / 0 / 19 | 0.0 / 2.8 |
| 2 | 4 | 12 / 0 / 6 | 25 / 6 / 19 | 0.0 / 1.1 |
| 3 | 0 | 25 / 0 / 56 | 94 / 25 / 81 | 0.1 / 7.9 |
| 3 | 1 | 19 / 0 / 31 | 62 / 12 / 31 | 0.0 / 7.8 |
| 3 | 2 | 38 / 0 / 6 | 50 / 0 / 25 | 0.0 / 7.1 |
| 3 | 3 | 25 / 0 / 12 | 44 / 0 / 25 | 0.0 / 5.3 |
| 3 | 4 | 12 / 0 / 6 | 6 / 0 / 6 | 0.0 / 2.7 |

## The tuning steps (engine, 16 seeds, `experiments/tune-023.json`)

| step | c2 d0 | c3 d0 | c2 d2 | c3 d2 |
|---|---|---|---|---|
| first cut | 50 | 25 | 12 | 0 |
| + 2 more freeing cards, the audit's fixes, tier-2 `needs` one lower | 69 | 44 | 12 | 12 |
| + the agent values the tier pick (`w.packet_keep` 1; 3 was worse: 56 / 31) | 75 | 56 | 25 | 19 |
| + `alignment.free_city` 5 (6: 88 / 81, 12 / 31; with `unity_ease` 1,1,1: no change) | 88 | 81 | 25 | 6 |
| + program abilities valued like buildings' (committed) | 75 | 81 | 31 | 25 |
| tried: `cards.deal` 5 | 69 | 12 | 19 | 6 |
| tried: `w.project=20` / `w.packet_keep=0` | 88 / 69 | 62 / 62 | – | – |

## Adaptations (each a candidate)

| Change | Why |
|---|---|
| `alignment.free_city` 5 (was 3) | d0 losses were corrupted cities left when the deck ran out: with 150 cards the freeing cards come round a third as often as in 52 |
| Station Prefab Order and Back-Channel Talks take a cube off a city or station | 3 freeing cards in 150 against 2 in 52; Industry's "stations" and Diplomacy's "freeing cities" |
| Tier-2 `needs` one tag lower (min 1) | tier-2 cards needing 3-4 of a tag were unplayable in an 8-10 round game |
| The card audit's fixes: Liquidity Event and Megacorp IPO cash out at most 5 / 6 Capital (`cash_out` got a `max`); Exabyte Archive 1 per 2 Datasets, max 4; Robotic and Arcology Foundry no longer lower energy (the Builder has none); Supply Chain Treaty 7; Synthetic Data Forge 4 Datasets; Citation Engine 1 per tag, 5 credits; Flash Exploit 3 credits, 1 insight; Positron Storage Ring +4 energy, no income cut; Beamed Power 5 credits, no energy; Peaker Turbines +2; Substation Hub +1; Shadow Broker Feed and Threat Forecast Engine 2; Endowed Research Chair 5 / 1 insight with a draw; Aegis Grid Lockdown no antimatter; Martial Cordon moves instead of consolidating (it was unplayable without an ambassador in reach); Planetary Referendum 4; Threat Triage 1 insight; Deep State Ledger no income cut; Consulate Tower 5 | a three-agent text-versus-JSON and price audit of every card |
| An `if` card is offered only when its firing branch has a target | Adaptive Malware could be paid for and do nothing |
| Agent: the draft packet counts like drafted cards less `w.packet_keep` × the keep price | a tier pick scored nothing, so ties went to tier 1: at r13 tier 2 was drafted only when the 52-card tier-1 deck ran dry; with 108 it never did (0 tier-2 plays a game) |
| Agent: a program in play with an ability earns `w.ability` a round, like a building | 021 allows program abilities; agents kept them and never played them |

## Cards

From `cards/summary.md` (greedy and engine games together):

- **Dead (45)**, kept 3+ times and never played, or kept under 15% of the times drafted:
  mostly tier-2 buildings that must be built over one of your own (Exabyte Archive,
  Foundation Model Campus, Hyperscale Botnet, Black Ice Citadel, Orbital Mass Driver,
  Arc Furnace Complex, Dyson Swarm Tranche, Positron Storage Ring, Planetary Assembly,
  Orbital Embassy, Agency Directorate: `cards.replaced` is 0.0-0.1 a game), the project
  hacks (Compute Siphon, Zero-Click Breach, Killswitch Directive, Cascade Failure,
  Intrusion Lab, Singularity Exploit), the token engines that pay late (Sovereign Capital
  Fund, Deep State Ledger, Kill Team Roster, Sentry Drone Wing, Synthetic Data Forge,
  Liquidity Event, Megacorp IPO, Manifesto Leak), the income-cut insight cards (Neural
  Mirror, Endowed Research Chair, Recursive Self-Improvement, Antiproton Offtake Deal)
  and shelter/hardening events (Storm Hardening, Hardpoint Lockdown, Aegis Grid
  Lockdown, Total Containment). Full list in `cards/summary.md`.
- **Must-picks (26)**, kept every time drafted and played 80%+: Amnesty Decree,
  Back-Channel Talks, Ceasefire Monitors, Consular Pouch, Consulate Tower, Development
  Loan, Diplomatic Expulsion, Embassy Compound, Mutual Aid Pact, Peace Dividend,
  Peacekeeper Rotation, Shuttle Diplomacy (Diplomacy: 12 of 26), Breakthrough Leak, Open
  Preprint, Red-Team Audit, Shadow Broker Feed, Threat Forecast Engine, Eviction Worm,
  Intrusion Countermeasures, Shared Threat Intel, Bulk Silicon Procurement, Joint
  Fabrication Pact, Station Prefab Order, Pylon Drop, Relay Mast Rollout, Transmission
  Corridor.
- **Tags in the engine's wins** (share of winning games playing one; mean played): at
  c2 d0, Industry, Diplomacy and Security 100%, Power 92%, Science and Cyber 50% (1.2 and
  1.1 cards); at c3 d0 Science 77% (2.5), every other tag 92-100%, Diplomacy most (12.1).
- **Hand and insight** (engine, c2 d0, both seats): hand after keeping 16-20 cards
  from round 2; insight on keeps ~6 a round against ~2 on plays (32 / 18 a game);
  credits unspent 70 a game at c2 d0, 135 at c3. Insight, not credits, is what limits
  play, and keeping is where it goes.

## Seeds for the viewer

Records in `records/` (each replays with `aic-sim replay`):

- `c2-d4-engine-seed7-win-r3.json`: a **round-3 win at d4**: Planetary Referendum
  (Unity 4), Liberation Accords and Back-Channel Talks free cities at 5 Sway each.
  Worth a look: is the free-city Sway plus cheap Unity too fast a purge?
- `c2-d0-engine-seed0-win-r3.json`: round-3 win at d0 with Exodus Corridors (the
  Spread card) and Martial Cordon.
- `c2-d0-engine-seed1-loss-deck.json`: a d0 loss to the deck in round 12, cities still
  corrupted: what the set lacks.
- `c3-d4-engine-seed2-win.json`: the one 3-country d4 win (round 8).
