// `npm run scene-picker`, after `npm run scene-gen`: everything below the raw draws, and
// picks/<picker>.json. Nothing it writes but the manifest is committed. Runs at
// below-normal priority, Chrome with it, one image at a time: the box is shared.
// Each draw is centre-cut to the program window's shape, <id>.png beside raw/; building
// windows centre-crop that again. Then its two sample cards (scenes.json) at 300 dpi in
// cards/, the same cards at true size in cards.pdf (9 to an A4 page), and one preview per
// draw, scene over cards, in art-originals/picks/<picker>/. Stops if that folder isn't
// reachable or a draw is missing.
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import sharp from 'sharp';

import { ART, CARD, REPO, TOOL, layout, openPage, readJson } from './lib.mjs';
import { inset, renderCard } from '../../web/card/card.js';

os.setPriority(os.constants.priority.PRIORITY_BELOW_NORMAL);
sharp.concurrency(1);

const s = await readJson(path.join(TOOL, 'scenes.json'));
const NAME = s.picker;
const shared = path.join(REPO, 'art-originals');
if (!existsSync(shared)) throw new Error(`scene-picker: ${shared} is not reachable: no picker written`);
const dir = path.join(shared, 'scenes', s.round);
const images = path.join(shared, 'picks', NAME);
await mkdir(path.join(dir, 'cards'), { recursive: true });
await mkdir(images, { recursive: true });

const l = inset(await layout());
const framesDir = path.join(ART, 'frames', 'r3');
const fits = await readJson(path.join(framesDir, 'fit.json'));
const cards = Object.fromEntries((await readJson(path.join(REPO, 'content', 'cards.json'))).map((c) => [c.id, c]));
const faces = await readJson(path.join(CARD, 'faces.json'));
const frameOf = (card) => `${card.kind}-t${card.tier >= 2 ? 2 : 1}`;
const frameUrl = (v) => pathToFileURL(path.join(framesDir, `${v}.jpg`)).href;

// The tier-1 program window, every draw's one shape.
const [wx0, wy0, wx1, wy1] = fits['program-t1'].art;
const SHAPE = (wx1 - wx0) / (wy1 - wy0);
async function cut(id) {
  const src = path.join(dir, 'raw', `${id}.png`);
  const m = await sharp(src).metadata();
  const cw = Math.min(m.width, Math.round(m.height * SHAPE));
  const ch = Math.min(m.height, Math.round(m.width / SHAPE));
  await sharp(src).extract({ left: Math.floor((m.width - cw) / 2), top: Math.floor((m.height - ch) / 2), width: cw, height: ch }).png().toFile(path.join(dir, `${id}.png`));
  return { width: cw, height: ch };
}

const MM = 96 / 25.4;
const scale = l.print_px[0] / (l.bleed_mm[0] * MM);
const { page, close } = await openPage(800, 600);
const ctx = await page.context().browser().newContext({ viewport: { width: 800, height: 1000 }, deviceScaleFactor: scale });
const p = await ctx.newPage();
const stub = path.join(CARD, 'card.css');
await p.goto(pathToFileURL(stub).href.replace('card.css', 'layout.json'));
const css = await readFile(stub, 'utf8');
const base = pathToFileURL(CARD + path.sep).href;
const show = async (body, extra = '') => {
  await p.setContent(`<!doctype html><html><head><base href="${base}"><style>body{margin:0}${css}${extra}</style></head><body>${body}</body></html>`, { waitUntil: 'load' });
  await p.evaluate(() => document.fonts.ready);
};
const clip = { x: 0, y: 0, width: l.bleed_mm[0] * MM, height: l.bleed_mm[1] * MM };
const face = (cardId, id) => {
  const card = { ...cards[cardId], ...faces[cardId] };
  const frame = frameOf(card);
  return renderCard(card, l, frameUrl(frame), fits[frame], pathToFileURL(path.join(dir, `${id}.png`)).href);
};

// The name of the card a scene was written for, which may be no player card.
const others = Object.fromEntries((await Promise.all(['adversary', 'alignment', 'threats'].map((f) => readJson(path.join(REPO, 'content', `${f}.json`))))).flat().map((c) => [c.id, c]));
const written = (id) => (cards[id] ?? others[id]).name;

const SHEET_W = l.print_px[0] * 2 + 20;
const rows = [];
const all = [];
const tagOf = (id) => Object.keys(s.scenes).find((tag) => s.scenes[tag][id]);
for (const [id, pair] of Object.entries(s.batch)) {
  const size = await cut(id);
  for (const c of pair) {
    await show(face(c, id));
    await p.screenshot({ path: path.join(dir, 'cards', `${id}--${c}.png`), clip });
    all.push(face(c, id));
  }
  const sceneH = Math.round((SHEET_W * size.height) / size.width);
  const [W, H] = [SHEET_W, sceneH + 20 + l.print_px[1]];
  const left = await sharp(path.join(dir, 'cards', `${id}--${pair[0]}.png`)).toBuffer();
  const right = await sharp(path.join(dir, 'cards', `${id}--${pair[1]}.png`)).toBuffer();
  const top = await sharp(path.join(dir, `${id}.png`)).resize(W, sceneH, { kernel: 'lanczos3' }).toBuffer();
  await sharp({ create: { width: W, height: H, channels: 3, background: '#1b1d20' } })
    .composite([
      { input: top, left: 0, top: 0 },
      { input: left, left: 0, top: sceneH + 20 },
      { input: right, left: l.print_px[0] + 20, top: sceneH + 20 },
    ])
    .jpeg({ quality: 86, mozjpeg: true })
    .toFile(path.join(images, `${id}.jpg`));
  const tag = tagOf(id);
  const scene = s.scenes[tag][id];
  rows.push({
    id,
    label: `${rows.length + 1}. ${s.tags[tag].label}: ${scene.subject}`,
    note: `Written for ${written(scene.card)}. Shown on ${pair.map((c) => cards[c].name).join(' and ')}. Tile on: keep. Tile off: reject.`,
    kind: 'many',
    default: ['keep'],
    options: [{ key: 'keep', image: `../art-originals/picks/${NAME}/${id}.jpg`, label: 'keep' }],
  });
}

const sheets = [];
for (let i = 0; i < all.length; i += 9) sheets.push(all.slice(i, i + 9));
await show(
  sheets.map((row) => `<section class="sheet-page">${row.join('')}</section>`).join(''),
  '@page{size:210mm 297mm;margin:0}.sheet-page{width:207mm;height:282mm;padding:7.500mm 1.500mm;display:grid;grid-template-columns:repeat(3,69mm);grid-auto-rows:94mm;break-after:page}',
);
await p.pdf({ path: path.join(dir, 'cards.pdf'), printBackground: true, preferCSSPageSize: true });
await close();

await writeFile(
  path.join(REPO, 'picks', `${NAME}.json`),
  `${JSON.stringify(
    {
      name: NAME,
      title: `Card scenes, batch ${s.round.slice(1)}`,
      intro: `Keep or reject each scene, in the C and E mix. Each tile is the bare scene over two real cards, a program or event and a building, composed at 300 dpi with the full bleed showing: the program window is the whole scene, the building window its centre. Scenes are a pool: the card a scene was written for is a suggestion. The second card only shows the other window, and adversary, Discord and Unity cards have no frame yet, so their scenes sit on the nearest fit by name. For true size, print art-originals/scenes/${s.round}/cards.pdf at 100%. Use the notes for what to change.`,
      rows,
    },
    null,
    1,
  )}\n`,
);
console.log(`scene-picker: ${rows.length} rows in picks/${NAME}.json, ${all.length} cards, ${sheets.length} pdf pages`);
