use std::collections::BTreeSet;

use rand::{RngCore, SeedableRng};

use super::*;
use crate::content::HexKind;
use crate::modules::adversary::{CURRENT, DECK, NEXT, STAGE, SUPPLY};
use crate::modules::common::{
    ambassador, cube, presence, presence_of, ADV_AMBASSADOR, ADV_CUBE, AMBASSADORS, FACTORY,
    HARDEN, STATION, TROOP, UNIT,
};

mod adversary;
mod alignment;
mod board;
mod cards;
mod config;
mod determinism;
mod freeing;
mod invariants;
mod map;
mod plan;
mod projects;
mod setup;
mod threats;
mod view;

const BASELINE: &str = include_str!("../../../../configs/baseline.json");
const SHARED_ROW: &str = include_str!("../../../../configs/shared_row.json");

/// The action phase of the baseline round.
const ACTIONS: &str = "cards.play+threats.counter+economy.give+alignment.act+board.act+ai.power";

fn baseline() -> Config {
    Config::from_json(BASELINE).unwrap()
}

/// The baseline with overrides.
fn baseline_with(sets: &[&str]) -> Config {
    let mut c = baseline();
    for o in sets {
        c.apply_override(o).unwrap();
    }
    c
}

/// The baseline playing only `round` (give the action phase too), with
/// buckets too roomy to tip, the clear-board win held back to stage III and
/// no outright Unity win: a crafted state stays put until the test acts.
fn lab_config(round: &[&str]) -> Config {
    let mut c = baseline_with(&[
        "alignment.tip_per_player=50",
        "alignment.undecided_per_player=50",
        "alignment.clear_win_stage=3",
        "alignment.unity_win=0",
        "alignment.purge_round=0",
        // 021: no credits, so no neutral city's energy is bought unasked.
        "ai.start_credits=0",
    ]);
    c.round = round.iter().map(|s| s.to_string()).collect();
    c
}

/// Rules playing `phases` then the action phase, and a game of them at the
/// action phase of round 1 with the board emptied (no tokens anywhere).
fn lab(phases: &[&str]) -> (Rules, GameState) {
    lab_with(phases, &[])
}

/// `lab` with overrides on top of the lab config.
fn lab_with(phases: &[&str], sets: &[&str]) -> (Rules, GameState) {
    let mut round = phases.to_vec();
    round.push(ACTIONS);
    let mut c = lab_config(&round);
    for o in sets {
        c.apply_override(o).unwrap();
    }
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(1);
    assert!(g.pending.is_none() && !g.is_over());
    for h in g.hexes.iter_mut() {
        h.tokens.clear();
    }
    (rules, g)
}

/// Rules playing only the action phase (the lab config), and a game of
/// them after setup, the board as set up.
fn lab_actions() -> (Rules, GameState) {
    let rules = Rules::new(&lab_config(&[ACTIONS])).unwrap();
    let g = rules.new_game(1);
    (rules, g)
}

/// Runs the round's phases again from the first, up to the next decision
/// (or the first choice).
fn run(rules: &Rules, g: &mut GameState) {
    g.phase = 0;
    rules.enter(g, None);
}

fn put(g: &mut GameState, h: usize, kind: &str, n: i64) {
    if n == 0 {
        g.hexes[h].tokens.remove(kind);
    } else {
        g.hexes[h].tokens.insert(kind.to_string(), n);
    }
}

fn total(g: &GameState, kind: &str) -> i64 {
    (0..g.hexes.len()).map(|h| g.tokens(h, kind)).sum()
}

fn set_res(g: &mut GameState, seat: usize, r: &str, n: i64) {
    g.seats[seat].resources.insert(r.to_string(), n);
}

fn hexes_where(m: &Map, f: impl Fn(usize) -> bool) -> Vec<usize> {
    (0..m.hexes.len()).filter(|&h| f(h)).collect()
}

fn kind(m: &Map, h: usize) -> HexKind {
    m.hexes[h].kind
}

fn cities(m: &Map) -> Vec<usize> {
    hexes_where(m, |h| kind(m, h) == HexKind::City)
}

/// Corrupts every city but `keep`.
fn corrupt_cities_except(m: &Map, g: &mut GameState, keep: &[usize]) {
    for h in cities(m) {
        if !keep.contains(&h) {
            put(g, h, ADV_CUBE, 1);
        }
    }
}

fn piece(c: usize) -> &'static content::CountryPiece {
    &crate::content().map.countries[c]
}

/// Answers the open choice with option `i`.
fn answer(rules: &Rules, g: &mut GameState, i: usize) {
    let p = g.pending.as_ref().expect("a choice");
    let o = p.ask.options[i].clone();
    let seat = p.ask.seat;
    rules
        .apply_mut(
            g,
            &Action::Choose {
                seat,
                option: o.option,
                target: o.target,
            },
            None,
        )
        .unwrap();
}

/// Answers the open choice with the option labelled `label`.
fn answer_with(rules: &Rules, g: &mut GameState, label: &str) {
    let p = g.pending.as_ref().expect("a choice");
    let i = p.ask.options.iter().position(|o| o.option == label);
    let i = i.unwrap_or_else(|| panic!("{label:?} is not an option of {:?}", p.ask));
    answer(rules, g, i);
}

/// Answers the open choice with the option naming hex `h`.
fn answer_hex(rules: &Rules, g: &mut GameState, h: usize) {
    let at = rules.map.hexes[h].hex;
    let p = g.pending.as_ref().expect("a choice");
    let i = p.ask.options.iter().position(|o| o.target == Some(at));
    let i = i.unwrap_or_else(|| panic!("hex {h} is not an option of {:?}", p.ask));
    answer(rules, g, i);
}

/// Answers every choice with its first option.
fn answer_all(rules: &Rules, g: &mut GameState) {
    while g.pending.is_some() {
        answer(rules, g, 0);
    }
}

/// Applies `a` and answers whatever it asks with first options.
fn act(rules: &Rules, g: &mut GameState, a: Action) {
    rules
        .apply_mut(g, &a, None)
        .unwrap_or_else(|e| panic!("{a:?}: {e}"));
    answer_all(rules, g);
}

fn std_act(seat: u8, act: &str) -> Action {
    Action::Standard {
        seat,
        act: act.into(),
    }
}

/// A Cx for calling module functions directly on a crafted state.
fn with_cx<T>(rules: &Rules, f: impl FnOnce(&mut module::Cx) -> T) -> T {
    let mut choices = module::Choices::new(Vec::new());
    let mut cx = module::Cx {
        world: rules.world(),
        module: "test",
        purposes: &[],
        events: None,
        choices: &mut choices,
    };
    f(&mut cx)
}

/// Plays first legal actions until `phase` is the current decision.
fn advance_to(rules: &Rules, g: &mut GameState, phase: &str) {
    for _ in 0..10_000 {
        if g.pending.is_none() && rules.phase(g).is_some_and(|p| p.name == phase) {
            return;
        }
        let a = rules.legal_actions(g)[0].clone();
        rules.apply_mut(g, &a, None).unwrap();
    }
    panic!("never reached {phase}");
}

fn random_game(rules: &Rules, seed: u64) -> (GameState, Vec<Action>) {
    struct Rand(rand_chacha::ChaCha8Rng);
    impl Agent for Rand {
        fn choose(&mut self, _: &Rules, _: &GameState, legal: &[Action]) -> usize {
            (self.0.next_u32() as usize) % legal.len()
        }
    }
    let mut agent = Rand(rand_chacha::ChaCha8Rng::seed_from_u64(seed ^ 0x5eed));
    let (g, rec) = play(rules, seed, &mut agent, 100_000);
    (g, rec.actions)
}

/// Threat variants, Alignment variants, difficulty 0 and 4, the
/// draft variants and 1-4 countries.
fn variant_matrix() -> Vec<Config> {
    let sets: &[&[&str]] = &[
        &[],
        &["threats.variant=shared_row"],
        &["threats.variant=off"],
        &["alignment.variant=still"],
        &["alignment.variant=off"],
        &[
            "alignment.momentum_every=2",
            "alignment.antimatter_discord=0",
        ],
        &["adversary.difficulty=4"],
        &[
            "adversary.difficulty=4",
            "adversary.counter_bonus_all=1",
            "threats.variant=shared_row",
        ],
        &["cards.variant=off"],
        &["board.clock=3"],
        &["countries=1"],
        &["countries=3"],
        &["countries=4"],
        &[
            "countries=3",
            "adversary.difficulty=4",
            "alignment.variant=still",
        ],
    ];
    sets.iter()
        .map(|s| {
            let c = baseline_with(s);
            Rules::new(&c).unwrap_or_else(|e| panic!("{s:?}: {e}"));
            c
        })
        .collect()
}

// ---------- the golden game ----------

const GOLDEN_FINGERPRINT: &str = "0011fb28bdb81e65";

/// A fixed game (baseline, seed u64::MAX; action `k` is legal action
/// `(k * 7919) % count`: "always the first" can pay and withdraw forever).
/// `scripts/wasm-smoke.mjs` checks the wasm build reaches the same state.
/// If this fails after an intended rules change, bump `RULES_REVISION`
/// and update both fingerprints.
#[test]
fn golden_game() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(u64::MAX);
    for k in 0usize.. {
        let legal = rules.legal_actions(&g);
        if legal.is_empty() {
            break;
        }
        let a = legal[k * 7919 % legal.len()].clone();
        rules.apply_mut(&mut g, &a, None).unwrap();
    }
    assert!(g.is_over());
    assert_eq!(
        (RULES_REVISION, g.fingerprint().as_str()),
        (15, GOLDEN_FINGERPRINT)
    );
}
