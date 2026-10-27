//! The board: one country piece per seat, placed by the layout for that
//! number of countries. Geometry is derived from content + config and is not
//! part of the game state; the state holds only what sits on each hex.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::content::Content;
use crate::hex::Hex;

#[derive(Debug, Clone, Serialize)]
pub struct Map {
    /// All hexes, country by country, each country in piece order
    /// (`i * size + j` for piece position `(i, j)`). A hex's index here is
    /// its index in `GameState::hexes`.
    pub hexes: Vec<Cell>,
    pub countries: Vec<CountryGeo>,
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
                    hexes.push(Cell {
                        hex,
                        country: c as u8,
                        piece_pos: [i as u8, j as u8],
                        neighbours: Vec::new(),
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
        Ok(Map {
            hexes,
            countries: geo,
            index,
        })
    }

    pub fn find(&self, hex: Hex) -> Option<usize> {
        self.index.get(&hex).copied()
    }

    pub fn country_of(&self, hex_index: usize) -> u8 {
        self.hexes[hex_index].country
    }
}
