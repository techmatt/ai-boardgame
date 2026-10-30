// `npm run render -- [ids…]`: compose the sample card over each frame in art/frames/r1/
// (all of them, or the named ids) into out/cards/<id>.png at 300 dpi. Composed cards are
// build outputs, never committed. Fails if a font fell back or anything printed leaves
// the safe zone. `--pdf` also writes out/cards/<id>.pdf at true size.
import { mkdir, readdir, readFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { ART, CARD, OUT, layout, openPage, readJson } from './lib.mjs';
import { renderCard } from '../../web/card/card.js';

const argv = process.argv.slice(2);
const pdf = argv.includes('--pdf');
const named = argv.filter((a) => !a.startsWith('--'));
const framesDir = path.join(ART, 'frames', 'r1');
const ids = named.length ? named : (await readdir(framesDir)).filter((f) => f.endsWith('.jpg')).map((f) => f.slice(0, -4));
const l = await layout();
const card = await readJson(path.join(CARD, 'sample', 'sentiment_mining.json'));
const fitFile = path.join(framesDir, 'fit.json');
const fits = existsSync(fitFile) ? await readJson(fitFile) : {};
await mkdir(OUT, { recursive: true });

const PX_PER_MM_CSS = 96 / 25.4;
const scale = l.print_px[0] / (l.bleed_mm[0] * PX_PER_MM_CSS);
const { page, close } = await openPage(Math.ceil(l.bleed_mm[0] * PX_PER_MM_CSS), Math.ceil(l.bleed_mm[1] * PX_PER_MM_CSS));
const browser = page.context().browser();
const ctx = await browser.newContext({ viewport: { width: 261, height: 356 }, deviceScaleFactor: scale });
const p = await ctx.newPage();

// A file: page so the relative font urls in card.css resolve.
const stub = path.join(CARD, 'card.css');
await p.goto(pathToFileURL(stub).href.replace('card.css', 'layout.json'));
let failed = 0;
for (const id of ids) {
  const frame = pathToFileURL(path.join(framesDir, `${id}.jpg`)).href;
  const css = await readFile(stub, 'utf8');
  const base = pathToFileURL(CARD + path.sep).href;
  await p.setContent(`<!doctype html><html><head><base href="${base}"><style>body{margin:0}${css}</style></head><body>${renderCard(card, l, frame, fits[id])}</body></html>`, { waitUntil: 'load' });
  await p.evaluate(() => document.fonts.ready);
  const check = await p.evaluate(({ safe, mm }) => {
    const bad = [];
    for (const f of ['600 10pt "Saira Condensed"', '700 10pt "Saira Condensed"', '400 10pt "Source Serif 4"'])
      if (!document.fonts.check(f)) bad.push(`font ${f}`);
    for (const el of document.querySelectorAll('[data-safe]')) {
      const r = document.createRange();
      r.selectNodeContents(el);
      // Text width from its glyph run, height from its line box: a range's height is the font's whole content area.
      const t = el.textContent.trim() ? r.getBoundingClientRect() : el.getBoundingClientRect();
      const e = el.getBoundingClientRect();
      const [x0, y0, x1, y1] = [t.left / mm, e.top / mm, t.right / mm, e.bottom / mm];
      if (x0 < safe.x0 - 0.01 || y0 < safe.y0 - 0.01 || x1 > safe.x1 + 0.01 || y1 > safe.y1 + 0.01) bad.push(`${el.className || el.textContent.slice(0, 20)} at ${[x0, y0, x1, y1].map((v) => v.toFixed(1))}`);
    }
    return bad;
  }, { safe: l.safe, mm: PX_PER_MM_CSS });
  if (check.length) {
    failed++;
    console.error(`render: ${id}: ${check.join('; ')}`);
  }
  await p.screenshot({ path: path.join(OUT, `${id}.png`), clip: { x: 0, y: 0, width: l.bleed_mm[0] * PX_PER_MM_CSS, height: l.bleed_mm[1] * PX_PER_MM_CSS } });
  if (pdf) await p.pdf({ path: path.join(OUT, `${id}.pdf`), width: `${l.bleed_mm[0]}mm`, height: `${l.bleed_mm[1]}mm`, printBackground: true });
}
await close();
console.log(`render: ${ids.length} cards in out/cards/${failed ? `, ${failed} with problems` : ''}`);
process.exit(failed ? 1 : 0);
