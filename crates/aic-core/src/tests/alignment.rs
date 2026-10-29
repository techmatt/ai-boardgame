use super::*;
use crate::modules::common::NODE;
use crate::modules::{adversary, alignment};

/// Stacks the Unity deck: `card` is drawn next, whatever the tier.
fn next_unity(g: &mut GameState, card: &str) {
    for tier in [1, 2] {
        g.piles.insert(
            format!("alignment.deck.unity.{tier}"),
            vec![card.to_string()],
        );
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
pub(super) fn give(rules: &Rules, g: &mut GameState) {
    set_res(g, 0, "insight", 1);
    act(rules, g, Action::Give { seat: 0, to: 1 });
}

/// No corrupted city and no node left (ambassadors and troops stay).
fn uncorrupt_cities(rules: &Rules, g: &mut GameState) {
    for h in cities(&rules.map) {
        put(g, h, ADV_CUBE, 0);
    }
    for h in 0..rules.map.hexes.len() {
        put(g, h, NODE, 0);
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
    // Two Unity tips: bar 3 (019), where only corrupted cities count.
    assert_eq!(alignment::bar_level(&rules.world(), &g), 3);
    let o = g.outcome.as_ref().expect("a win");
    assert!(o.won && o.reason == "purge", "{o:?}");
}

#[test]
fn bar_levels_follow_the_unity_tips() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(1);
    // 019: `unity_ease` 1, 1, 2: ambassadors and troops at 1 tip, nodes at 2.
    for (tips, level) in [(0, 0), (1, 2), (2, 3), (9, 3)] {
        g.vars.insert("alignment.unity_tips".into(), tips);
        assert_eq!(alignment::bar_level(&w, &g), level, "{tips} tips");
    }
    assert!(adversary::win_text(&w, 0).contains("node, adversary troop or adversary ambassador"));
    assert!(adversary::win_text(&w, 1).contains("ambassadors no longer count"));
    assert!(adversary::win_text(&w, 2).contains("no corrupted city or node"));
    assert!(adversary::win_text(&w, 3).contains("nodes no longer count"));
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
        (1, 1, 1, Some(ADV_AMBASSADOR), true),
        (0, 1, 1, Some(TROOP), false),
        (1, 1, 1, Some(TROOP), true),
        (5, 1, 1, Some(ADV_CUBE), false),
        (0, 1, 1, Some(NODE), false),
        (1, 1, 1, Some(NODE), false),
        (2, 1, 1, Some(NODE), true),
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
        let won = g
            .outcome
            .as_ref()
            .is_some_and(|o| o.won && o.reason == "purge");
        assert_eq!(
            won, wins,
            "{tips} tips, rule {rule}, stage {stage}, {piece_left:?}"
        );
        if !wins {
            // The view says when the clear-board win starts to count.
            let bar = rules
                .view(&g)
                .globals
                .into_iter()
                .find(|v| v.key == "alignment.bar")
                .unwrap();
            assert_eq!(
                bar.doc.contains("from adversary stage"),
                tips < 2 && stage < rule,
                "{}",
                bar.doc
            );
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
        assert_eq!(
            (g.var("alignment.unity"), g.var("alignment.discord")),
            (2 - moved, moved),
            "{variant}"
        );
        // Once a round per player.
        assert!(!rules.legal_actions(&g).contains(&a));
        assert!(rules.apply_mut(&mut g, &a, None).is_err());
        assert!(rules
            .legal_actions(&g)
            .contains(&Action::Antimatter { seat: 1 }));
    }
    let (rules, g) = action_only(&["alignment.antimatter_discord=0"]);
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| !matches!(a, Action::Antimatter { .. })));
}

/// A Discord tip no longer escalates the difficulty (017): it resolves one
/// card, one-shot; Momentum shrinks a bucket by 1 per 2 of its side's tips,
/// down to 1 per player.
#[test]
fn discord_tips_do_not_escalate_and_momentum_shrinks_buckets() {
    let (rules, mut g) = action_only(&[]);
    g.vars.insert("alignment.discord_tips".into(), 1);
    let undecided = g.var("alignment.undecided");
    g.vars.insert("alignment.undecided".into(), undecided - 4);
    g.vars.insert("alignment.discord".into(), 4);
    give(&rules, &mut g);
    assert_eq!(g.var("alignment.discord_tips"), 2);
    assert_eq!(g.var("adversary.escalation"), 0);
    let w = rules.world();
    assert!(!adversary::level(&w, 1));
    assert_eq!(g.pile("alignment.resolved.discord").len(), 1);
    let d = rules
        .view(&g)
        .globals
        .into_iter()
        .find(|v| v.key == "adversary.difficulty")
        .unwrap();
    assert_eq!(d.value, 0);

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
