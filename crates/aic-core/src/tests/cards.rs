use super::alignment::give;
use super::*;
use crate::modules::threats;

#[test]
fn the_ais_spend_their_resource_as_credits_on_cards() {
    let (rules, mut g) = lab_actions();
    for c in ["fab_line", "recon_sweep", "evacuate"] {
        g.piles.get_mut("cards.deck.1").unwrap().retain(|x| x != c);
    }
    // The Builder (seat 0): energy on buildings, 1 credit each, after
    // Economies of scale takes 1 off.
    g.piles.insert(
        "cards.hand.0".into(),
        vec!["fab_line".into(), "recon_sweep".into()],
    );
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
    // 021: Fab Line costs 4, 3 after Economies of scale; with 1 credit at
    // least 2 energy go in.
    set_res(&mut g, 0, "credits", 1);
    let energy = g.resource(0, "energy");
    rules
        .apply_mut(
            &mut g,
            &Action::Play {
                seat: 0,
                card: "fab_line".into(),
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
    assert_eq!((g.resource(0, "energy"), g.resource(0, "credits")), (energy - 3, 1));
    assert!(g.pile("cards.built.0").contains(&"fab_line".to_string()));
}

#[test]
fn growth_and_draft_picks_cannot_be_passed() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(2);
    // 021: round 1 is the opening (no draft); pass through its action phase
    // (first-action play would pay and withdraw a threat counter forever).
    for _ in 0..2000 {
        if g.pending.is_none() && rules.phase(&g).is_some_and(|p| p.name == "cards.draft") {
            break;
        }
        let legal = rules.legal_actions(&g);
        let a = legal
            .iter()
            .find(|a| matches!(a, Action::Pass { .. }))
            .unwrap_or(&legal[0])
            .clone();
        rules.apply_mut(&mut g, &a, None).unwrap();
    }
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
