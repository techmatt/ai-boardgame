//! [`Rules`]: a config compiled against the module registry and content.
//! Build once per config, then run any number of games with it.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::action::Action;
use crate::config::{Config, ModuleConfig, ParamValue};
use crate::content::{content, Content};
use crate::event::Event;
use crate::hash::fnv1a64;
use crate::map::Map;
use crate::module::{Build, Cx, Module, ParamKind, PhaseKind, VariantDef, World};
use crate::modules;
use crate::rng::Stream;
use crate::state::{GameState, HexState, Outcome, Seat, Stats};
use crate::view::{self, View, ViewOut};

/// Hard stop for configs whose round never ends the game.
pub const MAX_ROUNDS: u32 = 1000;

/// Bounds for parameters that content cards refer to.
const CARD_PARAM_MIN: i64 = 0;
const CARD_PARAM_MAX: i64 = 1000;

pub struct Rules {
    /// The resolved config: validated, lists normalized, `round` filled in.
    pub config: Config,
    pub config_hash: String,
    pub map: Map,
    pub round: Vec<PhaseRef>,
    content: &'static Content,
    modules: Vec<Instance>,
}

/// One phase of the round: one module's phase, or several modules'
/// decision phases played as one simultaneous phase (`"a.play+b.build"`).
#[derive(Debug, Clone)]
pub struct PhaseRef {
    pub parts: Vec<PhasePart>,
    /// The round entry, e.g. `"actions.play"`.
    pub name: String,
    pub kind: PhaseKind,
}

#[derive(Debug, Clone, Copy)]
pub struct PhasePart {
    /// Index into the config's modules.
    pub module: usize,
    /// Index into that variant's phases.
    pub phase: usize,
}

/// A tunable of a configured module, with its current value.
#[derive(Debug, Clone, Serialize)]
pub struct Tunable {
    /// `"<module>.<param>"`, the key `Config::apply_override` takes.
    pub key: String,
    pub kind: &'static str,
    pub min: i64,
    pub max: i64,
    pub doc: &'static str,
    pub value: ParamValue,
}

struct Instance {
    id: &'static str,
    variant: &'static VariantDef,
    module: Box<dyn Module>,
    specs: Vec<Spec>,
}

impl Rules {
    pub fn new(config: &Config) -> Result<Rules, String> {
        let content = content();
        if !(1..=5).contains(&config.countries) {
            return Err(format!("countries must be 1-5, got {}", config.countries));
        }
        let map = Map::build(content, config.countries)?;
        let world = World { map: &map, content };

        let mut resolved = config.clone();
        let mut instances = Vec::new();
        let mut purposes = BTreeSet::new();
        let mut card_owner: BTreeMap<usize, &'static str> = BTreeMap::new();
        for mc in resolved.modules.iter_mut() {
            let def = modules::find(&mc.module).ok_or_else(|| {
                let known: Vec<&str> = modules::REGISTRY.iter().map(|m| m.id).collect();
                format!(
                    "unknown module {:?} (known: {})",
                    mc.module,
                    known.join(", ")
                )
            })?;
            if instances.iter().any(|i: &Instance| i.id == def.id) {
                return Err(format!("module {:?} appears twice", def.id));
            }
            let variant = def
                .variants
                .iter()
                .find(|v| v.name == mc.variant)
                .ok_or_else(|| {
                    let known: Vec<&str> = def.variants.iter().map(|v| v.name).collect();
                    format!(
                        "module {}: unknown variant {:?} (known: {})",
                        def.id,
                        mc.variant,
                        known.join(", ")
                    )
                })?;
            for p in variant.rng {
                if !purposes.insert(*p) {
                    return Err(format!(
                        "RNG purpose {p:?} is declared by two modules; each purpose needs its own stream"
                    ));
                }
            }
            let cards = resolve_cards(def.id, variant, mc, content)?;
            for &c in &cards {
                if let Some(other) = card_owner.insert(c, def.id) {
                    return Err(format!(
                        "card {:?} is used by both {other} and {}; a card belongs to one module",
                        content.cards[c].id, def.id
                    ));
                }
            }
            let specs = param_specs(variant, &cards, content);
            resolve_params(def.id, &specs, mc)?;
            let b = Build {
                module: def.id,
                params: &mc.params,
                cards,
                world,
                countries: config.countries,
            };
            let module = (variant.build)(&b).map_err(|e| format!("module {}: {e}", def.id))?;
            instances.push(Instance {
                id: def.id,
                variant,
                module,
                specs,
            });
        }

        if resolved.round.is_empty() {
            for i in &instances {
                for p in i.variant.phases {
                    resolved.round.push(format!("{}.{}", i.id, p.name));
                }
            }
        }
        let mut round = Vec::new();
        for name in &resolved.round {
            let mut parts = Vec::new();
            let mut kinds = Vec::new();
            for part in name.split('+') {
                let (m, p) = part
                    .split_once('.')
                    .ok_or_else(|| format!("round entry {name:?}: expected <module>.<phase>"))?;
                let module = instances.iter().position(|i| i.id == m).ok_or_else(|| {
                    format!("round entry {name:?}: module {m:?} is not in the config")
                })?;
                let phases = instances[module].variant.phases;
                let phase = phases.iter().position(|s| s.name == p).ok_or_else(|| {
                    let known: Vec<&str> = phases.iter().map(|s| s.name).collect();
                    format!(
                        "round entry {name:?}: {m} has no phase {p:?} (has: {})",
                        known.join(", ")
                    )
                })?;
                if parts.iter().any(|q: &PhasePart| q.module == module) {
                    return Err(format!("round entry {name:?}: {m} appears twice"));
                }
                parts.push(PhasePart { module, phase });
                kinds.push(phases[phase].kind);
            }
            if parts.len() > 1 && kinds.iter().any(|&k| k != PhaseKind::Decision) {
                return Err(format!(
                    "round entry {name:?}: only decision phases can be combined with '+'"
                ));
            }
            round.push(PhaseRef {
                parts,
                name: name.clone(),
                kind: kinds[0],
            });
        }
        if round.is_empty() {
            return Err("the round has no phases".into());
        }

        let config_hash = resolved.hash();
        Ok(Rules {
            config: resolved,
            config_hash,
            map,
            round,
            content,
            modules: instances,
        })
    }

    pub fn world(&self) -> World<'_> {
        World {
            map: &self.map,
            content: self.content,
        }
    }

    fn cx<'a>(&'a self, i: &Instance, events: Option<&'a mut Vec<Event>>) -> Cx<'a> {
        Cx {
            world: self.world(),
            module: i.id,
            purposes: i.variant.rng,
            events,
        }
    }

    /// Every tunable of this config, card-derived ones included, in module
    /// then declaration order. What a sweep can vary with `--set`.
    pub fn tunables(&self) -> Vec<Tunable> {
        let mut out = Vec::new();
        for (i, mc) in self.modules.iter().zip(&self.config.modules) {
            for s in &i.specs {
                out.push(Tunable {
                    key: format!("{}.{}", i.id, s.name),
                    kind: match s.kind {
                        ParamKind::Int => "int",
                        ParamKind::List => "list",
                    },
                    min: s.min,
                    max: s.max,
                    doc: s.doc,
                    value: mc.params[&s.name].clone(),
                });
            }
        }
        out
    }

    /// A fresh game, advanced to the first decision.
    pub fn new_game(&self, seed: u64) -> GameState {
        self.new_game_logged(seed, None)
    }

    pub fn new_game_logged(&self, seed: u64, mut events: Option<&mut Vec<Event>>) -> GameState {
        let mut rng = BTreeMap::new();
        for i in &self.modules {
            for p in i.variant.rng {
                rng.insert(p.to_string(), Stream::new(seed, p));
            }
        }
        let n = self.config.countries as usize;
        let seat = Seat {
            slots: vec![None; self.content.map.piece.card_slots as usize],
            ..Seat::default()
        };
        let mut g = GameState {
            seed,
            config: self.config.clone(),
            round: 0,
            phase: self.round.len(),
            seats: vec![seat; n],
            hexes: vec![HexState::default(); self.map.hexes.len()],
            vars: BTreeMap::new(),
            rng,
            stats: Stats::default(),
            outcome: None,
        };
        for i in &self.modules {
            i.module
                .setup(&mut g, &mut self.cx(i, events.as_deref_mut()));
        }
        self.enter(&mut g, events);
        g
    }

    /// Checks that a state (say, one deserialized from a client) fits these
    /// rules, so a mismatch is an error rather than a panic.
    pub fn check_state(&self, g: &GameState) -> Result<(), String> {
        if g.config != self.config {
            return Err("state was built from a different config".into());
        }
        if g.seats.len() != self.config.countries as usize || g.hexes.len() != self.map.hexes.len()
        {
            return Err("state does not fit the board (seat or hex count)".into());
        }
        let want: BTreeSet<&str> = self
            .modules
            .iter()
            .flat_map(|i| i.variant.rng.iter().copied())
            .collect();
        let have: BTreeSet<&str> = g.rng.keys().map(String::as_str).collect();
        if want != have {
            return Err(format!(
                "state RNG streams {have:?} do not match the config's {want:?}"
            ));
        }
        for (p, s) in &g.rng {
            if s.position().0 != fnv1a64(p.as_bytes()) {
                return Err(format!("RNG stream {p:?} has the wrong stream id"));
            }
        }
        if !g.is_over()
            && self
                .round
                .get(g.phase)
                .is_none_or(|p| p.kind != PhaseKind::Decision)
        {
            return Err(format!("state is not at a decision (phase {})", g.phase));
        }
        Ok(())
    }

    /// The current phase, or `None` once the game is over.
    pub fn phase(&self, g: &GameState) -> Option<&PhaseRef> {
        if g.is_over() {
            None
        } else {
            self.round.get(g.phase)
        }
    }

    /// The generic view of a state, for clients (see `view.rs`).
    pub fn view(&self, g: &GameState) -> View {
        let mut v = view::generic(self, g);
        let w = self.world();
        for i in &self.modules {
            i.module.view(
                g,
                &w,
                &mut ViewOut {
                    module: i.id,
                    view: &mut v,
                },
            );
        }
        view::finish(&mut v, g);
        v
    }

    pub fn legal_actions(&self, g: &GameState) -> Vec<Action> {
        let mut out = Vec::new();
        self.legal_actions_into(g, &mut out);
        out
    }

    /// Clears `out` and fills it with every legal action, seat by seat
    /// (each seat's actions from each module of the phase, then its pass).
    /// Empty once the game is over.
    pub fn legal_actions_into(&self, g: &GameState, out: &mut Vec<Action>) {
        out.clear();
        let Some(p) = self.phase(g) else { return };
        debug_assert_eq!(p.kind, PhaseKind::Decision);
        let w = self.world();
        for (s, seat) in g.seats.iter().enumerate() {
            if seat.passed {
                continue;
            }
            for part in &p.parts {
                self.modules[part.module]
                    .module
                    .legal(part.phase, g, &w, s as u8, out);
            }
            out.push(Action::Pass { seat: s as u8 });
        }
    }

    /// Applies an action in place. On error the state is unchanged. Pass
    /// `None` for `events` in hot loops to skip building the event log.
    pub fn apply_mut(
        &self,
        g: &mut GameState,
        a: &Action,
        mut events: Option<&mut Vec<Event>>,
    ) -> Result<(), String> {
        let p = self.phase(g).ok_or("the game is over")?;
        let seat = a.seat() as usize;
        if seat >= g.seats.len() {
            return Err(format!("no seat {seat}"));
        }
        if g.seats[seat].passed {
            return Err(format!("seat {seat} has passed this phase"));
        }
        match a {
            Action::Pass { .. } => {
                g.seats[seat].passed = true;
                if let Some(log) = events.as_deref_mut() {
                    log.push(Event::Passed { seat: seat as u8 });
                }
            }
            _ => {
                let part = p
                    .parts
                    .iter()
                    .find(|q| self.modules[q.module].module.owns(q.phase, a))
                    .ok_or_else(|| format!("no module in phase {} takes {a:?}", p.name))?;
                let inst = &self.modules[part.module];
                inst.module
                    .act(part.phase, g, &mut self.cx(inst, events.as_deref_mut()), a)?;
            }
        }
        g.stats.actions += 1;
        if g.seats.iter().all(|s| s.passed) {
            g.phase += 1;
            self.enter(g, events);
        }
        Ok(())
    }

    /// Applies an action to a copy; returns the new state and its events.
    pub fn apply(&self, g: &GameState, a: &Action) -> Result<(GameState, Vec<Event>), String> {
        let mut next = g.clone();
        let mut events = Vec::new();
        self.apply_mut(&mut next, a, Some(&mut events))?;
        Ok((next, events))
    }

    /// Runs phases from `g.phase` until a decision phase is entered or the
    /// game ends, starting new rounds as needed.
    fn enter(&self, g: &mut GameState, mut events: Option<&mut Vec<Event>>) {
        loop {
            if g.is_over() {
                return;
            }
            if g.phase >= self.round.len() {
                if g.round >= MAX_ROUNDS {
                    let (reason, detail) = (
                        "round_limit",
                        format!("no module ended the game within {MAX_ROUNDS} rounds"),
                    );
                    if let Some(log) = events.as_deref_mut() {
                        log.push(Event::GameEnded {
                            won: false,
                            round: g.round,
                            reason: reason.into(),
                            detail: detail.clone(),
                        });
                    }
                    g.outcome = Some(Outcome {
                        won: false,
                        round: g.round,
                        reason: reason.into(),
                        detail,
                    });
                    return;
                }
                g.round += 1;
                g.phase = 0;
                g.stats.adversary_ops.resize(g.round as usize, 0);
                let round = g.round;
                if let Some(log) = events.as_deref_mut() {
                    log.push(Event::RoundStarted { round });
                }
                for i in &self.modules {
                    i.module
                        .round_start(g, &mut self.cx(i, events.as_deref_mut()));
                }
                if g.is_over() {
                    return;
                }
            }
            let p = &self.round[g.phase];
            if let Some(log) = events.as_deref_mut() {
                log.push(Event::PhaseStarted {
                    phase: p.name.clone(),
                });
            }
            match p.kind {
                PhaseKind::Auto => {
                    let part = p.parts[0];
                    let inst = &self.modules[part.module];
                    inst.module
                        .run(part.phase, g, &mut self.cx(inst, events.as_deref_mut()));
                    g.phase += 1;
                }
                PhaseKind::Decision => {
                    for s in g.seats.iter_mut() {
                        s.passed = false;
                    }
                    return;
                }
            }
        }
    }
}

fn resolve_cards(
    module: &str,
    variant: &VariantDef,
    mc: &ModuleConfig,
    content: &Content,
) -> Result<Vec<usize>, String> {
    if !variant.uses_cards && !mc.cards.is_empty() {
        return Err(format!(
            "module {module}: variant {} takes no cards",
            variant.name
        ));
    }
    let mut out = Vec::new();
    for id in &mc.cards {
        let i = content
            .card_index(id)
            .ok_or_else(|| format!("module {module}: unknown card {id:?}"))?;
        if out.contains(&i) {
            return Err(format!("module {module}: card {id:?} listed twice"));
        }
        out.push(i);
    }
    Ok(out)
}

/// A parameter a module instance requires: a variant's declared spec, or
/// one synthesized for a parameter that a selected card refers to.
struct Spec {
    name: String,
    kind: ParamKind,
    min: i64,
    max: i64,
    doc: &'static str,
}

/// The variant's own parameters plus every parameter its cards refer to.
fn param_specs(variant: &VariantDef, cards: &[usize], content: &Content) -> Vec<Spec> {
    let mut specs: Vec<Spec> = variant
        .params
        .iter()
        .map(|p| Spec {
            name: p.name.to_string(),
            kind: p.kind,
            min: p.min,
            max: p.max,
            doc: p.doc,
        })
        .collect();
    for &i in cards {
        for name in content.cards[i].params() {
            if !specs.iter().any(|s| s.name == name) {
                specs.push(Spec {
                    name: name.to_string(),
                    kind: ParamKind::Int,
                    min: CARD_PARAM_MIN,
                    max: CARD_PARAM_MAX,
                    doc: "referred to by a card",
                });
            }
        }
    }
    specs
}

/// Checks the config's parameters against `specs` and normalizes them
/// (a single integer for a list parameter becomes a one-item list).
fn resolve_params(module: &str, specs: &[Spec], mc: &mut ModuleConfig) -> Result<(), String> {
    for name in mc.params.keys() {
        if name == "variant" || !specs.iter().any(|s| s.name == *name) {
            let known: Vec<&str> = specs.iter().map(|s| s.name.as_str()).collect();
            return Err(format!(
                "{module}.{name}: not a parameter of variant {} (parameters: {})",
                mc.variant,
                if known.is_empty() {
                    "none".to_string()
                } else {
                    known.join(", ")
                }
            ));
        }
    }
    for s in specs {
        let v = mc.params.get_mut(&s.name).ok_or_else(|| {
            format!(
                "{module}.{}: missing (every parameter must be set in config: {})",
                s.name, s.doc
            )
        })?;
        let check = |x: i64| -> Result<(), String> {
            if x < s.min || x > s.max {
                Err(format!(
                    "{module}.{}: {x} is outside {}..={}",
                    s.name, s.min, s.max
                ))
            } else {
                Ok(())
            }
        };
        match (s.kind, &*v) {
            (ParamKind::Int, ParamValue::Int(x)) => check(*x)?,
            (ParamKind::Int, ParamValue::List(_)) => {
                return Err(format!(
                    "{module}.{}: expected an integer, got a list",
                    s.name
                ))
            }
            (ParamKind::List, ParamValue::Int(x)) => {
                check(*x)?;
                *v = ParamValue::List(vec![*x]);
            }
            (ParamKind::List, ParamValue::List(xs)) => {
                if xs.is_empty() {
                    return Err(format!("{module}.{}: list is empty", s.name));
                }
                xs.iter().try_for_each(|&x| check(x))?;
            }
        }
    }
    Ok(())
}
