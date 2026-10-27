// Typed wrapper over aic-wasm (crates/aic-wasm/src/lib.rs). No rules live here.
//
// States go back to the engine as the JSON string it produced; the parsed copy is for
// display. Seeds and RNG positions are decimal strings in that JSON, so nothing is lost.
// Shapes follow crates/aic-core/src/{state,action,event,view,record,map}.rs.

import init, * as wasm from './wasm/aic_wasm.js';

export type Hex = [number, number];

/**
 * An engine action. The client never builds one: it only sends back actions that
 * `legalActions` returned. Known fields are typed for labelling; any other field a
 * future action carries is kept and shown as-is.
 */
export interface Action {
  type: string;
  seat: number;
  card?: string;
  target?: Hex;
  [field: string]: unknown;
}

/** An engine event: `type` plus whatever fields it carries. */
export interface GameEvent {
  type: string;
  [field: string]: unknown;
}

export interface Outcome {
  won: boolean;
  round: number;
  reason: string;
  detail: string;
}

export interface GameState {
  seed: string;
  config: Config;
  round: number;
  phase: number;
  outcome: Outcome | null;
  [part: string]: unknown;
}

export interface Config {
  name?: string;
  countries: number;
  modules: { module: string; variant: string; params: Record<string, number | number[]>; cards?: string[] }[];
  round: string[];
}

// --- the generic view (crates/aic-core/src/view.rs) ---

export interface Value {
  key: string;
  label: string;
  value: number;
  max?: number;
  source: string;
  doc?: string;
}
export interface CardView { id: string; name: string; text?: string; status: string; cost?: number }
export interface CardGroup { key: string; label: string; source: string; cards: CardView[] }
export interface PileView { key: string; label: string; source: string; owner: number | null; count: number; visible: CardView[] }
export interface Token { kind: string; count: number }
export interface HexView { index: number; hex: Hex; country: number; piece_pos: [number, number]; neighbours: number[]; tokens: Token[] }
export interface CountryView { index: number; seat: number; colour: string; hexes: number[]; slots: (CardView | null)[]; values: Value[] }
export interface SeatView { index: number; country: number; passed: boolean; values: Value[]; cards: CardGroup[] }
export interface PhaseInfo { name: string; kind: 'auto' | 'decision' }
export interface PhaseView { index: number; name: string; kind: 'auto' | 'decision'; waiting: number[] }
export interface View {
  round: number;
  phase: PhaseView | null;
  round_order: PhaseInfo[];
  outcome: Outcome | null;
  hexes: HexView[];
  countries: CountryView[];
  seats: SeatView[];
  globals: Value[];
  piles: PileView[];
}

// --- records (crates/aic-core/src/record.rs) ---

export interface GameRecord {
  format: string;
  note?: string;
  config: Config;
  seed: string;
  actions: Action[];
  outcome?: Outcome | null;
  rounds?: number;
  final_fingerprint?: string;
  config_hash?: string;
  content_hash?: string;
  engine_version?: string;
  rules_revision?: number;
  /** Set by `parseRecord` when this engine rejects the record's config. */
  config_error?: string;
}

export interface CardDef { id: string; name: string; text: string }
export interface Content { cards: CardDef[] }

/** A state as the engine produced it, plus a parsed copy for display. */
export interface Snapshot { json: string; state: GameState }
export interface Step { snap: Snapshot; events: GameEvent[] }

export let content: Content;
export let version = '?';

export async function load(): Promise<void> {
  await init();
  content = JSON.parse(wasm.content());
  version = wasm.version();
}

const snapshot = (json: string): Snapshot => ({ json, state: JSON.parse(json) });

function step(o: wasm.Outcome): Step {
  try {
    return { snap: snapshot(o.state), events: JSON.parse(o.events) };
  } finally {
    o.free();
  }
}

/** `{config, hash}`; throws the engine's reason for an invalid config. */
export function resolveConfig(configJson: string): { config: Config; hash: string } {
  return JSON.parse(wasm.resolve_config(configJson));
}

/** A new game with its setup events. `seed` is a decimal u64 string. */
export function start(configJson: string, seed: string): Step {
  return step(wasm.start(configJson, seed));
}

export function legalActions(snap: Snapshot): Action[] {
  return JSON.parse(wasm.legal_actions(snap.json));
}

/** Throws the engine's error string on an illegal action. */
export function apply(snap: Snapshot, action: Action): Step {
  return step(wasm.apply(snap.json, JSON.stringify(action)));
}

export function view(snap: Snapshot): View {
  return JSON.parse(wasm.view(snap.json));
}

/** The record text (engine-formatted) for the game `actions` took to `snap`. */
export function recordText(snap: Snapshot, actions: Action[], note: string): string {
  return wasm.record_text(snap.json, JSON.stringify(actions), note);
}

/** Throws on bad JSON or a different record format. */
export function parseRecord(text: string): GameRecord {
  return JSON.parse(wasm.parse_record(text));
}

/** Replays a record in the engine and compares it with what was saved. */
export function checkRecord(text: string): { ok: boolean; warnings?: string[]; error?: string } {
  return JSON.parse(wasm.check_record(text));
}

export const cardName = (id: string) => content.cards.find((c) => c.id === id)?.name ?? id;
