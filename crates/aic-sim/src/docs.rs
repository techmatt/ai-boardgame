//! `aic-sim docs [--check]`: the parts of the rules docs generated from
//! content, configs and the op schema, so a number or a card has one home.
//!
//! `docs/rules.md` holds blocks between `<!-- generated: NAME -->` and
//! `<!-- /generated -->`; `docs/ops.md` is generated whole. `--check` changes
//! nothing and exits 1 if anything is stale, or if `rules.md` backticks a
//! `module.param` that isn't a tunable. The fast test lane runs the check.

use std::collections::BTreeMap;
use std::fs;
use std::process::exit;

use aic_core::config::ParamValue;
use aic_core::content::{content, Cost};
use aic_core::Rules;

use crate::cli::{die, load, rules, Options};

const BASELINE: &str = "configs/baseline.json";
const RULES_MD: &str = "docs/rules.md";
const OPS_MD: &str = "docs/ops.md";
const OP_SOURCE: &str = "crates/aic-core/src/op.rs";

/// `3 credits, 1 energy`.
fn costs(c: &[Cost]) -> String {
    let parts: Vec<String> = c.iter().map(|(r, n)| format!("{n} {r}")).collect();
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
    let mut stale = Vec::new();
    if new != old {
        stale.push(RULES_MD);
    }
    if ops != old_ops {
        stale.push(OPS_MD);
    }
    if o.check {
        let bad = unknown_params(&old, &r);
        for b in &bad {
            eprintln!("{RULES_MD}: `{b}` is not a tunable");
        }
        for s in &stale {
            eprintln!("{s} is stale: run aic-sim docs");
        }
        if !stale.is_empty() || !bad.is_empty() {
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
}
