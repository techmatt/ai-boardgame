//! Placeholder end condition: survive a number of rounds without any country
//! being overrun.

use super::{country_tokens, BLIGHT};
use crate::module::{
    Build, Cx, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, VariantDef,
};
use crate::GameState;

pub static DEF: ModuleDef = ModuleDef {
    id: "victory",
    summary: "Decides when the game ends and whether the team won.",
    variants: &[VariantDef {
        name: "survive",
        summary: "Lose as soon as a country holds `overrun` or more blight; win at the end of round `rounds`.",
        params: &[
            ParamSpec { name: "rounds", kind: ParamKind::Int, min: 1, max: 100, doc: "Rounds to survive." },
            ParamSpec { name: "overrun", kind: ParamKind::Int, min: 1, max: 1000, doc: "Blight in one country that loses the game." },
        ],
        phases: &[PhaseSpec { name: "check", kind: PhaseKind::Auto, doc: "Check for a loss, then for a win." }],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

#[derive(Debug)]
struct Survive {
    rounds: i64,
    overrun: i64,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Survive {
        rounds: b.int("rounds")?,
        overrun: b.int("overrun")?,
    }))
}

impl Module for Survive {
    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for c in 0..g.seats.len() {
            let n = country_tokens(g, &w, c, BLIGHT);
            if n >= self.overrun {
                let detail = format!("country {c} holds {n} blight (overrun at {})", self.overrun);
                cx.end(g, false, "overrun", detail);
                return;
            }
        }
        if g.round as i64 >= self.rounds {
            let detail = format!("survived {} rounds", self.rounds);
            cx.end(g, true, "survived", detail);
        }
    }
}
