//! The board (014, 016): the printed country pieces (cities, open land,
//! ocean, industrial zones; military bases on the fronts), player presence
//! (owned here; AI boards read the track through `track_taken`), the setup,
//! the production/war clock, production (factories grow from cities and
//! make corp units and chips, front battles on war steps, adversary
//! troops, cities, stations, megaprojects), player ambassadors and site
//! claims, the standard actions and the round-end cleanup.

use crate::action::Action;
use crate::content::{Op, RoundFlag, Side};
use crate::key::KeyBuf;
use crate::module::{
    scheduled, Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::*;
use super::{adversary, ai, alignment, cards, economy};

pub static DEF: ModuleDef = ModuleDef {
    id: "board",
    summary: "Country pieces, cities, buildings, bases and fronts; production, war, ambassadors, standard actions",
    requires: &["adversary", "ai", "alignment"],
    variants: &[VariantDef {
        name: "corps",
        summary: "Each country is a printed piece with cities, a research station, factories and a corp's bases",
        params: &[
            ParamSpec::int("start_corrupted", 0, 3, "cities per country that start corrupted"),
            ParamSpec::int("start_ambassadors", 0, 10, "player ambassadors each player starts with"),
            ParamSpec::int("unit_cap", 0, 20, "corp units a base holds at most"),
            ParamSpec::int("factory_units", 0, 5, "corp units each factory makes on a production step at each of its corp's bases"),
            ParamSpec::list("clock", 0, 3, "the printed round track, per round: 1 production, 2 war, 3 both (the last entry repeats)"),
            ParamSpec::list("corp_energy", 0, 10, "energy cubes each city of country i makes a round (a megacorp parameter; the list repeats)"),
            ParamSpec::int("city_energy_credits", 0, 20, "credits for a neutral city's energy cube, bought by a player with presence next to it"),
            ParamSpec::int("aligned_bonus", 0, 10, "extra energy an aligned city gives its owner"),
            ParamSpec::int("station_cubes", 0, 5, "megaproject cubes a research station makes a round"),
            ParamSpec::int("project_credits", 0, 50, "credits each player cube pays when a megaproject completes"),
            ParamSpec::int("project_discord_per", 1, 10,"adversary cubes per Discord when a megaproject completes"),
            ParamSpec::int("divert_discord", 0, 5, "Discord per unit of a corrupted city's energy diverted to the adversary"),
            ParamSpec::int("ambassador_discord", 0, 5, "Discord per unit of energy an adversary ambassador on a clean or aligned city diverts"),
            ParamSpec::int("base_discord", 0, 10, "Discord an adversary ambassador on a producing base sways"),
            ParamSpec::int("base_unity", 0, 10, "Unity a player ambassador on a producing base sways"),
            ParamSpec::int("factory_unity", 0, 10, "Unity a player ambassador on a producing factory sways"),
            ParamSpec::int("hold_base", 0, 10, "a corrupted city's hold before adversary ambassadors: 1 more for each on or next to it"),
            ParamSpec::int("last_safe_stage", 0, 3, "while this round's adversary card is below this stage, nothing destroys a player's last presence (0: never safe)"),
            ParamSpec::int("hold_min", 1, 10, "player ambassadors that free a corrupted city, at the least"),
            ParamSpec::int("free_leaves", 0, 5, "adversary ambassadors left on a city as it frees (the piece ladder)"),
            ParamSpec::int("destruction_discord", 0, 5, "Discord each building destroyed in war sways"),
            ParamSpec::int("troop_spent", 0, 1, "1: an adversary troop that strikes a target is used up"),
            ParamSpec::int("troop_steps", 0, 5, "steps each adversary troop takes in the march: destroy a target on or next to it, else move 1 hex toward the nearest"),
            ParamSpec::int("troop_presence", 0, 4, "adversary troops strike presence from this stage of adversary card on (0: always, 4: never)"),
            ParamSpec::int("presence_marker", 0, 1, "1: a destroyed presence leaves a corruption marker on its hex"),
            ParamSpec::int("corruption_reach", 0, 1, "a city that corrupts destroys presence within this many hexes of it"),
            ParamSpec::int("war_reach", 0, 1, "war that destroys a factory destroys presence within this many hexes of it"),
            ParamSpec::int("presence_energy", 0, 20, "standard action: energy to place a presence"),
            ParamSpec::int("build_energy", 0, 20, "standard action: energy to build a research station"),
            ParamSpec::int("station_chips", 0, 20, "standard action: chips to build a research station"),
            ParamSpec::int("factory_chips", 0, 5, "chips a player-owned factory makes its owner on a production step"),
            ParamSpec::int("chips_credits", 0, 50, "standard action: credits for selling 1 chip"),
            ParamSpec::int("remove_energy", 0, 20, "standard action: energy to remove an adversary ambassador within reach"),
            ParamSpec::int("move_energy", 0, 20, "standard action: energy to move a player ambassador"),
            ParamSpec::int("ambassador_credits", 1, 50, "standard action: credits to gain a player ambassador (at least 1: a free untargeted action has no limit)"),
            ParamSpec::int("harden_energy", 1, 20, "standard action (Harden passive): energy a hardening token (at least 1: free, it has no limit)"),
            ParamSpec::int("sell_credits", 0, 20, "standard action: credits for selling a card from hand"),
            ParamSpec::int("align_energy", 0, 20, "standard action: energy (and 1 ambassador) to align a clean city"),
            ParamSpec::int("presence_credits", 1, 50, "the rich action: credits to place a presence, before its rise and the step's extra"),
            ParamSpec::int("presence_rise", 0, 20, "the rich action: credits more for each earlier use this round"),
            ParamSpec::int("presence_stage", 1, 3, "the rich action: from this stage of adversary card on"),
            ParamSpec::int("harden_credits", 1, 50, "standard project: credits for a hardening token"),
            ParamSpec::int("energy_credits", 1, 50, "standard project: credits for 1 energy"),
            ParamSpec::int("drain_credits", 1, 50, "standard project: credits to drain compute from an adversary project"),
            ParamSpec::int("drain_amount", 1, 20, "standard project: compute drained"),
        ],
        phases: &[
            PhaseSpec::decision("act", "standard actions and projects: presence, station, remove, move, ambassador, align, harden, chips, sell, buy energy, drain"),
            PhaseSpec::auto("bases", "on a production step: bases and factories make corp units and chips, cities grow factories"),
            PhaseSpec::auto("war", "on a war step, the corp units on either side of each front fight"),
            PhaseSpec::auto("march", "each adversary troop takes its steps: destroy a target next to it, else move toward the nearest"),
            PhaseSpec::auto("produce", "cities and research stations produce; ambassadors divert them, and one on a lone presence drains its owner's insight into adversary compute"),
            PhaseSpec::auto("cleanup", "remove the round's country flags"),
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// Country flags: up while their var is above 0, counted down in cleanup (1 lasts to
/// the end of the round; a threat sets 2, to the end of the next).
pub const FLAGS: [RoundFlag; 4] =
    [RoundFlag::Blackout, RoundFlag::Sabotage, RoundFlag::Shelter, RoundFlag::Embargo];

/// The var key of a country flag.
pub fn flag(f: RoundFlag, country: usize) -> String {
    format!("board.{}.{country}", f.name())
}

/// A country flag's count: above 0 while it is up.
pub fn flag_level(g: &GameState, f: RoundFlag, country: usize) -> i64 {
    g.var(&flag(f, country))
}

/// Whether a country flag is up, by its name (`"blackout"`, ...).
pub fn flagged(g: &GameState, f: RoundFlag, country: usize) -> bool {
    g.var(&flag(f, country)) > 0
}

/// Shelters country `c`'s buildings for the rest of the round.
pub fn shelter(g: &mut GameState, cx: &mut Cx, country: usize) {
    cx.set_key(g, flag(RoundFlag::Shelter, country), 1);
}

/// Cubes on country `c`'s megaproject: `None` for the adversary's.
pub fn project_key(c: usize, owner: Option<usize>) -> KeyBuf {
    let k = KeyBuf::new("board.project.").num(c).text(".");
    match owner {
        Some(s) => k.num(s),
        None => k.text("adversary"),
    }
}

/// Set once country `c`'s megaproject has completed.
pub fn project_done(c: usize) -> KeyBuf {
    KeyBuf::new("board.project_done.").num(c)
}

/// A standard action. `Action::Standard` carries its [`name`](Self::name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardAction {
    Presence,
    Station,
    Remove,
    Move,
    Ambassador,
    Align,
    Harden,
    Chips,
    // 024: the credit sinks, Terraforming Mars' standard projects.
    PresenceBuy,
    HardenBuy,
    Energy,
    Drain,
}

/// The standard actions, in the order they are offered and published.
pub const STANDARD: [StandardAction; 12] = [
    StandardAction::Presence,
    StandardAction::Station,
    StandardAction::Remove,
    StandardAction::Move,
    StandardAction::Ambassador,
    StandardAction::Align,
    StandardAction::Harden,
    StandardAction::Chips,
    StandardAction::PresenceBuy,
    StandardAction::HardenBuy,
    StandardAction::Energy,
    StandardAction::Drain,
];

impl StandardAction {
    /// The action's name: `Action::Standard.act`, and its tally `board.std.<name>`.
    pub const fn name(self) -> &'static str {
        match self {
            StandardAction::Presence => "presence",
            StandardAction::Station => "station",
            StandardAction::Remove => "remove",
            StandardAction::Move => "move",
            StandardAction::Ambassador => "ambassador",
            StandardAction::Align => "align",
            StandardAction::Harden => "harden",
            StandardAction::Chips => "chips",
            StandardAction::PresenceBuy => "presence_buy",
            StandardAction::HardenBuy => "harden_buy",
            StandardAction::Energy => "energy",
            StandardAction::Drain => "drain",
        }
    }

    /// The action called `name`, if there is one.
    pub fn parse(name: &str) -> Option<StandardAction> {
        STANDARD.into_iter().find(|a| a.name() == name)
    }
}

/// The clock's step for round `round`: 1 production, 2 war, 3 both.
pub fn clock_step(w: &World, round: u32) -> i64 {
    scheduled(w.list("board.clock"), round)
}

/// Whether this round has a production step (factories, troops, chips).
pub fn production_round(w: &World, g: &GameState) -> bool {
    clock_step(w, g.round) & 1 != 0
}

/// Whether this round is a war step (the clock, or this round's adversary card).
pub fn war_round(w: &World, g: &GameState) -> bool {
    clock_step(w, g.round) & 2 != 0 || adversary::war_twist(w, g)
}

/// Presence tokens seat `s` has taken off its AI board's track (the
/// starting one included); AI boards read their uncovered spaces from it.
pub fn track_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.track.").num(seat)
}

pub fn track_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track_key(seat)).max(0) as usize
}

/// Tokens seat `s` has taken off its board's second presence track (021).
pub fn track2_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.track2.").num(seat)
}

pub fn track2_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track2_key(seat)).max(0) as usize
}

/// Where seat `s` may place presence: a land hex holding no presence,
/// adjacent to one of its own (with none left, any such hex in its home
/// country).
pub fn presence_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    let mine = presence(seat);
    let free = |h: usize| !is_ocean(w, h) && presence_owner(g, h).is_none() && g.tokens(h, MARKER) == 0;
    if presence_count(g, seat) == 0 {
        let c = home_country(w, seat);
        return w.map.countries[c]
            .hexes
            .iter()
            .copied()
            .filter(|&h| free(h))
            .collect();
    }
    (0..g.hexes.len())
        .filter(|&h| {
            free(h)
                && w.map.hexes[h]
                    .neighbours
                    .iter()
                    .any(|&x| g.tokens(x, &mine) > 0)
        })
        .collect()
}

/// Seat `s`'s uses of the rich action this round (034).
pub fn rich_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.rich.").num(seat)
}

/// The cheapest next step's extra energy (034: printed on the track, each
/// track's last steps cost more), or `None` with both tracks empty.
fn cheapest_extra(w: &World, g: &GameState, seat: usize) -> Option<i64> {
    let b = ai::board(w, seat);
    let first = b.presence.get(track_taken(g, seat)).map(|u| u.extra);
    let second = b.presence2.get(track2_taken(g, seat)).map(|u| u.extra);
    first.into_iter().chain(second).min()
}

/// How a track's step extra is paid when a presence is placed.
#[derive(Clone, Copy, PartialEq)]
pub enum StepPay {
    /// Cards and growth place at no extra.
    Free,
    /// The standard action: 1 energy a step.
    Energy,
    /// The rich action: `board.energy_credits` credits a step.
    Credits,
}

/// Places one presence for `seat`: the next token off its track (uncovering
/// the space under it) onto a hex next to its presence. Returns whether placed.
pub fn place_presence(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res<bool> {
    place_presence_paying(g, cx, seat, StepPay::Free)
}

/// [`place_presence`], with the track's step extra paid by `pay`: only tracks
/// whose next step the seat can pay are offered.
pub fn place_presence_paying(g: &mut GameState, cx: &mut Cx, seat: usize, pay: StepPay) -> Res<bool> {
    let w = cx.world;
    if !ai::can_place(&w, g, seat) {
        return Ok(false);
    }
    let hexes = presence_hexes(&w, g, seat);
    // 021: with two tracks the player chooses which one's next token to take.
    let b = ai::board(&w, seat);
    let (t1, t2) = (track_taken(g, seat), track2_taken(g, seat));
    let can_pay = |u: &crate::content::Uncover| match pay {
        StepPay::Free => true,
        StepPay::Energy => g.resource(seat, Resource::Energy) >= u.extra,
        StepPay::Credits => g.resource(seat, Resource::Credits) >= u.extra * w.int("board.energy_credits"),
    };
    let mut tracks: Vec<(bool, &crate::content::Uncover)> = Vec::new();
    if let Some(u) = b.presence.get(t1).filter(|u| can_pay(u)) {
        tracks.push((false, u));
    }
    if let Some(u) = b.presence2.get(t2).filter(|u| can_pay(u)) {
        tracks.push((true, u));
    }
    let opts = tracks
        .iter()
        .map(|(two, u)| {
            opt(format!(
                "track {}: uncovers {}",
                if *two { 2 } else { 1 },
                ai::uncover_label(u)
            ))
        })
        .collect();
    let prompt = format!("Seat {seat}: take presence off which track?");
    let Some(k) = cx.choose(seat, &prompt, opts)? else {
        return Ok(false);
    };
    let second = tracks[k].0;
    let extra = tracks[k].1.extra;
    let what = format!("uncovers {}", ai::uncover_label(tracks[k].1));
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!("Seat {seat}: place presence ({what}) on which hex?");
    let Some(j) = cx.choose(seat, &prompt, opts)? else {
        return Ok(false);
    };
    match pay {
        StepPay::Free => {}
        StepPay::Energy => cx.add_resource(g, seat, Resource::Energy, -extra),
        StepPay::Credits => cx.add_resource(g, seat, Resource::Credits, -extra * w.int("board.energy_credits")),
    }
    if second {
        cx.add_key(g, track2_key(seat).into(), 1);
    } else {
        cx.add_key(g, track_key(seat).into(), 1);
    }
    cx.add_tokens(g, hexes[j], &presence(seat), 1);
    cx.tally(g, "board.presence_placed", 1);
    Ok(true)
}

/// Whether seat `s` can claim the site on hex `h` (align a city, take a
/// station or factory): its presence is on or next to `h`, and no node is.
pub fn can_claim(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    in_reach(w, g, seat, h)
        && !next_to(w, h, |x| g.tokens(x, NODE) > 0)
        && !flagged(g, RoundFlag::Embargo, w.map.hexes[h].country as usize)
}

#[derive(Debug)]
struct Board;

fn build(_b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Board))
}

/// What seat `s` pays for a standard action now: the rich action rises with
/// each use this round; a presence also needs its cheapest step's extra.
fn seat_cost(w: &World, g: &GameState, seat: usize, act: StandardAction) -> (i64, i64, i64) {
    let (e, cr, chips) = std_cost(w, act);
    let extra = cheapest_extra(w, g, seat).unwrap_or(0);
    match act {
        StandardAction::Presence => (e + extra, cr, chips),
        StandardAction::PresenceBuy => (
            e,
            cr + g.var(&rich_key(seat)) * w.int("board.presence_rise") + extra * w.int("board.energy_credits"),
            chips,
        ),
        _ => (e, cr, chips),
    }
}

/// A standard action's printed cost: (energy, credits, chips).
pub fn std_cost(w: &World, act: StandardAction) -> (i64, i64, i64) {
    let i = |k: &str| w.int(&format!("board.{k}"));
    match act {
        StandardAction::Presence => (i("presence_energy"), 0, 0),
        StandardAction::Station => (i("build_energy"), 0, i("station_chips")),
        StandardAction::Remove => (i("remove_energy"), 0, 0),
        StandardAction::Move => (i("move_energy"), 0, 0),
        StandardAction::Ambassador => (0, i("ambassador_credits"), 0),
        StandardAction::Align => (i("align_energy"), 0, 0),
        StandardAction::Harden => (i("harden_energy"), 0, 0),
        StandardAction::Chips => (0, 0, 1),
        StandardAction::PresenceBuy => (0, i("presence_credits"), 0),
        StandardAction::HardenBuy => (0, i("harden_credits"), 0),
        StandardAction::Energy => (0, i("energy_credits"), 0),
        StandardAction::Drain => (0, i("drain_credits"), 0),
    }
}

/// What a standard action does, for the view.
pub fn std_text(act: StandardAction) -> &'static str {
    match act {
        StandardAction::Presence => "place a presence next to yours (uncovers your track's next space)",
        StandardAction::Station => "build a research station in reach (yours)",
        StandardAction::Remove => "remove an adversary ambassador in reach, or else a corruption marker",
        StandardAction::Move => "move a player ambassador onto a piece in reach",
        StandardAction::Ambassador => "gain a player ambassador",
        StandardAction::Align => "align a clean city you can claim (1 ambassador)",
        StandardAction::Harden => "harden a building in reach (Harden passive)",
        StandardAction::Chips => "sell 1 chip for credits",
        StandardAction::PresenceBuy => "the rich action: place a presence next to yours; steep, and rises with each use a round",
        StandardAction::HardenBuy => "standard project: harden a building in reach",
        StandardAction::Energy => "standard project: gain 1 energy",
        StandardAction::Drain => "standard project: remove compute from an adversary project",
    }
}

fn can_afford(g: &GameState, seat: usize, (e, cr, chips): (i64, i64, i64)) -> bool {
    g.resource(seat, Resource::Energy) >= e
        && g.resource(seat, Resource::Credits) >= cr
        && g.resource(seat, Resource::Chips) >= chips
}

/// The hexes a standard action of `seat` may target (empty: not offered).
fn std_targets(w: &World, g: &GameState, seat: usize, act: StandardAction) -> Vec<usize> {
    let reach = |h: usize| in_reach(w, g, seat, h);
    let all = 0..g.hexes.len();
    match act {
        StandardAction::Presence | StandardAction::PresenceBuy => {
            let open = act == StandardAction::Presence || g.var(adversary::STAGE) >= w.int("board.presence_stage");
            if open && ai::can_place(w, g, seat) {
                presence_hexes(w, g, seat)
            } else {
                Vec::new()
            }
        }
        StandardAction::Station => all
            .filter(|&h| buildable(w, g, h) && !w.map.hexes[h].industrial && reach(h))
            .collect(),
        StandardAction::Remove => all
            .filter(|&h| reach(h) && (g.tokens(h, ADV_AMBASSADOR) > 0 || g.tokens(h, MARKER) > 0))
            .collect(),
        // Onto a piece in reach, from the supply or another hex: never onto the
        // hex the seat's only ambassador already holds (020: that charged the
        // energy and moved nothing).
        StandardAction::Move => {
            let supply = g.resource(seat, Resource::Ambassadors) > 0;
            let on_board = ambassadors_on_board(g, seat);
            all.filter(|&h| {
                (supply || on_board > g.tokens(h, &ambassador(seat)))
                    && reach(h)
                    && has_piece(w, g, h)
            })
            .collect()
        }
        StandardAction::Align => {
            if g.resource(seat, Resource::Ambassadors) > 0 {
                all.filter(|&h| clean_city(w, g, h) && can_claim(w, g, seat, h))
                    .collect()
            } else {
                Vec::new()
            }
        }
        StandardAction::Harden => {
            if cards::has_passive(w, g, seat, |p| matches!(p, Op::HardenAction)) {
                all.filter(|&h| reach(h) && hardenable(g, h)).collect()
            } else {
                Vec::new()
            }
        }
        StandardAction::HardenBuy => all.filter(|&h| reach(h) && hardenable(g, h)).collect(),
        StandardAction::Drain => {
            if adversary::projects(g).iter().any(|p| g.var(&adversary::project_key(p)) > 0) {
                vec![usize::MAX]
            } else {
                Vec::new()
            }
        }
        // Gaining an ambassador, credits or energy needs no target.
        StandardAction::Ambassador | StandardAction::Chips | StandardAction::Energy => {
            vec![usize::MAX]
        }
    }
}

/// A standard action's effect, paid for and tallied; `targets` are its
/// [`std_targets`].
fn std_resolve(
    g: &mut GameState,
    cx: &mut Cx,
    s: usize,
    act: StandardAction,
    targets: &[usize],
) -> Res {
    let w = cx.world;
    let on_hex = |cx: &mut Cx| -> Res<Option<usize>> {
        let opts = targets.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = format!("Seat {s}: {} on which hex?", act.name());
        Ok(cx.choose(s, &prompt, opts)?.map(|i| targets[i]))
    };
    match act {
        StandardAction::Ambassador => cx.add_resource(g, s, Resource::Ambassadors, 1),
        StandardAction::Chips => {
            cx.add_resource(g, s, Resource::Credits, w.int("board.chips_credits"));
            cx.tally(g, "board.chips_sold", 1);
        }
        StandardAction::Presence => {
            place_presence_paying(g, cx, s, StepPay::Energy)?;
        }
        StandardAction::PresenceBuy => {
            // The base price and the rise; the step's extra follows the track.
            let rise = g.var(&rich_key(s)) * w.int("board.presence_rise");
            cx.add_resource(g, s, Resource::Credits, -rise);
            cx.add_key(g, rich_key(s).into(), 1);
            place_presence_paying(g, cx, s, StepPay::Credits)?;
        }
        StandardAction::Energy => cx.add_resource(g, s, Resource::Energy, 1),
        StandardAction::Drain => {
            let ids: Vec<crate::key::Key> = adversary::projects(g)
                .into_iter()
                .filter(|p| g.var(&adversary::project_key(p)) > 0)
                .collect();
            let opts = ids.iter().map(|p| opt(p.clone())).collect();
            let prompt = format!("Seat {s}: remove compute from which adversary project?");
            if let Some(i) = cx.choose(s, &prompt, opts)? {
                adversary::drain_project(g, cx, &ids[i], w.int("board.drain_amount"));
            }
        }
        StandardAction::Station => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, STATION, 1);
                cx.add_tokens(g, h, &cube(s), 1);
            }
        }
        StandardAction::Remove => {
            if let Some(h) = on_hex(cx)? {
                if g.tokens(h, ADV_AMBASSADOR) > 0 {
                    adversary::remove_piece(g, cx, Some(s), h, ADV_AMBASSADOR);
                } else {
                    cx.add_tokens(g, h, MARKER, -1);
                    cx.tally(g, "board.markers_removed", 1);
                }
            }
        }
        StandardAction::Move => {
            if let Some(h) = on_hex(cx)? {
                move_ambassador(g, cx, s, h)?;
            }
        }
        StandardAction::Align => {
            if let Some(h) = on_hex(cx)? {
                cx.add_resource(g, s, Resource::Ambassadors, -1);
                cx.add_tokens(g, h, &cube(s), 1);
                cx.tally(g, "board.cities_aligned", 1);
            }
        }
        StandardAction::Harden | StandardAction::HardenBuy => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, HARDEN, 1);
            }
        }
    }
    Ok(())
}

pub fn ambassadors_on_board(g: &GameState, seat: usize) -> i64 {
    let k = ambassador(seat);
    (0..g.hexes.len()).map(|h| g.tokens(h, &k)).sum()
}

/// Seat `s`'s player ambassador arrives on hex `h`: it cancels an
/// adversary ambassador there (both removed, no Unity); on a neutral
/// station or factory the seat can claim it makes the site the seat's; on a
/// corrupted city, enough of them free it. Otherwise it waits on the piece.
pub fn place_ambassador(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let w = cx.world;
    if g.tokens(h, ADV_AMBASSADOR) > 0 {
        adversary::remove(g, cx, h, 1);
        cx.tally(g, "board.cancelled", 1);
        return;
    }
    if has_building(g, h) && !corrupted(g, h) && cube_owner(g, h).is_none() && can_claim(&w, g, seat, h) {
        cx.add_tokens(g, h, &cube(seat), 1);
        let what = if g.tokens(h, STATION) > 0 {
            "station"
        } else {
            "factory"
        };
        cx.tally(
            g,
            &format!(
                "board.{}_owned",
                if what == "station" {
                    "stations"
                } else {
                    "factories"
                }
            ),
            1,
        );
        cx.note(Some(h), || {
            format!("Seat {seat} takes the {what} at {}", hex_name(&w, h))
        });
        return;
    }
    cx.add_tokens(g, h, &ambassador(seat), 1);
}

/// Whether an adversary ambassador stands on hex `h` or an adjacent hex
/// (each adds 1 to a corrupted city's [`hold`]).
pub fn guarded(w: &World, g: &GameState, h: usize) -> bool {
    next_to(w, h, |x| g.tokens(x, ADV_AMBASSADOR) > 0)
}

/// A corrupted city's hold (027): the player ambassadors, from any players,
/// that free it. `hold_base`, plus 1 for each adversary ambassador on or
/// next to it, never below `hold_min`.
pub fn hold(w: &World, g: &GameState, h: usize) -> i64 {
    let near: i64 = g.tokens(h, ADV_AMBASSADOR)
        + w.map.hexes[h]
            .neighbours
            .iter()
            .map(|&x| g.tokens(x, ADV_AMBASSADOR))
            .sum::<i64>();
    (w.int("board.hold_base") + near).max(w.int("board.hold_min"))
}

/// Player ambassadors on hex `h`, from all seats.
pub fn ambassadors_on(g: &GameState, h: usize) -> i64 {
    (0..g.seats.len()).map(|s| g.tokens(h, &ambassador(s))).sum()
}

/// City `h` frees: the adversary's cube comes off, Unity sways, and it drops
/// down the piece ladder rather than off the board (027: `free_leaves`
/// adversary ambassadors stay on it).
pub fn liberate(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    cx.add_tokens(g, h, ADV_CUBE, -1);
    cx.tally(g, "board.cities_freed", 1);
    alignment::for_city_freed(g, cx);
    adversary::place(g, cx, h, w.int("board.free_leaves"));
    cx.note(Some(h), || format!("{} is freed", hex_name(&w, h)));
}

/// 027: a corrupted city frees the moment player ambassadors on it, from any
/// players, reach its [`hold`]; that many are used up, lower seats first.
/// Checked after every action and automatic phase.
fn free_cities(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        if !is_city(&w, h) || !corrupted(g, h) {
            continue;
        }
        let need = hold(&w, g, h);
        if ambassadors_on(g, h) < need {
            continue;
        }
        let mut left = need;
        for s in 0..g.seats.len() {
            let k = g.tokens(h, &ambassador(s)).min(left);
            cx.add_tokens(g, h, &ambassador(s), -k);
            left -= k;
        }
        liberate(g, cx, h);
    }
}

/// Moves one of `seat`'s ambassadors (from its supply or a hex, the seat
/// chooses) onto hex `to`.
pub fn move_ambassador(g: &mut GameState, cx: &mut Cx, seat: usize, to: usize) -> Res {
    let w = cx.world;
    let k = ambassador(seat);
    let from: Vec<usize> = (0..g.hexes.len())
        .filter(|&h| h != to && g.tokens(h, &k) > 0)
        .collect();
    let mut opts = Vec::new();
    if g.resource(seat, Resource::Ambassadors) > 0 {
        opts.push(opt("from your supply"));
    }
    let off = opts.len();
    opts.extend(from.iter().map(|&h| hex_opt(&w, h)));
    let prompt = format!(
        "Seat {seat}: move which ambassador to {}?",
        hex_name(&w, to)
    );
    let Some(i) = cx.choose(seat, &prompt, opts)? else {
        return Ok(());
    };
    if i < off {
        cx.add_resource(g, seat, Resource::Ambassadors, -1);
    } else {
        cx.add_tokens(g, from[i - off], &k, -1);
    }
    place_ambassador(g, cx, seat, to);
    Ok(())
}

/// What destroys a piece: it decides the corp units' help, the Sway and the
/// tally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// An adversary troop's strike.
    Troop,
    /// Corp troops crossing a front.
    War,
    /// A threat, Discord card or project, named for the tally.
    Effect(&'static str),
}

impl Cause {
    fn name(self) -> &'static str {
        match self {
            Cause::Troop => "march",
            Cause::War => "war",
            Cause::Effect(n) => n,
        }
    }
}

/// Whether seat `s`'s presence is safe as its last (027): while this round's
/// adversary card is below `last_safe_stage` (stage II), nothing destroys a
/// player's last presence.
pub fn last_presence_safe(w: &World, g: &GameState, s: usize) -> bool {
    g.var(adversary::STAGE) < w.int("board.last_safe_stage") && presence_count(g, s) <= 1
}

/// Seat `s` loses its presence on hex `h`, unless it is safe as their last.
/// It leaves the game, as in Spirit Island (034: what it uncovered stays
/// uncovered), and leaves a corruption marker on the hex unless the hex has
/// corruption already. The tally names `source`. Returns whether it went.
pub fn lose_presence(g: &mut GameState, cx: &mut Cx, h: usize, s: usize, source: &str) -> bool {
    let w = cx.world;
    if last_presence_safe(&w, g, s) {
        cx.tally(g, "board.last_presence_safe", 1);
        return false;
    }
    cx.add_tokens(g, h, &presence(s), -1);
    let tainted = g.tokens(h, MARKER) > 0 || g.tokens(h, NODE) > 0 || corrupted(g, h);
    if w.int("board.presence_marker") > 0 && !tainted {
        cx.add_tokens(g, h, MARKER, 1);
        cx.tally(g, "board.markers", 1);
    }
    cx.tally(g, "board.presence_lost", 1);
    cx.tally(g, &format!("board.presence_lost.{source}"), 1);
    true
}

/// A piece of `kind` on hex `h` would be destroyed. A presence safe as its
/// owner's last stays. A Shelter Order in
/// its country, a corp unit at a base next to it (adversary strikes only,
/// [`Cause::Troop`]), a hardening token, a player ambassador on the hex, or an
/// Aegis Dome Array absorbs it (in that order); otherwise it goes, and a war
/// destruction (not `by_troop`) sways toward Discord. Returns whether it was
/// destroyed.
pub fn destroy_building(
    g: &mut GameState,
    cx: &mut Cx,
    h: usize,
    kind: &str,
    cause: Cause,
) -> bool {
    let w = cx.world;
    let c = w.map.hexes[h].country as usize;
    let owner = (0..g.seats.len()).find(|&s| kind == presence(s));
    if owner.is_some_and(|s| last_presence_safe(&w, g, s)) {
        cx.tally(g, "board.last_presence_safe", 1);
        return false;
    }
    let by_troop = cause == Cause::Troop;
    if flag_level(g, RoundFlag::Shelter, c) > 0 {
        return false;
    }
    if by_troop {
        let guard = std::iter::once(h)
            .chain(w.map.hexes[h].neighbours.iter().copied())
            .find(|&b| is_base(&w, b) && g.tokens(b, UNIT) > 0);
        if let Some(b) = guard {
            cx.add_tokens(g, b, UNIT, -1);
            cx.tally(g, "board.base_absorbed", 1);
            return false;
        }
    }
    if g.tokens(h, HARDEN) > 0 {
        cx.add_tokens(g, h, HARDEN, -1);
        cx.tally(g, "board.hardening_used", 1);
        return false;
    }
    if let Some(s) = player_ambassador(g, h) {
        cx.add_tokens(g, h, &ambassador(s), -1);
        cx.tally(g, "board.ambassador_shield", 1);
        return false;
    }
    if civic_shelter(g, cx, c) {
        return false;
    }
    match owner {
        Some(s) => {
            lose_presence(g, cx, h, s, cause.name());
        }
        None => {
            cx.add_tokens(g, h, kind, -1);
        }
    }
    if kind == STATION || kind == FACTORY {
        if let Some(s) = cube_owner(g, h) {
            cx.add_tokens(g, h, &cube(s), -1);
        }
    }
    if kind == FACTORY {
        cx.tally(g, "board.factories_destroyed", 1);
    }
    cx.tally(g, "board.destroyed", 1);
    cx.note(Some(h), || {
        format!("{} at {} destroyed", piece_name(kind), hex_name(&w, h))
    });
    if cause == Cause::War {
        alignment::sway(g, cx, Side::Discord, w.int("board.destruction_discord"));
    }
    true
}

/// Seat `s` destroys one of its presence in `hexes`, its choice (028): a hit
/// on the player, not a hex. Absorbed as any destruction is, so a player may
/// pick a token it has shielded. Safe as a last presence in stage I.
pub fn destroy_own_presence(
    g: &mut GameState,
    cx: &mut Cx,
    s: usize,
    hexes: &[usize],
    cause: Cause,
) -> Res {
    let w = cx.world;
    let mine: Vec<usize> = hexes.iter().copied().filter(|&h| g.tokens(h, presence(s)) > 0).collect();
    if mine.is_empty() {
        return Ok(());
    }
    if last_presence_safe(&w, g, s) {
        cx.tally(g, "board.last_presence_safe", 1);
        return Ok(());
    }
    let opts = mine.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!("Seat {s}: destroy which of your presence?");
    if let Some(i) = cx.choose(s, &prompt, opts)? {
        destroy_building(g, cx, mine[i], presence(s), cause);
    }
    Ok(())
}

/// Uses a seat's `CivicShelter` passive (Aegis Dome Array) for a building
/// destruction in `country`, if one applies.
fn civic_shelter(g: &mut GameState, cx: &mut Cx, country: usize) -> bool {
    let w = cx.world;
    for s in 0..g.seats.len() {
        if g.var(&cards::civic_used(s)) == 0
            && has_presence_in(&w, g, s, country)
            && cards::has_passive(&w, g, s, |p| matches!(p, Op::CivicShelter))
        {
            cx.set_key(g, cards::civic_used(s), 1);
            return true;
        }
    }
    false
}

/// The nearest hex to `from` among `hexes` (ties: country, then lower hex number).
fn nearest(w: &World, from: usize, hexes: impl Iterator<Item = usize>) -> Option<usize> {
    hexes.min_by_key(|&h| {
        (
            w.map.distance(from, h),
            w.map.hexes[h].country,
            w.map.hexes[h].number,
        )
    })
}

fn add_unit(g: &mut GameState, cx: &mut Cx, base: usize) {
    let w = cx.world;
    if g.tokens(base, UNIT) < w.int("board.unit_cap") {
        cx.add_tokens(g, base, UNIT, 1);
        cx.tally(g, "board.units_built", 1);
    }
}

/// Production step 1, on a production step of the clock: bases, then
/// factories, make their corp's units (a factory 1 at each of its corp's
/// bases); an ambassador on the piece diverts it and is removed; an
/// adversary-owned factory makes an adversary troop. Player-owned factories
/// make chips. Then the cities grow factories.
fn produce_units(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    if !production_round(&w, g) {
        return Ok(());
    }
    for h in 0..g.hexes.len() {
        if !is_base(&w, h) {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 {
            adversary::remove(g, cx, h, 1);
            cx.tally(g, "board.bases_diverted", 1);
            alignment::sway(g, cx, Side::Discord, w.int("board.base_discord"));
        } else if let Some(s) = player_ambassador(g, h) {
            cx.add_tokens(g, h, &ambassador(s), -1);
            cx.tally(g, "board.bases_united", 1);
            alignment::sway(g, cx, Side::Unity, w.int("board.base_unity"));
        } else {
            add_unit(g, cx, h);
        }
    }
    let per = w.int("board.factory_units");
    let chips = w.int("board.factory_chips");
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 {
            continue;
        }
        if let Some(s) = cube_owner(g, h) {
            cx.add_resource(g, s, Resource::Chips, chips);
            cx.tally(g, "board.chips_made", chips);
        }
        if per == 0 {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 || corrupted(g, h) {
            // 028: an adversary-owned factory uses its ambassador up too.
            if g.tokens(h, ADV_AMBASSADOR) > 0 {
                adversary::remove(g, cx, h, 1);
            } else if let Some(s) = player_ambassador(g, h) {
                // 022 (D1): a player ambassador on an adversary-owned factory
                // is used up for Unity; the troop is still made (locked).
                cx.add_tokens(g, h, &ambassador(s), -1);
                alignment::sway(g, cx, Side::Unity, w.int("board.factory_unity"));
            }
            for _ in 0..per {
                adversary::place_troop(g, cx, h);
            }
            cx.tally(g, "board.troops_built", per);
            cx.note(Some(h), || {
                format!(
                    "The factory at {} makes an adversary troop",
                    hex_name(&w, h)
                )
            });
        } else if let Some(s) = player_ambassador(g, h) {
            cx.add_tokens(g, h, &ambassador(s), -1);
            alignment::sway(g, cx, Side::Unity, w.int("board.factory_unity"));
        } else {
            let c = w.map.hexes[h].country;
            let bases: Vec<usize> = (0..g.hexes.len())
                .filter(|&b| is_base(&w, b) && w.map.hexes[b].country == c)
                .collect();
            for b in bases {
                for _ in 0..per {
                    add_unit(g, cx, b);
                }
            }
        }
    }
    grow_factories(g, cx)
}

/// Each city builds one factory: a corrupted city at a free zone in its
/// country next to an adversary ambassador if there is one, else the
/// lowest-numbered free zone (lowest number among equals); any other city on
/// a free zone next to it, where the players choose (an aligned city's
/// owner, else seat 0). A corrupted city's factory is the adversary's, an
/// aligned city's its owner's, and a clean city's goes to the player with
/// the most presence next to the zone (players choose on a tie), neutral if
/// none.
fn grow_factories(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for city in 0..g.hexes.len() {
        if !is_city(&w, city) {
            continue;
        }
        let zones = free_zones(&w, g);
        let at = if corrupted(g, city) {
            let c = w.map.hexes[city].country;
            zones
                .into_iter()
                .filter(|&z| w.map.hexes[z].country == c)
                .min_by_key(|&z| {
                    (
                        !next_to(&w, z, |x| g.tokens(x, ADV_AMBASSADOR) > 0),
                        w.map.hexes[z].number,
                    )
                })
        } else {
            let near: Vec<usize> = zones
                .into_iter()
                .filter(|&z| w.map.adjacent(city, z))
                .collect();
            if near.is_empty() {
                None
            } else {
                let who = cube_owner(g, city).unwrap_or(0);
                let opts = near.iter().map(|&z| hex_opt(&w, z)).collect();
                let prompt = format!("{} builds a factory on which zone?", hex_name(&w, city));
                cx.choose(who, &prompt, opts)?.map(|i| near[i])
            }
        };
        let Some(z) = at else {
            continue;
        };
        cx.add_tokens(g, z, FACTORY, 1);
        if corrupted(g, city) {
            cx.add_tokens(g, z, ADV_CUBE, 1);
            cx.tally(g, "board.factories_built.adversary", 1);
        } else if let Some(s) = match cube_owner(g, city) {
            Some(s) => Some(s),
            None => zone_owner(g, cx, z)?,
        } {
            cx.add_tokens(g, z, &cube(s), 1);
            cx.tally(g, "board.factories_built.player", 1);
        } else {
            cx.tally(g, "board.factories_built.neutral", 1);
        }
        cx.note(Some(z), || {
            format!(
                "{} builds a factory at {}",
                hex_name(&w, city),
                hex_name(&w, z)
            )
        });
    }
    Ok(())
}

/// Who takes a clean city's new factory on zone `z`: the player with the
/// most presence next to it (players choose on a tie), none if nobody has any.
fn zone_owner(g: &mut GameState, cx: &mut Cx, z: usize) -> Res<Option<usize>> {
    let w = cx.world;
    let near: Vec<usize> = (0..g.seats.len())
        .map(|s| {
            std::iter::once(z)
                .chain(w.map.hexes[z].neighbours.iter().copied())
                .filter(|&x| g.tokens(x, &presence(s)) > 0)
                .count()
        })
        .collect();
    let most = near.iter().copied().max().unwrap_or(0);
    if most == 0 {
        return Ok(None);
    }
    let tied: Vec<usize> = (0..g.seats.len()).filter(|&s| near[s] == most).collect();
    if tied.len() == 1 {
        return Ok(Some(tied[0]));
    }
    let opts = tied.iter().map(|&s| seat_opt(s)).collect();
    let prompt = format!("The factory at {} goes to which player?", hex_name(&w, z));
    Ok(cx.choose(0, &prompt, opts)?.map(|i| tied[i]))
}

/// Production step 2, on a war round: on each front the units cancel 1 for
/// 1; each surplus unit crosses and destroys the rival factory nearest its
/// base, taking any presence on its hex (027: a presence was the target with
/// no factory left). All units at the front are used up.
fn battle(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    if !war_round(&w, g) {
        return;
    }
    cx.note(None, || "War round: the fronts fight".into());
    let mut destroyed = 0;
    for f in &w.map.fronts {
        let [ba, bb] = f.bases;
        let (ua, ub) = (g.tokens(ba, UNIT), g.tokens(bb, UNIT));
        cx.add_tokens(g, ba, UNIT, -ua);
        cx.add_tokens(g, bb, UNIT, -ub);
        let (from, rival, n) = if ua > ub {
            (ba, f.countries[1], ua - ub)
        } else {
            (bb, f.countries[0], ub - ua)
        };
        for _ in 0..n {
            let hexes = &w.map.countries[rival as usize].hexes;
            let target = nearest(
                &w,
                from,
                hexes
                    .iter()
                    .copied()
                    .filter(|&h| g.tokens(h, FACTORY) > 0 && !corrupted(g, h)),
            );
            let Some(h) = target else {
                break;
            };
            if destroy_building(g, cx, h, FACTORY, Cause::War) {
                destroyed += 1;
                // War that destroys a building takes the presence on its hex
                // (027) and, by `war_reach`, around it (034).
                for x in within(&w, h, w.int("board.war_reach")) {
                    if let Some(s) = presence_owner(g, x) {
                        lose_presence(g, cx, x, s, "war");
                    }
                }
            }
        }
    }
    cx.tally(g, "board.war_rounds", 1);
    cx.tally(g, "board.war_destroyed", destroyed);
}

/// The adversary phase's march (019): every adversary troop, one at a time
/// (in hex-number order, fixed when the march starts), takes `troop_steps`
/// steps. A step destroys a target on or next to its hex (players choose)
/// or, with none there, moves one hex toward the nearest target (players
/// choose among equally good steps; with no target on the board it stays).
/// A troop that strikes is used up (`troop_spent`).
fn march(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let steps = w.int("board.troop_steps");
    let troops: Vec<usize> = (0..g.hexes.len())
        .flat_map(|h| std::iter::repeat_n(h, g.tokens(h, TROOP).max(0) as usize))
        .collect();
    // Troops that have already marched, per hex (none marches twice).
    let mut marched = vec![0i64; g.hexes.len()];
    for from in troops {
        if g.tokens(from, TROOP) - marched[from] <= 0 {
            continue;
        }
        let mut at = from;
        let mut alive = true;
        for _ in 0..steps {
            if has_troop_target(&w, g, at) {
                cx.tally(g, "board.troop_steps", 1);
                if !strike_from(g, cx, at)? {
                    break;
                }
                if w.int("board.troop_spent") > 0 {
                    alive = false;
                    break;
                }
            } else {
                let Some(to) = step_toward(g, cx, at)? else {
                    break;
                };
                cx.add_tokens(g, at, TROOP, -1);
                cx.add_tokens(g, to, TROOP, 1);
                cx.per_round(g, adversary::OPS, 1);
                cx.tally(g, "board.troop_steps", 1);
                cx.tally(g, "board.troop_moves", 1);
                at = to;
            }
            if g.is_over() {
                return Ok(());
            }
        }
        if alive {
            marched[at] += 1;
        }
    }
    Ok(())
}

/// What an adversary troop may destroy on hex `h`, as token keys: a
/// presence, and a player-owned research station or factory (019: not
/// neutral or corrupted ones, and never cities).
fn troop_targets_on(w: &World, g: &GameState, h: usize) -> Vec<String> {
    let mut out = Vec::new();
    if cube_owner(g, h).is_some() && !corrupted(g, h) {
        if g.tokens(h, FACTORY) > 0 {
            out.push(FACTORY.to_string());
        }
        if g.tokens(h, STATION) > 0 {
            out.push(STATION.to_string());
        }
    }
    if g.var(adversary::STAGE) >= w.int("board.troop_presence") {
        if let Some(s) = presence_owner(g, h) {
            out.push(presence(s).to_string());
        }
    }
    out
}

/// Whether a hardening token on hex `h` can protect something: a
/// player-owned building that is not corrupted (what troops strike; 022:
/// hardening went on any building, the adversary's too, where nothing used it).
pub fn hardenable(g: &GameState, h: usize) -> bool {
    has_building(g, h) && cube_owner(g, h).is_some() && !corrupted(g, h)
}

/// Whether hex `h` holds anything an adversary troop targets.
pub fn is_troop_target(w: &World, g: &GameState, h: usize) -> bool {
    !troop_targets_on(w, g, h).is_empty()
}

/// Whether an adversary troop at `h` has a target on or next to it.
fn has_troop_target(w: &World, g: &GameState, h: usize) -> bool {
    next_to(w, h, |x| is_troop_target(w, g, x))
}

/// Walking distance over land from the nearest of `sources`, for every hex
/// (`None`: no land path). Troops never enter ocean, and some pieces print
/// ocean inside the board, so straight-line distance would strand a troop.
fn land_distances(w: &World, sources: impl Iterator<Item = usize>) -> Vec<Option<u32>> {
    let mut dist: Vec<Option<u32>> = vec![None; w.map.hexes.len()];
    let mut queue = std::collections::VecDeque::new();
    for h in sources {
        if dist[h].is_none() {
            dist[h] = Some(0);
            queue.push_back(h);
        }
    }
    while let Some(h) = queue.pop_front() {
        let d = dist[h].unwrap_or(0);
        for &n in &w.map.hexes[h].neighbours {
            if dist[n].is_none() && !is_ocean(w, n) {
                dist[n] = Some(d + 1);
                queue.push_back(n);
            }
        }
    }
    dist
}

/// Walking distance from the nearest troop target, for every hex (`None`
/// with no target reachable over land).
fn target_gaps(w: &World, g: &GameState) -> Vec<Option<u32>> {
    land_distances(w, (0..g.hexes.len()).filter(|&h| is_troop_target(w, g, h)))
}

/// The land hexes next to `at` that bring a troop closer to the nearest
/// target, in hex order.
pub fn troop_steps_from(w: &World, g: &GameState, at: usize) -> Vec<usize> {
    let gaps = target_gaps(w, g);
    let Some(d) = gaps[at] else {
        return Vec::new();
    };
    w.map.hexes[at]
        .neighbours
        .iter()
        .copied()
        .filter(|&n| !is_ocean(w, n) && gaps[n].is_some_and(|x| x < d))
        .collect()
}

/// One move of the adversary troop at `at` toward the nearest target
/// (players choose among equally good steps). `None`: nowhere to go.
fn step_toward(g: &mut GameState, cx: &mut Cx, at: usize) -> Res<Option<usize>> {
    let w = cx.world;
    let steps = troop_steps_from(&w, g, at);
    let opts = steps.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!(
        "The adversary troop at {} moves to which hex?",
        hex_name(&w, at)
    );
    Ok(cx.choose(0, &prompt, opts)?.map(|i| steps[i]))
}

/// One adversary troop at `h` strikes a target on or next to its hex
/// (players choose): it is destroyed or the hit absorbed, and the troop is
/// used up (`troop_spent`). Returns false when it had no target.
fn strike_from(g: &mut GameState, cx: &mut Cx, h: usize) -> Res<bool> {
    let w = cx.world;
    let mut targets: Vec<(usize, String)> = Vec::new();
    let mut near = vec![h];
    near.extend(w.map.hexes[h].neighbours.iter().copied());
    for x in near {
        for k in troop_targets_on(&w, g, x) {
            targets.push((x, k));
        }
    }
    let opts = targets
        .iter()
        .map(|(x, k)| opt(format!("{} at {}", piece_name(k), hex_name(&w, *x))))
        .collect();
    let prompt = format!(
        "Adversary troop at {}: which target does it destroy?",
        hex_name(&w, h)
    );
    let Some(i) = cx.choose(0, &prompt, opts)? else {
        return Ok(false);
    };
    let (x, k) = targets[i].clone();
    cx.tally(g, "board.troop_strikes", 1);
    if destroy_building(g, cx, x, &k, Cause::Troop) {
        let kind = if k == FACTORY || k == STATION { k.as_str() } else { "presence" };
        cx.tally(g, &format!("board.troop_destroyed.{kind}"), 1);
        cx.tally(g, "board.troop_destroyed", 1);
    } else {
        cx.tally(g, "board.troop_absorbed", 1);
    }
    if w.int("board.troop_spent") > 0 {
        cx.add_tokens(g, h, TROOP, -1);
        cx.tally(g, "board.troops_spent", 1);
    }
    Ok(true)
}

/// Where the adversary troops on hex `h` are heading, for the view: the
/// nearest target over land (ties: country, then hex number; the players
/// choose in play) and the march in which they strike (1: the next one).
pub fn troop_plan(w: &World, g: &GameState, h: usize) -> Option<(usize, i64)> {
    if w.int("board.troop_steps") <= 0 {
        return None;
    }
    let from = land_distances(w, std::iter::once(h));
    let target = (0..g.hexes.len())
        .filter(|&x| is_troop_target(w, g, x) && from[x].is_some())
        .min_by_key(|&x| (from[x], w.map.hexes[x].country, w.map.hexes[x].number))?;
    let steps = w.int("board.troop_steps");
    // Moves until the target is on or next to the troop, then the strike.
    let moves = (from[target].unwrap_or(0) as i64 - 1).max(0);
    Some((target, (moves + 1 + steps - 1) / steps))
}

/// Production step 3: cities make energy, stations megaproject cubes; an
/// adversary ambassador on the piece diverts its output and is removed, and
/// diverted energy sways toward Discord. Then an adversary ambassador on a
/// lone presence drains 1 of its owner's next insight into adversary compute.
fn produce(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let divert = w.int("board.divert_discord");
    let price = w.int("board.city_energy_credits");
    for h in 0..g.hexes.len() {
        if !is_city(&w, h) {
            continue;
        }
        let c = w.map.hexes[h].country as usize;
        let energy = {
            let l = w.list("board.corp_energy");
            l[c % l.len()]
        };
        if flag_level(g, RoundFlag::Blackout, c) > 0 {
            continue;
        }
        if corrupted(g, h) {
            alignment::sway(g, cx, Side::Discord, energy * divert);
            cx.tally(g, "board.energy_diverted", energy);
        } else if g.tokens(h, ADV_AMBASSADOR) > 0 {
            adversary::remove(g, cx, h, 1);
            alignment::sway(g, cx, Side::Discord, energy * w.int("board.ambassador_discord"));
            cx.tally(g, "board.energy_diverted", energy);
        } else if let Some(s) = cube_owner(g, h) {
            // An aligned city's cube goes free to the player who aligned it.
            cx.add_resource(g, s, Resource::Energy, energy + w.int("board.aligned_bonus"));
        } else {
            // A neutral city's cube: a player with presence next to it may buy
            // it (players choose who), otherwise it is lost (021).
            let buyers: Vec<usize> = (0..g.seats.len())
                .filter(|&s| g.resource(s, Resource::Credits) >= price && in_reach(&w, g, s, h))
                .collect();
            if buyers.is_empty() {
                continue;
            }
            let mut opts: Vec<_> = buyers.iter().map(|&s| opt(format!("seat {s} buys"))).collect();
            opts.push(opt("nobody".to_string()));
            let prompt = format!(
                "The energy of {} ({energy}) can be bought for {price} credits: by whom?",
                hex_name(&w, h)
            );
            if let Some(i) = cx.choose(0, &prompt, opts)? {
                if let Some(&s) = buyers.get(i) {
                    cx.add_resource(g, s, Resource::Credits, -price);
                    cx.add_resource(g, s, Resource::Energy, energy);
                    cx.tally(g, "board.city_energy_bought", energy);
                }
            }
        }
    }
    let cubes = w.int("board.station_cubes");
    for h in 0..g.hexes.len() {
        if g.tokens(h, STATION) == 0 {
            continue;
        }
        let c = w.map.hexes[h].country as usize;
        let owner = cube_owner(g, h);
        let to = if g.tokens(h, ADV_AMBASSADOR) > 0 && (corrupted(g, h) || owner.is_some()) {
            adversary::remove(g, cx, h, 1);
            Some(None)
        } else if corrupted(g, h) {
            Some(None)
        } else {
            owner.map(Some)
        };
        let Some(to) = to else {
            continue;
        };
        if g.var(&project_done(c)) > 0 {
            if let Some(s) = to {
                for (r, n) in &w.content.map.countries[c].project.benefit {
                    cx.add_resource(g, s, *r, *n);
                }
            }
            continue;
        }
        cx.add_key(g, project_key(c, to).into(), cubes);
        complete_project(g, cx, c);
    }
    // An adversary ambassador on a presence (with no other piece to divert)
    // costs its owner 1 insight next round and adds 1 compute to the cheapest
    // adversary project; it is used up (021: it leaked the owner's compute).
    for h in 0..g.hexes.len() {
        let Some(s) = presence_owner(g, h) else {
            continue;
        };
        // (020: a Sabotage flag used to skip the leak here, a data-centre-era
        // leftover the rules don't have; once F1 let the flag reach production
        // it would have helped the players.)
        if g.tokens(h, ADV_AMBASSADOR) == 0 {
            continue;
        }
        // Only a lone presence: on a city, base, station or factory the
        // ambassador belongs to that piece (020: the leak drained a corrupted
        // city's ambassadors so it never spread, and ate those a clean city or
        // a base was gathering).
        if is_city(&w, h) || is_base(&w, h) || has_building(g, h) {
            continue;
        }
        adversary::remove(g, cx, h, 1);
        cx.add_key(g, economy::insight_debt(s), 1);
        cx.tally(g, "board.presence_drained", 1);
        adversary::gain_compute(g, cx, 1)?;
    }
    Ok(())
}

/// Completes country `c`'s megaproject if it is full: each player cube pays
/// its owner, adversary cubes become Discord.
fn complete_project(g: &mut GameState, cx: &mut Cx, c: usize) {
    let w = cx.world;
    let p = &w.content.map.countries[c].project;
    let adv = g.var(&project_key(c, None));
    let mine: Vec<i64> = (0..g.seats.len())
        .map(|s| g.var(&project_key(c, Some(s))))
        .collect();
    if adv + mine.iter().sum::<i64>() < p.cubes {
        return;
    }
    for (s, &n) in mine.iter().enumerate() {
        cx.add_resource(g, s, Resource::Credits, n * w.int("board.project_credits"));
        cx.set_key(g, project_key(c, Some(s)).into(), 0);
    }
    let per = w.int("board.project_discord_per");
    alignment::sway(g, cx, Side::Discord, adv / per);
    cx.set_key(g, project_key(c, None).into(), 0);
    cx.set_key(g, project_done(c).into(), 1);
    cx.tally(g, "board.projects_completed", 1);
    cx.note(None, || format!("{} completes", p.name));
}

impl Module for Board {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        let mut corrupt = w.int("board.start_corrupted");
        if adversary::level(&w, 4) {
            corrupt += w.int("adversary.extra_corrupted");
        }
        for c in 0..w.map.countries.len() {
            let piece = &w.content.map.countries[c];
            let at = |n: u8| w.map.numbered(c, n);
            for &n in piece.corrupted.iter().take(corrupt as usize) {
                cx.add_tokens(g, at(n), ADV_CUBE, 1);
            }
            for &n in &piece.stations {
                cx.add_tokens(g, at(n), STATION, 1);
                cx.add_tokens(g, at(n), ADV_CUBE, 1);
            }
            for &n in &piece.factories {
                cx.add_tokens(g, at(n), FACTORY, 1);
            }
        }
        for s in 0..g.seats.len() {
            let c = home_country(&w, s);
            let n = w.content.map.countries[c].presence;
            cx.add_tokens(g, w.map.numbered(c, n), &presence(s), 1);
            cx.set_key(g, track_key(s).into(), 1);
            cx.add_resource(g, s, Resource::Ambassadors, w.int("board.start_ambassadors"));
        }
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        for act in STANDARD {
            if !can_afford(g, s, seat_cost(w, g, s, act)) {
                continue;
            }
            if !std_targets(w, g, s, act).is_empty() {
                out.push(Action::Standard {
                    seat,
                    act: act.name().to_string(),
                });
            }
        }
        for id in g.pile(&cards::hand(s)) {
            out.push(Action::Sell {
                seat,
                card: id.clone(),
            });
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Standard { .. } | Action::Sell { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        match a {
            Action::Sell { seat, card } => {
                let s = *seat as usize;
                let hand = cards::hand(s);
                if !g.pile(&hand).contains(card) {
                    return Err(Halt::Illegal("not in your hand".into()));
                }
                cards::discard(g, cx, card, &hand);
                cx.add_resource(g, s, Resource::Credits, w.int("board.sell_credits"));
                cx.tally(g, "board.std.sell", 1);
                Ok(())
            }
            Action::Standard { seat, act } => {
                let s = *seat as usize;
                let Some(act) = StandardAction::parse(act) else {
                    return Err(Halt::Illegal("no such standard action".into()));
                };
                let targets = std_targets(&w, g, s, act);
                if !can_afford(g, s, seat_cost(&w, g, s, act)) || targets.is_empty() {
                    return Err("can't take that standard action now".into());
                }
                // The printed price; a presence's rise and extra follow in
                // `std_resolve`, once the track is chosen.
                let (e, cr, chips) = std_cost(&w, act);
                cx.add_resource(g, s, Resource::Energy, -e);
                cx.add_resource(g, s, Resource::Credits, -cr);
                cx.add_resource(g, s, Resource::Chips, -chips);
                cx.tally(g, "board.chips_spent", chips);
                cx.tally(g, &format!("board.std.{}", act.name()), 1);
                cx.tally(g, "board.std_actions", 1);
                std_resolve(g, cx, s, act, &targets)
            }
            _ => Err(Halt::Illegal("not a standard action".into())),
        }
    }

    fn after(&self, g: &mut GameState, cx: &mut Cx) -> Res {
        free_cities(g, cx);
        Ok(())
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            cx.set_key(g, rich_key(s).into(), 0);
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match phase {
            1 => produce_units(g, cx)?,
            2 => battle(g, cx),
            3 => march(g, cx)?,
            4 => produce(g, cx)?,
            _ => {
                // A flag counts down: 1 (set this round) clears, 2 (a threat
                // executed after this round's production) lasts the next round.
                for c in 0..w.map.countries.len() {
                    for f in FLAGS {
                        let k = flag(f, c);
                        let v = g.var(&k);
                        if v > 0 {
                            cx.set_key(g, k, v - 1);
                        }
                    }
                }
                // Rounds that end with only nodes between the players and the
                // purge (019: nodes are hard to remove).
                let level = alignment::bar_level(&w, g);
                let nodes = adversary::node_count(g, None);
                cx.peak(g, "adversary.nodes_peak", nodes);
                if level < 3 && nodes > 0 && adversary::purge_count(&w, g, level) == nodes {
                    cx.tally(g, "alignment.purge_blocked_by_nodes", 1);
                }
                // Factories per side, as the game ends (the last round's level).
                let mut sides = [0i64; 3];
                for h in 0..g.hexes.len() {
                    if g.tokens(h, FACTORY) > 0 {
                        let i = if corrupted(g, h) {
                            1
                        } else if cube_owner(g, h).is_some() {
                            0
                        } else {
                            2
                        };
                        sides[i] += 1;
                    }
                }
                for (i, side) in ["player", "adversary", "neutral"].iter().enumerate() {
                    cx.level(g, &format!("board.factories_end.{side}"), sides[i]);
                }
            }
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let flags: Vec<String> = FLAGS
            .iter()
            .map(|f| format!("board.{}.", f.name()))
            .collect();
        let mut keys: Vec<&str> = flags.iter().map(String::as_str).collect();
        keys.extend(["board.project.", "board.project_done.", "board.track.", "board.track2.", "board.rich."]);
        out.bookkeeping(&keys);
        for c in 0..w.map.countries.len() {
            for f in FLAGS {
                if flag_level(g, f, c) > 0 {
                    out.country(c, f.name(), f.name(), 1, None, flag_doc(f));
                }
            }
            let p = &w.content.map.countries[c].project;
            let done = g.var(&project_done(c)) > 0;
            let total: i64 = g.var(&project_key(c, None))
                + (0..g.seats.len())
                    .map(|s| g.var(&project_key(c, Some(s))))
                    .sum::<i64>();
            let doc = format!(
                "{}: {} ({} cubes; each player cube pays {} credits, every {} adversary cubes 1 Discord)",
                p.name,
                p.text,
                p.cubes,
                w.int("board.project_credits"),
                w.int("board.project_discord_per")
            );
            out.country(
                c,
                "project",
                &format!("megaproject: {}", p.name),
                if done { p.cubes } else { total },
                Some(p.cubes),
                &doc,
            );
            if !done {
                out.country(
                    c,
                    "project.adversary",
                    "adversary cubes",
                    g.var(&project_key(c, None)),
                    None,
                    "adversary cubes on the megaproject",
                );
                for s in 0..g.seats.len() {
                    out.country(
                        c,
                        &format!("project.{s}"),
                        &format!("seat {s} cubes"),
                        g.var(&project_key(c, Some(s))),
                        None,
                        "this player's cubes on the megaproject",
                    );
                }
            } else {
                out.country(
                    c,
                    "project.done",
                    "completed",
                    1,
                    None,
                    "owned stations here produce the project's benefit",
                );
            }
        }
        let step = clock_step(w, g.round);
        let name = |k: i64| match k & 3 {
            1 => "production",
            2 => "war",
            3 => "production + war",
            _ => "—",
        };
        let track: Vec<String> = (1..=8u32)
            .map(|r| format!("{r}: {}", name(clock_step(w, r))))
            .collect();
        out.global(
            "clock",
            name(step),
            step,
            None,
            &format!("the production/war clock (1 production, 2 war, 3 both); this round's step; track {}…", track.join(", ")),
        );
        let war = war_round(w, g) as i64;
        out.global(
            "war",
            "war step",
            war,
            None,
            "1 on a war step (the clock or a War card): in production the fronts fight",
        );
        out.global(
            "fronts",
            "fronts",
            w.map.fronts.len() as i64,
            None,
            "printed borders between countries, each with a base on either side",
        );
        // Each adversary troop's next target (019): players see each hit coming.
        for h in 0..g.hexes.len() {
            let n = g.tokens(h, TROOP);
            if n <= 0 {
                continue;
            }
            if let Some((t, rounds)) = troop_plan(w, g, h) {
                let what = troop_targets_on(w, g, t)
                    .iter()
                    .map(|k| piece_name(k))
                    .collect::<Vec<_>>()
                    .join(", ");
                let when = if rounds <= 1 {
                    "strikes it in the next adversary phase".to_string()
                } else {
                    format!("strikes it in {rounds} adversary phases unless stopped")
                };
                let doc = format!(
                    "adversary troop{} at {} heading for {what} at {}: {when} (players choose among equally near targets)",
                    if n > 1 { "s" } else { "" },
                    hex_name(w, h),
                    hex_name(w, t)
                );
                out.arrow("troop", h, t, n, rounds, &doc);
            }
        }
        for act in STANDARD {
            let (e, cr, chips) = std_cost(w, act);
            let cost: Vec<String> = [(e, Resource::Energy), (cr, Resource::Credits), (chips, Resource::Chips)]
                .iter()
                .filter(|(n, _)| *n > 0)
                .map(|(n, r)| r.amount(*n))
                .collect();
            let doc = format!(
                "{}; costs {}",
                std_text(act),
                if cost.is_empty() {
                    "nothing".into()
                } else {
                    cost.join(" + ")
                }
            );
            out.standard(act.name(), e, cr, chips, &doc);
        }
        for s in 0..g.seats.len() {
            // 034: what a presence costs this seat now, rise and step extra in.
            out.seat(
                s,
                "presence_energy_now",
                "next presence (energy)",
                seat_cost(w, g, s, StandardAction::Presence).0,
                None,
                "the standard action at your cheapest track: printed price plus the step's extra",
            );
            out.seat(
                s,
                "presence_credits_now",
                "next rich action (credits)",
                seat_cost(w, g, s, StandardAction::PresenceBuy).1,
                None,
                "printed price, plus its rise this round and the step's extra",
            );
            out.seat(
                s,
                "ambassadors_board",
                "ambassadors on the board",
                ambassadors_on_board(g, s),
                None,
                "your player ambassadors on pieces",
            );
            out.seat(
                s,
                "presence",
                "presence on the board",
                presence_count(g, s),
                None,
                "your presence: you act within 1 of it, and claim sites on or next to it",
            );
            let b = ai::board(w, s);
            let left = b.presence.len().saturating_sub(track_taken(g, s));
            out.seat(
                s,
                "presence_track",
                "presence left on track 1",
                left as i64,
                None,
                "presence tokens still on your AI board's first track (the second shows on the board)",
            );
        }
    }
}

fn flag_doc(f: RoundFlag) -> &'static str {
    match f {
        RoundFlag::Blackout => "Blackout: while this is up, cities here make no energy in production (an uncountered threat's lasts to the end of the next round)",
        RoundFlag::Sabotage => "Sabotage: while this is up, players with presence here make no energy in upkeep (an uncountered threat's lasts to the end of the next round)",
        RoundFlag::Shelter => "Shelter Order: buildings here can't be destroyed this round",
        RoundFlag::Embargo => "Embargo: while this is up, no site here can be claimed (an uncountered threat's lasts to the end of the next round)",
    }
}
