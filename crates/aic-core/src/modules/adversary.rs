//! The adversary: its deck (the clock), the explore -> build -> ravage
//! pipeline, corruption, the Militarist's innate rules and the difficulty
//! levels. Also the rules for destroying pieces on either side, which
//! cards, threats and the pipeline share.

use crate::content::{AdversaryCard, Passive, Selector, Terrain};
use crate::module::{
    Build, Cx, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::{GameState, Opt};
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{board, cards, opposition};

/// The Militarist's parameters, then any more a variant of it adds.
macro_rules! militarist_params {
    ($($extra:expr),* $(,)?) => {
        &[
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
            i("counter_bonus_all", 0, 10, "difficulty 1+: added to every counter requirement of every threat"),
            i("extra_threats", 0, 5, "difficulty 1+: extra threats dealt each round in all, one per country in turn"),
            i("stage1_removed", 0, 3, "difficulty 2+: stage-I cards removed"),
            i("foundry_bonus", 0, 10, "difficulty 3+: extra ravage damage in hexes with a foundry"),
            i("foundry_reach", 0, 10, "difficulty 3+: a ravaged foundry also deals this much damage to each adjacent hex"),
            i("tier2_early", 0, 3, "difficulty 4+: tier-2 threats start this many rounds sooner"),
            i("pool_reduction", 0, 10, "difficulty 4+: corruption pool smaller by this per country"),
            $($extra,)*
        ]
    };
}

pub static DEF: ModuleDef = ModuleDef {
    id: "adversary",
    summary: "The adversary deck and pipeline (explore, build, ravage), corruption, innate rules, difficulty",
    requires: &["board", "cards", "opposition"],
    variants: &[VariantDef {
        name: "militarist",
        summary: "The Militarist: Momentum and Escalation",
        params: militarist_params!(),
        phases: &[
            ph("ravage", "ravage with the card in the ravage slot"),
            ph("build", "build with the card in the build slot"),
            ph("explore", "reveal a card into the explore slot and explore (and escalate)"),
            ph("slide", "slide the cards one slot along; the ravage card is discarded"),
        ],
        rng: &["adversary"],
        uses_cards: false,
        build,
    }, VariantDef {
        name: "uplinks",
        summary: "The Militarist, exploring only from uplinks (and, with explore_foundries, next to foundries): a linear spread",
        params: militarist_params!(i(
            "explore_foundries",
            0,
            1,
            "1: hexes next to a foundry explore too; 0: only uplinks explore"
        )),
        phases: &[
            ph("ravage", "ravage with the card in the ravage slot"),
            ph("build", "build with the card in the build slot"),
            ph("explore", "reveal a card into the explore slot and explore from uplinks (and escalate)"),
            ph("slide", "slide the cards one slot along; the ravage card is discarded"),
        ],
        rng: &["adversary"],
        uses_cards: false,
        build,
    }, VariantDef {
        name: "clusters",
        summary: "Clusters and infection: the adversary places clusters, hegemonizes and assaults by terrain; infection flips data centres",
        params: &[
            i("difficulty", 0, 4, "difficulty level 0-4 (each level includes the ones below)"),
            l("stage_cards", "adversary cards dealt from stage I, II and III (before difficulty)"),
            i("cluster_health", 1, 10, "damage to destroy an adversary cluster"),
            i("cluster_damage", 0, 10, "assault damage each adversary cluster deals in its hex"),
            i("population_health", 1, 10, "damage to destroy a population"),
            i("building_health", 1, 10, "damage to destroy a factory or data centre"),
            i("defense_value", 0, 10, "assault damage and infection one defense token (firewall) prevents"),
            i("corruption_at", 1, 20, "an assault dealing at least this much damage adds corruption"),
            i("corruption_per_ravage", 0, 5, "corruption an assault adds"),
            i("cascade", 0, 5, "corruption a cascade adds to an adjacent hex"),
            i("pool_per_country", 0, 20, "corruption pool size per country"),
            i("pool_base", 0, 20, "corruption pool size on top of the per-country part"),
            i("explore_clusters", 0, 5, "clusters a hex gains when the adversary places (explore slot)"),
            i("source_clusters", 1, 10, "a hex with at least this many clusters is a source to place from"),
            i("hegemonize_clusters", 0, 5, "clusters each hegemonized hex gains (build slot)"),
            i("hegemonize_all", 0, 1, "1: hegemonize every hex of the terrain (Matt's note as written); 0: only hexes holding clusters"),
            i("infection_spill", 0, 1, "1: infection excess left after flipping data centres plants one cluster"),
            i("cluster_opposition", 0, 10, "opposition for destroying a cluster"),
            i("purge_allowance", 0, 20, "purge win: at opposition level L, at most L x this x countries clusters may remain"),
            i("counter_bonus", 0, 10, "difficulty 1+: added to every threat's first counter requirement"),
            i("counter_bonus_all", 0, 10, "difficulty 1+: added to every counter requirement of every threat"),
            i("extra_threats", 0, 5, "difficulty 1+: extra threats dealt each round in all, one per country in turn"),
            i("stage1_removed", 0, 3, "difficulty 2+: stage-I cards removed"),
            i("damage_bonus", 0, 10, "difficulty 3+: extra assault damage in each hex assaulted"),
            i("tier2_early", 0, 3, "difficulty 4+: tier-2 threats start this many rounds sooner"),
            i("pool_reduction", 0, 10, "difficulty 4+: corruption pool smaller by this per country"),
        ],
        phases: &[
            ph("ravage", "assault with the card in the ravage slot: infection, then combat"),
            ph("build", "hegemonize with the card in the build slot"),
            ph("explore", "reveal a card into the explore slot and place clusters"),
            ph("slide", "slide the cards one slot along; the ravage card is discarded"),
        ],
        rng: &["adversary"],
        uses_cards: false,
        build,
    }, VariantDef {
        name: "assets",
        summary: "Asset corruption: no adversary pieces; it corrupts factories and data centres and spreads between them by terrain",
        params: &[
            i("difficulty", 0, 4, "difficulty level 0-4 (each level includes the ones below)"),
            l("stage_cards", "adversary cards dealt from stage I, II and III (before difficulty)"),
            i("rogue_health", 1, 10, "player damage that reclaims a corrupted asset"),
            i("rogue_damage", 0, 10, "strike damage each corrupted asset next to a hex deals there"),
            i("population_health", 1, 10, "damage to destroy a population"),
            i("building_health", 1, 10, "damage to destroy a factory or data centre"),
            i("defense_value", 0, 10, "strike damage one defense token prevents"),
            i("pool_per_country", 0, 20, "corruption pool size per country (one per corrupted asset)"),
            i("pool_base", 0, 20, "corruption pool size on top of the per-country part"),
            i("spread", 0, 5, "assets each corrupted asset corrupts when it spreads (build slot)"),
            i("reclaim_opposition", 0, 10, "opposition for reclaiming a corrupted asset by damage"),
            i("purge_allowance", 0, 20, "purge win: at opposition level L, at most L x this x countries corrupted assets may remain"),
            i("counter_bonus", 0, 10, "difficulty 1+: added to every threat's first counter requirement"),
            i("counter_bonus_all", 0, 10, "difficulty 1+: added to every counter requirement of every threat"),
            i("extra_threats", 0, 5, "difficulty 1+: extra threats dealt each round in all, one per country in turn"),
            i("stage1_removed", 0, 3, "difficulty 2+: stage-I cards removed"),
            i("damage_bonus", 0, 10, "difficulty 3+: extra strike damage in each hex struck"),
            i("tier2_early", 0, 3, "difficulty 4+: tier-2 threats start this many rounds sooner"),
            i("pool_reduction", 0, 10, "difficulty 4+: corruption pool smaller by this per country"),
        ],
        phases: &[
            ph("ravage", "strike with the card in the ravage slot: hexes of its terrain next to corrupted assets take damage"),
            ph("build", "spread with the card in the build slot: corrupted assets corrupt adjacent assets"),
            ph("explore", "reveal a card into the explore slot and corrupt one asset in each country"),
            ph("slide", "slide the cards one slot along; the ravage card is discarded"),
        ],
        rng: &["adversary"],
        uses_cards: false,
        build,
    }],
};

/// Which adversary is in play: the adversary variant, read by the rules
/// that touch adversary pieces (threats, opposition, card effects).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Militarist,
    Clusters,
    Assets,
}

pub fn style(w: &World) -> Style {
    match w.variant("adversary") {
        Some("clusters") => Style::Clusters,
        Some("assets") => Style::Assets,
        _ => Style::Militarist,
    }
}

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
        CLUSTER => w.int("adversary.cluster_health"),
        ROGUE_FACTORY | ROGUE_DATA_CENTRE => w.int("adversary.rogue_health"),
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
    style: Style,
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
    Ok(Box::new(Adversary {
        stage_cards,
        style: style(&b.world),
    }))
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
        match self.style {
            Style::Militarist => {
                // Opening: reveal the first card and explore, then slide.
                if reveal(g, cx) {
                    explore(g, cx);
                }
            }
            Style::Clusters => {
                // The board's starting adversary pieces are placed as one
                // cluster each; then the opening as the Militarist's.
                for h in 0..g.hexes.len() {
                    let n: i64 = MILITARIST.iter().map(|k| g.tokens(h, k)).sum();
                    for k in MILITARIST {
                        let have = g.tokens(h, k);
                        cx.add_tokens(g, h, k, -have);
                    }
                    cx.add_tokens(g, h, CLUSTER, n);
                }
                if reveal(g, cx) {
                    place(g, cx);
                }
            }
            Style::Assets => {
                // No pieces of its own: the board's starting adversary
                // pieces are not used. Opening: reveal, then a beachhead in
                // each country.
                for h in 0..g.hexes.len() {
                    for k in MILITARIST {
                        let have = g.tokens(h, k);
                        cx.add_tokens(g, h, k, -have);
                    }
                }
                reveal(g, cx);
                beachhead(g, cx);
            }
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
        if self.style != Style::Militarist {
            return self.run_spread(phase, g, cx);
        }
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
                        build_hex(g, cx, h)?;
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
        out.bookkeeping(&["adversary.killed_foundry.", PEEK]);
        if peek > 0 {
            out.global("peek", "cards shown", g.var(PEEK), None, "Recon Sweep: the top cards of the deck are shown");
        }
        for s in 0..g.seats.len() {
            if g.var(&killed_foundry(s)) > 0 {
                out.seat(
                    s,
                    "killed_foundry",
                    "destroyed a foundry",
                    1,
                    None,
                    "you destroyed a foundry this round (for cards that ask)",
                );
            }
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
    // Variant `uplinks`: only uplinks explore, and with `explore_foundries`
    // also hexes next to a foundry; outposts no longer seed explore.
    let uplinks_only = w.variant("adversary") == Some("uplinks");
    let from_foundries = uplinks_only && w.int("adversary.explore_foundries") > 0;
    let mut hits = Vec::new();
    for h in 0..g.hexes.len() {
        if !has_terrain(&w, h, &c.terrains) {
            continue;
        }
        let source = |x: usize| {
            if uplinks_only {
                from_foundries && g.tokens(x, FOUNDRY) > 0
            } else {
                g.tokens(x, OUTPOST) + g.tokens(x, FOUNDRY) > 0
            }
        };
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
/// otherwise an outpost. Clusters: the hex is hegemonized. Assets: the hex
/// spreads (an asset on or adjacent to it is corrupted).
pub fn build_hex(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    match style(&cx.world) {
        Style::Militarist => {}
        Style::Clusters => {
            let n = cx.world.int("adversary.hegemonize_clusters");
            cx.add_tokens(g, h, CLUSTER, n);
            cx.adversary_ops(g, 1);
            return Ok(());
        }
        Style::Assets => return spread_from(g, cx, h, None),
    }
    let kind = if g.tokens(h, OUTPOST) > g.tokens(h, FOUNDRY) {
        FOUNDRY
    } else {
        OUTPOST
    };
    let n = cx.world.int("adversary.build_pieces");
    cx.add_tokens(g, h, kind, n);
    cx.adversary_ops(g, 1);
    Ok(())
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
/// Assets: the corruption spreads to an asset on or adjacent to `from`.
pub fn cascade(g: &mut GameState, cx: &mut Cx, from: usize) -> Res {
    let w = cx.world;
    if style(&w) == Style::Assets {
        return spread_from(g, cx, from, None);
    }
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
/// (Clusters: the combat half of an assault, without Momentum. Assets: a
/// strike, below.)
pub fn ravage(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    let w = cx.world;
    let mut raw: i64 = match style(&w) {
        Style::Militarist => MILITARIST.iter().map(|k| g.tokens(h, k) * piece_damage(&w, k)).sum(),
        Style::Clusters => g.tokens(h, CLUSTER) * w.int("adversary.cluster_damage"),
        Style::Assets => return strike(g, cx, h),
    };
    if raw == 0 {
        return Ok(());
    }
    match style(&w) {
        Style::Militarist if level(&w, 3) && g.tokens(h, FOUNDRY) > 0 => raw += w.int("adversary.foundry_bonus"),
        Style::Clusters if level(&w, 3) => raw += w.int("adversary.damage_bonus"),
        _ => {}
    }
    if style(&w) == Style::Militarist {
        foundry_reach(g, cx, h)?;
    }
    let prevented = prevented(&w, g, h);
    let dmg = (raw - prevented).max(0);
    cx.adversary_ops(g, 1);
    cx.note(Some(h), || {
        format!("Ravage at {}: {raw} damage, {prevented} prevented", hex_name(&w, h))
    });
    if dmg == 0 {
        return Ok(());
    }
    let lost_population = lose_pieces(g, cx, h, dmg)?;
    if dmg >= w.int("adversary.corruption_at") {
        let had = g.tokens(h, CORRUPTION) > 0;
        add_corruption(g, cx, h, w.int("adversary.corruption_per_ravage"));
        if had {
            cascade(g, cx, h)?;
        }
    }
    if style(&w) != Style::Militarist {
        return Ok(());
    }
    let c = w.map.hexes[h].country as usize;
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

/// Difficulty 3: the foundries in a ravaged hex `h` reach out first. Each
/// adjacent hex (any country) takes `foundry_reach` damage per foundry,
/// less its defense, destroying players' pieces as a ravage does (players
/// choose each). The reach adds no corruption and no Momentum.
fn foundry_reach(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    let w = cx.world;
    let reach = if level(&w, 3) {
        w.int("adversary.foundry_reach") * g.tokens(h, FOUNDRY)
    } else {
        0
    };
    if reach <= 0 {
        return Ok(());
    }
    for n in w.map.hexes[h].neighbours.clone() {
        let has_pieces = [POPULATION, FACTORY, DATA_CENTRE].iter().any(|k| g.tokens(n, k) > 0);
        if !has_pieces {
            continue;
        }
        let prevented = prevented(&w, g, n);
        let dmg = (reach - prevented).max(0);
        cx.note(Some(n), || {
            format!(
                "Foundry reach from {} to {}: {reach} damage, {prevented} prevented",
                hex_name(&w, h),
                hex_name(&w, n)
            )
        });
        if dmg > 0 {
            cx.adversary_ops(g, 1);
            lose_pieces(g, cx, n, dmg)?;
        }
    }
    Ok(())
}

/// Damage the players' defense tokens in `h` prevent (each token its
/// owner's defense value).
fn prevented(w: &World, g: &GameState, h: usize) -> i64 {
    (0..g.seats.len())
        .map(|s| g.tokens(h, &defense(s)) * cards::defense_value(w, g, s))
        .sum()
}

/// `dmg` adversary damage destroys the players' pieces in `h` one at a
/// time, the players choosing each among those it can still destroy.
/// Returns whether population was lost.
fn lose_pieces(g: &mut GameState, cx: &mut Cx, h: usize, dmg: i64) -> Res<bool> {
    let w = cx.world;
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
    Ok(lost_population)
}

/// Destroys one adversary piece of `kind` in `h`, scoring opposition for
/// `seat` (if a player destroyed it). A corrupted asset is reclaimed
/// instead: it flips back to the players and its corruption returns to
/// the pool.
pub fn destroy_piece(g: &mut GameState, cx: &mut Cx, seat: Option<usize>, h: usize, kind: &str) {
    if cx.add_tokens(g, h, kind, -1) == 0 {
        return;
    }
    if let Some(asset) = asset_of(kind) {
        cx.add_tokens(g, h, asset, 1);
        cx.add_key(g, POOL.into(), 1);
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

// ---------------------------------------------------------------------------
// Spread variants: clusters and asset corruption.
// ---------------------------------------------------------------------------

/// The asset a corrupted asset was (and flips back to when reclaimed).
pub fn asset_of(kind: &str) -> Option<&'static str> {
    match kind {
        ROGUE_FACTORY => Some(FACTORY),
        ROGUE_DATA_CENTRE => Some(DATA_CENTRE),
        _ => None,
    }
}

fn rogue_of(asset: &str) -> &'static str {
    if asset == FACTORY {
        ROGUE_FACTORY
    } else {
        ROGUE_DATA_CENTRE
    }
}

/// Every adversary piece on the board (clusters, or corrupted assets).
pub fn presence(g: &GameState) -> i64 {
    (0..g.hexes.len()).map(|h| adversary_count(g, h)).sum()
}

/// Whether the adversary's presence is under the purge bar for opposition
/// `level` (below the top level, which wins at once).
pub fn under_bar(w: &World, g: &GameState, level: i64) -> bool {
    let count = |k: &str| (0..g.hexes.len()).map(|h| g.tokens(h, k)).sum::<i64>();
    match style(w) {
        Style::Militarist => match level {
            0 => count(DRONE) + count(OUTPOST) + count(FOUNDRY) == 0,
            1 => count(OUTPOST) + count(FOUNDRY) == 0,
            _ => count(FOUNDRY) == 0,
        },
        _ => presence(g) <= allowance(w, level),
    }
}

/// Clusters or corrupted assets allowed to remain for the purge win at `level`.
fn allowance(w: &World, level: i64) -> i64 {
    level * w.int("adversary.purge_allowance") * w.map.countries.len() as i64
}

/// The purge win at `level`, in words.
pub fn win_text(w: &World, level: i64) -> String {
    let what = match style(w) {
        Style::Militarist => {
            return match level {
                0 => "win when no adversary pieces remain",
                1 => "win when no outposts or foundries remain",
                _ => "win when no foundries remain",
            }
            .to_string()
        }
        Style::Clusters => "clusters",
        Style::Assets => "corrupted assets",
    };
    match allowance(w, level) {
        0 => format!("win when no {what} remain"),
        n => format!("win when at most {n} {what} remain"),
    }
}

/// How many of `kind` hex `h` holds, reading a Militarist piece kind (named
/// by threats) as the adversary in play: any cluster, or any corrupted asset.
pub fn kind_count(w: &World, g: &GameState, h: usize, kind: &str) -> i64 {
    if !MILITARIST.contains(&kind) {
        return g.tokens(h, kind);
    }
    match style(w) {
        Style::Militarist => g.tokens(h, kind),
        Style::Clusters => g.tokens(h, CLUSTER),
        Style::Assets => ROGUE.iter().map(|k| g.tokens(h, k)).sum(),
    }
}

/// A threat's hex selector, with piece kinds read by [`kind_count`]. With
/// asset corruption, "corrupted" means holding a corrupted asset.
pub fn selects(w: &World, g: &GameState, h: usize, sel: &Selector) -> bool {
    let corrupted = match style(w) {
        Style::Assets => ROGUE.iter().any(|k| g.tokens(h, k) > 0),
        _ => g.tokens(h, CORRUPTION) > 0,
    };
    (!sel.uplink || w.map.hexes[h].uplink)
        && sel.has.iter().all(|k| kind_count(w, g, h, k) > 0)
        && sel.adversary_at_least.is_none_or(|n| adversary_count(g, h) >= n)
        && (!sel.corrupted || corrupted)
}

/// A threat adds `n` of `piece` to `h`. A Militarist piece kind is read as
/// the adversary in play: `n` clusters, or `n` spreads from `h`.
pub fn reinforce(g: &mut GameState, cx: &mut Cx, h: usize, piece: &str, n: i64) -> Res {
    let w = cx.world;
    let piece = match style(&w) {
        Style::Clusters if MILITARIST.contains(&piece) => CLUSTER,
        Style::Assets if MILITARIST.contains(&piece) => {
            for _ in 0..n {
                spread_from(g, cx, h, None)?;
            }
            return Ok(());
        }
        _ => piece,
    };
    cx.add_tokens(g, h, piece, n);
    cx.adversary_ops(g, 1);
    Ok(())
}

/// Forward base in country `c`: one drone on the lowest-numbered uplink
/// holding a drone becomes an outpost. Clusters: that uplink (holding a
/// cluster) gains a cluster. Assets: the lowest-numbered uplink spreads.
pub fn forward_base(g: &mut GameState, cx: &mut Cx, c: usize) -> Res {
    let w = cx.world;
    let uplinks: Vec<usize> = w.map.countries[c]
        .hexes
        .iter()
        .copied()
        .filter(|&h| w.map.hexes[h].uplink)
        .collect();
    match style(&w) {
        Style::Militarist => {
            if let Some(&h) = uplinks.iter().find(|&&h| g.tokens(h, DRONE) > 0) {
                cx.add_tokens(g, h, DRONE, -1);
                cx.add_tokens(g, h, OUTPOST, 1);
                cx.adversary_ops(g, 1);
            }
        }
        Style::Clusters => {
            if let Some(&h) = uplinks.iter().find(|&&h| g.tokens(h, CLUSTER) > 0) {
                cx.add_tokens(g, h, CLUSTER, 1);
                cx.adversary_ops(g, 1);
            }
        }
        Style::Assets => {
            if let Some(&h) = uplinks.first() {
                spread_from(g, cx, h, None)?;
            }
        }
    }
    Ok(())
}

/// Opposition pushback in country `c`: remove one drone (players choose the
/// hex). Clusters: one cluster. Assets: reclaim one corrupted asset.
pub fn pushback(g: &mut GameState, cx: &mut Cx, c: usize) -> Res {
    let w = cx.world;
    let kinds: &[&str] = match style(&w) {
        Style::Militarist => &[DRONE],
        Style::Clusters => &[CLUSTER],
        Style::Assets => &ROGUE,
    };
    let mut picks = Vec::new();
    for &h in &w.map.countries[c].hexes {
        for &k in kinds {
            if g.tokens(h, k) > 0 {
                picks.push((h, k));
            }
        }
    }
    let opts = picks.iter().map(|&(h, k)| piece_opt(&w, h, k, kinds.len() > 1)).collect();
    let prompt = match style(&w) {
        Style::Assets => format!("Pushback in country {c}: reclaim which corrupted asset?"),
        _ => format!("Pushback in country {c}: remove a {} from which hex?", kinds[0]),
    };
    if let Some(i) = cx.choose(0, &prompt, opts)? {
        let (h, k) = picks[i];
        match asset_of(k) {
            Some(_) => reclaim(g, cx, h, k),
            None => {
                cx.add_tokens(g, h, k, -1);
            }
        }
    }
    Ok(())
}

/// A hex option, naming the piece too when a hex may offer several.
fn piece_opt(w: &World, h: usize, kind: &str, named: bool) -> Opt {
    let mut o = hex_opt(w, h);
    if named {
        o.option = format!("{} {kind}", o.option);
    }
    o
}

/// Flips one `asset` (factory or data centre) in `h` to the adversary,
/// taking one corruption from the pool. Nothing if the pool is empty.
fn corrupt(g: &mut GameState, cx: &mut Cx, h: usize, asset: &str) {
    let w = cx.world;
    let pool = g.var(POOL);
    if pool <= 0 || g.tokens(h, asset) == 0 {
        return;
    }
    cx.set_key(g, POOL.into(), pool - 1);
    cx.add_tokens(g, h, asset, -1);
    cx.add_tokens(g, h, rogue_of(asset), 1);
    cx.adversary_ops(g, 1);
    cx.tally(g, "adversary.assets_corrupted", 1);
    cx.note(Some(h), || format!("The {asset} at {} is corrupted", hex_name(&w, h)));
}

/// Flips a corrupted asset back to the players; its corruption returns to
/// the pool. No opposition (that is for reclaiming by damage).
pub fn reclaim(g: &mut GameState, cx: &mut Cx, h: usize, rogue: &str) {
    let Some(asset) = asset_of(rogue) else {
        return;
    };
    if cx.add_tokens(g, h, rogue, -1) == 0 {
        return;
    }
    cx.add_tokens(g, h, asset, 1);
    cx.add_key(g, POOL.into(), 1);
    cx.tally(g, "adversary.reclaimed", 1);
}

/// Reclaims one corrupted asset in `h` (a factory first).
pub fn reclaim_any(g: &mut GameState, cx: &mut Cx, h: usize) {
    if let Some(k) = ROGUE.iter().copied().find(|k| g.tokens(h, k) > 0) {
        reclaim(g, cx, h, k);
    }
}

/// Whether hex `h` is of, or adjacent to a hex of, one of `terrains`.
fn near_terrain(w: &World, h: usize, terrains: &[Terrain]) -> bool {
    next_to(w, h, |x| has_terrain(w, x, terrains))
}

/// Uncorrupted assets among `hexes`, as (hex, asset), factories first in
/// each hex.
fn assets_in(g: &GameState, hexes: impl IntoIterator<Item = usize>) -> Vec<(usize, &'static str)> {
    let mut out = Vec::new();
    for h in hexes {
        for a in [FACTORY, DATA_CENTRE] {
            if g.tokens(h, a) > 0 {
                out.push((h, a));
            }
        }
    }
    out
}

/// The corruption spreads from `h`: one uncorrupted asset on `h` or an
/// adjacent hex (any country), and if `terrains` is given, on a hex next to
/// one of them, is corrupted (players choose which).
pub fn spread_from(g: &mut GameState, cx: &mut Cx, h: usize, terrains: Option<&[Terrain]>) -> Res {
    let w = cx.world;
    let around = std::iter::once(h).chain(w.map.hexes[h].neighbours.iter().copied());
    let picks: Vec<(usize, &str)> = assets_in(g, around)
        .into_iter()
        .filter(|&(x, _)| terrains.is_none_or(|t| near_terrain(&w, x, t)))
        .collect();
    let opts = picks.iter().map(|&(x, a)| piece_opt(&w, x, a, true)).collect();
    let prompt = format!("Corruption spreads from {}: which asset is corrupted?", hex_name(&w, h));
    if let Some(i) = cx.choose(0, &prompt, opts)? {
        corrupt(g, cx, picks[i].0, picks[i].1);
    }
    Ok(())
}

/// Opening beachhead: in each country, the asset nearest its printed
/// outpost uplink is corrupted (ties: lower hex number; a factory before a
/// data centre on the same hex).
fn beachhead(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        let piece = &w.content.map.countries[c];
        let Some(&n) = piece.outposts.first().or(piece.uplinks.first()) else {
            continue;
        };
        let from = w.map.numbered(c, n);
        let best = assets_in(g, w.map.countries[c].hexes.iter().copied())
            .into_iter()
            .min_by_key(|&(h, _)| (w.map.distance(from, h), w.map.hexes[h].number));
        if let Some((h, a)) = best {
            corrupt(g, cx, h, a);
        }
    }
}

/// A strike in hex `h`: each corrupted asset on or adjacent to it deals
/// `rogue_damage`, less defense, to the players' pieces there.
fn strike(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    let w = cx.world;
    let near: i64 = std::iter::once(h)
        .chain(w.map.hexes[h].neighbours.iter().copied())
        .map(|x| ROGUE.iter().map(|k| g.tokens(x, k)).sum::<i64>())
        .sum();
    let mut raw = near * w.int("adversary.rogue_damage");
    if raw == 0 {
        return Ok(());
    }
    if level(&w, 3) {
        raw += w.int("adversary.damage_bonus");
    }
    let prevented = prevented(&w, g, h);
    let dmg = (raw - prevented).max(0);
    cx.adversary_ops(g, 1);
    cx.note(Some(h), || {
        format!("Strike at {}: {raw} damage, {prevented} prevented", hex_name(&w, h))
    });
    if dmg > 0 {
        lose_pieces(g, cx, h, dmg)?;
    }
    Ok(())
}

/// Clusters' place step (the explore slot): each hex of the card's terrain
/// that is an uplink or next to a source (a hex with `source_clusters`)
/// gains `explore_clusters`. Firewall Node stops it as it stops explore.
fn place(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let Some(id) = g.pile(SLOTS[0]).first().cloned() else {
        return;
    };
    let c = card(&w, &id);
    cx.note(None, || format!("Place: {}", card_text(c)));
    let n = w.int("adversary.explore_clusters");
    let source_at = w.int("adversary.source_clusters");
    let shields: Vec<usize> = (0..g.seats.len())
        .filter(|&s| cards::has_passive(&w, g, s, |p| matches!(p, Passive::ExploreShield)))
        .collect();
    let mut hits = Vec::new();
    for h in 0..g.hexes.len() {
        if !has_terrain(&w, h, &c.terrains) {
            continue;
        }
        let source = |x: usize| g.tokens(x, CLUSTER) >= source_at;
        if !(w.map.hexes[h].uplink || next_to(&w, h, source)) {
            continue;
        }
        if shields.iter().any(|&s| g.tokens(h, &processor(s)) > 0) {
            cx.note(Some(h), || format!("Firewall Node: no cluster at {}", hex_name(&w, h)));
            continue;
        }
        hits.push(h);
    }
    for h in hits {
        cx.add_tokens(g, h, CLUSTER, n);
        cx.adversary_ops(g, 1);
    }
}

/// Infection in hex `h`: offense is the clusters on adjacent hexes;
/// defense is the processors on `h` (friendly clusters) plus its defense
/// tokens (the firewall). Each point of excess flips a data centre (a
/// neutral cluster) to an adversary cluster; with `infection_spill`, excess
/// left over plants one cluster on a hex that has none.
fn infect(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    let offense: i64 = w.map.hexes[h].neighbours.iter().map(|&n| g.tokens(n, CLUSTER)).sum();
    let friendly: i64 = (0..g.seats.len()).map(|s| g.tokens(h, &processor(s))).sum();
    let defense = friendly + prevented(&w, g, h);
    let excess = offense - defense;
    if excess <= 0 {
        return;
    }
    let flips = excess.min(g.tokens(h, DATA_CENTRE));
    if flips > 0 {
        cx.add_tokens(g, h, DATA_CENTRE, -flips);
        cx.add_tokens(g, h, CLUSTER, flips);
        cx.tally(g, "adversary.buildings_lost", flips);
        cx.tally(g, "adversary.infected", flips);
    }
    let spill = excess > flips && w.int("adversary.infection_spill") > 0 && g.tokens(h, CLUSTER) == 0;
    if spill {
        cx.add_tokens(g, h, CLUSTER, 1);
        cx.tally(g, "adversary.infected", 1);
    }
    if flips > 0 || spill {
        cx.adversary_ops(g, 1);
        cx.note(Some(h), || {
            format!("Infection at {}: {offense} against {defense}", hex_name(&w, h))
        });
    }
}

impl Adversary {
    /// The pipeline of the spread variants (`clusters`, `assets`).
    fn run_spread(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        let slot = match phase {
            0 => SLOTS[2],
            1 => SLOTS[1],
            2 => {
                if !reveal(g, cx) {
                    return Ok(());
                }
                SLOTS[0]
            }
            _ => {
                slide(g, cx);
                return Ok(());
            }
        };
        let Some(id) = g.pile(slot).first().cloned() else {
            return Ok(());
        };
        let c = card(&w, &id);
        let terrain: Vec<usize> = (0..g.hexes.len()).filter(|&h| has_terrain(&w, h, &c.terrains)).collect();
        match (self.style, phase) {
            // Assault: infection on every hex of the terrain, then combat
            // where clusters are.
            (Style::Clusters, 0) => {
                cx.note(None, || format!("Assault: {}", card_text(c)));
                for &h in &terrain {
                    infect(g, cx, h);
                }
                let fighting: Vec<usize> = terrain.into_iter().filter(|&h| g.tokens(h, CLUSTER) > 0).collect();
                for h in fighting {
                    ravage(g, cx, h)?;
                }
            }
            // Hegemonize: a cluster on each hex of the terrain holding one
            // (or on every hex of it, with `hegemonize_all`).
            (Style::Clusters, 1) => {
                cx.note(None, || format!("Hegemonize: {}", card_text(c)));
                let all = w.int("adversary.hegemonize_all") > 0;
                let hexes: Vec<usize> = terrain.into_iter().filter(|&h| all || g.tokens(h, CLUSTER) > 0).collect();
                for h in hexes {
                    build_hex(g, cx, h)?;
                }
            }
            (Style::Clusters, _) => place(g, cx),
            // Strike: hexes of the terrain next to corrupted assets.
            (_, 0) => {
                cx.note(None, || format!("Strike: {}", card_text(c)));
                let hexes: Vec<usize> = terrain
                    .into_iter()
                    .filter(|&h| next_to(&w, h, |x| ROGUE.iter().any(|k| g.tokens(x, k) > 0)))
                    .collect();
                for h in hexes {
                    strike(g, cx, h)?;
                }
            }
            // Spread: each corrupted asset corrupts `spread` assets on or
            // adjacent to it, on hexes next to the terrain.
            (_, 1) => {
                cx.note(None, || format!("Spread: {}", card_text(c)));
                let k = w.int("adversary.spread");
                let sources: Vec<(usize, i64)> = (0..g.hexes.len())
                    .map(|h| (h, ROGUE.iter().map(|r| g.tokens(h, r)).sum::<i64>()))
                    .filter(|&(_, n)| n > 0)
                    .collect();
                for (h, n) in sources {
                    for _ in 0..n * k {
                        spread_from(g, cx, h, Some(&c.terrains))?;
                    }
                }
            }
            // Corrupt: one asset in each country, next to the terrain
            // (players choose which). Firewall Node protects assets on its
            // owner's processor hexes, as it stops explore.
            _ => {
                cx.note(None, || format!("Corrupt: {}", card_text(c)));
                let shields: Vec<usize> = (0..g.seats.len())
                    .filter(|&s| cards::has_passive(&w, g, s, |p| matches!(p, Passive::ExploreShield)))
                    .collect();
                for country in 0..w.map.countries.len() {
                    let picks: Vec<(usize, &str)> = assets_in(g, w.map.countries[country].hexes.iter().copied())
                        .into_iter()
                        .filter(|&(h, _)| near_terrain(&w, h, &c.terrains))
                        .filter(|&(h, _)| !shields.iter().any(|&s| g.tokens(h, &processor(s)) > 0))
                        .collect();
                    let opts = picks.iter().map(|&(h, a)| piece_opt(&w, h, a, true)).collect();
                    let prompt = format!("Corrupt in country {country}: which asset is corrupted?");
                    if let Some(i) = cx.choose(0, &prompt, opts)? {
                        corrupt(g, cx, picks[i].0, picks[i].1);
                    }
                }
            }
        }
        Ok(())
    }
}
