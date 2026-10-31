//! 044's bug hunt: rules that printed but didn't run, cards offered for
//! nothing, prompts that named the wrong price.

use super::*;
use crate::content::{CountryRule, DamageAt, Op};
use crate::modules::adversary::NEXT;
use crate::modules::common::{hex_opt, presence, ADV_AMBASSADOR, MARKER, TROOP};
use crate::modules::{alignment, board, effects};
use crate::state::Pick;
use crate::vocab::{Side, TokenKind};

#[test]
fn a_stage_iii_card_pulls_sway_out_of_the_unity_bucket() {
    let (rules, mut g) = lab_with(&["adversary.build"], &[]);
    let w = rules.world();
    let card = w.content.adversary.iter().find(|c| c.stage == 3).unwrap();
    let pull = card.twists.iter().find_map(|t| match t {
        Op::PullUnity { amount } => Some(*amount),
        _ => None,
    });
    let pull = pull.expect("a stage-III card prints the pull");
    g.vars.insert(alignment::bucket(Side::Unity).into(), pull + 1);
    g.piles.insert(NEXT.into(), vec![card.id.as_str().into()].into());
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.stats.counters.get("alignment.unity_pulled").copied(), Some(pull));
    assert!(g.var(alignment::bucket(Side::Unity)) <= 1, "the bucket loses the pull");
}

#[test]
fn a_card_that_only_hardens_or_moves_needs_a_target_but_one_that_also_gains_does_not() {
    let (rules, g) = lab_with(&[], &[]);
    let w = rules.world();
    let harden = Op::Harden { count: 2, in_country: CountryRule::PresenceCountry };
    let moves = Op::MoveOne { range: 2 };
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&harden)), "no building, no presence");
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&moves)), "no ambassador");
    assert!(!effects::feasible(&w, &g, 0, &[harden.clone(), moves.clone()]));
    // Either one acting is enough: Peacekeeping Detail with an ambassador to move.
    let mut with_one = g.clone();
    let c = rules.map.numbered(0, piece(0).presence);
    let far = rules.map.numbered(0, piece(0).corrupted[0]);
    put(&mut with_one, c, crate::modules::common::ambassador(0), 1);
    put(&mut with_one, far, presence(1), 1);
    assert!(effects::feasible(&w, &with_one, 0, &[harden.clone(), Op::MoveOne { range: 20 }]));
    let gain = Op::Gain { resource: Resource::Credits, amount: 1 };
    assert!(effects::feasible(&w, &g, 0, &[gain, moves]), "a card that also gains is worth playing");
}

#[test]
fn a_small_removal_has_no_target_on_a_hex_holding_only_a_troop() {
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let a = rules.map.numbered(0, piece(0).presence);
    put(&mut g, a, presence(0), 1);
    put(&mut g, a, TROOP, 1);
    let one = [Op::Damage { amount: 1, at: DamageAt::Reach }];
    assert!(!effects::feasible(&w, &g, 0, &one), "a troop is worth troop_health");
    let two = [Op::Damage { amount: w.int("adversary.troop_health"), at: DamageAt::Reach }];
    assert!(effects::feasible(&w, &g, 0, &two));
    put(&mut g, a, ADV_AMBASSADOR, 1);
    assert!(effects::feasible(&w, &g, 0, &one));
}

#[test]
fn a_cash_in_counts_the_tokens_the_same_card_adds_first() {
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let kind = TokenKind::Capital;
    let holder = w.content.cards.iter().find(|c| c.tokens.as_ref().is_some_and(|t| t.kind == kind)).unwrap();
    g.piles.insert(crate::modules::cards::built(0).into(), vec![holder.id.as_str().into()].into());
    let cash = Op::CashOut { kind, resource: Resource::Credits, max: 0 };
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&cash)), "nothing to cash in");
    assert!(effects::feasible(&w, &g, 0, &[Op::AddTokens { kind, amount: 2 }, cash]));
}

#[test]
fn up_to_two_markers_can_stop_after_one() {
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let c = rules.map.numbered(0, piece(0).presence);
    let n: Vec<usize> = w.map.hexes[c].neighbours.iter().copied().filter(|&h| !crate::modules::common::is_ocean(&w, h)).take(2).collect();
    put(&mut g, c, presence(0), 1);
    for &h in &n {
        put(&mut g, h, MARKER, 1);
    }
    let first = hex_opt(&w, n[0]);
    let picks = vec![Pick { option: first.option, target: first.target }, Pick { option: "stop".into(), target: None }];
    let mut after = g.clone();
    with_cx_picks(&rules, picks, |cx| {
        effects::run(&mut after, cx, &[Op::ClearMarkers { count: 2 }], &mut effects::Ctx::seat(0)).unwrap();
    });
    assert_eq!(n.iter().map(|&h| after.tokens(h, MARKER)).sum::<i64>(), 1, "the second pick was stop");
}

#[test]
fn a_placements_prompt_names_what_it_charges() {
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let a = rules.map.numbered(0, piece(0).presence);
    put(&mut g, a, presence(0), 1);
    g.vars.insert(board::track_key(0).into(), 5);
    g.vars.insert(board::track2_key(0).into(), 5);
    let step = crate::modules::ai::board(&w, 0).presence[5].cost;
    assert!(step > 0);
    set_res(&mut g, 0, Resource::Energy, 20);
    set_res(&mut g, 0, Resource::Credits, 50);
    let prompt_for = |ops: &[Op]| {
        let mut gg = g.clone();
        let mut choices = module::Choices::new(Vec::new());
        let mut cx = module::Cx { world: rules.world(), module: "test", purposes: &[], events: None, choices: &mut choices };
        let _ = effects::run(&mut gg, &mut cx, ops, &mut effects::Ctx::seat(0));
        let ask = choices.asked.take().expect("two tracks to choose from");
        ask.options.iter().map(|o| o.option.clone()).collect::<Vec<_>>().join(" | ")
    };
    let free = prompt_for(&[Op::PlacePresence { count: 1, free: true }]);
    assert!(!free.contains("costs"), "{free}");
    let paid = prompt_for(&[Op::PlacePresence { count: 1, free: false }]);
    assert!(paid.contains(&format!("costs {step} energy")), "{paid}");
}

#[test]
fn a_seat_that_lost_its_presence_there_can_take_back_a_payment() {
    let (rules, mut g) = lab_with(&[], &[]);
    let t = rules.world().content.threats[0].id.clone();
    let on = format!("threats.on.{}", 0);
    g.piles.insert(on.as_str().into(), vec![t.as_str().into()].into());
    g.vars.insert(format!("threats.paid.{t}.0.credits").as_str().into(), 1);
    let legal = rules.legal_actions(&g);
    assert!(legal.iter().any(|a| matches!(a, Action::Withdraw { seat: 0, .. })), "{legal:?}");
    assert!(!legal.iter().any(|a| matches!(a, Action::Pay { seat: 0, .. })), "no presence, no paying");
}

/// Every number in `v`'s ops, not its name or text.
fn op_numbers(v: &serde_json::Value, out: &mut Vec<i64>) {
    match v {
        serde_json::Value::Number(n) => out.extend(n.as_i64()),
        serde_json::Value::Array(a) => a.iter().for_each(|x| op_numbers(x, out)),
        serde_json::Value::Object(o) => o
            .iter()
            .filter(|(k, _)| !["id", "name", "text", "tier", "stage"].contains(&k.as_str()))
            .for_each(|(_, x)| op_numbers(x, out)),
        _ => {}
    }
}

/// Numbers a text prints, a "3+" (a step's tip count, read off no op) aside.
fn text_numbers(text: &str) -> Vec<i64> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_digit() {
            let start = i;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            if b.get(i) != Some(&b'+') {
                out.push(text[start..i].parse().unwrap());
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Only player-card text is generated: the hand-written rest must not print a
/// number its ops don't have (044's content sweep found none; keep it so).
#[test]
fn hand_written_text_prints_no_number_its_ops_lack() {
    fn walk(v: &serde_json::Value, bad: &mut Vec<String>) {
        match v {
            serde_json::Value::Array(a) => a.iter().for_each(|x| walk(x, bad)),
            serde_json::Value::Object(o) => {
                if let (Some(id), Some(text)) = (o.get("id"), o.get("text").and_then(|t| t.as_str())) {
                    let mut have = Vec::new();
                    op_numbers(v, &mut have);
                    let missing: Vec<i64> = text_numbers(text).into_iter().filter(|n| !have.contains(n)).collect();
                    if !missing.is_empty() {
                        bad.push(format!("{id}: {missing:?} in {text:?}"));
                    }
                }
                o.values().for_each(|x| walk(x, bad));
            }
            _ => {}
        }
    }
    let mut bad = Vec::new();
    for file in [
        include_str!("../../../../content/threats.json"),
        include_str!("../../../../content/adversary.json"),
        include_str!("../../../../content/alignment.json"),
    ] {
        walk(&serde_json::from_str(file).unwrap(), &mut bad);
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn stage_cards_are_checked_at_what_the_difficulty_deals() {
    let printed = baseline();
    let rules = Rules::new(&printed).unwrap();
    let have = rules.world().content.adversary.iter().filter(|c| c.stage == 3).count() as i64;
    let ok = baseline_with(&[&format!("adversary.stage_cards=4,5,{have}")]);
    assert!(Rules::new(&ok).is_ok(), "difficulty 0 adds no stage-III card");
    let over = baseline_with(&[&format!("adversary.stage_cards=4,5,{have}"), "adversary.difficulty=2"]);
    assert!(Rules::new(&over).is_err(), "difficulty 2 adds one");
    let empty = baseline_with(&["adversary.stage_cards=1,1,0", "adversary.stage1_removed=1", "adversary.stage2_removed=1", "adversary.stage3_added=0", "adversary.difficulty=3"]);
    assert!(Rules::new(&empty).is_err(), "nothing left to deal");
}
