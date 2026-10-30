use super::alignment::give;
use super::projects::row;
use super::*;
use crate::modules::{adversary, threats};

#[test]
fn the_ais_spend_their_resource_as_credits_on_cards() {
    let (rules, mut g) = lab_actions();
    for c in ["darknet_exchange", "literature_crawler"] {
        Arc::make_mut(g.piles.get_mut("cards.deck.1").unwrap()).retain(|x| x != c);
    }
    // The Builder (seat 0): energy on buildings, 1 credit each, after
    // Economies of scale takes 1 off.
    g.piles.insert(
        "cards.hand.0".into(),
        vec!["darknet_exchange".into(), "literature_crawler".into()].into(),
    );
    set_res(&mut g, 0, Resource::Credits, 3);
    set_res(&mut g, 0, Resource::Energy, 10);
    set_res(&mut g, 0, Resource::Insight, 1);
    let plays = |g: &GameState, seat: u8| -> Vec<Key> {
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
    set_res(&mut g, 0, Resource::Credits, 0);
    assert_eq!(plays(&g, 0), ["darknet_exchange"]);
    // 021: Darknet Exchange (023; was Fab Line) costs 4, 3 after Economies of scale; with 1 credit at
    // least 2 energy go in.
    set_res(&mut g, 0, Resource::Credits, 1);
    let energy = g.resource(0, Resource::Energy);
    rules
        .apply_mut(
            &mut g,
            &Action::Play {
                seat: 0,
                card: "darknet_exchange".into(),
            },
            None,
        )
        .unwrap();
    let p = g.pending.as_ref().expect("a discount choice");
    let opts: Vec<&str> = p.ask.options.iter().map(|o| o.option.as_str()).collect();
    assert_eq!(opts, ["spend 2 energy", "spend 3 energy"]);
    answer_with(&rules, &mut g, "spend 3 energy");
    // 021: a new slot, or over the Builder's starting Foundry Core.
    answer_with(&rules, &mut g, "a new slot");
    assert_eq!((g.resource(0, Resource::Energy), g.resource(0, Resource::Credits)), (energy - 3, 1));
    assert!(g.pile("cards.built.0").contains(&Key::new("darknet_exchange")));
}

#[test]
fn growth_and_draft_picks_cannot_be_passed() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(2);
    // 021: round 1 is the opening (no draft); pass through its action phase.
    advance_to(&rules, &mut g, "cards.draft");
    let legal = rules.legal_actions(&g);
    assert!(
        legal.iter().all(|a| matches!(a, Action::Draft { .. })),
        "{legal:?}"
    );
    assert_eq!(legal.iter().filter(|a| a.seat() == 0).count(), 4);
    assert!(rules
        .apply_mut(&mut g, &Action::Pass { seat: 0 }, None)
        .is_err());
    advance_to(&rules, &mut g, "ai.growth");
    let legal = rules.legal_actions(&g);
    assert!(
        legal.iter().all(|a| matches!(a, Action::Growth { .. })),
        "{legal:?}"
    );
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
                for id in pile.iter() {
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
    assert_eq!(
        (g.var("alignment.unity"), g.var("alignment.discord")),
        (0, 0)
    );
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| !matches!(a, Action::Antimatter { .. })));
    g.vars.insert("alignment.unity".into(), 4);
    give(&rules, &mut g);
    assert_eq!(g.var("alignment.unity_tips"), 0);

    let rules = Rules::new(&baseline_with(&["cards.variant=off"])).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    // Only the AIs' unique cards: nothing dealt or drafted.
    assert!(g.pile("cards.hand.0").len() <= 3);
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| !matches!(a, Action::Tier { .. } | Action::Draft { .. })));
}

#[test]
fn playing_a_card_pays_its_insight_and_lowers_its_income() {
    // 022: insight to play, shared with keeps; `lowers` cuts an income for good.
    let (rules, mut g) = lab_actions();
    for c in ["darknet_exchange", "recursive_self_improvement"] {
        Arc::make_mut(g.piles.get_mut("cards.deck.1").unwrap()).retain(|x| x != c);
        Arc::make_mut(g.piles.get_mut("cards.deck.2").unwrap()).retain(|x| x != c);
    }
    g.piles.insert("cards.hand.1".into(), vec!["darknet_exchange".into()].into());
    set_res(&mut g, 1, Resource::Credits, 20);
    let playable = |g: &GameState| {
        rules
            .legal_actions(g)
            .iter()
            .any(|a| matches!(a, Action::Play { seat: 1, card } if card == "darknet_exchange"))
    };
    set_res(&mut g, 1, Resource::Insight, 0);
    assert!(!playable(&g), "Darknet Exchange costs 1 insight");
    set_res(&mut g, 1, Resource::Insight, 2);
    assert!(playable(&g));
    rules
        .apply_mut(&mut g, &Action::Play { seat: 1, card: "darknet_exchange".into() }, None)
        .unwrap();
    assert_eq!(g.resource(1, Resource::Insight), 1);
    // Recursive Self-Improvement lowers credit income by 1, through the production key.
    let w = rules.world();
    let d = crate::modules::cards::def(&w, "recursive_self_improvement");
    assert_eq!(d.lowers, vec![(Resource::Credits, 1)]);
    assert_eq!(d.insight, 2);
    let before = crate::modules::economy::income(&w, &g, 1)[Resource::Credits];
    let mut c = crate::modules::cards::def(&w, "recursive_self_improvement").clone();
    c.lowers = vec![(Resource::Credits, before + 1)];
    assert!(!crate::modules::cards::lowers_ok(&w, &g, 1, &c), "can't lower an income below 0");
    c.lowers = vec![(Resource::Credits, 1)];
    assert!(crate::modules::cards::lowers_ok(&w, &g, 1, &c));
}

#[test]
fn selling_a_card_discards_it_for_a_credit() {
    let (rules, mut g) = lab_actions();
    for p in ["cards.deck.1", "cards.discard.1"] {
        if let Some(pile) = g.piles.get_mut(p) {
            Arc::make_mut(pile).retain(|c| c != "darknet_exchange");
        }
    }
    g.piles.insert(
        "cards.hand.0".into(),
        vec!["turnkey_factory".into(), "darknet_exchange".into()].into(),
    );
    let sells: Vec<Action> = rules
        .legal_actions(&g)
        .into_iter()
        .filter(|a| matches!(a, Action::Sell { seat: 0, .. }))
        .collect();
    assert_eq!(sells.len(), 2);
    let credits = g.resource(0, Resource::Credits);
    act(
        &rules,
        &mut g,
        Action::Sell {
            seat: 0,
            card: "darknet_exchange".into(),
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
    assert_eq!(g.resource(0, Resource::Credits), credits + 2);
    assert!(g.pile("cards.hand.0").is_empty());
    // A tier-1 card to its discard pile; an AI's unique card back to the box.
    assert_eq!(g.pile("cards.discard.1")[0], "darknet_exchange");
    assert!(g
        .piles
        .values()
        .all(|p| !p.contains(&Key::new("turnkey_factory"))));
    assert!(rules
        .apply_mut(
            &mut g,
            &Action::Sell {
                seat: 0,
                card: "darknet_exchange".into()
            },
            None
        )
        .is_err());
}

/// 023's primitives: draw, drain every project, count tokens as power, cash
/// tokens out at a growing rate (with a cap); and an `if` is only offered
/// when the branch that would fire has a target.
#[test]
fn the_023_primitives_draw_drain_all_count_and_cash_out() {
    use crate::content::{Cond, Op};
    use crate::modules::{cards, effects};
    let (rules, mut g) = lab_actions();
    let run = |g: &mut GameState, e: Op| {
        with_cx(&rules, |cx| effects::run(g, cx, &[e], &mut effects::Ctx::seat(0))).unwrap()
    };
    let hand = g.pile("cards.hand.0").len();
    let deck = g.pile("cards.deck.1").len();
    run(&mut g, Op::Draw { count: 2 });
    assert_eq!(g.pile("cards.hand.0").len(), hand + 2);
    assert_eq!(g.pile("cards.deck.1").len(), deck - 2);

    row(&mut g, &["false_flag", "disinformation"]);
    g.vars.insert(adversary::project_key("false_flag").into(), 6);
    g.vars.insert(adversary::project_key("disinformation").into(), 3);
    run(&mut g, Op::DrainAll { amount: 4 });
    assert_eq!(g.var(&adversary::project_key("false_flag")), 2);
    assert_eq!(g.var(&adversary::project_key("disinformation")), 0);

    // Retained Earnings holds Capital.
    g.piles.insert("cards.built.0".into(), vec!["retained_earnings".into()].into());
    g.vars.insert(cards::tokens_key(0, "retained_earnings").into(), 5);
    let credits = g.resource(0, Resource::Credits);
    run(
        &mut g,
        Op::GainPerTokens {
            resource: Resource::Credits,
            kind: TokenKind::Capital,
            per: 2,
            max: 9,
        },
    );
    assert_eq!(g.resource(0, Resource::Credits), credits + 2, "5 Capital count as 2; they stay");
    assert_eq!(g.var(&cards::tokens_key(0, "retained_earnings")), 5);
    run(
        &mut g,
        Op::CashOut {
            kind: TokenKind::Capital,
            resource: Resource::Credits,
            max: 3,
        },
    );
    assert_eq!(g.resource(0, Resource::Credits), credits + 2 + 6, "3 cashed out: 1 + 2 + 3");
    assert_eq!(g.var(&cards::tokens_key(0, "retained_earnings")), 2);
    run(
        &mut g,
        Op::CashOut {
            kind: TokenKind::Capital,
            resource: Resource::Credits,
            max: 0,
        },
    );
    assert_eq!(g.resource(0, Resource::Credits), credits + 2 + 6 + 3);
    assert_eq!(g.var(&cards::tokens_key(0, "retained_earnings")), 0);

    // Both branches drain a project: with no compute left, nothing to target.
    let w = rules.world();
    let drain = vec![Op::DrainProject { amount: 1 }];
    let gate = [Op::If {
        cond: Cond::CityWithAmbassador,
        then: drain.clone(),
        otherwise: drain,
    }];
    g.vars.insert(adversary::project_key("false_flag").into(), 0);
    assert!(!effects::feasible(&w, &g, 0, &gate));
    g.vars.insert(adversary::project_key("false_flag").into(), 1);
    assert!(effects::feasible(&w, &g, 0, &gate));
}

fn face_of(id: &str) -> crate::face::Face {
    crate::face::card_face(content().cards.iter().find(|c| c.id == id).expect("a card"))
}

#[test]
fn a_face_opens_with_its_boxes_word() {
    let f = face_of("telemetry_archive");
    assert_eq!(f.top_text, "Each round, 1 Dataset on this card. Action: pay 2 Datasets from this card, get 1 insight.");
    assert_eq!(f.text, "");
    let f = face_of("orbital_embassy");
    assert_eq!(f.top_text, "Each round, 4 credits and 1 insight.");
    assert_eq!(f.text, "Pay 2 chips and 1 antimatter. Gain 2 ambassadors.");
}

#[test]
fn a_bare_arrow_is_a_free_action() {
    let f = face_of("exabyte_archive");
    assert!(f.top_text.contains("Action: once a round, get 1 credit per 2 Datasets on your active"), "{}", f.top_text);
}

#[test]
fn income_below_the_art_raises_or_lowers_the_track() {
    let f = face_of("antiproton_offtake_deal");
    assert_eq!(f.top_text, "");
    assert_eq!(f.text, "Pay 3 energy. Lower your credit income by 1. Raise your antimatter income by 1.");
}

#[test]
fn only_buildings_print_above_the_art() {
    for c in &content().cards {
        let f = crate::face::card_face(c);
        assert!(c.is_building() || f.top.is_empty(), "{} prints above the art", c.id);
        assert!(!c.text.is_empty() && c.text == f.full_text(), "{}", c.id);
    }
}

#[test]
fn hand_written_card_text_is_refused() {
    let mut c = content().clone();
    for card in &mut c.cards {
        card.text.clear();
    }
    c.cards[0].text = "Gain a thousand credits.".to_string();
    assert!(c.check().unwrap_err().contains("generated"));
}

#[test]
fn only_a_building_keeps_an_action() {
    let mut c = content().clone();
    for card in &mut c.cards {
        card.text.clear();
    }
    assert_eq!(c.check(), Ok(()));
    let bld = c.cards.iter_mut().find(|d| d.id == "telemetry_archive").expect("a card");
    bld.kind = crate::content::CardKind::Program;
    assert!(c.check().unwrap_err().contains("only a building"));
}
