//! The module framework. A game is a set of modules, each in one variant;
//! each variant declares its tunable parameters, the phases it contributes
//! to the round, and the RNG purposes it draws from, and builds a [`Module`]
//! (its typed, resolved parameters plus behaviour) from config.
//!
//! See `docs/architecture.md` for how to add a module or a variant.

use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::action::Action;
use crate::config::ParamValue;
use crate::content::{Amount, Content};
use crate::event::Event;
use crate::map::Map;
use crate::rng::Stream;
use crate::state::{GameState, Outcome};
use crate::view::ViewOut;

/// A module: an id, a one-line summary and its variants. Registered in
/// `modules::REGISTRY`.
pub struct ModuleDef {
    pub id: &'static str,
    pub summary: &'static str,
    pub variants: &'static [VariantDef],
}

pub struct VariantDef {
    pub name: &'static str,
    pub summary: &'static str,
    pub params: &'static [ParamSpec],
    pub phases: &'static [PhaseSpec],
    /// RNG purposes this variant draws from. Each gets its own stream.
    pub rng: &'static [&'static str],
    /// Whether the variant plays content cards (config `cards`). Card
    /// parameter references become required parameters of this module.
    pub uses_cards: bool,
    pub build: fn(&Build) -> Result<Box<dyn Module>, String>,
}

#[derive(Debug, Clone, Copy)]
pub struct ParamSpec {
    pub name: &'static str,
    pub kind: ParamKind,
    pub min: i64,
    pub max: i64,
    pub doc: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    Int,
    /// A non-empty list; a single integer in config is read as a one-item list.
    List,
}

#[derive(Debug, Clone, Copy)]
pub struct PhaseSpec {
    pub name: &'static str,
    pub kind: PhaseKind,
    pub doc: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseKind {
    /// Resolved by the module as soon as it is reached; no player input.
    Auto,
    /// Simultaneous player phase: every seat may act, in any order, until
    /// every seat has passed. A seat's actions must not change what another
    /// seat may do or what its actions do.
    Decision,
}

/// Behaviour of a built module. Phase arguments are indices into the
/// variant's `phases`. Every method is deterministic: randomness only via
/// [`Cx::rng`], and only from purposes the variant declared.
pub trait Module: Send + Sync + Debug {
    /// Once, when the game is created, before round 1.
    fn setup(&self, _g: &mut GameState, _cx: &mut Cx) {}
    /// At the start of every round, before its first phase.
    fn round_start(&self, _g: &mut GameState, _cx: &mut Cx) {}
    /// Resolve an `Auto` phase.
    fn run(&self, _phase: usize, _g: &mut GameState, _cx: &mut Cx) {}
    /// Append `seat`'s legal actions (other than `Pass`, which the engine
    /// adds) in a `Decision` phase.
    fn legal(&self, _phase: usize, _g: &GameState, _w: &World, _seat: u8, _out: &mut Vec<Action>) {}
    /// Whether a non-pass action in this decision phase is this module's to
    /// handle. Several modules can share a decision phase (`"a.x+b.y"` in the
    /// round); the engine routes each action to the module that owns it.
    fn owns(&self, _phase: usize, _action: &Action) -> bool {
        false
    }
    /// Validate and apply a non-pass action this module owns. Must leave
    /// the state untouched when it returns an error.
    fn act(
        &self,
        _phase: usize,
        _g: &mut GameState,
        _cx: &mut Cx,
        action: &Action,
    ) -> Result<(), String> {
        Err(format!("this phase takes no action like {action:?}"))
    }
    /// Add derived values, card groups or piles to the generic view (see
    /// `view.rs`). Tokens, resources, `vars` and exhausted cards already
    /// show without this. Display only: the rules never read the view.
    fn view(&self, _g: &GameState, _w: &World, _out: &mut ViewOut) {}
}

/// Read-only game data shared by every game of a config.
#[derive(Clone, Copy)]
pub struct World<'a> {
    pub map: &'a Map,
    pub content: &'a Content,
}

/// What a module gets while it resolves: the world, its own id (for
/// namespacing counters and vars), its RNG purposes and the event log.
pub struct Cx<'a> {
    pub world: World<'a>,
    pub module: &'static str,
    pub(crate) purposes: &'static [&'static str],
    pub(crate) events: Option<&'a mut Vec<Event>>,
}

impl<'a> Cx<'a> {
    pub fn emit(&mut self, e: impl FnOnce() -> Event) {
        if let Some(log) = self.events.as_deref_mut() {
            log.push(e());
        }
    }

    /// Adds `delta` tokens (negative removes, never below zero). Returns the
    /// change actually made.
    pub fn add_tokens(&mut self, g: &mut GameState, hex: usize, kind: &str, delta: i64) -> i64 {
        let cur = g.tokens(hex, kind);
        let next = (cur + delta).max(0);
        if next == cur {
            return 0;
        }
        if next == 0 {
            g.hexes[hex].tokens.remove(kind);
        } else {
            *g.hexes[hex].tokens.entry(kind.to_string()).or_insert(0) = next;
        }
        let at = self.world.map.hexes[hex].hex;
        self.emit(|| Event::TokensChanged {
            hex: at,
            token: kind.to_string(),
            delta: next - cur,
            total: next,
        });
        next - cur
    }

    pub fn add_resource(&mut self, g: &mut GameState, seat: usize, name: &str, delta: i64) {
        if delta == 0 {
            return;
        }
        let v = g.seats[seat].resources.entry(name.to_string()).or_insert(0);
        *v += delta;
        let total = *v;
        self.emit(|| Event::ResourceChanged {
            seat: seat as u8,
            resource: name.to_string(),
            delta,
            total,
        });
    }

    /// Records `n` adversary operations in the current round. Operations
    /// during setup (round 0) go to `stats.setup_ops` instead.
    pub fn adversary_ops(&mut self, g: &mut GameState, n: u32) {
        let r = g.round as usize;
        if r == 0 {
            g.stats.setup_ops += n;
            return;
        }
        if g.stats.adversary_ops.len() < r {
            g.stats.adversary_ops.resize(r, 0);
        }
        g.stats.adversary_ops[r - 1] += n;
    }

    /// Adds to the counter `"<module>.<name>"`.
    pub fn count(&mut self, g: &mut GameState, name: &str, n: i64) {
        *g.stats
            .counters
            .entry(format!("{}.{name}", self.module))
            .or_insert(0) += n;
    }

    /// This module's track `"<module>.<name>"`.
    pub fn var(&self, g: &GameState, name: &str) -> i64 {
        g.var(&format!("{}.{name}", self.module))
    }

    pub fn set_var(&mut self, g: &mut GameState, name: &str, v: i64) {
        let key = format!("{}.{name}", self.module);
        let old = g.vars.insert(key.clone(), v).unwrap_or(0);
        self.emit(|| Event::VarChanged {
            key,
            delta: v - old,
            total: v,
        });
    }

    /// Marks a card used (face down) for `seat`. Refreshing is the owning
    /// module's job (usually in `round_start`).
    pub fn exhaust(&mut self, g: &mut GameState, seat: usize, card: &str) {
        let ex = &mut g.seats[seat].exhausted;
        if let Err(at) = ex.binary_search_by(|c| c.as_str().cmp(card)) {
            ex.insert(at, card.to_string());
        }
    }

    pub fn refresh(&mut self, g: &mut GameState, seat: usize, card: &str) {
        g.seats[seat].exhausted.retain(|c| c != card);
    }

    /// The stream for one of this variant's declared purposes. Drawing from
    /// any other purpose, even one another module declared, panics.
    pub fn rng<'g>(&self, g: &'g mut GameState, purpose: &str) -> &'g mut Stream {
        if !self.purposes.contains(&purpose) {
            panic!(
                "module {} drew from RNG purpose {purpose:?}, which it did not declare",
                self.module
            );
        }
        g.rng
            .get_mut(purpose)
            .expect("declared purposes have streams")
    }

    /// Ends the game (first call wins).
    pub fn end(&mut self, g: &mut GameState, won: bool, reason: &str, detail: String) {
        if g.outcome.is_some() {
            return;
        }
        let round = g.round;
        self.emit(|| Event::GameEnded {
            won,
            round,
            reason: reason.to_string(),
            detail: detail.clone(),
        });
        g.outcome = Some(Outcome {
            won,
            round,
            reason: reason.to_string(),
            detail,
        });
    }
}

/// Input to a variant's `build`: its validated parameters and content.
pub struct Build<'a> {
    pub module: &'static str,
    pub params: &'a BTreeMap<String, ParamValue>,
    /// Content indices of the cards selected in config, in config order.
    pub cards: Vec<usize>,
    pub world: World<'a>,
    pub countries: u8,
}

impl Build<'_> {
    pub fn int(&self, name: &str) -> Result<i64, String> {
        match self.params.get(name) {
            Some(ParamValue::Int(v)) => Ok(*v),
            Some(ParamValue::List(_)) => Err(format!(
                "{}.{name}: expected an integer, got a list",
                self.module
            )),
            None => Err(format!("{}.{name}: missing", self.module)),
        }
    }

    pub fn list(&self, name: &str) -> Result<Vec<i64>, String> {
        match self.params.get(name) {
            Some(ParamValue::List(v)) => Ok(v.clone()),
            Some(ParamValue::Int(v)) => Ok(vec![*v]),
            None => Err(format!("{}.{name}: missing", self.module)),
        }
    }

    /// Resolves a content number against this module's parameters.
    pub fn amount(&self, a: &Amount) -> Result<i64, String> {
        match a {
            Amount::Lit(v) => Ok(*v),
            Amount::Param { param } => self.int(param),
        }
    }
}

/// Entry `round - 1` of a per-round schedule; the last entry repeats.
pub fn scheduled(schedule: &[i64], round: u32) -> i64 {
    let i = (round.max(1) as usize - 1).min(schedule.len() - 1);
    schedule[i]
}
