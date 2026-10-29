//! Shared vocabulary of the ruleset: piece names, board queries and choice
//! options. Rules live in the modules; this only answers questions about
//! the board.
//!
//! Terms used throughout (see `docs/rules.md`):
//! - **adjacent**: a neighbouring hex on the board (across country borders too);
//! - **next to X**: holding X, or adjacent to a hex holding X;
//! - **reach**: a seat's reach is every hex within 1 of one of its presence.
//! - **presence**: a player's tokens on the board (one per hex), placed off
//!   their AI board's presence track.

use crate::content::HexKind;
use crate::module::World;
use crate::state::{GameState, Opt};

pub const FACTORY: &str = "factory";
pub const STATION: &str = "station";
/// The adversary's owner cube (on a city or research station): corrupted.
pub const ADV_CUBE: &str = "cube:adversary";
pub const ADV_AMBASSADOR: &str = "ambassador:adversary";
/// An adversary troop: marches in the adversary phase (019) and is used up
/// when it strikes.
pub const TROOP: &str = "troop";
/// Corp units waiting at a military base (the base's country's corp).
pub const UNIT: &str = "unit";
/// A hardening token: absorbs one destruction of a building on its hex.
pub const HARDEN: &str = "harden";
/// Seat resource: player ambassadors in the seat's supply.
pub const AMBASSADORS: &str = "ambassadors";
/// Seat resource: hardware, made by player-owned factories.
pub const HARDWARE: &str = "hardware";
/// Seat resource: destroyed presence back beside the seat's board, placed
/// before the track's next token (018).
pub const RESERVE: &str = "presence_reserve";
/// An adversary node: persistent, makes adversary compute, blocks claims next to it.
pub const NODE: &str = "node";

/// Seat `s`'s presence on a hex.
pub fn presence(seat: usize) -> String {
    format!("presence:{seat}")
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

/// The seat whose presence is on hex `h`, if any.
pub fn presence_owner(g: &GameState, h: usize) -> Option<usize> {
    (0..g.seats.len()).find(|&s| g.tokens(h, &presence(s)) > 0)
}

/// Whether hex `h` holds a building (factory, research station).
pub fn has_building(g: &GameState, h: usize) -> bool {
    g.tokens(h, FACTORY) > 0 || g.tokens(h, STATION) > 0
}

/// An open hex with no building: where buildings go.
pub fn buildable(w: &World, g: &GameState, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::Open && !has_building(g, h)
}

/// Whether hex `h` holds a piece an ambassador can sit on (presence included).
pub fn has_piece(w: &World, g: &GameState, h: usize) -> bool {
    is_city(w, h) || is_base(w, h) || has_building(g, h) || presence_owner(g, h).is_some()
}

/// Player-owned: a presence, or an aligned city or owned station.
pub fn player_owned(g: &GameState, h: usize) -> bool {
    presence_owner(g, h).is_some() || cube_owner(g, h).is_some()
}

/// A player ambassador's seat on hex `h`, if any (seat order).
pub fn player_ambassador(g: &GameState, h: usize) -> Option<usize> {
    (0..g.seats.len()).find(|&s| g.tokens(h, &ambassador(s)) > 0)
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

/// Free industrial zones (no building): where a factory may be built.
pub fn factory_hexes(w: &World, g: &GameState) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| w.map.hexes[h].industrial && !has_building(g, h))
        .collect()
}

/// The country seat `s` starts in (one seat per country, wrapping).
pub fn home_country(w: &World, seat: usize) -> usize {
    seat % w.map.countries.len()
}

/// Hexes holding seat `s`'s presence, in board order.
pub fn presence_of(g: &GameState, seat: usize) -> Vec<usize> {
    let k = presence(seat);
    (0..g.hexes.len())
        .filter(|&h| g.tokens(h, &k) > 0)
        .collect()
}

pub fn presence_count(g: &GameState, seat: usize) -> i64 {
    presence_of(g, seat).len() as i64
}

/// Whether hex `h` is within seat `s`'s reach (1 of one of its presence).
pub fn in_reach(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    let k = presence(seat);
    next_to(w, h, |x| g.tokens(x, &k) > 0)
}

pub fn has_presence_in(w: &World, g: &GameState, seat: usize, country: usize) -> bool {
    let k = presence(seat);
    w.map.countries[country]
        .hexes
        .iter()
        .any(|&h| g.tokens(h, &k) > 0)
}

/// Seats with a presence in `country`, in seat order.
pub fn seats_in_country(w: &World, g: &GameState, country: usize) -> Vec<usize> {
    (0..g.seats.len())
        .filter(|&s| has_presence_in(w, g, s, country))
        .collect()
}

/// Seats a threat in `scope` concerns: with a presence in the country,
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

/// Seat `s` as an option label.
pub fn seat_opt(s: usize) -> Opt {
    opt(format!("seat {s}"))
}

pub fn country_opt(c: usize) -> Opt {
    opt(format!("country {c}"))
}
