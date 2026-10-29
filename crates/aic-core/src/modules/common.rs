//! Shared vocabulary of the ruleset: piece names, board queries and choice
//! options. Rules live in the modules; this only answers questions about
//! the board.
//!
//! Terms used throughout (see `docs/rules.md`):
//! - **adjacent**: a neighbouring hex on the board (across country borders too);
//! - **next to X**: holding X, or adjacent to a hex holding X;
//! - **reach**: a seat's reach is every hex within 1 of one of its data centres.

use crate::content::HexKind;
use crate::hex::Hex;
use crate::module::World;
use crate::state::{GameState, Opt};

pub const FACTORY: &str = "factory";
pub const STATION: &str = "station";
/// The adversary's owner cube (on a city or research station): corrupted.
pub const ADV_CUBE: &str = "cube:adversary";
pub const ADV_AMBASSADOR: &str = "ambassador:adversary";
/// An adversary troop: stays on the board and strikes in production.
pub const TROOP: &str = "troop";
/// Corp units waiting at a military base (the base's country's corp).
pub const UNIT: &str = "unit";
/// A hardening token: absorbs one destruction of a building on its hex.
pub const HARDEN: &str = "harden";
/// Seat resource: player ambassadors in the seat's supply.
pub const AMBASSADORS: &str = "ambassadors";

/// Seat `s`'s data centre on a hex.
pub fn dc(seat: usize) -> String {
    format!("dc:{seat}")
}

/// Seat `s`'s owner cube (an aligned city or an owned station).
pub fn cube(seat: usize) -> String {
    format!("cube:{seat}")
}

/// Seat `s`'s player ambassador on a piece.
pub fn ambassador(seat: usize) -> String {
    format!("ambassador:{seat}")
}

pub fn is_city(w: &World, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::City
}

pub fn is_ocean(w: &World, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::Ocean
}

pub fn is_base(w: &World, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::Base
}

pub fn corrupted(g: &GameState, h: usize) -> bool {
    g.tokens(h, ADV_CUBE) > 0
}

/// The seat whose cube is on hex `h`, if any.
pub fn cube_owner(g: &GameState, h: usize) -> Option<usize> {
    (0..g.seats.len()).find(|&s| g.tokens(h, &cube(s)) > 0)
}

/// A city with no cube.
pub fn clean_city(w: &World, g: &GameState, h: usize) -> bool {
    is_city(w, h) && !corrupted(g, h) && cube_owner(g, h).is_none()
}

/// The seat whose data centre is on hex `h`, if any.
pub fn dc_owner(g: &GameState, h: usize) -> Option<usize> {
    (0..g.seats.len()).find(|&s| g.tokens(h, &dc(s)) > 0)
}

/// Whether hex `h` holds a building (factory, research station, data centre).
pub fn has_building(g: &GameState, h: usize) -> bool {
    g.tokens(h, FACTORY) > 0 || g.tokens(h, STATION) > 0 || dc_owner(g, h).is_some()
}

/// An open hex with no building: where buildings go.
pub fn buildable(w: &World, g: &GameState, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::Open && !has_building(g, h)
}

/// Whether hex `h` holds a piece an ambassador can sit on.
pub fn has_piece(w: &World, g: &GameState, h: usize) -> bool {
    is_city(w, h) || is_base(w, h) || has_building(g, h)
}

/// Player-owned: a data centre, or an aligned city or owned station.
pub fn player_owned(g: &GameState, h: usize) -> bool {
    dc_owner(g, h).is_some() || cube_owner(g, h).is_some()
}

pub fn adversary_count(g: &GameState, h: usize) -> i64 {
    g.tokens(h, ADV_AMBASSADOR) + g.tokens(h, TROOP)
}

/// Adversary ambassadors or troops on the hex.
pub fn has_adversary(g: &GameState, h: usize) -> bool {
    adversary_count(g, h) > 0
}

/// Whether hex `h` holds, or is adjacent to a hex holding, something `pred` accepts.
pub fn next_to(w: &World, h: usize, pred: impl Fn(usize) -> bool) -> bool {
    pred(h) || w.map.hexes[h].neighbours.iter().any(|&n| pred(n))
}

/// Hexes of country `c`, or of the whole board for `None`, in board order.
pub fn scope_hexes(w: &World, scope: Option<usize>) -> Vec<usize> {
    match scope {
        Some(c) => w.map.countries[c].hexes.clone(),
        None => (0..w.map.hexes.len()).collect(),
    }
}

pub fn scope_countries(w: &World, scope: Option<usize>) -> Vec<usize> {
    match scope {
        Some(c) => vec![c],
        None => (0..w.map.countries.len()).collect(),
    }
}

/// Hexes holding seat `s`'s data centres, in board order.
pub fn dcs_of(g: &GameState, seat: usize) -> Vec<usize> {
    let k = dc(seat);
    (0..g.hexes.len()).filter(|&h| g.tokens(h, &k) > 0).collect()
}

pub fn dc_count(g: &GameState, seat: usize) -> i64 {
    dcs_of(g, seat).len() as i64
}

/// Whether hex `h` is within seat `s`'s reach (1 of one of its data centres).
pub fn in_reach(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    let k = dc(seat);
    next_to(w, h, |x| g.tokens(x, &k) > 0)
}

pub fn has_dc_in(w: &World, g: &GameState, seat: usize, country: usize) -> bool {
    let k = dc(seat);
    w.map.countries[country].hexes.iter().any(|&h| g.tokens(h, &k) > 0)
}

/// Seats with a data centre in `country`, in seat order.
pub fn seats_in_country(w: &World, g: &GameState, country: usize) -> Vec<usize> {
    (0..g.seats.len()).filter(|&s| has_dc_in(w, g, s, country)).collect()
}

/// Seats a threat in `scope` concerns: with a data centre in the country,
/// or every seat for the whole board.
pub fn seats_in_scope(w: &World, g: &GameState, scope: Option<usize>) -> Vec<usize> {
    match scope {
        Some(c) => seats_in_country(w, g, c),
        None => (0..g.seats.len()).collect(),
    }
}

/// "C1 #5" : country 1, printed hex number 5.
pub fn hex_name(w: &World, h: usize) -> String {
    let c = &w.map.hexes[h];
    format!("C{} #{}", c.country, c.number)
}

/// An option naming a hex (the viewer offers it as a hex pick).
pub fn hex_opt(w: &World, h: usize) -> Opt {
    Opt {
        option: hex_name(w, h),
        target: Some(w.map.hexes[h].hex),
    }
}

pub fn opt(label: impl Into<String>) -> Opt {
    Opt {
        option: label.into(),
        target: None,
    }
}

/// The hex of option `i` among hex options built from `hexes`.
pub fn hex_at(w: &World, hexes: &[usize], at: Option<Hex>) -> Option<usize> {
    let at = at?;
    hexes.iter().copied().find(|&h| w.map.hexes[h].hex == at)
}

/// Seat `s` as an option label.
pub fn seat_opt(s: usize) -> Opt {
    opt(format!("seat {s}"))
}

pub fn country_opt(c: usize) -> Opt {
    opt(format!("country {c}"))
}
