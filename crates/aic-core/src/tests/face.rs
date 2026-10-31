use super::*;

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
