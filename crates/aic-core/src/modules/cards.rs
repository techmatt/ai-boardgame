//! Cards and draft (021): the tier-1 and tier-2 decks (one copy of each
//! card; discards reshuffle when a deck runs out), the opening deal, the
//! Terraforming Mars-style draft with the tier chosen card by card, keeping
//! drafted cards for insight, playing buildings (slots that stack and splay
//! down), programs (they stay in your play area) and events, token engines,
//! building abilities and upkeep, and the sacrifices that pay AI-board powers.

use std::collections::BTreeMap;

use crate::action::Action;
use crate::content::{CardDef, DiscountOn, Op, Tag, TagOrAny, TokenKind, TAGS};
use crate::key::{Key, KeyBuf};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{ai, alignment, economy, effects};

const DRAFT_PARAMS: &[ParamSpec] = &[
    ParamSpec::int("deal", 0, 10, "cards each player drafts from each round after the first, choosing each one's tier"),
    ParamSpec::int("opening_deal", 0, 20, "cards dealt to each player in round 1 (the opening)"),
    ParamSpec::int("opening_free", 0, 20, "opening cards each player keeps free"),
    ParamSpec::int("opening_keep_credits", 0, 20, "credits for each opening card kept beyond the free ones"),
    ParamSpec::int("keep_tier1", 0, 10, "insight to keep a drafted tier-1 card"),
    ParamSpec::int("keep_tier2", 0, 10, "insight to keep a drafted tier-2 card"),
    ParamSpec::int("keep_tier2_upgraded", 0, 10, "insight to keep a tier-2 card once the AI board's keep upgrade is uncovered"),
    ParamSpec::int("insight_credits", 1, 20, "credits for 1 insight, usable only for keep costs"),
    ParamSpec::int("science_every", 0, 10, "1 insight off tier-2 keeps and card plays per this many Science tags (0: off)"),
    ParamSpec::int("industry_every", 0, 10, "1 credit off buildings per this many Industry tags (0: off)"),
    ParamSpec::int("discount_floor", 0, 5, "tag and building discounts stop at this cost; a cost printed under it stays"),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "cards",
    summary: "Card decks, the draft, keeping cards, playing buildings, programs and events",
    requires: &["ai", "adversary", "alignment", "board"],
    variants: &[VariantDef {
        name: "draft",
        summary: "Opening deal of 8 (4 kept free); then each round a Terraforming Mars draft, the tier chosen card by card; keeps cost insight",
        params: DRAFT_PARAMS,
        phases: PHASES,
        rng: &["cards"],
        uses_cards: true,
        build,
    }, VariantDef {
        name: "off",
        summary: "No draft: players have only their AI's unique cards",
        params: &[],
        phases: PHASES,
        rng: &["cards"],
        uses_cards: true,
        build: build_off,
    }],
};

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::decision(
        "tier",
        "each player builds their draft hand card by card, choosing each card's tier",
    ),
    PhaseSpec::auto("deal", "round 1: deal the opening hand"),
    PhaseSpec::decision(
        "draft",
        "take one card and pass the rest to the next seat, until the hands are gone",
    ),
    PhaseSpec::decision(
        "keep",
        "keep any drafted cards (insight; the opening: free, then credits); the rest are discarded",
    ),
    PhaseSpec::decision(
        "play",
        "play cards, use abilities",
    ),
];

pub fn hand(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.hand.").num(seat)
}

/// Seat `s`'s active buildings: the top card of each slot's stack, and
/// slotless buildings.
pub fn built(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.built.").num(seat)
}

/// The cards splayed under seat `s`'s active building `top`, bottom first:
/// their effects are off, their tags still count.
pub fn under(seat: usize, top: &str) -> KeyBuf {
    KeyBuf::new("cards.under.").num(seat).text(".").text(top)
}

/// Seat `s`'s play area: programs played (their tags count).
pub fn played(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.played.").num(seat)
}

/// Tokens on seat `s`'s card `card` (021: tokens live only on cards).
pub fn tokens_key(seat: usize, card: &str) -> KeyBuf {
    KeyBuf::new("cards.tokens.").num(seat).text(".").text(card)
}

/// Prefix of every seat's draft packet pile.
pub const PACKETS: &str = "cards.packet.";

/// The draft hand in front of seat `s`.
pub fn packet(seat: usize) -> KeyBuf {
    KeyBuf::new(PACKETS).num(seat)
}

/// Cards seat `s` has drafted this round and not yet kept.
pub fn drafted(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.drafted.").num(seat)
}

fn deck(tier: u8) -> String {
    format!("cards.deck.{tier}")
}

fn discard_pile(tier: u8) -> String {
    format!("cards.discard.{tier}")
}

fn picked_key(seat: usize) -> String {
    format!("cards.picked.{seat}")
}

/// Credits seat `s`'s next building this round costs less (a one-off discount).
pub fn next_discount(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.next_discount.").num(seat)
}

/// Set once seat `s`'s Aegis Dome Array has saved a building this round (module `board`).
pub fn civic_used(seat: usize) -> String {
    format!("cards.civic_used.{seat}")
}

fn unpowered(seat: usize, card: &str) -> KeyBuf {
    KeyBuf::new("cards.unpowered.").num(seat).text(".").text(card)
}

/// Drafted cards `seat` has kept this round.
pub fn kept_key(seat: usize) -> String {
    format!("cards.kept_round.{seat}")
}

/// The rule for keeping one more card of `tier` when `seat` has already kept
/// `kept` this round, before any conversion: (insight needed, credits). The
/// opening (round 1): free for the first `opening_free`, then credits. Later
/// rounds: insight by tier. Paying a shortfall of insight in credits is the
/// caller's: it depends on the insight the seat has left at that point.
///
/// Variant `off` has no draft and no keep parameters: keeping is free there.
pub fn keep_need(w: &World, g: &GameState, seat: usize, tier: u8, kept: i64) -> (i64, i64) {
    if w.variant("cards") == Some("off") {
        return (0, 0);
    }
    if g.round <= 1 {
        if kept < w.int("cards.opening_free") {
            return (0, 0);
        }
        return (0, w.int("cards.opening_keep_credits"));
    }
    let need = match tier {
        2 if ai::keep2(w, g, seat) => science_off(w, g, seat, w.int("cards.keep_tier2_upgraded")),
        2 => science_off(w, g, seat, w.int("cards.keep_tier2")),
        _ => w.int("cards.keep_tier1"),
    };
    (need, 0)
}

pub fn def<'w>(w: &World<'w>, id: &str) -> &'w CardDef {
    &w.content.cards[w.content.card_index(id).expect("card in content")]
}

/// A card's printed credit cost.
pub fn list_cost(w: &World, id: &str) -> i64 {
    def(w, id).cost
}

const SCIENCE_EVERY: &str = "cards.science_every";
const INDUSTRY_EVERY: &str = "cards.industry_every";

/// One off per `every` tags on a seat's cards, of which it has `count` (0: off).
fn per_tags(w: &World, count: i64, every: &str) -> i64 {
    if w.variant("cards") != Some("draft") {
        return 0;
    }
    match w.int(every) {
        0 => 0,
        n => count / n,
    }
}

/// An insight cost `need` after seat `s`'s Science tags (034): never below
/// `cards.discount_floor`, and a cost under the floor stays as printed.
fn science_off(w: &World, g: &GameState, seat: usize, need: i64) -> i64 {
    floored(need, per_tags(w, tags(w, g, seat, Tag::Science), SCIENCE_EVERY), discount_floor(w))
}

/// The lowest a discount takes a cost that is at least this: only the draft
/// variant has a floor.
fn discount_floor(w: &World) -> i64 {
    if w.variant("cards") == Some("draft") {
        w.int("cards.discount_floor")
    } else {
        0
    }
}

/// `cost` less `off`, never below `floor`: a cost under the floor stays as printed.
fn floored(cost: i64, off: i64, floor: i64) -> i64 {
    (cost - off).max(cost.min(floor))
}

/// The insight seat `s` pays to play `d`.
pub fn play_insight(w: &World, g: &GameState, seat: usize, d: &CardDef) -> i64 {
    science_off(w, g, seat, d.insight)
}

/// What seat `s` takes off card costs now, worked out once: a caller pricing
/// a whole hand gets the same numbers as [`play_cost`] and [`play_insight`]
/// without counting the seat's tags again for each card (042: that was most of
/// what the heuristic cost). Good until the seat's cards or vars change.
#[derive(Clone, Copy)]
pub struct Prices {
    insight_off: i64,
    building_off: i64,
    floor: i64,
}

pub fn prices(w: &World, g: &GameState, seat: usize) -> Prices {
    let counts = tag_counts(w, g, seat);
    Prices {
        insight_off: per_tags(w, counts[Tag::Science as usize], SCIENCE_EVERY),
        building_off: passive_discount(w, g, seat)
            + per_tags(w, counts[Tag::Industry as usize], INDUSTRY_EVERY),
        floor: discount_floor(w),
    }
}

impl Prices {
    /// [`play_insight`].
    pub fn insight(&self, d: &CardDef) -> i64 {
        floored(d.insight, self.insight_off, self.floor)
    }

    /// [`play_cost`].
    pub fn credits(&self, d: &CardDef) -> i64 {
        if d.is_building() {
            floored(d.cost, self.building_off, self.floor)
        } else {
            d.cost
        }
    }
}

/// A card's costs beyond credits, in the order its text prints them, with
/// its insight cost for the seat already known.
fn extra_costs_at(d: &CardDef, insight: i64) -> [(Resource, i64); 4] {
    [
        (Resource::Insight, insight),
        (Resource::Energy, d.energy),
        (Resource::Chips, d.chips),
        (Resource::Antimatter, d.antimatter),
    ]
}

/// A card's costs beyond credits, for seat `s`.
fn extra_costs(w: &World, g: &GameState, seat: usize, d: &CardDef) -> [(Resource, i64); 4] {
    extra_costs_at(d, play_insight(w, g, seat, d))
}

/// Discards `card` from pile `from`: to its tier's discard pile, or back to
/// the box for an AI's unique card.
pub fn discard(g: &mut GameState, cx: &mut Cx, card: &str, from: &str) {
    let w = cx.world;
    let to = match def(&w, card).tier {
        0 => String::new(),
        t => discard_pile(t),
    };
    cx.move_card(g, card, from, &to, true)
        .expect("card to discard is there");
}

/// Standing effects that apply to seat `s`: its AI's innate rule and its
/// powered active buildings.
pub fn passives<'w>(w: &World<'w>, g: &GameState, seat: usize) -> Vec<&'w Op> {
    let mut out: Vec<&'w Op> = ai::board(w, seat).innate.passives.iter().collect();
    for id in g.pile(&built(seat)) {
        if g.var(&unpowered(seat, id)) == 0 {
            out.extend(def(w, id).passives.iter());
        }
    }
    out
}

pub fn has_passive(w: &World, g: &GameState, seat: usize, f: impl Fn(&Op) -> bool) -> bool {
    passives(w, g, seat).into_iter().any(f)
}

/// Every card seat `s` has in play whose tags count: active buildings, the
/// cards splayed under them, and programs in the play area.
pub fn in_play(g: &GameState, seat: usize) -> Vec<Key> {
    let mut out = Vec::new();
    for top in g.pile(&built(seat)) {
        out.extend(g.pile(&under(seat, top)).iter().cloned());
        out.push(top.clone());
    }
    out.extend(g.pile(&played(seat)).iter().cloned());
    out
}

/// Seat `s`'s tags of one kind, over every card it has in play (021).
pub fn tags(w: &World, g: &GameState, seat: usize, tag: Tag) -> i64 {
    tag_counts(w, g, seat)[tag as usize]
}

/// Seat `s`'s tags of every kind, indexed by `Tag as usize`: one pass over its
/// cards in play, for a caller that wants more than one count.
pub fn tag_counts(w: &World, g: &GameState, seat: usize) -> [i64; TAGS.len()] {
    let mut out = [0; TAGS.len()];
    let mut count = |ids: &[Key]| {
        for t in ids.iter().flat_map(|id| &def(w, id).tags) {
            out[*t as usize] += 1;
        }
    };
    let tops = g.pile(&built(seat));
    for top in tops {
        count(g.pile(&under(seat, top)));
    }
    count(tops);
    count(g.pile(&played(seat)));
    out
}

/// Active buildings of seat `s` holding tokens of `kind`.
pub fn token_cards(w: &World, g: &GameState, seat: usize, kind: Option<TokenKind>) -> Vec<Key> {
    g.pile(&built(seat))
        .iter()
        .filter(|id| {
            def(w, id)
                .tokens
                .as_ref()
                .is_some_and(|t| kind.is_none_or(|k| t.kind == k))
        })
        .cloned()
        .collect()
}

/// Adds `n` tokens to seat `s`'s card (negative removes; never below 0).
pub fn add_tokens(g: &mut GameState, cx: &mut Cx, seat: usize, card: &str, n: i64) {
    let k = tokens_key(seat, card);
    let n = n.max(-g.var(&k));
    if n == 0 {
        return;
    }
    cx.add_key(g, &k, n);
    if n > 0 {
        cx.tally(g, "cards.tokens_gained", n);
        cx.per_round(g, "cards.tokens_gained", n);
    } else {
        cx.tally(g, "cards.tokens_spent", -n);
        cx.per_round(g, "cards.tokens_spent", -n);
    }
}

/// Building upkeep for seat `s` (in the economy's upkeep): pay each
/// building's upkeep if possible (else it is unpowered next round), then
/// upkeep conversions, then token cards gain their tokens.
pub fn upkeep(g: &mut GameState, cx: &mut Cx, seat: usize) {
    let w = cx.world;
    for id in g.pile(&built(seat)).to_vec() {
        let d = def(&w, &id);
        if d.upkeep.is_empty() {
            continue;
        }
        let ok = d.upkeep.iter().all(|(r, n)| g.resource(seat, *r) >= *n);
        if ok {
            for (r, n) in &d.upkeep {
                cx.add_resource(g, seat, *r, -n);
            }
        }
        cx.set_key(g, &unpowered(seat, &id), (!ok) as i64);
    }
    for p in passives(&w, g, seat) {
        if let Op::ConvertUpkeep {
            from,
            amount,
            to,
            gain,
        } = p
        {
            if g.resource(seat, *from) >= *amount {
                cx.add_resource(g, seat, *from, -amount);
                cx.add_resource(g, seat, *to, *gain);
            }
        }
    }
    for id in token_cards(&w, g, seat, None) {
        let n = def(&w, &id).tokens.as_ref().map_or(0, |t| t.per_round);
        if n > 0 && g.var(&unpowered(seat, &id)) == 0 {
            add_tokens(g, cx, seat, &id, n);
        }
    }
}

/// Building slots seat `s` has: the AI board's (`ai.slots`) plus expansions.
pub fn capacity(w: &World, g: &GameState, seat: usize) -> i64 {
    let extra: i64 = passives(w, g, seat)
        .iter()
        .map(|p| match p {
            Op::Slots { amount } => *amount,
            _ => 0,
        })
        .sum();
    ai::slots(w, seat) + extra
}

/// Active buildings that take a slot (each the top of a stack).
fn slotted(w: &World, g: &GameState, seat: usize) -> Vec<Key> {
    g.pile(&built(seat))
        .iter()
        .filter(|id| !def(w, id).slotless)
        .cloned()
        .collect()
}

/// Stacks a building `d` may be built over: tops carrying its `replaces`
/// tag, or any slotted top.
fn stack_targets(w: &World, g: &GameState, seat: usize, d: &CardDef) -> Vec<Key> {
    slotted(w, g, seat)
        .into_iter()
        .filter(|top| {
            d.replaces
                .as_ref()
                .is_none_or(|t| def(w, top).tags.contains(t))
        })
        .collect()
}

#[derive(Debug)]
struct Cards {
    deal: i64,
    opening_deal: i64,
    insight_credits: i64,
    /// Selected cards (content indices).
    cards: Vec<usize>,
    /// Variant `off`: no draft.
    off: bool,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Cards {
        deal: b.int("deal")?,
        opening_deal: b.int("opening_deal")?,
        insight_credits: b.int("insight_credits")?,
        cards: b.cards.clone(),
        off: false,
    }))
}

fn build_off(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Cards {
        deal: 0,
        opening_deal: 0,
        insight_credits: 1,
        cards: b.cards.clone(),
        off: true,
    }))
}

impl Cards {
    /// What keeping `card` costs `seat` now: (insight, credits). [`keep_need`],
    /// with any shortfall of insight at `insight_credits` credits an insight
    /// (that conversion is only for keeps).
    fn keep_price(&self, w: &World, g: &GameState, seat: usize, card: &str) -> (i64, i64) {
        let kept = g.var(&kept_key(seat));
        let (need, credits) = keep_need(w, g, seat, def(w, card).tier, kept);
        let have = g.resource(seat, Resource::Insight).max(0);
        let ins = need.min(have);
        (ins, credits + (need - ins) * self.insight_credits)
    }
}

/// Why a card can't be played now, or `None` if it can.
fn blocked(w: &World, g: &GameState, seat: usize, id: &str) -> Option<&'static str> {
    let d = def(w, id);
    if d.needs.iter().any(|(t, n)| tags(w, g, seat, *t) < *n) {
        return Some("needs tags");
    }
    if g.resource(seat, Resource::Credits)
        + discount_room(w, g, seat, d).map_or(0, |(_, rate, k)| rate * k)
        < play_cost(w, g, seat, d)
        || extra_costs(w, g, seat, d).iter().any(|&(r, n)| g.resource(seat, r) < n)
        || !lowers_ok(w, g, seat, d)
    {
        return Some("unaffordable");
    }
    if d.replaces.is_some() && stack_targets(w, g, seat, d).is_empty() {
        return Some("nothing to replace");
    }
    if !d.is_building() && !effects::feasible(w, g, seat, &d.effects) {
        return Some("no target");
    }
    None
}

/// Whether seat `s` has the income card `d` lowers (022): each `lowers`
/// resource's income at least the amount.
pub fn lowers_ok(w: &World, g: &GameState, seat: usize, d: &CardDef) -> bool {
    if d.lowers.is_empty() {
        return true;
    }
    let income = economy::income(w, g, seat);
    d.lowers
        .iter()
        .all(|(r, n)| income[*r] >= *n)
}

/// The AI's resource discount on card `d` for seat `s`: (resource, credits
/// per unit, most units worth spending), if the AI has one that applies and
/// the seat holds some of it (beyond what the card itself needs).
pub fn discount_room(
    w: &World,
    g: &GameState,
    seat: usize,
    d: &CardDef,
) -> Option<(Resource, i64, i64)> {
    discount_room_at(w, g, seat, d, &prices(w, g, seat))
}

/// [`discount_room`] with the seat's [`Prices`] already known.
pub fn discount_room_at(
    w: &World,
    g: &GameState,
    seat: usize,
    d: &CardDef,
    prices: &Prices,
) -> Option<(Resource, i64, i64)> {
    let cost = prices.credits(d);
    ai::board(w, seat).discounts.iter().find_map(|x| {
        let applies = match x.tag {
            DiscountOn::Building => d.is_building(),
            DiscountOn::Tag(t) => d.tags.contains(&t),
        };
        let reserved = extra_costs_at(d, prices.insight(d))
            .iter()
            .find(|&&(r, _)| r == x.resource)
            .map_or(0, |&(_, n)| n);
        let have = g.resource(seat, x.resource) - reserved;
        if !applies || have <= 0 || x.rate <= 0 || cost <= 0 {
            return None;
        }
        let most = ((cost + x.rate - 1) / x.rate).min(have);
        Some((x.resource, x.rate, most))
    })
}

/// Credits to play a card: buildings get the seat's discounts.
pub fn play_cost(w: &World, g: &GameState, seat: usize, d: &CardDef) -> i64 {
    if !d.is_building() {
        return d.cost;
    }
    discounted_cost(w, d, building_discount(w, g, seat))
}

/// The credits seat `s`'s buildings cost less now (its passives, a one-off
/// discount and its Industry tags, 034): what [`play_cost`] takes off a building.
pub fn building_discount(w: &World, g: &GameState, seat: usize) -> i64 {
    passive_discount(w, g, seat) + per_tags(w, tags(w, g, seat, Tag::Industry), INDUSTRY_EVERY)
}

/// [`building_discount`] before tags.
fn passive_discount(w: &World, g: &GameState, seat: usize) -> i64 {
    passives(w, g, seat)
        .iter()
        .map(|p| match p {
            Op::BuildingDiscount { amount } => *amount,
            _ => 0,
        })
        .sum::<i64>()
        + g.var(&next_discount(seat))
}

/// [`play_cost`] with the seat's [`building_discount`] already known (a
/// caller pricing a whole hand works it out once).
pub fn discounted_cost(w: &World, d: &CardDef, discount: i64) -> i64 {
    if d.is_building() {
        floored(d.cost, discount, discount_floor(w))
    } else {
        d.cost
    }
}

/// Why an ability can't be used now, or `None` if it can.
fn ability_blocked(w: &World, g: &GameState, seat: usize, id: &str) -> Option<&'static str> {
    let d = def(w, id);
    let ab = d.ability.as_ref()?;
    if g.seats[seat].exhausted.iter().any(|c| c == id) {
        return Some("used");
    }
    if g.var(&unpowered(seat, id)) > 0 {
        return Some("unpowered");
    }
    if g.var(&tokens_key(seat, id)) < ab.tokens || ab.cost.iter().any(|(r, n)| g.resource(seat, *r) < *n) {
        return Some("unaffordable");
    }
    if !effects::feasible(w, g, seat, &ab.effects) {
        return Some("no target");
    }
    None
}

// ---------- sacrifices for AI-board powers (021) ----------

/// Something a seat can sacrifice to pay a power's tags: a program in its
/// play area, a whole building stack (named by its top), or (with a
/// `tags_from_hand` building) a card in hand.
#[derive(Debug, Clone)]
pub struct Source {
    pub label: String,
    /// The pile the cards leave, and the cards (a stack: all of it).
    pub cards: Vec<(String, Key)>,
    pub tags: Vec<Tag>,
}

/// What seat `s` could sacrifice now, in a fixed order: programs, stacks,
/// then hand cards a `tags_from_hand` building counts.
pub fn sources(w: &World, g: &GameState, seat: usize) -> Vec<Source> {
    let mut out = Vec::new();
    for id in g.pile(&played(seat)) {
        out.push(Source {
            label: format!("program {id}"),
            cards: vec![(played(seat).to_string(), id.clone())],
            tags: def(w, id).tags.clone(),
        });
    }
    for top in g.pile(&built(seat)) {
        let mut cards: Vec<(String, Key)> = g
            .pile(&under(seat, top))
            .iter()
            .map(|c| (under(seat, top).to_string(), c.clone()))
            .collect();
        cards.push((built(seat).to_string(), top.clone()));
        let tags = cards
            .iter()
            .flat_map(|(_, c)| def(w, c).tags.iter().copied())
            .collect();
        out.push(Source {
            label: format!("stack {top}"),
            cards,
            tags,
        });
    }
    let from_hand: Vec<TagOrAny> = passives(w, g, seat)
        .into_iter()
        .filter_map(|p| match p {
            Op::TagsFromHand { tag } => Some(*tag),
            _ => None,
        })
        .collect();
    if !from_hand.is_empty() {
        for id in g.pile(&hand(seat)) {
            let tags: Vec<Tag> = def(w, id)
                .tags
                .iter()
                .copied()
                .filter(|&t| from_hand.iter().any(|x| x.accepts(t)))
                .collect();
            if !tags.is_empty() {
                out.push(Source {
                    label: format!("hand {id}"),
                    cards: vec![(hand(seat).to_string(), id.clone())],
                    tags,
                });
            }
        }
    }
    out
}

/// A power's tag cost still unpaid: specific tags, and `any`.
#[derive(Debug, Clone)]
pub struct TagNeed {
    pub specific: BTreeMap<Tag, i64>,
    pub any: i64,
}

impl TagNeed {
    pub fn new(cost: &[(TagOrAny, i64)]) -> TagNeed {
        let mut need = TagNeed {
            specific: BTreeMap::new(),
            any: 0,
        };
        for (t, n) in cost {
            match t {
                TagOrAny::Any => need.any += n,
                TagOrAny::Is(t) => *need.specific.entry(*t).or_default() += n,
            }
        }
        need
    }

    pub fn met(&self) -> bool {
        self.any <= 0 && self.specific.values().all(|&n| n <= 0)
    }

    /// Whether `tags` would pay any of what is still needed.
    pub fn helps(&self, tags: &[Tag]) -> bool {
        tags.iter()
            .any(|t| self.any > 0 || self.specific.get(t).is_some_and(|&n| n > 0))
    }

    /// Pays with `tags`: each tag fills its own requirement first, then `any`.
    pub fn pay(&mut self, tags: &[Tag]) {
        for t in tags {
            match self.specific.get_mut(t) {
                Some(n) if *n > 0 => *n -= 1,
                _ => {
                    if self.any > 0 {
                        self.any -= 1
                    }
                }
            }
        }
    }

    /// Whether the tags of all of `sources` together could pay it (exact:
    /// a tag fills its own requirement first, the rest go to `any`).
    pub fn payable(&self, sources: &[Source]) -> bool {
        let mut left = self.clone();
        for s in sources {
            left.pay(&s.tags);
        }
        left.met()
    }
}

/// Seat `s` pays a power's tag cost by sacrificing sources one at a time
/// (its choice among those that still help), until it is paid.
pub fn sacrifice_for(g: &mut GameState, cx: &mut Cx, seat: usize, cost: &[(TagOrAny, i64)]) -> Res {
    let w = cx.world;
    let mut need = TagNeed::new(cost);
    let mut given = 0;
    // What can be sacrificed is fixed when the power is used: sacrificing a
    // `tags_from_hand` building mid-payment doesn't take its hand cards away.
    let mut left = sources(&w, g, seat);
    while !need.met() {
        let helping: Vec<Source> = left
            .iter()
            .filter(|s| need.helps(&s.tags))
            .cloned()
            .collect();
        if helping.is_empty() {
            return Err(Halt::Illegal("not enough tags to sacrifice".into()));
        }
        let opts = helping.iter().map(|s| opt(format!("sacrifice {}", s.label))).collect();
        let prompt = format!("Seat {seat}: sacrifice what for the power's tags?");
        let Some(i) = cx.choose(seat, &prompt, opts)? else {
            return Err(Halt::Illegal("no sacrifice chosen".into()));
        };
        let src = &helping[i];
        left.retain(|x| x.label != src.label);
        need.pay(&src.tags);
        given += src.tags.len() as i64;
        for (pile, card) in &src.cards {
            cx.set_key(g, &tokens_key(seat, card), 0);
            cx.set_key(g, &unpowered(seat, card), 0);
            discard(g, cx, card, pile);
            cx.tally(g, "cards.sacrificed", 1);
        }
    }
    cx.tally(g, "ai.tags_sacrificed", given);
    cx.per_round(g, "ai.tags_sacrificed", given);
    Ok(())
}

/// Seat `s` draws up to `n` tier-1 cards into its hand, no keep paid (023's
/// `draw`); the deck reshuffles its discards when it runs out.
pub fn draw(g: &mut GameState, cx: &mut Cx, seat: usize, n: i64) -> Res {
    for _ in 0..n.max(0) {
        if !deal_top(g, cx, 1, &hand(seat))? {
            break;
        }
        cx.tally(g, "cards.drawn", 1);
    }
    Ok(())
}

/// The top card of a tier's deck, reshuffling its discards if it is empty.
fn top_of(g: &mut GameState, cx: &mut Cx, tier: u8) -> Option<Key> {
    if g.pile(&deck(tier)).is_empty() {
        let mut cards = g.pile(&discard_pile(tier)).to_vec();
        if cards.is_empty() {
            return None;
        }
        shuffle(&mut cards, cx.rng(g, "cards"));
        cx.set_pile(g, &discard_pile(tier), Vec::new());
        cx.set_pile(g, &deck(tier), cards);
        cx.note(None, || format!("Tier-{tier} discards reshuffled into the deck"));
    }
    g.pile(&deck(tier)).first().cloned()
}

/// Moves the top card of a tier's deck (see [`top_of`]) to pile `to`; false
/// with no card to move.
fn deal_top(g: &mut GameState, cx: &mut Cx, tier: u8, to: &str) -> Res<bool> {
    let Some(id) = top_of(g, cx, tier) else {
        return Ok(false);
    };
    cx.move_card(g, &id, &deck(tier), to, false)?;
    Ok(true)
}

fn all_picked(g: &GameState) -> bool {
    (0..g.seats.len()).all(|s| g.var(&picked_key(s)) > 0 || g.pile(&packet(s)).is_empty())
}

fn tier_open(g: &GameState, tier: u8) -> bool {
    !(g.pile(&deck(tier)).is_empty() && g.pile(&discard_pile(tier)).is_empty())
}

impl Cards {
    fn play(&self, g: &mut GameState, cx: &mut Cx, seat: usize, id: &str) -> Res {
        let w = cx.world;
        if !g.pile(&hand(seat)).iter().any(|c| c == id) {
            return Err(Halt::Illegal(format!("{id} is not in seat {seat}'s hand")));
        }
        if let Some(why) = blocked(&w, g, seat, id) {
            return Err(Halt::Illegal(format!("can't play {id}: {why}")));
        }
        let d = def(&w, id);
        let mut credits = play_cost(&w, g, seat, d);
        if let Some((resource, rate, most)) = discount_room(&w, g, seat, d) {
            // Spend the AI's resource for credits: at least enough to afford it.
            let short = (credits - g.resource(seat, Resource::Credits)).max(0);
            let least = (short + rate - 1) / rate;
            let opts = (least..=most)
                .map(|k| opt(format!("spend {}", resource.amount(k))))
                .collect();
            let prompt =
                format!("Seat {seat}: spend how much {resource} at {rate} credit each on {id}?");
            let k = least + cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
            if k > 0 {
                cx.add_resource(g, seat, resource, -k);
                credits = (credits - k * rate).max(0);
                cx.tally(g, "cards.discount_used", k);
            }
        }
        cx.add_resource(g, seat, Resource::Credits, -credits);
        let [insight, energy, chips, antimatter] = extra_costs(&w, g, seat, d);
        for (r, n) in [antimatter, energy, chips, insight] {
            cx.add_resource(g, seat, r, -n);
        }
        if insight.1 > 0 {
            cx.tally(g, "cards.insight_played", insight.1);
            cx.per_round(g, "cards.insight_played", insight.1);
        }
        for (r, n) in &d.lowers {
            cx.add_key(g, &economy::production_key(seat, *r), -n);
        }
        if d.chips > 0 {
            cx.tally(g, "board.chips_spent", d.chips);
        }
        cx.count(g, &format!("played.{id}"), 1);
        cx.per_round(g, "cards.played", 1);
        if d.is_building() {
            cx.set_key(g, &next_discount(seat), 0);
            if !d.slotless {
                // A new slot while one is free (not for a replacement), or
                // over one of your buildings: it splays down.
                let targets = stack_targets(&w, g, seat, d);
                let free = d.replaces.is_none()
                    && (slotted(&w, g, seat).len() as i64) < capacity(&w, g, seat);
                let mut opts = Vec::new();
                if free {
                    opts.push(opt("a new slot".to_string()));
                }
                opts.extend(targets.iter().map(|t| opt(format!("over {t}"))));
                let prompt = format!("Seat {seat}: build {id} where?");
                let k = cx.choose(seat, &prompt, opts)?;
                let over = match k {
                    Some(0) if free => None,
                    Some(k) => targets.get(k - free as usize).cloned(),
                    None => None,
                };
                if let Some(old) = over {
                    // The old stack goes under the new card, the old top last.
                    for c in g.pile(&under(seat, &old)).to_vec() {
                        cx.move_card(g, &c, &under(seat, &old), &under(seat, id), false)?;
                    }
                    cx.move_card(g, &old, &built(seat), &under(seat, id), false)?;
                    cx.set_key(g, &unpowered(seat, &old), 0);
                    cx.tally(g, "cards.replaced", 1);
                    cx.per_round(g, "cards.replaced", 1);
                }
            }
            cx.move_card(g, id, &hand(seat), &built(seat), false)?;
        } else if d.is_event() {
            discard(g, cx, id, &hand(seat));
        } else {
            cx.move_card(g, id, &hand(seat), &played(seat), false)?;
        }
        effects::run(g, cx, &d.effects, &mut effects::Ctx::seat(seat))
    }
}

impl Module for Cards {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        let starts: Vec<&str> = (0..g.seats.len())
            .filter_map(|s| ai::board(&w, s).start.as_deref())
            .collect();
        for tier in [1, 2] {
            let mut ids: Vec<Key> = self
                .cards
                .iter()
                .map(|&i| &w.content.cards[i])
                .filter(|c| c.tier == tier && !starts.contains(&c.id.as_str()))
                .map(|c| Key::new(&c.id))
                .collect();
            shuffle(&mut ids, cx.rng(g, "cards"));
            cx.set_pile(g, &deck(tier), ids);
        }
        for s in 0..g.seats.len() {
            let b = ai::board(&w, s);
            let selected =
                |id: &str| w.content.card_index(id).is_some_and(|i| self.cards.contains(&i));
            for id in &b.cards {
                let taken = (0..s).any(|o| g.pile(&hand(o)).iter().any(|c| c == id));
                if selected(id) && !taken {
                    cx.move_card(g, id, "", &hand(s), false).unwrap();
                }
            }
            // The board's starting building (021), one copy: a repeated AI's
            // second seat has none.
            if let Some(id) = &b.start {
                let taken = (0..s).any(|o| g.pile(&built(o)).iter().any(|c| c == id));
                if selected(id) && !taken {
                    cx.move_card(g, id, "", &built(s), false).unwrap();
                }
            }
        }
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            for id in g.seats[s].exhausted.clone() {
                cx.refresh(g, s, &id);
            }
            cx.set_key(g, &next_discount(s), 0);
            cx.set_key(g, &civic_used(s), 0);
            cx.set_key(g, &kept_key(s), 0);
        }
    }

    fn begin(&self, phase: usize, g: &mut GameState, cx: &mut Cx) {
        match phase {
            0 => {
                // The opening has no draft (round 1 deals the opening hand).
                // With both decks and discards empty there is nothing to deal.
                let dry = !tier_open(g, 1) && !tier_open(g, 2);
                for s in 0..g.seats.len() {
                    if self.off || g.round <= 1 || self.deal <= 0 || dry {
                        g.seats[s].passed = true;
                    }
                }
            }
            2 => {
                if (0..g.seats.len()).all(|s| g.pile(&packet(s)).is_empty()) {
                    for seat in g.seats.iter_mut() {
                        seat.passed = true;
                    }
                }
            }
            3 => {
                for s in 0..g.seats.len() {
                    if g.pile(&drafted(s)).is_empty() {
                        g.seats[s].passed = true;
                    }
                }
            }
            4 => {
                for s in 0..g.seats.len() {
                    let hand_n = g.pile(&hand(s)).len() as i64;
                    cx.per_round(g, "cards.hand_size", hand_n);
                    let ins = g.resource(s, Resource::Insight);
                    cx.per_round(g, "cards.insight_after_keep", ins);
                    for id in g.pile(&drafted(s)).to_vec() {
                        discard(g, cx, &id, &drafted(s));
                    }
                }
            }
            _ => {}
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        // The opening (round 1): each seat is dealt its opening hand to keep from.
        if self.off || g.round > 1 {
            return Ok(());
        }
        for s in 0..g.seats.len() {
            for _ in 0..self.opening_deal {
                if !deal_top(g, cx, 1, &drafted(s))? {
                    break;
                }
            }
        }
        Ok(())
    }

    fn legal(&self, phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        match phase {
            0 => {
                for tier in [1u8, 2] {
                    if tier_open(g, tier) {
                        out.push(Action::Tier { seat, tier });
                    }
                }
            }
            2 => {
                if g.var(&picked_key(s)) == 0 {
                    for id in g.pile(&packet(s)) {
                        out.push(Action::Draft {
                            seat,
                            card: id.clone(),
                        });
                    }
                }
            }
            3 => {
                let free = alignment::free_purchases(g) > 0;
                for id in g.pile(&drafted(s)) {
                    let (_, cr) = self.keep_price(w, g, s, id);
                    if free || g.resource(s, Resource::Credits) >= cr {
                        out.push(Action::Keep {
                            seat,
                            card: id.clone(),
                        });
                    }
                }
            }
            4 => {
                for id in g.pile(&hand(s)) {
                    if blocked(w, g, s, id).is_none() {
                        out.push(Action::Play {
                            seat,
                            card: id.clone(),
                        });
                    }
                }
                for id in g.pile(&built(s)) {
                    if def(w, id).ability.is_some() && ability_blocked(w, g, s, id).is_none() {
                        out.push(Action::Use {
                            seat,
                            card: id.clone(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn can_pass(&self, phase: usize, _g: &GameState, _seat: u8) -> bool {
        !matches!(phase, 0 | 2)
    }

    fn owns(&self, phase: usize, a: &Action) -> bool {
        matches!(
            (phase, a),
            (0, Action::Tier { .. })
                | (2, Action::Draft { .. })
                | (3, Action::Keep { .. })
                | (4, Action::Play { .. } | Action::Use { .. })
        )
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let s = a.seat() as usize;
        match a {
            Action::Tier { tier, .. } => {
                // Card by card: the chosen tier's top card joins the hand.
                if !(1..=2).contains(tier) || !tier_open(g, *tier) {
                    return Err("no such tier to draft from".into());
                }
                if !deal_top(g, cx, *tier, &packet(s))? {
                    return Err("no card to deal".into());
                }
                cx.tally(g, &format!("cards.dealt_tier{tier}"), 1);
                let full = g.pile(&packet(s)).len() as i64 >= self.deal;
                if full {
                    g.seats[s].passed = true;
                }
                if !(tier_open(g, 1) || tier_open(g, 2)) {
                    // The decks ran dry: every seat's hand is what it has.
                    for seat in g.seats.iter_mut() {
                        seat.passed = true;
                    }
                }
            }
            Action::Draft { card, .. } => {
                if g.var(&picked_key(s)) > 0 {
                    return Err("already took a card from this hand".into());
                }
                cx.move_card(g, card, &packet(s), &drafted(s), false)?;
                cx.set_key(g, &picked_key(s), 1);
                if all_picked(g) {
                    let n = g.seats.len();
                    let packets: Vec<Vec<Key>> =
                        (0..n).map(|p| g.pile(&packet(p)).to_vec()).collect();
                    // As in Terraforming Mars, the direction alternates by round.
                    let step = if g.round % 2 == 1 { 1 } else { n - 1 };
                    for (p, cards) in packets.into_iter().enumerate() {
                        cx.set_pile(g, &packet((p + step) % n), cards);
                    }
                    for p in 0..n {
                        cx.set_key(g, &picked_key(p), 0);
                    }
                    if (0..n).all(|p| g.pile(&packet(p)).is_empty()) {
                        for seat in g.seats.iter_mut() {
                            seat.passed = true;
                        }
                    }
                }
            }
            Action::Keep { card, .. } => {
                if !g.pile(&drafted(s)).contains(card) {
                    return Err(Halt::Illegal(format!("{card} was not drafted by seat {s}")));
                }
                // A Unity card's free purchase first, then the price.
                let (ins, cr) = self.keep_price(&w, g, s, card);
                if (ins > 0 || cr > 0) && !alignment::take_free_purchase(g, cx) {
                    if g.resource(s, Resource::Credits) < cr {
                        return Err("can't afford to keep it".into());
                    }
                    cx.add_resource(g, s, Resource::Insight, -ins);
                    cx.add_resource(g, s, Resource::Credits, -cr);
                    if ins > 0 {
                        cx.tally(g, "cards.insight_kept", ins);
                        cx.per_round(g, "cards.insight_kept", ins);
                    }
                    if g.round > 1 && cr > 0 {
                        cx.tally(g, "cards.insight_bought", cr / self.insight_credits);
                    }
                }
                cx.add_key(g, &kept_key(s), 1);
                cx.move_card(g, card, &drafted(s), &hand(s), false)?;
                cx.count(g, &format!("kept.{card}"), 1);
                cx.per_round(g, "cards.kept", 1);
                if g.pile(&drafted(s)).is_empty() {
                    g.seats[s].passed = true;
                }
            }
            Action::Play { card, .. } => self.play(g, cx, s, card)?,
            Action::Use { card, .. } => {
                if !g.pile(&built(s)).contains(card) {
                    return Err(Halt::Illegal(format!("seat {s} has no {card} in play")));
                }
                if let Some(why) = ability_blocked(&w, g, s, card) {
                    return Err(Halt::Illegal(format!("can't use {card}: {why}")));
                }
                let ab = def(&w, card).ability.as_ref().ok_or("no ability")?;
                for (r, n) in &ab.cost {
                    cx.add_resource(g, s, *r, -n);
                }
                add_tokens(g, cx, s, card, -ab.tokens);
                cx.exhaust(g, s, card);
                cx.count(g, &format!("used.{card}"), 1);
                effects::run(g, cx, &ab.effects, &mut effects::Ctx::seat(s))?;
            }
            _ => return Err(Halt::Illegal(format!("not a card action: {a:?}"))),
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let card = |s: usize, id: &str, status: &str, cost: Option<i64>| {
            let d = def(w, id);
            let mut bits = vec![format!("{:?}", d.kind).to_lowercase()];
            if d.tier > 0 {
                bits.push(format!("tier {}", d.tier));
            } else {
                bits.push("unique".into());
            }
            if !d.tags.is_empty() {
                bits.push(d.tags.iter().map(|t| t.name()).collect::<Vec<_>>().join("/"));
            }
            for (r, n) in extra_costs(&w, g, s, d) {
                if n > 0 {
                    bits.push(format!("+{}", r.amount(n)));
                }
            }
            for (r, n) in &d.lowers {
                bits.push(format!("-{} income", r.amount(*n)));
            }
            if !d.needs.is_empty() {
                let needs: Vec<String> = d.needs.iter().map(|(t, n)| format!("{n} {t}")).collect();
                bits.push(format!("needs {}", needs.join(", ")));
            }
            if let Some(t) = &d.replaces {
                bits.push(format!("built over a {t} building"));
            }
            let mut text = format!("[{}] {}", bits.join(", "), d.text);
            let mut name = d.name.clone();
            if let Some(t) = &d.tokens {
                let n = g.var(&tokens_key(s, id));
                text.push_str(&format!(" ({n} {} on it)", t.kind));
                // The count on the card itself, as tokens sit on a printed card.
                if n > 0 && status != "discarded" && status != "draft" {
                    name = format!("{} · {n} {}", d.name, t.kind);
                }
            }
            let under_ids = g.pile(&under(s, id));
            if !under_ids.is_empty() {
                let names: Vec<&str> = under_ids.iter().map(|c| def(w, c).name.as_str()).collect();
                text.push_str(&format!(" Splayed under it: {}.", names.join(", ")));
            }
            CardView {
                id: id.to_string(),
                name,
                text,
                status: status.to_string(),
                cost,
            }
        };
        for s in 0..g.seats.len() {
            let hand_cards: Vec<CardView> = g
                .pile(&hand(s))
                .iter()
                .map(|id| {
                    let st = blocked(w, g, s, id).unwrap_or("ready");
                    card(s, id, st, Some(play_cost(w, g, s, def(w, id))))
                })
                .collect();
            out.cards(s, "hand", "hand", hand_cards);
            let row: Vec<CardView> = g
                .pile(&built(s))
                .iter()
                .map(|id| {
                    let st = if def(w, id).ability.is_some() {
                        ability_blocked(w, g, s, id).unwrap_or("ready")
                    } else if g.var(&unpowered(s, id)) > 0 {
                        "unpowered"
                    } else if !g.pile(&under(s, id)).is_empty() {
                        "stacked"
                    } else {
                        "built"
                    };
                    card(s, id, st, None)
                })
                .collect();
            let label = format!("buildings {}/{}", slotted(w, g, s).len(), capacity(w, g, s));
            out.cards(s, "built", &label, row);
            let progs: Vec<CardView> = g
                .pile(&played(s))
                .iter()
                .map(|id| card(s, id, "in play", None))
                .collect();
            if !progs.is_empty() {
                out.cards(s, "played", "programs in play", progs);
            }
            let pk: Vec<CardView> = g
                .pile(&packet(s))
                .iter()
                .map(|id| card(s, id, "draft", None))
                .collect();
            if !pk.is_empty() {
                out.cards(s, "packet", "draft hand", pk);
            }
            let dr: Vec<CardView> = g
                .pile(&drafted(s))
                .iter()
                .map(|id| {
                    let (ins, cr) = self.keep_price(w, g, s, id);
                    let st = match (ins, cr) {
                        (0, 0) => "keep free".to_string(),
                        (i, 0) => format!("keep {i} insight"),
                        (0, c) => format!("keep {c} credits"),
                        (i, c) => format!("keep {i} insight + {c} credits"),
                    };
                    card(s, id, &st, None)
                })
                .collect();
            if !dr.is_empty() {
                let label = if g.round <= 1 { "opening hand (keep?)" } else { "drafted (keep?)" };
                out.cards(s, "drafted", label, dr);
            }
            out.seat(
                s,
                "slots",
                "building slots",
                capacity(w, g, s),
                None,
                "building slots (AI board + expansions)",
            );
            for tag in crate::content::TAGS {
                let n = tags(w, g, s, tag);
                if n > 0 {
                    out.seat(
                        s,
                        &format!("tag.{tag}"),
                        &format!("{tag} tags"),
                        n,
                        None,
                        "tags on your cards in play (buildings, splayed ones included, and programs)",
                    );
                }
            }
            let nd = g.var(&next_discount(s));
            if nd > 0 {
                out.seat(
                    s,
                    "next_discount",
                    "next building -",
                    nd,
                    None,
                    "your next building this round costs this much less",
                );
            }
            if g.var(&civic_used(s)) > 0 {
                out.seat(
                    s,
                    "civic_used",
                    "Aegis Dome Array used",
                    1,
                    None,
                    "your Aegis Dome Array has saved a building this round",
                );
            }
        }
        // Per-seat markers: shown above as seat values, card status
        // (`unpowered`), card text (tokens) or the draft piles (`picked`).
        out.bookkeeping(&[
            "cards.picked.",
            "cards.next_discount.",
            "cards.civic_used.",
            "cards.unpowered.",
            "cards.kept_round.",
            "cards.tokens.",
        ]);
        for tier in [1u8, 2] {
            out.pile(
                &format!("deck.{tier}"),
                &format!("Tier-{tier} deck"),
                None,
                g.pile(&deck(tier)).len(),
                Vec::new(),
            );
            let d = g.pile(&discard_pile(tier));
            out.pile(
                &format!("discard.{tier}"),
                &format!("Tier-{tier} discard"),
                None,
                d.len(),
                d.iter().map(|id| card(0, id, "discarded", None)).collect(),
            );
        }
    }
}
