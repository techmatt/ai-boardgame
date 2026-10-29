//! 017: the adversary project deck, compute and its counterplay, the
//! Discord troop advance, Discord card steps and the new Unity sources.

use super::alignment::give;
use super::*;
use crate::modules::adversary::{self, PROJECTS, PROJECTS_DONE, PROJECTS_ONGOING, PROJECT_DECK};
use crate::modules::common::{presence, NODE};

fn row(g: &mut GameState, ids: &[&str]) {
    g.piles
        .insert(PROJECTS.into(), ids.iter().map(|s| s.to_string()).collect());
}

#[test]
fn setup_deals_three_cheap_projects_from_a_staged_deck() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let g = rules.new_game(3);
    let all: Vec<String> = g
        .pile(PROJECTS)
        .iter()
        .chain(g.pile(PROJECT_DECK))
        .chain(g.pile(PROJECTS_DONE))
        .chain(g.pile(PROJECTS_ONGOING))
        .cloned()
        .collect();
    assert_eq!(all.len(), 12);
    let stage =
        |id: &str| crate::content().projects[crate::content().project_index(id).unwrap()].stage;
    // Cheap (stage 1) on top: the first six are cheap, the rest big.
    let stages: Vec<u8> = all.iter().map(|id| stage(id)).collect();
    assert!(stages[..6].iter().all(|&s| s == 1), "{stages:?}");
    assert!(stages[6..].iter().all(|&s| s == 2), "{stages:?}");
    // Cost is per player: 6–7 a player for a cheap one (12–14 at 2).
    for id in g.pile(PROJECTS) {
        let c = adversary::project_cost(&w, &g, id);
        assert!((12..=14).contains(&c), "{id} costs {c}");
    }
}

#[test]
fn compute_fills_the_cheapest_project_which_resolves_when_full() {
    let (rules, mut g) = lab(&[]);
    // At 2 players: False Flag 14, Disinformation 12, Node Swarm 12.
    row(&mut g, &["false_flag", "disinformation", "node_swarm"]);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 13)).unwrap();
    // Disinformation (the first of the cheapest) filled at 12 and resolved:
    // 3 toward Discord. The 13th point went onto Node Swarm.
    assert_eq!(g.pile(PROJECTS_DONE), ["disinformation"]);
    assert_eq!(g.var("alignment.discord"), 3);
    assert_eq!(g.var(&adversary::project_key("node_swarm")), 1);
    assert_eq!(g.var(&adversary::project_key("false_flag")), 0);
    assert_eq!(g.var(adversary::COMPUTE), 13);
    assert_eq!(g.stats.counters["adversary.projects_resolved.cheap"], 1);
    assert_eq!(g.stats.per_round["adversary.compute"][1], 13);
}

#[test]
fn a_new_row_is_dealt_when_all_three_have_resolved() {
    let (rules, mut g) = lab(&[]);
    row(&mut g, &["market_crash"]);
    g.piles.insert(
        PROJECT_DECK.into(),
        [
            "station_coup",
            "recruitment_drive",
            "total_offensive",
            "armoured_columns",
        ]
        .map(String::from)
        .to_vec(),
    );
    set_res(&mut g, 0, "credits", 7);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 13)).unwrap();
    // Market Crash (12) resolved: 5 credits each (as much as they have).
    assert_eq!(g.resource(0, "credits"), 2);
    assert_eq!(
        g.pile(PROJECTS),
        ["station_coup", "recruitment_drive", "total_offensive"]
    );
    assert_eq!(g.pile(PROJECT_DECK), ["armoured_columns"]);
    // The 13th point went onto the cheapest of the new row (Recruitment
    // Drive, 20).
    assert_eq!(g.var(&adversary::project_key("recruitment_drive")), 1);
    // An ongoing project goes into play and acts every round's build.
    g.vars
        .insert(adversary::project_key("recruitment_drive"), 19);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 1)).unwrap();
    assert_eq!(g.pile(PROJECTS_ONGOING), ["recruitment_drive"]);
}

#[test]
fn an_ongoing_project_acts_in_the_build() {
    let (rules, mut g) = lab(&["adversary.build"]);
    g.piles
        .insert(PROJECTS_ONGOING.into(), vec!["recruitment_drive".into()]);
    g.piles
        .insert(NEXT.into(), vec!["adv_s1_probing_strikes".into()]);
    run(&rules, &mut g);
    // Every clean city (lab: all of them) gains an adversary ambassador.
    assert_eq!(total(&g, ADV_AMBASSADOR), cities(&rules.map).len() as i64);
}

#[test]
fn a_player_ambassador_on_a_node_stops_its_compute_and_stays() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let m = &rules.map;
    let (a, b) = (
        m.numbered(0, piece(0).stations[0]),
        m.numbered(1, piece(1).stations[0]),
    );
    put(&mut g, a, NODE, 1);
    put(&mut g, b, NODE, 1);
    put(&mut g, a, &ambassador(0), 1);
    let before = g.var(adversary::COMPUTE);
    g.piles
        .insert(NEXT.into(), vec!["adv_s1_probing_strikes".into()]);
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), before + 1);
    assert_eq!(g.tokens(a, &ambassador(0)), 1);
    assert_eq!(g.stats.counters["adversary.compute_blocked"], 1);
}

#[test]
fn uncountered_threats_can_carry_compute() {
    let (rules, mut g) = lab(&[]);
    row(&mut g, &["false_flag"]);
    let t = crate::content()
        .threats
        .iter()
        .filter(|t| {
            t.effects
                .iter()
                .any(|e| matches!(e, crate::content::ThreatOp::Compute { .. }))
        })
        .count();
    // About half the threats carry compute.
    let n = crate::content().threats.len();
    assert!(t * 3 >= n && t * 3 <= 2 * n, "{t} of {n}");
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 2)).unwrap();
    assert_eq!(g.var(&adversary::project_key("false_flag")), 2);
}

#[test]
fn security_cards_drain_or_discard_a_project() {
    let (rules, mut g) = lab_actions();
    row(&mut g, &["false_flag", "disinformation"]);
    g.vars.insert(adversary::project_key("false_flag"), 6);
    g.piles.insert(
        "cards.hand.0".into(),
        vec!["security_patch".into(), "zero_day_wipe".into()],
    );
    set_res(&mut g, 0, "credits", 30);
    set_res(&mut g, 0, "insight", 5);
    set_res(&mut g, 0, "antimatter", 2);
    act(
        &rules,
        &mut g,
        Action::Play {
            seat: 0,
            card: "security_patch".into(),
        },
    );
    // The only project holding compute: 6 - 4.
    assert_eq!(g.var(&adversary::project_key("false_flag")), 2);
    g.vars.insert("ai.tier2.0".into(), 1);
    g.vars.insert(crate::modules::board::track_key(0), 3);
    if rules.legal_actions(&g).contains(&Action::Play {
        seat: 0,
        card: "zero_day_wipe".into(),
    }) {
        act(
            &rules,
            &mut g,
            Action::Play {
                seat: 0,
                card: "zero_day_wipe".into(),
            },
        );
        assert_eq!(g.pile(PROJECTS).len(), 1);
        assert_eq!(g.var(&adversary::project_key("false_flag")), 0);
        assert_eq!(g.stats.counters["adversary.projects_discarded"], 1);
    } else {
        panic!("Zero-Day Wipe is not playable with tier 2 unlocked");
    }
}

/// Stacks the Discord deck: `card` is drawn next, whatever the tier.
fn next_discord(g: &mut GameState, card: &str) {
    for tier in [1, 2] {
        g.piles.insert(
            format!("alignment.deck.discord.{tier}"),
            vec![card.to_string()],
        );
    }
}

/// Fills the Discord bucket so the next action tips it.
fn fill_discord(rules: &Rules, g: &mut GameState) {
    let cap = crate::modules::alignment::capacity(&rules.world(), g, crate::content::Side::Discord);
    g.vars.insert("alignment.discord".into(), cap);
    let u = g.var("alignment.undecided");
    g.vars
        .insert("alignment.undecided".into(), (u - cap).max(0));
}

#[test]
fn a_discord_tip_advances_every_troop_which_then_strikes() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    // A presence of seat 0 and a troop 3 hexes away, both on land.
    let p = m.numbered(0, piece(0).presence);
    let t = (0..m.hexes.len())
        .find(|&h| m.distance(p, h) == 3 && kind(m, h) != content::HexKind::Ocean)
        .unwrap();
    put(&mut g, p, &presence(0), 1);
    put(&mut g, t, TROOP, 1);
    next_discord(&mut g, "propaganda");
    fill_discord(&rules, &mut g);
    give(&rules, &mut g);
    answer_all(&rules, &mut g);
    let at = (0..m.hexes.len())
        .find(|&h| g.tokens(h, TROOP) > 0)
        .unwrap();
    assert_eq!(m.distance(at, p), 2);
    assert_eq!(g.stats.counters["board.troop_advances"], 1);
    // The second tip brings it next to the presence, and it strikes it.
    next_discord(&mut g, "propaganda");
    fill_discord(&rules, &mut g);
    give(&rules, &mut g);
    answer_all(&rules, &mut g);
    let at = (0..m.hexes.len())
        .find(|&h| g.tokens(h, TROOP) > 0)
        .unwrap();
    assert_eq!(m.distance(at, p), 1);
    assert_eq!(g.tokens(p, &presence(0)), 0);
    assert_eq!(g.stats.counters["board.advance_destroyed"], 1);
    // No escalation any more.
    assert_eq!(g.var("adversary.escalation"), 0);
}

#[test]
fn discord_cards_step_up_with_the_discord_tip_count() {
    for (tips, each) in [(0, 1), (2, 2), (5, 3)] {
        let (rules, mut g) = lab(&[]);
        g.vars.insert("alignment.discord_tips".into(), tips);
        next_discord(&mut g, "blight");
        fill_discord(&rules, &mut g);
        give(&rules, &mut g);
        answer_all(&rules, &mut g);
        assert_eq!(total(&g, ADV_AMBASSADOR), 2 * each, "{tips} tips");
    }
}

#[test]
fn alignment_cards_are_one_shot() {
    assert!(crate::content()
        .alignment
        .iter()
        .all(|c| !c.effects.is_empty()));
    // The ongoing field is gone from the format: content with it is rejected.
    let bad = r#"[{"id":"x","name":"X","side":"unity","tier":1,"ongoing":true,"text":"","effects":[{"op":"build"}]}]"#;
    assert!(serde_json::from_str::<Vec<content::AlignmentCard>>(bad).is_err());
}

#[test]
fn freeing_a_city_and_removing_a_node_sway_unity() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).cities[0]);
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, city, &presence(0), 1);
    put(&mut g, city, &ambassador(0), 2);
    set_res(&mut g, 0, "ambassadors", 1);
    set_res(&mut g, 0, "energy", 10);
    rules.apply_mut(&mut g, &std_act(0, "move"), None).unwrap();
    answer_hex(&rules, &mut g, city);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(city, ADV_CUBE), 0);
    assert_eq!(g.var("alignment.unity"), 2);
    put(&mut g, city, NODE, 1);
    rules.apply_mut(&mut g, &std_act(0, "node"), None).unwrap();
    if g.pending.is_some() {
        answer_hex(&rules, &mut g, city);
    }
    assert_eq!(g.tokens(city, NODE), 0);
    assert_eq!(g.var("alignment.unity"), 3);
}

#[test]
fn a_player_ambassador_on_a_base_sways_unity_in_production() {
    // Locked (013/016): a player ambassador on a producing base sways
    // `base_unity` instead of a troop.
    let (rules, mut g) = lab(&["board.bases"]);
    let m = &rules.map;
    let base = (0..m.hexes.len())
        .find(|&h| kind(m, h) == content::HexKind::Base)
        .unwrap();
    put(&mut g, base, &ambassador(0), 1);
    run(&rules, &mut g);
    assert_eq!(g.var("alignment.unity"), 2);
    assert_eq!(g.tokens(base, &ambassador(0)), 0);
}
