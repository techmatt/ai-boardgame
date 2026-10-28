//! The adversary: its deck (the clock), the explore -> build -> ravage
//! pipeline, corruption, the Militarist's innate rules and the difficulty
//! levels. Also the rules for destroying pieces on either side, which
//! cards, threats and the pipeline share.

use crate::content::{AdversaryCard, Passive};
use crate::module::{
    Build, Cx, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{board, cards, opposition};

pub static DEF: ModuleDef = ModuleDef {
    id: "adversary",
    summary: "The adversary deck and pipeline (explore, build, ravage), corruption, innate rules, difficulty",
    requires: &["board", "cards", "opposition"],
    variants: &[VariantDef {
        name: "militarist",
        summary: "The Militarist: Momentum and Escalation",
        params: &[
            i("difficulty", 0, 4, "difficulty level 0-4 (each level includes the ones below)"),
            l("stage_cards", "adversary cards dealt from stage I, II and III (before difficulty)"),
            i("drone_health", 1, 10, "damage to destroy a drone"),
            i("outpost_health", 1, 10, "damage to destroy an outpost"),
            i("foundry_health", 1, 10, "damage to destroy a foundry"),
            i("drone_damage", 0, 10, "ravage damage a drone deals"),
            i("outpost_damage", 0, 10, "ravage damage an outpost deals"),
            i("foundry_damage", 0, 10, "ravage damage a foundry deals"),
            i("population_health", 1, 10, "damage to destroy a population"),
            i("building_health", 1, 10, "damage to destroy a factory or data centre"),
            i("defense_value", 0, 10, "ravage damage one defense token prevents"),
            i("corruption_at", 1, 20, "a ravage dealing at least this much damage adds corruption"),
            i("corruption_per_ravage", 0, 5, "corruption a ravage adds"),
            i("cascade", 0, 5, "corruption a cascade adds to an adjacent hex"),
            i("pool_per_country", 0, 20, "corruption pool size per country"),
            i("pool_base", 0, 20, "corruption pool size on top of the per-country part"),
            i("explore_drones", 0, 5, "drones an exploring hex gains"),
            i("build_pieces", 0, 5, "outposts or foundries a building hex gains"),
            i("momentum_drones", 0, 5, "Momentum: drones added next to a ravage that destroyed population"),
            i("escalation_outposts", 0, 5, "Escalation: outposts each country's busiest uplink gains"),
            i("counter_bonus", 0, 10, "difficulty 1+: added to every threat's first counter requirement"),
            i("stage1_removed", 0, 3, "difficulty 2+: stage-I cards removed"),
            i("foundry_bonus", 0, 10, "difficulty 3+: extra ravage damage in hexes with a foundry"),
            i("tier2_early", 0, 3, "difficulty 4+: tier-2 threats start this many rounds sooner"),
            i("pool_reduction", 0, 10, "difficulty 4+: corruption pool smaller by this per country"),
        ],
        phases: &[
            ph("ravage", "ravage with the card in the ravage slot"),
            ph("build", "build with the card in the build slot"),
            ph("explore", "reveal a card into the explore slot and explore (and escalate)"),
            ph("slide", "slide the cards one slot along; the ravage card is discarded"),
        ],
        rng: &["adversary"],
        uses_cards: false,
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

const fn l(name: &'static str, doc: &'static str) -> ParamSpec {
    ParamSpec {
        name,
        kind: ParamKind::List,
        min: 0,
        max: 20,
        doc,
    }
}

const fn ph(name: &'static str, doc: &'static str) -> PhaseSpec {
    PhaseSpec {
        name,
        kind: PhaseKind::Auto,
        doc,
    }
}

pub const DECK: &str = "adversary.deck";
pub const DISCARD: &str = "adversary.discard";
/// The pipeline slots, in the order a card moves through them.
pub const SLOTS: [&str; 3] = ["adversary.explore", "adversary.build", "adversary.ravage"];
pub const POOL: &str = "adversary.pool";
pub const STAGE: &str = "adversary.stage";
/// How many top cards of the deck are shown (Recon Sweep).
pub const PEEK: &str = "adversary.peek";

pub fn killed_foundry(seat: usize) -> String {
    format!("adversary.killed_foundry.{seat}")
}

/// Whether the difficulty is at least `n`.
pub fn level(w: &World, n: i64) -> bool {
    w.int("adversary.difficulty") >= n
}

pub fn health(w: &World, kind: &str) -> i64 {
    match kind {
        DRONE => w.int("adversary.drone_health"),
        OUTPOST => w.int("adversary.outpost_health"),
        FOUNDRY => w.int("adversary.foundry_health"),
        POPULATION => w.int("adversary.population_health"),
        _ => w.int("adversary.building_health"),
    }
}

fn piece_damage(w: &World, kind: &str) -> i64 {
    match kind {
        DRONE => w.int("adversary.drone_damage"),
        OUTPOST => w.int("adversary.outpost_damage"),
        _ => w.int("adversary.foundry_damage"),
    }
}

/// The corruption pool at the start, for this many countries.
pub fn start_pool(w: &World) -> i64 {
    let n = w.map.countries.len() as i64;
    let mut pool = w.int("adversary.pool_per_country") * n + w.int("adversary.pool_base");
    if level(w, 4) {
        pool -= w.int("adversary.pool_reduction") * n;
    }
    pool.max(0)
}

/// Which tier of threats is dealt now: tier 2 once stage II has begun
/// (at difficulty 4, `tier2_early` reveals sooner).
pub fn threat_tier(w: &World, g: &GameState) -> u8 {
    if g.var(STAGE) >= 2 {
        return 2;
    }
    if level(w, 4) {
        let early = w.int("adversary.tier2_early") as usize;
        let soon = g.pile(DECK).iter().take(early).any(|id| card(w, id).stage >= 2);
        if soon {
            return 2;
        }
    }
    1
}

fn card<'w>(w: &World<'w>, id: &str) -> &'w AdversaryCard {
    &w.content.adversary[w.content.adversary_index(id).expect("adversary card in content")]
}

pub fn card_text(c: &AdversaryCard) -> String {
    let t: Vec<&str> = c.terrains.iter().map(|t| t.name()).collect();
    format!(
        "stage {}: {}{}",
        c.stage,
        t.join(" + "),
        if c.escalation { "; escalation" } else { "" }
    )
}

#[derive(Debug)]
struct Adversary {
    stage_cards: Vec<i64>,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let stage_cards = b.list("stage_cards")?;
    if stage_cards.len() != 3 {
        return Err("stage_cards: give 3 counts (stage I, II, III)".into());
    }
    for (s, &n) in stage_cards.iter().enumerate() {
        let have = b
            .world
            .content
            .adversary
            .iter()
            .filter(|c| c.stage as usize == s + 1)
            .count() as i64;
        if n > have {
            return Err(format!("stage_cards: stage {} has only {have} cards", s + 1));
        }
    }
    Ok(Box::new(Adversary { stage_cards }))
}

impl Module for Adversary {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        cx.set_key(g, POOL.into(), start_pool(&w));
        let mut deck = Vec::new();
        for stage in 1..=3u8 {
            let mut ids: Vec<String> = w
                .content
                .adversary
                .iter()
                .filter(|c| c.stage == stage)
                .map(|c| c.id.clone())
                .collect();
            shuffle(&mut ids, cx.rng(g, "adversary"));
            let mut n = self.stage_cards[stage as usize - 1];
            if stage == 1 && level(&w, 2) {
                n = (n - w.int("adversary.stage1_removed")).max(0);
            }
            deck.extend(ids.into_iter().take(n as usize));
        }
        cx.set_pile(g, DECK, deck);
        // Opening: reveal the first card and explore, then slide.
        if reveal(g, cx) {
            explore(g, cx);
        }
        slide(g, cx);
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            cx.set_key(g, killed_foundry(s), 0);
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match phase {
            0 => {
                if let Some(id) = g.pile(SLOTS[2]).first().cloned() {
                    let c = card(&w, &id);
                    cx.note(None, || format!("Ravage: {}", card_text(c)));
                    let hexes: Vec<usize> = (0..g.hexes.len())
                        .filter(|&h| has_terrain(&w, h, &c.terrains) && has_adversary(g, h))
                        .collect();
                    for h in hexes {
                        ravage(g, cx, h)?;
                    }
                }
            }
            1 => {
                if let Some(id) = g.pile(SLOTS[1]).first().cloned() {
                    let c = card(&w, &id);
                    cx.note(None, || format!("Build: {}", card_text(c)));
                    let hexes: Vec<usize> = (0..g.hexes.len())
                        .filter(|&h| has_terrain(&w, h, &c.terrains) && has_adversary(g, h))
                        .collect();
                    for h in hexes {
                        build_hex(g, cx, h);
                    }
                }
            }
            2 => {
                if reveal(g, cx) {
                    explore(g, cx);
                }
            }
            _ => slide(g, cx),
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let show = |id: &str, status: &str| {
            let c = card(w, id);
            CardView {
                id: id.to_string(),
                name: c.name.clone(),
                text: card_text(c),
                status: status.to_string(),
                cost: None,
            }
        };
        let deck = g.pile(DECK);
        let peek = g.var(PEEK).max(0) as usize;
        out.pile(
            "deck",
            "Adversary deck (the clock)",
            None,
            deck.len(),
            deck.iter().take(peek).map(|id| show(id, "next")).collect(),
        );
        for (slot, label) in SLOTS.iter().zip(["Explore", "Build", "Ravage"]).rev() {
            let cards: Vec<CardView> = g.pile(slot).iter().map(|id| show(id, "in slot")).collect();
            out.pile(&slot["adversary.".len()..], label, None, cards.len(), cards);
        }
        let disc = g.pile(DISCARD);
        out.pile(
            "discard",
            "Adversary discard",
            None,
            disc.len(),
            disc.iter().map(|id| show(id, "done")).collect(),
        );
        out.global(
            "pool",
            "corruption pool",
            g.var(POOL),
            Some(start_pool(w)),
            "corruption left in the pool; players lose when it runs out",
        );
        out.global("stage", "adversary stage", g.var(STAGE), Some(3), "stage of the last revealed adversary card");
        out.global(
            "difficulty",
            "difficulty",
            w.int("adversary.difficulty"),
            Some(4),
            "difficulty level (config)",
        );
        out.global(
            "deck",
            "adversary cards left",
            deck.len() as i64,
            None,
            "players lose if a card must be revealed and the deck is empty",
        );
        if peek > 0 {
            out.global("peek", "cards shown", g.var(PEEK), None, "Recon Sweep: the top cards of the deck are shown");
        }
    }
}

/// Reveals the top card into the explore slot. Ends the game if the deck is empty.
fn reveal(g: &mut GameState, cx: &mut Cx) -> bool {
    let w = cx.world;
    let Some(id) = g.pile(DECK).first().cloned() else {
        cx.end(
            g,
            false,
            "deck_empty",
            "an adversary card had to be revealed and the deck was empty".into(),
        );
        return false;
    };
    cx.move_card(g, &id, DECK, SLOTS[0], true).expect("top of the deck");
    let c = card(&w, &id);
    cx.set_key(g, STAGE.into(), c.stage as i64);
    let peek = g.var(PEEK);
    if peek > 0 {
        cx.set_key(g, PEEK.into(), peek - 1);
    }
    true
}

fn explore(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let Some(id) = g.pile(SLOTS[0]).first().cloned() else {
        return;
    };
    let c = card(&w, &id);
    cx.note(None, || format!("Explore: {}", card_text(c)));
    let n = w.int("adversary.explore_drones");
    let shields: Vec<usize> = (0..g.seats.len())
        .filter(|&s| cards::has_passive(&w, g, s, |p| matches!(p, Passive::ExploreShield)))
        .collect();
    let mut hits = Vec::new();
    for h in 0..g.hexes.len() {
        if !has_terrain(&w, h, &c.terrains) {
            continue;
        }
        let source = |x: usize| g.tokens(x, OUTPOST) + g.tokens(x, FOUNDRY) > 0;
        if !(w.map.hexes[h].uplink || next_to(&w, h, source)) {
            continue;
        }
        if shields.iter().any(|&s| g.tokens(h, &processor(s)) > 0) {
            cx.note(Some(h), || format!("Firewall Node: no drone at {}", hex_name(&w, h)));
            continue;
        }
        hits.push(h);
    }
    for h in hits {
        cx.add_tokens(g, h, DRONE, n);
        cx.adversary_ops(g, 1);
    }
    if c.escalation {
        escalate(g, cx);
    }
}

/// Militarist Escalation: each country's uplink with the most drones gains
/// outposts (ties: lower hex number).
fn escalate(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let n = w.int("adversary.escalation_outposts");
    for c in 0..w.map.countries.len() {
        let mut best: Option<usize> = None;
        for &h in &w.map.countries[c].hexes {
            if !w.map.hexes[h].uplink {
                continue;
            }
            if best.is_none_or(|b| g.tokens(h, DRONE) > g.tokens(b, DRONE)) {
                best = Some(h);
            }
        }
        if let Some(h) = best {
            cx.note(Some(h), || format!("Escalation: outpost at {}", hex_name(&w, h)));
            cx.add_tokens(g, h, OUTPOST, n);
            cx.adversary_ops(g, 1);
        }
    }
}

fn slide(g: &mut GameState, cx: &mut Cx) {
    if let Some(id) = g.pile(SLOTS[2]).first().cloned() {
        cx.move_card(g, &id, SLOTS[2], DISCARD, true).unwrap();
    }
    for k in [1, 0] {
        if let Some(id) = g.pile(SLOTS[k]).first().cloned() {
            cx.move_card(g, &id, SLOTS[k], SLOTS[k + 1], true).unwrap();
        }
    }
}

/// Build in one hex: a foundry if it has more outposts than foundries,
/// otherwise an outpost.
pub fn build_hex(g: &mut GameState, cx: &mut Cx, h: usize) {
    let kind = if g.tokens(h, OUTPOST) > g.tokens(h, FOUNDRY) {
        FOUNDRY
    } else {
        OUTPOST
    };
    let n = cx.world.int("adversary.build_pieces");
    cx.add_tokens(g, h, kind, n);
    cx.adversary_ops(g, 1);
}

/// Adds up to `n` corruption from the pool to hex `h`; returns how much.
pub fn add_corruption(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> i64 {
    let pool = g.var(POOL);
    let k = n.min(pool).max(0);
    if k == 0 {
        return 0;
    }
    cx.set_key(g, POOL.into(), pool - k);
    cx.add_tokens(g, h, CORRUPTION, k);
    cx.adversary_ops(g, 1);
    cx.tally(g, "adversary.corruption_added", k);
    k
}

/// Returns `n` corruption from hex `h` to the pool.
pub fn remove_corruption(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> i64 {
    let k = cx.add_tokens(g, h, CORRUPTION, -n);
    cx.add_key(g, POOL.into(), -k);
    -k
}

/// A cascade: corruption on an adjacent hex of the players' choice.
pub fn cascade(g: &mut GameState, cx: &mut Cx, from: usize) -> Res {
    let w = cx.world;
    let hexes = w.map.hexes[from].neighbours.clone();
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!("Corruption cascades from {}: onto which adjacent hex?", hex_name(&w, from));
    if let Some(i) = cx.choose(0, &prompt, opts)? {
        add_corruption(g, cx, hexes[i], w.int("adversary.cascade"));
    }
    Ok(())
}

/// Removes one population from `h`, unless something prevents it (Shelter
/// Order on the country, or a Civic Shelter). Returns whether it was lost.
pub fn lose_population(g: &mut GameState, cx: &mut Cx, h: usize) -> bool {
    let w = cx.world;
    if g.tokens(h, POPULATION) == 0 {
        return false;
    }
    let c = w.map.hexes[h].country as usize;
    if board::flagged(g, "shelter", c) {
        return false;
    }
    if cards::civic_shelter(g, cx, c) {
        cx.note(Some(h), || format!("Civic Shelter saves the population at {}", hex_name(&w, h)));
        return false;
    }
    cx.add_tokens(g, h, POPULATION, -1);
    cx.tally(g, "adversary.population_lost", 1);
    true
}

/// A ravage in hex `h`: the adversary pieces there deal their damage, less
/// defense, to the players' pieces; then corruption and Momentum.
pub fn ravage(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    let w = cx.world;
    let mut raw: i64 = ADVERSARY.iter().map(|k| g.tokens(h, k) * piece_damage(&w, k)).sum();
    if raw == 0 {
        return Ok(());
    }
    if level(&w, 3) && g.tokens(h, FOUNDRY) > 0 {
        raw += w.int("adversary.foundry_bonus");
    }
    let prevented: i64 = (0..g.seats.len())
        .map(|s| g.tokens(h, &defense(s)) * cards::defense_value(&w, g, s))
        .sum();
    let dmg = (raw - prevented).max(0);
    cx.adversary_ops(g, 1);
    cx.note(Some(h), || {
        format!("Ravage at {}: {raw} damage, {prevented} prevented", hex_name(&w, h))
    });
    if dmg == 0 {
        return Ok(());
    }
    let c = w.map.hexes[h].country as usize;
    let sheltered = board::flagged(g, "shelter", c);
    let mut left = dmg;
    let mut lost_population = false;
    loop {
        let mut kinds = Vec::new();
        for kind in [POPULATION, FACTORY, DATA_CENTRE] {
            if g.tokens(h, kind) > 0 && health(&w, kind) <= left && !(kind == POPULATION && sheltered) {
                kinds.push(kind);
            }
        }
        let opts = kinds.iter().map(|k| opt(*k)).collect();
        let prompt = format!("Ravage at {}: {left} damage left. Which piece is lost next?", hex_name(&w, h));
        let Some(i) = cx.choose(0, &prompt, opts)? else {
            break;
        };
        let kind = kinds[i];
        left -= health(&w, kind);
        if kind == POPULATION {
            lost_population |= lose_population(g, cx, h);
        } else {
            cx.add_tokens(g, h, kind, -1);
            cx.tally(g, "adversary.buildings_lost", 1);
        }
    }
    if dmg >= w.int("adversary.corruption_at") {
        let had = g.tokens(h, CORRUPTION) > 0;
        add_corruption(g, cx, h, w.int("adversary.corruption_per_ravage"));
        if had {
            cascade(g, cx, h)?;
        }
    }
    let momentum = w.int("adversary.momentum_drones");
    if lost_population && momentum > 0 {
        let hexes: Vec<usize> = w.map.hexes[h]
            .neighbours
            .iter()
            .copied()
            .filter(|&n| w.map.hexes[n].country as usize == c)
            .collect();
        let opts = hexes.iter().map(|&n| hex_opt(&w, n)).collect();
        let prompt = format!("Momentum: a drone joins next to {}. Onto which adjacent hex?", hex_name(&w, h));
        if let Some(i) = cx.choose(0, &prompt, opts)? {
            cx.add_tokens(g, hexes[i], DRONE, momentum);
            cx.adversary_ops(g, 1);
        }
    }
    Ok(())
}

/// Destroys one adversary piece of `kind` in `h`, scoring opposition for
/// `seat` (if a player destroyed it).
pub fn destroy_piece(g: &mut GameState, cx: &mut Cx, seat: Option<usize>, h: usize, kind: &str) {
    if cx.add_tokens(g, h, kind, -1) == 0 {
        return;
    }
    cx.tally(g, &format!("adversary.{kind}_destroyed"), 1);
    opposition::for_kill(g, cx, seat, kind);
    if let (FOUNDRY, Some(s)) = (kind, seat) {
        cx.set_key(g, killed_foundry(s), 1);
    }
}

/// `amount` player damage in hex `h`, less Fortify: the dealer picks which
/// pieces it destroys, one at a time (a piece needs its full health).
pub fn damage(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, amount: i64) -> Res {
    let w = cx.world;
    let mut left = amount - g.tokens(h, FORTIFY);
    cx.note(Some(h), || format!("{amount} damage at {}", hex_name(&w, h)));
    loop {
        let kinds: Vec<&str> = ADVERSARY
            .iter()
            .copied()
            .filter(|k| g.tokens(h, k) > 0 && health(&w, k) <= left)
            .collect();
        let opts = kinds.iter().map(|k| opt(*k)).collect();
        let prompt = format!("Damage at {}: {left} left. Destroy which piece?", hex_name(&w, h));
        let Some(i) = cx.choose(seat, &prompt, opts)? else {
            break;
        };
        left -= health(&w, kinds[i]);
        destroy_piece(g, cx, Some(seat), h, kinds[i]);
    }
    Ok(())
}

/// Destroys every adversary piece in `h`.
pub fn destroy_all(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    for kind in ADVERSARY {
        while g.tokens(h, kind) > 0 {
            destroy_piece(g, cx, Some(seat), h, kind);
        }
    }
}
