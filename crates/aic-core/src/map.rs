//! The board: one country piece per seat, placed by the layout for that
//! number of countries. Geometry is derived from content + config and is not
//! part of the game state; the state holds only what sits on each hex.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::content::{Content, HexKind};
use crate::hex::Hex;

#[derive(Debug, Clone, Serialize)]
pub struct Map {
    /// All hexes, country by country, each country in piece order
    /// (`i * size + j` for piece position `(i, j)`). A hex's index here is
    /// its index in `GameState::hexes`.
    pub hexes: Vec<Cell>,
    pub countries: Vec<CountryGeo>,
    /// Land borders between countries, each with a military base on either side.
    pub fronts: Vec<Front>,
    #[serde(skip)]
    index: BTreeMap<Hex, usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Cell {
    pub hex: Hex,
    pub country: u8,
    /// Position on its piece: steps along the piece's first and second axis.
    pub piece_pos: [u8; 2],
    /// Indices of on-board neighbours, in `Dir::ALL` order.
    pub neighbours: Vec<usize>,
    /// The printed hex number on its piece (1-16), for tie-breaks.
    pub number: u8,
    pub kind: HexKind,
    /// For a military base: the country across its front.
    pub facing: Option<u8>,
}

/// A front: a land border between two countries (an ocean edge is not one).
/// `bases[i]` is the military base of `countries[i]` on it.
#[derive(Debug, Clone, Serialize)]
pub struct Front {
    pub countries: [u8; 2],
    pub bases: [usize; 2],
}

#[derive(Debug, Clone, Serialize)]
pub struct CountryGeo {
    /// Hex indices of this country, in piece order.
    pub hexes: Vec<usize>,
    pub card_slots: u8,
    pub colour: String,
}

impl Map {
    pub fn build(content: &Content, countries: u8) -> Result<Map, String> {
        let layout = content
            .layout(countries)
            .ok_or_else(|| format!("no map layout for {countries} countries (content/map.json)"))?;
        let size = content.map.piece.size as i32;
        let mut hexes = Vec::new();
        let mut index = BTreeMap::new();
        let mut geo = Vec::new();
        for (c, p) in layout.pieces.iter().enumerate() {
            let design = content
                .map
                .countries
                .get(c)
                .ok_or_else(|| format!("no country piece for country {c} (content/map.json)"))?;
            let (a, b) = (p.axes[0].offset(), p.axes[1].offset());
            let mut mine = Vec::new();
            for i in 0..size {
                for j in 0..size {
                    let hex = p.anchor.add(a.scale(i)).add(b.scale(j));
                    if index.insert(hex, hexes.len()).is_some() {
                        return Err(format!(
                            "layout for {countries} countries: pieces overlap at {hex:?}"
                        ));
                    }
                    mine.push(hexes.len());
                    let number = (i * size + j + 1) as u8;
                    hexes.push(Cell {
                        hex,
                        country: c as u8,
                        piece_pos: [i as u8, j as u8],
                        neighbours: Vec::new(),
                        number,
                        kind: design.kind_at(number),
                        facing: None,
                    });
                }
            }
            geo.push(CountryGeo {
                hexes: mine,
                card_slots: content.map.piece.card_slots,
                colour: p.colour.clone(),
            });
        }
        for cell in hexes.iter_mut() {
            cell.neighbours = cell
                .hex
                .neighbours()
                .iter()
                .filter_map(|n| index.get(n).copied())
                .collect();
        }
        let fronts = place_bases(content, &mut hexes, &geo)?;
        Ok(Map {
            hexes,
            countries: geo,
            fronts,
            index,
        })
    }

    pub fn find(&self, hex: Hex) -> Option<usize> {
        self.index.get(&hex).copied()
    }

    pub fn country_of(&self, hex_index: usize) -> u8 {
        self.hexes[hex_index].country
    }

    /// Hex index of printed number `n` in country `c`.
    pub fn numbered(&self, c: usize, n: u8) -> usize {
        self.countries[c].hexes[n as usize - 1]
    }

    /// Whether two hexes are adjacent (neighbours on the board).
    pub fn adjacent(&self, a: usize, b: usize) -> bool {
        self.hexes[a].neighbours.contains(&b)
    }

    /// Hex distance between two hexes.
    pub fn distance(&self, a: usize, b: usize) -> i32 {
        self.hexes[a].hex.distance(self.hexes[b].hex)
    }
}

/// Finds the fronts and puts a military base on each side of each: on an
/// open hex of that country, touching land of the other, holding no setup
/// building. A country's fronts with the fewest candidates choose first;
/// each takes its lowest-numbered free candidate.
fn place_bases(content: &Content, hexes: &mut [Cell], geo: &[CountryGeo]) -> Result<Vec<Front>, String> {
    let n = geo.len();
    let land = |h: usize| hexes[h].kind != HexKind::Ocean;
    let mut touching = vec![vec![false; n]; n];
    for h in 0..hexes.len() {
        for &x in &hexes[h].neighbours {
            let (a, b) = (hexes[h].country as usize, hexes[x].country as usize);
            if a != b && land(h) && land(x) {
                touching[a][b] = true;
            }
        }
    }
    let mut base = vec![vec![usize::MAX; n]; n];
    for c in 0..n {
        let piece = &content.map.countries[c];
        let busy = |num: u8| {
            piece.stations.contains(&num) || piece.factories.contains(&num) || piece.data_centres.contains(&num)
        };
        let candidates = |o: usize| -> Vec<usize> {
            geo[c]
                .hexes
                .iter()
                .copied()
                .filter(|&h| hexes[h].kind == HexKind::Open && !busy(hexes[h].number))
                .filter(|&h| hexes[h].neighbours.iter().any(|&x| hexes[x].country as usize == o && land(x)))
                .collect()
        };
        let mut others: Vec<usize> = (0..n).filter(|&o| touching[c][o]).collect();
        others.sort_by_key(|&o| (candidates(o).len(), o));
        let mut used: Vec<usize> = Vec::new();
        for o in others {
            let h = candidates(o)
                .into_iter()
                .find(|h| !used.contains(h))
                .ok_or_else(|| format!("country {c}: no free hex for a military base facing country {o}"))?;
            used.push(h);
            base[c][o] = h;
        }
    }
    let mut fronts = Vec::new();
    for a in 0..n {
        for b in a + 1..n {
            if touching[a][b] {
                let bases = [base[a][b], base[b][a]];
                hexes[bases[0]].kind = HexKind::Base;
                hexes[bases[0]].facing = Some(b as u8);
                hexes[bases[1]].kind = HexKind::Base;
                hexes[bases[1]].facing = Some(a as u8);
                fronts.push(Front {
                    countries: [a as u8, b as u8],
                    bases,
                });
            }
        }
    }
    Ok(fronts)
}
