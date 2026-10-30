// `npm run guides`: draw a guide per variant from web/card/layout.json.
// art/guides/<kind>-t<tier>.png is the labelled one, for people. The unlabelled
// twin (<kind>-t<tier>-blocks.png) is what the image tool sees: labels invite text.
import { mkdir } from 'node:fs/promises';
import path from 'node:path';

import { ART, layout, openPage } from './lib.mjs';

const ZONE_COLOUR = {
  strip: '#8a93a0',
  cost: '#e2574c',
  insight: '#b06fd8',
  requirement: '#e6c84f',
  tags: '#4fb6d8',
  banner: '#f0a0c0',
  art: '#ffffff',
  effect: '#e8e4d8',
  top: '#d6e8c8',
  number: '#50d0a0',
};

function svg(l, kind, tier, labelled) {
  const [w, h] = l.bleed_mm;
  const zones = { strip: l.common.strip, banner: l.common.banner, ...l.kinds[kind], cost: l.common.cost, insight: l.common.insight, requirement: l.common.requirement, tags: l.common.tags, number: l.common.number };
  const box = ([x0, y0, x1, y1], fill) => `<rect x="${x0}" y="${y0}" width="${x1 - x0}" height="${y1 - y0}" fill="${fill}"/>`;
  const parts = [`<rect width="${w}" height="${h}" fill="#3a3f46"/>`, box(l.common.frame, l.kind_colour[kind])];
  if (tier === 2) {
    const [x0, y0, x1, y1] = l.common.frame;
    parts.push(`<rect x="${x0 + 0.25}" y="${y0 + 0.25}" width="${x1 - x0 - 0.5}" height="${y1 - y0 - 0.5}" fill="none" stroke="#e0b83a" stroke-width="0.5"/>`);
  }
  for (const [name, r] of Object.entries(zones)) {
    if (name === 'tags') {
      const d = r[3] - r[1];
      for (let i = 0; i < 3; i++) parts.push(`<circle cx="${r[2] - d / 2 - i * (d + 0.5)}" cy="${(r[1] + r[3]) / 2}" r="${d / 2}" fill="${ZONE_COLOUR.tags}"/>`);
    } else if (name === 'cost') {
      const [x0, y0, x1, y1] = r;
      const c = (x1 - x0) * 0.29;
      parts.push(`<polygon points="${x0 + c},${y0} ${x1 - c},${y0} ${x1},${y0 + c} ${x1},${y1 - c} ${x1 - c},${y1} ${x0 + c},${y1} ${x0},${y1 - c} ${x0},${y0 + c}" fill="${ZONE_COLOUR.cost}"/>`);
    } else parts.push(box(r, ZONE_COLOUR[name]));
    if (labelled) parts.push(`<text x="${(r[0] + r[2]) / 2}" y="${(r[1] + r[3]) / 2 + 0.7}" font-size="2" font-family="sans-serif" text-anchor="middle" fill="#000">${name}</text>`);
  }
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${w} ${h}" width="100%" height="100%" style="display:block">${parts.join('')}</svg>`;
}

const l = await layout();
const [W, H] = l.print_px;
await mkdir(path.join(ART, 'guides'), { recursive: true });
const { page, close } = await openPage(W, H);
for (const kind of Object.keys(l.kinds))
  for (const tier of [1, 2])
    for (const labelled of [true, false]) {
      await page.setContent(`<!doctype html><body style="margin:0;width:${W}px;height:${H}px">${svg(l, kind, tier, labelled)}</body>`);
      await page.screenshot({ path: path.join(ART, 'guides', `${kind}-t${tier}${labelled ? '' : '-blocks'}.png`) });
    }
await close();
console.log('guides: 6 labelled, 6 unlabelled in art/guides/');
