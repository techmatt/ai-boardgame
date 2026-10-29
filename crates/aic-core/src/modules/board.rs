//! The board (014): the printed country pieces (cities, open land, ocean;
//! military bases on the land borders), the setup, production (bases and
//! factories, front battles on war rounds, adversary troops, cities,
//! stations and data centres, megaprojects), player ambassadors, the
//! standard actions and the round-end cleanup.

use crate::action::Action;
use crate::content::{Passive, RoundFlag, Side};
use crate::module::{scheduled, Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::*;
use super::{adversary, ai, alignment, cards};

pub static DEF: ModuleDef = ModuleDef {
    id: "board",
    summary: "Country pieces, cities, buildings, bases and fronts; production, war, ambassadors, standard actions",
    requires: &["adversary", "ai", "cards", "alignment"],
    variants: &[VariantDef {
        name: "corps",
        summary: "Each country is a printed piece with cities, a research station, factories and a corp's bases",
        params: &[
            ParamSpec::int("start_corrupted", 0, 3, "cities per country that start corrupted"),
            ParamSpec::int("start_ambassadors", 0, 10, "player ambassadors each player starts with"),
            ParamSpec::int("troop_health", 0, 10, "removal a card must spend on an adversary troop (an ambassador takes 1)"),
            ParamSpec::int("unit_cap", 0, 20, "corp units a base holds at most"),
            ParamSpec::int("factory_units", 0, 5, "units each factory builds a round for its corp's nearest base"),
            ParamSpec::list("war", 0, 1, "per round, 1 for a war round (the last entry repeats)"),
            ParamSpec::int("city_energy", 0, 10, "energy a city makes a round"),
            ParamSpec::int("aligned_bonus", 0, 10, "extra energy an aligned city gives its owner"),
            ParamSpec::int("dc_compute", 0, 10, "compute a data centre makes a round"),
            ParamSpec::int("station_cubes", 0, 5, "megaproject cubes a research station makes a round"),
            ParamSpec::int("project_credits", 0, 50, "credits each player cube pays when a megaproject completes"),
            ParamSpec::int("project_discord_per", 0, 10, "adversary cubes per Discord when a megaproject completes"),
            ParamSpec::int("divert_discord", 0, 5, "Discord per unit of energy or compute diverted to the adversary"),
            ParamSpec::int("base_discord", 0, 10, "Discord an adversary ambassador on a producing base sways"),
            ParamSpec::int("base_unity", 0, 10, "Unity a player ambassador on a producing base sways"),
            ParamSpec::int("factory_unity", 0, 10, "Unity a player ambassador on a producing factory sways"),
            ParamSpec::int("free_city_at", 0, 10, "player ambassadors on a corrupted city that free it"),
            ParamSpec::int("destruction_discord", 0, 5, "Discord each destroyed building sways"),
            ParamSpec::int("dc_energy", 0, 20, "standard action: energy to place a data centre"),
            ParamSpec::int("dc_credits", 0, 50, "standard action: credits to place a data centre"),
            ParamSpec::int("build_energy", 0, 20, "standard action: energy to build a factory or research station"),
            ParamSpec::int("build_credits", 0, 50, "standard action: credits to build a factory or research station"),
            ParamSpec::int("remove_energy", 0, 20, "standard action: energy to remove an adversary ambassador within reach"),
            ParamSpec::int("move_energy", 0, 20, "standard action: energy to move a player ambassador"),
            ParamSpec::int("ambassador_credits", 0, 50, "standard action: credits to gain a player ambassador"),
            ParamSpec::int("harden_credits", 0, 50, "standard action (Harden passive): credits a hardening token"),
            ParamSpec::int("sell_credits", 0, 20, "standard action: credits for selling a card from hand"),
            ParamSpec::int("align_energy", 0, 20, "standard action: energy (and 1 ambassador) to align a clean city"),
        ],
        phases: &[
            PhaseSpec::decision("act", "standard actions: build, place, remove, move, align, harden, sell"),
            PhaseSpec::auto("bases", "bases and factories produce corp units (ambassadors divert them)"),
            PhaseSpec::auto("war", "on a war round, the units on either side of each front fight"),
            PhaseSpec::auto("strike", "each adversary troop destroys an adjacent building"),
            PhaseSpec::auto("produce", "cities, research stations and data centres produce (ambassadors divert)"),
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
pub const STANDARD: [&str; 8] = ["data_centre", "factory", "station", "remove", "move", "ambassador", "align", "harden"];

/// Whether round `round` is a war round (the schedule, or this round's adversary card).
pub fn war_round(w: &World, g: &GameState) -> bool {
    scheduled(w.list("board.war"), g.round) > 0 || adversary::war_twist(w, g)
}

#[derive(Debug)]
struct Board;

fn build(_b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Board))
}

/// A standard action's cost: (energy, credits).
fn std_cost(w: &World, act: &str) -> (i64, i64) {
    let i = |k: &str| w.int(&format!("board.{k}"));
    match act {
        "data_centre" => (i("dc_energy"), i("dc_credits")),
        "factory" | "station" => (i("build_energy"), i("build_credits")),
        "remove" => (i("remove_energy"), 0),
        "move" => (i("move_energy"), 0),
        "ambassador" => (0, i("ambassador_credits")),
        "align" => (i("align_energy"), 0),
        _ => (0, i("harden_credits")),
    }
}

/// The hexes a standard action of `seat` may target (empty: not offered).
fn std_targets(w: &World, g: &GameState, seat: usize, act: &str) -> Vec<usize> {
    let reach = |h: usize| in_reach(w, g, seat, h);
    let all = 0..g.hexes.len();
    match act {
        "data_centre" => {
            if ai::can_place(w, g, seat) {
                ai::dc_hexes(w, g, seat)
            } else {
                Vec::new()
            }
        }
        "factory" => factory_hexes(w, g).into_iter().filter(|&h| reach(h)).collect(),
        "station" => all.filter(|&h| buildable(w, g, h) && reach(h)).collect(),
        "remove" => all.filter(|&h| reach(h) && g.tokens(h, ADV_AMBASSADOR) > 0).collect(),
        "move" => {
            if g.resource(seat, AMBASSADORS) > 0 || ambassadors_on_board(g, seat) > 0 {
                all.filter(|&h| reach(h) && has_piece(w, g, h)).collect()
            } else {
                Vec::new()
            }
        }
        "align" => {
            if g.resource(seat, AMBASSADORS) > 0 {
                all.filter(|&h| reach(h) && clean_city(w, g, h)).collect()
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
        // Gaining an ambassador needs no target.
        _ => vec![usize::MAX],
    }
}

pub fn ambassadors_on_board(g: &GameState, seat: usize) -> i64 {
    let k = ambassador(seat);
    (0..g.hexes.len()).map(|h| g.tokens(h, &k)).sum()
}

/// Seat `s`'s player ambassador arrives on hex `h`: it cancels an
/// adversary ambassador there (both removed, no Unity); on a neutral
/// station it makes the station the seat's; on a corrupted city, enough of
/// them free it. Otherwise it waits on the piece.
pub fn place_ambassador(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let w = cx.world;
    if g.tokens(h, ADV_AMBASSADOR) > 0 {
        adversary::remove(g, cx, h, 1);
        cx.tally(g, "board.cancelled", 1);
        return;
    }
    if g.tokens(h, STATION) > 0 && !corrupted(g, h) && cube_owner(g, h).is_none() {
        cx.add_tokens(g, h, &cube(seat), 1);
        cx.tally(g, "board.stations_owned", 1);
        cx.note(Some(h), || format!("Seat {seat} takes the station at {}", hex_name(&w, h)));
        return;
    }
    cx.add_tokens(g, h, &ambassador(seat), 1);
    if is_city(&w, h) && corrupted(g, h) {
        let here: i64 = (0..g.seats.len()).map(|s| g.tokens(h, &ambassador(s))).sum();
        if here >= w.int("board.free_city_at") {
            for s in 0..g.seats.len() {
                let n = g.tokens(h, &ambassador(s));
                cx.add_tokens(g, h, &ambassador(s), -n);
            }
            cx.add_tokens(g, h, ADV_CUBE, -1);
            cx.tally(g, "board.cities_freed", 1);
            cx.note(Some(h), || format!("{} is freed", hex_name(&w, h)));
        }
    }
}

/// Moves one of `seat`'s ambassadors (from its supply or a hex, the seat
/// chooses) onto hex `to`.
pub fn move_ambassador(g: &mut GameState, cx: &mut Cx, seat: usize, to: usize) -> Res {
    let w = cx.world;
    let k = ambassador(seat);
    let from: Vec<usize> = (0..g.hexes.len()).filter(|&h| h != to && g.tokens(h, &k) > 0).collect();
    let mut opts = Vec::new();
    if g.resource(seat, AMBASSADORS) > 0 {
        opts.push(opt("from your supply"));
    }
    let off = opts.len();
    opts.extend(from.iter().map(|&h| hex_opt(&w, h)));
    let prompt = format!("Seat {seat}: move which ambassador to {}?", hex_name(&w, to));
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
    if g.tokens(h, FACTORY) > 0 {
        out.push(FACTORY.to_string());
    }
    if g.tokens(h, STATION) > 0 && !corrupted(g, h) {
        out.push(STATION.to_string());
    }
    if let Some(s) = dc_owner(g, h) {
        out.push(dc(s));
    }
    out
}

/// A building of `kind` on hex `h` would be destroyed. A Shelter Order in
/// its country, a hardening token, a player ambassador on the hex, or a
/// Civic Shelter absorbs it (in that order); otherwise it goes and sways
/// toward Discord. Returns whether it was destroyed.
pub fn destroy_building(g: &mut GameState, cx: &mut Cx, h: usize, kind: &str) -> bool {
    let w = cx.world;
    let c = w.map.hexes[h].country as usize;
    if flagged(g, RoundFlag::Shelter.name(), c) {
        return false;
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
    if kind == STATION {
        if let Some(s) = cube_owner(g, h) {
            cx.add_tokens(g, h, &cube(s), -1);
        }
    }
    cx.tally(g, "board.destroyed", 1);
    cx.note(Some(h), || format!("{kind} at {} destroyed", hex_name(&w, h)));
    alignment::sway(g, cx, Side::Discord, w.int("board.destruction_discord"));
    true
}

/// Uses a Civic Shelter for a building destruction in `country`, if one applies.
fn civic_shelter(g: &mut GameState, cx: &mut Cx, country: usize) -> bool {
    let w = cx.world;
    for s in 0..g.seats.len() {
        if g.var(&cards::civic_used(s)) == 0
            && has_dc_in(&w, g, s, country)
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
    hexes.min_by_key(|&h| (w.map.distance(from, h), w.map.hexes[h].country, w.map.hexes[h].number))
}

fn add_unit(g: &mut GameState, cx: &mut Cx, base: usize) {
    let w = cx.world;
    if g.tokens(base, UNIT) < w.int("board.unit_cap") {
        cx.add_tokens(g, base, UNIT, 1);
        cx.tally(g, "board.units_built", 1);
    }
}

/// Production step 1: bases, then factories, build their corp's units; an
/// ambassador on the piece diverts it and is removed.
fn produce_units(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
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
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 || per == 0 {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 {
            adversary::remove(g, cx, h, 1);
            adversary::place_troop(g, cx, h);
            cx.tally(g, "board.troops_built", 1);
            cx.note(Some(h), || format!("The factory at {} builds an adversary troop", hex_name(&w, h)));
        } else if let Some(s) = player_ambassador(g, h) {
            cx.add_tokens(g, h, &ambassador(s), -1);
            alignment::sway(g, cx, Side::Unity, w.int("board.factory_unity"));
        } else {
            let c = w.map.hexes[h].country;
            let bases = (0..g.hexes.len()).filter(|&b| is_base(&w, b) && w.map.hexes[b].country == c);
            if let Some(b) = nearest(&w, h, bases) {
                for _ in 0..per {
                    add_unit(g, cx, b);
                }
            }
        }
    }
}

/// Production step 2, on a war round: on each front the units cancel 1 for
/// 1; each surplus unit crosses and destroys the rival factory nearest its
/// base, or with none a data centre. All units at the front are used up.
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
            let target = nearest(&w, from, hexes.iter().copied().filter(|&h| g.tokens(h, FACTORY) > 0))
                .map(|h| (h, FACTORY.to_string()))
                .or_else(|| {
                    nearest(&w, from, hexes.iter().copied().filter(|&h| dc_owner(g, h).is_some()))
                        .map(|h| (h, dc(dc_owner(g, h).unwrap())))
                });
            let Some((h, kind)) = target else {
                break;
            };
            if destroy_building(g, cx, h, &kind) {
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
    let w = cx.world;
    let troops: Vec<(usize, i64)> = (0..g.hexes.len()).map(|h| (h, g.tokens(h, TROOP))).filter(|&(_, n)| n > 0).collect();
    for (h, n) in troops {
        for _ in 0..n {
            let mut targets: Vec<(usize, String)> = Vec::new();
            let mut near = vec![h];
            near.extend(w.map.hexes[h].neighbours.iter().copied());
            for x in near {
                for k in buildings_on(g, x) {
                    targets.push((x, k));
                }
            }
            let opts = targets.iter().map(|(x, k)| opt(format!("{k} at {}", hex_name(&w, *x)))).collect();
            let prompt = format!("Adversary troop at {}: which building does it destroy?", hex_name(&w, h));
            let Some(i) = cx.choose(0, &prompt, opts)? else {
                break;
            };
            let (x, k) = targets[i].clone();
            if destroy_building(g, cx, x, &k) {
                cx.tally(g, "board.troop_destroyed", 1);
            }
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
            // The player with the most data centres next to it; players choose on a tie.
            let near: Vec<i64> = (0..g.seats.len())
                .map(|s| w.map.hexes[h].neighbours.iter().filter(|&&x| g.tokens(x, &dc(s)) > 0).count() as i64)
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
    for h in 0..g.hexes.len() {
        let Some(s) = dc_owner(g, h) else {
            continue;
        };
        let c = w.map.hexes[h].country as usize;
        if flagged(g, RoundFlag::Sabotage.name(), c) {
            continue;
        }
        let n = w.int("board.dc_compute");
        if g.tokens(h, ADV_AMBASSADOR) > 0 {
            adversary::remove(g, cx, h, 1);
            alignment::sway(g, cx, Side::Discord, n * divert);
            cx.tally(g, "board.compute_diverted", n);
        } else {
            cx.add_resource(g, s, "compute", n);
        }
    }
    Ok(())
}

/// Completes country `c`'s megaproject if it is full: each player cube pays
/// its owner, adversary cubes become Discord.
fn complete_project(g: &mut GameState, cx: &mut Cx, c: usize) {
    let w = cx.world;
    let p = &w.content.map.countries[c].project;
    let adv = g.var(&project_key(c, None));
    let mine: Vec<i64> = (0..g.seats.len()).map(|s| g.var(&project_key(c, Some(s)))).collect();
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
            for &n in &w.content.map.countries[c].data_centres {
                cx.add_tokens(g, w.map.numbered(c, n), &dc(s), 1);
            }
            cx.add_resource(g, s, AMBASSADORS, w.int("board.start_ambassadors"));
        }
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        for act in STANDARD {
            let (e, cr) = std_cost(w, act);
            if g.resource(s, "energy") < e || g.resource(s, "credits") < cr {
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
            out.push(Action::Sell { seat, card: id.clone() });
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
                let (e, cr) = std_cost(&w, act);
                let targets = std_targets(&w, g, s, act);
                if g.resource(s, "energy") < e || g.resource(s, "credits") < cr || targets.is_empty() {
                    return Err("can't take that standard action now".into());
                }
                cx.add_resource(g, s, "energy", -e);
                cx.add_resource(g, s, "credits", -cr);
                cx.tally(g, &format!("board.std.{act}"), 1);
                cx.tally(g, "board.std_actions", 1);
                if act == "ambassador" {
                    cx.add_resource(g, s, AMBASSADORS, 1);
                    return Ok(());
                }
                if act == "data_centre" {
                    ai::place_data_centre(g, cx, s)?;
                    return Ok(());
                }
                let opts = targets.iter().map(|&h| hex_opt(&w, h)).collect();
                let prompt = format!("Seat {s}: {act} on which hex?");
                let Some(i) = cx.choose(s, &prompt, opts)? else {
                    return Ok(());
                };
                let h = targets[i];
                match act {
                    "factory" => {
                        cx.add_tokens(g, h, FACTORY, 1);
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
            1 => produce_units(g, cx),
            2 => battle(g, cx),
            3 => strike(g, cx)?,
            4 => produce(g, cx)?,
            _ => {
                for c in 0..w.map.countries.len() {
                    for f in FLAGS {
                        cx.set_key(g, flag(f, c), 0);
                    }
                }
            }
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let flags: Vec<String> = FLAGS.iter().map(|f| format!("board.{}.", f.name())).collect();
        let mut keys: Vec<&str> = flags.iter().map(String::as_str).collect();
        keys.extend(["board.project.", "board.project_done."]);
        out.bookkeeping(&keys);
        for c in 0..w.map.countries.len() {
            for f in FLAGS {
                if flagged(g, f.name(), c) {
                    out.country(c, f.name(), f.name(), 1, None, flag_doc(f));
                }
            }
            let p = &w.content.map.countries[c].project;
            let done = g.var(&project_done(c)) > 0;
            let total: i64 = g.var(&project_key(c, None)) + (0..g.seats.len()).map(|s| g.var(&project_key(c, Some(s)))).sum::<i64>();
            let doc = format!(
                "{}: {} ({} cubes; each player cube pays {} credits, every {} adversary cubes 1 Discord)",
                p.name,
                p.text,
                p.cubes,
                w.int("board.project_credits"),
                w.int("board.project_discord_per")
            );
            out.country(c, "project", &format!("megaproject: {}", p.name), if done { p.cubes } else { total }, Some(p.cubes), &doc);
            if !done {
                out.country(c, "project.adversary", "adversary cubes", g.var(&project_key(c, None)), None, "adversary cubes on the megaproject");
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
                out.country(c, "project.done", "completed", 1, None, "owned stations here produce the project's benefit");
            }
        }
        let war = war_round(w, g) as i64;
        out.global("war", "war round", war, None, "1 on a war round: in production the fronts fight");
        let rounds: Vec<String> = (1..=12u32).filter(|&r| scheduled(w.list("board.war"), r) > 0).map(|r| r.to_string()).collect();
        out.global(
            "fronts",
            "fronts",
            w.map.fronts.len() as i64,
            None,
            &format!("land borders between countries, each with a base on either side; war rounds {}…", rounds.join(", ")),
        );
        for s in 0..g.seats.len() {
            out.seat(s, "ambassadors_board", "ambassadors on the board", ambassadors_on_board(g, s), None, "your player ambassadors on pieces");
            out.seat(s, "data_centres", "data centres", dc_count(g, s), None, "your data centres: you act within 1 of them");
        }
    }
}

fn flag_doc(f: RoundFlag) -> &'static str {
    match f {
        RoundFlag::Blackout => "Blackout: cities here make no energy this round",
        RoundFlag::Sabotage => "Sabotage: data centres here make no compute this round",
        RoundFlag::Shelter => "Shelter Order: buildings here can't be destroyed this round",
    }
}
