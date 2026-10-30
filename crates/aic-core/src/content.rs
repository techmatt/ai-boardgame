//! Game content as data, embedded from `content/` at compile time (no I/O
//! at runtime): the map (piece shape, layouts, the printed country pieces),
//! player cards, threats, the adversary deck, the AI boards and the
//! Alignment decks.
//!
//! Effects are composed from code primitives ([`Effect`], [`Passive`],
//! [`ThreatOp`], [`Twist`], [`AlignOp`]). A new primitive is a code change;
//! a new card, threat, adversary card, AI board or Alignment card that only
//! combines existing primitives is a data change.

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
const PROJECTS_JSON: &str = include_str!("../../../content/projects.json");

/// Every content file, in hashing order.
const FILES: [&str; 7] = [
    MAP_JSON,
    CARDS_JSON,
    THREATS_JSON,
    ADVERSARY_JSON,
    AIS_JSON,
    ALIGNMENT_JSON,
    PROJECTS_JSON,
];

/// Player resources, in display order.
pub const RESOURCES: [&str; 5] = ["credits", "compute", "energy", "antimatter", "insight"];

/// Positions in [`RESOURCES`], the order of income and production arrays.
pub const CREDITS: usize = 0;
pub const COMPUTE: usize = 1;
pub const ENERGY: usize = 2;
pub const ANTIMATTER: usize = 3;
pub const INSIGHT: usize = 4;

// The positions above name the right resources (checked at compile time).
const _: () = {
    assert!(str_eq(RESOURCES[CREDITS], "credits"));
    assert!(str_eq(RESOURCES[COMPUTE], "compute"));
    assert!(str_eq(RESOURCES[ENERGY], "energy"));
    assert!(str_eq(RESOURCES[ANTIMATTER], "antimatter"));
    assert!(str_eq(RESOURCES[INSIGHT], "insight"));
};

/// String equality usable in a const context.
const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

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
    /// The adversary's project deck (module `adversary`, 017).
    pub projects: Vec<ProjectCard>,
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

/// The country piece: a `size` x `size` rhombus of hexes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PieceDef {
    pub size: u8,
}

/// Where the pieces go for a given number of countries. Piece `i` is country `i`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutDef {
    pub countries: u8,
    pub pieces: Vec<PiecePlacement>,
    /// The fronts, printed on the tiles: pairs of countries sharing a long
    /// edge. Pieces touching only at the middle of the map are not fronts.
    #[serde(default)]
    pub fronts: Vec<[u8; 2]>,
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

/// What a hex is. Cities, open land and ocean are printed on the piece; a
/// military base goes on an open hex of each land border (see [`crate::Map`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HexKind {
    City,
    Base,
    Open,
    Ocean,
}

impl HexKind {
    pub fn name(self) -> &'static str {
        match self {
            HexKind::City => "city",
            HexKind::Base => "base",
            HexKind::Open => "open",
            HexKind::Ocean => "ocean",
        }
    }
}

/// A printed country piece. Hexes are numbered 1-16 in piece order
/// (`i * size + j + 1` for piece position `(i, j)`); the numbers are
/// printed and break ties. The lists are hex numbers: printed cities and
/// ocean, and where the setup pieces go. Every other hex is open land.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CountryPiece {
    pub name: String,
    pub cities: Vec<u8>,
    /// The order cities start corrupted in (an adversary cube): setup
    /// corrupts the first `board.start_corrupted`, difficulty 4
    /// `adversary.extra_corrupted` more.
    pub corrupted: Vec<u8>,
    pub ocean: Vec<u8>,
    /// Research stations at setup (they start corrupted).
    pub stations: Vec<u8>,
    /// Printed industrial zones: open hexes next to a city where factories go.
    pub industry: Vec<u8>,
    /// Industrial zones holding a factory at setup (neutral).
    pub factories: Vec<u8>,
    /// Where the starting presence of the player who starts here goes.
    pub presence: u8,
    /// The corp's megaproject card.
    pub project: Project,
}

/// A megaproject card: research stations fill it with cubes; when it holds
/// `cubes`, it completes and every cube pays its owner. After that, each
/// owned station in the country produces `benefit` for its owner every round.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    pub cubes: i64,
    #[serde(default)]
    pub text: String,
    pub benefit: Vec<Cost>,
}

impl CountryPiece {
    /// The printed kind of hex number `n` (bases are placed, not printed).
    pub fn kind_at(&self, n: u8) -> HexKind {
        if self.cities.contains(&n) {
            HexKind::City
        } else if self.ocean.contains(&n) {
            HexKind::Ocean
        } else {
            HexKind::Open
        }
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

/// A tag and a count, written `["power", 2]`.
pub type TagCount = (String, i64);

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
    pub needs: Vec<TagCount>,
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

/// Where a removal effect may land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageAt {
    /// Any hex holding adversary ambassadors or troops.
    Any,
    /// A hex within your reach (1 of one of your presence).
    Reach,
    /// A hex in a country where you have a presence.
    PresenceCountry,
}

/// Which country a placement may go to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountryRule {
    Any,
    PresenceCountry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cond {
    /// You removed an adversary troop this round.
    RemovedTroop,
    /// Some clean city holds adversary ambassadors.
    CityWithAmbassador,
}

/// Player effect primitives (cards, abilities, growth options), resolved in
/// order for the acting player. Choices they need are asked as
/// `Action::Choose`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Effect {
    Gain {
        resource: String,
        amount: i64,
    },
    /// 1 of `resource` per presence you have on the board, up to `max`.
    GainPerPresence {
        resource: String,
        max: i64,
    },
    /// Choose another player for the `other_gain` / `transfer` that follow.
    ChoosePlayer,
    OtherGain {
        resource: String,
        amount: i64,
    },
    /// Give up to `max` of `resource` to the chosen player; they gain
    /// `bonus` more if you gave any.
    Transfer {
        resource: String,
        max: i64,
        bonus: i64,
    },
    /// Convert up to `max` of `from` into `rate` of `to` each.
    Convert {
        from: String,
        to: String,
        max: i64,
        rate: i64,
    },
    PlacePresence {
        count: i64,
    },
    /// Remove adversary pieces in one hex, `amount` worth: an ambassador
    /// costs 1, a troop `board.troop_health`.
    Damage {
        amount: i64,
        at: DamageAt,
    },
    /// `amount` worth of removal in each hex of one country holding adversary pieces.
    DamageCountry {
        amount: i64,
    },
    /// Remove every adversary ambassador and troop in one hex.
    DestroyAll,
    /// Remove one node within your reach (019: the tier-2 way to remove a node).
    RemoveNode,
    /// Remove every adversary ambassador in one country; sway 1 toward Unity per `unity_per` removed.
    RemoveAmbassadorsCountry {
        unity_per: i64,
    },
    /// Build a factory on an empty open hex touching a city.
    BuildFactory,
    /// Take the adversary cube off up to `amount` corrupted cities or
    /// stations, one at a time (`one_country`: all in the first one's country).
    Cleanse {
        amount: i64,
        one_country: bool,
    },
    /// Put `count` hardening tokens on buildings in one country.
    Harden {
        count: i64,
        #[serde(rename = "where")]
        in_country: CountryRule,
    },
    /// Move up to `max` of your ambassadors (from your supply or the board)
    /// onto pieces within your reach, one at a time.
    MoveAmbassadors {
        max: i64,
    },
    /// Gain `count` player ambassadors into your supply.
    GainAmbassadors {
        count: i64,
    },
    /// In one country, buildings can't be destroyed this round.
    Shelter,
    /// The next `count` adversary cards are shown.
    Peek {
        count: i64,
    },
    /// One threat needs `amount` less of each resource.
    Jam {
        amount: i64,
    },
    CounterThreat,
    /// Remove up to `amount` compute from one visible adversary project
    /// (017; the Security cards' primitive).
    DrainProject {
        amount: i64,
    },
    /// Discard one visible adversary project, with its compute (017).
    DiscardProject,
    /// Sway `amount` toward Unity (module `alignment`).
    Unity {
        amount: i64,
    },
    /// Every player gains `amount` insight next round.
    NextRoundInsight {
        amount: i64,
    },
    /// Your next building this round costs `amount` less.
    NextBuildingDiscount {
        amount: i64,
    },
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
    Income {
        resource: String,
        amount: i64,
    },
    /// `1` per `per` of your `tag` tags, up to `max`.
    IncomePerTag {
        resource: String,
        tag: String,
        per: i64,
        max: i64,
    },
    /// `amount` if you have at least `count` `tag` tags.
    IncomeIfTags {
        resource: String,
        tag: String,
        count: i64,
        amount: i64,
    },
    BuildingDiscount {
        amount: i64,
    },
    /// You may use the Harden standard action.
    HardenAction,
    Slots {
        amount: i64,
    },
    /// Gain this when a threat you paid toward is countered.
    Logistics {
        resource: String,
        amount: i64,
    },
    /// Extra Sway toward Unity when you remove an adversary troop.
    UnityBonus {
        amount: i64,
    },
    /// When you give insight, the receiver also gains this.
    GiveBonus {
        resource: String,
        amount: i64,
    },
    /// The adversary's spread places no ambassador on hexes holding your presence.
    SpreadShield,
    /// Once a round, prevent the first building destruction in a country
    /// where you have a presence.
    CivicShelter,
    /// In upkeep, convert `amount` `from` into `gain` `to` (if you have it).
    ConvertUpkeep {
        from: String,
        amount: i64,
        to: String,
        gain: i64,
    },
}

// ---------- threats ----------

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

/// Pieces a threat places adversary ambassadors on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Spot {
    /// Player-owned pieces: presence, aligned cities, owned stations and factories.
    PlayerBuildings,
    CleanCities,
    Factories,
    Bases,
}

/// Threat effect primitives. Each applies to the threat's scope: its
/// country, or every country for a shared-row threat. Team choices are
/// asked of seat 0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum ThreatOp {
    /// `count` adversary ambassadors from the supply, each onto a piece of
    /// kind `at` in scope (players choose each).
    Ambassadors {
        at: Spot,
        count: i64,
    },
    /// `count` adversary troops, each onto a factory in scope (players
    /// choose), or with no factory onto a city.
    Troop {
        count: i64,
    },
    /// One research station in scope that is not corrupted becomes
    /// corrupted (players choose).
    FlipStation,
    /// The clean city in scope with the most adversary ambassadors (ties:
    /// lower hex number) becomes corrupted.
    CorruptCity,
    /// Each player with a presence in scope loses `amount` of `resource`.
    PlayersLose {
        resource: String,
        amount: i64,
    },
    /// A round marker on the country, read by production.
    Flag {
        name: RoundFlag,
    },
    /// Each player in scope discards `cards` cards or pays `amount`
    /// `resource` (their choice).
    Requisition {
        resource: String,
        amount: i64,
        #[serde(default = "one")]
        cards: i64,
    },
    /// Each player in scope loses a player ambassador (from their supply,
    /// else one of theirs on the board).
    LoseAmbassador,
    /// Sway `amount` toward Discord.
    Discord {
        amount: i64,
    },
    /// The adversary builds now, in the threat's scope (its country, or everywhere on the shared row).
    Build,
    DiscardMostExpensive,
    /// The adversary gains `amount` compute (onto its projects, 017).
    Compute {
        amount: i64,
    },
}

fn one() -> i64 {
    1
}

/// A country marker that lasts until the end of the round (module `board`):
/// set by threats and cards, cleared in the board's cleanup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundFlag {
    /// Cities in the country make no energy.
    Blackout,
    /// Data centres in the country make no compute.
    Sabotage,
    /// Buildings in the country can't be destroyed.
    Shelter,
}

impl RoundFlag {
    pub fn name(self) -> &'static str {
        match self {
            RoundFlag::Blackout => "blackout",
            RoundFlag::Sabotage => "sabotage",
            RoundFlag::Shelter => "shelter",
        }
    }
}

// ---------- adversary ----------

/// An adversary deck card: the clock and the stager. Its twists change
/// the round it is the current card of.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdversaryCard {
    pub id: String,
    pub name: String,
    pub stage: u8,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub twists: Vec<Twist>,
    /// The Militarist's escalation: each military base gains an adversary
    /// ambassador when this card's round builds.
    #[serde(default)]
    pub escalation: bool,
}

/// What an adversary card changes about its round.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Twist {
    /// Each corrupted city builds `amount` more.
    CityBuild { amount: i64 },
    /// Factories next to clean cities build `amount` too.
    CleanFactories { amount: i64 },
    /// Each corrupted research station builds `amount`.
    Stations { amount: i64 },
    /// Spread reaches `amount` hexes further.
    Reach { amount: i64 },
    /// Clean cities corrupt at `amount` fewer ambassadors.
    CorruptSooner { amount: i64 },
    /// Sway `amount` toward Discord.
    Discord { amount: i64 },
    /// This round is a war round.
    War,
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
    pub text: String,
    pub effects: Vec<AlignOp>,
    /// Printed strength steps (017): from `at` tips of the card's side
    /// (counted before this tip), `effects` replace the card's effects; the
    /// last step reached applies.
    #[serde(default)]
    pub steps: Vec<Step>,
}

/// A strength step printed on an Alignment card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub at: i64,
    pub effects: Vec<AlignOp>,
}

/// An adversary project (017): one shared deck, staged (stage 1 cheap,
/// stage 2 big). Adversary compute fills the cheapest visible project; a
/// full one resolves its effects once, or, `ongoing`, every round from then
/// on. Effects are Alignment primitives; team choices are asked of seat 0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectCard {
    pub id: String,
    pub name: String,
    pub stage: u8,
    /// Compute to fill it, per player.
    pub cost: i64,
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
    /// In each country, remove `ambassadors` adversary ambassadors (players choose).
    Pushback { ambassadors: i64 },
    /// Sway `amount` toward `side`.
    Sway { side: Side, amount: i64 },
    /// Deal `count` extra threats now, one per country in turn.
    Threats { count: i64 },
    /// In each country, the clean city with the most adversary ambassadors
    /// (ties: lower hex number) gains `amount` more.
    Infiltrate { amount: i64 },
    /// The adversary builds now.
    Build,
    /// The adversary spreads now.
    Spread,
    /// Every player sacrifices one of their ambassadors (supply or board)
    /// or one of their presence tokens (their choice).
    Sacrifice,
    /// Every node makes `amount` adversary ambassadors on its hex.
    NodeAmbassadors { amount: i64 },
    /// Every clean city gains `amount` adversary ambassadors.
    CleanCityAmbassadors { amount: i64 },
    /// In each country, `count` adversary troops onto a factory (players
    /// choose; with no factory, onto a city).
    Troops { count: i64 },
    /// In each country, one research station that is not corrupted becomes
    /// corrupted (players choose).
    FlipStations,
    /// Every player loses `amount` of `resource` (as much as they have).
    PlayersLose { resource: String, amount: i64 },
    /// Remove `count` nodes anywhere on the board, one at a time (players choose).
    RemoveNodes { count: i64 },
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
    /// Credit production every round, before cards raise it.
    pub credits: i64,
    /// Resources this AI may spend as credits on some cards.
    #[serde(default)]
    pub discounts: Vec<Discount>,
    /// Income every round before the presence track adds to it.
    #[serde(default)]
    pub income: Vec<Cost>,
    /// The presence track: one space per presence token. The first token
    /// starts on the board, so the first space is uncovered from setup.
    pub presence: Vec<Uncover>,
    pub growth: Vec<Growth>,
    /// Unique cards, in hand at the start.
    pub cards: Vec<String>,
    /// Sway toward Unity this board adds at the start of every round.
    #[serde(default)]
    pub unity: i64,
}

/// Spend `resource` for `rate` credits each off a card's credit cost: on
/// cards tagged `tag`, or on any building for `tag` "building".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Discount {
    pub resource: String,
    pub tag: String,
    pub rate: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Innate {
    pub name: String,
    #[serde(default)]
    pub text: String,
    pub passives: Vec<Passive>,
}

/// A presence-track space: what placing its token uncovers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Uncover {
    /// Income every round once uncovered (insight raises the round's insight).
    #[serde(default)]
    pub income: Vec<Cost>,
    /// Uncovering this space unlocks tier 2.
    #[serde(default)]
    pub tier2: bool,
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
    pub fn parse(files: [&str; 7]) -> Result<Content, String> {
        let [map, cards, threats, adversary, ais, alignment, projects] = files;
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
            projects: p("projects.json", projects)?,
            card_ix: BTreeMap::new(),
        };
        c.card_ix = c
            .cards
            .iter()
            .enumerate()
            .map(|(i, d)| (d.id.clone(), i))
            .collect();
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
            .chain(self.alignment.iter().map(|a| &a.id))
            .chain(self.projects.iter().map(|a| &a.id));
        for id in all {
            if !ids.insert(id.as_str()) {
                return Err(format!(
                    "content: duplicate id {id:?} (ids are unique across all content)"
                ));
            }
        }
        for c in &self.cards {
            let what = format!("card {:?}", c.id);
            for t in c.tags.iter().chain(c.needs.iter().map(|(t, _)| t)) {
                tag(t, &what)?;
            }
            check_effects(&c.effects, &what)?;
            check_passives(&c.passives, &what)?;
            if let Some(a) = &c.ability {
                check_effects(&a.effects, &what)?;
            }
            if !c.is_building()
                && (!c.passives.is_empty()
                    || c.ability.is_some()
                    || !c.upkeep.is_empty()
                    || c.slotless)
            {
                return Err(format!(
                    "card {:?}: only buildings have passives, abilities or upkeep",
                    c.id
                ));
            }
            if c.tier > 2 || (c.tier == 0) != c.unique.is_some() {
                return Err(format!(
                    "card {:?}: tier 0 exactly for unique cards, else 1 or 2",
                    c.id
                ));
            }
            for (r, _) in c
                .upkeep
                .iter()
                .chain(c.ability.iter().flat_map(|a| &a.cost))
            {
                resource(r, &what)?;
            }
        }
        for t in &self.threats {
            if !(1..=2).contains(&t.tier) || t.counter.is_empty() {
                return Err(format!("threat {:?}: tier 1 or 2 and a counter cost", t.id));
            }
            let what = format!("threat {:?}", t.id);
            for (r, _) in &t.counter {
                resource(r, &what)?;
            }
            for op in &t.effects {
                match op {
                    ThreatOp::PlayersLose { resource: r, .. }
                    | ThreatOp::Requisition { resource: r, .. } => resource(r, &what)?,
                    _ => {}
                }
            }
        }
        for a in &self.adversary {
            if !(1..=3).contains(&a.stage) {
                return Err(format!("adversary card {:?}: stage 1-3", a.id));
            }
        }
        for a in &self.alignment {
            if !(1..=2).contains(&a.tier) || a.effects.is_empty() {
                return Err(format!(
                    "alignment card {:?}: tier 1 or 2 and an effect",
                    a.id
                ));
            }
            if a.steps.windows(2).any(|p| p[0].at >= p[1].at) || a.steps.iter().any(|s| s.at < 1) {
                return Err(format!(
                    "alignment card {:?}: steps at 1 or more tips, in rising order",
                    a.id
                ));
            }
            for op in a
                .effects
                .iter()
                .chain(a.steps.iter().flat_map(|s| &s.effects))
            {
                if let AlignOp::PlayersLose { resource: r, .. } = op {
                    resource(r, &format!("alignment card {:?}", a.id))?;
                }
            }
        }
        for p in &self.projects {
            if !(1..=2).contains(&p.stage) || p.cost < 1 || p.effects.is_empty() {
                return Err(format!(
                    "project {:?}: stage 1 or 2, a cost of 1 or more and an effect",
                    p.id
                ));
            }
            for op in &p.effects {
                if let AlignOp::PlayersLose { resource: r, .. } = op {
                    resource(r, &format!("project {:?}", p.id))?;
                }
            }
        }
        for b in &self.ais {
            if b.presence.is_empty() {
                return Err(format!("AI board {:?}: a presence track", b.id));
            }
            for c in &b.cards {
                match self.card_index(c) {
                    Some(i) if self.cards[i].unique.as_deref() == Some(b.id.as_str()) => {}
                    _ => {
                        return Err(format!(
                            "AI board {:?}: {c:?} is not a card unique to it",
                            b.id
                        ))
                    }
                }
            }
            let what = format!("AI board {:?}", b.id);
            for (r, _) in b
                .income
                .iter()
                .chain(b.presence.iter().flat_map(|u| u.income.iter()))
            {
                resource(r, &what)?;
            }
            for d in &b.discounts {
                resource(&d.resource, &what)?;
                if d.tag != "building" {
                    tag(&d.tag, &what)?;
                }
            }
            for gr in &b.growth {
                check_effects(&gr.effects, &what)?;
            }
            check_passives(&b.innate.passives, &what)?;
        }
        let size = self.map.piece.size;
        let n = size as usize * size as usize;
        for (i, p) in self.map.countries.iter().enumerate() {
            let start = vec![p.presence];
            let lists = [
                &p.cities,
                &p.corrupted,
                &p.ocean,
                &p.stations,
                &p.industry,
                &start,
            ];
            if lists
                .iter()
                .flat_map(|l| l.iter())
                .any(|&h| h == 0 || h as usize > n)
            {
                return Err(format!("country piece {i}: hex numbers are 1-{n}"));
            }
            if p.corrupted.iter().any(|c| !p.cities.contains(c)) {
                return Err(format!(
                    "country piece {i}: corrupted cities must be cities"
                ));
            }
            if p.factories.iter().any(|f| !p.industry.contains(f)) {
                return Err(format!(
                    "country piece {i}: setup factories go on industrial zones"
                ));
            }
            let built: Vec<u8> = [&p.stations, &p.industry, &start]
                .iter()
                .flat_map(|l| l.iter().copied())
                .collect();
            for (k, &h) in built.iter().enumerate() {
                if p.kind_at(h) != HexKind::Open || built[..k].contains(&h) {
                    return Err(format!(
                        "country piece {i}: buildings go on open hexes, one per hex (#{h})"
                    ));
                }
            }
            for (r, _) in &p.project.benefit {
                resource(r, &format!("project of country piece {i}"))?;
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
                return Err(format!(
                    "layout for {} countries: not enough country pieces",
                    l.countries
                ));
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

    pub fn project_index(&self, id: &str) -> Option<usize> {
        self.projects.iter().position(|c| c.id == id)
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

    /// Display name of any content id (card, threat, adversary card, AI
    /// board, Alignment card).
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

fn resource(r: &str, what: &str) -> Result<(), String> {
    if RESOURCES.contains(&r) {
        Ok(())
    } else {
        Err(format!("{what}: unknown resource {r:?}"))
    }
}

/// A resource an effect may give or move: a player resource, or the seat's
/// ambassador supply (`modules::common::AMBASSADORS`), which effects reach
/// through the same `add_resource`.
fn seat_resource(r: &str, what: &str) -> Result<(), String> {
    if r == "ambassadors" {
        Ok(())
    } else {
        resource(r, what)
    }
}

fn tag(t: &str, what: &str) -> Result<(), String> {
    if TAGS.contains(&t) {
        Ok(())
    } else {
        Err(format!("{what}: unknown tag {t:?}"))
    }
}

/// Checks the resource names in `effects`, `if` branches included.
fn check_effects(effects: &[Effect], what: &str) -> Result<(), String> {
    for e in effects {
        match e {
            Effect::Gain { resource: r, .. }
            | Effect::GainPerPresence { resource: r, .. }
            | Effect::OtherGain { resource: r, .. }
            | Effect::Transfer { resource: r, .. } => seat_resource(r, what)?,
            Effect::Convert { from, to, .. } => {
                resource(from, what)?;
                resource(to, what)?;
            }
            Effect::If {
                then, otherwise, ..
            } => {
                check_effects(then, what)?;
                check_effects(otherwise, what)?;
            }
            _ => {}
        }
    }
    Ok(())
}

/// Checks the resource and tag names in `passives`.
fn check_passives(passives: &[Passive], what: &str) -> Result<(), String> {
    for p in passives {
        match p {
            Passive::Income { resource: r, .. }
            | Passive::Logistics { resource: r, .. }
            | Passive::GiveBonus { resource: r, .. } => resource(r, what)?,
            Passive::IncomePerTag {
                resource: r,
                tag: t,
                ..
            }
            | Passive::IncomeIfTags {
                resource: r,
                tag: t,
                ..
            } => {
                resource(r, what)?;
                tag(t, what)?;
            }
            Passive::ConvertUpkeep { from, to, .. } => {
                resource(from, what)?;
                resource(to, what)?;
            }
            _ => {}
        }
    }
    Ok(())
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
