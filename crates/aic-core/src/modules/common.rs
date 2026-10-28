//! Shared vocabulary of the v0 ruleset: piece names, board queries and
//! choice options. Rules live in the modules; this only answers questions
//! about the board.
//!
//! Terms used throughout (see `docs/rules.md`):
//! - **adjacent**: a neighbouring hex on the board (across country borders too);
//! - **next to X**: holding X, or adjacent to a hex holding X.

use crate::content::{Selector, Terrain};
use crate::hex::Hex;
use crate::module::World;
use crate::state::{GameState, Opt};

pub const POPULATION: &str = "population";
pub const FACTORY: &str = "factory";
pub const DATA_CENTRE: &str = "data_centre";
pub const DRONE: &str = "drone";
pub const OUTPOST: &str = "outpost";
pub const FOUNDRY: &str = "foundry";
pub const CORRUPTION: &str = "corruption";
/// Adversary defense from the Fortify threat: prevents player damage.
pub const FORTIFY: &str = "fortify";

/// Adversary pieces, weakest first.
pub const ADVERSARY: [&str; 3] = [DRONE, OUTPOST, FOUNDRY];

/// Seat `s`'s processors on a hex.
pub fn processor(seat: usize) -> String {
    format!("processor:{seat}")
}

/// Seat `s`'s defense tokens on a hex.
pub fn defense(seat: usize) -> String {
    format!("defense:{seat}")
}

pub fn adversary_count(g: &GameState, h: usize) -> i64 {
    ADVERSARY.iter().map(|k| g.tokens(h, k)).sum()
}

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

pub fn processors_of(g: &GameState, seat: usize) -> i64 {
    let k = processor(seat);
    (0..g.hexes.len()).map(|h| g.tokens(h, &k)).sum()
}

pub fn has_processor_in(w: &World, g: &GameState, seat: usize, country: usize) -> bool {
    let k = processor(seat);
    w.map.countries[country]
        .hexes
        .iter()
        .any(|&h| g.tokens(h, &k) > 0)
}

/// Seats with a processor in `country`, in seat order.
pub fn seats_in_country(w: &World, g: &GameState, country: usize) -> Vec<usize> {
    (0..g.seats.len())
        .filter(|&s| has_processor_in(w, g, s, country))
        .collect()
}

/// Seats a threat in `scope` concerns: with a processor in the country, or
/// every seat for the whole board.
pub fn seats_in_scope(w: &World, g: &GameState, scope: Option<usize>) -> Vec<usize> {
    match scope {
        Some(c) => seats_in_country(w, g, c),
        None => (0..g.seats.len()).collect(),
    }
}

pub fn matches(w: &World, g: &GameState, h: usize, sel: &Selector) -> bool {
    (!sel.uplink || w.map.hexes[h].uplink)
        && sel.has.iter().all(|k| g.tokens(h, k) > 0)
        && sel.adversary_at_least.is_none_or(|n| adversary_count(g, h) >= n)
        && (!sel.corrupted || g.tokens(h, CORRUPTION) > 0)
}

pub fn has_terrain(w: &World, h: usize, terrains: &[Terrain]) -> bool {
    terrains.contains(&w.map.hexes[h].terrain)
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
