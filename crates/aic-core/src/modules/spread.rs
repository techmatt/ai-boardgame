//! Placeholder adversary: blight tokens spread onto every country each round.

use super::BLIGHT;
use crate::module::{
    scheduled, Build, Cx, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec,
    VariantDef, World,
};
use crate::view::ViewOut;
use crate::GameState;

const RNG: &str = "spread";

const START: ParamSpec = ParamSpec {
    name: "start",
    kind: ParamKind::Int,
    min: 0,
    max: 64,
    doc: "Blight per country at setup, placed by the variant's rule.",
};
const PER_COUNTRY: ParamSpec = ParamSpec {
    name: "per_country",
    kind: ParamKind::List,
    min: 0,
    max: 64,
    doc: "Blight added to each country per round, by round; the last entry repeats.",
};

pub static DEF: ModuleDef = ModuleDef {
    id: "spread",
    summary: "The adversary adds blight to every country each round.",
    variants: &[
        VariantDef {
            name: "random",
            summary: "Each new blight lands on a uniformly random hex of the country.",
            params: &[START, PER_COUNTRY],
            phases: PHASES,
            rng: &[RNG],
            uses_cards: false,
            build: build_random,
        },
        VariantDef {
            name: "cluster",
            summary: "Each new blight lands on the country's most-blighted hex still below `cap` \
                      (ties: lowest piece position); on a random hex if the country is clean.",
            params: &[
                START,
                PER_COUNTRY,
                ParamSpec {
                    name: "cap",
                    kind: ParamKind::Int,
                    min: 1,
                    max: 64,
                    doc: "Most blight one hex can hold.",
                },
            ],
            phases: PHASES,
            rng: &[RNG],
            uses_cards: false,
            build: build_cluster,
        },
    ],
};

const PHASES: &[PhaseSpec] = &[PhaseSpec {
    name: "advance",
    kind: PhaseKind::Auto,
    doc: "The adversary places blight.",
}];

#[derive(Debug)]
struct Spread {
    start: i64,
    per_country: Vec<i64>,
    /// `None`: random placement. `Some(cap)`: cluster placement.
    cluster_cap: Option<i64>,
}

fn build_random(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Spread {
        start: b.int("start")?,
        per_country: b.list("per_country")?,
        cluster_cap: None,
    }))
}

fn build_cluster(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Spread {
        start: b.int("start")?,
        per_country: b.list("per_country")?,
        cluster_cap: Some(b.int("cap")?),
    }))
}

impl Spread {
    fn random_hex(&self, g: &mut GameState, cx: &Cx, country: usize) -> usize {
        let hexes = &cx.world.map.countries[country].hexes;
        hexes[cx.rng(g, RNG).below(hexes.len())]
    }

    /// Where the next blight goes in `country`, or `None` if it has no room.
    fn target(&self, g: &mut GameState, cx: &Cx, country: usize) -> Option<usize> {
        let Some(cap) = self.cluster_cap else {
            return Some(self.random_hex(g, cx, country));
        };
        let hexes = &cx.world.map.countries[country].hexes;
        let mut best: Option<(i64, usize)> = None;
        for &h in hexes {
            let n = g.tokens(h, BLIGHT);
            if n > 0 && n < cap && best.is_none_or(|(b, _)| n > b) {
                best = Some((n, h));
            }
        }
        if let Some((_, h)) = best {
            return Some(h);
        }
        if hexes.iter().all(|&h| g.tokens(h, BLIGHT) == 0) {
            return Some(self.random_hex(g, cx, country));
        }
        // Every blighted hex is full: start a new cluster next to the first one.
        let full = hexes
            .iter()
            .copied()
            .find(|&h| g.tokens(h, BLIGHT) >= cap)?;
        let map = cx.world.map;
        map.hexes[full]
            .neighbours
            .iter()
            .copied()
            .find(|&n| map.hexes[n].country as usize == country && g.tokens(n, BLIGHT) == 0)
            .or_else(|| hexes.iter().copied().find(|&h| g.tokens(h, BLIGHT) == 0))
    }

    fn place(&self, g: &mut GameState, cx: &mut Cx, country: usize, n: i64) {
        for _ in 0..n {
            match self.target(g, cx, country) {
                Some(h) => {
                    cx.add_tokens(g, h, BLIGHT, 1);
                    cx.adversary_ops(g, 1);
                    cx.count(g, "placed", 1);
                }
                None => cx.count(g, "overflow", 1),
            }
        }
    }
}

impl Module for Spread {
    fn view(&self, g: &GameState, _w: &World, out: &mut ViewOut) {
        let n = scheduled(&self.per_country, g.round);
        out.global(
            "per_country",
            "spread this round",
            n,
            None,
            "Blight each country gets in this round's spread.",
        );
        if let Some(cap) = self.cluster_cap {
            out.global(
                "cap",
                "blight cap per hex",
                cap,
                None,
                "Most blight one hex can hold (cluster).",
            );
        }
    }

    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        // Same placement rule as every round, so `cap` holds from the start.
        for c in 0..g.seats.len() {
            self.place(g, cx, c, self.start);
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) {
        let n = scheduled(&self.per_country, g.round);
        for c in 0..g.seats.len() {
            self.place(g, cx, c, n);
        }
    }
}
