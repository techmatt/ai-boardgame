use rand::{RngCore, SeedableRng};

use super::*;

const BASELINE: &str = include_str!("../../../configs/baseline.json");
const SHARED_ROW: &str = include_str!("../../../configs/shared_row.json");

fn baseline() -> Config {
    Config::from_json(BASELINE).unwrap()
}

/// Both threat variants, every opposition reward variant, difficulty 0 and 4.
fn variant_matrix() -> Vec<Config> {
    let mut out = Vec::new();
    let threats: [&[&str]; 2] = [&[], &["threats.variant=shared_row"]];
    let rewards: [&[&str]; 4] = [
        &[],
        &["opposition.variant=income", "opposition.income_credits=2"],
        &["opposition.variant=free_purchases", "opposition.free_purchases=2"],
        &["opposition.variant=counter_threat"],
    ];
    let levels: [&[&str]; 2] = [&[], &["adversary.difficulty=4"]];
    for t in threats {
        for r in rewards {
            for l in levels {
                let mut c = baseline();
                for o in t.iter().chain(r).chain(l) {
                    c.apply_override(o).unwrap();
                }
                Rules::new(&c).unwrap();
                out.push(c);
            }
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

#[test]
fn the_shared_row_config_differs_only_in_the_threat_variant() {
    let a = Rules::new(&baseline()).unwrap();
    let b = Rules::new(&Config::from_json(SHARED_ROW).unwrap()).unwrap();
    assert_eq!(a.config.diff(&b.config), ["threats.variant: countries -> shared_row"]);
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
    let expect = [
        (3, vec![&beige, &pink, &blue3]),
        (4, vec![&beige, &pink, &blue, &green]),
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
    for n in 1..=4 {
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
fn country_pieces_follow_the_ruleset() {
    let c = content();
    for n in 1..=4u8 {
        let map = Map::build(c, n).unwrap();
        for country in 0..n as usize {
            let piece = &c.map.countries[country];
            let hexes = &map.countries[country].hexes;
            for t in content::Terrain::ALL {
                let k = hexes.iter().filter(|&&h| map.hexes[h].terrain == t).count();
                assert_eq!(k, 4, "{n} countries, country {country}: {t:?}");
            }
            let uplinks: Vec<usize> = hexes.iter().copied().filter(|&h| map.hexes[h].uplink).collect();
            assert_eq!(uplinks.len(), 2);
            for &u in &uplinks {
                assert_eq!(map.hexes[u].terrain, content::Terrain::Frontier);
                // On the outer edge: off-board neighbours, none in another country.
                assert!(map.hexes[u].neighbours.len() < 6, "{n}: uplink {u} is inland");
                assert!(map.hexes[u]
                    .neighbours
                    .iter()
                    .all(|&o| map.country_of(o) as usize == country));
            }
            let pop: Vec<usize> = piece.population.iter().map(|&k| map.numbered(country, k)).collect();
            assert_eq!(pop.len(), 6);
            let urban = pop.iter().filter(|&&h| map.hexes[h].terrain == content::Terrain::Urban).count();
            assert_eq!(urban, 4, "population on every Urban hex");
            for &f in &piece.factories {
                let h = map.numbered(country, f);
                assert_eq!(map.hexes[h].terrain, content::Terrain::Industrial);
                assert!(map.hexes[h].neighbours.iter().any(|o| pop.contains(o)), "factory {f} next to population");
            }
            for &d in &piece.data_centres {
                assert_eq!(map.hexes[map.numbered(country, d)].terrain, content::Terrain::Network);
            }
        }
    }
    // The countries differ.
    let t: Vec<String> = c.map.countries.iter().map(|p| p.terrain.concat()).collect();
    assert!(t.windows(2).all(|w| w[0] != w[1]));
}

#[test]
fn setup_follows_the_ruleset() {
    for n in 1..=4u8 {
        let mut c = baseline();
        c.countries = n;
        let rules = Rules::new(&c).unwrap();
        let g = rules.new_game(5);
        let w = rules.world();
        let total = |k: &str, c: usize| -> i64 { rules.map.countries[c].hexes.iter().map(|&h| g.tokens(h, k)).sum() };
        for country in 0..n as usize {
            assert_eq!(total("population", country), 6);
            assert_eq!(total("factory", country), 2);
            assert_eq!(total("data_centre", country), 2);
            assert_eq!(total("outpost", country), 1);
            // Opening explore: uplinks may have gained a drone.
            assert!(total("drone", country) >= 2);
        }
        assert_eq!(g.var("adversary.pool"), 2 * n as i64 + 2);
        assert_eq!(g.var("board.start_population"), 6 * n as i64);
        // Opening: the first card explored and slid into the build slot.
        assert_eq!(g.pile("adversary.build").len(), 1);
        assert!(g.pile("adversary.explore").is_empty());
        assert_eq!(g.pile("adversary.deck").len(), 9);
        for s in 0..n as usize {
            assert_eq!(g.resource(s, "credits"), 12);
            let board = modules::ai::board(&w, s);
            assert_eq!(modules::common::processors_of(&g, s), 1);
            // The first processor is in the starting country.
            assert!(modules::common::has_processor_in(&w, &g, s, s));
            // An AI's unique cards: one copy each, so a repeated AI gets none.
            let hand = g.pile(&format!("cards.hand.{s}"));
            if s < 2 {
                assert_eq!(hand, board.cards.as_slice());
            } else {
                assert!(hand.is_empty());
            }
        }
        // Round 1: threats dealt (tier 1), insight from the tracks.
        assert_eq!(g.round, 1);
        for country in 0..n as usize {
            let on = g.pile(&format!("threats.on.{country}"));
            assert_eq!(on.len(), 1);
            assert_eq!(content().threats[content().threat_index(&on[0]).unwrap()].tier, 1);
        }
        assert_eq!(g.resource(0, "insight"), 2);
        if n > 1 {
            assert_eq!(g.resource(1, "insight"), 1);
        }
    }
}

#[test]
fn every_card_is_unique_in_every_game() {
    let rules = Rules::new(&baseline()).unwrap();
    for seed in 0..8 {
        let (_, actions) = random_game(&rules, seed);
        let mut g = rules.new_game(seed);
        for a in &actions {
            rules.apply_mut(&mut g, a, None).unwrap();
            let mut seen = std::collections::BTreeSet::new();
            for (k, pile) in &g.piles {
                for id in pile {
                    assert!(seen.insert(id.clone()), "{id} twice (in {k})");
                }
            }
        }
    }
}

#[test]
fn same_seed_same_game() {
    let rules = Rules::new(&baseline()).unwrap();
    assert_eq!(rules.new_game(7), rules.new_game(7));
    assert_ne!(rules.new_game(7).piles, rules.new_game(8).piles);
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
    assert!(v["rng"]["cards"]["word_pos"].is_string());
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
        for seed in 0..6 {
            let (g, actions) = random_game(&rules, seed);
            assert!(g.is_over(), "{} seed {seed} did not finish", config.hash());
            assert_eq!(replay(&rules, seed, &actions).unwrap(), g);
            // The slow, JSON-shaped path agrees with the hot path, and a
            // state survives serialization at any point (mid-choice too).
            let mut s = new_game(&config, seed).unwrap();
            for (i, a) in actions.iter().enumerate() {
                if i % 5 == 0 {
                    s = serde_json::from_str(&s.to_json()).unwrap();
                }
                s = apply(&s, a).unwrap().0;
            }
            assert_eq!(s, g);
        }
    }
}

#[test]
fn every_country_count_plays_to_an_end_and_replays() {
    for n in 1..=4u8 {
        let mut c = baseline();
        c.countries = n;
        let rules = Rules::new(&c).unwrap();
        for seed in 0..10 {
            let (g, actions) = random_game(&rules, seed);
            assert!(g.is_over(), "{n} countries, seed {seed}");
            assert_eq!(replay(&rules, seed, &actions).unwrap(), g);
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
    // An omitted round resolves to module order, then phase order.
    let mut implicit = baseline();
    implicit.round.clear();
    let r = Rules::new(&implicit).unwrap();
    assert_eq!(r.config.round[0], "board.cleanup");
    assert_eq!(r.config.round[1], "adversary.ravage");
    let mut tuned = baseline();
    tuned.apply_override("adversary.difficulty=3").unwrap();
    assert_ne!(Rules::new(&tuned).unwrap().config_hash, rules.config_hash);
}

#[test]
fn list_parameters_accept_a_single_integer() {
    let mut c = baseline();
    c.apply_override("ai.boards=1").unwrap();
    let rules = Rules::new(&c).unwrap();
    assert_eq!(
        rules.config.module("ai").unwrap().params["boards"],
        ParamValue::List(vec![1])
    );
    // Every seat now plays the Protector.
    let w = rules.world();
    assert_eq!(modules::ai::board(&w, 0).id, "protector");
    assert_eq!(modules::ai::board(&w, 1).id, "protector");
}

#[test]
fn bad_configs_are_rejected_with_a_reason() {
    let cases: &[(&str, &str)] = &[
        ("countries=6", "countries must be 1-5"),
        ("countries=5", "no map layout for 5 countries"),
        ("adversary.variant=nope", "unknown variant"),
        ("adversary.dificulty=3", "not a parameter"),
        ("adversary.difficulty=5", "outside"),
        ("threats.per_country=1,2", "expected an integer"),
        ("ai.boards=7", "no AI board 7"),
        ("adversary.stage_cards=3,4", "give 3 counts"),
        ("threats=off", "needs module threats"),
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
    c.modules[1].params.remove("difficulty");
    assert!(Rules::new(&c)
        .err()
        .unwrap()
        .contains("adversary.difficulty: missing"));
    let mut c = baseline();
    c.modules[6].cards.push("nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("unknown card"));
    let mut c = baseline();
    c.round.push("adversary.nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("no phase"));
    let mut c = baseline();
    c.modules.push(c.modules[0].clone());
    assert!(Rules::new(&c).err().unwrap().contains("appears twice"));
    assert!(Config::from_json(r#"{"countries": 3, "modules": [], "extra": 1}"#).is_err());
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

const ACTIONS: &str = "cards.play+threats.counter+economy.give";

#[test]
fn growth_and_draft_picks_cannot_be_passed() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(2);
    advance_to(&rules, &mut g, "cards.draft");
    let legal = rules.legal_actions(&g);
    assert!(legal.iter().all(|a| matches!(a, Action::Draft { .. })), "{legal:?}");
    assert_eq!(legal.iter().filter(|a| a.seat() == 0).count(), 4);
    assert!(rules.apply_mut(&mut g, &Action::Pass { seat: 0 }, None).is_err());
    advance_to(&rules, &mut g, "ai.growth");
    let legal = rules.legal_actions(&g);
    assert!(legal.iter().all(|a| matches!(a, Action::Growth { .. })), "{legal:?}");
    assert_eq!(legal.len(), 6);
    // Each player keeps the 4 cards they drafted until they pass the keep step: here
    // the first legal action kept or passed, so hands hold uniques plus kept cards.
    assert!(g.pile("cards.hand.0").len() >= 3);
}

/// A config whose round is one ravage then the action phase, for crafted states.
fn ravage_only() -> (Rules, GameState) {
    let mut c = baseline();
    c.round = vec!["adversary.ravage".into(), ACTIONS.into()];
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(1);
    for h in g.hexes.iter_mut() {
        h.tokens.clear();
    }
    (rules, g)
}

#[test]
fn ravage_asks_the_loss_order_then_corrupts_cascades_and_brings_momentum() {
    let (rules, mut g) = ravage_only();
    let map = &rules.map;
    // Country 0, hex 5 (Urban), ravaged by the Urban stage-I card.
    let h = map.numbered(0, 5);
    assert_eq!(map.hexes[h].terrain, content::Terrain::Urban);
    g.piles.insert("adversary.ravage".into(), vec!["s1_urban".into()]);
    for (k, n) in [("drone", 1), ("outpost", 1), ("population", 1), ("factory", 1)] {
        g.hexes[h].tokens.insert(k.into(), n);
    }
    // Another population elsewhere keeps the population loss in check.
    g.hexes[map.numbered(1, 5)].tokens.insert("population".into(), 6);
    g.vars.insert("board.start_population".into(), 2);
    let start = g.clone();
    g.phase = 0;
    rules.enter(&mut g, None);
    // 3 damage: population (2) or factory (3) first: the players choose.
    let p = g.pending.as_ref().expect("a loss-order choice");
    let opts: Vec<&str> = p.ask.options.iter().map(|o| o.option.as_str()).collect();
    assert_eq!(opts, ["population", "factory"]);
    let legal = rules.legal_actions(&g);
    assert!(legal.iter().all(|a| matches!(a, Action::Choose { .. })));
    assert!(rules.apply_mut(&mut g.clone(), &Action::Pass { seat: 0 }, None).is_err());
    // Mid-choice states survive JSON.
    let back: GameState = serde_json::from_str(&g.to_json()).unwrap();
    assert_eq!(back, g);

    // Factory first: 3 damage spent; corruption from the pool, no Momentum.
    let mut a = g.clone();
    let pick = |o: &str| Action::Choose { seat: 0, option: o.into(), target: None };
    rules.apply_mut(&mut a, &pick("factory"), None).unwrap();
    assert!(a.pending.is_none());
    assert_eq!((a.tokens(h, "factory"), a.tokens(h, "population")), (0, 1));
    assert_eq!(a.tokens(h, "corruption"), 1);
    assert_eq!(a.var("adversary.pool"), start.var("adversary.pool") - 1);
    assert_eq!(rules.phase(&a).unwrap().name, ACTIONS);

    // Population first: 1 damage left destroys nothing more; Momentum asks
    // where the new drone goes (adjacent, same country).
    let mut b = g.clone();
    rules.apply_mut(&mut b, &pick("population"), None).unwrap();
    let p = b.pending.as_ref().expect("a Momentum choice");
    assert!(p.ask.prompt.starts_with("Momentum"));
    for o in &p.ask.options {
        let t = map.find(o.target.unwrap()).unwrap();
        assert!(map.adjacent(t, h) && map.country_of(t) == 0);
    }
    let first = p.ask.options[0].clone();
    rules
        .apply_mut(&mut b, &Action::Choose { seat: 0, option: first.option, target: first.target }, None)
        .unwrap();
    let t = map.find(first.target.unwrap()).unwrap();
    assert_eq!(b.tokens(t, "drone"), 1);
    assert_eq!((b.tokens(h, "population"), b.tokens(h, "factory")), (0, 1));
    assert_eq!(b.tokens(h, "corruption"), 1);
    // Answers are actions: replaying the log rebuilds the same state.
    assert_eq!(b.stats.actions, 2);

    // Already corrupted: the new corruption cascades to an adjacent hex.
    let mut c = start.clone();
    c.hexes[h].tokens.insert("corruption".into(), 1);
    c.hexes[h].tokens.remove("factory");
    c.phase = 0;
    rules.enter(&mut c, None);
    // Population is the only choice (auto); then the cascade asks.
    let p = c.pending.as_ref().expect("a cascade choice");
    assert!(p.ask.prompt.starts_with("Corruption cascades"), "{}", p.ask.prompt);
    let o = p.ask.options[0].clone();
    rules
        .apply_mut(&mut c, &Action::Choose { seat: 0, option: o.option, target: o.target }, None)
        .unwrap();
    assert_eq!(c.tokens(h, "corruption"), 2);
    assert_eq!(c.tokens(map.find(o.target.unwrap()).unwrap(), "corruption"), 1);
}

#[test]
fn defense_prevents_ravage_damage_and_the_protector_doubles_it() {
    let (rules, mut g) = ravage_only();
    let h = rules.map.numbered(0, 5);
    g.piles.insert("adversary.ravage".into(), vec!["s1_urban".into()]);
    for (k, n) in [("outpost", 1), ("population", 1)] {
        g.hexes[h].tokens.insert(k.into(), n);
    }
    g.hexes[rules.map.numbered(1, 5)].tokens.insert("population".into(), 6);
    // Seat 1 is the Protector: one of its defense tokens prevents 2.
    g.hexes[h].tokens.insert("defense:1".into(), 1);
    g.phase = 0;
    rules.enter(&mut g, None);
    assert!(g.pending.is_none());
    assert_eq!((g.tokens(h, "population"), g.tokens(h, "corruption")), (1, 0));
}

#[test]
fn threats_are_countered_only_when_fully_paid_and_partial_payments_come_back() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(3);
    advance_to(&rules, &mut g, ACTIONS);
    g.piles.insert("threats.on.0".into(), vec!["patrol".into()]);
    g.piles.insert("threats.on.1".into(), vec!["conscription".into()]);
    for s in 0..2 {
        g.seats[s].resources.insert("energy".into(), 5);
        g.seats[s].resources.insert("credits".into(), 10);
    }
    let legal = rules.legal_actions(&g);
    // Only players with a processor in the threat's country may pay.
    let pays = |seat: u8, t: &str| {
        legal
            .iter()
            .any(|a| matches!(a, Action::Pay { seat: s, threat, .. } if *s == seat && threat == t))
    };
    assert!(pays(0, "patrol") && !pays(0, "conscription"));
    assert!(pays(1, "conscription") && !pays(1, "patrol"));
    let pay = |seat, threat: &str, resource: &str, amount| Action::Pay {
        seat,
        threat: threat.into(),
        resource: resource.into(),
        amount,
    };
    // Conscription: 2 energy. Pay 1, withdraw, then pay in full.
    rules.apply_mut(&mut g, &pay(1, "conscription", "energy", 1), None).unwrap();
    assert_eq!(g.resource(1, "energy"), 4);
    rules
        .apply_mut(&mut g, &Action::Withdraw { seat: 1, threat: "conscription".into(), resource: "energy".into() }, None)
        .unwrap();
    assert_eq!(g.resource(1, "energy"), 5);
    assert!(rules.apply_mut(&mut g, &pay(1, "conscription", "energy", 3), None).is_err());
    rules.apply_mut(&mut g, &pay(1, "conscription", "energy", 2), None).unwrap();
    assert!(g.pile("threats.on.1").is_empty());
    assert_eq!(g.pile("threats.discard.1")[0], "conscription");
    // Patrol: 2 energy + 2 credits. Pay only the energy; it is refunded.
    rules.apply_mut(&mut g, &pay(0, "patrol", "energy", 2), None).unwrap();
    assert_eq!(g.pile("threats.on.0"), ["patrol"]);
    let energy = g.resource(0, "energy");
    for s in 0..2 {
        let _ = rules.apply_mut(&mut g, &Action::Pass { seat: s }, None);
    }
    // Refunded before it executed (and before upkeep income).
    assert!(g.resource(0, "energy") >= energy + 2);
    assert!(g.pile("threats.discard.1").contains(&"patrol".to_string()));
}

#[test]
fn jamming_a_threat_down_to_what_is_paid_counters_it() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(3);
    advance_to(&rules, &mut g, ACTIONS);
    // Panic needs 1 insight; Jam Signals takes it to 0: countered at once.
    g.piles.insert("threats.on.0".into(), vec!["panic".into()]);
    g.piles.insert("cards.hand.0".into(), vec!["jam_signals".into()]);
    g.seats[0].resources.insert("credits".into(), 10);
    let play = Action::Play { seat: 0, card: "jam_signals".into() };
    rules.apply_mut(&mut g, &play, None).unwrap();
    while let Some(p) = &g.pending {
        let o = p.ask.options.iter().find(|o| o.option == "panic").unwrap_or(&p.ask.options[0]).clone();
        let seat = p.ask.seat;
        rules
            .apply_mut(&mut g, &Action::Choose { seat, option: o.option, target: o.target }, None)
            .unwrap();
    }
    assert!(g.pile("threats.on.0").is_empty());
    assert!(g.pile("threats.discard.1").contains(&"panic".to_string()));
}

#[test]
fn malformed_pending_states_are_errors_not_panics() {
    let (rules, mut g) = ravage_only();
    let h = rules.map.numbered(0, 5);
    g.piles.insert("adversary.ravage".into(), vec!["s1_urban".into()]);
    for (k, n) in [("drone", 1), ("outpost", 1), ("population", 1), ("factory", 1)] {
        g.hexes[h].tokens.insert(k.into(), n);
    }
    g.phase = 0;
    rules.enter(&mut g, None);
    assert!(rules.check_state(&g).is_ok());
    let mut bad = g.clone();
    bad.pending.as_mut().unwrap().snapshot.seats.pop();
    assert!(rules.check_state(&bad).is_err());
    let mut bad = g.clone();
    bad.pending.as_mut().unwrap().ask.seat = 9;
    assert!(rules.check_state(&bad).is_err());
}

#[test]
fn difficulty_levels_change_the_numbers() {
    let mut c = baseline();
    c.apply_override("adversary.difficulty=4").unwrap();
    let rules = Rules::new(&c).unwrap();
    let g = rules.new_game(1);
    let w = rules.world();
    // Level 4: pool 1 smaller per country; level 2: one stage-I card fewer.
    assert_eq!(g.var("adversary.pool"), 2 * 2 + 2 - 2);
    assert_eq!(g.pile("adversary.deck").len(), 8);
    // Level 1: the first counter requirement is +1.
    assert_eq!(modules::threats::requirement(&w, &g, "patrol"), [("energy".to_string(), 3), ("credits".to_string(), 2)]);
    let base = Rules::new(&baseline()).unwrap();
    let w0 = base.world();
    assert_eq!(modules::threats::requirement(&w0, &g, "patrol")[0].1, 2);
}

#[test]
fn reaching_an_opposition_level_pushes_back_and_can_win() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    g.seats[0].resources.insert("insight".into(), 1);
    // One short of level 1 (4 per player, 2 players); no outposts or foundries.
    g.vars.insert("opposition.points".into(), 8);
    for h in g.hexes.iter_mut() {
        h.tokens.remove("outpost");
        h.tokens.remove("foundry");
    }
    let mut events = Vec::new();
    rules
        .apply_mut(&mut g, &Action::Give { seat: 0, to: 1 }, Some(&mut events))
        .unwrap();
    // Level 1 reached: each country removes a drone (players choose).
    assert_eq!(g.var("opposition.level"), 1);
    while let Some(p) = &g.pending {
        assert!(p.ask.prompt.starts_with("Pushback"));
        let o = p.ask.options[0].clone();
        rules
            .apply_mut(&mut g, &Action::Choose { seat: 0, option: o.option, target: o.target }, None)
            .unwrap();
    }
    // At level 1, players win once no outposts or foundries remain.
    let o = g.outcome.as_ref().expect("a win");
    assert!(o.won && o.reason == "opposition", "{o:?}");
}

#[test]
fn the_game_is_lost_when_the_corruption_pool_runs_out() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    g.vars.insert("adversary.pool".into(), 0);
    g.seats[0].resources.insert("insight".into(), 1);
    rules.apply_mut(&mut g, &Action::Give { seat: 0, to: 1 }, None).unwrap();
    assert_eq!(g.outcome.as_ref().unwrap().reason, "corruption");
}

#[test]
fn rng_draws_are_fixed_width_and_pinned() {
    // Pinned so a native/wasm or rand-version difference shows up here.
    let mut s = rng::Stream::new(42, "spread");
    let draws: Vec<usize> = (0..8).map(|_| s.below(16)).collect();
    assert_eq!(draws, GOLDEN_DRAWS);
}

const GOLDEN_DRAWS: [usize; 8] = [10, 13, 7, 2, 12, 9, 1, 7];
const GOLDEN_FINGERPRINT: &str = "4140c11858dbf6fd";

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
        (2, GOLDEN_FINGERPRINT)
    );
}

#[test]
fn the_view_shows_terrain_threats_pipeline_tracks_and_choices() {
    for config in variant_matrix() {
        let rules = Rules::new(&config).unwrap();
        let shared = config.module("threats").unwrap().variant == "shared_row";
        let mut g = rules.new_game(9);
        let mut saw_choice = false;
        for _ in 0..400 {
            if g.is_over() {
                break;
            }
            let v = rules.view(&g);
            assert_eq!(v.hexes.len(), g.hexes.len());
            assert!(v.hexes.iter().all(|h| !h.terrain.is_empty() && (1..=16).contains(&h.number)));
            assert_eq!(v.hexes.iter().filter(|h| h.uplink).count(), 2 * g.seats.len());
            let pile = |k: &str| v.piles.iter().find(|p| p.key == k);
            for k in ["adversary.deck", "adversary.explore", "adversary.build", "adversary.ravage"] {
                assert!(pile(k).is_some(), "{k}");
            }
            let global = |k: &str| v.globals.iter().find(|x| x.key == k);
            assert!(global("opposition.points").is_some_and(|x| x.max.is_some()));
            assert!(global("adversary.pool").is_some());
            for s in &v.seats {
                assert!(s.values.iter().any(|x| x.key == "ai.track0"));
                assert!(s.cards.iter().any(|c| c.key == "cards.hand"));
                assert!(s.cards.iter().any(|c| c.key == "cards.built"));
            }
            let threats_shown: usize = if shared {
                pile("threats.row").map_or(0, |p| p.count)
            } else {
                v.countries.iter().flat_map(|c| &c.cards).map(|c| c.cards.len()).sum()
            };
            assert_eq!(threats_shown, modules::threats::active(&g).len());
            assert_eq!(v.choice.is_some(), g.pending.is_some());
            saw_choice |= v.choice.is_some();
            let a = rules.legal_actions(&g)[g.round as usize % 3 % rules.legal_actions(&g).len()].clone();
            rules.apply_mut(&mut g, &a, None).unwrap();
        }
        let _ = saw_choice;
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
fn saved_games_round_trip_and_replay_to_the_saved_state() {
    let mut config = baseline();
    config.apply_override("threats.variant=shared_row").unwrap();
    let rules = Rules::new(&config).unwrap();
    let (g, actions) = random_game(&rules, 21);
    let rec = SavedGame::new(&rules, 21, actions.clone(), &g, "a loss");
    let text = rec.to_text();
    assert!(text.starts_with("{\n  \"format\": \"aic-game-record/1\",\n  \"note\": \"a loss\""));
    let back = SavedGame::from_json(&text).unwrap();
    assert_eq!(back, rec);
    let (h, warnings) = back.replay().unwrap();
    assert_eq!((h, warnings), (g.clone(), vec![]));

    // The minimal hand-written form loads too.
    for seed in [serde_json::json!("21"), serde_json::json!(21)] {
        let minimal = serde_json::json!({
            "config": config, "seed": seed, "actions": actions,
        });
        let (h, warnings) = SavedGame::from_json(&minimal.to_string())
            .unwrap()
            .replay()
            .unwrap();
        assert_eq!((h, warnings), (g.clone(), vec![]));
    }

    let mut bad = rec.clone();
    bad.final_fingerprint = "0000000000000000".into();
    let w = bad.replay().unwrap().1;
    assert!(w[0].contains("fingerprint"), "{w:?}");
    let mut bad = rec.clone();
    bad.actions.insert(0, Action::Pass { seat: 9 });
    assert!(bad.replay().err().unwrap().contains("action 0"));
    let other = text.replace("aic-game-record/1", "aic-game-record/9");
    assert!(SavedGame::from_json(&other)
        .err()
        .unwrap()
        .contains("format"));
    let empty = SavedGame::new(&rules, 1, vec![], &rules.new_game(1), "");
    assert_eq!(SavedGame::from_json(&empty.to_text()).unwrap(), empty);
}

#[test]
fn tunables_include_card_parameters() {
    let mut c = baseline();
    // A card cost written as {"param": ...} would become a cards tunable;
    // the v0 content uses literals, so only declared parameters show.
    c.modules[6].cards.retain(|id| id != "nope");
    let rules = Rules::new(&c).unwrap();
    let keys: Vec<String> = rules.tunables().into_iter().map(|t| t.key).collect();
    for k in [
        "board.start_population",
        "adversary.difficulty",
        "adversary.stage_cards",
        "threats.per_country",
        "opposition.per_player",
        "economy.factory_credits",
        "ai.boards",
        "cards.keep_cost",
    ] {
        assert!(keys.iter().any(|x| x == k), "{k} missing from {keys:?}");
    }
}

const CLUSTERS: &str = include_str!("../../../configs/clusters.json");
const ASSETS: &str = include_str!("../../../configs/assets.json");
const OFFS: [&str; 3] = ["threats.variant=off", "opposition.variant=off", "cards.variant=off"];

/// Every adversary variant, alone and with each off-variant, at 2 and 3
/// countries; the spread variants at difficulty 4 too.
fn spread_and_off_configs() -> Vec<Config> {
    let mut out = Vec::new();
    for base in [BASELINE, CLUSTERS, ASSETS] {
        for off in std::iter::once(None).chain(OFFS.iter().map(Some)) {
            for n in [2, 3] {
                let mut c = Config::from_json(base).unwrap();
                c.countries = n;
                if let Some(o) = off {
                    c.apply_override(o).unwrap();
                }
                out.push(c);
            }
        }
    }
    for base in [CLUSTERS, ASSETS] {
        let mut c = Config::from_json(base).unwrap();
        c.apply_override("adversary.difficulty=4").unwrap();
        out.push(c);
    }
    out
}

#[test]
fn spread_and_off_variants_play_and_replay_at_2_and_3_countries() {
    for config in spread_and_off_configs() {
        let rules = Rules::new(&config).unwrap();
        for seed in 0..4 {
            let (g, actions) = random_game(&rules, seed);
            assert!(g.is_over(), "{} seed {seed} did not finish", config.hash());
            assert_eq!(replay(&rules, seed, &actions).unwrap(), g);
            let mut s = new_game(&config, seed).unwrap();
            for (i, a) in actions.iter().enumerate() {
                if i % 7 == 0 {
                    s = serde_json::from_str(&s.to_json()).unwrap();
                }
                s = apply(&s, a).unwrap().0;
            }
            assert_eq!(s, g);
            let v = rules.view(&g);
            assert!(v.globals.iter().any(|x| x.key == "opposition.level"));
        }
    }
}

#[test]
fn the_spread_configs_differ_from_baseline_only_in_the_adversary() {
    let a = Rules::new(&baseline()).unwrap();
    for json in [CLUSTERS, ASSETS] {
        let b = Rules::new(&Config::from_json(json).unwrap()).unwrap();
        assert!(
            a.config.diff(&b.config).iter().all(|d| d.starts_with("adversary.")),
            "{:?}",
            a.config.diff(&b.config)
        );
    }
}

/// A Cx for calling module functions directly on a crafted state.
fn with_cx<T>(rules: &Rules, f: impl FnOnce(&mut module::Cx) -> T) -> T {
    let mut choices = module::Choices::new(Vec::new());
    let mut cx = module::Cx {
        world: rules.world(),
        module: "adversary",
        purposes: &[],
        events: None,
        choices: &mut choices,
    };
    f(&mut cx)
}

#[test]
fn clusters_start_from_the_militarist_pieces_and_assault_with_infection_then_combat() {
    let mut c = Config::from_json(CLUSTERS).unwrap();
    let rules = Rules::new(&c).unwrap();
    let g = rules.new_game(1);
    let count = |g: &GameState, k: &str| (0..g.hexes.len()).map(|h| g.tokens(h, k)).sum::<i64>();
    assert_eq!(count(&g, "drone") + count(&g, "outpost") + count(&g, "foundry"), 0);
    // One cluster per starting drone and outpost (2 uplinks + 1 outpost per
    // country), plus whatever the opening placed.
    assert!(count(&g, "cluster") >= 6, "{}", count(&g, "cluster"));

    c.round = vec!["adversary.ravage".into(), ACTIONS.into()];
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(1);
    for h in g.hexes.iter_mut() {
        h.tokens.clear();
    }
    let map = &rules.map;
    let h = map.numbered(0, 5);
    assert_eq!(map.hexes[h].terrain, content::Terrain::Urban);
    g.piles.insert("adversary.ravage".into(), vec!["s1_urban".into()]);
    for (k, n) in [("data_centre", 1), ("population", 1), ("processor:0", 1)] {
        g.hexes[h].tokens.insert(k.into(), n);
    }
    // Two non-Urban neighbours with 2 clusters each: offense 4, defense 1
    // (the processor). Excess 3 flips the data centre; the hex now holds a
    // cluster, so nothing spills.
    let next: Vec<usize> = map.hexes[h]
        .neighbours
        .iter()
        .copied()
        .filter(|&n| map.hexes[n].terrain != content::Terrain::Urban)
        .take(2)
        .collect();
    assert_eq!(next.len(), 2);
    for &n in &next {
        g.hexes[n].tokens.insert("cluster".into(), 2);
    }
    g.vars.insert("board.start_population".into(), 0);
    g.phase = 0;
    rules.enter(&mut g, None);
    assert!(g.pending.is_none());
    assert_eq!(g.tokens(h, "data_centre"), 0);
    assert_eq!(g.tokens(h, "cluster"), 1);
    // Combat: 1 cluster deals 1, which destroys the population (health 1).
    assert_eq!(g.tokens(h, "population"), 0);
    assert_eq!(g.tokens(h, "corruption"), 0);
    assert_eq!(g.stats.counters["adversary.infected"], 1 + count_spills(&g, &rules, h, &next));
}

/// Urban hexes other than `h` and its sources that picked up a cluster
/// (spilled infection).
fn count_spills(g: &GameState, rules: &Rules, h: usize, sources: &[usize]) -> i64 {
    (0..g.hexes.len())
        .filter(|&x| x != h && !sources.contains(&x))
        .filter(|&x| rules.map.hexes[x].terrain == content::Terrain::Urban)
        .map(|x| g.tokens(x, "cluster"))
        .sum()
}

#[test]
fn asset_corruption_opens_with_a_beachhead_and_damage_reclaims() {
    let rules = Rules::new(&Config::from_json(ASSETS).unwrap()).unwrap();
    let g = rules.new_game(3);
    let count = |g: &GameState, k: &str| (0..g.hexes.len()).map(|h| g.tokens(h, k)).sum::<i64>();
    assert_eq!(count(&g, "drone") + count(&g, "outpost"), 0);
    let rogue = |g: &GameState| count(g, "rogue_factory") + count(g, "rogue_data_centre");
    assert_eq!(rogue(&g), 2);
    let w = rules.world();
    let start = modules::adversary::start_pool(&w);
    assert_eq!(g.var("adversary.pool"), start - 2);

    // 2 damage reclaims a corrupted factory: it flips back, its corruption
    // returns to the pool, and it scores opposition.
    let mut g = g;
    let h = (0..g.hexes.len()).find(|&h| g.tokens(h, "rogue_factory") + g.tokens(h, "rogue_data_centre") > 0).unwrap();
    let kind = if g.tokens(h, "rogue_factory") > 0 { "factory" } else { "data_centre" };
    let before = g.tokens(h, kind);
    let points = g.var("opposition.points");
    with_cx(&rules, |cx| modules::adversary::damage(&mut g, cx, 0, h, 2)).unwrap();
    assert_eq!(g.tokens(h, kind), before + 1);
    assert_eq!(rogue(&g), 1);
    assert_eq!(g.var("adversary.pool"), start - 1);
    assert_eq!(g.var("opposition.points"), points + 2);
}

#[test]
fn asset_corruption_pushback_reclaims_and_the_purge_bar_counts_corrupted_assets() {
    let rules = Rules::new(&Config::from_json(ASSETS).unwrap()).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    let rogue = |g: &GameState| {
        (0..g.hexes.len()).map(|h| g.tokens(h, "rogue_factory") + g.tokens(h, "rogue_data_centre")).sum::<i64>()
    };
    assert!(rogue(&g) > 0);
    let w = rules.world();
    assert!(!modules::adversary::under_bar(&w, &g, 0));
    assert!(modules::adversary::win_text(&w, 1).contains("at most 2 corrupted assets"));
    g.seats[0].resources.insert("insight".into(), 1);
    g.vars.insert("opposition.points".into(), 8);
    rules.apply_mut(&mut g, &Action::Give { seat: 0, to: 1 }, None).unwrap();
    assert_eq!(g.var("opposition.level"), 1);
    while let Some(p) = &g.pending {
        assert!(p.ask.prompt.starts_with("Pushback"), "{}", p.ask.prompt);
        let o = p.ask.options[0].clone();
        rules
            .apply_mut(&mut g, &Action::Choose { seat: 0, option: o.option, target: o.target }, None)
            .unwrap();
    }
    if rogue(&g) <= 2 {
        let o = g.outcome.as_ref().expect("a win");
        assert!(o.won && o.reason == "opposition", "{o:?}");
    }
}

#[test]
fn off_variants_remove_threats_level_rewards_and_the_draft() {
    let mut c = baseline();
    c.apply_override("threats.variant=off").unwrap();
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    assert!(modules::threats::active(&g).is_empty());

    let mut c = baseline();
    c.apply_override("opposition.variant=off").unwrap();
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    let drones = |g: &GameState| (0..g.hexes.len()).map(|h| g.tokens(h, "drone")).sum::<i64>();
    let before = drones(&g);
    g.seats[0].resources.insert("insight".into(), 1);
    g.vars.insert("opposition.points".into(), 8);
    rules.apply_mut(&mut g, &Action::Give { seat: 0, to: 1 }, None).unwrap();
    assert_eq!(g.var("opposition.level"), 1);
    assert!(g.pending.is_none());
    assert_eq!(drones(&g), before);

    let mut c = baseline();
    c.apply_override("cards.variant=off").unwrap();
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    // Only the AIs' unique cards: nothing dealt, drafted or drawable.
    assert_eq!(g.pile("cards.hand.0").len(), 3);
    assert!(rules.legal_actions(&g).iter().all(|a| !matches!(a, Action::Draw { .. })));
}
