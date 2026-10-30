// The one card renderer: card content in, HTML out, no I/O. Sizes are mm, so the same
// markup serves the web client (scale it with CSS) and print (a PDF at true size).
// Needs card.css. Geometry comes from layout.json; icons are placeholders until the
// rendered icon set, drawn by the locked rules (see icons.js).
import { icon, tagMedallion } from './icons.js';

const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);

const place = ([x0, y0, x1, y1]) => `left:${x0}mm;top:${y0}mm;width:${(x1 - x0).toFixed(2)}mm;height:${(y1 - y0).toFixed(2)}mm`;

/** Tag medallions, right-aligned on the frame's circles. */
function tags(card, circles, safe) {
  const shown = card.tags.slice(0, 3);
  return shown.map((t, i) => `<div class="tag" data-safe style="${place(inSafe(circles[circles.length - shown.length + i], safe))}">${tagMedallion(t)}</div>`).join('');
}

/** The largest square in the box, inside the safe zone: a painted socket may overhang it. */
function inSafe([x0, y0, x1, y1], s) {
  const [a, b, c, d] = [Math.max(x0, s.x0), Math.max(y0, s.y0), Math.min(x1, s.x1), Math.min(y1, s.y1)];
  const side = Math.min(c - a, d - b);
  const [cx, cy] = [(a + c) / 2, (b + d) / 2];
  return [cx - side / 2, cy - side / 2, cx + side / 2, cy + side / 2];
}

/** The guide's three tag circles, left to right. */
function guideCircles([, y0, x1, y1]) {
  const d = y1 - y0;
  return [2, 1, 0].map((i) => [x1 - d - i * (d + 0.5), y0, x1 - i * (d + 0.5), y1]);
}

const effectLine = (parts) =>
  `<div class="fx-line" data-safe>${parts.map((p) => (p.text !== undefined ? `<span class="fx-num">${esc(p.text)}</span>` : icon(p.icon, p))).join('')}</div>`;

/**
 * @param card     content: name, kind, cost, insight, tags, effect (icon lines), text, number
 * @param layout   web/card/layout.json
 * @param frameUrl the painted frame for this kind and tier
 * @param fit      optional measured plates of this frame (art/frames/r1/fit.json), in mm:
 *                 cost, insight, banner, number, tags; the guide's where absent
 */
export function renderCard(card, layout, frameUrl, fit = {}) {
  const c = { ...layout.common, ...fit };
  const k = layout.kinds[card.kind];
  // The tab crosses the safe zone: centre the figure in it, but no lower than the zone allows.
  const NUMBER_H = 1.5;
  const numberY = Math.min((c.number[1] + c.number[3]) / 2, layout.safe.y1 - NUMBER_H / 2);
  const number = [c.number[0], numberY - NUMBER_H / 2, c.number[2], numberY + NUMBER_H / 2];
  return `<article class="card kind-${esc(card.kind)}">
  <img class="frame" src="${esc(frameUrl)}" alt="">
  <div class="content">
    <div class="cost" style="${place(c.cost)}"><span data-safe>${esc(card.cost)}</span></div>
    ${card.insight ? `<div class="insight" style="${place(c.insight)}"><span data-safe>${esc(card.insight)}</span>${icon('insight', { small: true })}</div>` : ''}
    ${tags(card, Array.isArray(c.tags[0]) ? c.tags : guideCircles(c.tags), layout.safe)}
    <div class="name" style="${place(c.banner)}"><span data-safe>${esc(card.name)}</span></div>
    <div class="effect" style="${place(k.effect)}">
      <div class="fx-icons">${card.effect.map(effectLine).join('')}</div>
      <p class="rules" data-safe>${esc(card.text)}</p>
    </div>
    <div class="number" style="${place(number)}"><span data-safe>${esc(card.number)}</span></div>
  </div>
</article>`;
}
