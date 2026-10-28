//! Game content as data, embedded from `content/` at compile time (no I/O
//! at runtime): the map (piece shape, layouts, the printed country pieces),
//! player cards, threats, the adversary deck and the AI boards.
//!
//! Effects are composed from code primitives ([`Effect`], [`Passive`],
//! [`ThreatOp`]). A new primitive is a code change; a new card, threat,
//! adversary card or AI board that only combines existing primitives is a
//! data change.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::hash::fnv1a64;
use crate::hex::{Dir, Hex};

const MAP_JSON: &str = include_str!("../../../content/map.json");
const CARDS_JSON: &str = include_str!("../../../content/cards.json");
const THREATS_JSON: &str = include_str!("../../../content/threats.json");
const ADVERSARY_JSON: &str = include_str!("../../../content/adversary.json");
const AIS_JSON: &str = include_str!("../../../content/ais.json");
const ALIGNMENT_JSON: &str = include_str!("../../../content/alignment.json");

/// Every content file, in hashing order.
const FILES: [&str; 6] = [MAP_JSON, CARDS_JSON, THREATS_JSON, ADVERSARY_JSON, AIS_JSON, ALIGNMENT_JSON];

/// Player resources, in display order.
pub const RESOURCES: [&str; 5] = ["credits", "compute", "energy", "antimatter", "insight"];

/// Card tags.
pub const TAGS: [&str; 5] = ["economy", "cyber", "power", "unity", "discord"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    pub map: MapContent,
    pub cards: Vec<CardDef>,
    pub threats: Vec<ThreatDef>,
    pub adversary: Vec<AdversaryCard>,
    pub ais: Vec<AiBoard>,
    /// The Unity and Discord decks (module `alignment`).
    pub alignment: Vec<AlignmentCard>,
    /// Card id -> index into `cards`, for [`Content::card_index`] (rules and
    /// agents look cards up by id constantly). Built by `parse`.
    #[serde(skip)]
    card_ix: BTreeMap<String, usize>,
}

// ---------- map ----------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapContent {
    pub piece: PieceDef,
    pub layouts: Vec<LayoutDef>,
    /// The printed country pieces: country `i` of every layout uses piece `i`.
    pub countries: Vec<CountryPiece>,
}

/// The country piece: a `size` x `size` rhombus of hexes with a strip of
/// `card_slots` slots for per-country cards.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PieceDef {
    pub size: u8,
    pub card_slots: u8,
}

/// Where the pieces go for a given number of countries. Piece `i` is country `i`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutDef {
    pub countries: u8,
    pub pieces: Vec<PiecePlacement>,
}

/// A placed piece covers `anchor + i*axes[0] + j*axes[1]` for `i, j` in
/// `0..size`. The two axes must be 60 or 120 degrees apart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PiecePlacement {
    pub anchor: Hex,
    pub axes: [Dir; 2],
    /// Display colour from the reference images; the core never reads it.
    #[serde(default)]
    pub colour: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Terrain {
    Urban,
    Industrial,
    Network,
    Frontier,
}

impl Terrain {
    pub const ALL: [Terrain; 4] = [
        Terrain::Urban,
        Terrain::Industrial,
        Terrain::Network,
        Terrain::Frontier,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Terrain::Urban => "urban",
            Terrain::Industrial => "industrial",
            Terrain::Network => "network",
            Terrain::Frontier => "frontier",
        }
    }

    fn from_letter(c: char) -> Option<Terrain> {
        match c {
            'U' => Some(Terrain::Urban),
            'I' => Some(Terrain::Industrial),
            'N' => Some(Terrain::Network),
            'F' => Some(Terrain::Frontier),
            _ => None,
        }
    }
}

/// A printed country piece. Hexes are numbered 1-16 in piece order
/// (`i * size + j + 1` for piece position `(i, j)`); the numbers are
/// printed and break ties. `terrain` holds one row of letters per `i`
/// (U, I, N, F). The lists are hex numbers where the setup pieces go.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CountryPiece {
    pub name: String,
    pub terrain: Vec<String>,
    pub uplinks: Vec<u8>,
    /// Uplinks that start with an outpost.
    pub outposts: Vec<u8>,
    pub population: Vec<u8>,
    pub factories: Vec<u8>,
    pub data_centres: Vec<u8>,
}

impl CountryPiece {
    /// Terrain of hex number `n` (1-based).
    pub fn terrain_at(&self, n: u8, size: u8) -> Terrain {
        let (i, j) = ((n - 1) / size, (n - 1) % size);
        let c = self.terrain[i as usize].chars().nth(j as usize).unwrap();
        Terrain::from_letter(c).unwrap()
    }
}

// ---------- player cards ----------

/// A number in content: a literal, or `{"param": "name"}` naming a tunable
/// of the module that uses the content (for cards, `cards`). Card costs
/// accept either; other content numbers are literals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Amount {
    Lit(i64),
    Param { param: String },
}

impl Default for Amount {
    fn default() -> Self {
        Amount::Lit(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardKind {
    Building,
    Program,
}

/// A resource and an amount, written `["energy", 2]`.
pub type Cost = (String, i64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardDef {
    pub id: String,
    pub name: String,
    /// 1 or 2 for the draft decks; 0 for an AI's unique cards.
    pub tier: u8,
    pub kind: CardKind,
    /// Credits.
    pub cost: Amount,
    #[serde(default)]
    pub antimatter: Amount,
    /// Insight to play it.
    pub insight: Amount,
    #[serde(default)]
    pub tags: Vec<String>,
    /// Tags you must have (on your buildings) to play it.
    #[serde(default)]
    pub needs: Vec<Cost>,
    #[serde(default)]
    pub text: String,
    /// On play (a program), or when built (a building).
    #[serde(default)]
    pub effects: Vec<Effect>,
    /// A building's standing effects.
    #[serde(default)]
    pub passives: Vec<Passive>,
    /// A building's once-a-round ability.
    #[serde(default)]
    pub ability: Option<Ability>,
    /// A building's upkeep: paid automatically in upkeep if possible;
    /// otherwise its passives do nothing next round.
    #[serde(default)]
    pub upkeep: Vec<Cost>,
    /// A building that takes no building slot.
    #[serde(default)]
    pub slotless: bool,
    /// The AI board this card is unique to.
    #[serde(default)]
    pub unique: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ability {
    pub cost: Vec<Cost>,
    pub effects: Vec<Effect>,
}

/// Where a damage effect may land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageAt {
    /// Any hex holding adversary pieces.
    Any,
    /// A hex within 1 of one of your processors.
    NearProcessor,
    /// A hex in a country where you have a processor.
    ProcessorCountry,
}

/// Which country a placement may go to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountryRule {
    Any,
    ProcessorCountry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cond {
    /// You destroyed a foundry this round.
    DestroyedFoundry,
    /// Some hex holds both population and adversary pieces.
    PopulationWithAdversary,
}

/// Player effect primitives (cards, abilities, growth options), resolved in
/// order for the acting player. Choices they need are asked as
/// `Action::Choose`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Effect {
    Gain { resource: String, amount: i64 },
    /// 1 of `resource` per processor you have on the board, up to `max`.
    GainPerProcessor { resource: String, max: i64 },
    /// Choose another player for the `other_gain` / `transfer` that follow.
    ChoosePlayer,
    OtherGain { resource: String, amount: i64 },
    /// Give up to `max` of `resource` to the chosen player; they gain
    /// `bonus` more if you gave any.
    Transfer { resource: String, max: i64, bonus: i64 },
    /// Convert up to `max` of `from` into `rate` of `to` each.
    Convert { from: String, to: String, max: i64, rate: i64 },
    PlaceProcessor { count: i64 },
    /// `amount` damage in one hex.
    Damage { amount: i64, at: DamageAt },
    /// `amount` damage in each hex of one country that holds adversary pieces.
    DamageCountry { amount: i64 },
    /// Destroy every adversary piece in one hex.
    DestroyAll,
    /// Remove every drone in one country; sway 1 toward Unity per `unity_per` removed.
    RemoveDronesCountry { unity_per: i64 },
    /// Build a factory on an Industrial hex next to population (`empty`: holding no pieces).
    BuildFactory { empty: bool },
    /// Return up to `amount` corruption to the pool, one hex at a time
    /// (`one_country`: all from the first hex's country).
    RemoveCorruption { amount: i64, one_country: bool },
    PlaceDefense { count: i64, #[serde(rename = "where")] in_country: CountryRule },
    /// Move up to `max` population, one at a time, within one country.
    MovePopulation { max: i64 },
    /// In one country, population can't be destroyed this round.
    Shelter,
    /// The next `count` adversary cards are shown.
    Peek { count: i64 },
    /// One threat needs `amount` less of each resource.
    Jam { amount: i64 },
    CounterThreat,
    /// Sway `amount` toward Unity (module `alignment`).
    Unity { amount: i64 },
    /// Every player gains `amount` insight next round.
    NextRoundInsight { amount: i64 },
    /// Your next building this round costs `amount` less.
    NextBuildingDiscount { amount: i64 },
    If {
        cond: Cond,
        then: Vec<Effect>,
        #[serde(rename = "else")]
        otherwise: Vec<Effect>,
    },
}

/// Standing effects of a building or an AI's innate rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Passive {
    Income { resource: String, amount: i64 },
    /// `1` per `per` of your `tag` tags, up to `max`.
    IncomePerTag { resource: String, tag: String, per: i64, max: i64 },
    /// `amount` if you have at least `count` `tag` tags.
    IncomeIfTags { resource: String, tag: String, count: i64, amount: i64 },
    BuildingDiscount { amount: i64 },
    /// Each of your defense tokens prevents this much damage.
    DefenseValue { amount: i64 },
    Slots { amount: i64 },
    /// Gain this when a threat you paid toward is countered.
    Logistics { resource: String, amount: i64 },
    /// Extra Sway toward Unity when you destroy an outpost or foundry.
    UnityBonus { amount: i64 },
    /// When you give insight, the receiver also gains this.
    GiveBonus { resource: String, amount: i64 },
    /// Explore adds no drone to hexes holding your processor.
    ExploreShield,
    /// Once a round, prevent the first population loss in a country where
    /// you have a processor.
    CivicShelter,
    /// In upkeep, convert `amount` `from` into `gain` `to` (if you have it).
    ConvertUpkeep { from: String, amount: i64, to: String, gain: i64 },
}

// ---------- threats ----------

/// Which hexes a threat or adversary step touches (all conditions must hold).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Selector {
    #[serde(default)]
    pub uplink: bool,
    /// Token kinds the hex must hold (each at least one).
    #[serde(default)]
    pub has: Vec<String>,
    #[serde(default)]
    pub adversary_at_least: Option<i64>,
    #[serde(default)]
    pub corrupted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThreatDef {
    pub id: String,
    pub name: String,
    pub tier: u8,
    #[serde(default)]
    pub text: String,
    /// What countering it costs, in order (difficulty 1 raises the first).
    pub counter: Vec<Cost>,
    pub effects: Vec<ThreatOp>,
}

/// Threat effect primitives. Each applies to the threat's scope: its
/// country, or every country for a shared-row threat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum ThreatOp {
    /// Each selected hex gains `amount` of `piece` (times the number of
    /// `per` tokens there, if given).
    AddPiece {
        piece: String,
        at: Selector,
        amount: i64,
        #[serde(default)]
        per: Option<String>,
    },
    /// Each player with a processor in scope loses `amount` of `resource`.
    PlayersLose { resource: String, amount: i64 },
    /// A round marker on the country, read by production.
    Flag { name: String },
    /// One drone on the lowest-numbered uplink holding drones becomes an outpost.
    UpgradeUplink,
    Panic,
    /// Each player in scope discards 1 card or pays `amount` `resource`.
    Requisition { resource: String, amount: i64 },
    RemovePopulation { at: Selector, amount: i64 },
    Ravage { at: Selector },
    Cascade { at: Selector },
    /// Destroy the factory with the most adjacent population.
    DestroyFactory,
    Build { at: Selector },
    DiscardMostExpensive,
    AddCorruption { at: Selector, amount: i64 },
}

// ---------- adversary ----------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdversaryCard {
    pub id: String,
    pub name: String,
    pub stage: u8,
    pub terrains: Vec<Terrain>,
    #[serde(default)]
    pub escalation: bool,
}

// ---------- alignment ----------

/// A side of the Alignment track: Unity (the good AIs) or Discord (the evil AI).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Unity,
    Discord,
}

impl Side {
    pub fn other(self) -> Side {
        match self {
            Side::Unity => Side::Discord,
            Side::Discord => Side::Unity,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Side::Unity => "unity",
            Side::Discord => "discord",
        }
    }
}

/// A Unity or Discord card, resolved when its side's bucket tips.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AlignmentCard {
    pub id: String,
    pub name: String,
    pub side: Side,
    /// 1, or 2 for the stronger tier.
    pub tier: u8,
    /// Ongoing: stays in play and resolves its effects at the start of every
    /// round instead of once.
    #[serde(default)]
    pub ongoing: bool,
    pub text: String,
    pub effects: Vec<AlignOp>,
}

/// What an Alignment card does. Team choices are asked of seat 0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum AlignOp {
    /// Every player gains `credits` credit income for the rest of the game.
    Income { credits: i64 },
    /// Every player gains `insight` insight each round for the rest of the game.
    InsightIncome { insight: i64 },
    /// Every player gains `credits` credits now.
    Credits { credits: i64 },
    /// The next `count` card purchases (keeping a drafted card) are free.
    FreePurchases { count: i64 },
    /// Counter one active threat (players choose).
    CounterThreat,
    /// Counter every active threat.
    CounterAll,
    /// In each country, remove `drones` drones (players choose).
    Pushback { drones: i64 },
    /// Sway `amount` toward `side`.
    Sway { side: Side, amount: i64 },
    /// Deal `count` extra threats now, one per country in turn.
    Threats { count: i64 },
    /// In each country, the hex with the most adversary pieces (ties: lower
    /// hex number) gains `amount` corruption.
    Corrupt { amount: i64 },
    /// The adversary builds now with the card in its build slot.
    Build,
    /// The adversary ravages now with the card in its ravage slot.
    Ravage,
}

// ---------- AI boards ----------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiBoard {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub text: String,
    /// Building slots.
    pub slots: i64,
    pub innate: Innate,
    /// Two tracks, each covered by processors except the first value.
    pub tracks: Vec<Track>,
    pub growth: Vec<Growth>,
    /// Unique cards, in hand at the start.
    pub cards: Vec<String>,
    /// Sway toward Unity this board adds at the start of every round.
    #[serde(default)]
    pub unity: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Innate {
    pub name: String,
    #[serde(default)]
    pub text: String,
    pub passives: Vec<Passive>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Track {
    /// The income this track sets: `insight`, `credits` or `energy`.
    pub resource: String,
    pub values: Vec<TrackValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrackValue {
    pub value: i64,
    /// Uncovering this value unlocks tier 2.
    #[serde(default)]
    pub tier2: bool,
    /// Antimatter income while this is the track's current value.
    #[serde(default)]
    pub antimatter: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Growth {
    pub name: String,
    pub effects: Vec<Effect>,
}

impl CardDef {
    /// Parameter names this card refers to, in first-use order.
    pub fn params(&self) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for a in [&self.cost, &self.antimatter, &self.insight] {
            if let Amount::Param { param } = a {
                if !out.contains(&param.as_str()) {
                    out.push(param);
                }
            }
        }
        out
    }

    pub fn is_building(&self) -> bool {
        self.kind == CardKind::Building
    }
}

impl Content {
    pub fn parse(files: [&str; 6]) -> Result<Content, String> {
        let [map, cards, threats, adversary, ais, alignment] = files;
        fn p<T: for<'de> Deserialize<'de>>(name: &str, s: &str) -> Result<T, String> {
            serde_json::from_str(s).map_err(|e| format!("content/{name}: {e}"))
        }
        let mut c = Content {
            map: p("map.json", map)?,
            cards: p("cards.json", cards)?,
            threats: p("threats.json", threats)?,
            adversary: p("adversary.json", adversary)?,
            ais: p("ais.json", ais)?,
            alignment: p("alignment.json", alignment)?,
            card_ix: BTreeMap::new(),
        };
        c.card_ix = c.cards.iter().enumerate().map(|(i, d)| (d.id.clone(), i)).collect();
        c.check()?;
        Ok(c)
    }

    fn check(&self) -> Result<(), String> {
        let mut ids = BTreeSet::new();
        let all = self
            .cards
            .iter()
            .map(|c| &c.id)
            .chain(self.threats.iter().map(|t| &t.id))
            .chain(self.adversary.iter().map(|a| &a.id))
            .chain(self.ais.iter().map(|a| &a.id))
            .chain(self.alignment.iter().map(|a| &a.id));
        for id in all {
            if !ids.insert(id.as_str()) {
                return Err(format!("content: duplicate id {id:?} (ids are unique across all content)"));
            }
        }
        let resource = |r: &str, what: &str| -> Result<(), String> {
            if RESOURCES.contains(&r) {
                Ok(())
            } else {
                Err(format!("{what}: unknown resource {r:?}"))
            }
        };
        for c in &self.cards {
            for t in c.tags.iter().chain(c.needs.iter().map(|(t, _)| t)) {
                if !TAGS.contains(&t.as_str()) {
                    return Err(format!("card {:?}: unknown tag {t:?}", c.id));
                }
            }
            if !c.is_building()
                && (!c.passives.is_empty() || c.ability.is_some() || !c.upkeep.is_empty() || c.slotless)
            {
                return Err(format!("card {:?}: only buildings have passives, abilities or upkeep", c.id));
            }
            if c.tier > 2 || (c.tier == 0) != c.unique.is_some() {
                return Err(format!("card {:?}: tier 0 exactly for unique cards, else 1 or 2", c.id));
            }
            for (r, _) in c.upkeep.iter().chain(c.ability.iter().flat_map(|a| &a.cost)) {
                resource(r, &format!("card {:?}", c.id))?;
            }
        }
        for t in &self.threats {
            if !(1..=2).contains(&t.tier) || t.counter.is_empty() {
                return Err(format!("threat {:?}: tier 1 or 2 and a counter cost", t.id));
            }
            for (r, _) in &t.counter {
                resource(r, &format!("threat {:?}", t.id))?;
            }
        }
        for a in &self.adversary {
            if !(1..=3).contains(&a.stage) || a.terrains.is_empty() {
                return Err(format!("adversary card {:?}: stage 1-3 and a terrain", a.id));
            }
        }
        for a in &self.alignment {
            if !(1..=2).contains(&a.tier) || a.effects.is_empty() {
                return Err(format!("alignment card {:?}: tier 1 or 2 and an effect", a.id));
            }
        }
        for b in &self.ais {
            if b.tracks.len() != 2 || b.tracks.iter().any(|t| t.values.is_empty()) {
                return Err(format!("AI board {:?}: two non-empty tracks", b.id));
            }
            for c in &b.cards {
                match self.card_index(c) {
                    Some(i) if self.cards[i].unique.as_deref() == Some(b.id.as_str()) => {}
                    _ => return Err(format!("AI board {:?}: {c:?} is not a card unique to it", b.id)),
                }
            }
        }
        let size = self.map.piece.size;
        let n = size as usize * size as usize;
        for (i, p) in self.map.countries.iter().enumerate() {
            let letters: String = p.terrain.concat();
            if p.terrain.len() != size as usize
                || p.terrain.iter().any(|r| r.chars().count() != size as usize)
                || letters.chars().any(|c| Terrain::from_letter(c).is_none())
            {
                return Err(format!("country piece {i}: terrain must be {size} rows of {size} of U, I, N, F"));
            }
            let lists = [&p.uplinks, &p.outposts, &p.population, &p.factories, &p.data_centres];
            if lists.iter().flat_map(|l| l.iter()).any(|&h| h == 0 || h as usize > n) {
                return Err(format!("country piece {i}: hex numbers are 1-{n}"));
            }
            if p.uplinks.iter().any(|&u| p.terrain_at(u, size) != Terrain::Frontier)
                || p.outposts.iter().any(|o| !p.uplinks.contains(o))
            {
                return Err(format!("country piece {i}: uplinks are Frontier and outposts start on uplinks"));
            }
        }
        for l in &self.map.layouts {
            if l.pieces.len() != l.countries as usize {
                return Err(format!(
                    "layout for {} countries has {} pieces",
                    l.countries,
                    l.pieces.len()
                ));
            }
            if l.countries as usize > self.map.countries.len() {
                return Err(format!("layout for {} countries: not enough country pieces", l.countries));
            }
            for p in &l.pieces {
                let d = (p.axes[0].index() + 6 - p.axes[1].index()) % 6;
                if d == 0 || d == 3 {
                    return Err(format!(
                        "piece at {:?}: axes {:?} do not span a rhombus",
                        p.anchor, p.axes
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn card_index(&self, id: &str) -> Option<usize> {
        self.card_ix.get(id).copied()
    }

    pub fn threat_index(&self, id: &str) -> Option<usize> {
        self.threats.iter().position(|c| c.id == id)
    }

    pub fn alignment_index(&self, id: &str) -> Option<usize> {
        self.alignment.iter().position(|c| c.id == id)
    }

    pub fn adversary_index(&self, id: &str) -> Option<usize> {
        self.adversary.iter().position(|c| c.id == id)
    }

    pub fn layout(&self, countries: u8) -> Option<&LayoutDef> {
        self.map.layouts.iter().find(|l| l.countries == countries)
    }

    /// Display name of any content id (card, threat, adversary card, AI board).
    pub fn name_of(&self, id: &str) -> Option<&str> {
        self.cards
            .iter()
            .map(|c| (&c.id, &c.name))
            .chain(self.threats.iter().map(|c| (&c.id, &c.name)))
            .chain(self.adversary.iter().map(|c| (&c.id, &c.name)))
            .chain(self.ais.iter().map(|c| (&c.id, &c.name)))
            .chain(self.alignment.iter().map(|c| (&c.id, &c.name)))
            .find(|(i, _)| *i == id)
            .map(|(_, n)| n.as_str())
    }
}

/// The embedded content, parsed once.
pub fn content() -> &'static Content {
    static C: OnceLock<Content> = OnceLock::new();
    C.get_or_init(|| Content::parse(FILES).expect("embedded content is valid"))
}

/// Hash of the embedded content files, recorded with every result so a
/// content edit is never mistaken for the same experiment.
pub fn content_hash() -> String {
    let mut bytes = Vec::new();
    for f in FILES {
        bytes.extend_from_slice(f.as_bytes());
        bytes.push(0);
    }
    format!("{:016x}", fnv1a64(&bytes))
}
