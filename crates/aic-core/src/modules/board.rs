//! The board (014, 016): the printed country pieces (cities, open land,
//! ocean, industrial zones; military bases on the fronts), player presence
//! (owned here; AI boards read the track through `track_taken`), the setup,
//! the production/war clock, production (factories grow from cities and
//! make troops and hardware, front battles on war steps, adversary troops,
//! cities, stations, megaprojects), player ambassadors and site claims, the
//! standard actions and the round-end cleanup.

use crate::action::Action;
use crate::content::{Passive, RoundFlag, Side};
use crate::module::{
    scheduled, Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::*;
use super::{adversary, ai, alignment, cards};

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
            ParamSpec::int("troop_health", 0, 10, "removal a card must spend on an adversary troop (an ambassador takes 1)"),
            ParamSpec::int("unit_cap", 0, 20, "corp units a base holds at most"),
            ParamSpec::int("factory_units", 0, 5, "troops each factory makes on a production step at each of its corp's bases"),
            ParamSpec::list("clock", 0, 3, "the printed round track, per round: 1 production, 2 war, 3 both (the last entry repeats)"),
            ParamSpec::int("city_energy", 0, 10, "energy a city makes a round"),
            ParamSpec::int("aligned_bonus", 0, 10, "extra energy an aligned city gives its owner"),
            ParamSpec::int("station_cubes", 0, 5, "megaproject cubes a research station makes a round"),
            ParamSpec::int("project_credits", 0, 50, "credits each player cube pays when a megaproject completes"),
            ParamSpec::int("project_discord_per", 0, 10, "adversary cubes per Discord when a megaproject completes"),
            ParamSpec::int("divert_discord", 0, 5, "Discord per unit of energy or compute diverted to the adversary"),
            ParamSpec::int("base_discord", 0, 10, "Discord an adversary ambassador on a producing base sways"),
            ParamSpec::int("base_unity", 0, 10, "Unity a player ambassador on a producing base sways"),
            ParamSpec::int("factory_unity", 0, 10, "Unity a player ambassador on a producing factory sways"),
            ParamSpec::int("free_city_at", 0, 10, "player ambassadors on a corrupted city that free it"),
            ParamSpec::int("destruction_discord", 0, 5, "Discord each building destroyed in war sways"),
            ParamSpec::int("troop_spent", 0, 1, "1: an adversary troop that strikes a target is used up"),
            ParamSpec::int("presence_returns", 0, 1, "1: a destroyed presence goes to its owner's reserve, placed before the track"),
            ParamSpec::int("presence_energy", 0, 20, "standard action: energy to place a presence"),
            ParamSpec::int("build_energy", 0, 20, "standard action: energy to build a research station"),
            ParamSpec::int("station_hardware", 0, 20, "standard action: hardware to build a research station"),
            ParamSpec::int("factory_hardware", 0, 5, "hardware a player-owned factory makes its owner on a production step"),
            ParamSpec::int("hardware_credits", 0, 50, "standard action: credits for 1 hardware"),
            ParamSpec::int("node_energy", 0, 20, "standard action: energy to remove a node within reach"),
            ParamSpec::int("remove_energy", 0, 20, "standard action: energy to remove an adversary ambassador within reach"),
            ParamSpec::int("move_energy", 0, 20, "standard action: energy to move a player ambassador"),
            ParamSpec::int("ambassador_credits", 0, 50, "standard action: credits to gain a player ambassador"),
            ParamSpec::int("harden_hardware", 0, 20, "standard action (Harden passive): hardware a hardening token"),
            ParamSpec::int("sell_credits", 0, 20, "standard action: credits for selling a card from hand"),
            ParamSpec::int("align_energy", 0, 20, "standard action: energy (and 1 ambassador) to align a clean city"),
        ],
        phases: &[
            PhaseSpec::decision("act", "standard actions: presence, station, remove, node, move, align, harden, hardware, sell"),
            PhaseSpec::auto("bases", "on a production step: bases and factories make troops and hardware, cities grow factories"),
            PhaseSpec::auto("war", "on a war step, the troops on either side of each front fight"),
            PhaseSpec::auto("strike", "each adversary troop destroys an adjacent building"),
            PhaseSpec::auto("produce", "cities and research stations produce; ambassadors divert them and leak presence compute"),
            PhaseSpec::auto("cleanup", "remove the round's country flags"),
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// Country flags that last until the end of the round.
pub const FLAGS: [RoundFlag; 3] = [RoundFlag::Blackout, RoundFlag::Sabotage, RoundFlag::Shelter];

/// The var key of a country flag.
pub fn flag(f: RoundFlag, country: usize) -> String {
    format!("board.{}.{country}", f.name())
}

/// Whether a country flag is up, by its name (`"blackout"`, ...).
pub fn flagged(g: &GameState, name: &str, country: usize) -> bool {
    g.var(&format!("board.{name}.{country}")) > 0
}

/// Shelters country `c`'s buildings for the rest of the round.
pub fn shelter(g: &mut GameState, cx: &mut Cx, country: usize) {
    cx.set_key(g, flag(RoundFlag::Shelter, country), 1);
}

/// Cubes on country `c`'s megaproject: `None` for the adversary's.
pub fn project_key(c: usize, owner: Option<usize>) -> String {
    match owner {
        Some(s) => format!("board.project.{c}.{s}"),
        None => format!("board.project.{c}.adversary"),
    }
}

/// Set once country `c`'s megaproject has completed.
pub fn project_done(c: usize) -> String {
    format!("board.project_done.{c}")
}

/// The standard actions, by name.
pub const STANDARD: [&str; 9] = [
    "presence",
    "station",
    "remove",
    "node",
    "move",
    "ambassador",
    "align",
    "harden",
    "hardware",
];

/// The clock's step for round `round`: 1 production, 2 war, 3 both.
pub fn clock_step(w: &World, round: u32) -> i64 {
    scheduled(w.list("board.clock"), round)
}

/// Whether this round has a production step (factories, troops, hardware).
pub fn production_round(w: &World, g: &GameState) -> bool {
    clock_step(w, g.round) & 1 != 0
}

/// Whether this round is a war step (the clock, or this round's adversary card).
pub fn war_round(w: &World, g: &GameState) -> bool {
    clock_step(w, g.round) & 2 != 0 || adversary::war_twist(w, g)
}

/// Presence tokens seat `s` has taken off its AI board's track (the
/// starting one included); AI boards read their uncovered spaces from it.
pub fn track_key(seat: usize) -> String {
    format!("board.track.{seat}")
}

pub fn track_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track_key(seat)).max(0) as usize
}

/// Where seat `s` may place presence: a land hex holding no presence,
/// adjacent to one of its own (with none left, any such hex in its home
/// country).
pub fn presence_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    let mine = presence(seat);
    let free = |h: usize| !is_ocean(w, h) && presence_owner(g, h).is_none();
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

/// Places one presence for `seat`: the next token off its track (uncovering
/// the space under it) onto a hex next to its presence. Returns whether placed.
pub fn place_presence(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res<bool> {
    let w = cx.world;
    if !ai::can_place(&w, g, seat) {
        return Ok(false);
    }
    let hexes = presence_hexes(&w, g, seat);
    let reserve = g.resource(seat, RESERVE) > 0;
    let what = if reserve {
        "from your reserve".to_string()
    } else {
        let next = &ai::board(&w, seat).presence[track_taken(g, seat)];
        format!("uncovers {}", ai::uncover_label(next))
    };
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!("Seat {seat}: place presence ({what}) on which hex?");
    let Some(j) = cx.choose(seat, &prompt, opts)? else {
        return Ok(false);
    };
    if reserve {
        cx.add_resource(g, seat, RESERVE, -1);
    } else {
        cx.add_key(g, track_key(seat), 1);
    }
    cx.add_tokens(g, hexes[j], &presence(seat), 1);
    cx.tally(g, "board.presence_placed", 1);
    Ok(true)
}

/// Whether seat `s` can claim the site on hex `h` (align a city, take a
/// station or factory): its presence is on or next to `h`, and no node is.
pub fn can_claim(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    let k = presence(seat);
    next_to(w, h, |x| g.tokens(x, &k) > 0) && !next_to(w, h, |x| g.tokens(x, NODE) > 0)
}

#[derive(Debug)]
struct Board;

fn build(_b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Board))
}

/// A standard action's cost: (energy, credits, hardware).
pub fn std_cost(w: &World, act: &str) -> (i64, i64, i64) {
    let i = |k: &str| w.int(&format!("board.{k}"));
    match act {
        "presence" => (i("presence_energy"), 0, 0),
        "station" => (i("build_energy"), 0, i("station_hardware")),
        "remove" => (i("remove_energy"), 0, 0),
        "node" => (i("node_energy"), 0, 0),
        "move" => (i("move_energy"), 0, 0),
        "ambassador" => (0, i("ambassador_credits"), 0),
        "align" => (i("align_energy"), 0, 0),
        "hardware" => (0, i("hardware_credits"), 0),
        _ => (0, 0, i("harden_hardware")),
    }
}

/// What a standard action does, for the view.
pub fn std_text(act: &str) -> &'static str {
    match act {
        "presence" => "place a presence next to yours (uncovers your track's next space)",
        "station" => "build a research station in reach (yours)",
        "remove" => "remove an adversary ambassador in reach",
        "node" => "remove a node in reach",
        "move" => "move a player ambassador onto a piece in reach",
        "ambassador" => "gain a player ambassador",
        "align" => "align a clean city you can claim (1 ambassador)",
        "hardware" => "buy 1 hardware",
        _ => "harden a building in reach (Harden passive)",
    }
}

fn can_afford(g: &GameState, seat: usize, (e, cr, hw): (i64, i64, i64)) -> bool {
    g.resource(seat, "energy") >= e
        && g.resource(seat, "credits") >= cr
        && g.resource(seat, HARDWARE) >= hw
}

/// The hexes a standard action of `seat` may target (empty: not offered).
fn std_targets(w: &World, g: &GameState, seat: usize, act: &str) -> Vec<usize> {
    let reach = |h: usize| in_reach(w, g, seat, h);
    let all = 0..g.hexes.len();
    match act {
        "presence" => {
            if ai::can_place(w, g, seat) {
                presence_hexes(w, g, seat)
            } else {
                Vec::new()
            }
        }
        "station" => all
            .filter(|&h| buildable(w, g, h) && !w.map.hexes[h].industrial && reach(h))
            .collect(),
        "remove" => all
            .filter(|&h| reach(h) && g.tokens(h, ADV_AMBASSADOR) > 0)
            .collect(),
        "node" => all.filter(|&h| reach(h) && g.tokens(h, NODE) > 0).collect(),
        "move" => {
            if g.resource(seat, AMBASSADORS) > 0 || ambassadors_on_board(g, seat) > 0 {
                all.filter(|&h| reach(h) && has_piece(w, g, h)).collect()
            } else {
                Vec::new()
            }
        }
        "align" => {
            if g.resource(seat, AMBASSADORS) > 0 {
                all.filter(|&h| reach(h) && clean_city(w, g, h) && can_claim(w, g, seat, h))
                    .collect()
            } else {
                Vec::new()
            }
        }
        "harden" => {
            if cards::has_passive(w, g, seat, |p| matches!(p, Passive::HardenAction)) {
                all.filter(|&h| reach(h) && has_building(g, h)).collect()
            } else {
                Vec::new()
            }
        }
        // Gaining an ambassador or hardware needs no target.
        _ => vec![usize::MAX],
    }
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
    let site = g.tokens(h, STATION) > 0 || g.tokens(h, FACTORY) > 0;
    if site && !corrupted(g, h) && cube_owner(g, h).is_none() && can_claim(&w, g, seat, h) {
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
    if is_city(&w, h) && corrupted(g, h) {
        let here: i64 = (0..g.seats.len())
            .map(|s| g.tokens(h, &ambassador(s)))
            .sum();
        if here >= w.int("board.free_city_at") {
            for s in 0..g.seats.len() {
                let n = g.tokens(h, &ambassador(s));
                cx.add_tokens(g, h, &ambassador(s), -n);
            }
            cx.add_tokens(g, h, ADV_CUBE, -1);
            cx.tally(g, "board.cities_freed", 1);
            alignment::for_city_freed(g, cx);
            cx.note(Some(h), || format!("{} is freed", hex_name(&w, h)));
        }
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
    if g.resource(seat, AMBASSADORS) > 0 {
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
        cx.add_resource(g, seat, AMBASSADORS, -1);
    } else {
        cx.add_tokens(g, from[i - off], &k, -1);
    }
    place_ambassador(g, cx, seat, to);
    Ok(())
}

/// The building kinds on hex `h` a destruction can take, as token keys.
fn buildings_on(g: &GameState, h: usize) -> Vec<String> {
    let mut out = Vec::new();
    if g.tokens(h, FACTORY) > 0 && !corrupted(g, h) {
        out.push(FACTORY.to_string());
    }
    if g.tokens(h, STATION) > 0 && !corrupted(g, h) {
        out.push(STATION.to_string());
    }
    if let Some(s) = presence_owner(g, h) {
        out.push(presence(s));
    }
    out
}

/// A building of `kind` on hex `h` would be destroyed. A Shelter Order in
/// its country, a corp troop at a base next to it (adversary strikes only,
/// `by_troop`), a hardening token, a player ambassador on the hex, or a
/// Civic Shelter absorbs it (in that order); otherwise it goes, and a war
/// destruction (not `by_troop`) sways toward Discord. Returns whether it was
/// destroyed.
pub fn destroy_building(
    g: &mut GameState,
    cx: &mut Cx,
    h: usize,
    kind: &str,
    by_troop: bool,
) -> bool {
    let w = cx.world;
    let c = w.map.hexes[h].country as usize;
    if flagged(g, RoundFlag::Shelter.name(), c) {
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
    cx.add_tokens(g, h, kind, -1);
    if w.int("board.presence_returns") > 0 {
        if let Some(s) = (0..g.seats.len()).find(|&s| kind == presence(s)) {
            cx.add_resource(g, s, RESERVE, 1);
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
        format!("{kind} at {} destroyed", hex_name(&w, h))
    });
    if !by_troop {
        alignment::sway(g, cx, Side::Discord, w.int("board.destruction_discord"));
    }
    true
}

/// Uses a Civic Shelter for a building destruction in `country`, if one applies.
fn civic_shelter(g: &mut GameState, cx: &mut Cx, country: usize) -> bool {
    let w = cx.world;
    for s in 0..g.seats.len() {
        if g.var(&cards::civic_used(s)) == 0
            && has_presence_in(&w, g, s, country)
            && cards::has_passive(&w, g, s, |p| matches!(p, Passive::CivicShelter))
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
/// factories, make their corp's troops (a factory 1 at each of its corp's
/// bases); an ambassador on the piece diverts it and is removed; an
/// adversary-owned factory makes an adversary troop. Player-owned factories
/// make hardware. Then the cities grow factories.
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
    let hw = w.int("board.factory_hardware");
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 {
            continue;
        }
        if let Some(s) = cube_owner(g, h) {
            cx.add_resource(g, s, HARDWARE, hw);
            cx.tally(g, "board.hardware_made", hw);
        }
        if per == 0 {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 || corrupted(g, h) {
            if !corrupted(g, h) {
                adversary::remove(g, cx, h, 1);
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

/// Free industrial zones: printed zones holding no building.
pub fn free_zones(w: &World, g: &GameState) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| w.map.hexes[h].industrial && !has_building(g, h))
        .collect()
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
/// base, or with none a presence. All units at the front are used up.
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
            )
            .map(|h| (h, FACTORY.to_string()))
            .or_else(|| {
                nearest(
                    &w,
                    from,
                    hexes
                        .iter()
                        .copied()
                        .filter(|&h| presence_owner(g, h).is_some()),
                )
                .map(|h| (h, presence(presence_owner(g, h).unwrap())))
            });
            let Some((h, kind)) = target else {
                break;
            };
            if destroy_building(g, cx, h, &kind, false) {
                destroyed += 1;
            }
        }
    }
    cx.tally(g, "board.war_rounds", 1);
    cx.tally(g, "board.war_destroyed", destroyed);
}

/// Production step 3: each adversary troop destroys one building on or
/// next to its hex (players choose).
fn strike(g: &mut GameState, cx: &mut Cx) -> Res {
    let troops: Vec<(usize, i64)> = (0..g.hexes.len())
        .map(|h| (h, g.tokens(h, TROOP)))
        .filter(|&(_, n)| n > 0)
        .collect();
    for (h, n) in troops {
        for _ in 0..n {
            if !strike_from(g, cx, h, "board.troop_destroyed")? {
                break;
            }
        }
    }
    Ok(())
}

/// One adversary troop at `h` strikes a building or presence on or next to
/// its hex (players choose); a destruction is tallied under `key`.
/// Returns false when it had no target.
fn strike_from(g: &mut GameState, cx: &mut Cx, h: usize, key: &str) -> Res<bool> {
    let w = cx.world;
    let mut targets: Vec<(usize, String)> = Vec::new();
    let mut near = vec![h];
    near.extend(w.map.hexes[h].neighbours.iter().copied());
    for x in near {
        for k in buildings_on(g, x) {
            targets.push((x, k));
        }
    }
    let opts = targets
        .iter()
        .map(|(x, k)| opt(format!("{k} at {}", hex_name(&w, *x))))
        .collect();
    let prompt = format!(
        "Adversary troop at {}: which building does it destroy?",
        hex_name(&w, h)
    );
    let Some(i) = cx.choose(0, &prompt, opts)? else {
        return Ok(false);
    };
    let (x, k) = targets[i].clone();
    if destroy_building(g, cx, x, &k, true) {
        cx.tally(g, key, 1);
    }
    if w.int("board.troop_spent") > 0 {
        cx.add_tokens(g, h, TROOP, -1);
        cx.tally(g, "board.troops_spent", 1);
    }
    Ok(true)
}

/// A Discord tip (017): every adversary troop, one at a time (by hex
/// number order, fixed when the advance starts), moves one hex toward the
/// nearest player presence or player-owned station or factory, then
/// strikes. Among equally good steps the players choose; a troop already on
/// such a hex stays, and with no such hex on the board troops stay. This is
/// the one movement rule.
pub fn advance_troops(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let troops: Vec<usize> = (0..g.hexes.len())
        .flat_map(|h| std::iter::repeat_n(h, g.tokens(h, TROOP).max(0) as usize))
        .collect();
    // Troops that already advanced this tip, per hex (they don't move twice).
    let mut moved = vec![0i64; g.hexes.len()];
    for from in troops {
        if g.tokens(from, TROOP) - moved[from] <= 0 {
            continue;
        }
        let targets: Vec<usize> = (0..g.hexes.len())
            .filter(|&h| {
                presence_owner(g, h).is_some()
                    || ((g.tokens(h, STATION) > 0 || g.tokens(h, FACTORY) > 0)
                        && cube_owner(g, h).is_some())
            })
            .collect();
        let gap = |h: usize| targets.iter().map(|&t| w.map.distance(h, t)).min();
        let mut at = from;
        if let Some(d) = gap(from) {
            if d > 0 {
                let steps: Vec<usize> = w.map.hexes[from]
                    .neighbours
                    .iter()
                    .copied()
                    .filter(|&n| !is_ocean(&w, n) && gap(n).is_some_and(|x| x < d))
                    .collect();
                let opts = steps.iter().map(|&h| hex_opt(&w, h)).collect();
                let prompt = format!(
                    "Discord: the adversary troop at {} advances to which hex?",
                    hex_name(&w, from)
                );
                if let Some(i) = cx.choose(0, &prompt, opts)? {
                    at = steps[i];
                    cx.add_tokens(g, from, TROOP, -1);
                    cx.add_tokens(g, at, TROOP, 1);
                    moved[at] += 1;
                    cx.per_round(g, adversary::OPS, 1);
                    cx.tally(g, "board.troop_advances", 1);
                }
            }
        }
        cx.tally(g, "board.advance_strikes", 1);
        if strike_from(g, cx, at, "board.advance_destroyed")?
            && at != from
            && w.int("board.troop_spent") > 0
        {
            // The troop that moved here was used up.
            moved[at] -= 1;
        }
        if g.is_over() {
            return Ok(());
        }
    }
    Ok(())
}

/// Production step 4: cities make energy, stations megaproject cubes, data
/// centres compute; an adversary ambassador on the piece diverts its output
/// and is removed. Diverted energy and compute sway toward Discord.
fn produce(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let energy = w.int("board.city_energy");
    let divert = w.int("board.divert_discord");
    for h in 0..g.hexes.len() {
        if !is_city(&w, h) {
            continue;
        }
        let c = w.map.hexes[h].country as usize;
        if flagged(g, RoundFlag::Blackout.name(), c) {
            continue;
        }
        if corrupted(g, h) {
            alignment::sway(g, cx, Side::Discord, energy * divert);
            cx.tally(g, "board.energy_diverted", energy);
        } else if g.tokens(h, ADV_AMBASSADOR) > 0 {
            adversary::remove(g, cx, h, 1);
            alignment::sway(g, cx, Side::Discord, energy * divert);
            cx.tally(g, "board.energy_diverted", energy);
        } else if let Some(s) = cube_owner(g, h) {
            cx.add_resource(g, s, "energy", energy + w.int("board.aligned_bonus"));
        } else {
            // The player with the most presence next to it; players choose on a tie.
            let near: Vec<i64> = (0..g.seats.len())
                .map(|s| {
                    std::iter::once(h)
                        .chain(w.map.hexes[h].neighbours.iter().copied())
                        .filter(|&x| g.tokens(x, &presence(s)) > 0)
                        .count() as i64
                })
                .collect();
            let most = near.iter().copied().max().unwrap_or(0);
            if most == 0 {
                continue;
            }
            let tied: Vec<usize> = (0..g.seats.len()).filter(|&s| near[s] == most).collect();
            let opts = tied.iter().map(|&s| seat_opt(s)).collect();
            let prompt = format!("The energy of {} goes to which player?", hex_name(&w, h));
            if let Some(i) = cx.choose(0, &prompt, opts)? {
                cx.add_resource(g, tied[i], "energy", energy);
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
                    cx.add_resource(g, s, r, *n);
                }
            }
            continue;
        }
        cx.add_key(g, project_key(c, to), cubes);
        complete_project(g, cx, c);
    }
    // An adversary ambassador on a presence (with no other piece to divert)
    // leaks 1 of its owner's compute.
    for h in 0..g.hexes.len() {
        let Some(s) = presence_owner(g, h) else {
            continue;
        };
        if g.tokens(h, ADV_AMBASSADOR) == 0 {
            continue;
        }
        let c = w.map.hexes[h].country as usize;
        if flagged(g, RoundFlag::Sabotage.name(), c) {
            continue;
        }
        adversary::remove(g, cx, h, 1);
        if g.resource(s, "compute") > 0 {
            cx.add_resource(g, s, "compute", -1);
        }
        alignment::sway(g, cx, Side::Discord, divert);
        cx.tally(g, "board.compute_diverted", 1);
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
        cx.add_resource(g, s, "credits", n * w.int("board.project_credits"));
        cx.set_key(g, project_key(c, Some(s)), 0);
    }
    let per = w.int("board.project_discord_per").max(1);
    alignment::sway(g, cx, Side::Discord, adv / per);
    cx.set_key(g, project_key(c, None), 0);
    cx.set_key(g, project_done(c), 1);
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
            cx.set_key(g, track_key(s), 1);
            cx.add_resource(g, s, AMBASSADORS, w.int("board.start_ambassadors"));
        }
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        for act in STANDARD {
            if !can_afford(g, s, std_cost(w, act)) {
                continue;
            }
            if act == "align" && g.resource(s, AMBASSADORS) < 1 {
                continue;
            }
            if !std_targets(w, g, s, act).is_empty() {
                out.push(Action::Standard {
                    seat,
                    act: act.to_string(),
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
                cx.add_resource(g, s, "credits", w.int("board.sell_credits"));
                cx.tally(g, "board.std.sell", 1);
                Ok(())
            }
            Action::Standard { seat, act } => {
                let s = *seat as usize;
                let act = act.as_str();
                if !STANDARD.contains(&act) {
                    return Err(Halt::Illegal("no such standard action".into()));
                }
                let cost = std_cost(&w, act);
                let targets = std_targets(&w, g, s, act);
                if !can_afford(g, s, cost) || targets.is_empty() {
                    return Err("can't take that standard action now".into());
                }
                let (e, cr, hw) = cost;
                cx.add_resource(g, s, "energy", -e);
                cx.add_resource(g, s, "credits", -cr);
                cx.add_resource(g, s, HARDWARE, -hw);
                cx.tally(g, "board.hardware_spent", hw);
                cx.tally(g, &format!("board.std.{act}"), 1);
                cx.tally(g, "board.std_actions", 1);
                if act == "ambassador" {
                    cx.add_resource(g, s, AMBASSADORS, 1);
                    return Ok(());
                }
                if act == "hardware" {
                    cx.add_resource(g, s, HARDWARE, 1);
                    cx.tally(g, "board.hardware_bought", 1);
                    return Ok(());
                }
                if act == "presence" {
                    place_presence(g, cx, s)?;
                    return Ok(());
                }
                let opts = targets.iter().map(|&h| hex_opt(&w, h)).collect();
                let prompt = format!("Seat {s}: {act} on which hex?");
                let Some(i) = cx.choose(s, &prompt, opts)? else {
                    return Ok(());
                };
                let h = targets[i];
                match act {
                    "node" => {
                        adversary::remove_node(g, cx, h);
                    }
                    "station" => {
                        cx.add_tokens(g, h, STATION, 1);
                        cx.add_tokens(g, h, &cube(s), 1);
                    }
                    "remove" => {
                        adversary::remove_piece(g, cx, Some(s), h, ADV_AMBASSADOR);
                    }
                    "move" => move_ambassador(g, cx, s, h)?,
                    "align" => {
                        cx.add_resource(g, s, AMBASSADORS, -1);
                        cx.add_tokens(g, h, &cube(s), 1);
                        cx.tally(g, "board.cities_aligned", 1);
                    }
                    _ => {
                        cx.add_tokens(g, h, HARDEN, 1);
                    }
                }
                Ok(())
            }
            _ => Err(Halt::Illegal("not a standard action".into())),
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match phase {
            1 => produce_units(g, cx)?,
            2 => battle(g, cx),
            3 => strike(g, cx)?,
            4 => produce(g, cx)?,
            _ => {
                for c in 0..w.map.countries.len() {
                    for f in FLAGS {
                        cx.set_key(g, flag(f, c), 0);
                    }
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
        keys.extend(["board.project.", "board.project_done.", "board.track."]);
        out.bookkeeping(&keys);
        for c in 0..w.map.countries.len() {
            for f in FLAGS {
                if flagged(g, f.name(), c) {
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
        for act in STANDARD {
            let (e, cr, hw) = std_cost(w, act);
            let cost: Vec<String> = [(e, "energy"), (cr, "credits"), (hw, "hardware")]
                .iter()
                .filter(|(n, _)| *n > 0)
                .map(|(n, r)| format!("{n} {r}"))
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
            out.standard(act, e, cr, hw, &doc);
        }
        for s in 0..g.seats.len() {
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
            let left = ai::board(w, s)
                .presence
                .len()
                .saturating_sub(track_taken(g, s));
            out.seat(
                s,
                "presence_track",
                "presence left on the track",
                left as i64,
                None,
                "presence tokens still on your AI board's track",
            );
        }
    }
}

fn flag_doc(f: RoundFlag) -> &'static str {
    match f {
        RoundFlag::Blackout => "Blackout: cities here make no energy this round",
        RoundFlag::Sabotage => "Sabotage: presence here leaks no compute this round",
        RoundFlag::Shelter => "Shelter Order: buildings here can't be destroyed this round",
    }
}
