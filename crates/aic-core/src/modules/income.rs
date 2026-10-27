//! Placeholder income: each country gains credits once per round.

use super::{BLIGHT, CREDITS};
use crate::module::{
    Build, Cx, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, VariantDef, World,
};
use crate::view::ViewOut;
use crate::GameState;

const INCOME_DOC: &str = "Credits this country would gain if income were collected now.";

pub static DEF: ModuleDef = ModuleDef {
    id: "income",
    summary: "Each country gains credits once per round.",
    variants: &[
        VariantDef {
            name: "flat",
            summary: "Every country gains `amount` credits.",
            params: &[ParamSpec { name: "amount", kind: ParamKind::Int, min: 0, max: 100, doc: "Credits per country per round." }],
            phases: PHASES,
            rng: &[],
            uses_cards: false,
            build: build_flat,
        },
        VariantDef {
            name: "clean_land",
            summary: "Every country gains `base` credits plus 1 per `per_clean` blight-free hexes it holds.",
            params: &[
                ParamSpec { name: "base", kind: ParamKind::Int, min: 0, max: 100, doc: "Credits every country gains." },
                ParamSpec {
                    name: "per_clean",
                    kind: ParamKind::Int,
                    min: 1,
                    max: 64,
                    doc: "Blight-free hexes per extra credit (rounded down).",
                },
            ],
            phases: PHASES,
            rng: &[],
            uses_cards: false,
            build: build_clean_land,
        },
    ],
};

const PHASES: &[PhaseSpec] = &[PhaseSpec {
    name: "collect",
    kind: PhaseKind::Auto,
    doc: "Every country gains its income.",
}];

#[derive(Debug)]
struct Flat {
    amount: i64,
}

fn build_flat(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Flat {
        amount: b.int("amount")?,
    }))
}

impl Module for Flat {
    fn view(&self, g: &GameState, _w: &World, out: &mut ViewOut) {
        for c in 0..g.seats.len() {
            out.country(c, "income", "income", self.amount, None, INCOME_DOC);
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) {
        for seat in 0..g.seats.len() {
            cx.add_resource(g, seat, CREDITS, self.amount);
            cx.count(g, "credits", self.amount);
        }
    }
}

#[derive(Debug)]
struct CleanLand {
    base: i64,
    per_clean: i64,
}

fn build_clean_land(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(CleanLand {
        base: b.int("base")?,
        per_clean: b.int("per_clean")?,
    }))
}

impl CleanLand {
    fn gain(&self, g: &GameState, w: &World, seat: usize) -> i64 {
        let clean = w.map.countries[seat]
            .hexes
            .iter()
            .filter(|&&h| g.tokens(h, BLIGHT) == 0)
            .count() as i64;
        self.base + clean / self.per_clean
    }
}

impl Module for CleanLand {
    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        for c in 0..g.seats.len() {
            out.country(c, "income", "income", self.gain(g, w, c), None, INCOME_DOC);
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for seat in 0..g.seats.len() {
            let gain = self.gain(g, &w, seat);
            cx.add_resource(g, seat, CREDITS, gain);
            cx.count(g, "credits", gain);
        }
    }
}
