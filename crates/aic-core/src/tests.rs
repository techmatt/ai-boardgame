use rand::{RngCore, SeedableRng};

use super::*;

const BASELINE: &str = include_str!("../../../configs/baseline.json");
const CLEAN_LAND: &str = include_str!("../../../configs/clean_land.json");

fn baseline() -> Config {
    Config::from_json(BASELINE).unwrap()
}

/// Every combination of the two placeholder modules' variants.
fn variant_matrix() -> Vec<Config> {
    let mut out = Vec::new();
    let incomes: [&[&str]; 2] = [
        &[],
        &[
            "income.variant=clean_land",
            "income.base=1",
            "income.per_clean=4",
        ],
    ];
    let spreads: [&[&str]; 2] = [&[], &["spread.variant=cluster", "spread.cap=4"]];
    for income in incomes {
        for spread in spreads {
            let mut c = baseline();
            for o in income.iter().chain(spread) {
                c.apply_override(o).unwrap();
            }
            Rules::new(&c).unwrap();
            out.push(c);
        }
    }
    out
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

fn hexes(list: &[[i32; 2]]) -> Vec<Hex> {
    let mut v: Vec<Hex> = list.iter().map(|&h| Hex::from(h)).collect();
    v.sort();
    v
}

/// The hex sets read off `ai-complete-scratch/{3,4,5}country.png` (origin:
/// centre hex of the middle row of the reference grid).
#[test]
fn layouts_match_the_reference_images() {
    let beige = hexes(&[
        [-4, 3],
        [-4, 4],
        [-4, 5],
        [-4, 6],
        [-3, 3],
        [-3, 4],
        [-3, 5],
        [-3, 6],
        [-2, 3],
        [-2, 4],
        [-2, 5],
        [-2, 6],
        [-1, 3],
        [-1, 4],
        [-1, 5],
        [-1, 6],
    ]);
    let pink = hexes(&[
        [0, 3],
        [0, 4],
        [0, 5],
        [0, 6],
        [1, 2],
        [1, 3],
        [1, 4],
        [1, 5],
        [2, 1],
        [2, 2],
        [2, 3],
        [2, 4],
        [3, 0],
        [3, 1],
        [3, 2],
        [3, 3],
    ]);
    let blue3 = hexes(&[
        [-3, 2],
        [-2, 1],
        [-2, 2],
        [-1, 0],
        [-1, 1],
        [-1, 2],
        [0, -1],
        [0, 0],
        [0, 1],
        [0, 2],
        [1, -1],
        [1, 0],
        [1, 1],
        [2, -1],
        [2, 0],
        [3, -1],
    ]);
    let blue = hexes(&[
        [0, -1],
        [0, 0],
        [0, 1],
        [0, 2],
        [1, -2],
        [1, -1],
        [1, 0],
        [1, 1],
        [2, -3],
        [2, -2],
        [2, -1],
        [2, 0],
        [3, -4],
        [3, -3],
        [3, -2],
        [3, -1],
    ]);
    let green = hexes(&[
        [-4, -1],
        [-4, 0],
        [-4, 1],
        [-4, 2],
        [-3, -1],
        [-3, 0],
        [-3, 1],
        [-3, 2],
        [-2, -1],
        [-2, 0],
        [-2, 1],
        [-2, 2],
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [-1, 2],
    ]);
    let orange = hexes(&[
        [-3, -2],
        [-2, -3],
        [-2, -2],
        [-1, -4],
        [-1, -3],
        [-1, -2],
        [0, -5],
        [0, -4],
        [0, -3],
        [0, -2],
        [1, -5],
        [1, -4],
        [1, -3],
        [2, -5],
        [2, -4],
        [3, -5],
    ]);
    let expect = [
        (3, vec![&beige, &pink, &blue3]),
        (4, vec![&beige, &pink, &blue, &green]),
        (5, vec![&beige, &pink, &blue, &green, &orange]),
    ];
    for (n, countries) in expect {
        let map = Map::build(content(), n).unwrap();
        assert_eq!(map.hexes.len(), 16 * n as usize);
        for (c, want) in countries.iter().enumerate() {
            let mut got: Vec<Hex> = map.countries[c]
                .hexes
                .iter()
                .map(|&h| map.hexes[h].hex)
                .collect();
            got.sort();
            assert_eq!(&got, *want, "{n} countries, piece {c}");
        }
    }
    for n in 1..=5 {
        let map = Map::build(content(), n).unwrap();
        // Every country touches another (except a lone one).
        for c in 0..n as usize {
            let touches = map.countries[c].hexes.iter().any(|&h| {
                map.hexes[h]
                    .neighbours
                    .iter()
                    .any(|&o| map.country_of(o) as usize != c)
            });
            assert_eq!(touches, n > 1, "{n} countries, piece {c}");
        }
    }
}

#[test]
fn same_seed_same_game() {
    let rules = Rules::new(&baseline()).unwrap();
    assert_eq!(rules.new_game(7), rules.new_game(7));
    assert_ne!(rules.new_game(7).hexes, rules.new_game(8).hexes);
}

#[test]
fn state_round_trips_through_json_with_string_seeds() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(u64::MAX - 3);
    let a = rules.legal_actions(&g)[0].clone();
    rules.apply_mut(&mut g, &a, None).unwrap();
    let json = g.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["seed"], serde_json::json!((u64::MAX - 3).to_string()));
    assert!(v["rng"]["spread"]["word_pos"].is_string());
    let back: GameState = serde_json::from_str(&json).unwrap();
    assert_eq!(g, back);
    for a in rules.legal_actions(&g) {
        let back: Action = serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
        assert_eq!(a, back);
    }
}

#[test]
fn random_games_replay_exactly_in_every_variant() {
    for config in variant_matrix() {
        let rules = Rules::new(&config).unwrap();
        for seed in 0..25 {
            let (g, actions) = random_game(&rules, seed);
            assert!(g.is_over(), "{} seed {seed} did not finish", config.hash());
            assert_eq!(replay(&rules, seed, &actions).unwrap(), g);
            // The slow, JSON-shaped path agrees with the hot path, and a
            // state survives serialization at any point.
            let mut s = new_game(&config, seed).unwrap();
            for (i, a) in actions.iter().enumerate() {
                if i % 7 == 0 {
                    s = serde_json::from_str(&s.to_json()).unwrap();
                }
                s = apply(&s, a).unwrap().0;
            }
            assert_eq!(s, g);
        }
    }
}

#[test]
fn simultaneous_actions_commute() {
    // In a decision phase no seat's action may change another seat's
    // options or effects: any two actions by different seats commute.
    for config in variant_matrix() {
        let rules = Rules::new(&config).unwrap();
        for seed in 0..10 {
            let (_, actions) = random_game(&rules, seed);
            let mut g = rules.new_game(seed);
            for a in actions.iter().take(40) {
                let legal = rules.legal_actions(&g);
                for x in &legal {
                    for y in &legal {
                        if x.seat() == y.seat() {
                            continue;
                        }
                        let mut xy = g.clone();
                        let mut yx = g.clone();
                        // Applying x must leave y legal (unless x ended the phase).
                        rules.apply_mut(&mut xy, x, None).unwrap();
                        if xy.phase != g.phase || xy.round != g.round {
                            continue;
                        }
                        assert!(rules.legal_actions(&xy).contains(y), "{x:?} removed {y:?}");
                        rules.apply_mut(&mut xy, y, None).unwrap();
                        rules.apply_mut(&mut yx, y, None).unwrap();
                        rules.apply_mut(&mut yx, x, None).unwrap();
                        // Everything but the event order is identical.
                        assert_eq!(xy, yx, "{x:?} and {y:?} do not commute");
                    }
                }
                rules.apply_mut(&mut g, a, None).unwrap();
            }
        }
    }
}

#[test]
fn rng_streams_do_not_depend_on_module_order() {
    let a = baseline();
    let mut b = baseline();
    b.modules.reverse();
    let (ra, rb) = (Rules::new(&a).unwrap(), Rules::new(&b).unwrap());
    assert_ne!(ra.config_hash, rb.config_hash);
    let (ga, actions) = random_game(&ra, 3);
    let gb = replay(&rb, 3, &actions).unwrap();
    assert_eq!(ga.hexes, gb.hexes);
    assert_eq!(ga.seats, gb.seats);
}

#[test]
fn config_hash_is_stable_and_ignores_the_name() {
    let rules = Rules::new(&baseline()).unwrap();
    // Resolving a resolved config changes nothing.
    let again = Rules::new(&rules.config).unwrap();
    assert_eq!(again.config, rules.config);
    assert_eq!(again.config_hash, rules.config_hash);
    let mut renamed = baseline();
    renamed.name = "other".into();
    assert_eq!(Rules::new(&renamed).unwrap().config_hash, rules.config_hash);
    // An omitted round resolves to module order, which differs here.
    let mut implicit = baseline();
    implicit.round.clear();
    let r = Rules::new(&implicit).unwrap();
    assert_eq!(
        r.config.round,
        vec![
            "income.collect",
            "actions.play",
            "spread.advance",
            "victory.check"
        ]
    );
    let mut tuned = baseline();
    tuned.apply_override("spread.start=3").unwrap();
    assert_ne!(Rules::new(&tuned).unwrap().config_hash, rules.config_hash);
}

#[test]
fn list_parameters_accept_a_single_integer() {
    let mut c = baseline();
    c.apply_override("spread.per_country=2").unwrap();
    let rules = Rules::new(&c).unwrap();
    assert_eq!(
        rules.config.module("spread").unwrap().params["per_country"],
        ParamValue::List(vec![2])
    );
    let mut c = baseline();
    c.apply_override("spread.per_country=1,2,3").unwrap();
    let rules = Rules::new(&c).unwrap();
    assert_eq!(
        rules.config.module("spread").unwrap().params["per_country"],
        ParamValue::List(vec![1, 2, 3])
    );
}

#[test]
fn bad_configs_are_rejected_with_a_reason() {
    let cases: &[(&str, &str)] = &[
        ("countries=6", "countries must be 1-5"),
        ("income.variant=nope", "unknown variant"),
        ("income.amont=3", "not a parameter"),
        ("income.amount=-1", "outside"),
        ("income.amount=1,2", "expected an integer"),
        ("spread.variant=cluster", "spread.cap: missing"),
    ];
    for (o, want) in cases {
        let mut c = baseline();
        c.apply_override(o).unwrap();
        let err = Rules::new(&c)
            .err()
            .unwrap_or_else(|| panic!("{o} was accepted"));
        assert!(err.contains(want), "{o}: {err}");
    }
    let mut c = baseline();
    c.modules[1].params.remove("scrub_cost");
    assert!(Rules::new(&c)
        .err()
        .unwrap()
        .contains("actions.scrub_cost: missing"));
    let mut c = baseline();
    c.modules[1].cards.push("nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("unknown card"));
    let mut c = baseline();
    c.round.push("spread.nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("no phase"));
    let mut c = baseline();
    c.modules.push(c.modules[0].clone());
    assert!(Rules::new(&c).err().unwrap().contains("appears twice"));
    assert!(Config::from_json(r#"{"countries": 3, "modules": [], "extra": 1}"#).is_err());
}

#[test]
fn every_seat_must_pass_before_the_phase_ends() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(1);
    assert_eq!(g.round, 1);
    assert_eq!(rules.phase(&g).unwrap().name, "actions.play");
    rules
        .apply_mut(&mut g, &Action::Pass { seat: 1 }, None)
        .unwrap();
    assert!(rules
        .apply_mut(&mut g, &Action::Pass { seat: 1 }, None)
        .is_err());
    assert!(rules.legal_actions(&g).iter().all(|a| a.seat() != 1));
    rules
        .apply_mut(&mut g, &Action::Pass { seat: 0 }, None)
        .unwrap();
    assert_eq!(g.round, 1);
    let mut events = Vec::new();
    rules
        .apply_mut(&mut g, &Action::Pass { seat: 2 }, Some(&mut events))
        .unwrap();
    assert_eq!(g.round, 2);
    assert!(events.contains(&Event::RoundStarted { round: 2 }));
    assert!(events.contains(&Event::PhaseStarted {
        phase: "spread.advance".into()
    }));
}

#[test]
fn cards_cost_credits_are_used_once_and_never_offer_useless_targets() {
    let rules = Rules::new(&baseline()).unwrap();
    let g = rules.new_game(4);
    let (scrub, invest) = (String::from("scrub"), String::from("invest"));
    for a in rules.legal_actions(&g) {
        if let Action::Play {
            seat,
            target: Some(t),
            ..
        } = a
        {
            let h = rules.map.find(t).unwrap();
            assert_eq!(rules.map.country_of(h), seat);
            assert!(g.tokens(h, modules::BLIGHT) > 0);
        }
    }
    let play = Action::Play {
        seat: 0,
        card: invest,
        target: None,
    };
    let (g2, events) = rules.apply(&g, &play).unwrap();
    assert_eq!(
        g2.resource(0, modules::CREDITS),
        g.resource(0, modules::CREDITS) - 2 + 3
    );
    assert!(events
        .iter()
        .any(|e| matches!(e, Event::CardPlayed { seat: 0, .. })));
    assert!(rules.apply(&g2, &play).is_err());
    assert_eq!(g2.stats.counters["actions.played.invest"], 1);
    // Targeting a clean hex or another country's hex is rejected.
    let clean = rules.map.countries[0]
        .hexes
        .iter()
        .find(|&&h| g.tokens(h, modules::BLIGHT) == 0)
        .unwrap();
    let bad = Action::Play {
        seat: 0,
        card: scrub.clone(),
        target: Some(rules.map.hexes[*clean].hex),
    };
    assert!(rules.apply(&g, &bad).is_err());
    let theirs = rules.map.countries[1]
        .hexes
        .iter()
        .find(|&&h| g.tokens(h, modules::BLIGHT) > 0)
        .unwrap();
    let bad = Action::Play {
        seat: 0,
        card: scrub,
        target: Some(rules.map.hexes[*theirs].hex),
    };
    assert!(rules.apply(&g, &bad).is_err());
}

#[test]
fn games_end_with_a_round_and_a_reason_and_count_adversary_ops() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut saw = std::collections::BTreeSet::new();
    for seed in 0..60 {
        let (g, _) = random_game(&rules, seed);
        let o = g.outcome.clone().unwrap();
        assert!(o.round >= 1 && o.round <= 8);
        assert_eq!(o.round, g.round);
        assert_eq!(g.stats.adversary_ops.len(), g.round as usize);
        assert!(g.stats.adversary_ops.iter().all(|&n| n > 0));
        // spread.start = 2 blight in each of 3 countries.
        assert_eq!(g.stats.setup_ops, 6);
        saw.insert(o.reason);
    }
    assert!(
        saw.contains("survived") && saw.contains("overrun"),
        "{saw:?}"
    );
}

#[test]
fn a_round_without_an_ending_hits_the_round_limit() {
    let mut c = baseline();
    c.modules.retain(|m| m.module == "income");
    c.round.clear();
    let g = new_game(&c, 1).unwrap();
    let o = g.outcome.unwrap();
    assert_eq!(
        (o.reason.as_str(), o.round),
        ("round_limit", rules::MAX_ROUNDS)
    );
}

#[test]
fn the_comparison_configs_differ_only_in_a_variant_and_a_tunable() {
    let a = Config::from_json(BASELINE).unwrap();
    let b = Config::from_json(CLEAN_LAND).unwrap();
    let diff = a.diff(&b);
    assert_eq!(
        diff,
        vec![
            "income.variant: flat -> clean_land",
            "income.amount: 3 -> (unset)",
            "income.base: (unset) -> 1",
            "income.per_clean: (unset) -> 4",
            "spread.per_country: [1,2,2,3] -> [1,2,3,3]",
        ]
    );
}

#[test]
fn overrides_switch_variants_unset_and_remove_modules() {
    let mut c = baseline();
    c.apply_override("income.variant=clean_land").unwrap();
    // The old variant's parameters are dropped; the new ones must be set.
    assert!(c.module("income").unwrap().params.is_empty());
    assert!(Rules::new(&c)
        .err()
        .unwrap()
        .contains("income.base: missing"));
    c.apply_override("income.base=1").unwrap();
    c.apply_override("income.per_clean=4").unwrap();
    Rules::new(&c).unwrap();
    c.apply_override("income.base=").unwrap();
    assert!(!c.module("income").unwrap().params.contains_key("base"));

    let mut c = baseline();
    c.apply_override("spread=off").unwrap();
    assert!(c.module("spread").is_none());
    assert!(!c.round.iter().any(|p| p.starts_with("spread.")));
    let rules = Rules::new(&c).unwrap();
    let (g, _) = random_game(&rules, 1);
    assert_eq!(g.outcome.unwrap().reason, "survived");
}

/// Two card modules share one simultaneous phase via `a.play+b.play`.
#[test]
fn modules_can_share_a_decision_phase() {
    let mut c = baseline();
    c.modules[1].cards = vec!["scrub".into(), "invest".into()];
    c.modules.push(ModuleConfig {
        module: "twin_actions".into(),
        variant: "open_cards".into(),
        params: Default::default(),
        cards: vec!["purge".into()],
    });
    c.round = vec![
        "income.collect".into(),
        "actions.play+twin_actions.play".into(),
        "spread.advance".into(),
        "victory.check".into(),
    ];
    let rules = Rules::new(&c).unwrap();
    // Seat 0 has enough credits for purge only after a few rounds; check
    // routing directly instead.
    let mut g = rules.new_game(2);
    for s in g.seats.iter_mut() {
        s.resources.insert(modules::CREDITS.into(), 10);
    }
    let legal = rules.legal_actions(&g);
    let card = |a: &Action| match a {
        Action::Play { card, .. } => card.clone(),
        _ => String::new(),
    };
    assert!(legal.iter().any(|a| card(a) == "purge"));
    assert!(legal.iter().any(|a| card(a) == "invest"));
    let purge = legal.iter().find(|a| card(a) == "purge").unwrap().clone();
    rules.apply_mut(&mut g, &purge, None).unwrap();
    assert_eq!(g.stats.counters["twin_actions.played.purge"], 1);
    // Replays and commutes like any other phase.
    for seed in 0..10 {
        let (g, actions) = random_game(&rules, seed);
        assert_eq!(replay(&rules, seed, &actions).unwrap(), g);
    }
    // A card belongs to one module.
    let mut bad = c.clone();
    bad.modules.last_mut().unwrap().cards.push("scrub".into());
    assert!(Rules::new(&bad)
        .err()
        .unwrap()
        .contains("belongs to one module"));
    // Only decision phases combine.
    let mut bad = c.clone();
    bad.round[0] = "income.collect+spread.advance".into();
    assert!(Rules::new(&bad)
        .err()
        .unwrap()
        .contains("only decision phases"));
    // Removing one module of a combined phase keeps the rest of it.
    let mut off = c.clone();
    off.apply_override("twin_actions=off").unwrap();
    assert_eq!(off.round[1], "actions.play");
    Rules::new(&off).unwrap();
}

#[test]
fn tunables_include_card_parameters() {
    let rules = Rules::new(&baseline()).unwrap();
    let keys: Vec<String> = rules.tunables().into_iter().map(|t| t.key).collect();
    for k in [
        "income.amount",
        "actions.scrub_cost",
        "actions.invest_gain",
        "spread.per_country",
        "victory.overrun",
    ] {
        assert!(keys.iter().any(|x| x == k), "{k} missing from {keys:?}");
    }
}

#[test]
fn mismatched_states_are_errors_not_panics() {
    let config = baseline();
    let mut g = new_game(&config, 1).unwrap();
    g.rng.clear();
    assert!(legal_actions(&g).err().unwrap().contains("RNG streams"));
    let mut g = new_game(&config, 1).unwrap();
    g.seats.pop();
    assert!(apply(&g, &Action::Pass { seat: 0 }).is_err());
}

#[test]
fn rng_draws_are_fixed_width_and_pinned() {
    // Pinned so a native/wasm or rand-version difference shows up here.
    let mut s = rng::Stream::new(42, "spread");
    let draws: Vec<usize> = (0..8).map(|_| s.below(16)).collect();
    assert_eq!(draws, GOLDEN_DRAWS);
}

const GOLDEN_DRAWS: [usize; 8] = [10, 13, 7, 2, 12, 9, 1, 7];
const GOLDEN_FINGERPRINT: &str = "dd7d9b197e69f946";

/// A fixed game (baseline, seed u64::MAX, always the first legal action).
/// `scripts/wasm-smoke.mjs` checks the wasm build reaches the same state.
/// If this fails after an intended rules change, bump `RULES_REVISION`
/// and update both fingerprints.
#[test]
fn golden_game() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(u64::MAX);
    loop {
        let legal = rules.legal_actions(&g);
        let Some(a) = legal.first() else { break };
        rules.apply_mut(&mut g, a, None).unwrap();
    }
    assert_eq!(
        (RULES_REVISION, g.fingerprint().as_str()),
        (1, GOLDEN_FINGERPRINT)
    );
}

#[test]
fn removing_the_last_module_of_an_explicit_round_is_an_error() {
    let mut c = baseline();
    c.round = vec!["spread.advance".into(), "victory.check".into()];
    c.apply_override("spread=off").unwrap();
    let err = c.apply_override("victory=off").err().unwrap();
    assert!(err.contains("no phases"), "{err}");
    assert!(c.module("victory").is_some());
}
