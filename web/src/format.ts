// Labels and colours for engine data. Generic: any action, event or token kind gets a
// readable label without code here; known card ids get their content names.

import { cardName, type Action, type GameEvent, type Hex } from './engine';

export const hexKey = (h: Hex) => `${h[0]},${h[1]}`;

export const isHex = (v: unknown): v is Hex =>
  Array.isArray(v) && v.length === 2 && v.every((x) => Number.isInteger(x));

const words = (s: string) => s.replace(/_/g, ' ');

function field(k: string, v: unknown): string {
  if (isHex(v)) return `@${hexKey(v)}`;
  if (k === 'card' && typeof v === 'string') return cardName(v);
  if (typeof v === 'number' && k === 'delta') return v > 0 ? `+${v}` : `${v}`;
  return typeof v === 'string' ? v : JSON.stringify(v);
}

/**
 * "seat 1: play Scrub @-3,4". The seat is left out with `withSeat` false, and the
 * target with `withTarget` false (a choice's label, before its hex is picked).
 */
export function actionLabel(a: Action, withSeat = true, withTarget = true): string {
  const rest = Object.entries(a)
    .filter(([k]) => k !== 'type' && k !== 'seat' && (withTarget || k !== 'target'))
    .map(([k, v]) => (k === 'card' || k === 'target' ? field(k, v) : `${k}=${field(k, v)}`));
  const body = [words(a.type), ...rest].join(' ');
  return withSeat ? `seat ${a.seat}: ${body}` : body;
}

/** A one-line event: its type, then its fields. */
export function eventText(e: GameEvent): string {
  const rest = Object.entries(e)
    .filter(([k]) => k !== 'type')
    .map(([k, v]) => `${k} ${field(k, v)}`);
  return [words(e.type), ...rest].join(' · ');
}

/** Hexes an action or event mentions (any `[q, r]` field). */
export function hexesIn(x: object): Hex[] {
  return Object.values(x).filter(isHex);
}

/** The key that groups actions into one choice: everything but the seat and target. */
export function choiceKey(a: Action): string {
  const { seat: _s, target: _t, ...rest } = a;
  return JSON.stringify(rest);
}

// Token colours: a few kinds with chosen colours, every other kind hashed to a hue.
const KNOWN: Record<string, string> = { blight: '#5b2a86' };

export function tokenColour(kind: string): string {
  if (KNOWN[kind]) return KNOWN[kind];
  let h = 0;
  for (const c of kind) h = (h * 31 + c.charCodeAt(0)) >>> 0;
  return `hsl(${h % 360} 65% 38%)`;
}

export function escapeHtml(s: string): string {
  return s.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}
