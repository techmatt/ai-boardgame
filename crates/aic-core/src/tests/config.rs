use super::*;
use crate::modules::board;

#[test]
fn the_shared_row_config_differs_only_in_the_threat_variant() {
    let a = Rules::new(&baseline()).unwrap();
    let b = Rules::new(&Config::from_json(SHARED_ROW).unwrap()).unwrap();
    assert_eq!(a.config.diff(&b.config), ["threats.variant: countries -> shared_row"]);
}

#[test]
fn bad_configs_are_rejected_with_a_reason() {
    let cases: &[(&str, &str)] = &[
        ("countries=6", "countries must be 1-5"),
        ("countries=5", "no map layout for 5 countries"),
        ("adversary.variant=nope", "unknown variant"),
        ("adversary.dificulty=3", "not a parameter"),
        ("adversary.difficulty=5", "outside"),
        ("board.war=0,2", "outside"),
        ("threats.per_country=1,2", "expected an integer"),
        ("ai.boards=7", "no AI board 7"),
        ("adversary.stage_cards=3,4", "give 3 counts"),
        ("adversary.stage_cards=9,4,3", "stage 1 has only"),
        ("alignment.unity_ease=5,2", "give tip counts"),
        ("threats=off", "needs module threats"),
        ("board=off", "needs module board"),
    ];
    for (o, want) in cases {
        let mut c = baseline();
        c.apply_override(o).unwrap();
        let err = Rules::new(&c).err().unwrap_or_else(|| panic!("{o} was accepted"));
        assert!(err.contains(want), "{o}: {err}");
    }
    fn module<'a>(c: &'a mut Config, id: &str) -> &'a mut ModuleConfig {
        c.modules.iter_mut().find(|m| m.module == id).unwrap()
    }
    let mut c = baseline();
    module(&mut c, "adversary").params.remove("difficulty");
    assert!(Rules::new(&c).err().unwrap().contains("adversary.difficulty: missing"));
    let mut c = baseline();
    module(&mut c, "cards").cards.push("nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("unknown card"));
    let mut c = baseline();
    c.round.push("adversary.nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("no phase"));
    let mut c = baseline();
    c.round.push("board.bases+board.war".into());
    assert!(Rules::new(&c).err().unwrap().contains("appears twice"));
    let mut c = baseline();
    c.modules.push(c.modules[0].clone());
    assert!(Rules::new(&c).err().unwrap().contains("appears twice"));
    assert!(Config::from_json(r#"{"countries": 3, "modules": [], "extra": 1}"#).is_err());
}

#[test]
fn list_parameters_accept_a_single_integer() {
    let rules = Rules::new(&baseline_with(&["ai.boards=1", "board.war=1"])).unwrap();
    let params = |m: &str| rules.config.module(m).unwrap().params.clone();
    assert_eq!(params("ai")["boards"], ParamValue::List(vec![1]));
    assert_eq!(params("board")["war"], ParamValue::List(vec![1]));
    // Every seat now plays the Protector, and every round is a war round.
    let w = rules.world();
    assert_eq!(modules::ai::board(&w, 0).id, "protector");
    assert_eq!(modules::ai::board(&w, 1).id, "protector");
    let mut g = rules.new_game(1);
    g.piles.remove(CURRENT);
    for r in 1..=5 {
        g.round = r;
        assert!(board::war_round(&w, &g), "round {r}");
    }
}

#[test]
fn config_hash_is_stable_and_ignores_the_name() {
    let rules = Rules::new(&baseline()).unwrap();
    // Resolving a resolved config changes nothing.
    let again = Rules::new(&rules.config).unwrap();
    assert_eq!(again.config, rules.config);
    assert_eq!(again.config_hash, rules.config_hash);
    let mut renamed = baseline();
    renamed.name = "other".into();
    assert_eq!(Rules::new(&renamed).unwrap().config_hash, rules.config_hash);
    // An omitted round resolves to module order, then phase order.
    let mut implicit = baseline();
    implicit.round.clear();
    let r = Rules::new(&implicit).unwrap();
    assert_eq!(&r.config.round[..3], ["board.act", "board.bases", "board.war"]);
    let tuned = baseline_with(&["adversary.difficulty=3"]);
    assert_ne!(Rules::new(&tuned).unwrap().config_hash, rules.config_hash);
}

#[test]
fn tunables_list_every_module_parameter() {
    let rules = Rules::new(&baseline()).unwrap();
    let t = rules.tunables();
    let find = |k: &str| t.iter().find(|x| x.key == k).unwrap_or_else(|| panic!("{k} missing"));
    for k in [
        "board.start_corrupted",
        "board.city_energy",
        "board.free_city_at",
        "board.align_energy",
        "adversary.difficulty",
        "adversary.supply_per_country",
        "adversary.corrupt_at",
        "threats.per_country",
        "alignment.tip_per_player",
        "alignment.unity_win",
        "economy.give_insight",
        "ai.start_credits",
        "cards.keep_cost",
    ] {
        assert_eq!(find(k).kind, "int", "{k}");
    }
    for k in ["board.war", "adversary.stage_cards", "alignment.unity_ease", "ai.boards"] {
        assert_eq!(find(k).kind, "list", "{k}");
    }
    let war = find("board.war");
    assert_eq!((war.min, war.max), (0, 1));
    assert_eq!(war.value, ParamValue::List(vec![0, 1, 0, 1, 0, 1, 1, 1]));
    // One per declared parameter: no duplicates.
    let keys: BTreeSet<&str> = t.iter().map(|x| x.key.as_str()).collect();
    assert_eq!(keys.len(), t.len());
}
