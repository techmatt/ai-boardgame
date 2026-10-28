//! Cards and draft: the tier-1 and tier-2 decks (one copy of each card;
//! discards reshuffle when a deck runs out), the Terraforming Mars-style
//! draft, keeping drafted cards, playing programs and buildings, building
//! abilities and upkeep, and drawing with compute.

use crate::action::Action;
use crate::content::{CardDef, Passive};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, Res, VariantDef,
    World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{ai, effects, opposition};

pub static DEF: ModuleDef = ModuleDef {
    id: "cards",
    summary: "Card decks, the draft, keeping cards, playing programs and buildings",
    requires: &["ai", "adversary", "opposition", "threats", "economy", "board"],
    variants: &[VariantDef {
        name: "draft",
        summary: "Terraforming Mars draft: choose a tier, take one card and pass the rest, then keep any at a price",
        params: &[
            i("deal", 0, 10, "cards each player is dealt for the draft"),
            i("keep_cost", 0, 20, "credits to keep one drafted card"),
            i("draw_cost", 0, 20, "compute to draw one card"),
        ],
        phases: &[
            PhaseSpec {
                name: "tier",
                kind: PhaseKind::Decision,
                doc: "players who have unlocked tier 2 choose which tier they are dealt from",
            },
            PhaseSpec {
                name: "deal",
                kind: PhaseKind::Auto,
                doc: "deal each player their draft hand",
            },
            PhaseSpec {
                name: "draft",
                kind: PhaseKind::Decision,
                doc: "take one card and pass the rest to the next seat, until the hands are gone",
            },
            PhaseSpec {
                name: "keep",
                kind: PhaseKind::Decision,
                doc: "buy any drafted cards into your hand; the rest are discarded",
            },
            PhaseSpec {
                name: "play",
                kind: PhaseKind::Decision,
                doc: "play cards, use building abilities, draw with compute",
            },
        ],
        rng: &["cards"],
        uses_cards: true,
        build,
    }],
};

const fn i(name: &'static str, min: i64, max: i64, doc: &'static str) -> ParamSpec {
    ParamSpec {
        name,
        kind: ParamKind::Int,
        min,
        max,
        doc,
    }
}

pub fn hand(seat: usize) -> String {
    format!("cards.hand.{seat}")
}

pub fn built(seat: usize) -> String {
    format!("cards.built.{seat}")
}

fn packet(seat: usize) -> String {
    format!("cards.packet.{seat}")
}

fn drafted(seat: usize) -> String {
    format!("cards.drafted.{seat}")
}

fn deck(tier: u8) -> String {
    format!("cards.deck.{tier}")
}

fn discard_pile(tier: u8) -> String {
    format!("cards.discard.{tier}")
}

fn tier_key(seat: usize) -> String {
    format!("cards.tier.{seat}")
}

fn picked_key(seat: usize) -> String {
    format!("cards.picked.{seat}")
}

fn next_discount(seat: usize) -> String {
    format!("cards.next_discount.{seat}")
}

fn civic_used(seat: usize) -> String {
    format!("cards.civic_used.{seat}")
}

fn unpowered(seat: usize, card: &str) -> String {
    format!("cards.unpowered.{seat}.{card}")
}

pub fn def<'w>(w: &World<'w>, id: &str) -> &'w CardDef {
    &w.content.cards[w.content.card_index(id).expect("card in content")]
}

/// A card's printed credit cost.
pub fn list_cost(w: &World, id: &str) -> i64 {
    w.amount(&def(w, id).cost)
}

/// Discards `card` from pile `from`: to its tier's discard pile, or back to
/// the box for an AI's unique card.
pub fn discard(g: &mut GameState, cx: &mut Cx, card: &str, from: &str) {
    let w = cx.world;
    let to = match def(&w, card).tier {
        0 => String::new(),
        t => discard_pile(t),
    };
    cx.move_card(g, card, from, &to, true).expect("card to discard is there");
}

/// Standing effects that apply to seat `s`: its AI's innate rule and its
/// powered buildings.
pub fn passives<'w>(w: &World<'w>, g: &GameState, seat: usize) -> Vec<&'w Passive> {
    let mut out: Vec<&'w Passive> = ai::board(w, seat).innate.passives.iter().collect();
    for id in g.pile(&built(seat)) {
        if g.var(&unpowered(seat, id)) == 0 {
            out.extend(def(w, id).passives.iter());
        }
    }
    out
}

pub fn has_passive(w: &World, g: &GameState, seat: usize, f: impl Fn(&Passive) -> bool) -> bool {
    passives(w, g, seat).into_iter().any(f)
}

/// Seat `s`'s tags of one kind, over its buildings.
pub fn tags(w: &World, g: &GameState, seat: usize, tag: &str) -> i64 {
    g.pile(&built(seat))
        .iter()
        .map(|id| def(w, id).tags.iter().filter(|t| *t == tag).count() as i64)
        .sum()
}

/// Ravage damage one of seat `s`'s defense tokens prevents.
pub fn defense_value(w: &World, g: &GameState, seat: usize) -> i64 {
    passives(w, g, seat)
        .iter()
        .filter_map(|p| match p {
            Passive::DefenseValue { amount } => Some(*amount),
            _ => None,
        })
        .max()
        .unwrap_or_else(|| w.int("adversary.defense_value"))
}

/// Uses a Civic Shelter for a population loss in `country`, if one applies.
pub fn civic_shelter(g: &mut GameState, cx: &mut Cx, country: usize) -> bool {
    let w = cx.world;
    for s in 0..g.seats.len() {
        if g.var(&civic_used(s)) == 0
            && has_processor_in(&w, g, s, country)
            && has_passive(&w, g, s, |p| matches!(p, Passive::CivicShelter))
        {
            cx.set_key(g, civic_used(s), 1);
            return true;
        }
    }
    false
}

/// Building upkeep for seat `s` (in the economy's upkeep): pay each
/// building's upkeep if possible (else it is unpowered next round), then
/// upkeep conversions.
pub fn upkeep(g: &mut GameState, cx: &mut Cx, seat: usize) {
    let w = cx.world;
    for id in g.pile(&built(seat)).to_vec() {
        let d = def(&w, &id);
        if d.upkeep.is_empty() {
            continue;
        }
        let ok = d.upkeep.iter().all(|(r, n)| g.resource(seat, r) >= *n);
        if ok {
            for (r, n) in &d.upkeep {
                cx.add_resource(g, seat, r, -n);
            }
        }
        cx.set_key(g, unpowered(seat, &id), (!ok) as i64);
    }
    for p in passives(&w, g, seat) {
        if let Passive::ConvertUpkeep {
            from,
            amount,
            to,
            gain,
        } = p
        {
            if g.resource(seat, from) >= *amount {
                cx.add_resource(g, seat, from, -amount);
                cx.add_resource(g, seat, to, *gain);
            }
        }
    }
}

/// Building slots seat `s` has.
pub fn capacity(w: &World, g: &GameState, seat: usize) -> i64 {
    let extra: i64 = passives(w, g, seat)
        .iter()
        .map(|p| match p {
            Passive::Slots { amount } => *amount,
            _ => 0,
        })
        .sum();
    ai::board(w, seat).slots + extra
}

/// Buildings that take a slot.
fn slotted(w: &World, g: &GameState, seat: usize) -> Vec<String> {
    g.pile(&built(seat))
        .iter()
        .filter(|id| !def(w, id).slotless)
        .cloned()
        .collect()
}

#[derive(Debug)]
struct Cards {
    deal: i64,
    keep_cost: i64,
    draw_cost: i64,
    /// Selected cards (content indices).
    cards: Vec<usize>,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Cards {
        deal: b.int("deal")?,
        keep_cost: b.int("keep_cost")?,
        draw_cost: b.int("draw_cost")?,
        cards: b.cards.clone(),
    }))
}

/// Why a card can't be played now, or `None` if it can.
fn blocked(w: &World, g: &GameState, seat: usize, id: &str) -> Option<&'static str> {
    let d = def(w, id);
    if d.tier == 2 && !ai::tier2(w, g, seat) {
        return Some("needs tier 2");
    }
    if d.needs.iter().any(|(t, n)| tags(w, g, seat, t) < *n) {
        return Some("needs tags");
    }
    if g.resource(seat, "insight") < w.amount(&d.insight)
        || g.resource(seat, "credits") < play_cost(w, g, seat, d)
        || g.resource(seat, "antimatter") < w.amount(&d.antimatter)
    {
        return Some("unaffordable");
    }
    if !d.is_building() && !effects::feasible(w, g, seat, &d.effects) {
        return Some("no target");
    }
    None
}

/// Credits to play a card: buildings get the seat's discounts.
pub fn play_cost(w: &World, g: &GameState, seat: usize, d: &CardDef) -> i64 {
    let cost = w.amount(&d.cost);
    if !d.is_building() {
        return cost;
    }
    let discount: i64 = passives(w, g, seat)
        .iter()
        .map(|p| match p {
            Passive::BuildingDiscount { amount } => *amount,
            _ => 0,
        })
        .sum::<i64>()
        + g.var(&next_discount(seat));
    (cost - discount).max(0)
}

/// Why a building's ability can't be used now, or `None` if it can.
fn ability_blocked(w: &World, g: &GameState, seat: usize, id: &str) -> Option<&'static str> {
    let d = def(w, id);
    let ab = d.ability.as_ref()?;
    if g.seats[seat].exhausted.iter().any(|c| c == id) {
        return Some("used");
    }
    if g.var(&unpowered(seat, id)) > 0 {
        return Some("unpowered");
    }
    if ab.cost.iter().any(|(r, n)| g.resource(seat, r) < *n) {
        return Some("unaffordable");
    }
    if !effects::feasible(w, g, seat, &ab.effects) {
        return Some("no target");
    }
    None
}

fn can_draw(w: &World, g: &GameState, seat: usize, tier: u8, cost: i64) -> bool {
    g.resource(seat, "compute") >= cost
        && (tier == 1 || ai::tier2(w, g, seat))
        && !(g.pile(&deck(tier)).is_empty() && g.pile(&discard_pile(tier)).is_empty())
}

impl Cards {
    /// The top card of a tier's deck, reshuffling its discards if it is empty.
    fn top(&self, g: &mut GameState, cx: &mut Cx, tier: u8) -> Option<String> {
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

    fn all_picked(&self, g: &GameState) -> bool {
        (0..g.seats.len()).all(|s| g.var(&picked_key(s)) > 0 || g.pile(&packet(s)).is_empty())
    }

    fn play(&self, g: &mut GameState, cx: &mut Cx, seat: usize, id: &str) -> Res {
        let w = cx.world;
        if !g.pile(&hand(seat)).iter().any(|c| c == id) {
            return Err(Halt::Illegal(format!("{id} is not in seat {seat}'s hand")));
        }
        if let Some(why) = blocked(&w, g, seat, id) {
            return Err(Halt::Illegal(format!("can't play {id}: {why}")));
        }
        let d = def(&w, id);
        let credits = play_cost(&w, g, seat, d);
        cx.add_resource(g, seat, "credits", -credits);
        cx.add_resource(g, seat, "antimatter", -w.amount(&d.antimatter));
        cx.add_resource(g, seat, "insight", -w.amount(&d.insight));
        cx.count(g, &format!("played.{id}"), 1);
        let mut ctx = effects::Ctx::default();
        if d.is_building() {
            cx.set_key(g, next_discount(seat), 0);
            if !d.slotless {
                let row = slotted(&w, g, seat);
                if row.len() as i64 >= capacity(&w, g, seat) {
                    let opts = row.iter().map(|c| opt(c.clone())).collect();
                    let prompt = format!("Seat {seat}: the building row is full. Discard which building?");
                    if let Some(k) = cx.choose(seat, &prompt, opts)? {
                        discard(g, cx, &row[k], &built(seat));
                        cx.set_key(g, unpowered(seat, &row[k]), 0);
                    }
                }
            }
            cx.move_card(g, id, &hand(seat), &built(seat), false)?;
            effects::run(g, cx, seat, &d.effects, &mut ctx)?;
        } else {
            discard(g, cx, id, &hand(seat));
            effects::run(g, cx, seat, &d.effects, &mut ctx)?;
        }
        Ok(())
    }
}

impl Module for Cards {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for tier in [1, 2] {
            let mut ids: Vec<String> = self
                .cards
                .iter()
                .map(|&i| &w.content.cards[i])
                .filter(|c| c.tier == tier)
                .map(|c| c.id.clone())
                .collect();
            shuffle(&mut ids, cx.rng(g, "cards"));
            cx.set_pile(g, &deck(tier), ids);
        }
        for s in 0..g.seats.len() {
            for id in &ai::board(&w, s).cards {
                let selected = w.content.card_index(id).is_some_and(|i| self.cards.contains(&i));
                let taken = (0..s).any(|o| g.pile(&hand(o)).contains(id));
                if selected && !taken {
                    cx.move_card(g, id, "", &hand(s), false).unwrap();
                }
            }
        }
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            for id in g.seats[s].exhausted.clone() {
                cx.refresh(g, s, &id);
            }
            cx.set_key(g, next_discount(s), 0);
            cx.set_key(g, civic_used(s), 0);
        }
    }

    fn begin(&self, phase: usize, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        match phase {
            0 => {
                for s in 0..g.seats.len() {
                    if !ai::tier2(&w, g, s) {
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
                    for id in g.pile(&drafted(s)).to_vec() {
                        discard(g, cx, &id, &drafted(s));
                    }
                }
            }
            _ => {}
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        // Deal: each seat from the tier it chose.
        for s in 0..g.seats.len() {
            let tier = g.var(&tier_key(s)).max(1) as u8;
            cx.set_key(g, tier_key(s), 0);
            for _ in 0..self.deal {
                let Some(id) = self.top(g, cx, tier) else {
                    break;
                };
                cx.move_card(g, &id, &deck(tier), &packet(s), false)?;
            }
        }
        Ok(())
    }

    fn legal(&self, phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        match phase {
            0 => {
                out.push(Action::Tier { seat, tier: 1 });
                out.push(Action::Tier { seat, tier: 2 });
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
                if g.resource(s, "credits") >= self.keep_cost || g.var(opposition::FREE_LEFT) > 0 {
                    for id in g.pile(&drafted(s)) {
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
                for tier in [1, 2] {
                    if can_draw(w, g, s, tier, self.draw_cost) {
                        out.push(Action::Draw { seat, tier });
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
                | (4, Action::Play { .. } | Action::Use { .. } | Action::Draw { .. })
        )
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let s = a.seat() as usize;
        match a {
            Action::Tier { tier, .. } => {
                if !(1..=2).contains(tier) || (*tier == 2 && !ai::tier2(&w, g, s)) {
                    return Err("no such tier to draft from".into());
                }
                cx.set_key(g, tier_key(s), *tier as i64);
                g.seats[s].passed = true;
            }
            Action::Draft { card, .. } => {
                if g.var(&picked_key(s)) > 0 {
                    return Err("already took a card from this hand".into());
                }
                cx.move_card(g, card, &packet(s), &drafted(s), false)?;
                cx.set_key(g, picked_key(s), 1);
                if self.all_picked(g) {
                    let n = g.seats.len();
                    let packets: Vec<Vec<String>> = (0..n).map(|p| g.pile(&packet(p)).to_vec()).collect();
                    // As in Terraforming Mars, the direction alternates by round.
                    let step = if g.round % 2 == 1 { 1 } else { n - 1 };
                    for (p, cards) in packets.into_iter().enumerate() {
                        cx.set_pile(g, &packet((p + step) % n), cards);
                    }
                    for p in 0..n {
                        cx.set_key(g, picked_key(p), 0);
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
                let free = g.var(opposition::FREE_LEFT);
                if free > 0 {
                    cx.set_key(g, opposition::FREE_LEFT.into(), free - 1);
                } else if g.resource(s, "credits") >= self.keep_cost {
                    cx.add_resource(g, s, "credits", -self.keep_cost);
                } else {
                    return Err("can't afford to keep it".into());
                }
                cx.move_card(g, card, &drafted(s), &hand(s), false)?;
                cx.count(g, &format!("kept.{card}"), 1);
                if g.pile(&drafted(s)).is_empty() {
                    g.seats[s].passed = true;
                }
            }
            Action::Play { card, .. } => self.play(g, cx, s, card)?,
            Action::Use { card, .. } => {
                if !g.pile(&built(s)).contains(card) {
                    return Err(Halt::Illegal(format!("seat {s} has no {card}")));
                }
                if let Some(why) = ability_blocked(&w, g, s, card) {
                    return Err(Halt::Illegal(format!("can't use {card}: {why}")));
                }
                let ab = def(&w, card).ability.as_ref().ok_or("no ability")?;
                for (r, n) in &ab.cost {
                    cx.add_resource(g, s, r, -n);
                }
                cx.exhaust(g, s, card);
                cx.count(g, &format!("used.{card}"), 1);
                effects::run(g, cx, s, &ab.effects, &mut effects::Ctx::default())?;
            }
            Action::Draw { tier, .. } => {
                if !can_draw(&w, g, s, *tier, self.draw_cost) {
                    return Err("can't draw".into());
                }
                cx.add_resource(g, s, "compute", -self.draw_cost);
                let id = self.top(g, cx, *tier).ok_or("no card to draw")?;
                cx.move_card(g, &id, &deck(*tier), &hand(s), false)?;
            }
            _ => return Err(Halt::Illegal(format!("not a card action: {a:?}"))),
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let card = |id: &str, status: &str, cost: Option<i64>| {
            let d = def(w, id);
            let mut text = d.text.clone();
            let mut bits = vec![format!("{:?}", d.kind).to_lowercase()];
            if d.tier > 0 {
                bits.push(format!("tier {}", d.tier));
            } else {
                bits.push("unique".into());
            }
            if !d.tags.is_empty() {
                bits.push(d.tags.join("/"));
            }
            bits.push(format!("{} insight", w.amount(&d.insight)));
            let am = w.amount(&d.antimatter);
            if am > 0 {
                bits.push(format!("+{am} antimatter"));
            }
            if !d.needs.is_empty() {
                let needs: Vec<String> = d.needs.iter().map(|(t, n)| format!("{n} {t}")).collect();
                bits.push(format!("needs {}", needs.join(", ")));
            }
            text = format!("[{}] {text}", bits.join(", "));
            CardView {
                id: id.to_string(),
                name: d.name.clone(),
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
                    card(id, st, Some(play_cost(w, g, s, def(w, id))))
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
                    } else {
                        "built"
                    };
                    card(id, st, None)
                })
                .collect();
            let label = format!("buildings {}/{}", slotted(w, g, s).len(), capacity(w, g, s));
            out.cards(s, "built", &label, row);
            let pk: Vec<CardView> = g.pile(&packet(s)).iter().map(|id| card(id, "draft", None)).collect();
            if !pk.is_empty() {
                out.cards(s, "packet", "draft hand", pk);
            }
            let dr: Vec<CardView> = g
                .pile(&drafted(s))
                .iter()
                .map(|id| card(id, "drafted", Some(self.keep_cost)))
                .collect();
            if !dr.is_empty() {
                out.cards(s, "drafted", "drafted (keep?)", dr);
            }
            out.seat(s, "slots", "building slots", capacity(w, g, s), None, "building slots (AI board + expansions)");
            for tag in crate::content::TAGS {
                let n = tags(w, g, s, tag);
                if n > 0 {
                    out.seat(s, &format!("tag.{tag}"), &format!("{tag} tags"), n, None, "tags on your buildings");
                }
            }
            let nd = g.var(&next_discount(s));
            if nd > 0 {
                out.seat(s, "next_discount", "next building -", nd, None, "your next building this round costs this much less");
            }
            let t = g.var(&tier_key(s));
            if t > 0 {
                out.seat(s, "draft_tier", "drafting tier", t, None, "the tier you are dealt from this round");
            }
        }
        for tier in [1u8, 2] {
            out.pile(&format!("deck.{tier}"), &format!("Tier-{tier} deck"), None, g.pile(&deck(tier)).len(), Vec::new());
            let d = g.pile(&discard_pile(tier));
            out.pile(
                &format!("discard.{tier}"),
                &format!("Tier-{tier} discard"),
                None,
                d.len(),
                d.iter().map(|id| card(id, "discarded", None)).collect(),
            );
        }
    }
}
