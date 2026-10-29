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
    // Economies of scale takes 2 off. Fab Line costs 6: with 3 credits it
    // spends 3 to 6 energy, the players choose.
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
    set_res(&mut g, 0, "credits", 3);
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
    assert_eq!(
        opts,
        [
            "spend 3 energy",
            "spend 4 energy",
            "spend 5 energy",
            "spend 6 energy"
        ]
    );
    answer_with(&rules, &mut g, "spend 4 energy");
    assert_eq!((g.resource(0, "energy"), g.resource(0, "credits")), (6, 1));
    assert!(g.pile("cards.built.0").contains(&"fab_line".to_string()));

    // The Protector (seat 1): compute on cyber cards. Recon Sweep costs 2:
    // with no credits both units must be compute, so nothing is asked.
    g.piles.insert(
        "cards.hand.1".into(),
        vec!["recon_sweep".into(), "evacuate".into()],
    );
    set_res(&mut g, 1, "credits", 0);
    set_res(&mut g, 1, "compute", 5);
    set_res(&mut g, 1, "insight", 1);
    assert_eq!(plays(&g, 1), ["recon_sweep"]);
    act(
        &rules,
        &mut g,
        Action::Play {
            seat: 1,
            card: "recon_sweep".into(),
        },
    );
    // 2 spent, 1 gained by the card.
    assert_eq!((g.resource(1, "compute"), g.resource(1, "credits")), (4, 0));
}

#[test]
fn growth_and_draft_picks_cannot_be_passed() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(2);
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
    // Only the AIs' unique cards: nothing dealt, drafted or drawable.
    assert!(g.pile("cards.hand.0").len() <= 3);
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| !matches!(a, Action::Draw { .. })));
}
