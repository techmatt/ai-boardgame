//! The adversary (014): its deck (the clock and the stager), its
//! ambassadors (build, spread, corrupt), the ambassador supply and the
//! difficulty levels. Also the shared rules for placing and removing
//! adversary ambassadors and troops, which cards, threats and Alignment use,
//! and the board conditions for winning and losing.

use crate::content::{AdversaryCard, Passive, Twist};
use crate::module::{Build, Cx, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{alignment, cards};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::int("difficulty", 0, 4, "difficulty level 0-4 (each level includes the ones below)"),
    ParamSpec::list("stage_cards", 0, 20, "adversary cards dealt from stage I, II and III (before difficulty)"),
    ParamSpec::int("supply_per_country", 0, 40, "adversary ambassadors in the supply per country"),
    ParamSpec::int("city_build", 0, 10, "build: ambassadors each corrupted city gains"),
    ParamSpec::int("factory_build", 0, 10, "build: ambassadors each factory next to a corrupted city gains"),
    ParamSpec::int("spread_range", 1, 6, "spread: how far (hexes) a spreading ambassador may travel"),
    ParamSpec::int("corrupt_at", 1, 20, "corrupt: a city that is not corrupted, holding this many adversary ambassadors, is corrupted"),
    ParamSpec::int("escalation_ambassadors", 0, 5, "stage-II escalation: ambassadors each military base gains when the card builds"),
    ParamSpec::int("counter_bonus", 0, 10, "difficulty 1+: added to every threat's first counter requirement"),
    ParamSpec::int("counter_bonus_all", 0, 10, "difficulty 1+: added to every counter requirement of every threat"),
    ParamSpec::int("extra_threats", 0, 5, "difficulty 1+: extra threats dealt each round in all, one per country in turn"),
    ParamSpec::int("stage1_removed", 0, 3, "difficulty 2+: stage-I cards removed"),
    ParamSpec::int("extra_corrupted", 0, 3, "difficulty 4: more cities per country start corrupted"),
    ParamSpec::int("build_bonus", 0, 5, "difficulty 3+: ambassadors each corrupted city gains on top of city_build"),
    ParamSpec::int("tier2_early", 0, 3, "difficulty 4+: tier-2 threats start this many rounds sooner"),
    ParamSpec::int("supply_reduction", 0, 20, "difficulty 4+: ambassador supply smaller by this per country"),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto("build", "the next card becomes this round's; corrupted cities and the factories next to them gain ambassadors"),
    PhaseSpec::auto("spread", "every hex with several adversary ambassadors keeps one and sends the rest out"),
    PhaseSpec::auto("corrupt", "cities that are not corrupted and hold enough adversary ambassadors are corrupted"),
    PhaseSpec::auto("reveal", "reveal the next adversary card face up, a round ahead"),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "adversary",
    summary: "The adversary deck (the clock), ambassadors (build, spread, corrupt), the supply, difficulty",
    requires: &["board", "cards", "alignment"],
    variants: &[
        VariantDef {
            name: "militarist",
            summary: "The Militarist: ambassadors spread from corrupted cities; stage II escalates at the bases",
            params: PARAMS,
            phases: PHASES,
            rng: &["adversary"],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "overflow",
            summary: "The Militarist, with overflow: an ambassador arriving on a city or station that already holds one corrupts it",
            params: PARAMS,
            phases: PHASES,
            rng: &["adversary"],
            uses_cards: false,
            build,
        },
    ],
};

pub const DECK: &str = "adversary.deck";
pub const DISCARD: &str = "adversary.discard";
/// This round's card (its twists apply).
pub const CURRENT: &str = "adversary.current";
/// Next round's card, face up a round ahead.
pub const NEXT: &str = "adversary.next";
/// Adversary ambassadors in the supply.
pub const SUPPLY: &str = "adversary.supply";
/// Set when an ambassador had to be placed and the supply was empty.
pub const STARVED: &str = "adversary.starved";
/// Stage of this round's card.
pub const STAGE: &str = "adversary.stage";
/// How many more deck cards beyond the next are shown (Recon Sweep).
pub const PEEK: &str = "adversary.peek";
/// How many levels Discord has escalated the adversary (module `alignment`).
pub const ESCALATION: &str = "adversary.escalation";
/// The round of the first corruption after setup (sim observation).
const FIRST_CORRUPTION: &str = "adversary.first_corruption";

/// Seat `s` removed an adversary troop this round (card conditions).
pub fn removed_troop(seat: usize) -> String {
    format!("adversary.removed_troop.{seat}")
}

/// Whether the configured difficulty is at least `n` (setup rules).
pub fn level(w: &World, n: i64) -> bool {
    w.int("adversary.difficulty") >= n
}

/// Whether the difficulty in play, the configured one plus any Discord
/// escalation, is at least `n`.
pub fn at(w: &World, g: &GameState, n: i64) -> bool {
    w.int("adversary.difficulty") + g.var(ESCALATION) >= n
}

/// The difficulty in play (configured plus escalation), up to 4.
pub fn difficulty(w: &World, g: &GameState) -> i64 {
    (w.int("adversary.difficulty") + g.var(ESCALATION)).min(4)
}

/// The ambassador supply at the start, for this many countries.
pub fn start_supply(w: &World) -> i64 {
    let mut per = w.int("adversary.supply_per_country");
    if level(w, 4) {
        per -= w.int("adversary.supply_reduction");
    }
    (per * w.map.countries.len() as i64).max(0)
}

/// Which tier of threats is dealt now: tier 2 once this round's card is
/// stage II or later (at difficulty 4, `tier2_early` cards sooner).
pub fn threat_tier(w: &World, g: &GameState) -> u8 {
    if g.var(STAGE) >= 2 {
        return 2;
    }
    if at(w, g, 4) {
        let early = w.int("adversary.tier2_early") as usize;
        let soon = g.pile(NEXT).iter().chain(g.pile(DECK)).take(early).any(|id| card(w, id).stage >= 2);
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
    format!("stage {}: {}", c.stage, if c.text.is_empty() { "no twist" } else { &c.text })
}

/// This round's twists.
pub fn twists<'w>(w: &World<'w>, g: &GameState) -> Vec<&'w Twist> {
    g.pile(CURRENT).iter().flat_map(|id| card(w, id).twists.iter()).collect()
}

fn twist_sum(w: &World, g: &GameState, f: impl Fn(&Twist) -> Option<i64>) -> i64 {
    twists(w, g).into_iter().filter_map(f).sum()
}

/// Whether this round's card makes it a war round.
pub fn war_twist(w: &World, g: &GameState) -> bool {
    twists(w, g).iter().any(|t| matches!(t, Twist::War))
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
        let have = b.world.content.adversary.iter().filter(|c| c.stage as usize == s + 1).count() as i64;
        if n > have {
            return Err(format!("stage_cards: stage {} has only {have} cards", s + 1));
        }
    }
    Ok(Box::new(Adversary { stage_cards }))
}

impl Module for Adversary {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        cx.set_key(g, SUPPLY.into(), start_supply(&w));
        let mut deck = Vec::new();
        for stage in 1..=3u8 {
            let mut ids: Vec<String> =
                w.content.adversary.iter().filter(|c| c.stage == stage).map(|c| c.id.clone()).collect();
            shuffle(&mut ids, cx.rng(g, "adversary"));
            let mut n = self.stage_cards[stage as usize - 1];
            if stage == 1 && level(&w, 2) {
                n = (n - w.int("adversary.stage1_removed")).max(0);
            }
            deck.extend(ids.into_iter().take(n as usize));
        }
        cx.set_pile(g, DECK, deck);
        reveal(g, cx);
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            cx.set_key(g, removed_troop(s), 0);
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match phase {
            0 => {
                // The face-up card becomes this round's.
                for id in g.pile(CURRENT).to_vec() {
                    cx.move_card(g, &id, CURRENT, DISCARD, true).expect("current card");
                }
                let Some(id) = g.pile(NEXT).first().cloned() else {
                    cx.end(g, false, "deck_empty", "the adversary deck ran out".into());
                    return Ok(());
                };
                cx.move_card(g, &id, NEXT, CURRENT, true).expect("next card");
                let c = card(&w, &id);
                cx.set_key(g, STAGE.into(), c.stage as i64);
                cx.note(None, || format!("Adversary card: {} ({})", c.name, card_text(c)));
                for t in &c.twists {
                    if let Twist::Discord { amount } = t {
                        alignment::sway(g, cx, crate::content::Side::Discord, *amount);
                    }
                }
                build_all(g, cx, None)?;
                if c.escalation {
                    let n = w.int("adversary.escalation_ambassadors");
                    let bases: Vec<usize> = (0..g.hexes.len()).filter(|&h| is_base(&w, h)).collect();
                    for h in bases {
                        place(g, cx, h, n);
                    }
                }
            }
            1 => spread(g, cx, None)?,
            2 => {
                let at = (w.int("adversary.corrupt_at")
                    - twist_sum(&w, g, |t| match t {
                        Twist::CorruptSooner { amount } => Some(*amount),
                        _ => None,
                    }))
                .max(1);
                let cities: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| is_city(&w, h) && !corrupted(g, h) && g.tokens(h, ADV_AMBASSADOR) >= at)
                    .collect();
                for h in cities {
                    corrupt_city(g, cx, h);
                }
                let on_board: i64 = (0..g.hexes.len()).map(|h| g.tokens(h, ADV_AMBASSADOR)).sum();
                cx.tally(g, &format!("adversary.ambassadors_r{:02}", g.round), on_board);
            }
            _ => reveal(g, cx),
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
        let next: Vec<CardView> = g.pile(NEXT).iter().map(|id| show(id, "face up: next round")).collect();
        out.pile("next", "Next round's card", None, next.len(), next);
        let cur: Vec<CardView> = g.pile(CURRENT).iter().map(|id| show(id, "this round")).collect();
        out.pile("current", "This round's card", None, cur.len(), cur);
        let disc = g.pile(DISCARD);
        out.pile("discard", "Adversary discard", None, disc.len(), disc.iter().map(|id| show(id, "done")).collect());
        out.global(
            "supply",
            "adversary ambassador supply",
            g.var(SUPPLY),
            Some(start_supply(w)),
            "adversary ambassadors left; players lose when one must be placed and none is left",
        );
        out.global("stage", "adversary stage", g.var(STAGE), Some(3), "stage of this round's adversary card");
        out.global(
            "difficulty",
            "difficulty",
            difficulty(w, g),
            Some(4),
            "difficulty level in play: the configured level plus Discord escalation",
        );
        if g.var(ESCALATION) > 0 {
            out.global("escalation", "Discord escalation", g.var(ESCALATION), None, "difficulty levels Discord tips have added");
        }
        out.global(
            "deck",
            "adversary cards left",
            deck.len() as i64 + g.pile(NEXT).len() as i64,
            None,
            "rounds left on the clock after this one; players lose when the next round has no card",
        );
        out.bookkeeping(&[PEEK, STARVED, FIRST_CORRUPTION, "adversary.removed_troop."]);
    }
}

/// Shows the next `n` deck cards beyond the face-up one (Recon Sweep); a
/// longer peek already running is kept.
pub fn peek(g: &mut GameState, cx: &mut Cx, n: i64) {
    let cur = g.var(PEEK);
    cx.set_key(g, PEEK.into(), cur.max(n));
}

/// Reveals the top deck card face up as next round's card (nothing if the deck is empty).
fn reveal(g: &mut GameState, cx: &mut Cx) {
    if let Some(id) = g.pile(DECK).first().cloned() {
        cx.move_card(g, &id, DECK, NEXT, true).expect("top of the deck");
        let peek = g.var(PEEK);
        if peek > 0 {
            cx.set_key(g, PEEK.into(), peek - 1);
        }
    }
}

/// The build step, in country `scope` or everywhere: each corrupted city
/// gains `city_build` (+ difficulty 3, + twists), each factory next to a
/// corrupted city `factory_build`; twists add clean-city factories and
/// corrupted stations.
pub fn build_all(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) -> Res {
    let w = cx.world;
    let mut city = w.int("adversary.city_build");
    if at(&w, g, 3) {
        city += w.int("adversary.build_bonus");
    }
    city += twist_sum(&w, g, |t| match t {
        Twist::CityBuild { amount } => Some(*amount),
        _ => None,
    });
    let clean = twist_sum(&w, g, |t| match t {
        Twist::CleanFactories { amount } => Some(*amount),
        _ => None,
    });
    let stations = twist_sum(&w, g, |t| match t {
        Twist::Stations { amount } => Some(*amount),
        _ => None,
    });
    let factory = w.int("adversary.factory_build");
    let mut plan: Vec<(usize, i64)> = Vec::new();
    for h in scope_hexes(&w, scope) {
        let mut n = 0;
        if is_city(&w, h) && corrupted(g, h) {
            n += city;
        }
        if g.tokens(h, FACTORY) > 0 {
            let adj = |pred: &dyn Fn(usize) -> bool| w.map.hexes[h].neighbours.iter().any(|&x| pred(x));
            if adj(&|x| is_city(&w, x) && corrupted(g, x)) {
                n += factory;
            }
            if adj(&|x| clean_city(&w, g, x)) {
                n += clean;
            }
        }
        if g.tokens(h, STATION) > 0 && corrupted(g, h) {
            n += stations;
        }
        if n > 0 {
            plan.push((h, n));
        }
    }
    for (h, n) in plan {
        place(g, cx, h, n);
    }
    Ok(())
}

/// Places `n` adversary ambassadors from the supply on hex `h`. With the
/// supply empty the adversary is starved (players lose).
pub fn place(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) {
    for _ in 0..n {
        if g.var(SUPPLY) <= 0 {
            cx.set_key(g, STARVED.into(), 1);
            return;
        }
        cx.add_key(g, SUPPLY.into(), -1);
        cx.adversary_ops(g, 1);
        arrive(g, cx, h);
    }
}

/// One adversary ambassador arrives on hex `h` (from the supply or by
/// spreading): it cancels a player ambassador there (both are removed, no
/// Unity); with overflow, it corrupts a city or station already holding one.
fn arrive(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    if let Some(s) = player_ambassador(g, h) {
        cx.add_tokens(g, h, &ambassador(s), -1);
        cx.add_key(g, SUPPLY.into(), 1);
        cx.tally(g, "adversary.cancelled", 1);
        return;
    }
    if w.variant("adversary") == Some("overflow") && g.tokens(h, ADV_AMBASSADOR) > 0 && !corrupted(g, h) {
        if is_city(&w, h) {
            cx.add_key(g, SUPPLY.into(), 1);
            corrupt_city(g, cx, h);
            return;
        }
        if g.tokens(h, STATION) > 0 {
            cx.add_key(g, SUPPLY.into(), 1);
            corrupt_station(g, cx, h);
            return;
        }
    }
    cx.add_tokens(g, h, ADV_AMBASSADOR, 1);
}

/// Removes up to `n` adversary ambassadors from hex `h` to the supply.
pub fn remove(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> i64 {
    let k = n.min(g.tokens(h, ADV_AMBASSADOR)).max(0);
    if k > 0 {
        cx.add_tokens(g, h, ADV_AMBASSADOR, -k);
        cx.add_key(g, SUPPLY.into(), k);
    }
    k
}

/// Removes every adversary ambassador in country `c` to the supply (one
/// tally for them all); returns how many.
pub fn remove_all_in_country(g: &mut GameState, cx: &mut Cx, c: usize) -> i64 {
    let w = cx.world;
    let mut removed = 0;
    for &h in &w.map.countries[c].hexes {
        let n = g.tokens(h, ADV_AMBASSADOR);
        removed += remove(g, cx, h, n);
    }
    cx.tally(g, "adversary.ambassadors_removed", removed);
    removed
}

/// An adversary troop lands on hex `h` (the caller counts it).
pub fn place_troop(g: &mut GameState, cx: &mut Cx, h: usize) {
    cx.add_tokens(g, h, TROOP, 1);
}

/// City `h` is corrupted: the adversary's cube goes on (a player cube
/// comes off), and its adversary ambassadors return to the supply.
pub fn corrupt_city(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    if let Some(s) = cube_owner(g, h) {
        cx.add_tokens(g, h, &cube(s), -1);
    }
    cx.add_tokens(g, h, ADV_CUBE, 1);
    let n = g.tokens(h, ADV_AMBASSADOR);
    remove(g, cx, h, n);
    cx.tally(g, "adversary.corruptions", 1);
    if g.round > 0 && g.var(FIRST_CORRUPTION) == 0 {
        cx.set_key(g, FIRST_CORRUPTION.into(), g.round as i64);
        cx.tally(g, "adversary.first_corruption_round", g.round as i64);
    }
    cx.note(Some(h), || format!("{} is corrupted", hex_name(&w, h)));
}

/// Research station `h` flips to the adversary.
pub fn corrupt_station(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    if let Some(s) = cube_owner(g, h) {
        cx.add_tokens(g, h, &cube(s), -1);
    }
    if !corrupted(g, h) {
        cx.add_tokens(g, h, ADV_CUBE, 1);
        cx.tally(g, "adversary.stations_flipped", 1);
        cx.note(Some(h), || format!("The station at {} flips to the adversary", hex_name(&w, h)));
    }
}

/// Where an ambassador spreading from `from` may go: non-ocean hexes within
/// `range`, without an adversary ambassador unless a clean city (cities
/// collect them), and not shielded. Returns the best by priority (player
/// owned, then clean cities, then the rest), then distance.
fn destinations(w: &World, g: &GameState, from: usize, range: i32) -> Vec<usize> {
    let shields: Vec<usize> = (0..g.seats.len())
        .filter(|&s| cards::passives(w, g, s).iter().any(|p| matches!(p, Passive::SpreadShield)))
        .collect();
    let mut best: Option<(u8, i32)> = None;
    let mut out = Vec::new();
    for h in 0..g.hexes.len() {
        let d = w.map.distance(from, h);
        if h == from || d > range || is_ocean(w, h) {
            continue;
        }
        if shields.iter().any(|&s| g.tokens(h, &dc(s)) > 0) {
            continue;
        }
        let city = clean_city(w, g, h);
        if g.tokens(h, ADV_AMBASSADOR) > 0 && !city {
            continue;
        }
        let tier = if player_owned(g, h) {
            0
        } else if city {
            1
        } else {
            2
        };
        let key = (tier, d);
        match best {
            Some(b) if key > b => {}
            Some(b) if key == b => out.push(h),
            _ => {
                best = Some(key);
                out = vec![h];
            }
        }
    }
    out
}

/// The spread step, in country `scope` or everywhere: each hex (fixed when
/// the step starts) holding several adversary ambassadors, except a city
/// that is not corrupted, keeps one and sends each other one to the best
/// destination in range (players choose among equals).
pub fn spread(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) -> Res {
    let w = cx.world;
    let range = w.int("adversary.spread_range")
        + twist_sum(&w, g, |t| match t {
            Twist::Reach { amount } => Some(*amount),
            _ => None,
        });
    let sources: Vec<usize> = scope_hexes(&w, scope)
        .into_iter()
        .filter(|&h| g.tokens(h, ADV_AMBASSADOR) > 1 && !(is_city(&w, h) && !corrupted(g, h)))
        .collect();
    for from in sources {
        let n = g.tokens(from, ADV_AMBASSADOR) - 1;
        for _ in 0..n {
            let dest = destinations(&w, g, from, range as i32);
            if dest.is_empty() {
                break;
            }
            let opts = dest.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Adversary spread: an ambassador from {} goes to which hex?", hex_name(&w, from));
            let Some(i) = cx.choose(0, &prompt, opts)? else {
                break;
            };
            cx.add_tokens(g, from, ADV_AMBASSADOR, -1);
            cx.adversary_ops(g, 1);
            arrive(g, cx, dest[i]);
        }
    }
    Ok(())
}

/// Removal cost of an adversary piece: an ambassador 1, a troop `board.troop_health`.
fn cost(w: &World, kind: &str) -> i64 {
    if kind == TROOP {
        w.int("board.troop_health")
    } else {
        1
    }
}

/// Removes adversary pieces worth `amount` in hex `h` for seat `seat`,
/// one at a time; the seat chooses each among those the rest can pay for.
pub fn damage(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, amount: i64) -> Res {
    let w = cx.world;
    let mut left = amount;
    loop {
        let kinds: Vec<&str> = [ADV_AMBASSADOR, TROOP]
            .into_iter()
            .filter(|k| g.tokens(h, k) > 0 && cost(&w, k) <= left)
            .collect();
        if kinds.is_empty() {
            return Ok(());
        }
        let opts = kinds.iter().map(|k| opt(format!("remove {k} at {}", hex_name(&w, h)))).collect();
        let prompt = format!("Seat {seat}: remove which piece ({left} left)?");
        let Some(i) = cx.choose(seat, &prompt, opts)? else {
            return Ok(());
        };
        left -= cost(&w, kinds[i]);
        remove_piece(g, cx, Some(seat), h, kinds[i]);
    }
}

/// Removes one adversary piece of `kind` from hex `h`. A removed troop
/// sways toward Unity.
pub fn remove_piece(g: &mut GameState, cx: &mut Cx, seat: Option<usize>, h: usize, kind: &str) {
    if kind == TROOP {
        cx.add_tokens(g, h, TROOP, -1);
        cx.tally(g, "adversary.troops_removed", 1);
        if let Some(s) = seat {
            cx.set_key(g, removed_troop(s), 1);
        }
        alignment::for_troop_removed(g, cx, seat);
    } else {
        remove(g, cx, h, 1);
        cx.tally(g, "adversary.ambassadors_removed", 1);
    }
}

/// Removes every adversary ambassador and troop in hex `h`.
pub fn destroy_all(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let n = g.tokens(h, ADV_AMBASSADOR);
    for _ in 0..n {
        remove_piece(g, cx, Some(seat), h, ADV_AMBASSADOR);
    }
    for _ in 0..g.tokens(h, TROOP) {
        remove_piece(g, cx, Some(seat), h, TROOP);
    }
}

/// Removes one adversary ambassador in country `c` (players choose the hex).
pub fn pushback(g: &mut GameState, cx: &mut Cx, c: usize) -> Res {
    let w = cx.world;
    let hexes: Vec<usize> =
        w.map.countries[c].hexes.iter().copied().filter(|&h| g.tokens(h, ADV_AMBASSADOR) > 0).collect();
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    if let Some(i) = cx.choose(0, &format!("Unity: remove an adversary ambassador in country {c} from which hex?"), opts)? {
        remove_piece(g, cx, None, hexes[i], ADV_AMBASSADOR);
    }
    Ok(())
}

/// In each country, the clean city with the most adversary ambassadors
/// (ties: lower hex number) gains `n` more (Discord cards).
pub fn infiltrate(g: &mut GameState, cx: &mut Cx, n: i64) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        if let Some(h) = most_infiltrated_city(&w, g, c, |h| clean_city(&w, g, h)) {
            cx.note(Some(h), || format!("Discord: ambassadors at {}", hex_name(&w, h)));
            place(g, cx, h, n);
        }
    }
}

/// In country `c`, the hex passing `pred` with the most adversary
/// ambassadors (ties: the first in the country's hex order), if any.
pub fn most_infiltrated_city(w: &World, g: &GameState, c: usize, pred: impl Fn(usize) -> bool) -> Option<usize> {
    let mut best: Option<(usize, i64)> = None;
    for &h in &w.map.countries[c].hexes {
        if pred(h) {
            let k = g.tokens(h, ADV_AMBASSADOR);
            if best.is_none_or(|(_, b)| k > b) {
                best = Some((h, k));
            }
        }
    }
    best.map(|(h, _)| h)
}

/// What still counts against the win at bar `level`: corrupted cities
/// always; adversary ambassadors below bar 1; adversary troops below bar 2.
pub fn presence(w: &World, g: &GameState, level: i64) -> i64 {
    let mut n = 0;
    for h in 0..g.hexes.len() {
        if is_city(w, h) && corrupted(g, h) {
            n += 1;
        }
        if level < 1 {
            n += g.tokens(h, ADV_AMBASSADOR);
        }
        if level < 2 {
            n += g.tokens(h, TROOP);
        }
    }
    n
}

pub fn under_bar(w: &World, g: &GameState, level: i64) -> bool {
    presence(w, g, level) == 0
}

pub fn win_text(_w: &World, level: i64) -> String {
    match level {
        0 => "win when no city is corrupted and no adversary ambassador or troop is on the board".into(),
        1 => "win when no city is corrupted and no adversary troop is on the board".into(),
        _ => "win when no city is corrupted".into(),
    }
}

/// A board loss, if any: every city corrupted, or the supply starved.
/// (The deck running out ends the game in the build step.)
pub fn loss(w: &World, g: &GameState) -> Option<(&'static str, String)> {
    if g.var(STARVED) > 0 {
        return Some(("supply", "the adversary's ambassador supply ran out".into()));
    }
    let cities: Vec<usize> = (0..g.hexes.len()).filter(|&h| is_city(w, h)).collect();
    if !cities.is_empty() && cities.iter().all(|&h| corrupted(g, h)) {
        return Some(("cities", "every city is corrupted".into()));
    }
    None
}
