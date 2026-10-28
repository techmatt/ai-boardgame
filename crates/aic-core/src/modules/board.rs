//! Board and terrain: the printed country pieces (terrain, uplinks, hex
//! numbers, in `content/map.json`), the starting pieces, and the round-end
//! cleanup of round markers (defense, fortify, country flags).

use crate::module::{Build, Cx, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, Res, VariantDef, World};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::*;

pub static DEF: ModuleDef = ModuleDef {
    id: "board",
    summary: "Country pieces with terrain and uplinks, the starting pieces, round-end cleanup",
    requires: &[],
    variants: &[VariantDef {
        name: "pieces",
        summary: "Each country is a printed piece: 4 hexes of each terrain, 2 Frontier uplinks",
        params: &[
            p("start_population", "population on each printed population hex"),
            p("start_factories", "factories on each printed factory hex"),
            p("start_data_centres", "data centres on each printed data centre hex"),
            p("start_drones", "drones on each uplink"),
            p("start_outposts", "outposts on each printed outpost uplink"),
        ],
        phases: &[PhaseSpec {
            name: "cleanup",
            kind: PhaseKind::Auto,
            doc: "remove defense and fortify tokens and the round's country flags",
        }],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

const fn p(name: &'static str, doc: &'static str) -> ParamSpec {
    ParamSpec {
        name,
        kind: ParamKind::Int,
        min: 0,
        max: 10,
        doc,
    }
}

/// Country flags that last until the end of the round.
pub const FLAGS: [&str; 3] = ["blockade", "sabotage", "shelter"];

/// The var key of a country flag.
pub fn flag(name: &str, country: usize) -> String {
    format!("board.{name}.{country}")
}

pub fn flagged(g: &GameState, name: &str, country: usize) -> bool {
    g.var(&flag(name, country)) > 0
}

/// Starting population (all countries), recorded at setup.
pub const START_POPULATION: &str = "board.start_population";

#[derive(Debug)]
struct Board {
    population: i64,
    factories: i64,
    data_centres: i64,
    drones: i64,
    outposts: i64,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Board {
        population: b.int("start_population")?,
        factories: b.int("start_factories")?,
        data_centres: b.int("start_data_centres")?,
        drones: b.int("start_drones")?,
        outposts: b.int("start_outposts")?,
    }))
}

impl Module for Board {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        let mut total = 0;
        for c in 0..w.map.countries.len() {
            let piece = &w.content.map.countries[c];
            let at = |n: u8| w.map.numbered(c, n);
            for &n in &piece.population {
                total += cx.add_tokens(g, at(n), POPULATION, self.population);
            }
            for &n in &piece.factories {
                cx.add_tokens(g, at(n), FACTORY, self.factories);
            }
            for &n in &piece.data_centres {
                cx.add_tokens(g, at(n), DATA_CENTRE, self.data_centres);
            }
            for &n in &piece.uplinks {
                cx.add_tokens(g, at(n), DRONE, self.drones);
            }
            for &n in &piece.outposts {
                cx.add_tokens(g, at(n), OUTPOST, self.outposts);
            }
        }
        cx.set_key(g, START_POPULATION.into(), total);
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        for h in 0..g.hexes.len() {
            let kinds: Vec<String> = g.hexes[h]
                .tokens
                .keys()
                .filter(|k| k.starts_with("defense:") || k.as_str() == FORTIFY)
                .cloned()
                .collect();
            for k in kinds {
                let n = g.tokens(h, &k);
                cx.add_tokens(g, h, &k, -n);
            }
        }
        for c in 0..w.map.countries.len() {
            for f in FLAGS {
                cx.set_key(g, flag(f, c), 0);
            }
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        for c in 0..w.map.countries.len() {
            for f in FLAGS {
                if flagged(g, f, c) {
                    out.country(c, f, f, 1, None, flag_doc(f));
                }
            }
        }
        out.global(
            "start_population",
            "starting population",
            g.var(START_POPULATION),
            None,
            "population on the board at setup",
        );
    }
}

fn flag_doc(f: &str) -> &'static str {
    match f {
        "blockade" => "Blockade: factories next to adversary pieces produce nothing this round",
        "sabotage" => "Sabotage: processors on data centres next to adversary pieces produce nothing this round",
        _ => "Shelter Order: population here can't be destroyed this round",
    }
}
