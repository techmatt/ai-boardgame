use super::*;
use crate::modules::adversary;

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
        let (rules, mut g) = lab_with(&["adversary.spread"], &[&format!("adversary.variant={variant}")]);
        let m = &rules.map;
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
