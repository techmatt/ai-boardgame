//! `aic-sim docs [--check]`: the parts of the rules docs generated from
//! content, configs and the op schema, so a number or a card has one home.
//!
//! `docs/rules.md` holds blocks between `<!-- generated: NAME -->` and
//! `<!-- /generated -->`; `docs/ops.md` and `docs/harms.md` are generated
//! whole (the ledger's checks fail the run); `web/card/faces.json`, every
//! card's face from its ops (`aic_core::face`), too. `--check` changes
//! nothing and exits 1 if anything is stale, or if `rules.md` backticks a
//! `module.param` that isn't a tunable. The fast test lane runs the check.

use std::collections::BTreeMap;
use std::fs;
use std::process::exit;

use aic_core::config::ParamValue;
use aic_core::content::{content, Cost, Op, Resource};
use aic_core::face::card_face;
use aic_core::Rules;

use crate::cli::{die, load, rules, Options};

const BASELINE: &str = "configs/baseline.json";
const RULES_MD: &str = "docs/rules.md";
const OPS_MD: &str = "docs/ops.md";
const OP_SOURCE: &str = "crates/aic-core/src/op.rs";
const HARMS_MD: &str = "docs/harms.md";
const FACES_JSON: &str = "web/card/faces.json";

/// The harms of the ledger, in its row order.
pub const HARMS: [&str; 14] = [
    "presence destroyed",
    "credits or income lost",
    "cards discarded",
    "insight lost",
    "ambassadors lost",
    "tokens stripped from cards",
    "buildings, stations or factories destroyed or seized",
    "production blacked out",
    "adversary pieces placed",
    "projects advanced (compute)",
    "extra threats",
    "Discord sway",
    "Unity pulled back",
    "actions or claims blocked",
];

/// The sources of the ledger, in its column order. The first five read
/// content; `RULE_HARMS` says what the rest do.
pub const SOURCES: [&str; 10] = [
    "tier-1 threats",
    "tier-2 threats",
    "Discord cards",
    "adversary twists",
    "adversary projects",
    "arrivals",
    "build and spread",
    "corruption",
    "war",
    "march",
];

/// What the board rules (the sources content doesn't print) do: source, harm,
/// and the rule, as the ledger's note. Update this when an adversary rule changes.
pub const RULE_HARMS: [(&str, &str, &str); 16] = [
    ("arrivals", "adversary pieces placed", "every matching site gains an ambassador"),
    ("arrivals", "ambassadors lost", "an arrival cancels a player ambassador there"),
    ("arrivals", "Discord sway", "an ambassador on a base sways Discord when it produces"),
    ("arrivals", "production blacked out", "an ambassador on a city or station diverts it"),
    ("build and spread", "adversary pieces placed", "cities build; surplus spreads; two meeting make a node"),
    ("build and spread", "projects advanced (compute)", "each node makes compute"),
    ("build and spread", "actions or claims blocked", "a node blocks site claims next to it"),
    ("build and spread", "insight lost", "an ambassador on a lone presence drains insight"),
    ("build and spread", "ambassadors lost", "a build or spread cancels a player ambassador there"),
    ("corruption", "presence destroyed", "presence on a city that corrupts is destroyed"),
    ("corruption", "ambassadors lost", "player ambassadors on it are lost"),
    ("corruption", "production blacked out", "its energy goes to the adversary"),
    ("corruption", "Discord sway", "its energy sways Discord each round"),
    ("war", "presence destroyed", "a destroyed factory takes the presence on its hex"),
    ("war", "buildings, stations or factories destroyed or seized", "surplus troops destroy factories"),
    ("march", "buildings, stations or factories destroyed or seized", "a troop strikes player-owned buildings"),
];

/// The harms one op does.
fn op_harms(op: &Op) -> Vec<&'static str> {
    match op {
        Op::DestroyPresence | Op::EachDestroyPresence { .. } | Op::LosePresenceOr { .. } => vec![HARMS[0]],
        Op::Sacrifice => vec![HARMS[0], HARMS[4]],
        Op::PlayersLose { resource: Resource::Credits, .. } => vec![HARMS[1]],
        Op::Requisition { .. } | Op::DiscardMostExpensive => vec![HARMS[2]],
        Op::LoseInsight { .. } => vec![HARMS[3]],
        Op::LoseAmbassador => vec![HARMS[4]],
        Op::LoseTokens { .. } => vec![HARMS[5]],
        Op::FlipStation { .. } => vec![HARMS[6]],
        Op::Flag { name } => match name.name() {
            "embargo" => vec![HARMS[13]],
            _ => vec![HARMS[7]],
        },
        Op::Ambassadors { .. }
        | Op::Infiltrate { .. }
        | Op::NodeAmbassadors { .. }
        | Op::CleanCityAmbassadors { .. }
        | Op::AdversaryBuild
        | Op::AdversarySpread
        | Op::CorruptCity
        | Op::CityBuild { .. }
        | Op::CleanFactories { .. }
        | Op::StationBuild { .. } => vec![HARMS[8]],
        Op::Troops { destroy, .. } => {
            if *destroy {
                vec![HARMS[8], HARMS[0]]
            } else {
                vec![HARMS[8]]
            }
        }
        Op::Compute { .. } => vec![HARMS[9]],
        Op::DealThreats { .. } => vec![HARMS[10]],
        Op::Sway { side, .. } if side.name() == "discord" => vec![HARMS[11]],
        Op::PullUnity { .. } => vec![HARMS[12]],
        _ => Vec::new(),
    }
}

/// `(harm, source)` marks with how many printings or the rule's note.
fn harm_marks() -> BTreeMap<(usize, usize), String> {
    let c = content();
    let mut counts: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut add = |src: usize, ops: &[Op]| {
        let mut seen = Vec::new();
        for o in ops {
            for h in op_harms(o) {
                if !seen.contains(&h) {
                    seen.push(h);
                    let i = HARMS.iter().position(|x| *x == h).expect("a harm");
                    *counts.entry((i, src)).or_default() += 1;
                }
            }
        }
    };
    for t in &c.threats {
        add(t.tier as usize - 1, &t.effects);
    }
    for a in c.alignment.iter().filter(|a| a.side.name() == "discord") {
        let mut all = a.effects.clone();
        for s in &a.steps {
            all.extend(s.effects.iter().cloned());
        }
        add(2, &all);
    }
    for a in &c.adversary {
        add(3, &a.twists);
    }
    for p in &c.projects {
        add(4, &p.effects);
    }
    let mut out: BTreeMap<(usize, usize), String> =
        counts.into_iter().map(|(k, n)| (k, n.to_string())).collect();
    for (src, harm, _) in RULE_HARMS {
        let s = SOURCES.iter().position(|x| *x == src).expect("a source");
        let h = HARMS.iter().position(|x| *x == harm).expect("a harm");
        out.insert((h, s), "yes".to_string());
    }
    out
}

/// What fails the ledger's distribution checks.
pub fn harm_failures() -> Vec<String> {
    let marks = harm_marks();
    let mut bad = Vec::new();
    for (h, name) in HARMS.iter().enumerate() {
        let n = (0..SOURCES.len()).filter(|&s| marks.contains_key(&(h, s))).count();
        if n < 2 {
            bad.push(format!("{name}: in {n} kind(s) of source, needs 2"));
        }
    }
    for s in 0..2 {
        let n = (0..HARMS.len()).filter(|&h| marks.contains_key(&(h, s))).count();
        if n < 5 {
            bad.push(format!("{}: {n} kinds of harm, needs 5", SOURCES[s]));
        }
    }
    let all: usize = (0..SOURCES.len())
        .filter_map(|s| marks.get(&(0, s)))
        .map(|m| m.parse::<usize>().unwrap_or(1))
        .sum();
    for s in 0..SOURCES.len() {
        if let Some(m) = marks.get(&(0, s)) {
            let n = m.parse::<usize>().unwrap_or(1);
            if 2 * n > all + 1 {
                bad.push(format!("{}: {n} of {all} printed presence destructions", SOURCES[s]));
            }
        }
    }
    bad
}

/// `docs/harms.md`: the ledger, from content and `RULE_HARMS`.
fn harms_md() -> String {
    let marks = harm_marks();
    let mut head = vec!["Harm"];
    head.extend(SOURCES);
    let rows = HARMS
        .iter()
        .enumerate()
        .map(|(h, name)| {
            let mut r = vec![name.to_string()];
            r.extend((0..SOURCES.len()).map(|s| marks.get(&(h, s)).cloned().unwrap_or_default()));
            r
        })
        .collect();
    let notes: String = RULE_HARMS
        .iter()
        .map(|(s, h, why)| format!("- **{s}, {h}:** {why}.\n"))
        .collect();
    format!(
        "# The harms ledger\n\nGenerated by `aic-sim docs` from `content/` and `RULE_HARMS` in \
         `crates/aic-sim/src/docs.rs`: edit those, not this file. Keep it current: a change to \
         threats, Discord cards or adversary rules that adds or moves a harm updates \
         `RULE_HARMS` or `op_harms`.\n\nOne row per harm, one column per kind of source. A number \
         is how many printed cards do it (a threat, Discord card, twist or project counts once); \
         `yes` is a rule of the board. `Discord cards` covers both tiers; `march` is the adversary \
         troops' strikes, which take buildings only at `board.troop_presence` 4. The `yes` cells \
         are true at the baseline's numbers (`board.destruction_discord` and \
         `board.ambassador_discord` are 0, so neither war nor an ambassador on a clean city sways).\n\n{}\n\
         **Checks** (`aic-sim docs --check`, and a fast test): every harm in at least 2 kinds of \
         source; each threat tier at least 5 kinds of harm; no kind of source more than about half \
         of the printed presence destruction. Measured shares are in the 028 screen.\n\n\
         ## The board's rules\n\n{notes}",
        table(&head, rows)
    )
}

/// `3 credits, 1 energy`.
fn costs(c: &[Cost]) -> String {
    let parts: Vec<String> = c.iter().map(|(r, n)| r.amount(*n)).collect();
    parts.join(", ")
}

/// Escapes a table cell.
fn cell(s: &str) -> String {
    s.replace('|', "\\|")
}

fn table(head: &[&str], rows: Vec<Vec<String>>) -> String {
    let mut out = format!("| {} |\n|{}\n", head.join(" | "), "---|".repeat(head.len()));
    for r in rows {
        let r: Vec<String> = r.iter().map(|c| cell(c)).collect();
        out.push_str(&format!("| {} |\n", r.join(" | ")));
    }
    out
}

fn threats_block() -> String {
    let rows = content()
        .threats
        .iter()
        .map(|t| vec![t.name.clone(), t.tier.to_string(), t.text.clone(), costs(&t.counter)])
        .collect();
    table(&["Threat", "Tier", "If not countered", "Counter"], rows)
}

fn adversary_block() -> String {
    let rows = content()
        .adversary
        .iter()
        .map(|a| {
            vec![
                a.name.clone(),
                a.stage.to_string(),
                a.text.clone(),
                if a.escalation { "yes" } else { "" }.to_string(),
            ]
        })
        .collect();
    table(&["Card", "Stage", "Twist", "Escalation"], rows)
}

fn alignment_block() -> String {
    let rows = content()
        .alignment
        .iter()
        .map(|a| {
            let steps: Vec<String> = a.steps.iter().map(|s| s.at.to_string()).collect();
            vec![
                a.name.clone(),
                a.side.to_string(),
                a.tier.to_string(),
                a.text.clone(),
                steps.join(", "),
            ]
        })
        .collect();
    table(&["Card", "Side", "Tier", "Text", "Stronger from tip"], rows)
}

fn projects_block() -> String {
    let rows = content()
        .projects
        .iter()
        .map(|p| {
            vec![
                p.name.clone(),
                p.stage.to_string(),
                p.cost.to_string(),
                if p.ongoing { "ongoing" } else { "once" }.to_string(),
                p.text.clone(),
            ]
        })
        .collect();
    table(&["Project", "Stage", "Compute per player", "Resolves", "Text"], rows)
}

fn value(v: &ParamValue) -> String {
    match v {
        ParamValue::Int(n) => n.to_string(),
        ParamValue::List(l) => {
            let l: Vec<String> = l.iter().map(|n| n.to_string()).collect();
            l.join(", ")
        }
    }
}

fn tunables_block(r: &Rules) -> String {
    let rows = r
        .tunables()
        .iter()
        .map(|t| {
            vec![
                format!("`{}`", t.key),
                value(&t.value),
                format!("{}..{}", t.min, t.max),
                t.doc.to_string(),
            ]
        })
        .collect();
    table(&["Parameter", "Baseline", "Range", "What it does"], rows)
}

fn snake(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// One op's source: its doc comment and name, read from the `Op` enum.
struct OpDoc {
    section: String,
    name: String,
    doc: String,
}

/// Reads `enum Op` from `op.rs`: section comments, doc comments, variants.
fn op_docs(src: &str) -> Vec<OpDoc> {
    let body = src
        .split_once("pub enum Op {")
        .expect("enum Op in op.rs")
        .1
        .split_once("\n}\n")
        .expect("enum Op ends")
        .0;
    let (mut out, mut doc, mut section) = (Vec::new(), Vec::<String>::new(), String::new());
    for line in body.lines() {
        let Some(l) = line.strip_prefix("    ") else {
            continue;
        };
        if let Some(s) = l.strip_prefix("// ---------- ") {
            section = s.trim_end_matches(" ----------").to_string();
        } else if let Some(d) = l.strip_prefix("/// ") {
            doc.push(d.to_string());
        } else if l.starts_with(|c: char| c.is_uppercase()) {
            let name: String = l.chars().take_while(|c| c.is_alphanumeric()).collect();
            out.push(OpDoc {
                section: section.clone(),
                name: snake(&name),
                doc: doc.join(" "),
            });
            doc.clear();
        }
    }
    out
}

/// How many times each op is printed in content.
fn op_uses() -> BTreeMap<String, usize> {
    fn walk(v: &serde_json::Value, out: &mut BTreeMap<String, usize>) {
        match v {
            serde_json::Value::Object(o) => {
                if let Some(n) = o.get("op").and_then(|n| n.as_str()) {
                    *out.entry(n.to_string()).or_default() += 1;
                }
                o.values().for_each(|x| walk(x, out));
            }
            serde_json::Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            _ => {}
        }
    }
    let c = content();
    let mut out = BTreeMap::new();
    fn add<T: serde::Serialize>(ops: &[T], out: &mut BTreeMap<String, usize>) {
        walk(&serde_json::to_value(ops).expect("ops serialize"), out);
    }
    for d in &c.cards {
        add(&d.effects, &mut out);
        add(&d.passives, &mut out);
        if let Some(a) = &d.ability {
            add(&a.effects, &mut out);
        }
    }
    for t in &c.threats {
        add(&t.effects, &mut out);
    }
    for a in &c.adversary {
        add(&a.twists, &mut out);
    }
    for a in &c.alignment {
        add(&a.effects, &mut out);
        for s in &a.steps {
            add(&s.effects, &mut out);
        }
    }
    for p in &c.projects {
        add(&p.effects, &mut out);
    }
    for b in &c.ais {
        add(&b.innate.passives, &mut out);
        for g in &b.growth {
            add(&g.effects, &mut out);
        }
        for p in &b.powers {
            add(&p.effects, &mut out);
        }
    }
    out
}

fn ops_md(src: &str) -> String {
    let uses = op_uses();
    let mut out = String::from(
        "# Ops\n\nGenerated by `aic-sim docs` from `crates/aic-core/src/op.rs` and `content/`; \
         don't edit. An op is one thing content can print (`{\"op\": \"gain\", ...}`). \
         **Printed** counts it across all content: 0 means rules code with nothing using it yet.\n",
    );
    let mut section = String::new();
    let mut rows = Vec::new();
    let mut flush = |section: &str, rows: &mut Vec<Vec<String>>| {
        if !rows.is_empty() {
            out.push_str(&format!("\n## {section}\n\n"));
            out.push_str(&table(&["Op", "Printed", "What it does"], std::mem::take(rows)));
        }
    };
    for o in op_docs(src) {
        if o.section != section {
            flush(&section, &mut rows);
            section = o.section.clone();
        }
        rows.push(vec![
            format!("`{}`", o.name),
            uses.get(&o.name).copied().unwrap_or(0).to_string(),
            o.doc,
        ]);
    }
    flush(&section, &mut rows);
    out
}

/// Every card's face, one card a line, so a change shows as that card's diff.
fn faces_json() -> String {
    let rows: Vec<String> = content()
        .cards
        .iter()
        .map(|c| {
            let face = serde_json::to_string(&card_face(c)).expect("a face is JSON");
            format!(" {}: {face}", serde_json::to_string(&c.id).expect("an id is JSON"))
        })
        .collect();
    format!("{{
{}
}}
", rows.join(",
"))
}

const OPEN: &str = "<!-- generated: ";
const CLOSE: &str = "<!-- /generated -->";

/// `text` with every generated block replaced by what `block` says; panics
/// (dies) on an unknown name.
fn fill(text: &str, block: &dyn Fn(&str) -> Option<String>) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(i) = rest.find(OPEN) {
        let after = &rest[i + OPEN.len()..];
        let Some(end) = after.find(" -->") else {
            die("docs: unclosed generated marker")
        };
        let name = &after[..end];
        let Some(body) = block(name) else {
            die(&format!("docs: unknown generated block {name:?}"))
        };
        let Some(close) = after.find(CLOSE) else {
            die(&format!("docs: block {name:?} has no {CLOSE}"))
        };
        out.push_str(&rest[..i]);
        out.push_str(&format!("{OPEN}{name} -->\n{body}{CLOSE}"));
        rest = &after[close + CLOSE.len()..];
    }
    out.push_str(rest);
    out
}

/// Dotted names rules.md may print that are counters, not tunables or phases.
const COUNTERS: [&str; 1] = ["threats.short"];

/// Backticked `module.name` words in `text` that are no tunable, phase or
/// counter of the baseline.
fn unknown_params(text: &str, r: &Rules) -> Vec<String> {
    let mut known: Vec<String> = r.tunables().into_iter().map(|t| t.key).collect();
    let modules: Vec<String> = known
        .iter()
        .filter_map(|k| k.split('.').next().map(str::to_string))
        .collect();
    for m in aic_core::describe_modules().as_array().expect("modules") {
        for v in m["variants"].as_array().expect("variants") {
            for p in v["phases"].as_array().expect("phases") {
                known.push(format!("{}.{}", m["id"].as_str().unwrap(), p["name"].as_str().unwrap()));
            }
        }
    }
    known.extend(COUNTERS.iter().map(|c| c.to_string()));
    let mut out = Vec::new();
    for tick in text.split('`').skip(1).step_by(2) {
        if let Some((m, p)) = tick.split_once('.') {
            let word = !p.is_empty() && p.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit());
            if modules.iter().any(|x| x == m) && word && !known.iter().any(|k| k == tick) {
                out.push(tick.to_string());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Regenerates the docs, or with `--check` reports what is stale.
pub fn run(o: &Options) {
    let r = rules(BASELINE, &load(BASELINE, &[]));
    let src = fs::read_to_string(OP_SOURCE).unwrap_or_else(|e| die(&format!("{OP_SOURCE}: {e}")));
    let block = |name: &str| match name {
        "threats" => Some(threats_block()),
        "adversary" => Some(adversary_block()),
        "alignment" => Some(alignment_block()),
        "projects" => Some(projects_block()),
        "tunables" => Some(tunables_block(&r)),
        _ => None,
    };
    let old = fs::read_to_string(RULES_MD).unwrap_or_else(|e| die(&format!("{RULES_MD}: {e}")));
    let new = fill(&old, &block);
    let ops = ops_md(&src);
    let old_ops = fs::read_to_string(OPS_MD).unwrap_or_default();
    let harms = harms_md();
    let old_harms = fs::read_to_string(HARMS_MD).unwrap_or_default();
    let faces = faces_json();
    let old_faces = fs::read_to_string(FACES_JSON).unwrap_or_default();
    let mut stale = Vec::new();
    if faces != old_faces {
        stale.push(FACES_JSON);
    }
    if new != old {
        stale.push(RULES_MD);
    }
    if ops != old_ops {
        stale.push(OPS_MD);
    }
    if harms != old_harms {
        stale.push(HARMS_MD);
    }
    let failures = harm_failures();
    for f in &failures {
        eprintln!("{HARMS_MD}: {f}");
    }
    if o.check {
        let bad = unknown_params(&old, &r);
        for b in &bad {
            eprintln!("{RULES_MD}: `{b}` is not a tunable");
        }
        for s in &stale {
            eprintln!("{s} is stale: run aic-sim docs");
        }
        if !stale.is_empty() || !bad.is_empty() || !failures.is_empty() {
            exit(1);
        }
        return;
    }
    if new != old {
        fs::write(RULES_MD, new).unwrap_or_else(|e| die(&format!("{RULES_MD}: {e}")));
    }
    if ops != old_ops {
        fs::write(OPS_MD, ops).unwrap_or_else(|e| die(&format!("{OPS_MD}: {e}")));
    }
    if faces != old_faces {
        fs::write(FACES_JSON, faces).unwrap_or_else(|e| die(&format!("{FACES_JSON}: {e}")));
    }
    if harms != old_harms {
        fs::write(HARMS_MD, harms).unwrap_or_else(|e| die(&format!("{HARMS_MD}: {e}")));
    }
}
