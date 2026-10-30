//! 027: the two decks as clocks, the presence loss, arrivals, the hold, the
//! piece ladder and what destroys presence.

use super::alignment::give;
use super::*;
use crate::modules::common::{ambassador, presence};
use crate::modules::{adversary, board};

/// The baseline playing only the action phase, after setup.
fn action_only(sets: &[&str]) -> (Rules, GameState) {
    let mut c = baseline_with(sets);
    c.round = vec![ACTIONS.into()];
    let rules = Rules::new(&c).unwrap();
    let g = rules.new_game(4);
    (rules, g)
}

/// The hold rule's numbers (the lab pins them off).
const HOLD: [&str; 3] = ["board.hold_base=3", "board.hold_min=2", "board.free_leaves=1"];

fn empty_deck(g: &mut GameState, side: &str) {
    for tier in 1..=2 {
        g.piles.insert(format!("alignment.deck.{side}.{tier}"), Vec::new());
    }
}

#[test]
fn a_discord_tip_with_an_empty_deck_loses_and_nothing_reshuffles() {
    let (rules, mut g) = action_only(&[]);
    empty_deck(&mut g, "discord");
    g.piles.insert("alignment.resolved.discord".into(), vec!["blight".into()]);
    fill_bucket(&rules, &mut g, Side::Discord);
    give(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "discord_deck"));
    assert_eq!(g.pile("alignment.resolved.discord"), ["blight"]);
}

#[test]
fn drawing_the_last_unity_card_wins_outright() {
    let (rules, mut g) = action_only(&[]);
    empty_deck(&mut g, "unity");
    g.piles.insert("alignment.deck.unity.1".into(), vec!["civil_defense".into()]);
    fill_bucket(&rules, &mut g, Side::Unity);
    give(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a win");
    assert_eq!((o.won, o.reason.as_str()), (true, "unity_deck"));
}

#[test]
fn a_player_with_no_presence_loses_the_game_for_everyone() {
    let (rules, mut g) = action_only(&[]);
    for h in 0..g.hexes.len() {
        put(&mut g, h, &presence(1), 0);
    }
    give(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "presence"));
}

#[test]
fn the_supply_is_a_cap_not_a_loss() {
    let (rules, mut g) = lab_with(&["adversary.build"], &[]);
    let city = rules.map.numbered(0, piece(0).corrupted[0]);
    put(&mut g, city, ADV_CUBE, 1);
    g.vars.insert(SUPPLY.into(), 1);
    g.piles.insert(NEXT.into(), vec!["adv_s1_long_patrols".into()]);
    run(&rules, &mut g);
    assert_eq!(g.tokens(city, ADV_AMBASSADOR), 1);
    assert!(!g.is_over());
}

#[test]
fn every_corrupted_city_is_no_loss() {
    let (rules, mut g) = lab(&[]);
    corrupt_cities_except(&rules.map, &mut g, &[]);
    set_res(&mut g, 0, Resource::Insight, 1);
    act(&rules, &mut g, Action::Give { seat: 0, to: 1 });
    assert!(!g.is_over());
}

#[test]
fn a_card_brings_arrivals_to_every_site_it_names_a_round_ahead() {
    let (rules, mut g) = lab_with(&["adversary.build"], &["adversary.arrivals=1"]);
    g.vars.insert(SUPPLY.into(), 40);
    g.piles.insert(NEXT.into(), vec!["adv_s1_probing_strikes".into()]);
    let w = rules.world();
    let plan = crate::modules::plan::next_build(&w, &g);
    let coast = hexes_where(&rules.map, |h| rules.map.hexes[h].coastal);
    assert_eq!(plan.iter().map(|&(h, _)| h).collect::<Vec<_>>(), coast);
    assert_eq!(coast.len(), 4, "2 a country");
    run(&rules, &mut g);
    for h in coast {
        assert_eq!(g.tokens(h, ADV_AMBASSADOR), 1);
    }
}

#[test]
fn each_stage_spreads_the_site_types() {
    use crate::op::{Op, Site};
    for stage in 1..=3u8 {
        let mut seen: Vec<String> = Vec::new();
        for a in content().adversary.iter().filter(|a| a.stage == stage) {
            let sites: Vec<&Site> = a
                .twists
                .iter()
                .filter_map(|t| match t {
                    Op::Arrivals { site } => Some(site),
                    _ => None,
                })
                .collect();
            assert_eq!(sites.len(), 1, "{} names one site", a.id);
            seen.push(format!("{:?}", sites[0]));
        }
        seen.sort();
        seen.dedup();
        assert_eq!(seen.len(), 4, "stage {stage}: {seen:?}");
    }
}

#[test]
fn every_piece_prints_two_coastal_cities() {
    let (rules, _) = action_only(&["countries=4"]);
    for c in 0..4 {
        let n = rules.map.countries[c]
            .hexes
            .iter()
            .filter(|&&h| rules.map.hexes[h].coastal)
            .count();
        assert_eq!(n, 2, "country {c}");
    }
}

#[test]
fn the_hold_counts_any_players_and_the_adversary_on_or_next_to_it() {
    let (rules, mut g) = lab_with(&[], &HOLD);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    let n = rules.map.hexes[h].neighbours[0];
    put(&mut g, h, ADV_CUBE, 1);
    let w = rules.world();
    assert_eq!(board::hold(&w, &g, h), 3);
    put(&mut g, n, ADV_AMBASSADOR, 2);
    assert_eq!(board::hold(&w, &g, h), 5);
    put(&mut g, n, ADV_AMBASSADOR, 0);
    put(&mut g, h, &ambassador(0), 2);
    put(&mut g, h, &ambassador(1), 1);
    // The next end check frees it: nothing else moved.
    set_res(&mut g, 0, Resource::Insight, 1);
    act(&rules, &mut g, Action::Give { seat: 0, to: 1 });
    assert_eq!(g.tokens(h, ADV_CUBE), 0);
    assert_eq!(g.tokens(h, &ambassador(0)) + g.tokens(h, &ambassador(1)), 0);
    // Down the ladder, not off the board.
    assert_eq!(g.tokens(h, ADV_AMBASSADOR), 1);
}

#[test]
fn the_hold_is_never_below_its_minimum() {
    let (rules, mut g) = lab_with(&[], &["board.hold_base=0", "board.hold_min=2"]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    assert_eq!(board::hold(&rules.world(), &g, h), 2);
    put(&mut g, h, ADV_CUBE, 1);
    put(&mut g, h, &ambassador(0), 1);
    set_res(&mut g, 0, Resource::Insight, 1);
    act(&rules, &mut g, Action::Give { seat: 0, to: 1 });
    assert_eq!(g.tokens(h, ADV_CUBE), 1);
}

#[test]
fn a_city_that_corrupts_destroys_presence_on_it_to_the_destroyed_pile() {
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    let far = rules.map.numbered(1, piece(1).cities[0]);
    put(&mut g, h, &presence(0), 1);
    put(&mut g, far, &presence(0), 1);
    g.vars.insert(STAGE.into(), 2);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    assert_eq!(g.tokens(h, &presence(0)), 0);
    assert_eq!(g.resource(0, Resource::PresenceReserve), 1);
}

#[test]
fn stage_one_never_takes_a_last_presence() {
    let (rules, mut g) = lab_with(&[], &["board.last_safe_stage=2"]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, &presence(0), 1);
    g.vars.insert(STAGE.into(), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    assert_eq!(g.tokens(h, &presence(0)), 1, "stage I: it stays");
    let gone = with_cx(&rules, |cx| {
        board::destroy_building(&mut g, cx, h, &presence(0), board::Cause::Troop)
    });
    assert!(!gone);
    g.vars.insert(STAGE.into(), 2);
    let gone = with_cx(&rules, |cx| {
        board::destroy_building(&mut g, cx, h, &presence(0), board::Cause::Troop)
    });
    assert!(gone, "stage II: it goes");
}

#[test]
fn a_discord_card_takes_the_boards_most_threatened_hex() {
    use crate::content::Op;
    use crate::modules::effects;
    let (rules, mut g) = lab(&[]);
    let (a, b) = (rules.map.numbered(0, piece(0).presence), rules.map.numbered(1, piece(1).presence));
    put(&mut g, a, &presence(0), 1);
    put(&mut g, b, &presence(1), 1);
    put(&mut g, b, ADV_AMBASSADOR, 2);
    with_cx(&rules, |cx| {
        effects::run(&mut g, cx, &[Op::DestroyPresence], &mut effects::Ctx::team().named("discord"))
    })
    .unwrap();
    assert_eq!((g.tokens(a, &presence(0)), g.tokens(b, &presence(1))), (1, 0));
    assert_eq!(g.stats.counters["board.presence_lost.discord"], 1);
}

#[test]
fn a_choice_threat_never_takes_a_last_presence() {
    use crate::modules::threats;
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).presence);
    put(&mut g, h, &presence(0), 1);
    set_res(&mut g, 0, Resource::Credits, 2);
    // No option is left: the player pays what they have and keeps the presence.
    with_cx(&rules, |cx| {
        threats::lose_presence_or(&mut g, cx, Some(0), Resource::Credits, 6, "threat1")
    })
    .unwrap();
    assert_eq!(g.tokens(h, &presence(0)), 1);
    assert_eq!(g.resource(0, Resource::Credits), 0);
}
