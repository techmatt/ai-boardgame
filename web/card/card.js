// The one card renderer: card content in, HTML out, no I/O. Sizes are mm, so the same
// markup serves the web client (scale it with CSS) and print (a PDF at true size).
// Needs card.css. Geometry comes from layout.json; icons are placeholders until the
// rendered icon set, drawn by the locked rules (see icons.js). The painted frame holds
// the fixed structure only: what changes card by card is drawn here.
import { icon, tagMedallion } from './icons.js';

const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);

const place = ([x0, y0, x1, y1]) => `left:${x0}mm;top:${y0}mm;width:${(x1 - x0).toFixed(2)}mm;height:${(y1 - y0).toFixed(2)}mm`;

/** Tag medallions, right-aligned on the strip: only the card's own, so no socket shows empty. */
function tags(card, circles, safe) {
  const shown = card.tags.slice(0, 3);
  return shown.map((t, i) => `<div class="tag" data-safe style="${place(inSafe(circles[circles.length - shown.length + i], safe))}">${tagMedallion(t)}</div>`).join('');
}

/** The largest square in the box, inside the safe zone. */
function inSafe([x0, y0, x1, y1], s) {
  const [a, b, c, d] = [Math.max(x0, s.x0), Math.max(y0, s.y0), Math.min(x1, s.x1), Math.min(y1, s.y1)];
  const side = Math.min(c - a, d - b);
  const [cx, cy] = [(a + c) / 2, (b + d) / 2];
  return [cx - side / 2, cy - side / 2, cx + side / 2, cy + side / 2];
}

/** The three tag circles, left to right. */
function tagCircles([, y0, x1, y1]) {
  const d = y1 - y0;
  return [2, 1, 0].map((i) => [x1 - d - i * (d + 1), y0, x1 - i * (d + 1), y1]);
}

/** The octagon with its painted rim, as a clip-path: the frame again, over the plate that tucks under it. */
function octagonClip([x0, y0, x1, y1], rim = 0.8) {
  const c = (x1 - x0) * 0.29;
  const [a, b, d, e] = [x0 - rim, y0 - rim, x1 + rim, y1 + rim];
  const pts = [[a + c, b], [d - c, b], [d, b + c], [d, e - c], [d - c, e], [a + c, e], [a, e - c], [a, b + c]];
  return `polygon(${pts.map(([x, y]) => `${x.toFixed(2)}mm ${y.toFixed(2)}mm`).join(',')})`;
}

const effectPart = (p) => {
  if (p.income) return `<span class="income">${p.income.map(effectPart).join('')}</span>`;
  return p.text !== undefined ? `<span class="fx-num">${esc(p.text)}</span>` : icon(p.icon, p);
};
const effectLine = (parts) => `<div class="fx-line" data-safe>${parts.map(effectPart).join('')}</div>`;

/** Icon lines over rules text: the effect panel, and a building's top box. */
const effectBox = (box, lines = [], text = '') =>
  `<div class="effect" style="${place(box)}">
      ${lines.length ? `<div class="fx-icons">${lines.map(effectLine).join('')}</div>` : ''}
      ${text ? `<p class="rules" data-safe>${esc(text)}</p>` : ''}
    </div>`;

/**
 * @param card     content: name, kind, cost, insight, needs, tags, effect (icon lines), text;
 *                 a building's lasting part in top (icon lines) and top_text
 * @param layout   web/card/layout.json
 * @param frameUrl the painted frame for this kind and tier
 * @param fit      optional measured zones of this frame (art/frames/r2/fit.json), in mm:
 *                 cost, banner, top, effect, the guide's where absent; plate, a colour
 */
export function renderCard(card, layout, frameUrl, fit = {}) {
  const z = { ...layout.common, ...layout.kinds[card.kind], ...fit };
  // The plates hang off the octagon, so they follow wherever the frame really put it.
  const dx = z.cost[2] - layout.common.cost[2];
  const dy = (z.cost[1] + z.cost[3] - layout.common.cost[1] - layout.common.cost[3]) / 2;
  const insight = [z.insight[0] + dx, z.insight[1] + dy, z.insight[2] + dx, z.insight[3] + dy];
  const reqX = card.insight ? insight[2] + 0.5 : z.cost[2] + 1.4;
  const needs = (card.needs ?? []).flatMap(([tag, n]) => Array(n).fill(tag));
  return `<article class="card kind-${esc(card.kind)}"${fit.plate ? ` style="--plate:${esc(fit.plate)}"` : ''}>
  <img class="frame" src="${esc(frameUrl)}" alt="">
  <div class="content">
    ${card.insight ? `<div class="insight plate" style="${place(insight)}"><span data-safe>${esc(card.insight)}</span>${icon('insight', { small: true })}</div>` : ''}
    <img class="frame" src="${esc(frameUrl)}" alt="" style="clip-path:${octagonClip(z.cost)}">
    <div class="cost" style="${place(z.cost)}"><span data-safe>${esc(card.cost)}</span></div>
    ${needs.length ? `<div class="requirement plate" style="left:${reqX.toFixed(2)}mm;top:${(z.requirement[1] + dy).toFixed(2)}mm;height:${(z.requirement[3] - z.requirement[1]).toFixed(2)}mm">${needs.map((t) => tagMedallion(t)).join('')}</div>` : ''}
    ${tags(card, tagCircles([z.tags[0], z.tags[1] + dy, z.tags[2], z.tags[3] + dy]), layout.safe)}
    <div class="name" style="${place(z.banner)}"><span data-safe>${esc(card.name)}</span></div>
    ${z.top ? effectBox(z.top, card.top, card.top_text) : ''}
    ${effectBox(z.effect, card.effect, card.text)}
  </div>
</article>`;
}
