//! Alignment and the end of the game: the Alignment track (the Undecided
//! and the Unity and Discord buckets, fed with Sway), tipping and the Unity
//! and Discord decks, the +1 Discord antimatter action, and the win and loss
//! conditions, checked after every action and every automatic phase.
//!
//! Replaces human opposition (rules revision 5).

use crate::action::Action;
use crate::content::{AlignOp, AlignmentCard, Passive, Side};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{adversary, ai, cards, threats};

const PARAMS: [ParamSpec; 12] = [
    ParamSpec::int("undecided_per_player", 1, 50, "Sway in the Undecided at the start, per player"),
    ParamSpec::int("tip_per_player", 1, 50, "a bucket tips when it holds this many Sway per player"),
    ParamSpec::int("momentum_every", 0, 10, "Momentum: a side's bucket holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off)"),
    ParamSpec::int("troop", 0, 10, "Sway toward Unity for removing an adversary troop"),
    ParamSpec::int("threat_discord", 0, 10, "Sway toward Discord when a threat executes uncountered"),
    ParamSpec::int("adversary_discord", 0, 10, "Sway toward Discord the adversary adds at the start of every round"),
    ParamSpec::int("antimatter_discord", 0, 5, "the antimatter action: Sway toward Discord to produce 1 antimatter (0: no such action)"),
    ParamSpec::int("antimatter_uses", 0, 5, "times each player may take the antimatter action per round"),
    ParamSpec::list("unity_ease", 0, 50, "Unity tip counts at which the clear-board bar drops a level (the first also starts the stronger Unity cards)"),
    ParamSpec::list("discord_escalate", 0, 50, "Discord tip counts at which the adversary escalates a difficulty level (the first also starts the stronger Discord cards)"),
    ParamSpec::int("unity_win", 0, 50, "Unity tips that win outright (0: never)"),
    ParamSpec::int("clear_win_stage", 1, 3, "the bar-0 win counts only once this round's adversary card is of this stage (1: any time)"),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto("sway", "the round's Sway: the adversary toward Discord, AI boards toward Unity, then ongoing cards"),
    PhaseSpec::decision("act", "the antimatter action: sway toward Discord to produce 1 antimatter"),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "alignment",
    summary: "The Alignment track (Sway, the Undecided, Unity and Discord buckets that tip into cards), and winning and losing",
    requires: &["adversary", "ai", "board"],
    variants: &[
        VariantDef {
            name: "tug",
            summary: "Tug of war: Sway with the Undecided empty comes from the opposite bucket",
            params: &PARAMS,
            phases: PHASES,
            rng: &["alignment"],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "still",
            summary: "Sway with the Undecided empty does nothing",
            params: &PARAMS,
            phases: PHASES,
            rng: &["alignment"],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "off",
            summary: "No Sway moves and nothing tips (for measuring Alignment); winning and losing stay",
            params: &PARAMS,
            phases: PHASES,
            rng: &["alignment"],
            uses_cards: false,
            build,
        },
    ],
};

/// Every variant is one struct: `sway` and the action read the variant.
fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    for name in ["unity_ease", "discord_escalate"] {
        let v = b.list(name)?;
        if v.is_empty() || v.iter().any(|&x| x < 1) || v.windows(2).any(|p| p[0] > p[1]) {
            return Err(format!(
                "alignment.{name}: give tip counts of 1 or more, in order"
            ));
        }
    }
    Ok(Box::new(Alignment))
}

#[derive(Debug)]
struct Alignment;

/// Sway in the Undecided.
pub const UNDECIDED: &str = "alignment.undecided";
/// Credit income every player has from Unity cards.
const INCOME_BONUS: &str = "alignment.income_bonus";
/// Free card purchases left (Unity cards).
const FREE_LEFT: &str = "alignment.free_purchases";
/// Insight every player gains each round from Unity cards.
const INSIGHT_BONUS: &str = "alignment.insight_bonus";
/// Ongoing cards in play.
pub const ONGOING: &str = "alignment.ongoing";

/// Credit income every player has from Unity cards (economy's upkeep).
pub fn income_bonus(g: &GameState) -> i64 {
    g.var(INCOME_BONUS)
}

/// Insight every player gains each round from Unity cards.
pub fn insight_bonus(g: &GameState) -> i64 {
    g.var(INSIGHT_BONUS)
}

/// Free card purchases left from Unity cards.
pub fn free_purchases(g: &GameState) -> i64 {
    g.var(FREE_LEFT)
}

/// Uses up one free card purchase, if any is left; returns whether it did.
pub fn take_free_purchase(g: &mut GameState, cx: &mut Cx) -> bool {
    let free = g.var(FREE_LEFT);
    if free <= 0 {
        return false;
    }
    cx.set_key(g, FREE_LEFT.into(), free - 1);
    true
}

/// Sway in a side's bucket.
pub fn bucket(side: Side) -> &'static str {
    match side {
        Side::Unity => "alignment.unity",
        Side::Discord => "alignment.discord",
    }
}

/// A side's tip count (only counts up).
pub fn tips_key(side: Side) -> &'static str {
    match side {
        Side::Unity => "alignment.unity_tips",
        Side::Discord => "alignment.discord_tips",
    }
}

fn deck(side: Side, tier: u8) -> String {
    format!("alignment.deck.{}.{tier}", side.name())
}

/// A side's resolved one-shot cards (top: the latest).
pub fn resolved(side: Side) -> String {
    format!("alignment.resolved.{}", side.name())
}

fn used_key(seat: usize) -> String {
    format!("alignment.used.{seat}")
}

fn card<'w>(w: &World<'w>, id: &str) -> &'w AlignmentCard {
    &w.content.alignment[w
        .content
        .alignment_index(id)
        .expect("alignment card in content")]
}

/// Sway a side's bucket holds before it tips (Momentum applied).
pub fn capacity(w: &World, g: &GameState, side: Side) -> i64 {
    let seats = g.seats.len().max(1) as i64;
    let full = w.int("alignment.tip_per_player") * seats;
    let every = w.int("alignment.momentum_every");
    if every <= 0 {
        return full;
    }
    (full - g.var(tips_key(side)) / every).max(seats)
}

/// How many of `thresholds` a tip count has reached.
fn reached(thresholds: &[i64], tips: i64) -> i64 {
    thresholds.iter().filter(|&&t| tips >= t).count() as i64
}

/// The clear-board bar level, from Unity tips (0: no adversary pieces
/// left; see `adversary::under_bar`).
pub fn bar_level(w: &World, g: &GameState) -> i64 {
    reached(w.list("alignment.unity_ease"), g.var(tips_key(Side::Unity)))
}

/// Moves `n` Sway toward `side`, one at a time: from the Undecided, or with
/// it empty (variant `tug`) from the opposite bucket. A bucket that fills
/// tips once the action or phase has resolved.
pub fn sway(g: &mut GameState, cx: &mut Cx, side: Side, n: i64) {
    if n <= 0 {
        return;
    }
    let variant = cx.world.variant("alignment");
    if variant == Some("off") {
        return;
    }
    let tug = variant == Some("tug");
    let mut moved = 0;
    for _ in 0..n {
        if g.var(UNDECIDED) > 0 {
            cx.add_key(g, UNDECIDED.into(), -1);
        } else if tug && g.var(bucket(side.other())) > 0 {
            cx.add_key(g, bucket(side.other()).into(), -1);
        } else {
            continue;
        }
        cx.add_key(g, bucket(side).into(), 1);
        moved += 1;
    }
    if moved > 0 {
        cx.tally(g, &format!("alignment.sway_{}", side.name()), moved);
    }
}

/// Sway toward Unity for a removed adversary troop (plus the remover's
/// Unity bonus).
pub fn for_troop_removed(g: &mut GameState, cx: &mut Cx, seat: Option<usize>) {
    let w = cx.world;
    let base = w.int("alignment.troop");
    let bonus: i64 = seat.map_or(0, |s| {
        cards::passives(&w, g, s)
            .iter()
            .map(|p| match p {
                Passive::UnityBonus { amount } => *amount,
                _ => 0,
            })
            .sum()
    });
    sway(g, cx, Side::Unity, base + bonus);
}

/// Whether the adversary's presence is under the bar for `level`. The
/// level-0 bar (no adversary pieces) counts only once a card of stage
/// `clear_win_stage` has been revealed (the stages are stacked and public).
fn under_bar(w: &World, g: &GameState, level: i64) -> bool {
    if level == 0 && g.var(adversary::STAGE) < w.int("alignment.clear_win_stage") {
        return false;
    }
    adversary::under_bar(w, g, level)
}

/// Resolves one card effect. Team choices are asked of seat 0.
fn resolve(g: &mut GameState, cx: &mut Cx, op: &AlignOp) -> Res {
    let w = cx.world;
    match op {
        AlignOp::Income { credits } => cx.add_key(g, INCOME_BONUS.into(), *credits),
        AlignOp::InsightIncome { insight } => cx.add_key(g, INSIGHT_BONUS.into(), *insight),
        AlignOp::Credits { credits } => {
            for s in 0..g.seats.len() {
                cx.add_resource(g, s, "credits", *credits);
            }
        }
        AlignOp::FreePurchases { count } => cx.add_key(g, FREE_LEFT.into(), *count),
        AlignOp::CounterThreat => {
            let active = threats::active(g);
            let opts = active.iter().map(|(t, _)| opt(t.clone())).collect();
            if let Some(i) = cx.choose(0, "Unity: counter which threat?", opts)? {
                threats::counter(g, cx, &active[i].0);
            }
        }
        AlignOp::CounterAll => {
            for (t, _) in threats::active(g) {
                threats::counter(g, cx, &t);
            }
        }
        AlignOp::Pushback { ambassadors } => {
            for c in 0..w.map.countries.len() {
                for _ in 0..*ambassadors {
                    adversary::pushback(g, cx, c)?;
                }
            }
        }
        AlignOp::Sway { side, amount } => sway(g, cx, *side, *amount),
        AlignOp::Threats { count } => threats::deal_extra(g, cx, *count),
        AlignOp::Infiltrate { amount } => adversary::infiltrate(g, cx, *amount),
        AlignOp::Build => adversary::build_all(g, cx, None)?,
        AlignOp::Spread => adversary::spread(g, cx, None)?,
    }
    Ok(())
}

impl Alignment {
    /// Draws the next card of `side`: the stronger tier once the side's tip
    /// count has reached its first threshold, else tier 1; the other tier
    /// if that deck is empty; with both empty, the side's resolved cards
    /// are shuffled back into their decks first.
    fn draw(&self, g: &mut GameState, cx: &mut Cx, side: Side) -> Option<(String, String)> {
        let w = cx.world;
        let first = match side {
            Side::Unity => w.list("alignment.unity_ease")[0],
            Side::Discord => w.list("alignment.discord_escalate")[0],
        };
        let want: u8 = if g.var(tips_key(side)) >= first { 2 } else { 1 };
        for pass in 0..2 {
            for tier in [want, 3 - want] {
                if let Some(id) = g.pile(&deck(side, tier)).first().cloned() {
                    return Some((id, deck(side, tier)));
                }
            }
            if pass == 0 {
                let back = g.pile(&resolved(side)).to_vec();
                if back.is_empty() {
                    return None;
                }
                cx.set_pile(g, &resolved(side), Vec::new());
                for tier in [1u8, 2] {
                    let mut ids: Vec<String> = back
                        .iter()
                        .filter(|id| card(&w, id).tier == tier)
                        .cloned()
                        .collect();
                    shuffle(&mut ids, cx.rng(g, "alignment"));
                    cx.set_pile(g, &deck(side, tier), ids);
                }
                cx.note(None, || {
                    format!("The resolved {} cards are shuffled back", side.name())
                });
            }
        }
        None
    }

    /// A bucket tips: one card of its side resolves, the side's tip count
    /// goes up by one, and the bucket's Sway returns to the Undecided.
    fn tip(&self, g: &mut GameState, cx: &mut Cx, side: Side) -> Res {
        let w = cx.world;
        let round = g.round;
        cx.note(None, || format!("The {} bucket tips", side.name()));
        if let Some((id, from)) = self.draw(g, cx, side) {
            let c = card(&w, &id);
            if c.ongoing {
                cx.move_card(g, &id, &from, ONGOING, false).unwrap();
                cx.note(None, || {
                    format!("{} ({}) is now in play: {}", c.name, side.name(), c.text)
                });
            } else {
                cx.move_card(g, &id, &from, &resolved(side), true).unwrap();
                cx.note(None, || {
                    format!("{} ({}) resolves: {}", c.name, side.name(), c.text)
                });
                for op in &c.effects {
                    resolve(g, cx, op)?;
                }
            }
            cx.tally(g, &format!("alignment.card.{id}"), 1);
        }
        let tips = g.var(tips_key(side)) + 1;
        cx.set_key(g, tips_key(side).into(), tips);
        cx.tally(g, &format!("alignment.tips_{}", side.name()), 1);
        cx.tally(g, &format!("alignment.tip_{}.r{round:02}", side.name()), 1);
        let back = g.var(bucket(side));
        cx.set_key(g, bucket(side).into(), 0);
        cx.add_key(g, UNDECIDED.into(), back);
        if side == Side::Discord {
            let esc = reached(w.list("alignment.discord_escalate"), tips);
            if esc > g.var(adversary::ESCALATION) {
                cx.set_key(g, adversary::ESCALATION.into(), esc);
                cx.note(None, || {
                    format!(
                        "Discord escalates the adversary (difficulty {})",
                        adversary::difficulty(&w, g)
                    )
                });
            }
        }
        Ok(())
    }
}

impl Module for Alignment {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        let start = w.int("alignment.undecided_per_player") * g.seats.len() as i64;
        cx.set_key(g, UNDECIDED.into(), start);
        for side in [Side::Unity, Side::Discord] {
            cx.set_key(g, bucket(side).into(), 0);
            cx.set_key(g, tips_key(side).into(), 0);
            for tier in [1u8, 2] {
                let mut ids: Vec<String> = w
                    .content
                    .alignment
                    .iter()
                    .filter(|c| c.side == side && c.tier == tier)
                    .map(|c| c.id.clone())
                    .collect();
                shuffle(&mut ids, cx.rng(g, "alignment"));
                cx.set_pile(g, &deck(side, tier), ids);
            }
        }
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            if g.var(&used_key(s)) != 0 {
                cx.set_key(g, used_key(s), 0);
            }
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        sway(g, cx, Side::Discord, w.int("alignment.adversary_discord"));
        for s in 0..g.seats.len() {
            sway(g, cx, Side::Unity, ai::board(&w, s).unity);
        }
        for id in g.pile(ONGOING).to_vec() {
            for op in &card(&w, &id).effects {
                resolve(g, cx, op)?;
            }
        }
        Ok(())
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        if w.int("alignment.antimatter_discord") > 0
            && w.variant("alignment") != Some("off")
            && g.var(&used_key(seat as usize)) < w.int("alignment.antimatter_uses")
        {
            out.push(Action::Antimatter { seat });
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Antimatter { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let s = a.seat() as usize;
        let n = w.int("alignment.antimatter_discord");
        if n <= 0
            || w.variant("alignment") == Some("off")
            || g.var(&used_key(s)) >= w.int("alignment.antimatter_uses")
        {
            return Err(Halt::Illegal("no antimatter action left this round".into()));
        }
        cx.add_key(g, used_key(s), 1);
        cx.add_resource(g, s, "antimatter", 1);
        sway(g, cx, Side::Discord, n);
        cx.count(g, "antimatter_actions", 1);
        Ok(())
    }

    fn after(&self, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        // Full buckets tip, Discord first; a tip's card can fill a bucket again.
        let tips = if w.variant("alignment") == Some("off") {
            0
        } else {
            64
        };
        'tips: for _ in 0..tips {
            for side in [Side::Discord, Side::Unity] {
                if g.var(bucket(side)) >= capacity(&w, g, side) {
                    self.tip(g, cx, side)?;
                    if g.is_over() {
                        return Ok(());
                    }
                    continue 'tips;
                }
            }
            break;
        }
        // Losses first.
        if let Some((reason, detail)) = adversary::loss(&w, g) {
            cx.end(g, false, reason, detail);
            return Ok(());
        }
        let unity = g.var(tips_key(Side::Unity));
        let outright = w.int("alignment.unity_win");
        if outright > 0 && unity >= outright {
            cx.end(g, true, "unity", format!("{unity} Unity tips win outright"));
            return Ok(());
        }
        let level = bar_level(&w, g);
        if under_bar(&w, g, level) {
            cx.end(
                g,
                true,
                "purge",
                format!(
                    "the adversary fell under the bar for {} Unity tips",
                    g.var(tips_key(Side::Unity))
                ),
            );
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let seats = g.seats.len() as i64;
        let total = w.int("alignment.undecided_per_player") * seats;
        out.global(
            "undecided",
            "the Undecided",
            g.var(UNDECIDED),
            Some(total),
            "Sway not yet in either bucket",
        );
        out.global(
            "unity",
            "Unity bucket",
            g.var(bucket(Side::Unity)),
            Some(capacity(w, g, Side::Unity)),
            "Sway toward the good AIs; tips into a Unity card when full",
        );
        out.global(
            "discord",
            "Discord bucket",
            g.var(bucket(Side::Discord)),
            Some(capacity(w, g, Side::Discord)),
            "Sway toward the evil AI; tips into a Discord card when full",
        );
        let next = |list: &[i64], tips: i64| list.iter().copied().find(|&t| t > tips);
        let ut = g.var(tips_key(Side::Unity));
        let dt = g.var(tips_key(Side::Discord));
        out.global(
            "unity_tips",
            "Unity tips",
            ut,
            next(w.list("alignment.unity_ease"), ut),
            "Unity tips so far; the clear-board bar drops at each Unity threshold",
        );
        out.global(
            "discord_tips",
            "Discord tips",
            dt,
            next(w.list("alignment.discord_escalate"), dt),
            "Discord tips so far; the adversary escalates at each Discord threshold",
        );
        let level = bar_level(w, g);
        let mut text = adversary::win_text(w, level);
        let stage = w.int("alignment.clear_win_stage");
        if level == 0 && g.var(adversary::STAGE) < stage {
            text += &format!(" (from adversary stage {stage})");
        }
        out.global(
            "bar",
            "clear-board bar",
            level,
            Some(w.list("alignment.unity_ease").len() as i64),
            &text,
        );
        let cities = (0..g.hexes.len()).filter(|&h| is_city(w, h)).count() as i64;
        let bad = (0..g.hexes.len())
            .filter(|&h| is_city(w, h) && corrupted(g, h))
            .count() as i64;
        out.global(
            "corrupted_cities",
            "corrupted cities",
            bad,
            Some(cities),
            "players lose when every city is corrupted",
        );
        if w.int("alignment.unity_win") > 0 {
            out.global(
                "unity_win",
                "Unity tips to win",
                w.int("alignment.unity_win"),
                None,
                "this many Unity tips win outright",
            );
        }
        if g.var(INCOME_BONUS) > 0 {
            out.global(
                "income_bonus",
                "Unity income bonus",
                g.var(INCOME_BONUS),
                None,
                "credit income every player has from Unity cards",
            );
        }
        if g.var(INSIGHT_BONUS) > 0 {
            out.global(
                "insight_bonus",
                "Unity insight bonus",
                g.var(INSIGHT_BONUS),
                None,
                "insight every player gains each round from Unity cards",
            );
        }
        if g.var(FREE_LEFT) > 0 {
            out.global(
                "free_purchases",
                "free purchases",
                g.var(FREE_LEFT),
                None,
                "card purchases that are free",
            );
        }
        let show = |id: &str, status: &str| {
            let c = card(w, id);
            CardView {
                id: id.to_string(),
                name: c.name.clone(),
                text: format!("{} {}: {}", c.side.name(), c.tier, c.text),
                status: status.to_string(),
                cost: None,
            }
        };
        for side in [Side::Unity, Side::Discord] {
            let n = g.pile(&deck(side, 1)).len() + g.pile(&deck(side, 2)).len();
            let label = match side {
                Side::Unity => "Unity deck",
                Side::Discord => "Discord deck",
            };
            out.pile(&format!("deck.{}", side.name()), label, None, n, Vec::new());
            let done = g.pile(&resolved(side));
            let label = match side {
                Side::Unity => "Unity cards resolved",
                Side::Discord => "Discord cards resolved",
            };
            out.pile(
                &format!("resolved.{}", side.name()),
                label,
                None,
                done.len(),
                done.iter().map(|id| show(id, "resolved")).collect(),
            );
        }
        let on = g.pile(ONGOING);
        out.pile(
            "ongoing",
            "Ongoing cards",
            None,
            on.len(),
            on.iter().map(|id| show(id, "ongoing")).collect(),
        );
        out.bookkeeping(&["alignment.used."]);
        let uses = w.int("alignment.antimatter_uses");
        if w.int("alignment.antimatter_discord") > 0 {
            for s in 0..g.seats.len() {
                out.seat(
                    s,
                    "antimatter_left",
                    "antimatter actions left",
                    (uses - g.var(&used_key(s))).max(0),
                    Some(uses),
                    "sway toward Discord to produce 1 antimatter",
                );
            }
        }
    }
}
