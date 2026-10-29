use std::collections::BTreeSet;

use rand::{RngCore, SeedableRng};

use super::*;
use crate::content::HexKind;
use crate::modules::adversary::{CURRENT, DECK, NEXT, STAGE, SUPPLY};
use crate::modules::common::{ambassador, cube, dc, dcs_of, ADV_AMBASSADOR, ADV_CUBE, AMBASSADORS, FACTORY, HARDEN, STATION, TROOP, UNIT};
use crate::modules::{adversary, alignment, board, threats};

const BASELINE: &str = include_str!("../../../configs/baseline.json");
const SHARED_ROW: &str = include_str!("../../../configs/shared_row.json");

/// The action phase of the baseline round.
const ACTIONS: &str = "cards.play+threats.counter+economy.give+alignment.act+board.act";

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
    ]);
    c.round = round.iter().map(|s| s.to_string()).collect();
    c
}

/// Rules playing `phases` then the action phase, and a game of them at the
/// action phase of round 1 with the board emptied (no tokens anywhere).
fn lab(phases: &[&str]) -> (Rules, GameState) {
    let mut round = phases.to_vec();
    round.push(ACTIONS);
    let rules = Rules::new(&lab_config(&round)).unwrap();
    let mut g = rules.new_game(1);
    assert!(g.pending.is_none() && !g.is_over());
    for h in g.hexes.iter_mut() {
        h.tokens.clear();
    }
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
        .apply_mut(g, &Action::Choose { seat, option: o.option, target: o.target }, None)
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
    rules.apply_mut(g, &a, None).unwrap_or_else(|e| panic!("{a:?}: {e}"));
    answer_all(rules, g);
}

fn std_act(seat: u8, act: &str) -> Action {
    Action::Standard { seat, act: act.into() }
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

/// Threat variants, Alignment variants, overflow, difficulty 0 and 4, the
/// draft variants and 1-4 countries.
fn variant_matrix() -> Vec<Config> {
    let sets: &[&[&str]] = &[
        &[],
        &["threats.variant=shared_row"],
        &["threats.variant=off"],
        &["alignment.variant=still"],
        &["alignment.variant=off"],
        &["alignment.momentum_every=2", "alignment.antimatter_discord=0"],
        &["adversary.variant=overflow"],
        &["adversary.difficulty=4"],
        &[
            "adversary.difficulty=4",
            "adversary.counter_bonus_all=1",
            "adversary.variant=overflow",
            "threats.variant=shared_row",
        ],
        &["cards.variant=free_keep", "cards.free_keeps=1"],
        &["cards.variant=off"],
        &["board.war=1"],
        &["countries=1"],
        &["countries=3"],
        &["countries=4"],
        &["countries=3", "adversary.difficulty=4", "alignment.variant=still"],
    ];
    sets.iter()
        .map(|s| {
            let c = baseline_with(s);
            Rules::new(&c).unwrap_or_else(|e| panic!("{s:?}: {e}"));
            c
        })
        .collect()
}

// ---------- configs ----------

#[test]
fn the_shared_row_config_differs_only_in_the_threat_variant() {
    let a = Rules::new(&baseline()).unwrap();
    let b = Rules::new(&Config::from_json(SHARED_ROW).unwrap()).unwrap();
    assert_eq!(a.config.diff(&b.config), ["threats.variant: countries -> shared_row"]);
}

#[test]
fn bad_configs_are_rejected_with_a_reason() {
    let cases: &[(&str, &str)] = &[
        ("countries=6", "countries must be 1-5"),
        ("countries=5", "no map layout for 5 countries"),
        ("adversary.variant=nope", "unknown variant"),
        ("adversary.dificulty=3", "not a parameter"),
        ("adversary.difficulty=5", "outside"),
        ("board.war=0,2", "outside"),
        ("threats.per_country=1,2", "expected an integer"),
        ("ai.boards=7", "no AI board 7"),
        ("adversary.stage_cards=3,4", "give 3 counts"),
        ("adversary.stage_cards=9,4,3", "stage 1 has only"),
        ("alignment.unity_ease=5,2", "give tip counts"),
        ("threats=off", "needs module threats"),
        ("board=off", "needs module board"),
    ];
    for (o, want) in cases {
        let mut c = baseline();
        c.apply_override(o).unwrap();
        let err = Rules::new(&c).err().unwrap_or_else(|| panic!("{o} was accepted"));
        assert!(err.contains(want), "{o}: {err}");
    }
    let mut c = baseline();
    c.modules[1].params.remove("difficulty");
    assert!(Rules::new(&c).err().unwrap().contains("adversary.difficulty: missing"));
    let mut c = baseline();
    c.modules[6].cards.push("nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("unknown card"));
    let mut c = baseline();
    c.round.push("adversary.nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("no phase"));
    let mut c = baseline();
    c.round.push("board.bases+board.war".into());
    assert!(Rules::new(&c).err().unwrap().contains("appears twice"));
    let mut c = baseline();
    c.modules.push(c.modules[0].clone());
    assert!(Rules::new(&c).err().unwrap().contains("appears twice"));
    assert!(Config::from_json(r#"{"countries": 3, "modules": [], "extra": 1}"#).is_err());
}

#[test]
fn list_parameters_accept_a_single_integer() {
    let rules = Rules::new(&baseline_with(&["ai.boards=1", "board.war=1"])).unwrap();
    let params = |m: &str| rules.config.module(m).unwrap().params.clone();
    assert_eq!(params("ai")["boards"], ParamValue::List(vec![1]));
    assert_eq!(params("board")["war"], ParamValue::List(vec![1]));
    // Every seat now plays the Protector, and every round is a war round.
    let w = rules.world();
    assert_eq!(modules::ai::board(&w, 0).id, "protector");
    assert_eq!(modules::ai::board(&w, 1).id, "protector");
    let mut g = rules.new_game(1);
    g.piles.remove(CURRENT);
    for r in 1..=5 {
        g.round = r;
        assert!(board::war_round(&w, &g), "round {r}");
    }
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
    assert_eq!(&r.config.round[..3], ["board.act", "board.bases", "board.war"]);
    let tuned = baseline_with(&["adversary.difficulty=3"]);
    assert_ne!(Rules::new(&tuned).unwrap().config_hash, rules.config_hash);
}

#[test]
fn tunables_list_every_module_parameter() {
    let rules = Rules::new(&baseline()).unwrap();
    let t = rules.tunables();
    let find = |k: &str| t.iter().find(|x| x.key == k).unwrap_or_else(|| panic!("{k} missing"));
    for k in [
        "board.start_corrupted",
        "board.city_energy",
        "board.free_city_at",
        "board.align_energy",
        "adversary.difficulty",
        "adversary.supply_per_country",
        "adversary.corrupt_at",
        "threats.per_country",
        "alignment.tip_per_player",
        "alignment.unity_win",
        "economy.give_insight",
        "ai.start_credits",
        "cards.keep_cost",
    ] {
        assert_eq!(find(k).kind, "int", "{k}");
    }
    for k in ["board.war", "adversary.stage_cards", "alignment.unity_ease", "ai.boards"] {
        assert_eq!(find(k).kind, "list", "{k}");
    }
    let war = find("board.war");
    assert_eq!((war.min, war.max), (0, 1));
    assert_eq!(war.value, ParamValue::List(vec![0, 1, 0, 1, 0, 1, 1, 1]));
    // One per declared parameter: no duplicates.
    let keys: BTreeSet<&str> = t.iter().map(|x| x.key.as_str()).collect();
    assert_eq!(keys.len(), t.len());
}

// ---------- the map ----------

#[test]
fn every_layout_builds_and_every_piece_follows_the_design() {
    let content = crate::content();
    for n in 1..=4u8 {
        let map = Map::build(content, n).unwrap();
        assert_eq!(map.hexes.len(), 16 * n as usize);
        for c in 0..n as usize {
            let p = &content.map.countries[c];
            let at = |num: u8| map.numbered(c, num);
            assert_eq!(p.cities.len(), 3, "country {c}");
            assert!(p.corrupted.len() >= 2, "country {c}: two cities to corrupt at difficulty 3");
            assert_eq!(p.stations.len(), 1, "country {c}");
            assert_eq!(p.factories.len(), 2, "country {c}");
            assert_eq!(p.data_centres.len(), 2, "country {c}");
            assert!(p.ocean.len() <= 2, "country {c}");
            let count = |k: HexKind| map.countries[c].hexes.iter().filter(|&&h| kind(&map, h) == k).count();
            assert_eq!(count(HexKind::City), 3);
            assert_eq!(count(HexKind::Ocean), p.ocean.len());
            for &f in &p.factories {
                let h = at(f);
                assert!(
                    map.hexes[h].neighbours.iter().any(|&x| map.hexes[x].country as usize == c && kind(&map, x) == HexKind::City),
                    "country {c}: factory #{f} touches no city of its piece"
                );
            }
            for &o in &p.ocean {
                assert!(map.hexes[at(o)].neighbours.len() < 6, "country {c}: ocean #{o} is inland with {n} countries");
            }
            let [a, b] = [at(p.data_centres[0]), at(p.data_centres[1])];
            assert!(map.adjacent(a, b), "country {c}: the starting data centres are apart");
        }
    }
}

#[test]
fn every_front_has_a_base_on_each_side_and_every_country_a_front() {
    let content = crate::content();
    for n in 1..=4u8 {
        let map = Map::build(content, n).unwrap();
        let land = |h: usize| kind(&map, h) != HexKind::Ocean;
        for f in &map.fronts {
            assert_ne!(f.countries[0], f.countries[1]);
            for i in 0..2 {
                let (b, me, other) = (f.bases[i], f.countries[i], f.countries[1 - i]);
                let cell = &map.hexes[b];
                let p = &content.map.countries[me as usize];
                assert_eq!((cell.country, cell.kind, cell.facing), (me, HexKind::Base, Some(other)));
                assert_eq!(p.kind_at(cell.number), HexKind::Open, "{n} countries: base on a printed city or ocean");
                for list in [&p.stations, &p.factories, &p.data_centres] {
                    assert!(!list.contains(&cell.number), "{n} countries: base on a setup building");
                }
                assert!(cell.neighbours.iter().any(|&x| map.hexes[x].country == other && land(x)));
            }
        }
        // Exactly the pairs of countries that touch by land have a front.
        for a in 0..n {
            for b in a + 1..n {
                let touch = (0..map.hexes.len()).any(|h| {
                    map.hexes[h].country == a
                        && land(h)
                        && map.hexes[h].neighbours.iter().any(|&x| map.hexes[x].country == b && land(x))
                });
                let front = map.fronts.iter().any(|f| f.countries == [a, b]);
                assert_eq!(touch, front, "{n} countries: {a}-{b}");
            }
        }
        assert_eq!(map.hexes.iter().filter(|c| c.kind == HexKind::Base).count(), 2 * map.fronts.len());
        if n == 1 {
            assert!(map.fronts.is_empty());
        }
        if n == 2 || n == 3 {
            for c in 0..n {
                assert!(map.fronts.iter().any(|f| f.countries.contains(&c)), "{n} countries: country {c} has no front");
            }
        }
    }
}

#[test]
fn an_ocean_edge_is_not_a_front() {
    let mut content = crate::content().clone();
    let map = Map::build(&content, 2).unwrap();
    assert_eq!(map.fronts.len(), 1);
    // Flood country 1's border: the two pieces now meet only across ocean.
    let border: Vec<u8> = map.countries[1]
        .hexes
        .iter()
        .filter(|&&h| map.hexes[h].neighbours.iter().any(|&x| map.hexes[x].country == 0))
        .map(|&h| map.hexes[h].number)
        .collect();
    let p = &mut content.map.countries[1];
    p.cities.retain(|n| !border.contains(n));
    p.ocean.extend(border);
    let flooded = Map::build(&content, 2).unwrap();
    assert!(flooded.fronts.is_empty());
    assert!(flooded.hexes.iter().all(|c| c.kind != HexKind::Base && c.facing.is_none()));
}

// ---------- setup ----------

#[test]
fn setup_places_the_printed_pieces() {
    // Difficulty 4's second corrupted city is a tunable, 0 in the baseline.
    let cases: [(i64, usize, usize, i64, &str); 4] = [
        (0, 1, 9, 20, ""),
        (3, 1, 8, 20, ""),
        (4, 1, 8, 16, ""),
        (4, 2, 8, 16, "adversary.extra_corrupted=1"),
    ];
    for (d, corrupt, deck, supply, set) in cases {
        let level = format!("adversary.difficulty={d}");
        let mut sets = vec![level.as_str()];
        if !set.is_empty() {
            sets.push(set);
        }
        let mut c = baseline_with(&sets);
        c.round = vec![ACTIONS.into()];
        let rules = Rules::new(&c).unwrap();
        let g = rules.new_game(1);
        let m = &rules.map;
        for c in 0..2 {
            let p = piece(c);
            let nums = |f: &dyn Fn(usize) -> bool| -> BTreeSet<u8> {
                m.countries[c].hexes.iter().filter(|&&h| f(h)).map(|&h| m.hexes[h].number).collect()
            };
            let corrupted = nums(&|h| kind(m, h) == HexKind::City && g.tokens(h, ADV_CUBE) > 0);
            assert_eq!(corrupted, p.corrupted[..corrupt].iter().copied().collect(), "d{d} country {c}");
            let stations = nums(&|h| g.tokens(h, STATION) > 0);
            assert_eq!(stations, p.stations.iter().copied().collect());
            assert!(nums(&|h| g.tokens(h, STATION) > 0 && g.tokens(h, ADV_CUBE) == 0).is_empty(), "stations start corrupted");
            assert_eq!(nums(&|h| g.tokens(h, FACTORY) > 0), p.factories.iter().copied().collect());
        }
        for s in 0..2 {
            let want: Vec<usize> = piece(s).data_centres.iter().map(|&n| m.numbered(s, n)).collect();
            let mut have = dcs_of(&g, s);
            have.sort();
            let mut want = want;
            want.sort();
            assert_eq!(have, want, "seat {s}");
            assert_eq!(g.resource(s, AMBASSADORS), 1);
            assert_eq!(g.resource(s, "credits"), 25);
        }
        assert_eq!(total(&g, ADV_AMBASSADOR) + total(&g, TROOP) + total(&g, UNIT), 0);
        assert_eq!(g.var(SUPPLY), supply, "d{d}");
        // The next adversary card is face up, from stage I.
        let next = g.pile(NEXT);
        assert_eq!(next.len(), 1);
        assert!(next[0].starts_with("adv_s1_"), "{next:?}");
        assert_eq!(g.pile(DECK).len(), deck, "d{d}");
        assert!(g.pile(CURRENT).is_empty());
    }
}

// ---------- the adversary round ----------

#[test]
fn the_build_step_grows_corrupted_cities_and_the_factories_next_to_them() {
    let (rules, mut g0) = lab(&["adversary.build"]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).corrupted[0]);
    put(&mut g0, city, ADV_CUBE, 1);
    let next_to_city = |h: usize, c: usize| m.hexes[h].neighbours.contains(&c);
    let clean_city_next = |h: usize| m.hexes[h].neighbours.iter().any(|&x| kind(m, x) == HexKind::City && x != city);
    let open = |h: usize| kind(m, h) == HexKind::Open;
    // A factory next to the corrupted city, and one next only to a clean city.
    let near = hexes_where(m, |h| open(h) && next_to_city(h, city))[0];
    let far = hexes_where(m, |h| open(h) && !next_to_city(h, city) && clean_city_next(h))[0];
    put(&mut g0, near, FACTORY, 1);
    put(&mut g0, far, FACTORY, 1);
    g0.piles.remove(CURRENT);
    let supply = g0.var(SUPPLY);

    let mut g = g0.clone();
    g.piles.insert(NEXT.into(), vec!["adv_s1_probing_strikes".into()]);
    run(&rules, &mut g);
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(near, ADV_AMBASSADOR), g.tokens(far, ADV_AMBASSADOR)), (2, 1, 0));
    assert_eq!(g.var(SUPPLY), supply - 3);
    assert_eq!(g.pile(CURRENT), ["adv_s1_probing_strikes"]);
    assert!(g.pile(NEXT).is_empty());
    assert_eq!(g.var(STAGE), 1);

    // Factory Drive: factories next to clean cities build too.
    let mut g = g0.clone();
    g.piles.insert(NEXT.into(), vec!["adv_s1_factory_drive".into()]);
    run(&rules, &mut g);
    assert_eq!(g.tokens(near, ADV_AMBASSADOR), 1 + clean_city_next(near) as i64);
    assert_eq!(g.tokens(far, ADV_AMBASSADOR), 1);

    // Mobilisation (stage II): corrupted cities build 1 more, and the
    // escalation puts an ambassador on every military base.
    let mut g = g0.clone();
    g.piles.insert(NEXT.into(), vec!["adv_s2_mobilisation".into()]);
    run(&rules, &mut g);
    assert_eq!(g.tokens(city, ADV_AMBASSADOR), 3);
    for f in &m.fronts {
        for &b in &f.bases {
            assert_eq!(g.tokens(b, ADV_AMBASSADOR), 1);
        }
    }
    assert_eq!(g.var(STAGE), 2);
}

#[test]
fn spread_keeps_one_and_sends_the_rest_by_priority() {
    let (rules, mut g0) = lab(&["adversary.spread"]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).corrupted[0]);
    put(&mut g0, city, ADV_CUBE, 1);
    let land = |h: usize| kind(m, h) != HexKind::Ocean;

    // A player-owned piece first, even further than a clean city or empty land.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 2);
    let owned = hexes_where(m, |h| m.distance(h, city) == 2 && kind(m, h) == HexKind::Open)[0];
    put(&mut g, owned, &dc(0), 1);
    run(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(owned, ADV_AMBASSADOR)), (1, 1));

    // Then a clean city, which collects them.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 3);
    let target = cities(m).into_iter().find(|&h| h != city && m.distance(h, city) <= 2).expect("a city near");
    corrupt_cities_except(m, &mut g, &[target]);
    run(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(target, ADV_AMBASSADOR)), (1, 2));
    assert_eq!(total(&g, ADV_AMBASSADOR), 3);

    // Among equals the players choose: here every land hex next to it.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 2);
    let far = cities(m).into_iter().find(|&h| m.distance(h, city) > 2).expect("a city far");
    corrupt_cities_except(m, &mut g, &[far]);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("a spread choice");
    assert!(p.ask.prompt.starts_with("Adversary spread"), "{}", p.ask.prompt);
    let options: BTreeSet<usize> = p.ask.options.iter().map(|o| m.find(o.target.unwrap()).unwrap()).collect();
    let next: BTreeSet<usize> = m.hexes[city].neighbours.iter().copied().filter(|&h| land(h)).collect();
    assert_eq!(options, next);
    let to = *next.iter().last().unwrap();
    answer_hex(&rules, &mut g, to);
    assert!(g.pending.is_none());
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(to, ADV_AMBASSADOR)), (1, 1));

    // A city that is not corrupted holds its ambassadors.
    let mut g = g0.clone();
    let clean = cities(m).into_iter().find(|&h| h != city).unwrap();
    put(&mut g, clean, ADV_AMBASSADOR, 4);
    run(&rules, &mut g);
    assert_eq!(g.tokens(clean, ADV_AMBASSADOR), 4);
    assert_eq!(total(&g, ADV_AMBASSADOR), 4);
}

#[test]
fn a_clean_city_holding_five_ambassadors_is_corrupted() {
    let (rules, mut g) = lab(&["adversary.corrupt"]);
    let m = &rules.map;
    let [a, b, aligned] = [0, 1, 2].map(|i| m.numbered(0, piece(0).cities[i]));
    put(&mut g, a, ADV_AMBASSADOR, 5);
    put(&mut g, b, ADV_AMBASSADOR, 4);
    put(&mut g, aligned, ADV_AMBASSADOR, 5);
    put(&mut g, aligned, &cube(0), 1);
    let supply = g.var(SUPPLY);
    run(&rules, &mut g);
    // Their ambassadors go back to the supply; an aligned city loses its cube.
    assert_eq!((g.tokens(a, ADV_CUBE), g.tokens(a, ADV_AMBASSADOR)), (1, 0));
    assert_eq!((g.tokens(b, ADV_CUBE), g.tokens(b, ADV_AMBASSADOR)), (0, 4));
    assert_eq!((g.tokens(aligned, ADV_CUBE), g.tokens(aligned, &cube(0))), (1, 0));
    assert_eq!(g.var(SUPPLY), supply + 10);
    assert_eq!(g.stats.counters["adversary.corruptions"], 2);
}

#[test]
fn overflow_corrupts_a_city_or_station_already_holding_an_ambassador() {
    for (variant, overflow) in [("militarist", false), ("overflow", true)] {
        let mut c = lab_config(&["adversary.spread", ACTIONS]);
        c.apply_override(&format!("adversary.variant={variant}")).unwrap();
        let rules = Rules::new(&c).unwrap();
        let m = &rules.map;
        let mut g = rules.new_game(1);
        for h in g.hexes.iter_mut() {
            h.tokens.clear();
        }
        let city = m.numbered(0, piece(0).corrupted[0]);
        let target = cities(m).into_iter().find(|&h| m.adjacent(h, city)).expect("a city next to it");
        let far = cities(m).into_iter().find(|&h| m.distance(h, city) > 2).unwrap();
        corrupt_cities_except(m, &mut g, &[target, far]);
        put(&mut g, city, ADV_AMBASSADOR, 2);
        put(&mut g, target, ADV_AMBASSADOR, 1);
        let supply = g.var(SUPPLY);
        run(&rules, &mut g);
        assert!(g.pending.is_none());
        if overflow {
            assert_eq!((g.tokens(target, ADV_CUBE), g.tokens(target, ADV_AMBASSADOR)), (1, 0), "{variant}");
            assert_eq!(g.var(SUPPLY), supply + 2);
        } else {
            assert_eq!((g.tokens(target, ADV_CUBE), g.tokens(target, ADV_AMBASSADOR)), (0, 2), "{variant}");
            assert_eq!(g.var(SUPPLY), supply);
        }
        // A placed ambassador on an owned station already holding one.
        let station = m.numbered(1, piece(1).stations[0]);
        put(&mut g, station, STATION, 1);
        put(&mut g, station, &cube(1), 1);
        put(&mut g, station, ADV_AMBASSADOR, 1);
        with_cx(&rules, |cx| adversary::place(&mut g, cx, station, 1));
        assert_eq!(g.tokens(station, ADV_CUBE), overflow as i64, "{variant}");
        assert_eq!(g.tokens(station, &cube(1)), !overflow as i64, "{variant}");
        assert_eq!(g.tokens(station, ADV_AMBASSADOR), 2 - overflow as i64, "{variant}");
    }
}

#[test]
fn the_players_lose_when_the_next_round_has_no_adversary_card() {
    let rules = Rules::new(&lab_config(&["adversary.build", "adversary.reveal", ACTIONS])).unwrap();
    let mut g = rules.new_game(1);
    // The last card is face up: its round plays, the one after is lost.
    g.piles.remove(DECK);
    g.piles.insert(NEXT.into(), vec!["adv_s3_occupation".into()]);
    run(&rules, &mut g);
    assert!(!g.is_over());
    assert_eq!(g.pile(CURRENT), ["adv_s3_occupation"]);
    assert!(g.pile(NEXT).is_empty());
    let deck = rules.view(&g).globals.into_iter().find(|v| v.key == "adversary.deck").unwrap();
    assert_eq!(deck.value, 0);
    run(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "deck_empty"));
}

#[test]
fn the_players_lose_when_the_supply_starves_or_every_city_is_corrupted() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let city = rules.map.numbered(0, piece(0).corrupted[0]);
    put(&mut g, city, ADV_CUBE, 1);
    g.vars.insert(SUPPLY.into(), 1);
    g.piles.insert(NEXT.into(), vec!["adv_s1_probing_strikes".into()]);
    run(&rules, &mut g);
    assert_eq!(g.tokens(city, ADV_AMBASSADOR), 1);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "supply"));

    let (rules, mut g) = lab(&["adversary.corrupt"]);
    let m = &rules.map;
    let last = cities(m)[0];
    corrupt_cities_except(m, &mut g, &[last]);
    put(&mut g, last, ADV_AMBASSADOR, 4);
    run(&rules, &mut g);
    assert!(!g.is_over());
    put(&mut g, last, ADV_AMBASSADOR, 5);
    run(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "cities"));
}

// ---------- ambassadors ----------

#[test]
fn opposing_ambassadors_on_one_piece_cancel_without_unity() {
    let (rules, g0) = lab(&[]);
    let city = rules.map.numbered(0, piece(0).cities[0]);
    let sway = |g: &GameState| (g.var("alignment.unity"), g.var("alignment.discord"));

    // A player ambassador arrives on an adversary one.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 1);
    let supply = g.var(SUPPLY);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, city));
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(city, &ambassador(0))), (0, 0));
    assert_eq!(g.var(SUPPLY), supply + 1);
    assert_eq!(sway(&g), sway(&g0));

    // An adversary ambassador arrives on a player one.
    let mut g = g0.clone();
    put(&mut g, city, &ambassador(1), 1);
    let supply = g.var(SUPPLY);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, city, 1));
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(city, &ambassador(1))), (0, 0));
    assert_eq!(g.var(SUPPLY), supply);
    assert_eq!(sway(&g), sway(&g0));
}

#[test]
fn a_player_ambassador_takes_a_neutral_station_and_three_free_a_corrupted_city() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let neutral = m.numbered(0, piece(0).stations[0]);
    let corrupted = m.numbered(1, piece(1).stations[0]);
    put(&mut g, neutral, STATION, 1);
    put(&mut g, corrupted, STATION, 1);
    put(&mut g, corrupted, ADV_CUBE, 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, neutral));
    assert_eq!((g.tokens(neutral, &cube(0)), g.tokens(neutral, &ambassador(0))), (1, 0));
    // On a corrupted station it just waits.
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, corrupted));
    assert_eq!((g.tokens(corrupted, &cube(0)), g.tokens(corrupted, &ambassador(0))), (0, 1));

    let city = m.numbered(0, piece(0).corrupted[0]);
    put(&mut g, city, ADV_CUBE, 1);
    for (k, seat) in [0, 1, 0].into_iter().enumerate() {
        with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, seat, city));
        let freed = k == 2;
        assert_eq!(g.tokens(city, ADV_CUBE), !freed as i64, "after {} ambassadors", k + 1);
    }
    assert_eq!(g.tokens(city, &ambassador(0)) + g.tokens(city, &ambassador(1)), 0);
    assert_eq!(g.stats.counters["board.cities_freed"], 1);
}

#[test]
fn ambassadors_divert_bases_and_factories() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let [base, other] = m.fronts[0].bases;
    assert_eq!(m.fronts[0].countries, [0, 1]);
    let factory = m.numbered(0, piece(0).factories[0]);
    put(&mut g0, factory, FACTORY, 1);
    let unity = |g: &GameState| g.var("alignment.unity");
    let discord = |g: &GameState| g.var("alignment.discord");

    // Untouched: each base builds a unit, each factory one for its nearest base.
    let mut g = g0.clone();
    run(&rules, &mut g);
    assert_eq!((g.tokens(base, UNIT), g.tokens(other, UNIT)), (2, 1));

    // An adversary ambassador on a base: no unit, 3 Discord.
    let mut g = g0.clone();
    put(&mut g, base, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(base, UNIT), g.tokens(base, ADV_AMBASSADOR)), (1, 0));
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (3, 0));

    // A player ambassador on a base: no unit, 2 Unity.
    let mut g = g0.clone();
    put(&mut g, base, &ambassador(0), 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(base, UNIT), g.tokens(base, &ambassador(0))), (1, 0));
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 2));

    // An adversary ambassador on a factory: it builds an adversary troop.
    let mut g = g0.clone();
    put(&mut g, factory, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(factory, TROOP), g.tokens(factory, ADV_AMBASSADOR), g.tokens(base, UNIT)), (1, 0, 1));
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 0));

    // A player ambassador on a factory: no unit, 1 Unity.
    let mut g = g0.clone();
    put(&mut g, factory, &ambassador(1), 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(factory, &ambassador(1)), g.tokens(base, UNIT)), (0, 1));
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 1));
}

#[test]
fn ambassadors_divert_cities_stations_and_data_centres() {
    let (rules, g0) = lab(&["board.produce"]);
    let m = &rules.map;
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");
    let res = |g: &GameState, s: usize, r: &str| g.resource(s, r) - g0.resource(s, r);
    let (city, lonely) = city_with_lonely_neighbours(m);
    let open_next = |_: usize| lonely[0];

    // A corrupted city's energy goes to the adversary: 1 Discord a unit.
    let mut g = g0.clone();
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, open_next(city), &dc(0), 1);
    run(&rules, &mut g);
    assert_eq!((discord(&g), res(&g, 0, "energy"), res(&g, 0, "compute")), (1, 0, 1));

    // So does a clean city's with an adversary ambassador on it (which goes).
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 1);
    put(&mut g, open_next(city), &dc(0), 1);
    run(&rules, &mut g);
    assert_eq!((discord(&g), res(&g, 0, "energy"), g.tokens(city, ADV_AMBASSADOR)), (1, 0, 0));

    // And a data centre's compute.
    let mut g = g0.clone();
    let d = m.numbered(0, piece(0).data_centres[0]);
    put(&mut g, d, &dc(0), 1);
    put(&mut g, d, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    assert_eq!((discord(&g), res(&g, 0, "compute"), g.tokens(d, ADV_AMBASSADOR)), (1, 0, 0));

    // An owned station's cube goes to the adversary; a corrupted station's
    // always does; a neutral one makes none.
    let mut g = g0.clone();
    let [s0, s1] = [0, 1].map(|c| m.numbered(c, piece(c).stations[0]));
    let neutral = m.numbered(0, piece(0).factories[0]);
    for s in [s0, s1, neutral] {
        put(&mut g, s, STATION, 1);
    }
    put(&mut g, s0, &cube(0), 1);
    put(&mut g, s0, ADV_AMBASSADOR, 1);
    put(&mut g, s1, ADV_CUBE, 1);
    run(&rules, &mut g);
    // (Round 1 of the new game already put cubes on the projects.)
    let key = |g: &GameState, c: usize, s: Option<usize>| {
        let k = board::project_key(c, s);
        g.var(&k) - g0.var(&k)
    };
    assert_eq!((key(&g, 0, None), key(&g, 0, Some(0)), key(&g, 1, None)), (1, 0, 1));
    assert_eq!(g.tokens(s0, ADV_AMBASSADOR), 0);
    assert_eq!(g.tokens(neutral, ADV_AMBASSADOR), 0);
    let mut g = g0.clone();
    put(&mut g, s0, STATION, 1);
    put(&mut g, s0, &cube(0), 1);
    run(&rules, &mut g);
    assert_eq!((key(&g, 0, None), key(&g, 0, Some(0))), (0, 1));
}

#[test]
fn a_player_ambassador_absorbs_a_destruction() {
    let (rules, mut g) = lab(&["board.strike"]);
    let m = &rules.map;
    let at = m.numbered(0, piece(0).factories[0]);
    for h in std::iter::once(at).chain(m.hexes[at].neighbours.iter().copied()) {
        assert!(g.hexes[h].tokens.is_empty());
    }
    put(&mut g, at, FACTORY, 1);
    put(&mut g, at, TROOP, 1);
    put(&mut g, at, &ambassador(0), 1);
    let discord = g.var("alignment.discord");
    run(&rules, &mut g);
    assert_eq!((g.tokens(at, FACTORY), g.tokens(at, &ambassador(0))), (1, 0));
    assert_eq!(g.var("alignment.discord"), discord);
    run(&rules, &mut g);
    assert_eq!((g.tokens(at, FACTORY), g.tokens(at, TROOP)), (0, 1));
    assert_eq!(g.var("alignment.discord"), discord + 1);
}

// ---------- production ----------

/// A city with at least three open neighbours that touch no other city.
fn city_with_lonely_neighbours(m: &Map) -> (usize, Vec<usize>) {
    for c in cities(m) {
        let lonely: Vec<usize> = m.hexes[c]
            .neighbours
            .iter()
            .copied()
            .filter(|&n| kind(m, n) == HexKind::Open)
            .filter(|&n| m.hexes[n].neighbours.iter().all(|&x| x == c || kind(m, x) != HexKind::City))
            .collect();
        if lonely.len() >= 3 {
            return (c, lonely);
        }
    }
    panic!("no city with three lonely neighbours");
}

#[test]
fn cities_power_the_player_with_the_most_data_centres_next_to_them() {
    let (rules, g0) = lab(&["board.produce"]);
    let (city, n) = city_with_lonely_neighbours(&rules.map);
    let res = |g: &GameState, s: usize, r: &str| g.resource(s, r) - g0.resource(s, r);

    let mut g = g0.clone();
    put(&mut g, n[0], &dc(0), 1);
    put(&mut g, n[1], &dc(1), 1);
    put(&mut g, n[2], &dc(1), 1);
    run(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!((res(&g, 0, "energy"), res(&g, 1, "energy")), (0, 1));
    // Every data centre makes its compute.
    assert_eq!((res(&g, 0, "compute"), res(&g, 1, "compute")), (1, 2));

    // A tie: the players choose.
    let mut g = g0.clone();
    put(&mut g, n[0], &dc(0), 1);
    put(&mut g, n[1], &dc(1), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("an energy choice");
    assert!(p.ask.prompt.starts_with("The energy of"), "{}", p.ask.prompt);
    answer_with(&rules, &mut g, "seat 0");
    assert_eq!((res(&g, 0, "energy"), res(&g, 1, "energy")), (1, 0));

    // An aligned city powers its owner, with the bonus.
    let mut g = g0.clone();
    put(&mut g, city, &cube(1), 1);
    put(&mut g, n[0], &dc(0), 1);
    run(&rules, &mut g);
    assert_eq!((res(&g, 0, "energy"), res(&g, 1, "energy")), (0, 2));
}

#[test]
fn stations_fill_the_megaproject_and_its_completion_pays_out() {
    let (rules, mut g) = lab(&["board.produce"]);
    let m = &rules.map;
    let station = m.numbered(0, piece(0).stations[0]);
    put(&mut g, station, STATION, 1);
    put(&mut g, station, &cube(0), 1);
    let rival = m.numbered(1, piece(1).stations[0]);
    put(&mut g, rival, STATION, 1);
    put(&mut g, rival, ADV_CUBE, 1);
    assert_eq!(piece(0).project.cubes, 7);
    g.vars.insert(board::project_key(0, Some(0)), 3);
    g.vars.insert(board::project_key(0, Some(1)), 1);
    g.vars.insert(board::project_key(0, None), 2);
    g.vars.remove(&board::project_key(1, None));
    let (c0, c1, discord) = (g.resource(0, "credits"), g.resource(1, "credits"), g.var("alignment.discord"));
    run(&rules, &mut g);
    // The 7th cube completes it: 10 credits a player cube, 1 Discord per 2 adversary cubes.
    assert_eq!((g.resource(0, "credits") - c0, g.resource(1, "credits") - c1), (40, 10));
    assert_eq!(g.var("alignment.discord") - discord, 1);
    assert_eq!(g.var(&board::project_done(0)), 1);
    for s in [Some(0), Some(1), None] {
        assert_eq!(g.var(&board::project_key(0, s)), 0);
    }
    assert_eq!(g.var(&board::project_key(1, None)), 1);
    // Afterwards the owned station makes the project's benefit.
    run(&rules, &mut g);
    assert_eq!(g.resource(0, "credits") - c0, 42);
    assert_eq!(g.var(&board::project_key(0, Some(0))), 0);
    assert_eq!(g.var(&board::project_key(1, None)), 2);
}

// ---------- war ----------

#[test]
fn war_rounds_follow_the_schedule_and_the_war_twist() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(1);
    g.piles.remove(CURRENT);
    let war: Vec<u32> = (1..=12).filter(|&r| {
        g.round = r;
        board::war_round(&w, &g)
    }).collect();
    assert_eq!(war, [2, 4, 6, 7, 8, 9, 10, 11, 12]);
    g.round = 1;
    g.piles.insert(CURRENT.into(), vec!["adv_s2_border_war".into()]);
    assert!(board::war_round(&w, &g));
    let v = rules.view(&g);
    assert_eq!(v.globals.iter().find(|x| x.key == "board.war").unwrap().value, 1);
}

#[test]
fn on_a_war_round_units_cancel_and_the_surplus_destroys_rival_buildings() {
    let (rules, mut g0) = lab(&["board.war"]);
    let m = &rules.map;
    let [b0, b1] = m.fronts[0].bases;
    let [f1, f2] = [0, 1].map(|i| m.numbered(1, piece(1).factories[i]));
    let d1 = m.numbered(1, piece(1).data_centres[0]);
    put(&mut g0, d1, &dc(1), 1);
    g0.round = 2;
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");
    let units = |g: &GameState| (g.tokens(b0, UNIT), g.tokens(b1, UNIT));

    // 3 against 1: two cross, and take both factories (not the data centre).
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, f2, FACTORY, 1);
    put(&mut g, b0, UNIT, 3);
    put(&mut g, b1, UNIT, 1);
    run(&rules, &mut g);
    assert_eq!(units(&g), (0, 0));
    assert_eq!((g.tokens(f1, FACTORY), g.tokens(f2, FACTORY), g.tokens(d1, &dc(1))), (0, 0, 1));
    assert_eq!(discord(&g), 2);

    // With no factory left, a data centre; a surplus with nothing to hit is spent.
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b0, UNIT, 3);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f1, FACTORY), g.tokens(d1, &dc(1))), (0, 0));
    assert_eq!((units(&g), discord(&g)), ((0, 0), 2));

    // Hardening absorbs one.
    let mut g = g0.clone();
    for f in [f1, f2] {
        put(&mut g, f, FACTORY, 1);
        put(&mut g, f, HARDEN, 1);
    }
    put(&mut g, b0, UNIT, 2);
    put(&mut g, b1, UNIT, 1);
    run(&rules, &mut g);
    assert_eq!((total(&g, FACTORY), total(&g, HARDEN), discord(&g)), (2, 1, 0));

    // Equal forces cancel; the other way round hits the other country.
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b0, UNIT, 2);
    put(&mut g, b1, UNIT, 2);
    run(&rules, &mut g);
    assert_eq!((units(&g), g.tokens(f1, FACTORY)), ((0, 0), 1));
    let mut g = g0.clone();
    let f0 = m.numbered(0, piece(0).factories[0]);
    put(&mut g, f0, FACTORY, 1);
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b1, UNIT, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f0, FACTORY), g.tokens(f1, FACTORY)), (0, 1));

    // At peace nothing fights.
    let mut g = g0.clone();
    g.round = 1;
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b0, UNIT, 3);
    run(&rules, &mut g);
    assert_eq!((units(&g), g.tokens(f1, FACTORY)), ((3, 0), 1));
}

#[test]
fn bases_hold_at_most_three_units() {
    let (rules, mut g) = lab(&["board.bases"]);
    let m = &rules.map;
    for &f in &piece(0).factories {
        put(&mut g, m.numbered(0, f), FACTORY, 1);
    }
    let [b0, b1] = m.fronts[0].bases;
    run(&rules, &mut g);
    assert_eq!((g.tokens(b0, UNIT), g.tokens(b1, UNIT)), (3, 1));
    for _ in 0..3 {
        run(&rules, &mut g);
    }
    assert_eq!((g.tokens(b0, UNIT), g.tokens(b1, UNIT)), (3, 3));
}

#[test]
fn adversary_troops_strike_buildings_next_to_them() {
    let (rules, mut g0) = lab(&["board.strike"]);
    let m = &rules.map;
    let (city, n) = city_with_lonely_neighbours(m);
    let at = n[0];
    let next: Vec<usize> = m.hexes[at].neighbours.iter().copied().filter(|&x| x != city && kind(m, x) != HexKind::Ocean).collect();
    put(&mut g0, at, TROOP, 1);
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");

    let mut g = g0.clone();
    put(&mut g, next[0], FACTORY, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(next[0], FACTORY), discord(&g), g.tokens(at, TROOP)), (0, 1, 1));

    // Two in reach: the players choose.
    let mut g = g0.clone();
    put(&mut g, next[0], FACTORY, 1);
    put(&mut g, next[1], &dc(0), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("a strike choice");
    assert!(p.ask.prompt.starts_with("Adversary troop"), "{}", p.ask.prompt);
    assert_eq!(p.ask.options.len(), 2);
    let label = p.ask.options.iter().find(|o| o.option.starts_with("dc:0")).unwrap().option.clone();
    answer_with(&rules, &mut g, &label);
    assert_eq!((g.tokens(next[0], FACTORY), g.tokens(next[1], &dc(0)), discord(&g)), (1, 0, 1));
}

// ---------- standard actions ----------

fn standard_offered(rules: &Rules, g: &GameState, seat: u8) -> BTreeSet<String> {
    rules
        .legal_actions(g)
        .into_iter()
        .filter_map(|a| match a {
            Action::Standard { seat: s, act } if s == seat => Some(act),
            _ => None,
        })
        .collect()
}

/// The baseline at its action phase after setup. Every seat has room to
/// grow at setup: free open hexes next to its starting data centres.
fn actions_with_room() -> (Rules, GameState) {
    let rules = Rules::new(&lab_config(&[ACTIONS])).unwrap();
    let g = rules.new_game(1);
    let w = rules.world();
    for s in 0..2 {
        assert!(modules::ai::dc_hexes(&w, &g, s).len() >= 2, "seat {s} has no room to grow at setup");
    }
    (rules, g)
}

#[test]
fn standard_actions_are_offered_only_when_affordable_and_targeted() {
    let (rules, mut g) = actions_with_room();
    let w = rules.world();
    for s in 0..2 {
        set_res(&mut g, s, "energy", 0);
        set_res(&mut g, s, "credits", 0);
        assert!(standard_offered(&rules, &g, s as u8).is_empty(), "seat {s}");
    }
    // Enough for a data centre: placing, moving and aligning (a clean city
    // in reach) are offered; removing needs an adversary ambassador in reach.
    set_res(&mut g, 0, "energy", 2);
    set_res(&mut g, 0, "credits", 4);
    let clean_in_reach = (0..g.hexes.len()).any(|h| {
        modules::common::clean_city(&w, &g, h) && modules::common::in_reach(&w, &g, 0, h)
    });
    let mut want: BTreeSet<String> = ["data_centre", "move"].map(String::from).into();
    if clean_in_reach {
        want.insert("align".into());
    }
    assert_eq!(standard_offered(&rules, &g, 0), want);
    // No ambassadors: no moving or aligning.
    set_res(&mut g, 0, AMBASSADORS, 0);
    assert_eq!(standard_offered(&rules, &g, 0), ["data_centre".to_string()].into());
    set_res(&mut g, 0, AMBASSADORS, 1);

    for s in 0..2 {
        set_res(&mut g, s, "energy", 20);
        set_res(&mut g, s, "credits", 50);
    }
    let o0 = standard_offered(&rules, &g, 0);
    for a in ["data_centre", "factory", "station", "move", "ambassador"] {
        assert!(o0.contains(a), "{a}: {o0:?}");
    }
    assert!(!o0.contains("remove") && !o0.contains("harden"), "{o0:?}");
    let d = dcs_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    assert!(standard_offered(&rules, &g, 0).contains("remove"));
    // Only the Protector (seat 1) may harden.
    assert!(standard_offered(&rules, &g, 1).contains("harden"));
    assert!(!standard_offered(&rules, &g, 0).contains("harden"));
    assert!(rules.apply_mut(&mut g.clone(), &std_act(0, "harden"), None).is_err());
    assert!(rules.apply_mut(&mut g.clone(), &std_act(0, "nope"), None).is_err());
}

#[test]
fn standard_actions_cost_what_the_config_says() {
    let (rules, mut g0) = actions_with_room();
    for s in 0..2 {
        set_res(&mut g0, s, "energy", 20);
        set_res(&mut g0, s, "credits", 50);
    }
    let spent = |g: &GameState, s: usize| {
        (g0.resource(s, "energy") - g.resource(s, "energy"), g0.resource(s, "credits") - g.resource(s, "credits"))
    };

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "factory"));
    assert_eq!((spent(&g, 0), total(&g, FACTORY) - total(&g0, FACTORY)), ((2, 8), 1));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "station"));
    assert_eq!((spent(&g, 0), total(&g, STATION) - total(&g0, STATION)), ((2, 8), 1));
    assert_eq!(total(&g, &cube(0)), 1);

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "data_centre"));
    assert_eq!((spent(&g, 0), dcs_of(&g, 0).len()), ((2, 4), 3));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "ambassador"));
    assert_eq!((spent(&g, 0), g.resource(0, AMBASSADORS)), ((0, 6), 2));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "move"));
    assert_eq!((spent(&g, 0), g.resource(0, AMBASSADORS)), ((1, 0), 0));

    let mut g = g0.clone();
    let d = dcs_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    let supply = g.var(SUPPLY);
    act(&rules, &mut g, std_act(0, "remove"));
    assert_eq!((spent(&g, 0), g.tokens(d, ADV_AMBASSADOR), g.var(SUPPLY)), ((2, 0), 0, supply + 1));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(1, "harden"));
    assert_eq!((spent(&g, 1), total(&g, HARDEN)), ((0, 3), 1));
}

#[test]
fn aligning_puts_a_cube_on_a_clean_city_in_reach() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let (city, next) = city_with_lonely_neighbours(m);
    put(&mut g, next[0], &dc(0), 1);
    set_res(&mut g, 0, "energy", 2);
    set_res(&mut g, 0, AMBASSADORS, 1);
    act(&rules, &mut g, std_act(0, "align"));
    assert_eq!(g.tokens(city, &cube(0)), 1);
    assert_eq!((g.resource(0, "energy"), g.resource(0, AMBASSADORS)), (0, 0));
    // A corrupted or aligned city can't be aligned.
    set_res(&mut g, 0, "energy", 2);
    set_res(&mut g, 0, AMBASSADORS, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("align"));
}

#[test]
fn selling_a_card_discards_it_for_a_credit() {
    let rules = Rules::new(&lab_config(&[ACTIONS])).unwrap();
    let mut g = rules.new_game(1);
    for p in ["cards.deck.1", "cards.discard.1"] {
        if let Some(pile) = g.piles.get_mut(p) {
            pile.retain(|c| c != "fab_line");
        }
    }
    g.piles.insert("cards.hand.0".into(), vec!["turnkey_factory".into(), "fab_line".into()]);
    let sells: Vec<Action> = rules.legal_actions(&g).into_iter().filter(|a| matches!(a, Action::Sell { seat: 0, .. })).collect();
    assert_eq!(sells.len(), 2);
    let credits = g.resource(0, "credits");
    act(&rules, &mut g, Action::Sell { seat: 0, card: "fab_line".into() });
    act(&rules, &mut g, Action::Sell { seat: 0, card: "turnkey_factory".into() });
    assert_eq!(g.resource(0, "credits"), credits + 2);
    assert!(g.pile("cards.hand.0").is_empty());
    // A tier-1 card to its discard pile; an AI's unique card back to the box.
    assert_eq!(g.pile("cards.discard.1")[0], "fab_line");
    assert!(g.piles.values().all(|p| !p.contains(&"turnkey_factory".to_string())));
    assert!(rules.apply_mut(&mut g, &Action::Sell { seat: 0, card: "fab_line".into() }, None).is_err());
}

// ---------- threats ----------

fn threat_lab() -> (Rules, GameState) {
    let rules = Rules::new(&lab_config(&[ACTIONS, "threats.execute"])).unwrap();
    let g = rules.new_game(3);
    (rules, g)
}

/// Puts `threat` on country `c` (out of its deck).
fn deal(g: &mut GameState, threat: &str, c: usize) {
    for t in [1, 2] {
        if let Some(p) = g.piles.get_mut(&format!("threats.deck.{t}")) {
            p.retain(|x| x != threat);
        }
    }
    g.piles.entry(format!("threats.on.{c}")).or_default().push(threat.into());
}

#[test]
fn threats_are_countered_only_when_fully_paid_and_partial_payments_come_back() {
    let (rules, mut g) = threat_lab();
    deal(&mut g, "sabotage", 0);
    deal(&mut g, "blackout", 1);
    for s in 0..2 {
        set_res(&mut g, s, "compute", 5);
        set_res(&mut g, s, "energy", 5);
    }
    let legal = rules.legal_actions(&g);
    // Only players with a data centre in the threat's country may pay.
    let pays = |seat: u8, t: &str| legal.iter().any(|a| matches!(a, Action::Pay { seat: s, threat, .. } if *s == seat && threat == t));
    assert!(pays(0, "sabotage") && !pays(0, "blackout"));
    assert!(pays(1, "blackout") && !pays(1, "sabotage"));
    let pay = |seat, threat: &str, resource: &str, amount| Action::Pay {
        seat,
        threat: threat.into(),
        resource: resource.into(),
        amount,
    };
    // Blackout: 2 compute. Pay 1, withdraw, then pay in full.
    rules.apply_mut(&mut g, &pay(1, "blackout", "compute", 1), None).unwrap();
    assert_eq!(g.resource(1, "compute"), 4);
    let withdraw = Action::Withdraw { seat: 1, threat: "blackout".into(), resource: "compute".into() };
    rules.apply_mut(&mut g, &withdraw, None).unwrap();
    assert_eq!(g.resource(1, "compute"), 5);
    assert!(rules.apply_mut(&mut g, &pay(1, "blackout", "compute", 3), None).is_err());
    assert!(rules.apply_mut(&mut g, &pay(0, "blackout", "compute", 1), None).is_err());
    rules.apply_mut(&mut g, &pay(1, "blackout", "compute", 2), None).unwrap();
    assert!(g.pile("threats.on.1").is_empty());
    assert_eq!(g.pile("threats.discard.1")[0], "blackout");
    assert_eq!(g.resource(1, "compute"), 3);
    // Sabotage: 2 compute + 1 energy. Pay only the compute; it comes back.
    rules.apply_mut(&mut g, &pay(0, "sabotage", "compute", 2), None).unwrap();
    assert_eq!(g.pile("threats.on.0"), ["sabotage"]);
    let discord = g.var("alignment.discord");
    for s in 0..2 {
        rules.apply_mut(&mut g, &Action::Pass { seat: s }, None).unwrap();
    }
    assert!(g.pending.is_none());
    assert_eq!(g.resource(0, "compute"), 5);
    assert!(g.pile("threats.discard.1").contains(&"sabotage".to_string()));
    assert!(board::flagged(&g, "sabotage", 0));
    assert_eq!(g.var("alignment.discord"), discord + 1);
}

#[test]
fn jamming_a_threat_down_to_what_is_paid_counters_it() {
    let (rules, mut g) = threat_lab();
    // Requisition needs 1 insight; Jam Signals takes it to 0: countered at once.
    deal(&mut g, "requisition", 0);
    g.piles.get_mut("cards.deck.1").unwrap().retain(|c| c != "jam_signals");
    g.piles.insert("cards.hand.0".into(), vec!["jam_signals".into()]);
    set_res(&mut g, 0, "credits", 10);
    act(&rules, &mut g, Action::Play { seat: 0, card: "jam_signals".into() });
    assert!(g.pile("threats.on.0").is_empty());
    assert!(g.pile("threats.discard.1").contains(&"requisition".to_string()));
    assert_eq!(g.resource(0, "credits"), 7);
}

#[test]
fn difficulty_1_raises_the_counters_and_deals_more_threats() {
    let sets = ["adversary.counter_bonus=1", "adversary.counter_bonus_all=1", "adversary.extra_threats=1"];
    let at = |d: i64| {
        let mut c = lab_config(&["threats.deal", ACTIONS]);
        for s in sets.iter().map(|s| s.to_string()).chain([format!("adversary.difficulty={d}")]) {
            c.apply_override(&s).unwrap();
        }
        Rules::new(&c).unwrap()
    };
    let (r0, r1) = (at(0), at(1));
    let (g0, mut g1) = (r0.new_game(1), r1.new_game(1));
    let req = |r: &Rules, g: &GameState| threats::requirement(&r.world(), g, "orbital_strike");
    let pair = |e, c| vec![("energy".to_string(), e), ("credits".to_string(), c)];
    assert_eq!(req(&r0, &g0), pair(2, 2));
    assert_eq!(req(&r1, &g1), pair(4, 3));
    let on = |g: &GameState, c: usize| threats::active(g).iter().filter(|(_, at)| *at == Some(c)).count();
    assert_eq!((on(&g0, 0), on(&g0, 1)), (1, 1));
    // One extra a round, one country after the other.
    assert_eq!((on(&g1, 0), on(&g1, 1)), (2, 1));
    for s in 0..2 {
        r1.apply_mut(&mut g1, &Action::Pass { seat: s }, None).unwrap();
    }
    assert_eq!(g1.round, 2);
    assert_eq!((on(&g1, 0), on(&g1, 1)), (3, 3));
}

// ---------- Alignment and the end of the game ----------

/// Stacks the Unity deck: `card` is drawn next, whatever the tier.
fn next_unity(g: &mut GameState, card: &str) {
    for tier in [1, 2] {
        g.piles.insert(format!("alignment.deck.unity.{tier}"), vec![card.to_string()]);
    }
}

/// The baseline playing only the action phase, after setup (no build yet).
fn action_only(sets: &[&str]) -> (Rules, GameState) {
    let mut c = baseline_with(sets);
    c.round = vec![ACTIONS.into()];
    let rules = Rules::new(&c).unwrap();
    let g = rules.new_game(4);
    (rules, g)
}

/// Gives 1 insight from seat 0 to seat 1: an action, so the end checks run.
fn give(rules: &Rules, g: &mut GameState) {
    set_res(g, 0, "insight", 1);
    act(rules, g, Action::Give { seat: 0, to: 1 });
}

fn uncorrupt_cities(rules: &Rules, g: &mut GameState) {
    for h in cities(&rules.map) {
        put(g, h, ADV_CUBE, 0);
    }
}

#[test]
fn a_full_unity_bucket_tips_a_card_and_lowers_the_bar() {
    let (rules, mut g) = action_only(&[]);
    let m = &rules.map;
    // One Unity tip made, the bucket full (2 a player, 2 players); no city
    // corrupted but adversary ambassadors about. The next card is Civil
    // Defense (pushback).
    uncorrupt_cities(&rules, &mut g);
    for c in 0..2 {
        put(&mut g, m.numbered(c, piece(c).cities[1]), ADV_AMBASSADOR, 2);
    }
    g.vars.insert(STAGE.into(), 1);
    g.vars.insert("alignment.unity_tips".into(), 1);
    let undecided = g.var("alignment.undecided");
    g.vars.insert("alignment.undecided".into(), undecided - 4);
    g.vars.insert("alignment.unity".into(), 4);
    next_unity(&mut g, "civil_defense");
    give(&rules, &mut g);
    // The tip: each country removes an ambassador.
    assert_eq!(total(&g, ADV_AMBASSADOR), 2);
    assert_eq!(g.var("alignment.unity_tips"), 2);
    assert_eq!(g.var("alignment.unity"), 0);
    assert_eq!(g.var("alignment.undecided"), undecided);
    assert_eq!(g.pile("alignment.resolved.unity"), ["civil_defense"]);
    // Two Unity tips: bar 1, where ambassadors no longer count.
    assert_eq!(alignment::bar_level(&rules.world(), &g), 1);
    let o = g.outcome.as_ref().expect("a win");
    assert!(o.won && o.reason == "purge", "{o:?}");
}

#[test]
fn bar_levels_follow_the_unity_tips() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(1);
    for (tips, level) in [(0, 0), (1, 0), (2, 1), (4, 1), (5, 2), (9, 2)] {
        g.vars.insert("alignment.unity_tips".into(), tips);
        assert_eq!(alignment::bar_level(&w, &g), level, "{tips} tips");
    }
    assert!(adversary::win_text(&w, 0).contains("no adversary ambassador or troop"));
    assert!(adversary::win_text(&w, 1).contains("no adversary troop"));
    assert_eq!(adversary::win_text(&w, 2), "win when no city is corrupted");
}

#[test]
fn the_board_clear_win_and_the_bar_easing() {
    // (Unity tips, clear_win_stage, this round's stage, a piece left, wins)
    let cases: &[(i64, i64, i64, Option<&str>, bool)] = &[
        (0, 1, 1, None, true),
        (0, 1, 0, None, false),
        (0, 2, 1, None, false),
        (0, 2, 2, None, true),
        (0, 1, 1, Some(ADV_AMBASSADOR), false),
        (2, 1, 1, Some(ADV_AMBASSADOR), true),
        (2, 1, 1, Some(TROOP), false),
        (5, 1, 1, Some(TROOP), true),
        (5, 1, 1, Some(ADV_CUBE), false),
    ];
    for &(tips, rule, stage, piece_left, wins) in cases {
        let (rules, mut g) = action_only(&[&format!("alignment.clear_win_stage={rule}")]);
        uncorrupt_cities(&rules, &mut g);
        g.vars.insert(STAGE.into(), stage);
        g.vars.insert("alignment.unity_tips".into(), tips);
        if let Some(p) = piece_left {
            put(&mut g, cities(&rules.map)[0], p, 1);
        }
        give(&rules, &mut g);
        let won = g.outcome.as_ref().is_some_and(|o| o.won && o.reason == "purge");
        assert_eq!(won, wins, "{tips} tips, rule {rule}, stage {stage}, {piece_left:?}");
        if !wins {
            // The view says when the clear-board win starts to count.
            let bar = rules.view(&g).globals.into_iter().find(|v| v.key == "alignment.bar").unwrap();
            assert_eq!(bar.doc.contains("from adversary stage"), tips < 2 && stage < rule, "{}", bar.doc);
        }
    }
}

#[test]
fn eight_unity_tips_win_outright() {
    let (rules, mut g) = action_only(&[]);
    // Corrupted cities stay: only the outright win can end it.
    g.vars.insert("alignment.unity_tips".into(), 7);
    let undecided = g.var("alignment.undecided");
    g.vars.insert("alignment.undecided".into(), undecided - 4);
    g.vars.insert("alignment.unity".into(), 4);
    next_unity(&mut g, "grassroots_funding");
    give(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a win");
    assert_eq!((o.won, o.reason.as_str()), (true, "unity"));
    // Off at 0.
    let (rules, mut g) = action_only(&["alignment.unity_win=0"]);
    g.vars.insert("alignment.unity_tips".into(), 20);
    give(&rules, &mut g);
    assert!(!g.is_over());
}

/// The antimatter action sways toward Discord; with the Undecided empty
/// the Sway comes from the Unity bucket (`tug`) or nowhere (`still`).
#[test]
fn the_antimatter_action_and_the_empty_undecided() {
    for (variant, from_unity) in [("tug", true), ("still", false)] {
        let (rules, mut g) = action_only(&[&format!("alignment.variant={variant}")]);
        g.vars.insert("alignment.undecided".into(), 0);
        g.vars.insert("alignment.unity".into(), 2);
        g.vars.insert("alignment.discord".into(), 0);
        let am = g.resource(0, "antimatter");
        let a = Action::Antimatter { seat: 0 };
        assert!(rules.legal_actions(&g).contains(&a));
        rules.apply_mut(&mut g, &a, None).unwrap();
        assert_eq!(g.resource(0, "antimatter"), am + 1);
        let moved = i64::from(from_unity);
        assert_eq!((g.var("alignment.unity"), g.var("alignment.discord")), (2 - moved, moved), "{variant}");
        // Once a round per player.
        assert!(!rules.legal_actions(&g).contains(&a));
        assert!(rules.apply_mut(&mut g, &a, None).is_err());
        assert!(rules.legal_actions(&g).contains(&Action::Antimatter { seat: 1 }));
    }
    let (rules, g) = action_only(&["alignment.antimatter_discord=0"]);
    assert!(rules.legal_actions(&g).iter().all(|a| !matches!(a, Action::Antimatter { .. })));
}

/// Discord tips escalate the adversary at 2 and 5; Momentum shrinks a
/// bucket by 1 per 2 of its side's tips, down to 1 per player.
#[test]
fn discord_tips_escalate_and_momentum_shrinks_buckets() {
    let (rules, mut g) = action_only(&[]);
    g.vars.insert("alignment.discord_tips".into(), 1);
    let undecided = g.var("alignment.undecided");
    g.vars.insert("alignment.undecided".into(), undecided - 4);
    g.vars.insert("alignment.discord".into(), 4);
    give(&rules, &mut g);
    assert_eq!(g.var("alignment.discord_tips"), 2);
    assert_eq!(g.var("adversary.escalation"), 1);
    let w = rules.world();
    assert!(adversary::at(&w, &g, 1) && !adversary::level(&w, 1));
    assert_eq!(g.pile("alignment.resolved.discord").len() + g.pile("alignment.ongoing").len(), 1);
    let d = rules.view(&g).globals.into_iter().find(|v| v.key == "adversary.difficulty").unwrap();
    assert_eq!(d.value, 1);

    use crate::content::Side;
    let rules = Rules::new(&baseline_with(&["alignment.momentum_every=2"])).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(4);
    for (tips, cap) in [(0, 4), (1, 4), (2, 3), (4, 2), (9, 2)] {
        g.vars.insert("alignment.unity_tips".into(), tips);
        assert_eq!(alignment::capacity(&w, &g, Side::Unity), cap, "{tips} tips");
        assert_eq!(alignment::capacity(&w, &g, Side::Discord), 4);
    }
}

// ---------- cards ----------

#[test]
fn the_ais_spend_their_resource_as_credits_on_cards() {
    let rules = Rules::new(&lab_config(&[ACTIONS])).unwrap();
    let mut g = rules.new_game(1);
    for c in ["fab_line", "recon_sweep", "evacuate"] {
        g.piles.get_mut("cards.deck.1").unwrap().retain(|x| x != c);
    }
    // The Builder (seat 0): energy on buildings, 1 credit each, after
    // Economies of scale takes 2 off. Fab Line costs 6: with 3 credits it
    // spends 3 to 6 energy, the players choose.
    g.piles.insert("cards.hand.0".into(), vec!["fab_line".into(), "recon_sweep".into()]);
    set_res(&mut g, 0, "credits", 3);
    set_res(&mut g, 0, "energy", 10);
    set_res(&mut g, 0, "insight", 1);
    let plays = |g: &GameState, seat: u8| -> Vec<String> {
        rules
            .legal_actions(g)
            .into_iter()
            .filter_map(|a| match a {
                Action::Play { seat: s, card } if s == seat => Some(card),
                _ => None,
            })
            .collect()
    };
    // Not on a program: with no credits only the building is playable.
    set_res(&mut g, 0, "credits", 0);
    assert_eq!(plays(&g, 0), ["fab_line"]);
    set_res(&mut g, 0, "credits", 3);
    rules.apply_mut(&mut g, &Action::Play { seat: 0, card: "fab_line".into() }, None).unwrap();
    let p = g.pending.as_ref().expect("a discount choice");
    let opts: Vec<&str> = p.ask.options.iter().map(|o| o.option.as_str()).collect();
    assert_eq!(opts, ["spend 3 energy", "spend 4 energy", "spend 5 energy", "spend 6 energy"]);
    answer_with(&rules, &mut g, "spend 4 energy");
    assert_eq!((g.resource(0, "energy"), g.resource(0, "credits")), (6, 1));
    assert!(g.pile("cards.built.0").contains(&"fab_line".to_string()));

    // The Protector (seat 1): compute on cyber cards. Recon Sweep costs 2:
    // with no credits both units must be compute, so nothing is asked.
    g.piles.insert("cards.hand.1".into(), vec!["recon_sweep".into(), "evacuate".into()]);
    set_res(&mut g, 1, "credits", 0);
    set_res(&mut g, 1, "compute", 5);
    set_res(&mut g, 1, "insight", 1);
    assert_eq!(plays(&g, 1), ["recon_sweep"]);
    act(&rules, &mut g, Action::Play { seat: 1, card: "recon_sweep".into() });
    // 2 spent, 1 gained by the card.
    assert_eq!((g.resource(1, "compute"), g.resource(1, "credits")), (4, 0));
}

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
}

#[test]
fn every_card_is_unique_in_every_game() {
    let rules = Rules::new(&baseline()).unwrap();
    for seed in 0..4 {
        let (_, actions) = random_game(&rules, seed);
        let mut g = rules.new_game(seed);
        for a in &actions {
            rules.apply_mut(&mut g, a, None).unwrap();
            let mut seen = BTreeSet::new();
            for (k, pile) in &g.piles {
                for id in pile {
                    assert!(seen.insert(id.clone()), "{id} twice (in {k})");
                }
            }
        }
    }
}

#[test]
fn off_variants_remove_threats_alignment_and_the_draft() {
    let rules = Rules::new(&baseline_with(&["threats.variant=off"])).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    assert!(threats::active(&g).is_empty());

    let rules = Rules::new(&baseline_with(&["alignment.variant=off"])).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    assert_eq!((g.var("alignment.unity"), g.var("alignment.discord")), (0, 0));
    assert!(rules.legal_actions(&g).iter().all(|a| !matches!(a, Action::Antimatter { .. })));
    g.vars.insert("alignment.unity".into(), 4);
    give(&rules, &mut g);
    assert_eq!(g.var("alignment.unity_tips"), 0);

    let rules = Rules::new(&baseline_with(&["cards.variant=off"])).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    // Only the AIs' unique cards: nothing dealt, drafted or drawable.
    assert!(g.pile("cards.hand.0").len() <= 3);
    assert!(rules.legal_actions(&g).iter().all(|a| !matches!(a, Action::Draw { .. })));
}

// ---------- determinism ----------

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
fn every_country_count_plays_to_an_end_and_replays() {
    for n in 1..=4u8 {
        let mut c = baseline();
        c.countries = n;
        let rules = Rules::new(&c).unwrap();
        for seed in 0..4 {
            let (g, actions) = random_game(&rules, seed);
            let o = g.outcome.as_ref().expect("the game ends");
            assert_ne!(o.reason, "round_limit", "{n} countries, seed {seed}: {o:?}");
            let again = replay(&rules, seed, &actions).unwrap();
            assert_eq!(again.fingerprint(), g.fingerprint());
        }
    }
}

/// Fast lane: one seed per variant, the JSON path on every fourth config;
/// the slow lane plays more seeds, all through the JSON path.
#[test]
fn random_games_replay_exactly_in_every_variant() {
    random_games_replay_in_every_variant(0..1, 4);
}

#[test]
#[ignore = "slow lane: more seeds of the variant replay check, all through the JSON path"]
fn random_games_replay_exactly_in_every_variant_more_seeds() {
    random_games_replay_in_every_variant(1..6, 1);
}

fn random_games_replay_in_every_variant(seeds: std::ops::Range<u64>, json_every: usize) {
    for (i, config) in variant_matrix().into_iter().enumerate() {
        let rules = Rules::new(&config).unwrap();
        for seed in seeds.clone() {
            let (g, actions) = random_game(&rules, seed);
            assert!(g.is_over(), "{} seed {seed} did not finish", config.hash());
            assert_eq!(replay(&rules, seed, &actions).unwrap(), g);
            if i % json_every != 0 {
                continue;
            }
            // The slow, JSON-shaped path agrees with the hot path, and a
            // state survives serialization at any point (mid-choice too).
            let mut s = new_game(&config, seed).unwrap();
            for (k, a) in actions.iter().enumerate() {
                if k % 5 == 0 {
                    s = serde_json::from_str(&s.to_json()).unwrap();
                }
                s = apply(&s, a).unwrap().0;
            }
            assert_eq!(s, g);
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
fn rng_draws_are_fixed_width_and_pinned() {
    // Pinned so a native/wasm or rand-version difference shows up here.
    let mut s = rng::Stream::new(42, "spread");
    let draws: Vec<usize> = (0..8).map(|_| s.below(16)).collect();
    assert_eq!(draws, GOLDEN_DRAWS);
}

/// A state waiting on a spread choice (players choose among equals).
fn spread_choice() -> (Rules, GameState) {
    let (rules, mut g) = lab(&["adversary.spread"]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).corrupted[0]);
    let far = cities(m).into_iter().find(|&h| m.distance(h, city) > 2).unwrap();
    corrupt_cities_except(m, &mut g, &[far]);
    put(&mut g, city, ADV_AMBASSADOR, 2);
    run(&rules, &mut g);
    assert!(g.pending.is_some());
    (rules, g)
}

#[test]
fn malformed_and_mismatched_states_are_errors_not_panics() {
    let (rules, g) = spread_choice();
    assert!(rules.check_state(&g).is_ok());
    // Mid-choice states survive JSON, and only answers are legal.
    let back: GameState = serde_json::from_str(&g.to_json()).unwrap();
    assert_eq!(back, g);
    assert!(rules.legal_actions(&g).iter().all(|a| matches!(a, Action::Choose { .. })));
    assert!(rules.apply_mut(&mut g.clone(), &Action::Pass { seat: 0 }, None).is_err());
    let mut bad = g.clone();
    std::sync::Arc::make_mut(&mut bad.pending.as_mut().unwrap().snapshot).seats.pop();
    assert!(rules.check_state(&bad).is_err());
    let mut bad = g.clone();
    bad.pending.as_mut().unwrap().ask.seat = 9;
    assert!(rules.check_state(&bad).is_err());
    let mut bad = g.clone();
    bad.pending.as_mut().unwrap().ask.options.truncate(1);
    assert!(rules.check_state(&bad).is_err());

    let config = baseline();
    let mut g = new_game(&config, 1).unwrap();
    g.rng.clear();
    assert!(legal_actions(&g).err().unwrap().contains("RNG streams"));
    let mut g = new_game(&config, 1).unwrap();
    g.seats.pop();
    assert!(apply(&g, &Action::Pass { seat: 0 }).is_err());
    let mut g = new_game(&config, 1).unwrap();
    g.hexes.pop();
    assert!(view(&g).is_err());
    let other = Rules::new(&baseline_with(&["countries=3"])).unwrap();
    assert!(other.check_state(&new_game(&config, 1).unwrap()).is_err());
}

#[test]
fn saved_games_round_trip_and_replay_to_the_saved_state() {
    let config = baseline_with(&["threats.variant=shared_row"]);
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
        let (h, warnings) = SavedGame::from_json(&minimal.to_string()).unwrap().replay().unwrap();
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
    assert!(SavedGame::from_json(&other).err().unwrap().contains("format"));
    let empty = SavedGame::new(&rules, 1, vec![], &rules.new_game(1), "");
    assert_eq!(SavedGame::from_json(&empty.to_text()).unwrap(), empty);
}

// ---------- the view ----------

#[test]
fn the_view_shows_the_board_the_fronts_the_projects_and_the_next_card() {
    let rules = Rules::new(&baseline()).unwrap();
    let m = &rules.map;
    // A seed whose opening stops at the draft, not at a spread choice
    // before next round's card is revealed.
    let g = (0..64).map(|s| rules.new_game(s)).find(|g| g.pending.is_none()).unwrap();
    let v = rules.view(&g);
    assert_eq!(v.hexes.len(), g.hexes.len());
    let count = |k: &str| v.hexes.iter().filter(|h| h.kind == k).count();
    assert_eq!(count("city"), 6);
    assert_eq!(count("base"), 2 * m.fronts.len());
    assert_eq!(count("ocean"), piece(0).ocean.len() + piece(1).ocean.len());
    assert_eq!(count("city") + count("base") + count("open") + count("ocean"), 32);
    for h in &v.hexes {
        assert!((1..=16).contains(&h.number));
        assert_eq!(h.facing.is_some(), h.kind == "base");
        // Tokens as in the state, non-zero and sorted.
        let want: Vec<(String, i64)> = g.hexes[h.index].tokens.iter().map(|(k, &n)| (k.clone(), n)).collect();
        let have: Vec<(String, i64)> = h.tokens.iter().map(|t| (t.kind.clone(), t.count)).collect();
        assert_eq!(have, want);
    }
    let corrupted = m.numbered(0, piece(0).corrupted[0]);
    assert!(v.hexes[corrupted].tokens.iter().any(|t| t.kind == ADV_CUBE));
    let d = m.numbered(1, piece(1).data_centres[0]);
    assert!(v.hexes[d].tokens.iter().any(|t| t.kind == dc(1)));

    let global = |k: &str| v.globals.iter().find(|x| x.key == k).unwrap_or_else(|| panic!("{k} missing"));
    assert_eq!(global("board.fronts").value, m.fronts.len() as i64);
    assert!(global("board.fronts").doc.contains("war rounds 2, 4, 6, 7, 8, 9"));
    assert_eq!(global("board.war").value, 0);
    let supply = global("adversary.supply");
    assert_eq!((supply.value, supply.max), (g.var(SUPPLY), Some(20)));
    assert_eq!(global("adversary.deck").value, (g.pile(DECK).len() + 1) as i64);
    let next = v.piles.iter().find(|p| p.key == "adversary.next").unwrap();
    assert_eq!(next.count, 1);
    assert_eq!(next.visible[0].id, g.pile(NEXT)[0]);
    assert_eq!(next.visible[0].status, "face up: next round");
    let deck = v.piles.iter().find(|p| p.key == "adversary.deck").unwrap();
    assert!(deck.visible.is_empty() && deck.count == g.pile(DECK).len());

    for (c, cv) in v.countries.iter().enumerate() {
        let value = |k: &str| cv.values.iter().find(|x| x.key == k).unwrap_or_else(|| panic!("{k} missing"));
        let p = value("board.project");
        assert_eq!(p.max, Some(piece(c).project.cubes));
        assert!(p.label.contains(&piece(c).project.name));
        assert_eq!(value("board.project.adversary").value, g.var(&board::project_key(c, None)));
        assert_eq!(value("tokens.factory").value, 2);
    }
    for s in &v.seats {
        let value = |k: &str| s.values.iter().find(|x| x.key == k).unwrap_or_else(|| panic!("{k} missing"));
        assert_eq!(value("board.data_centres").value, 2);
        assert_eq!(value("resource.ambassadors").value, g.resource(s.index as usize, AMBASSADORS));
    }
}

#[test]
fn the_view_follows_every_variant_and_keeps_bookkeeping_out_of_globals() {
    // Per-seat, per-country and per-card `vars`: never game-wide globals.
    const BOOKKEEPING: &[&str] = &[
        "ai.track.",
        "adversary.peek",
        "adversary.starved",
        "adversary.first_corruption",
        "adversary.removed_troop.",
        "alignment.used.",
        "board.blackout.",
        "board.sabotage.",
        "board.shelter.",
        "board.project.",
        "board.project_done.",
        "cards.tier.",
        "cards.picked.",
        "cards.next_discount.",
        "cards.civic_used.",
        "cards.unpowered.",
        "cards.kept_round.",
        "economy.bonus_insight",
        "threats.paid.",
        "threats.jam.",
    ];
    let names: Vec<&str> = crate::content().map.countries.iter().map(|c| c.name.as_str()).collect();
    let (mut saw_track_var, mut saw_project, mut saw_choice) = (false, false, false);
    for config in variant_matrix() {
        let rules = Rules::new(&config).unwrap();
        let shared = config.module("threats").unwrap().variant == "shared_row";
        let mut g = rules.new_game(5);
        for step in 0..300 {
            if g.is_over() {
                break;
            }
            let v = rules.view(&g);
            assert_eq!(v.hexes.len(), g.hexes.len());
            assert_eq!(v.choice.is_some(), g.pending.is_some());
            saw_choice |= v.choice.is_some();
            for (c, cv) in v.countries.iter().enumerate() {
                assert_eq!(cv.name, names[c]);
                assert!(cv.values.iter().any(|x| x.key == "board.project"));
            }
            for s in &v.seats {
                let b = s.board.as_ref().expect("every seat shows its AI board");
                assert_eq!((b.key.as_str(), b.tracks.len()), ("ai.board", 2));
                for t in &b.tracks {
                    assert!((1..=t.steps.len()).contains(&t.uncovered));
                    assert_eq!(t.current, t.uncovered - 1);
                }
                assert!(s.cards.iter().any(|c| c.key == "cards.hand"));
            }
            let threats_shown: usize = if shared {
                v.piles.iter().find(|p| p.key == "threats.row").map_or(0, |p| p.count)
            } else {
                v.countries.iter().flat_map(|c| &c.cards).map(|c| c.cards.len()).sum()
            };
            assert_eq!(threats_shown, threats::active(&g).len());
            for x in &v.globals {
                let claimed = BOOKKEEPING.iter().any(|p| if p.ends_with('.') { x.key.starts_with(p) } else { x.key == *p });
                assert!(!claimed || x.source != "engine", "bookkeeping {} in globals", x.key);
            }
            for k in ["alignment.undecided", "alignment.unity_tips", "alignment.bar", "adversary.supply", "adversary.deck", "board.war", "board.fronts"] {
                assert!(v.globals.iter().any(|x| x.key == k), "{k}");
            }
            assert!(v.piles.iter().any(|p| p.key == "adversary.next"));
            saw_track_var |= g.vars.keys().any(|k| k.starts_with("ai.track."));
            saw_project |= g.vars.keys().any(|k| k.starts_with("board.project."));
            let legal = rules.legal_actions(&g);
            let a = legal[step * 7919 % legal.len()].clone();
            rules.apply_mut(&mut g, &a, None).unwrap();
        }
    }
    // The games really did hold the bookkeeping the view hides.
    assert!(saw_track_var && saw_project && saw_choice);
}

// ---------- the golden game ----------

const GOLDEN_DRAWS: [usize; 8] = [10, 13, 7, 2, 12, 9, 1, 7];
const GOLDEN_FINGERPRINT: &str = "a26ae448b79bd0ee";

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
    assert_eq!((RULES_REVISION, g.fingerprint().as_str()), (6, GOLDEN_FINGERPRINT));
}
