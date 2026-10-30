# Card pricing: the reasoning (023, rules r14)

Every price here is a **candidate**. The card list itself is [cards.md](cards.md),
generated from `content/cards.json`. The screen that checked these prices, and the
patches it led to, are in
[experiments/results/screen-023-cards/cards.md](../experiments/results/screen-023-cards/cards.md).

## The yardsticks

- **The budget.** A board makes 4 insight a round (+1 on the presence track's spaces,
  most boards reach 5 by round 3-4) and about 9-10 credits plus buildings. Four cards
  are drafted a round. Keeps cost 1 insight (tier 1) or 3 (tier 2); credits buy keep
  insight only, at 5 per insight. A round's insight is roughly "keep 2, play 2", or
  "keep a tier-2 card and play one".
- **Exchange rates used for pricing:** 1 insight is worth 3-5 credits (5 when it buys a
  keep); a drawn tier-1 card about 1 insight (it is blind, not chosen); a point of
  income about 3-4 credits on a tier-1 building (Fab Line 4 for +1 credit +1 hardware,
  Solar Field 3 for +1 energy); a Dataset, Capital, Follower or Agent about 1 credit;
  a lowered point of income 3 credits (the keep plan's rate, 022).
- **The clock.** Games run 8-10 rounds; most wins end in rounds 6-8. A tier-1 income
  card bought in round 2 earns for 5-7 rounds, so a 3-5 round payback is fair. Tier-2
  cards land in rounds 4-7 and earn for 2-4 rounds, so **tier-2 slow income must be
  extremely strong** (Megastructure-class: +5 a round, or several resources at once),
  and tier-2 one-shots must swing the board now (country sweeps, project discards,
  node removal, Unity tips).
- **Insight to play:** 0 for small one-offs worth 1-3 credits and cheap fillers whose
  real value is the tag left in play; 1 by default (a building, a program that stays,
  a removal event); 2 for big effects and nearly all tier 2. Events are not
  automatically 0 (022: free removal events made keep-all beat the engine).
- **Other costs follow the tag:** Science and Cyber pay insight; Industry pays
  hardware and lowered income; Power pays energy and credits; Diplomacy credits;
  Security credits and energy. Antimatter appears only on tier-2 ultimates (9 cards,
  1-2 a tag). Strong income cards carry an income cut.
- **Tokens.** Science's Datasets, Industry's Capital, Diplomacy's Followers and
  Security's Agents. Cards that hold them gain them each upkeep; abilities spend them
  at a rate; tier-2 cards count them (`gain_per_tokens`, the "count as power" pattern,
  built in 023) or cash them out at a growing rate (`cash_out`: 1 + 2 + ... + n).
- **New primitives (023):** `draw` (Science's card draw, tier-1 deck, no keep paid),
  `drain_all` (Cyber's "every project −X"), `gain_per_tokens`, `cash_out`. Not built,
  so not on any card: a keep discount (Science approximates it with
  `next_round_insight`), turning an adversary ambassador, permanent extra reach,
  presence on any hex, spending an ambassador as a cost.

The per-tag reasoning below was written with the cards, one designer per tag, and
edited after the screen where a card changed.

## Science

Yardsticks: a tier-1 keep costs 1 insight (5 credits when short), so **one drawn tier-1 card ~ 1 insight ~ 3-5 credits** (cheaper than a keep because it is blind, not chosen). Players make 4-5 insight and ~9-10 credits a round over an 8-10 round game. Fab Line 4 / Solar Field 3 set a tier-1 income point at ~3-4 credits.

- **Small draw and filler programs (rounds 1-4).** Literature Crawler (1 cr, 0 insight, draw 1 + peek 1) and Cryo-Compute Burn (energy into insight 1:1) are cheaper than the keep they replace, so they cost 0 insight; their real payoff is the Science tag left in play for Citation Engine, the tier-2 `needs` and AI-board powers.
- **Insight converters.** Field Sensor Grid (3 cr for up to 2 insight) and Benchmark Suite (2 cr, 1 insight + 2 Datasets) price insight at ~1.5-2 credits, well under the 5:1 keep rate, which is the tag's identity; capped at 2 so they never pay more than half a round's insight.
- **Draw-for-insight cards cost 1 insight.** Breakthrough Leak (draw 3) nets +2 cards; the hacks with a draw rider (Neural Cartographer, Zero-Knowledge Exploit, Red-Team Audit) pay 1 insight because the draw replaces it and the hack is the real effect, priced against Data Mining (0) and Security Patch (2 cr for 4 compute: Zero-Knowledge Exploit drains only 2).
- **Help cards** (Open Preprint, Shared Compute Grant, Open-Weights Release): the helper pays credits and insight and draws; the other player gets the insight. Net table gain is +1 insight and a card, so they cost about what a draw costs.
- **Dataset engines (rounds 2-5).** One Dataset ~ 1 credit (Data Lake: 2 for 3). Telemetry Archive and Grad Student Pool yield one insight/card every 2 rounds: they earn their 2-3 credits + 1 insight back in ~4 rounds, so they are round-2-to-4 buys. Synthetic Data Forge (2 a round, 5 cr) is the faster battery. Every Dataset spent is one the tier-2 counters no longer see: that is the tension.
- **Endowed Research Chair** (7 cr, 2 insight, credit income -1): +1 insight a round. Played in round 3 it returns ~6 insight (~25 credit-equivalent at the keep rate) for ~13 credits of cost and lost income; played after round 6 it barely breaks even, which is the intended window.
- **Simulation Cluster** (5 cr + 1 hardware): 3 credits a draw is a premium over a keep's insight, but it never runs out; a mid-game credit sink.
- **Citation Engine** (4 cr): +1 credit per 2 Science tags (max 3); with 4 tags it beats Fab Line's credit side by round 5.

Tier 2 lands in rounds 5-8, so it pays back in 2-4 rounds or not at all:

- **Exabyte Archive** (8 cr, 1 hardware, 2 insight, over a Science building): +2 Datasets on build, +2 a round, and a free ability worth 1 credit per Dataset (max 6). With one other Dataset card it reaches 6 a round in two rounds: ~15-20 credits over 3 rounds, Megastructure's (+5) value at a lower price because it takes a Science slot and the Datasets must be kept, not spent.
- **Orbital Observatory Array** (9 cr, needs 2 Cyber): +1-2 insight income and 2 cards every other round; the Cyber-hook building.
- **Data Licensing Deal** (0 cr, 1 insight): one-shot cash-out of a Dataset pile, up to 8 credits + 2 insight, kept tokens. Cheap because its value is entirely what you already built; the tier-2 keep (3 insight) is its real price.
- **Ultimates carry antimatter (1 each)**: Recursive Self-Improvement (10 cr, -1 credit income, +2 insight production, draw 3) and Foundation Model Campus (15 cr, 2 hardware, needs 4 Science: +2 insight income, +1 credit per Science tag max 6, draw 2). From round 6 each must return ~10 insight-equivalents in 3 rounds to beat their keep + play cost, hence numbers above any tier-1 rate.
- **Adversarial Inference Sweep** (7 cr, 2 insight): drain_all 1 is a Cyber primitive borrowed at half strength; the draw 2 is the Science half.
- No keep-discount primitive exists; Predictive Modelling and Open-Weights Release approximate one with `next_round_insight` 1 (every player).

## Industry

Baseline: 20 starting credits, ~9-10 credits and 4-5 insight a round, games of 8-10
rounds. One permanent +1 credit is worth about (rounds left) credits, so ~8 if played
in round 2 and ~3 in round 7. Prices are against Fab Line (4 for +1 credit +1 hardware),
Contract Bid (0 for 3 credits) and Megastructure (13 for +5 credits).

- **Tier-1 income (Payroll Automation 4, Supply Chain Treaty 4, Grid-Tied Smelter 5,
  Bonded Warehouse 5):** earns its cost by round 4-5 when played in rounds 1-3; a dead
  draft pick after round 6. Insight 1: the default price.
- **Robotic Foundry (7 + 1 hardware, insight 2, lowers energy 1):** +3 credit income is
  the strongest tier-1 income, so it carries the income cut and the second insight. The
  energy cut is the Industry-Power hook: a factory eats a power player's grid.
- **Instant gains (Bridge Loan, Bulk Silicon Procurement, Hardware Spot Market, Station
  Prefab Order):** small, any-round tempo cards; insight 0 where the gain is about 3-5
  credits' worth. Bridge Loan's 6 credits are paid back by its -1 income in an early
  round and are nearly free in round 8+, which is fine for a cheap tempo card.
- **Factories (Prefab Plant Kit 3, Expansion Fund 3):** priced at Emergency Fab (2) plus
  the rider; a factory pays 1 hardware on production steps and a site, so these are
  best in rounds 2-5.
- **Capital engines (Retained Earnings 2/0, Expansion Fund, Bonded Warehouse, Sovereign
  Capital Fund 2 a round):** tokens are worth little alone; their value is the cash-out.
  cash_out pays n(n+1)/2, so 4 Capital is 10 credits and 8 is 36. Holding two engines
  from round 3 and cashing out once in round 7 is the intended arc; cashing out every
  round (Materials Hedge Desk, 1 credit a use) is the low-yield alternative.
- **Liquidity Event (0, insight 1)** and **Megacorp IPO (3, insight 2, +2 Capital first):**
  the insight is the gate, not the credits, because the payout scales with how long you
  saved. IPO's +2 Capital adds 2n+3 credits on top of an n-token cash-out.
- **Collateralized Grid:** the "count as power" pattern; up to 3 energy a round with 6
  Capital held, tokens kept, so it rewards saving rather than spending.
- **Help cards (Joint Fabrication Pact, Supply Chain Treaty, Vendor Financing):** the
  helper pays credits and gets a little less than the receiver; Vendor Financing's
  2 Capital is the helper's gain and needs a Capital card to be playable.
- **Tier 2 (rounds 5-9, ~3-5 incomes left):** slow income must pay back in 3 rounds.
  Full Automation Mandate (12, +4 credit +1 hardware production, insight 1) repays by its
  third round; Arcology Foundry (14 + 3 hardware, +5/+2, lowers energy 1) is Megastructure
  plus 2 hardware income for the extra hardware and the energy cut; Arc Furnace Complex
  turns 2 energy into 6 credits a round for a Power-heavy player.
- **Orbital Mass Driver (25, 1 antimatter, 4 hardware):** the tag's one ultimate: +7
  credits, +2 hardware and a 2-credit building discount; ~4 incomes left make it ~40
  credits of value, so the 25 is the brake. The only antimatter cost in the tag.
- **Asteroid Salvage Rights (5, insight 2):** a big one-off (10 credits, 4 hardware,
  1 antimatter) that is equally good in any late round; its cost is the tier-2 keep (3
  insight) plus the play insight.

## Cyber

Cyber has no token. Its timing comes from holding a hack until a project is nearly full, so
the value of a drain is the project it stops, not the raw compute. A cheap project is 18-21
compute at 3 players and the board makes about 6-15 a round, so 5-8 compute is roughly half
to one round of adversary progress: enough to stop a project that would resolve this build.

- **Rate.** About 1.5-2 credit-equivalents per compute (1 insight ~ 5 credits). Flash Exploit
  (2 cr, 5 compute) and Compute Siphon (4 cr + 1 insight, 6) sit at the cheap end because a
  single-target drain is wasted if no project is close; Zero-Click Breach (5 cr + 2 insight,
  8 plus a cut) pays a premium for the one burst big enough to stop a full stage-1 project.
- **Cut node** is worth about 1 compute and is the cheap filler: Blackout Script and Rootkit
  Cache are 0 insight, and Rootkit's 2 credits a use is a fair rate for 1 compute a round.
- **Expel (damage)** is priced against Precision Strike (2 cr + 1 insight for 2 worth, any hex):
  Cyber's is in reach only, so Eviction Worm is a program (tag stays) at 3 cr + 1 insight.
- **Repeatable hacks** (Intrusion Lab, Honeypot Relay, Sentinel Mesh) earn their cost from
  round 3-4 on: 4-6 uses over the game. Bought in round 7+ they don't pay back; that is fine,
  the cheap one-shots cover late play.
- **Insight.** 0 for small hacks and the help kit (they must be casual plays), 1 for removal
  and standard drains, 2 for the 8-compute burst and the tier-2 cards. Insight is the cost
  that competes with keeps, so it is what stops a player from chaining hacks every round.
- **Help cards** (Shared Exploit Kit, Mesh Intrusion Pact): the helper pays, the target gains
  1 insight or 2 credits, the helper gets a small refund plus the shared drain.
- **Tier 2** lands in rounds 5-8 with 2-4 rounds left. Discard (Killswitch Directive, 10 cr +
  2 insight) removes 18-36 compute and a whole effect, so it is the dearest one-shot without
  antimatter. Node Severance (6 cr + 2 insight) is cheaper: a node is ~1 compute a round for
  the rest of the game plus the Unity sway. drain_all 5 (Cascade Failure, 9 cr) is up to 15.
- **Ultimate.** Singularity Exploit is the only antimatter card (1): node removal plus
  drain_all 4, at 14 cr + 2 insight.
- **Tier-2 income** must pay back in ~3 rounds: Hyperscale Botnet (13 cr, needs 3 Cyber,
  built over a Cyber building) yields 4-6 credits and 1 insight a round, ~20-25 credits'
  worth by game end. Black Ice Citadel is an upgrade path for Intrusion Lab (4 compute per
  energy).
- **Income cut.** Neural Mirror's permanent +1 insight costs 1 credit income: insight is the
  scarcer resource, so the swap is the reward.

## Power

Draft cards: [`content/drafts/power.json`](../../content/drafts/power.json). Anchors: Solar Field 3 credits for +1 energy income (in a slot), Power Surge 2 for 3 energy, Fork Process 2 for 1 presence, and the standard actions: 1 energy to place a presence, 1 energy for each hardening token (Protector only).

- **Exchange rates.** Energy is worth about 1 credit: the Builder's discount rate, and the hardening and presence actions price it there. A presence is worth more than 1 energy, because each one uncovers a track space that pays every round (insight, credits, energy, and finally antimatter). The track has only 6 spaces, though, so the presence cards in this tag place 1 to 3 at a time, never more.
- **Tier-1 income (rounds 1-4, 6-9 rounds of payoff).** +1 energy production costs 4 as a program (Baseload Contract), 1 more than Solar Field because it takes no slot and can't be built over. A building pays for itself by round 3-4: Tidal Barrage costs 5 for +2. Peaker Turbines gives +3 energy production for 5 credits and lowers your credit income by 1, a net gain of about 2 a round. It is the strong income card, and the income cut keeps it from being an automatic keep.
- **Presence cards earn their cost at once.** Pylon Drop (1 credit, 0 insight, 1 presence + 1 energy) is barely better than the standard action, and that is on purpose: it is a filler card, also worth playing for its tag. Relay Mast Rollout (2 presence for 2 credits) is worth playing early, while track spaces still have many rounds left to pay out.
- **Hardening is cheap but conditional.** A token only lands on a station or factory a troop can strike, so harden cards cost 0-1 credits and usually 1 energy, which is the standard action's price paid once for several tokens. They are worth it in war rounds and dead otherwise, so the 0-insight ones stay cheap enough to hold on to.
- **Tier 2 (rounds 5+, 3-5 rounds left).** Slow income has to pay back in about 3 rounds, so it is priced well above tier-1 rates. Dyson Swarm Tranche gives +5 energy and +2 credits a round (about 7) for 16 credits, 3 energy, 2 hardware and 1 antimatter, and it has to be built over a Power building. Continental Supergrid gives up to +5 energy for 10. Antimatter output, the scarce tier-2 resource, is priced at about 1 antimatter a round for 8-11 credits plus 1 credit of income: Antiproton Offtake Deal, and Positron Storage Ring, which beats Antimatter Refinery's 4:1 conversion with 3:1 but needs a Power building to replace.
- **One-shot tier-2 cards pay back at once.** Beamed Power Constellation places 3 presence and then pays energy for each presence you have, so it is worth playing even in round 8. Interconnector Treaty's +2 energy production is backed by an immediate gift and a presence, so it is not dead late.
- **Insight.** It is set at 0 for one-shot fillers and helps that net you little (Pylon Drop, Capacitor Discharge, Surplus Auction, Shared Grid Accord, the cheap harden cards). It is 1 for anything that lasts (production, buildings, abilities) and for Storm Hardening, a strong event that blocks a destruction. Every tier-2 card costs 2. With 4-5 insight a round, shared with keeps, a 1-insight income card effectively costs a second card.
- **Energy as a cost.** The tag's own currency gates its strong plays: helps and harden cards spend energy, and the tier-2 cards cost 2-3 energy. That keeps a Power engine from being only credits in disguise, and it makes Power's income matter to Power's own cards.
- **Antimatter as a cost** appears only on the two ultimates: Dyson Swarm Tranche, and Aegis Grid Lockdown (a shelter, 4 hardening tokens and 2 presence in one event).
- **Not used:** the four new ops. Power has no token, `draw` belongs to Science, and `drain_all` and `cash_out` belong to other tags. `gain_per_presence` already covers Power's version of "count it as power".
- **Help cards** (Shared Grid Accord, Brownout Relief, Interconnector Treaty): the helper pays and still gets something (credits, a presence, production), and the other player gets more energy than the helper spent. That is the Power-Diplomacy hook.

## Diplomacy

Baseline: 20 starting credits, ~9-10 credits and 4-5 insight a round, 8-10 rounds. Freeing a corrupted city sways 3 Unity, so an ambassador (two free a city) is worth roughly 1.5 Unity plus board tempo; we price 1 ambassador at about 1.5-2 credits, a Move-2 rider at about 1, Consolidate 3 at about 2, 1 Unity at about 1.5-2.

- **Insight 0 (6 cards)**: the cheap round-1 to round-3 tools (Consular Pouch, Shuttle Diplomacy, Peace Dividend, Peacekeeping Detail, Mutual Aid Pact, Manifesto Leak). Each is one or two small effects; charging insight would crowd out the keeps, and they are what a Diplomacy player fills a turn with.
- **Insight 1 (12 cards)**: everything that persists (token programs, buildings) or does two real things (Summit Caucus, Ceasefire Monitors, Amnesty Decree). Amnesty Decree is an event but a removal (a freed city), so it pays 1 like Precision Strike.
- **Insight 2 (all 7 tier 2)**: tier-2 effects change the board state (free 2-3 cities, 3-4 Unity, spread) and should compete with a tier-2 keep (3 insight).
- **Token programs** (Liaison Corps, Open Assembly, Commons Grid) earn their cost only if played by round 3-4: 1 Follower a round means the first spend lands two rounds later, so 4-5 spends a game at most. Priced 2-4 credits, near Grassroots Movement. Manifesto Leak (1 credit, +3 Followers) is the catch-up that makes a late token card fire immediately.
- **Commons Grid** is the "count as power" pattern: Followers stay, so it rewards stacking several Follower cards; the 1-credit activation keeps it from being free energy, max 3 caps it.
- **Tier-1 buildings** (4-6 credits) follow Fab Line / Solar Field: Treaty Grid Exchange is +1 energy +1 credit at 6 (a slot and three tags); Consulate Tower scales with Diplomacy tags, max 3, so it is only strong in a committed build. Embassy Compound pays for its 2 ambassadors up front and sells movement at 2 credits a use.
- **Help cards** (4): the helper pays (credits, or the card and its insight); the receiver gets slightly more than the helper spends (Mutual Aid: 3 credits for 4 plus an ambassador to you; Development Loan gives a +2 bonus on up to 6). Each gives the helper something back (ambassador, hardening, Unity) so playing one is never a pure gift. Coalition Charter's 3 Unity helps everyone.
- **Tier 2 arrives around round 5-6, so 3-4 rounds remain.** One-shots must pay off now: Liberation Accords (9 credits, needs 2 Diplomacy) frees 2 cities anywhere, ~6 Unity; Great Liberation frees up to 3 in one country (~9-10 Unity) and so is an ultimate carrying 1 antimatter. Planetary Referendum (7 credits, 4 Unity, no needs) is dearer than Global Mandate (5, needs 2 Diplomacy).
- **Slow tier-2 income has to be extremely strong.** Planetary Assembly gets 2 Followers a round and spends 2 for 2 Unity plus an ambassador every round: ~6-8 Unity and 3-4 ambassadors over the remaining rounds, for 12 credits, 1 hardware, a credit-income cut and building over a Diplomacy building. Orbital Embassy (+4 credits, +1 insight, 2 ambassadors) returns ~16 credits and 4 insight by game end against 14 credits, 2 hardware and 1 antimatter, which is why it is the tag's second (and last) antimatter card.
- **Spread is rare**: only Exodus Corridors (7 credits, plus 2 ambassadors to spread with), alongside the existing Diaspora.
- No Diplomacy card spends ambassadors as a cost (there is no primitive for it); credits are the extra cost throughout, with hardware and needs/replaces on the two tier-2 buildings.

## Security

Yardsticks: troop = 2 removal worth (`board.troop_health` 2); Precision Strike pays 2 credits + 1 insight for 2 worth anywhere, Jam Signals 2 credits for jam 1, Counter-intelligence 3 credits + 1 antimatter + 2 insight for a counter. Income ~9-10 credits and 4-5 insight a round; an insight is worth ~5 credits (the keep conversion rate).

- **Agent engines (tier 1, rounds 1-4).** Sentry Drone Wing, Kill Team Roster, Safehouse Network, Agency Field Station gain 1 Agent a round, so they fire every 2-3 rounds: 3-5 uses in a 10-round game if played by round 3, about 1-2 if played after round 6. Priced 3-4 credits + 1 insight so they pay back after two uses. Kill Team Roster spends 3 for a troop (2 worth), Security Cell 2 for 1: the larger the spend, the better the rate, which is why it carries the extra 1 energy.
- **Recruit Operatives (1, 0 insight)** is Seed Funding for Agents: one extra firing, only playable with an Agent card out.
- **Counting Agents as power.** Agency Field Station caps at 3 credits for 1 energy a round and only reaches it at 6 hoarded Agents; spending Agents on removal costs income, which is the tension. Deep State Ledger (tier 2) pays 1 per Agent up to 5: from round 6 with ~3-5 Agents in play that is ~4 net credits a round for 4 rounds, ~16 credits back on 8 credits + 2 insight + 1 credit income. Tier-2 income has to be this strong because it only has 3-4 rounds to run.
- **Removal (tier 1).** Breach and Clear is Precision Strike limited to reach, plus 2 credits on a troop kill, so 3 credits + 1 energy + 1 insight (removal events cost 1 insight). Peacekeeper Rotation and Diplomatic Expulsion remove 1 worth (an ambassador only), so they are 2 credits; the rider (move 2 / 1 Unity) is the reason to take them, and Expulsion's insight is 0 because it is limited to presence countries.
- **Interceptor Battery (5 credits)** is Watchtower Net upgraded: 3 energy for a troop in reach every round. It earns from round 3 on, when troops are moving; 3 energy a use keeps it from replacing removal cards.
- **Threat counters.** Jam 1 is ~2 credits. Threat Triage (jam 2) clears most tier-1 counters outright and halves tier-2 ones (1 antimatter + 3 energy becomes 1 energy), so 4 credits + 2 insight, the only tier-1 card at 2 insight. Intrusion Countermeasures (drain 3 + jam 1) and Shared Threat Intel (jam 1, and it gives 1 insight away) stay cheap because they are halves of a counter.
- **Protection.** Hardpoint Lockdown (shelter + harden 2) is a 0-insight event: it only matters in a war round, so it is a reaction card. Hardened Bunker Complex is Solar Field (+1 energy, 3) plus the Protector's Harden action, so 5.
- **Security Contractor HQ** is Trade Office for Security without the production bump: 4 credits, max 3, needs 6 Security tags to cap, so it is a round 2-4 buy.
- **Help cards.** Mutual Defense Pact (3 credits: harden 2, the partner gets 2 credits, you 1 energy) and Shared Threat Intel: the helper pays roughly what they gain plus the gift, as Grant does.
- **Tier 2 sweeps.** Kinetic Interdiction (damage 3 in every hex of a country: a troop and an ambassador per hex) is the ultimate: 6 credits + 1 antimatter + 2 insight, the tag's only antimatter card. Total Containment (destroy_all + shelter) pays credits and 2 energy instead of Purge Protocol's antimatter, so 9 credits and needs 2 Security. Martial Cordon is a program (tags stay) with consolidate, but 1 Unity per 3 rather than Swarm Override's 2, so 8 credits at 1 insight.
- **Threat Neutralization Mandate** is Counter-intelligence without antimatter: 7 credits + 2 energy + 2 insight and needs 2 Security. A tier-2 threat uncountered costs a permanent loss plus 2-3 compute, worth 10+ credits late.
- **Agency Directorate (12 credits, 2 hardware, built over Security, needs 3)**: 2 Agents a round fires a 2-worth country sweep every other round, about 2 sweeps in the rounds left. Aegis Dome Array bundles Firewall Node and Civic Shelter (4 each) with a harden ability: 10 credits + 2 energy + 1 hardware, worth it only where a war front sits near your buildings.

