use super::*;
use crate::modules::common::{HARDWARE, NODE};
use crate::modules::{adversary, board};

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
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(city, &ambassador(0))
        ),
        (0, 0)
    );
    assert_eq!(g.var(SUPPLY), supply + 1);
    assert_eq!(sway(&g), sway(&g0));

    // An adversary ambassador arrives on a player one.
    let mut g = g0.clone();
    put(&mut g, city, &ambassador(1), 1);
    let supply = g.var(SUPPLY);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, city, 1));
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(city, &ambassador(1))
        ),
        (0, 0)
    );
    assert_eq!(g.var(SUPPLY), supply);
    assert_eq!(sway(&g), sway(&g0));
}

#[test]
fn a_player_ambassador_takes_a_neutral_station_and_two_free_a_corrupted_city() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let neutral = m.numbered(0, piece(0).stations[0]);
    let corrupted = m.numbered(1, piece(1).stations[0]);
    put(&mut g, neutral, STATION, 1);
    put(&mut g, corrupted, STATION, 1);
    put(&mut g, corrupted, ADV_CUBE, 1);
    put(&mut g, neutral, &presence(0), 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, neutral));
    assert_eq!(
        (
            g.tokens(neutral, &cube(0)),
            g.tokens(neutral, &ambassador(0))
        ),
        (1, 0)
    );
    // On a corrupted station it just waits.
    with_cx(&rules, |cx| {
        board::place_ambassador(&mut g, cx, 0, corrupted)
    });
    assert_eq!(
        (
            g.tokens(corrupted, &cube(0)),
            g.tokens(corrupted, &ambassador(0))
        ),
        (0, 1)
    );

    let city = m.numbered(0, piece(0).corrupted[0]);
    put(&mut g, city, ADV_CUBE, 1);
    assert_eq!(rules.world().int("board.free_city_at"), 2);
    for (k, seat) in [0, 1].into_iter().enumerate() {
        with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, seat, city));
        let freed = k == 1;
        assert_eq!(
            g.tokens(city, ADV_CUBE),
            !freed as i64,
            "after {} ambassadors",
            k + 1
        );
    }
    assert_eq!(
        g.tokens(city, &ambassador(0)) + g.tokens(city, &ambassador(1)),
        0
    );
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
    assert_eq!(
        (g.tokens(base, UNIT), g.tokens(base, ADV_AMBASSADOR)),
        (1, 0)
    );
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (3, 0));

    // A player ambassador on a base: no unit, 2 Unity.
    let mut g = g0.clone();
    put(&mut g, base, &ambassador(0), 1);
    run(&rules, &mut g);
    assert_eq!(
        (g.tokens(base, UNIT), g.tokens(base, &ambassador(0))),
        (1, 0)
    );
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 2));

    // An adversary ambassador on a factory: it builds an adversary troop.
    let mut g = g0.clone();
    put(&mut g, factory, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    assert_eq!(
        (
            g.tokens(factory, TROOP),
            g.tokens(factory, ADV_AMBASSADOR),
            g.tokens(base, UNIT)
        ),
        (1, 0, 1)
    );
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 0));

    // A player ambassador on a factory: no unit, 1 Unity.
    let mut g = g0.clone();
    put(&mut g, factory, &ambassador(1), 1);
    run(&rules, &mut g);
    assert_eq!(
        (g.tokens(factory, &ambassador(1)), g.tokens(base, UNIT)),
        (0, 1)
    );
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 1));
}

#[test]
fn ambassadors_divert_cities_and_stations_and_leak_presence_compute() {
    let (rules, g0) = lab(&["board.produce"]);
    let m = &rules.map;
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");
    let res = |g: &GameState, s: usize, r: &str| g.resource(s, r) - g0.resource(s, r);
    let (city, lonely) = city_with_lonely_neighbours(m);
    let open_next = |_: usize| lonely[0];

    // A corrupted city's energy goes to the adversary: 1 Discord a unit.
    let mut g = g0.clone();
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, open_next(city), &presence(0), 1);
    run(&rules, &mut g);
    assert_eq!(
        (discord(&g), res(&g, 0, "energy"), res(&g, 0, "compute")),
        (1, 0, 0)
    );

    // So does a clean city's with an adversary ambassador on it (which goes).
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 1);
    put(&mut g, open_next(city), &presence(0), 1);
    run(&rules, &mut g);
    assert_eq!(
        (
            discord(&g),
            res(&g, 0, "energy"),
            g.tokens(city, ADV_AMBASSADOR)
        ),
        (1, 0, 0)
    );

    // An adversary ambassador on a presence leaks 1 of its owner's compute.
    let mut g = g0.clone();
    let d = m.numbered(0, piece(0).presence);
    put(&mut g, d, &presence(0), 1);
    put(&mut g, d, ADV_AMBASSADOR, 1);
    set_res(&mut g, 0, "compute", g0.resource(0, "compute") + 2);
    run(&rules, &mut g);
    assert_eq!(
        (
            discord(&g),
            res(&g, 0, "compute"),
            g.tokens(d, ADV_AMBASSADOR)
        ),
        (1, 1, 0)
    );

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
    assert_eq!(
        (key(&g, 0, None), key(&g, 0, Some(0)), key(&g, 1, None)),
        (1, 0, 1)
    );
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
    // The ambassador absorbs the strike; the troop that struck is used up.
    run(&rules, &mut g);
    assert_eq!(
        (
            g.tokens(at, FACTORY),
            g.tokens(at, &ambassador(0)),
            g.tokens(at, TROOP)
        ),
        (1, 0, 0)
    );
    assert_eq!(g.var("alignment.discord"), discord);
    // A second troop destroys it; a troop strike sways no Discord.
    put(&mut g, at, TROOP, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(at, FACTORY), g.tokens(at, TROOP)), (0, 0));
    assert_eq!(g.var("alignment.discord"), discord);
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
            .filter(|&n| {
                m.hexes[n]
                    .neighbours
                    .iter()
                    .all(|&x| x == c || kind(m, x) != HexKind::City)
            })
            .collect();
        if lonely.len() >= 3 {
            return (c, lonely);
        }
    }
    panic!("no city with three lonely neighbours");
}

#[test]
fn cities_power_the_player_with_the_most_presence_next_to_them() {
    let (rules, g0) = lab(&["board.produce"]);
    let (city, n) = city_with_lonely_neighbours(&rules.map);
    let res = |g: &GameState, s: usize, r: &str| g.resource(s, r) - g0.resource(s, r);

    let mut g = g0.clone();
    put(&mut g, n[0], &presence(0), 1);
    put(&mut g, n[1], &presence(1), 1);
    put(&mut g, n[2], &presence(1), 1);
    run(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!((res(&g, 0, "energy"), res(&g, 1, "energy")), (0, 1));
    // Presence makes no compute in production (the track does, in upkeep).
    assert_eq!((res(&g, 0, "compute"), res(&g, 1, "compute")), (0, 0));

    // A tie: the players choose.
    let mut g = g0.clone();
    put(&mut g, n[0], &presence(0), 1);
    put(&mut g, n[1], &presence(1), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("an energy choice");
    assert!(
        p.ask.prompt.starts_with("The energy of"),
        "{}",
        p.ask.prompt
    );
    answer_with(&rules, &mut g, "seat 0");
    assert_eq!((res(&g, 0, "energy"), res(&g, 1, "energy")), (1, 0));

    // An aligned city powers its owner, with the bonus.
    let mut g = g0.clone();
    put(&mut g, city, &cube(1), 1);
    put(&mut g, n[0], &presence(0), 1);
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
    let (c0, c1, discord) = (
        g.resource(0, "credits"),
        g.resource(1, "credits"),
        g.var("alignment.discord"),
    );
    run(&rules, &mut g);
    // The 7th cube completes it: 10 credits a player cube, 1 Discord per 2 adversary cubes.
    assert_eq!(
        (g.resource(0, "credits") - c0, g.resource(1, "credits") - c1),
        (40, 10)
    );
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
fn the_clock_sets_production_and_war_steps_and_the_war_twist() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(1);
    g.piles.remove(CURRENT);
    let war: Vec<u32> = (1..=12)
        .filter(|&r| {
            g.round = r;
            board::war_round(&w, &g)
        })
        .collect();
    assert_eq!(war, [2, 4, 6, 7, 8, 9, 10, 11, 12]);
    let production: Vec<u32> = (1..=8)
        .filter(|&r| {
            g.round = r;
            board::production_round(&w, &g)
        })
        .collect();
    assert_eq!(production, [1, 3, 5, 6, 7, 8]);
    g.round = 1;
    g.piles
        .insert(CURRENT.into(), vec!["adv_s2_border_war".into()]);
    assert!(board::war_round(&w, &g));
    let v = rules.view(&g);
    assert_eq!(
        v.globals
            .iter()
            .find(|x| x.key == "board.war")
            .unwrap()
            .value,
        1
    );
}

#[test]
fn on_a_war_round_units_cancel_and_the_surplus_destroys_rival_buildings() {
    let (rules, mut g0) = lab(&["board.war"]);
    let m = &rules.map;
    let [b0, b1] = m.fronts[0].bases;
    let [f1, f2] = [0, 1].map(|i| m.numbered(1, piece(1).factories[i]));
    let d1 = m.numbered(1, piece(1).presence);
    put(&mut g0, d1, &presence(1), 1);
    g0.round = 2;
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");
    let units = |g: &GameState| (g.tokens(b0, UNIT), g.tokens(b1, UNIT));

    // 3 against 1: two cross, and take both factories (not the presence).
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, f2, FACTORY, 1);
    put(&mut g, b0, UNIT, 3);
    put(&mut g, b1, UNIT, 1);
    run(&rules, &mut g);
    assert_eq!(units(&g), (0, 0));
    assert_eq!(
        (
            g.tokens(f1, FACTORY),
            g.tokens(f2, FACTORY),
            g.tokens(d1, &presence(1))
        ),
        (0, 0, 1)
    );
    assert_eq!(discord(&g), 2);

    // With no factory left, a presence; a surplus with nothing to hit is spent.
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b0, UNIT, 3);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f1, FACTORY), g.tokens(d1, &presence(1))), (0, 0));
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
    assert_eq!(
        (total(&g, FACTORY), total(&g, HARDEN), discord(&g)),
        (2, 1, 0)
    );

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
    let next: Vec<usize> = m.hexes[at]
        .neighbours
        .iter()
        .copied()
        .filter(|&x| x != city && kind(m, x) != HexKind::Ocean)
        .collect();
    put(&mut g0, at, TROOP, 1);
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");

    let mut g = g0.clone();
    put(&mut g, next[0], FACTORY, 1);
    run(&rules, &mut g);
    // A troop strike sways no Discord (only war does), and uses the troop up.
    assert_eq!(
        (g.tokens(next[0], FACTORY), discord(&g), g.tokens(at, TROOP)),
        (0, 0, 0)
    );

    // Two in reach: the players choose.
    let mut g = g0.clone();
    put(&mut g, next[0], FACTORY, 1);
    put(&mut g, next[1], &presence(0), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("a strike choice");
    assert!(
        p.ask.prompt.starts_with("Adversary troop"),
        "{}",
        p.ask.prompt
    );
    assert_eq!(p.ask.options.len(), 2);
    let label = p
        .ask
        .options
        .iter()
        .find(|o| o.option.starts_with("presence:0"))
        .unwrap()
        .option
        .clone();
    answer_with(&rules, &mut g, &label);
    assert_eq!(
        (
            g.tokens(next[0], FACTORY),
            g.tokens(next[1], &presence(0)),
            discord(&g),
            g.tokens(at, TROOP)
        ),
        (1, 0, 0, 0)
    );
}

#[test]
fn a_troop_that_strikes_is_used_up_and_one_with_no_target_stays() {
    let (rules, mut g0) = lab(&["board.strike"]);
    let m = &rules.map;
    assert_eq!(rules.world().int("board.troop_spent"), 1);
    let (city, n) = city_with_lonely_neighbours(m);
    let at = n[0];
    let target = m.hexes[at]
        .neighbours
        .iter()
        .copied()
        .find(|&x| x != city && kind(m, x) != HexKind::Ocean)
        .unwrap();
    put(&mut g0, at, TROOP, 2);

    // Nothing in reach: both stay, round after round.
    let mut g = g0.clone();
    run(&rules, &mut g);
    run(&rules, &mut g);
    assert_eq!(g.tokens(at, TROOP), 2);
    assert!(!g.stats.counters.contains_key("board.troops_spent"));

    // One building in reach: the first troop destroys it and is used up;
    // the second finds nothing left and stays.
    let mut g = g0.clone();
    put(&mut g, target, FACTORY, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(target, FACTORY), g.tokens(at, TROOP)), (0, 1));
    assert_eq!(g.stats.counters["board.troops_spent"], 1);

    // With `troop_spent` 0 a troop strikes and stays.
    let (rules, mut g) = lab_with(&["board.strike"], &["board.troop_spent=0"]);
    put(&mut g, at, TROOP, 1);
    put(&mut g, target, FACTORY, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(target, FACTORY), g.tokens(at, TROOP)), (0, 1));
}

#[test]
fn with_presence_returns_a_destroyed_presence_goes_to_the_reserve_and_is_placed_first() {
    use crate::modules::common::RESERVE;
    let (rules, mut g) = lab_with(&["board.strike"], &["board.presence_returns=1"]);
    let m = &rules.map;
    let (city, n) = city_with_lonely_neighbours(m);
    let at = n[0];
    let target = m.hexes[at]
        .neighbours
        .iter()
        .copied()
        .find(|&x| x != city && kind(m, x) == HexKind::Open)
        .unwrap();
    put(&mut g, at, TROOP, 1);
    put(&mut g, target, &presence(0), 1);
    run(&rules, &mut g);
    assert_eq!(g.tokens(target, &presence(0)), 0);
    assert_eq!((g.resource(0, RESERVE), g.resource(1, RESERVE)), (1, 0));

    // Placing takes from the reserve, not the track.
    put(&mut g, city, &presence(0), 1);
    let track = board::track_taken(&g, 0);
    set_res(&mut g, 0, "energy", 1);
    rules
        .apply_mut(&mut g, &std_act(0, "presence"), None)
        .unwrap();
    let p = g.pending.as_ref().expect("a placement choice");
    assert!(
        p.ask.prompt.contains("from your reserve"),
        "{}",
        p.ask.prompt
    );
    answer_all(&rules, &mut g);
    assert_eq!(g.resource(0, RESERVE), 0);
    assert_eq!(board::track_taken(&g, 0), track);
    assert_eq!(presence_of(&g, 0).len(), 2);

    // Off: the presence is just gone.
    let (rules, mut g) = lab_with(&["board.strike"], &["board.presence_returns=0"]);
    put(&mut g, at, TROOP, 1);
    put(&mut g, target, &presence(0), 1);
    run(&rules, &mut g);
    assert_eq!(g.tokens(target, &presence(0)), 0);
    assert_eq!(g.resource(0, RESERVE), 0);
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
/// grow at setup: free open hexes next to its starting presence.
fn actions_with_room() -> (Rules, GameState) {
    let (rules, g) = lab_actions();
    let w = rules.world();
    for s in 0..2 {
        assert!(
            modules::board::presence_hexes(&w, &g, s).len() >= 2,
            "seat {s} has no room to grow at setup"
        );
    }
    (rules, g)
}

#[test]
fn standard_actions_are_offered_only_when_affordable_and_targeted() {
    let (rules, mut g) = actions_with_room();
    for s in 0..2 {
        set_res(&mut g, s, "energy", 0);
        set_res(&mut g, s, "credits", 0);
        set_res(&mut g, s, HARDWARE, 0);
        assert!(standard_offered(&rules, &g, s as u8).is_empty(), "seat {s}");
    }
    // 1 energy: placing presence and moving an ambassador.
    set_res(&mut g, 0, "energy", 1);
    assert_eq!(
        standard_offered(&rules, &g, 0),
        ["move", "presence"].map(String::from).into()
    );
    // No ambassadors: no moving.
    set_res(&mut g, 0, AMBASSADORS, 0);
    assert_eq!(
        standard_offered(&rules, &g, 0),
        ["presence".to_string()].into()
    );
    set_res(&mut g, 0, AMBASSADORS, 1);

    for s in 0..2 {
        set_res(&mut g, s, "energy", 20);
        set_res(&mut g, s, "credits", 50);
        set_res(&mut g, s, HARDWARE, 5);
    }
    let o0 = standard_offered(&rules, &g, 0);
    for a in ["presence", "station", "move", "ambassador", "hardware"] {
        assert!(o0.contains(a), "{a}: {o0:?}");
    }
    assert!(
        !o0.contains("remove") && !o0.contains("harden") && !o0.contains("node"),
        "{o0:?}"
    );
    assert!(!o0.contains("factory"), "cities build factories now");
    let d = presence_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    assert!(standard_offered(&rules, &g, 0).contains("remove"));
    put(&mut g, d, NODE, 1);
    assert!(standard_offered(&rules, &g, 0).contains("node"));
    // Only the Protector (seat 1) may harden.
    assert!(standard_offered(&rules, &g, 1).contains("harden"));
    assert!(!standard_offered(&rules, &g, 0).contains("harden"));
    assert!(rules
        .apply_mut(&mut g.clone(), &std_act(0, "harden"), None)
        .is_err());
    assert!(rules
        .apply_mut(&mut g.clone(), &std_act(0, "factory"), None)
        .is_err());
    assert!(rules
        .apply_mut(&mut g.clone(), &std_act(0, "nope"), None)
        .is_err());
    // The view publishes every standard action with its cost.
    let v = rules.view(&g);
    let acts: Vec<&str> = v.standard.iter().map(|x| x.act.as_str()).collect();
    assert_eq!(acts, board::STANDARD);
    let station = v.standard.iter().find(|x| x.act == "station").unwrap();
    assert_eq!(
        (station.energy, station.credits, station.hardware),
        (2, 0, 2)
    );
}

#[test]
fn standard_actions_cost_what_the_config_says() {
    let (rules, mut g0) = actions_with_room();
    for s in 0..2 {
        set_res(&mut g0, s, "energy", 20);
        set_res(&mut g0, s, "credits", 50);
        set_res(&mut g0, s, HARDWARE, 5);
    }
    let spent = |g: &GameState, s: usize| {
        (
            g0.resource(s, "energy") - g.resource(s, "energy"),
            g0.resource(s, "credits") - g.resource(s, "credits"),
            g0.resource(s, HARDWARE) - g.resource(s, HARDWARE),
        )
    };

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "station"));
    assert_eq!(
        (spent(&g, 0), total(&g, STATION) - total(&g0, STATION)),
        ((2, 0, 2), 1)
    );
    assert_eq!(total(&g, &cube(0)), 1);

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "presence"));
    assert_eq!((spent(&g, 0), presence_of(&g, 0).len()), ((1, 0, 0), 2));
    assert_eq!(board::track_taken(&g, 0), 2);

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "hardware"));
    assert_eq!(spent(&g, 0), (0, 4, -1));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "ambassador"));
    assert_eq!((spent(&g, 0), g.resource(0, AMBASSADORS)), ((0, 6, 0), 2));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "move"));
    assert_eq!((spent(&g, 0), g.resource(0, AMBASSADORS)), ((1, 0, 0), 0));

    let mut g = g0.clone();
    let d = presence_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    let supply = g.var(SUPPLY);
    act(&rules, &mut g, std_act(0, "remove"));
    assert_eq!(
        (spent(&g, 0), g.tokens(d, ADV_AMBASSADOR), g.var(SUPPLY)),
        ((2, 0, 0), 0, supply + 1)
    );

    let mut g = g0.clone();
    put(&mut g, d, NODE, 1);
    let nodes = total(&g, NODE);
    act(&rules, &mut g, std_act(0, "node"));
    assert_eq!((spent(&g, 0), total(&g, NODE)), ((2, 0, 0), nodes - 1));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(1, "harden"));
    assert_eq!((spent(&g, 1), total(&g, HARDEN)), ((0, 0, 1), 1));
}

#[test]
fn aligning_puts_a_cube_on_a_clean_city_in_reach() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let (city, next) = city_with_lonely_neighbours(m);
    put(&mut g, next[0], &presence(0), 1);
    set_res(&mut g, 0, "energy", 2);
    set_res(&mut g, 0, AMBASSADORS, 1);
    act(&rules, &mut g, std_act(0, "align"));
    assert_eq!(g.tokens(city, &cube(0)), 1);
    assert_eq!(
        (g.resource(0, "energy"), g.resource(0, AMBASSADORS)),
        (0, 0)
    );
    // A corrupted or aligned city can't be aligned.
    set_res(&mut g, 0, "energy", 2);
    set_res(&mut g, 0, AMBASSADORS, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("align"));
}

// ---------- presence, reach and site claims ----------

#[test]
fn placing_presence_takes_the_next_track_token_and_uncovers_its_income() {
    use crate::modules::ai;
    let (rules, mut g) = actions_with_room();
    let w = rules.world();
    let compute = |g: &GameState| {
        ai::track_income(&w, g, 0)
            .iter()
            .filter(|(r, _)| r == "compute")
            .map(|(_, n)| n)
            .sum::<i64>()
    };
    assert_eq!(
        (board::track_taken(&g, 0), compute(&g), ai::tier2(&w, &g, 0)),
        (1, 1, false)
    );
    set_res(&mut g, 0, "energy", 5);
    let start = presence_of(&g, 0)[0];
    // A seat 1 presence next to it blocks that hex (one presence a hex).
    let blocked = rules.map.hexes[start]
        .neighbours
        .iter()
        .copied()
        .find(|&h| kind(&rules.map, h) != HexKind::Ocean)
        .unwrap();
    put(&mut g, blocked, &presence(1), 1);
    let hexes = board::presence_hexes(&w, &g, 0);
    assert!(!hexes.contains(&blocked));
    assert!(hexes
        .iter()
        .all(|&h| rules.map.adjacent(h, start) && g.tokens(h, &presence(0)) == 0));
    act(&rules, &mut g, std_act(0, "presence"));
    // The 2nd space: +1 compute a round.
    assert_eq!(
        (board::track_taken(&g, 0), compute(&g), ai::tier2(&w, &g, 0)),
        (2, 2, false)
    );
    act(&rules, &mut g, std_act(0, "presence"));
    // The 3rd: tier 2.
    assert_eq!((board::track_taken(&g, 0), ai::tier2(&w, &g, 0)), (3, true));
    assert_eq!(presence_of(&g, 0).len(), 3);
    // The track runs out after its last space.
    g.vars
        .insert(board::track_key(0), ai::board(&w, 0).presence.len() as i64);
    assert!(!standard_offered(&rules, &g, 0).contains("presence"));
}

#[test]
fn card_targets_on_the_map_must_be_in_reach() {
    use crate::content::DamageAt;
    use crate::modules::effects;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let me = m.numbered(0, piece(0).presence);
    put(&mut g, me, &presence(0), 1);
    let near = m.hexes[me].neighbours[0];
    let far = hexes_where(m, |h| m.distance(h, me) > 2 && kind(m, h) != HexKind::Ocean)[0];
    put(&mut g, near, ADV_AMBASSADOR, 1);
    put(&mut g, far, ADV_AMBASSADOR, 1);
    assert_eq!(effects::damage_hexes(&w, &g, 0, DamageAt::Any), vec![near]);
}

#[test]
fn claiming_a_site_needs_presence_and_no_node_on_or_next_to_it() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let station = m.numbered(0, piece(0).stations[0]);
    let next = m.hexes[station]
        .neighbours
        .iter()
        .copied()
        .find(|&h| kind(m, h) == HexKind::Open)
        .unwrap();
    let mut g0 = g0;
    put(&mut g0, station, STATION, 1);

    // Presence two hexes away: the ambassador just waits.
    let mut g = g0.clone();
    let away = hexes_where(m, |h| {
        m.distance(h, station) == 2 && kind(m, h) == HexKind::Open
    })[0];
    put(&mut g, away, &presence(0), 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(
        (
            g.tokens(station, &cube(0)),
            g.tokens(station, &ambassador(0))
        ),
        (0, 1)
    );

    // Presence next to it: claimed.
    let mut g = g0.clone();
    put(&mut g, next, &presence(0), 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(g.tokens(station, &cube(0)), 1);

    // A node next to the presence but not the site does not block it.
    let mut g = g0.clone();
    put(&mut g, next, &presence(0), 1);
    let node_far = m.hexes[next]
        .neighbours
        .iter()
        .copied()
        .find(|&h| m.distance(h, station) == 2)
        .unwrap();
    put(&mut g, node_far, NODE, 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(g.tokens(station, &cube(0)), 1);

    // A node next to the site blocks it, wherever the presence is.
    let mut g = g0.clone();
    put(&mut g, next, &presence(0), 1);
    let node_near = m.hexes[station]
        .neighbours
        .iter()
        .copied()
        .find(|&h| h != next && kind(m, h) != HexKind::Ocean)
        .unwrap();
    put(&mut g, node_near, NODE, 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(
        (
            g.tokens(station, &cube(0)),
            g.tokens(station, &ambassador(0))
        ),
        (0, 1)
    );
    // So does a node on the site itself.
    let mut g = g0.clone();
    put(&mut g, next, &presence(0), 1);
    put(&mut g, station, NODE, 1);
    assert!(!board::can_claim(&rules.world(), &g, 0, station));

    // Aligning a city: likewise.
    let (city, lonely) = city_with_lonely_neighbours(m);
    let mut g = g0.clone();
    put(&mut g, lonely[0], &presence(0), 1);
    set_res(&mut g, 0, "energy", 2);
    set_res(&mut g, 0, AMBASSADORS, 1);
    assert!(standard_offered(&rules, &g, 0).contains("align"));
    // A node next to the presence, two from the city: still offered.
    let far = m.hexes[lonely[0]]
        .neighbours
        .iter()
        .copied()
        .find(|&h| m.distance(h, city) == 2)
        .unwrap();
    put(&mut g, far, NODE, 1);
    assert!(standard_offered(&rules, &g, 0).contains("align"));
    // A node next to the city: not.
    put(&mut g, far, NODE, 0);
    put(&mut g, lonely[1], NODE, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("align"));
}

// ---------- factories, troops and hardware ----------

#[test]
fn cities_grow_factories_that_take_their_owner() {
    let (rules, g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0: cities 6, 7, 14; zones 2 (by 6), 8 and 11 (by 7), 11 and 13 (by 14).
    // Cities build in board order: 6, 7, 14.
    assert_eq!(
        (piece(0).cities.clone(), piece(0).industry.clone()),
        (vec![7, 6, 14], vec![2, 8, 11, 13])
    );
    let (c6, c7) = (at(6), at(7));
    let zones =
        |g: &GameState| [2, 8, 11, 13].map(|n| (g.tokens(at(n), FACTORY), cube_on(g, at(n))));

    // Clean 6 builds at 2 (its only zone), neutral (no presence near);
    // corrupted 7 and 14 at the lowest-numbered free zones, 8 then 11,
    // adversary-owned.
    let mut g = g0.clone();
    put(&mut g, c7, ADV_CUBE, 1);
    put(&mut g, at(14), ADV_CUBE, 1);
    let mut gg = g.clone();
    run(&rules, &mut gg);
    answer_all(&rules, &mut gg);
    let adv = Some("cube:adversary".to_string());
    assert_eq!(
        zones(&gg),
        [(1, None), (1, adv.clone()), (1, adv.clone()), (0, None)]
    );

    // An aligned city's factory is its owner's.
    let mut gg = g0.clone();
    put(&mut gg, c6, &cube(1), 1);
    run(&rules, &mut gg);
    answer_all(&rules, &mut gg);
    assert_eq!(zones(&gg)[0], (1, Some("cube:1".into())));

    // A clean city with two free zones: the players choose (seat 0).
    let mut gg = g0.clone();
    run(&rules, &mut gg);
    let p = gg.pending.as_ref().expect("a factory choice");
    assert!(
        p.ask.prompt.starts_with("C0 #7 builds a factory"),
        "{}",
        p.ask.prompt
    );
    let options: BTreeSet<usize> = p
        .ask
        .options
        .iter()
        .map(|o| m.find(o.target.unwrap()).unwrap())
        .collect();
    assert_eq!(options, [at(8), at(11)].into());
    answer_hex(&rules, &mut gg, at(11));
    // 14 is then left only 13.
    answer_all(&rules, &mut gg);
    assert_eq!(zones(&gg).map(|z| z.0), [1, 0, 1, 1]);
}

fn cube_on(g: &GameState, h: usize) -> Option<String> {
    g.hexes[h]
        .tokens
        .keys()
        .find(|k| k.starts_with("cube:"))
        .cloned()
}

#[test]
fn a_corrupted_city_builds_next_to_an_adversary_ambassador_else_at_its_lowest_free_zone() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0 (see above): city 6 builds first; zone 2 is taken, so its
    // own neighbourhood has no free zone.
    put(&mut g0, at(2), STATION, 1);
    put(&mut g0, at(6), ADV_CUBE, 1);
    let adv = Some("cube:adversary".to_string());
    let zone = |g: &GameState, n: u8| (g.tokens(at(n), FACTORY), cube_on(g, at(n)));

    // No adversary ambassador: the lowest-numbered free zone, 8, even though
    // it is not next to the city; then 7 builds at 11 and 14 at 13, neutral.
    let mut g = g0.clone();
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert!(!m.adjacent(at(6), at(8)));
    assert_eq!(zone(&g, 8), (1, adv.clone()));
    assert_eq!((zone(&g, 11), zone(&g, 13)), ((1, None), (1, None)));

    // An adversary ambassador next to 13 (on 9) draws it there, past 8 and 11.
    let mut g = g0.clone();
    put(&mut g, at(9), ADV_AMBASSADOR, 1);
    assert!(m.adjacent(at(9), at(13)) && !m.adjacent(at(9), at(8)) && !m.adjacent(at(9), at(11)));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 13), (1, adv.clone()));
    assert_ne!(zone(&g, 8).1, adv);
    assert_ne!(zone(&g, 11).1, adv);

    // An ambassador on a zone counts too; among two drawn zones (13 on one,
    // 11 next to the one on 14; 8 next to neither), the lower.
    let mut g = g0.clone();
    put(&mut g, at(13), ADV_AMBASSADOR, 1);
    put(&mut g, at(14), ADV_AMBASSADOR, 1);
    assert!(!m.adjacent(at(8), at(14)) && !m.adjacent(at(8), at(13)));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 11), (1, adv.clone()));
    assert_ne!(zone(&g, 13).1, adv);
    // A player ambassador no longer repels it.
    let mut g = g0.clone();
    put(&mut g, at(8), &ambassador(0), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 8), (1, adv));
}

#[test]
fn a_clean_citys_factory_goes_to_the_most_presence_by_its_zone() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0 (see above), 8 taken: 6 builds at 2, 7 at 11, 14 at 13.
    put(&mut g0, at(8), STATION, 1);
    let owners = |g: &GameState| {
        [2, 11, 13].map(|n| {
            assert_eq!(g.tokens(at(n), FACTORY), 1, "zone {n}");
            cube_on(g, at(n))
        })
    };

    // Nobody's presence near: neutral.
    let mut g = g0.clone();
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(owners(&g), [None, None, None]);

    // Seat 1 next to 2; seat 1 on 11 against seat 0 on 10 (next to it), a
    // tie the players settle; seat 0 on 9 and 10, both next to 13.
    let mut g = g0.clone();
    put(&mut g, at(1), &presence(1), 1);
    put(&mut g, at(11), &presence(1), 1);
    put(&mut g, at(9), &presence(0), 1);
    put(&mut g, at(10), &presence(0), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("a tie to settle");
    assert!(
        p.ask
            .prompt
            .starts_with("The factory at C0 #11 goes to which player?"),
        "{}",
        p.ask.prompt
    );
    assert_eq!(p.ask.options.len(), 2);
    answer_with(&rules, &mut g, "seat 1");
    answer_all(&rules, &mut g);
    let c = |s: usize| Some(cube(s));
    assert_eq!(owners(&g), [c(1), c(1), c(0)]);
    let built = |g: &GameState| {
        g.stats
            .counters
            .get("board.factories_built.player")
            .copied()
    };
    assert_eq!(built(&g).unwrap_or(0) - built(&g0).unwrap_or(0), 3);

    // An aligned city's factory stays its owner's, whatever presence is near.
    let mut g = g0.clone();
    put(&mut g, at(6), &cube(0), 1);
    put(&mut g, at(1), &presence(1), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(owners(&g)[0], c(0));
}

#[test]
fn player_factories_make_hardware_on_production_steps_only() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g0, f, FACTORY, 1);
    put(&mut g0, f, &cube(0), 1);
    // Every zone taken, so nothing grows.
    for c in 0..2 {
        for &z in &piece(c).industry {
            put(&mut g0, m.numbered(c, z), FACTORY, 1);
        }
    }
    let mut g = g0.clone();
    run(&rules, &mut g);
    assert_eq!(g.resource(0, HARDWARE) - g0.resource(0, HARDWARE), 1);
    // Round 2 is a war step only: no hardware, troops or growth.
    let mut g = g0.clone();
    g.round = 2;
    run(&rules, &mut g);
    assert_eq!(g.resource(0, HARDWARE), g0.resource(0, HARDWARE));
    assert_eq!(total(&g, UNIT), 0);
}

#[test]
fn a_factory_makes_a_troop_at_each_of_its_corps_bases() {
    let (rules, mut g) = lab_with(&["board.bases"], &["countries=3"]);
    let m = &rules.map;
    for c in 0..3 {
        for &z in &piece(c).industry {
            put(&mut g, m.numbered(c, z), FACTORY, 1);
        }
        // Only country 0's first zone keeps its factory.
        for &z in &piece(c).industry {
            if !(c == 0 && z == piece(0).factories[0]) {
                put(&mut g, m.numbered(c, z), FACTORY, 0);
                put(&mut g, m.numbered(c, z), STATION, 1);
            }
        }
    }
    let bases: Vec<usize> = hexes_where(m, |h| {
        kind(m, h) == HexKind::Base && m.hexes[h].country == 0
    });
    assert_eq!(bases.len(), 2);
    run(&rules, &mut g);
    for b in bases {
        assert_eq!(g.tokens(b, UNIT), 2, "1 from the base, 1 from the factory");
    }
    let other: Vec<usize> = hexes_where(m, |h| {
        kind(m, h) == HexKind::Base && m.hexes[h].country != 0
    });
    assert!(other.iter().all(|&b| g.tokens(b, UNIT) == 1));
}

#[test]
fn an_adversary_factory_makes_adversary_troops() {
    let (rules, mut g) = lab(&["board.bases"]);
    let m = &rules.map;
    for c in 0..2 {
        for &z in &piece(c).industry {
            put(&mut g, m.numbered(c, z), STATION, 1);
        }
    }
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g, f, STATION, 0);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    let [b0, _] = m.fronts[0].bases;
    run(&rules, &mut g);
    assert_eq!((g.tokens(f, TROOP), g.tokens(b0, UNIT)), (1, 1));
}

#[test]
fn a_troop_at_a_base_absorbs_a_strike_next_to_it() {
    let (rules, mut g) = lab(&["board.strike"]);
    let m = &rules.map;
    let [b0, _] = m.fronts[0].bases;
    let next = m.hexes[b0]
        .neighbours
        .iter()
        .copied()
        .find(|&h| kind(m, h) == HexKind::Open)
        .unwrap();
    put(&mut g, next, FACTORY, 1);
    put(&mut g, next, TROOP, 1);
    put(&mut g, b0, UNIT, 1);
    let discord = g.var("alignment.discord");
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(
        (
            g.tokens(next, FACTORY),
            g.tokens(b0, UNIT),
            g.var("alignment.discord")
        ),
        (1, 0, discord)
    );
    assert_eq!(g.stats.counters["board.base_absorbed"], 1);
}

#[test]
fn selling_a_card_discards_it_for_a_credit() {
    let (rules, mut g) = lab_actions();
    for p in ["cards.deck.1", "cards.discard.1"] {
        if let Some(pile) = g.piles.get_mut(p) {
            pile.retain(|c| c != "fab_line");
        }
    }
    g.piles.insert(
        "cards.hand.0".into(),
        vec!["turnkey_factory".into(), "fab_line".into()],
    );
    let sells: Vec<Action> = rules
        .legal_actions(&g)
        .into_iter()
        .filter(|a| matches!(a, Action::Sell { seat: 0, .. }))
        .collect();
    assert_eq!(sells.len(), 2);
    let credits = g.resource(0, "credits");
    act(
        &rules,
        &mut g,
        Action::Sell {
            seat: 0,
            card: "fab_line".into(),
        },
    );
    act(
        &rules,
        &mut g,
        Action::Sell {
            seat: 0,
            card: "turnkey_factory".into(),
        },
    );
    assert_eq!(g.resource(0, "credits"), credits + 2);
    assert!(g.pile("cards.hand.0").is_empty());
    // A tier-1 card to its discard pile; an AI's unique card back to the box.
    assert_eq!(g.pile("cards.discard.1")[0], "fab_line");
    assert!(g
        .piles
        .values()
        .all(|p| !p.contains(&"turnkey_factory".to_string())));
    assert!(rules
        .apply_mut(
            &mut g,
            &Action::Sell {
                seat: 0,
                card: "fab_line".into()
            },
            None
        )
        .is_err());
}
