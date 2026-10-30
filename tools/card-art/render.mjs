// `npm run render -- [variants…] [--pdf]`: compose each frame variant in art/frames/r3/
// with its real card (samples.json: card from content/cards.json, face from the generated
// web/card/faces.json) into out/cards/<variant>.png at 300 dpi. With no variants named, also
// the splay, the icon sheet and a check of every card in content. Composed cards are build
// outputs, never committed. Fails if a font fell back, anything printed leaves the safe
// zone, a panel overflows or a requirement plate reaches the tags. `--pdf` also writes
// out/cards/<variant>.pdf at true size and, with no variants named, all-cards.pdf (every
// real card at true size, 9 to an A4 page) and splay.pdf.
import { mkdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { ART, CARD, OUT, REPO, TOOL, layout, openPage, readJson } from './lib.mjs';
import { inset, renderCard } from '../../web/card/card.js';
import { icon, stacked, tagMedallion } from '../../web/card/icons.js';

const argv = process.argv.slice(2);
const pdf = argv.includes('--pdf');
const named = argv.filter((a) => !a.startsWith('--'));
const framesDir = path.join(ART, 'frames', 'r3');
const samples = await readJson(path.join(TOOL, 'samples.json'));
const ids = named.length ? named : Object.keys(samples.variants);
const l = inset(await layout());
const fits = await readJson(path.join(framesDir, 'fit.json'));
const cards = Object.fromEntries((await readJson(path.join(REPO, 'content', 'cards.json'))).map((c) => [c.id, c]));
const faces = await readJson(path.join(CARD, 'faces.json'));
const faced = (id) => ({ ...cards[id], ...faces[id] });
const frameOf = (card) => `${card.kind}-t${card.tier >= 2 ? 2 : 1}`;
const frameUrl = (v) => pathToFileURL(path.join(framesDir, `${v}.jpg`)).href;
await mkdir(OUT, { recursive: true });

const MM = 96 / 25.4;
const scale = l.print_px[0] / (l.bleed_mm[0] * MM);
const { page, close } = await openPage(Math.ceil(l.bleed_mm[0] * MM), Math.ceil(l.bleed_mm[1] * MM));
const browser = page.context().browser();
const ctx = await browser.newContext({ viewport: { width: 700, height: 900 }, deviceScaleFactor: scale });
const p = await ctx.newPage();

// A file: page so the relative font and container urls in card.css resolve.
const stub = path.join(CARD, 'card.css');
await p.goto(pathToFileURL(stub).href.replace('card.css', 'layout.json'));
const css = await readFile(stub, 'utf8');
const base = pathToFileURL(CARD + path.sep).href;
const show = async (body, extra = '') => {
  await p.setContent(`<!doctype html><html><head><base href="${base}"><style>body{margin:0}${css}${extra}</style></head><body>${body}</body></html>`, { waitUntil: 'load' });
  await p.evaluate(() => document.fonts.ready);
};

const inspect = () =>
  p.evaluate(({ safe, mm }) => {
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
    for (const box of document.querySelectorAll('.effect')) {
      const b = box.getBoundingClientRect();
      for (const c of box.children) {
        const r = c.getBoundingClientRect();
        if (r.top < b.top - 0.5 || r.bottom > b.bottom + 0.5) bad.push(`${box.className} overflows`);
      }
    }
    const plate = [...document.querySelectorAll('.requirement')].at(-1)?.getBoundingClientRect();
    const tag = document.querySelector('.tag')?.getBoundingClientRect();
    if (plate && tag && plate.right > tag.left) bad.push(`requirement reaches the tags at ${(plate.right / mm).toFixed(1)}`);
    return bad;
  }, { safe: l.safe, mm: MM });

let failed = 0;
const report = (id, check) => {
  if (!check.length) return;
  failed++;
  console.error(`render: ${id}: ${check.join('; ')}`);
};
const clip = { x: 0, y: 0, width: l.bleed_mm[0] * MM, height: l.bleed_mm[1] * MM };

for (const id of ids) {
  const card = faced(samples.variants[id]);
  if (frameOf(card) !== id) console.error(`render: ${id}: closest fit, ${card.id} is ${frameOf(card)}`);
  await show(renderCard(card, l, frameUrl(id), fits[id]));
  report(id, await inspect());
  await p.screenshot({ path: path.join(OUT, `${id}.png`), clip });
  if (pdf) await p.pdf({ path: path.join(OUT, `${id}.pdf`), width: `${l.bleed_mm[0]}mm`, height: `${l.bleed_mm[1]}mm`, printBackground: true });
}

if (!named.length) {
  // Every real card, whole.
  for (const c of Object.values(cards)) {
    await show(renderCard(faced(c.id), l, frameUrl(frameOf(c)), fits[frameOf(c)]));
    report(c.id, await inspect());
  }

  // The splay: trimmed, each newer card over the older's name, only its top strip showing.
  const stack = samples.splay.map(faced);
  const trim = (l.bleed_mm[0] - l.trim_mm[0]) / 2;
  const [tw, th] = l.trim_mm;
  const showing = fits['building-t1'].banner[1] - trim;
  const slot = (card, i) =>
    `<div class="slot-card" style="top:${i * showing}mm;width:${tw}mm;height:${th}mm"><div style="margin:${-trim}mm">${renderCard(card, l, frameUrl(frameOf(card)), fits[frameOf(card)])}</div></div>`;
  const height = th + (stack.length - 1) * showing;
  await show(`<div class="splay" style="width:${tw}mm;height:${height}mm;margin:2mm">${stack.map(slot).join('')}</div>`, `@page{size:${tw + 4}mm ${height + 4}mm;margin:0}body{background:#3a3f46}`);
  await p.screenshot({ path: path.join(OUT, 'splay.png'), clip: { x: 0, y: 0, width: (tw + 4) * MM, height: (height + 4) * MM } });
  if (pdf) await p.pdf({ path: path.join(OUT, 'splay.pdf'), printBackground: true, preferCSSPageSize: true });

  // Every real card at true size: 9 to an A4 page, bleeds touching, in content order.
  if (pdf) {
    const all = Object.values(cards);
    const pages = [];
    for (let i = 0; i < all.length; i += 9) pages.push(all.slice(i, i + 9));
    const body = pages
      .map((row) => `<section class="sheet-page">${row.map((c) => renderCard(faced(c.id), l, frameUrl(frameOf(c)), fits[frameOf(c)])).join('')}</section>`)
      .join('');
    await show(body, '@page{size:210mm 297mm;margin:0}.sheet-page{width:207mm;height:282mm;padding:7.500mm 1.500mm;display:grid;grid-template-columns:repeat(3,69mm);grid-auto-rows:94mm;break-after:page}');
    await p.pdf({ path: path.join(OUT, 'all-cards.pdf'), printBackground: true, preferCSSPageSize: true });
    console.log(`render: all-cards.pdf, ${all.length} cards on ${pages.length} pages`);
  }

  // Every icon, for review.
  const set = (names) => names.map((n) => `<div class="cell">${n.startsWith('#') ? tagMedallion(n.slice(1)) : icon(n)}<small>${n.replace('#', '')}</small></div>`).join('');
  const sheet = `<div class="card sheet" style="--strip:#333;height:auto;width:120mm;background:#efeadc"><div class="row">${set(['credits', 'insight', 'chips', 'energy', 'antimatter', 'compute', 'cards', 'datasets', 'capital', 'followers', 'agents', 'ambassador', 'presence', 'factory', 'move', 'harden', 'shelter', 'country', 'project', 'hex', 'troop', 'node', 'cube', 'city', 'building', 'slot', 'sway', 'threat', 'counter', 'peek', '#science', '#power', '#cyber', '#diplomacy', '#industry', '#security'])}</div>
    <div class="row">${['<span class="income">', '<span class="adversary">', '<span class="other">'].map((o) => `<div class="fx-line">${o}<span class="fx-num">2</span>${icon('credits')}</span></div>`).join('')}</div>
    <div class="row plates"><div class="requirement at-least"><span class="fx-num">2</span>${tagMedallion('power')}</div><div class="requirement at-most"><span class="fx-num">1</span>${tagMedallion('cyber')}</div><div class="requirement replaces">${stacked()}${tagMedallion('science')}</div></div></div>`;
  await show(sheet, '.sheet .row{display:flex;flex-wrap:wrap;gap:2mm;padding:2mm}.cell{width:17mm;display:flex;flex-direction:column;align-items:center;gap:.5mm}.cell small{font:6pt sans-serif}.cell .medallion{width:6.2mm;height:6.2mm}.plates>div{height:5.2mm}');
  await (await p.$('.sheet')).screenshot({ path: path.join(OUT, 'icons.png') });
}
await close();
console.log(`render: ${ids.length} variants in out/cards/${failed ? `, ${failed} with problems` : ''}`);
process.exit(failed ? 1 : 0);
