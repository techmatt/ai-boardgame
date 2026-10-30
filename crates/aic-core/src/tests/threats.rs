use super::*;
use crate::modules::{board, threats};

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
    g.piles
        .entry(format!("threats.on.{c}"))
        .or_default()
        .push(threat.into());
}

#[test]
fn threats_are_countered_only_when_fully_paid_and_partial_payments_come_back() {
    let (rules, mut g) = threat_lab();
    deal(&mut g, "sabotage", 0);
    deal(&mut g, "blackout", 1);
    for s in 0..2 {
        set_res(&mut g, s, "credits", 6);
    }
    let legal = rules.legal_actions(&g);
    // Only players with a presence in the threat's country may pay.
    let pays = |seat: u8, t: &str| {
        legal
            .iter()
            .any(|a| matches!(a, Action::Pay { seat: s, threat, .. } if *s == seat && threat == t))
    };
    assert!(pays(0, "sabotage") && !pays(0, "blackout"));
    assert!(pays(1, "blackout") && !pays(1, "sabotage"));
    let pay = |seat, threat: &str, resource: &str, amount| Action::Pay {
        seat,
        threat: threat.into(),
        resource: resource.into(),
        amount,
    };
    // Blackout: 4 credits (024). Pay 1, withdraw, then pay in full.
    rules
        .apply_mut(&mut g, &pay(1, "blackout", "credits", 1), None)
        .unwrap();
    assert_eq!(g.resource(1, "credits"), 5);
    let withdraw = Action::Withdraw {
        seat: 1,
        threat: "blackout".into(),
        resource: "credits".into(),
    };
    rules.apply_mut(&mut g, &withdraw, None).unwrap();
    assert_eq!(g.resource(1, "credits"), 6);
    assert!(rules
        .apply_mut(&mut g, &pay(1, "blackout", "credits", 5), None)
        .is_err());
    assert!(rules
        .apply_mut(&mut g, &pay(0, "blackout", "credits", 1), None)
        .is_err());
    rules
        .apply_mut(&mut g, &pay(1, "blackout", "credits", 4), None)
        .unwrap();
    assert!(g.pile("threats.on.1").is_empty());
    assert_eq!(g.pile("threats.discard.1")[0], "blackout");
    assert_eq!(g.resource(1, "credits"), 2);
    // Sabotage: 5 credits. Pay only part; it comes back.
    rules
        .apply_mut(&mut g, &pay(0, "sabotage", "credits", 2), None)
        .unwrap();
    assert_eq!(g.pile("threats.on.0"), ["sabotage"]);
    let discord = g.var("alignment.discord");
    for s in 0..2 {
        rules
            .apply_mut(&mut g, &Action::Pass { seat: s }, None)
            .unwrap();
    }
    assert!(g.pending.is_none());
    assert_eq!(g.resource(0, "credits"), 6);
    assert!(g
        .pile("threats.discard.1")
        .contains(&"sabotage".to_string()));
    assert!(board::flagged(&g, "sabotage", 0));
    assert_eq!(g.var("alignment.discord"), discord + 1);
}

#[test]
fn jamming_a_threat_down_to_what_is_paid_counters_it() {
    let (rules, mut g) = threat_lab();
    // Requisition needs 4 credits (024); with 2 paid, Threat Triage (023; was
    // Jam Signals) takes it to 2: countered at once.
    deal(&mut g, "requisition", 0);
    set_res(&mut g, 0, "credits", 12);
    rules
        .apply_mut(
            &mut g,
            &Action::Pay {
                seat: 0,
                threat: "requisition".into(),
                resource: "credits".into(),
                amount: 2,
            },
            None,
        )
        .unwrap();
    g.piles
        .get_mut("cards.deck.1")
        .unwrap()
        .retain(|c| c != "threat_triage");
    g.piles
        .insert("cards.hand.0".into(), vec!["threat_triage".into()]);
    set_res(&mut g, 0, "insight", 1);
    act(
        &rules,
        &mut g,
        Action::Play {
            seat: 0,
            card: "threat_triage".into(),
        },
    );
    assert!(g.pile("threats.on.0").is_empty());
    assert!(g
        .pile("threats.discard.1")
        .contains(&"requisition".to_string()));
    assert_eq!(g.resource(0, "credits"), 6);
}

#[test]
fn difficulty_1_raises_the_counters_and_deals_more_threats() {
    let sets = [
        "adversary.counter_bonus=1",
        "adversary.counter_bonus_all=1",
        "adversary.extra_threats=1",
    ];
    let at = |d: i64| {
        let mut c = lab_config(&["threats.deal", ACTIONS]);
        for s in sets
            .iter()
            .map(|s| s.to_string())
            .chain([format!("adversary.difficulty={d}")])
        {
            c.apply_override(&s).unwrap();
        }
        Rules::new(&c).unwrap()
    };
    let (r0, r1) = (at(0), at(1));
    let (g0, mut g1) = (r0.new_game(1), r1.new_game(1));
    let req = |r: &Rules, g: &GameState| threats::requirement(&r.world(), g, "orbital_strike");
    let credits = |c| vec![("credits".to_string(), c)];
    assert_eq!(req(&r0, &g0), credits(6));
    assert_eq!(req(&r1, &g1), credits(8));
    let on = |g: &GameState, c: usize| {
        threats::active(g)
            .iter()
            .filter(|(_, at)| *at == Some(c))
            .count()
    };
    assert_eq!((on(&g0, 0), on(&g0, 1)), (1, 1));
    // One extra a round, one country after the other.
    assert_eq!((on(&g1, 0), on(&g1, 1)), (2, 1));
    for s in 0..2 {
        r1.apply_mut(&mut g1, &Action::Pass { seat: s }, None)
            .unwrap();
    }
    assert_eq!(g1.round, 2);
    assert_eq!((on(&g1, 0), on(&g1, 1)), (3, 3));
}

/// 020: threats execute after production and upkeep, so a Blackout or
/// Sabotage flag lasts through the next round's production and upkeep
/// (cleared "this round", it never landed).
#[test]
fn blackout_and_sabotage_last_into_the_next_rounds_production() {
    let round = [ACTIONS, "board.produce", "economy.upkeep", "threats.execute", "board.cleanup"];
    let mut c = lab_config(&round);
    // 021: no neutral city's energy offered for sale (that would ask).
    c.apply_override("board.city_energy_credits=20").unwrap();
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(3);
    deal(&mut g, "blackout", 0);
    deal(&mut g, "sabotage", 1);
    let pass_round = |g: &mut GameState| {
        for s in 0..2 {
            rules.apply_mut(g, &Action::Pass { seat: s }, None).unwrap();
        }
    };
    // A twin with no threats: what round 2 would have paid.
    let mut twin = rules.new_game(3);
    pass_round(&mut g);
    pass_round(&mut twin);
    assert_eq!(g.round, 2);
    assert!(board::flagged(&g, "blackout", 0) && board::flagged(&g, "sabotage", 1));
    let before = |g: &GameState| (0..2).map(|s| g.resource(s, "energy")).collect::<Vec<_>>();
    let (b, bt) = (before(&g), before(&twin));
    pass_round(&mut g);
    pass_round(&mut twin);
    assert_eq!(g.round, 3);
    assert!(!board::flagged(&g, "blackout", 1) && !board::flagged(&g, "sabotage", 0));
    // 021: round 2's upkeep paid seat 1 (presence in sabotaged country 1) no
    // energy; its twin got its energy income.
    let gain = |g: &GameState, b: &[i64], s: usize| g.resource(s, "energy") - b[s];
    let track = crate::modules::economy::income(&rules.world(), &twin, 1)[crate::content::ENERGY];
    assert!(track > 0);
    assert_eq!(gain(&g, &b, 1), gain(&twin, &bt, 1) - track);
}

/// 022 (D11): a threat dealt after the counter phase (a Discord tip in
/// production) waits beside the row: it executes next round, after a
/// counter phase in which it could be paid.
#[test]
fn a_threat_dealt_after_the_counter_phase_executes_next_round() {
    let (rules, mut g) = threat_lab();
    deal(&mut g, "supply_raid", 1);
    // Deal one extra as if in the execute phase (after the counter phase).
    g.phase = 1;
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 1));
    g.phase = 0;
    let late = g.pile("threats.on.0")[0].clone();
    assert_eq!(g.var(&format!("threats.late.{late}")), 1);
    let pass_round = |g: &mut GameState| {
        for s in 0..2 {
            rules.apply_mut(g, &Action::Pass { seat: s }, None).unwrap();
        }
        answer_all(&rules, g);
    };
    pass_round(&mut g);
    assert_eq!(g.round, 2);
    assert_eq!(g.stats.counters["threats.executed"], 1);
    assert_eq!(g.pile("threats.on.0"), [late.clone()]);
    pass_round(&mut g);
    assert_eq!(g.stats.counters["threats.executed"], 2);
    assert!(g.pile("threats.on.0").is_empty());
    assert_eq!(g.var(&format!("threats.late.{late}")), 0);
}

/// 022 (D12): the tier-2 deck covers the largest round's deal (4 countries,
/// difficulty 1+), and a deal the deck can't supply is counted.
#[test]
fn the_tier_2_deck_covers_four_countries_and_a_short_deal_is_counted() {
    let base = Rules::new(&baseline()).unwrap();
    let w = base.world();
    let tier2 = crate::content().threats.iter().filter(|t| t.tier == 2).count() as i64;
    let most = 4 * w.int("threats.per_country") + w.int("adversary.extra_threats");
    assert!(tier2 > most, "{tier2} tier-2 threats for a deal of {most}");
    let (rules, mut g) = threat_lab();
    g.piles.insert("threats.deck.1".into(), vec!["blackout".into()]);
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 3));
    assert_eq!(g.stats.counters["threats.short"], 2);
}
