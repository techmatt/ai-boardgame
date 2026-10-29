use super::*;
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
    let (rules, g) = lab_actions();
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
    let (rules, mut g) = lab_actions();
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
