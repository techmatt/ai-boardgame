//! 024: freeing a corrupted city, the credit-priced standard projects, the
//! purge's first round and the card text limit.

use super::*;
use crate::content::Effect;
use crate::modules::adversary::{self, PROJECTS};
use crate::modules::common::{ambassador, presence, presence_of, ADV_AMBASSADOR, NODE};

/// A corrupted city with seat 0's presence on it, and an adjacent land hex.
fn city(rules: &Rules, g: &mut GameState) -> (usize, usize) {
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(g, h, ADV_CUBE, 1);
    put(g, h, &presence(0), 1);
    let n = rules.map.hexes[h].neighbours[0];
    (h, n)
}

/// Answers a hex choice, if it was asked (a single option is taken for you).
fn pick(rules: &Rules, g: &mut GameState, h: usize) {
    if g.pending.is_some() {
        answer_hex(rules, g, h);
    }
}

fn play_card(rules: &Rules, g: &mut GameState, seat: u8, card: &str) {
    g.piles.get_mut("cards.hand.0").unwrap().push(card.into());
    set_res(g, 0, "credits", 30);
    set_res(g, 0, "insight", 5);
    set_res(g, 0, "antimatter", 2);
    rules
        .apply_mut(g, &Action::Play { seat, card: card.into() }, None)
        .unwrap();
}

#[test]
fn a_corrupted_city_frees_with_x_of_your_own_and_no_adversary_ambassador_next_to_it() {
    // At 2, so that seats' ambassadors could pool if they did.
    let (rules, mut g) = lab_with(&[], &["board.free_city_at=2"]);
    let (h, n) = city(&rules, &mut g);
    set_res(&mut g, 0, "energy", 10);
    set_res(&mut g, 0, "credits", 10);
    set_res(&mut g, 0, "ambassadors", 2);
    put(&mut g, h, &ambassador(1), 1);
    // Another player's ambassadors don't pool (024: was any player's), with
    // nothing guarding the city.
    rules.apply_mut(&mut g, &std_act(0, "move"), None).unwrap();
    pick(&rules, &mut g, h);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_CUBE), 1);
    // Two of your own with an adversary ambassador next to it: still corrupted.
    put(&mut g, n, ADV_AMBASSADOR, 1);
    rules.apply_mut(&mut g, &std_act(0, "move"), None).unwrap();
    pick(&rules, &mut g, h);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(h, &ambassador(0)), 2);
    assert_eq!(g.tokens(h, ADV_CUBE), 1);
    // Removing the neighbour frees it at once; your ambassadors are used up.
    let unity = g.var("alignment.unity");
    let mut k = g.clone();
    put(&mut k, n, ADV_AMBASSADOR, 0);
    rules.apply_mut(&mut k, &std_act(0, "energy"), None).unwrap();
    assert_eq!(k.tokens(h, ADV_CUBE), 0);
    assert_eq!(k.tokens(h, &ambassador(0)), 0);
    assert_eq!(k.tokens(h, &ambassador(1)), 1);
    assert_eq!(k.var("alignment.unity") - unity, rules.world().int("alignment.free_city"));
    assert_eq!(k.stats.counters["board.cities_freed"], 1);
}

#[test]
fn only_a_tier_2_ultimate_frees_a_city_outright() {
    for c in &crate::content().cards {
        let cities = c.effects.iter().any(|e| matches!(e, Effect::Cleanse { cities: true, .. }));
        assert_eq!(cities, c.id == "great_liberation", "{}", c.id);
    }
    // A tier-1 cleanse is not offered for a city, and does nothing to one.
    let (rules, mut g) = lab_actions();
    let (h, _) = city(&rules, &mut g);
    let others: Vec<usize> = (0..g.hexes.len()).filter(|&x| x != h).collect();
    for x in others {
        put(&mut g, x, ADV_CUBE, 0);
    }
    g.piles.get_mut("cards.hand.0").unwrap().push("back_channel_talks".into());
    set_res(&mut g, 0, "credits", 30);
    set_res(&mut g, 0, "insight", 5);
    let plays = rules.legal_actions(&g);
    assert!(!plays.iter().any(|a| matches!(a, Action::Play { card, .. } if card == "back_channel_talks")));
    // The ultimate frees it.
    let mut k = g.clone();
    play_card(&rules, &mut k, 0, "great_liberation");
    pick(&rules, &mut k, h);
    answer_all(&rules, &mut k);
    assert_eq!(k.tokens(h, ADV_CUBE), 0);
}

#[test]
fn clearing_a_city_takes_the_ambassadors_next_to_it_first() {
    let (rules, mut g) = lab_actions();
    let (h, n) = city(&rules, &mut g);
    put(&mut g, h, ADV_AMBASSADOR, 2);
    put(&mut g, n, ADV_AMBASSADOR, 2);
    play_card(&rules, &mut g, 0, "amnesty_decree");
    pick(&rules, &mut g, h);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(n, ADV_AMBASSADOR), g.tokens(h, ADV_AMBASSADOR)), (0, 2));
    let mut k = g.clone();
    put(&mut k, n, ADV_AMBASSADOR, 3);
    // It needs a Diplomacy tag in play.
    k.piles.entry(crate::modules::cards::played(0)).or_default().push("consular_pouch".into());
    play_card(&rules, &mut k, 0, "liberation_accords");
    pick(&rules, &mut k, h);
    answer_all(&rules, &mut k);
    assert_eq!((k.tokens(n, ADV_AMBASSADOR), k.tokens(h, ADV_AMBASSADOR)), (0, 0));
}

#[test]
fn standard_projects_turn_credits_into_energy_presence_hardening_and_drains() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    set_res(&mut g, 0, "credits", 40);
    set_res(&mut g, 0, "energy", 0);
    rules.apply_mut(&mut g, &std_act(0, "energy"), None).unwrap();
    assert_eq!(g.resource(0, "energy"), 1);
    assert_eq!(g.resource(0, "credits"), 40 - w.int("board.energy_credits"));

    let before = presence_of(&g, 0).len();
    let credits = g.resource(0, "credits");
    rules.apply_mut(&mut g, &std_act(0, "presence_buy"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(presence_of(&g, 0).len(), before + 1);
    assert_eq!(g.resource(0, "credits"), credits - w.int("board.presence_credits"));

    // Any player may harden with credits (the Builder has no Harden passive).
    let p = presence_of(&g, 0)[0];
    put(&mut g, p, STATION, 1);
    put(&mut g, p, &cube(0), 1);
    rules.apply_mut(&mut g, &std_act(0, "harden_buy"), None).unwrap();
    pick(&rules, &mut g, p);
    assert_eq!(g.tokens(p, HARDEN), 1);

    // A drain needs compute on a project.
    g.piles.insert(PROJECTS.into(), vec!["market_crash".into()]);
    assert!(rules.apply_mut(&mut g.clone(), &std_act(0, "drain"), None).is_err());
    g.vars.insert(adversary::project_key("market_crash"), 5);
    rules.apply_mut(&mut g, &std_act(0, "drain"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.var(&adversary::project_key("market_crash")), 5 - w.int("board.drain_amount"));
}

#[test]
fn the_purge_counts_only_from_its_round() {
    assert_eq!(Rules::new(&baseline()).unwrap().world().int("alignment.purge_round"), 6);
    let mut c = lab_config(&[ACTIONS]);
    c.apply_override("alignment.purge_round=6").unwrap();
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(1);
    for h in 0..g.hexes.len() {
        put(&mut g, h, ADV_CUBE, 0);
        put(&mut g, h, NODE, 0);
        put(&mut g, h, ADV_AMBASSADOR, 0);
    }
    g.vars.insert("alignment.unity_tips".into(), 2);
    set_res(&mut g, 0, "credits", 10);
    g.round = 5;
    let mut k = g.clone();
    rules.apply_mut(&mut k, &std_act(0, "energy"), None).unwrap();
    assert!(k.outcome.is_none());
    g.round = 6;
    rules.apply_mut(&mut g, &std_act(0, "energy"), None).unwrap();
    assert!(g.outcome.as_ref().is_some_and(|o| o.won && o.reason == "purge"));
}

/// A corruption doesn't free the city at once: a player ambassador waiting
/// there is lost with it (review of 024).
#[test]
fn a_corruption_takes_a_waiting_ambassador_with_it() {
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, &ambassador(0), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    set_res(&mut g, 0, "credits", 10);
    rules.apply_mut(&mut g, &std_act(0, "energy"), None).unwrap();
    assert_eq!(g.tokens(h, ADV_CUBE), 1);
    assert_eq!(g.tokens(h, &ambassador(0)), 0);
}

/// 024: a card repeats its rules text; it must fit a fixed art height.
#[test]
fn card_text_fits_the_art_height() {
    for c in &crate::content().cards {
        assert!(c.text.len() <= 100, "{}: {} chars", c.id, c.text.len());
    }
}
