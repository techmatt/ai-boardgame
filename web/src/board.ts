// The SVG board, drawn from the engine's generic view: every hex of every country piece
// (pointy-top axial, `data-hex="q,r"`), filled by terrain, with a band in the country's
// colour along each country border, the printed hex number, an antenna on uplinks, and
// each token stack drawn as a small glyph with its count. Knows no rules: what to
// highlight is handed in. Token kinds it has no glyph for are drawn as plain counters.

import type { HexView, Token, View } from './engine';
import { hexKey, strong, terrainFill, tokenColour } from './format';

const SQRT3 = Math.sqrt(3);
const NS = 'http://www.w3.org/2000/svg';

export interface Marks {
  /** Hexes the armed choice can target: clicking one takes that action. */
  targets: Set<string>;
  /** Hexes the events of the shown step touched. */
  changed: Set<string>;
}

const centre = (q: number, r: number): [number, number] => [SQRT3 * (q + r / 2), 1.5 * r];

const corner = (x: number, y: number, k: number, s: number): [number, number] => {
  const a = (Math.PI / 180) * (60 * k - 30);
  return [x + s * Math.cos(a), y + s * Math.sin(a)];
};

const corners = (x: number, y: number, s = 0.97) =>
  Array.from({ length: 6 }, (_, k) => corner(x, y, k, s).map((v) => v.toFixed(3)).join(',')).join(' ');

/** Axial neighbour across the edge from corner k to corner k + 1. */
const EDGE: [number, number][] = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];

function el<K extends keyof SVGElementTagNameMap>(tag: K, attrs: Record<string, string | number>, text?: string) {
  const e = document.createElementNS(NS, tag);
  for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, String(v));
  if (text !== undefined) e.textContent = text;
  return e;
}

const f = (n: number) => n.toFixed(3);

/** A glyph for one token kind, centred on 0,0 with half-size `s`. */
function glyph(kind: string, s: number, seatColour: (seat: number) => string): SVGElement {
  const g = el('g', {});
  const [base, arg] = kind.split(':');
  const c = tokenColour(kind);
  const p = (d: string, fill: string, cls = 'glyph') => el('path', { d, fill, class: cls });
  switch (base) {
    case 'population':
      g.append(
        el('circle', { cx: 0, cy: f(-0.5 * s), r: f(0.38 * s), fill: c, class: 'glyph' }),
        p(`M${f(-0.75 * s)},${f(0.95 * s)} Q${f(-0.75 * s)},${f(-0.05 * s)} 0,${f(-0.05 * s)} Q${f(0.75 * s)},${f(-0.05 * s)} ${f(0.75 * s)},${f(0.95 * s)} Z`, c),
      );
      break;
    case 'factory':
      g.append(p(`M${f(-s)},${f(0.85 * s)} V${f(-0.15 * s)} L${f(-0.45 * s)},${f(-0.6 * s)} V${f(-0.15 * s)} L${f(0.1 * s)},${f(-0.6 * s)} V${f(-0.15 * s)} L${f(0.45 * s)},${f(-0.5 * s)} V${f(-s)} H${f(s)} V${f(0.85 * s)} Z`, c));
      break;
    case 'data_centre':
      g.append(el('rect', { x: f(-0.7 * s), y: f(-0.95 * s), width: f(1.4 * s), height: f(1.9 * s), rx: f(0.15 * s), fill: c, class: 'glyph' }));
      for (const yy of [-0.5, 0, 0.5]) {
        g.append(el('line', { x1: f(-0.45 * s), x2: f(0.2 * s), y1: f(yy * s), y2: f(yy * s), class: 'detail' }));
        g.append(el('circle', { cx: f(0.42 * s), cy: f(yy * s), r: f(0.09 * s), fill: '#7cfc8a' }));
      }
      break;
    case 'processor': {
      const col = arg !== undefined ? seatColour(Number(arg)) : c;
      for (const t of [-0.4, 0, 0.4]) {
        const o = f(t * s);
        g.append(
          el('line', { x1: f(-s), x2: f(s), y1: o, y2: o, class: 'pin' }),
          el('line', { y1: f(-s), y2: f(s), x1: o, x2: o, class: 'pin' }),
        );
      }
      g.append(el('rect', { x: f(-0.72 * s), y: f(-0.72 * s), width: f(1.44 * s), height: f(1.44 * s), rx: f(0.12 * s), fill: col, class: 'glyph' }));
      if (arg !== undefined) g.append(el('text', { x: 0, y: f(0.05 * s), class: 'in-glyph', 'font-size': f(1.1 * s) }, arg));
      break;
    }
    case 'drone':
      g.append(p(`M0,${f(-0.8 * s)} L${f(0.8 * s)},${f(0.65 * s)} L0,${f(0.3 * s)} L${f(-0.8 * s)},${f(0.65 * s)} Z`, c));
      break;
    case 'outpost':
      g.append(p(`M${f(-0.85 * s)},${f(0.9 * s)} V${f(-0.45 * s)} H${f(-0.55 * s)} V${f(-0.9 * s)} H${f(-0.18 * s)} V${f(-0.45 * s)} H${f(0.18 * s)} V${f(-0.9 * s)} H${f(0.55 * s)} V${f(-0.45 * s)} H${f(0.85 * s)} V${f(0.9 * s)} Z`, c));
      break;
    case 'foundry': {
      const pts = Array.from({ length: 8 }, (_, k) => {
        const a = (Math.PI / 4) * k + Math.PI / 8;
        return `${f(1.12 * s * Math.cos(a))},${f(1.12 * s * Math.sin(a))}`;
      }).join(' ');
      g.append(el('polygon', { points: pts, fill: c, class: 'glyph' }), el('circle', { cx: 0, cy: 0, r: f(0.42 * s), fill: '#ff7043' }));
      break;
    }
    case 'corruption': {
      const lobes = [1, 0.7, 0.95, 0.65, 1.05, 0.75, 0.9, 0.6];
      const pts = lobes.map((r, k) => {
        const a = (Math.PI / 4) * k;
        return [r * s * Math.cos(a), r * s * Math.sin(a)];
      });
      const d = pts.map(([x, y], k) => {
        const [nx, ny] = pts[(k + 1) % pts.length];
        return `${k ? '' : `M${f(x)},${f(y)} `}Q${f((x + nx) * 0.75)},${f((y + ny) * 0.75)} ${f(nx)},${f(ny)}`;
      }).join(' ');
      g.append(p(`${d} Z`, c));
      break;
    }
    case 'defense': {
      const col = arg !== undefined ? seatColour(Number(arg)) : c;
      g.append(p(`M0,${f(-s)} L${f(0.85 * s)},${f(-0.62 * s)} V${f(0.05 * s)} Q${f(0.8 * s)},${f(0.75 * s)} 0,${f(1.02 * s)} Q${f(-0.8 * s)},${f(0.75 * s)} ${f(-0.85 * s)},${f(0.05 * s)} V${f(-0.62 * s)} Z`, col));
      g.append(p(`M0,${f(-0.6 * s)} V${f(0.65 * s)}`, 'none', 'detail'));
      break;
    }
    case 'fortify':
      g.append(el('rect', { x: f(-0.9 * s), y: f(-0.7 * s), width: f(1.8 * s), height: f(1.4 * s), fill: c, class: 'glyph' }));
      g.append(p(`M${f(-0.9 * s)},0 H${f(0.9 * s)} M0,${f(-0.7 * s)} V0 M${f(-0.45 * s)},0 V${f(0.7 * s)} M${f(0.45 * s)},0 V${f(0.7 * s)}`, 'none', 'detail'));
      break;
    default:
      g.append(el('circle', { cx: 0, cy: 0, r: f(0.9 * s), fill: c, class: 'glyph' }));
  }
  return g;
}

/** Glyph scale per kind: adversary pieces grow drone < outpost < foundry. */
const SCALE: Record<string, number> = { drone: 0.85, outpost: 1, foundry: 1.12, corruption: 1.05 };
const ADVERSARY = new Set(['drone', 'outpost', 'foundry', 'corruption']);
const ORDER = ['population', 'factory', 'data_centre', 'processor', 'defense', 'fortify', 'drone', 'outpost', 'foundry', 'corruption'];
const rank = (k: string) => {
  const i = ORDER.indexOf(k.split(':')[0]);
  return i < 0 ? ORDER.length : i;
};

/** Players' tokens on the upper row, the adversary's on the lower one. */
function rows(tokens: Token[]): Token[][] {
  const sorted = [...tokens].sort((a, b) => rank(a.kind) - rank(b.kind) || a.kind.localeCompare(b.kind));
  const top = sorted.filter((t) => !ADVERSARY.has(t.kind));
  const bottom = sorted.filter((t) => ADVERSARY.has(t.kind));
  return [top, bottom].filter((r) => r.length);
}

function drawTokens(layer: SVGGElement, h: HexView, x: number, y: number, seatColour: (seat: number) => string) {
  const key = hexKey(h.hex);
  const rs = rows(h.tokens);
  const ys = rs.length === 1 ? [y + 0.12] : [y - 0.07, y + 0.45];
  rs.forEach((row, ri) => {
    const step = Math.min(0.47, 1.5 / row.length);
    const s = step * 0.44;
    row.forEach((t, j) => {
      const tx = x + (j - (row.length - 1) / 2) * step;
      const ty = ys[ri];
      const k = s * (SCALE[t.kind] ?? 1);
      const tg = el('g', { class: 'token', 'data-hex': key, 'data-token': t.kind });
      const hit = el('circle', { cx: f(tx), cy: f(ty), r: f(step / 2), class: 'hit' });
      const gl = glyph(t.kind, k, seatColour);
      gl.setAttribute('transform', `translate(${f(tx)} ${f(ty)})`);
      tg.append(hit, gl);
      if (t.count !== 1) {
        tg.append(el('text', { x: f(tx + k * 0.95), y: f(ty + k * 0.85), class: 'count', 'font-size': f(Math.max(0.15, k * 1.25)) }, String(t.count)));
      }
      layer.append(tg);
    });
  });
}

function uplink(x: number, y: number): SVGElement {
  const g = el('g', { class: 'uplink', transform: `translate(${f(x)} ${f(y)}) scale(1.35)` });
  g.append(
    el('circle', { cx: 0, cy: 0, r: 0.15, class: 'badge' }),
    el('path', { d: 'M0,0.1 V-0.04 M-0.06,0.1 H0.06', class: 'mast' }),
    el('circle', { cx: 0, cy: -0.05, r: 0.025, class: 'tip' }),
    el('path', { d: 'M-0.06,-0.11 A0.08,0.08 0 0 1 0.06,-0.11 M-0.09,-0.02 A0.11,0.11 0 0 1 -0.09,-0.1 M0.09,-0.02 A0.11,0.11 0 0 0 0.09,-0.1', class: 'waves' }),
  );
  return g;
}

export function renderBoard(svg: SVGSVGElement, view: View, marks: Marks) {
  svg.replaceChildren();
  if (!view.hexes.length) return;
  const pts = view.hexes.map((h) => centre(h.hex[0], h.hex[1]));
  const xs = pts.map((p) => p[0]);
  const ys = pts.map((p) => p[1]);
  const pad = 1.15;
  const [x0, y0] = [Math.min(...xs) - pad, Math.min(...ys) - pad];
  svg.setAttribute('viewBox', `${f(x0)} ${f(y0)} ${f(Math.max(...xs) + pad - x0)} ${f(Math.max(...ys) + pad - y0)}`);
  svg.setAttribute('preserveAspectRatio', 'xMidYMid meet');

  const at = new Map(view.hexes.map((h) => [hexKey(h.hex), h]));
  const seatColour = (seat: number) => strong(view.countries[view.seats[seat]?.country ?? seat]?.colour ?? '#888888');

  const hexLayer = el('g', { class: 'hexes' });
  const borderLayer = el('g', { class: 'borders' });
  const labelLayer = el('g', { class: 'labels' });
  const tokenLayer = el('g', { class: 'tokens' });
  const markLayer = el('g', { class: 'marks' });

  view.hexes.forEach((h, i) => {
    const [x, y] = pts[i];
    const key = hexKey(h.hex);
    const country = view.countries[h.country];
    const cls = ['hex', `t-${h.terrain}`];
    if (marks.targets.has(key)) cls.push('target');
    if (marks.changed.has(key)) cls.push('changed');
    const g = el('g', { class: cls.join(' '), 'data-hex': key, 'data-index': h.index, 'data-country': h.country });
    g.append(el('polygon', { points: corners(x, y), fill: terrainFill(h.terrain), class: 'cell' }));
    g.append(el('text', { x: f(x), y: f(y - 0.66), class: 'num' }, String(h.number)));
    if (h.uplink) g.append(uplink(x + 0.43, y - 0.47));
    hexLayer.append(g);

    // A band in the country's colour along every edge that leaves the country.
    const band = strong(country?.colour || '#888888');
    EDGE.forEach(([dq, dr], k) => {
      const n = at.get(`${h.hex[0] + dq},${h.hex[1] + dr}`);
      if (n && n.country === h.country) return;
      const [ax, ay] = corner(x, y, k, 0.9);
      const [bx, by] = corner(x, y, (k + 1) % 6, 0.9);
      borderLayer.append(el('line', { x1: f(ax), y1: f(ay), x2: f(bx), y2: f(by), stroke: band, class: 'border' }));
    });

    // Highlights sit above the tokens so a target stays visible under a crowd.
    if (marks.targets.has(key) || marks.changed.has(key)) {
      markLayer.append(el('polygon', { points: corners(x, y, 0.86), class: marks.targets.has(key) ? 'mark target' : 'mark changed' }));
    }

    drawTokens(tokenLayer, h, x, y, seatColour);
  });

  // Country labels at each piece's centre, under the tokens and not catching the mouse.
  for (const c of view.countries) {
    const cp = c.hexes.map((i) => pts[i]);
    const cx = cp.reduce((s, p) => s + p[0], 0) / cp.length;
    const cy = cp.reduce((s, p) => s + p[1], 0) / cp.length;
    const g = el('g', { class: 'country-label', 'data-country': c.index });
    g.append(el('text', { x: f(cx), y: f(cy), fill: strong(c.colour) }, `C${c.index}`));
    labelLayer.append(g);
  }

  svg.append(hexLayer, labelLayer, borderLayer, tokenLayer, markLayer);
}
