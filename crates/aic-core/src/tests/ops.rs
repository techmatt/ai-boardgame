//! The op schema and the word lists: what can go where, and the orderings
//! the state and the agents depend on.

use super::*;
use crate::content::{check_ops, Op, Slot, Tag, TagOrAny, RESOURCES, TAGS};
use crate::vocab::{DiscountOn, TokenKind};

#[test]
fn an_op_in_the_wrong_list_fails_to_load() {
    let bad = [
        (Op::War, Slot::Player),
        (Op::HardenAction, Slot::Threat),
        (Op::Sacrifice, Slot::Player),
        (Op::CutNode, Slot::Team),
        (Op::Sway { side: crate::content::Side::Unity, amount: 1 }, Slot::Standing),
        (Op::Compute { amount: 1 }, Slot::Twist),
    ];
    for (op, slot) in bad {
        assert!(check_ops(&[op.clone()], slot, "test").is_err(), "{op:?} in {slot:?}");
    }
    // Inside an `if` too.
    let gate = Op::If {
        cond: crate::content::Cond::RemovedTroop,
        then: vec![Op::War],
        otherwise: Vec::new(),
    };
    assert!(check_ops(&[gate], Slot::Player, "test").is_err());
}

#[test]
fn ops_check_the_resources_they_name() {
    let reserve = Op::Gain { resource: Resource::PresenceReserve, amount: 1 };
    assert!(check_ops(&[reserve], Slot::Player, "test").is_err());
    let gives = Op::Gain { resource: Resource::Ambassadors, amount: 1 };
    assert!(check_ops(&[gives], Slot::Player, "test").is_ok());
    let income = Op::TeamIncome { resource: Resource::Energy, amount: 1 };
    assert!(check_ops(&[income], Slot::Team, "test").is_err());
    let each = Op::Troops { count: 1, each_country: true, destroy: false };
    assert!(check_ops(&[each], Slot::Threat, "test").is_err());
}

#[test]
fn a_misspelt_word_does_not_load() {
    assert!(serde_json::from_str::<Op>(r#"{"op":"gain","resource":"credit","amount":1}"#).is_err());
    assert!(serde_json::from_str::<Tag>(r#""sciense""#).is_err());
    assert!(serde_json::from_str::<TokenKind>(r#""tokens""#).is_err());
    assert!(serde_json::from_str::<TagOrAny>(r#""all""#).is_err());
    assert!(serde_json::from_str::<DiscountOn>(r#""buildings""#).is_err());
}

/// `Seat::resources` serializes in `Ord` order: the fingerprint depends on it
/// being the words' order.
#[test]
fn resources_order_as_their_words() {
    use Resource::*;
    let all = [Ambassadors, Antimatter, Chips, Credits, Energy, Insight, PresenceReserve];
    assert!(all.windows(2).all(|p| p[0] < p[1] && p[0].name() < p[1].name()));
    assert_eq!((Chips.amount(1), Chips.amount(2)), ("1 chip".to_string(), "2 chips".to_string()));
    for r in all {
        assert_eq!(serde_json::to_string(&r).unwrap(), format!("\"{}\"", r.name()));
        assert_eq!(r.is_player(), RESOURCES.contains(&r));
    }
    for t in TAGS {
        assert_eq!(serde_json::to_string(&t).unwrap(), format!("\"{}\"", t.name()));
        assert_eq!(serde_json::to_string(&TagOrAny::Is(t)).unwrap(), format!("\"{}\"", t.name()));
    }
    assert_eq!(serde_json::to_string(&TagOrAny::Any).unwrap(), "\"any\"");
}
