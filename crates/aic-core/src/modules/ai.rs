//! AI boards: which AI each seat plays, its two processor tracks, processor
//! placement, tier-2 unlock, starting resources and growth.

use crate::action::Action;
use crate::content::{AiBoard, Track};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, PhaseSpec, Res, VariantDef,
    World,
};
use crate::state::GameState;
use crate::view::{BoardView, CardView, InnateView, TrackMark, TrackStep, TrackView, ViewOut};

use super::common::*;
use super::{cards, effects};

pub static DEF: ModuleDef = ModuleDef {
    id: "ai",
    summary: "AI boards: tracks, processors, growth, starting resources",
    requires: &["cards", "adversary", "alignment", "threats"],
    variants: &[VariantDef {
        name: "boards",
        summary: "Each seat plays an AI board from content/ais.json",
        params: &[
            ParamSpec {
                name: "boards",
                kind: ParamKind::List,
                min: 0,
                max: 99,
                doc: "AI board (index into content/ais.json) of each seat; the list repeats for more seats",
            },
            ParamSpec {
                name: "start_credits",
                kind: ParamKind::Int,
                min: 0,
                max: 100,
                doc: "credits each player starts with",
            },
            ParamSpec {
                name: "start_processors",
                kind: ParamKind::Int,
                min: 0,
                max: 5,
                doc: "processors each player starts with on a data centre in their starting country",
            },
        ],
        phases: &[PhaseSpec {
            name: "growth",
            kind: PhaseKind::Decision,
            doc: "each player picks one growth option on their board",
        }],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// The AI board seat `s` plays.
pub fn board<'w>(w: &World<'w>, seat: usize) -> &'w AiBoard {
    let list = w.list("ai.boards");
    &w.content.ais[list[seat % list.len()] as usize]
}

fn track_key(seat: usize, t: usize) -> String {
    format!("ai.track.{seat}.{t}")
}

/// Processors taken off track `t` of seat `s` so far.
pub fn track_pos(g: &GameState, seat: usize, t: usize) -> usize {
    g.var(&track_key(seat, t)) as usize
}

/// A track's current value: its highest uncovered entry.
pub fn track_value<'w>(w: &World<'w>, g: &GameState, seat: usize, t: usize) -> (&'w Track, usize) {
    let tr = &board(w, seat).tracks[t];
    (tr, track_pos(g, seat, t).min(tr.values.len() - 1))
}

/// Income from the AI tracks: `(resource, amount)` pairs, antimatter included.
pub fn track_income(w: &World, g: &GameState, seat: usize) -> Vec<(String, i64)> {
    let mut out = Vec::new();
    for t in 0..2 {
        let (tr, p) = track_value(w, g, seat, t);
        out.push((tr.resource.clone(), tr.values[p].value));
        if tr.values[p].antimatter > 0 {
            out.push(("antimatter".to_string(), tr.values[p].antimatter));
        }
    }
    out
}

/// Whether seat `s` has unlocked tier 2.
pub fn tier2(w: &World, g: &GameState, seat: usize) -> bool {
    (0..2).any(|t| {
        let (tr, p) = track_value(w, g, seat, t);
        tr.values[..=p].iter().any(|v| v.tier2)
    })
}

/// Places one processor for `seat`: off the left end of a track (players
/// choose which), onto a hex within 1 of one of their processors or onto a
/// data centre in a country where they have one. Returns whether placed.
pub fn place_processor(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res<bool> {
    let w = cx.world;
    let b = board(&w, seat);
    let tracks: Vec<usize> = (0..2)
        .filter(|&t| track_pos(g, seat, t) + 1 < b.tracks[t].values.len())
        .collect();
    let opts = tracks
        .iter()
        .map(|&t| {
            let tr = &b.tracks[t];
            let next = &tr.values[track_pos(g, seat, t) + 1];
            opt(format!("{} track -> {}", tr.resource, next.value))
        })
        .collect();
    let prompt = format!("Seat {seat}: take a processor from which track?");
    let Some(i) = cx.choose(seat, &prompt, opts)? else {
        return Ok(false);
    };
    let t = tracks[i];
    let hexes = processor_hexes(&w, g, seat);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!("Seat {seat}: place the processor on which hex?");
    let Some(j) = cx.choose(seat, &prompt, opts)? else {
        return Ok(false);
    };
    cx.add_key(g, track_key(seat, t), 1);
    cx.add_tokens(g, hexes[j], &processor(seat), 1);
    cx.tally(g, "ai.processors_placed", 1);
    Ok(true)
}

/// Where seat `s` may place a processor.
pub fn processor_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    let mine = processor(seat);
    let own: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, &mine) > 0).collect();
    (0..g.hexes.len())
        .filter(|&h| {
            own.iter().any(|&p| w.map.distance(p, h) <= 1)
                || (g.tokens(h, DATA_CENTRE) > 0
                    && has_processor_in(w, g, seat, w.map.hexes[h].country as usize))
        })
        .collect()
}

/// Whether seat `s` has a processor left on a track.
pub fn can_place(w: &World, g: &GameState, seat: usize) -> bool {
    let b = board(w, seat);
    (0..2).any(|t| track_pos(g, seat, t) + 1 < b.tracks[t].values.len())
}

#[derive(Debug)]
struct Ai {
    start_credits: i64,
    start_processors: i64,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let boards = b.list("boards")?;
    if let Some(&x) = boards.iter().find(|&&x| x as usize >= b.world.content.ais.len()) {
        return Err(format!("boards: no AI board {x} (content/ais.json has {})", b.world.content.ais.len()));
    }
    Ok(Box::new(Ai {
        start_credits: b.int("start_credits")?,
        start_processors: b.int("start_processors")?,
    }))
}

impl Module for Ai {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for s in 0..g.seats.len() {
            cx.add_resource(g, s, "credits", self.start_credits);
            // The starting country only says where the first processor goes.
            let c = s % w.map.countries.len();
            let piece = &w.content.map.countries[c];
            if let Some(&n) = piece.data_centres.iter().min() {
                cx.add_tokens(g, w.map.numbered(c, n), &processor(s), self.start_processors);
            }
        }
    }

    fn legal(&self, _phase: usize, _g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        for option in 0..board(w, seat as usize).growth.len() {
            out.push(Action::Growth {
                seat,
                option: option as u8,
            });
        }
    }

    fn can_pass(&self, _phase: usize, _g: &GameState, _seat: u8) -> bool {
        false
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Growth { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let Action::Growth { seat, option } = a else {
            return Err(Halt::Illegal("not a growth".into()));
        };
        let s = *seat as usize;
        let growth = board(&w, s)
            .growth
            .get(*option as usize)
            .ok_or("no such growth option")?;
        cx.count(g, &format!("growth.{}.{option}", board(&w, s).id), 1);
        effects::run(g, cx, s, &growth.effects, &mut effects::Ctx::default())?;
        g.seats[s].passed = true;
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        // The track positions are shown on the board below.
        out.bookkeeping(&["ai.track."]);
        for s in 0..g.seats.len() {
            let b = board(w, s);
            let tracks = (0..b.tracks.len().min(2))
                .map(|t| {
                    let (tr, p) = track_value(w, g, s, t);
                    TrackView {
                        label: format!("{} track", tr.resource),
                        resource: tr.resource.clone(),
                        steps: tr
                            .values
                            .iter()
                            .map(|v| {
                                let mut marks = Vec::new();
                                if v.tier2 {
                                    marks.push(TrackMark {
                                        key: "tier2".into(),
                                        label: "T2".into(),
                                        doc: "uncovering this unlocks tier 2".into(),
                                    });
                                }
                                if v.antimatter > 0 {
                                    marks.push(TrackMark {
                                        key: "antimatter".into(),
                                        label: format!("+{} AM", v.antimatter),
                                        doc: format!("{} antimatter income while this is the current value", v.antimatter),
                                    });
                                }
                                TrackStep { value: v.value, marks }
                            })
                            .collect(),
                        uncovered: p + 1,
                        current: p,
                    }
                })
                .collect();
            let (hand, built) = (cards::hand(s), cards::built(s));
            let unique = b
                .cards
                .iter()
                .map(|id| {
                    let status = if g.pile(&hand).contains(id) {
                        "in hand"
                    } else if g.pile(&built).contains(id) {
                        "built"
                    } else {
                        "gone"
                    };
                    let (name, text) = match w.content.card_index(id) {
                        Some(i) => (w.content.cards[i].name.clone(), w.content.cards[i].text.clone()),
                        None => (id.clone(), String::new()),
                    };
                    CardView {
                        id: id.clone(),
                        name,
                        text,
                        status: status.into(),
                        cost: None,
                    }
                })
                .collect();
            out.board(
                s,
                "board",
                BoardView {
                    key: String::new(),
                    source: String::new(),
                    id: b.id.clone(),
                    name: b.name.clone(),
                    text: b.text.clone(),
                    innate: Some(InnateView {
                        name: b.innate.name.clone(),
                        text: b.innate.text.clone(),
                    }),
                    tracks,
                    slots: Some(b.slots),
                    cards: unique,
                },
            );
            out.seat(
                s,
                "tier2",
                "tier 2",
                tier2(w, g, s) as i64,
                None,
                "1 once this AI has unlocked tier 2",
            );
            out.seat(
                s,
                "processors",
                "processors",
                processors_of(g, s),
                None,
                "processors on the board",
            );
        }
    }
}
