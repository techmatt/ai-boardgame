//! Invariants checked after every action of many random games (020's bug
//! hunt): what a printed table can't hold (negative piles, two presence of one
//! seat on a hex, pieces on the ocean, more presence out than taken off the
//! track), a game that waits for nobody, a state the rules reject, a view
//! that can't be drawn.

use super::*;
use crate::modules::common::NODE;
use crate::modules::{ai, board};

/// Random play that mostly acts: a pass only now and then, so games reach the
/// late rounds (uniform random play passes away most of its turns).
struct Busy(rand_chacha::ChaCha8Rng);

impl Agent for Busy {
    fn choose(&mut self, _: &Rules, _: &GameState, legal: &[Action]) -> usize {
        let acting: Vec<usize> = (0..legal.len())
            .filter(|&i| !matches!(legal[i], Action::Pass { .. }))
            .collect();
        let r = self.0.next_u32() as usize;
        if !acting.is_empty() && r % 100 < 85 {
            acting[(r / 100) % acting.len()]
        } else {
            r % legal.len()
        }
    }
}

/// Every broken invariant of `g`, as text.
fn broken(rules: &Rules, g: &GameState) -> Vec<String> {
    let w = rules.world();
    let mut out = Vec::new();
    if let Err(e) = rules.check_state(g) {
        out.push(format!("check_state: {e}"));
    }
    if !g.is_over() && rules.legal_actions(g).is_empty() {
        out.push("no legal action and no outcome".into());
    }
    for (h, hex) in g.hexes.iter().enumerate() {
        for (k, &n) in &hex.tokens {
            if n < 0 {
                out.push(format!("hex {h}: {n} {k}"));
            }
            if n != 0 && w.map.hexes[h].kind == HexKind::Ocean {
                out.push(format!("hex {h} is ocean but holds {n} {k}"));
            }
        }
        for s in 0..g.seats.len() {
            if g.tokens(h, &presence(s)) > 1 {
                out.push(format!("hex {h}: {} presence of seat {s}", g.tokens(h, &presence(s))));
            }
        }
        if g.tokens(h, NODE) > 1 {
            out.push(format!("hex {h}: {} nodes", g.tokens(h, NODE)));
        }
    }
    for (s, seat) in g.seats.iter().enumerate() {
        for (r, &n) in &seat.resources {
            if n < 0 {
                out.push(format!("seat {s}: {n} {r}"));
            }
        }
        let track = ai::board(&w, s).presence.len();
        let taken = board::track_taken(g, s);
        let placed = total(g, &presence(s)) + g.resource(s, crate::modules::common::RESERVE);
        if taken > track {
            out.push(format!("seat {s}: {taken} presence taken off a track of {track}"));
        }
        if placed > taken as i64 {
            out.push(format!("seat {s}: {placed} presence out (board + reserve) but {taken} taken off the track"));
        }
    }
    if g.var(SUPPLY) < 0 {
        out.push(format!("adversary supply {}", g.var(SUPPLY)));
    }
    if g.var("adversary.compute") < 0 {
        out.push(format!("adversary compute {}", g.var("adversary.compute")));
    }
    // Words players read carry no raw token keys (020: "presence:3 at C3 #3").
    // ... nor the engine's old hex names ("C1 #5"; 020: the piece's name now).
    let index_name = |s: &str| {
        s.match_indices(" #").any(|(i, _)| {
            let head = &s[..i];
            let digits = head.trim_end_matches(|c: char| c.is_ascii_digit());
            digits.len() < head.len() && digits.ends_with('C')
                && digits[..digits.len() - 1].chars().last().is_none_or(|c| !c.is_alphanumeric())
        })
    };
    let raw_key = |s: &str| {
        ["presence:", "cube:", "ambassador:"]
            .iter()
            .any(|k| s.contains(k))
            || index_name(s)
    };
    if let Some(p) = &g.pending {
        for text in std::iter::once(&p.ask.prompt).chain(p.ask.options.iter().map(|o| &o.option)) {
            if raw_key(text) {
                out.push(format!("raw token key in a choice: {text:?}"));
            }
        }
    }
    // The view is what clients draw: building it must not panic.
    let v = rules.view(g);
    if v.hexes.len() != g.hexes.len() {
        out.push("view hex count".into());
    }
    // Labels are words, not keys (020: `presence_reserve`).
    let labels = v
        .globals
        .iter()
        .chain(v.seats.iter().flat_map(|s| s.values.iter()))
        .chain(v.countries.iter().flat_map(|c| c.values.iter()));
    for val in labels {
        if val.label.contains('_') || raw_key(&val.label) {
            out.push(format!("view label {:?} for {}", val.label, val.key));
        }
    }
    out
}

fn check_games(configs: &[Config], seeds: std::ops::Range<u64>) {
    let mut failures = Vec::new();
    for config in configs {
        let rules = Rules::new(config).unwrap();
        for seed in seeds.clone() {
            let mut agent = Busy(rand_chacha::ChaCha8Rng::seed_from_u64(seed ^ 0xb57));
            let mut g = rules.new_game(seed);
            let mut step = 0;
            while !g.is_over() && step < 20_000 {
                for b in broken(&rules, &g) {
                    failures.push(format!("{} seed {seed} step {step} round {}: {b}", config.hash(), g.round));
                }
                if failures.len() > 20 {
                    break;
                }
                let legal = rules.legal_actions(&g);
                if legal.is_empty() {
                    break;
                }
                let a = legal[agent.choose(&rules, &g, &legal)].clone();
                rules.apply_mut(&mut g, &a, None).unwrap();
                step += 1;
            }
            for b in broken(&rules, &g) {
                failures.push(format!("{} seed {seed} at the end: {b}", config.hash()));
            }
            if !g.is_over() {
                failures.push(format!("{} seed {seed}: not over after {step} actions", config.hash()));
            }
        }
    }
    assert!(failures.is_empty(), "{} broken invariants:\n{}", failures.len(), failures.join("\n"));
}

/// Fast lane: one game of the baseline and one of a short variant (~1 s); the
/// slow lane plays every variant, 3 and 4 countries included.
#[test]
fn invariants_hold_in_random_games() {
    let configs: Vec<Config> = [&[][..], &["countries=1", "adversary.difficulty=4"]]
        .iter()
        .map(|s| baseline_with(s))
        .collect();
    check_games(&configs, 0..1);
}

#[test]
#[ignore = "slow lane: more seeds of the invariant check, every variant"]
fn invariants_hold_in_random_games_every_variant() {
    // One seed a variant (~14 s): the slow lane's slowest aic-core test.
    check_games(&variant_matrix(), 2..3);
}
