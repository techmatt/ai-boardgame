// The SVG board, drawn from the engine's generic view: every hex of every country piece
// (pointy-top axial, `data-hex="q,r"`), tinted by country and labelled, with each token
// stack drawn as a counter. Knows no rules: what to highlight is handed in.

import type { View } from './engine';
import { hexKey, tokenColour } from './format';

const SQRT3 = Math.sqrt(3);
const NS = 'http://www.w3.org/2000/svg';

export interface Marks {
  /** Hexes the armed choice can target: clicking one takes that action. */
  targets: Set<string>;
  /** Hexes the events of the shown step touched. */
  changed: Set<string>;
}

const centre = (q: number, r: number): [number, number] => [SQRT3 * (q + r / 2), 1.5 * r];

const corners = (x: number, y: number, s = 0.97) =>
  Array.from({ length: 6 }, (_, k) => {
    const a = (Math.PI / 180) * (60 * k - 30);
    return `${(x + s * Math.cos(a)).toFixed(3)},${(y + s * Math.sin(a)).toFixed(3)}`;
  }).join(' ');

function el<K extends keyof SVGElementTagNameMap>(tag: K, attrs: Record<string, string | number>, text?: string) {
  const e = document.createElementNS(NS, tag);
  for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, String(v));
  if (text !== undefined) e.textContent = text;
  return e;
}

export function renderBoard(svg: SVGSVGElement, view: View, marks: Marks) {
  svg.replaceChildren();
  if (!view.hexes.length) return;
  const pts = view.hexes.map((h) => centre(h.hex[0], h.hex[1]));
  const xs = pts.map((p) => p[0]);
  const ys = pts.map((p) => p[1]);
  const pad = 1.3;
  const [x0, y0] = [Math.min(...xs) - pad, Math.min(...ys) - pad];
  svg.setAttribute('viewBox', `${x0} ${y0} ${Math.max(...xs) + pad - x0} ${Math.max(...ys) + pad - y0}`);
  svg.setAttribute('preserveAspectRatio', 'xMidYMid meet');

  const hexLayer = el('g', { class: 'hexes' });
  const labelLayer = el('g', { class: 'labels' });
  const tokenLayer = el('g', { class: 'tokens' });

  view.hexes.forEach((h, i) => {
    const [x, y] = pts[i];
    const key = hexKey(h.hex);
    const country = view.countries[h.country];
    const cls = ['hex'];
    if (marks.targets.has(key)) cls.push('target');
    if (marks.changed.has(key)) cls.push('changed');
    const g = el('g', { class: cls.join(' '), 'data-hex': key, 'data-index': h.index, 'data-country': h.country });
    g.append(el('polygon', { points: corners(x, y), fill: country?.colour || '#ddd' }));
    g.append(el('text', { x, y: y - 0.55, class: 'coord' }, key));
    hexLayer.append(g);

    // Token stacks: counters in a row across the hex, each its own hover target.
    const n = h.tokens.length;
    const r = n > 2 ? 0.26 : 0.34;
    h.tokens.forEach((t, j) => {
      const tx = x + (j - (n - 1) / 2) * (r * 2.15);
      const ty = y + 0.12;
      const tg = el('g', { class: 'token', 'data-hex': key, 'data-token': t.kind });
      tg.append(el('circle', { cx: tx, cy: ty, r, fill: tokenColour(t.kind) }));
      tg.append(el('text', { x: tx, y: ty, class: 'count', 'font-size': r * 1.15 }, String(t.count)));
      tokenLayer.append(tg);
    });
  });

  // Country labels at each piece's centre, under the tokens and not catching the mouse.
  for (const c of view.countries) {
    const cp = c.hexes.map((i) => pts[i]);
    const cx = cp.reduce((s, p) => s + p[0], 0) / cp.length;
    const cy = cp.reduce((s, p) => s + p[1], 0) / cp.length;
    const g = el('g', { class: 'country-label', 'data-country': c.index });
    g.append(el('text', { x: cx, y: cy }, `Country ${c.index}`));
    labelLayer.append(g);
  }

  svg.append(hexLayer, labelLayer, tokenLayer);
}
