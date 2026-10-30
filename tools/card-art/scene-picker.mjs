// `npm run scene-picker`, after `npm run scene-gen`: everything below the raw draws, and
// picks/scene_style_<round>.json. Nothing it writes but the manifest is committed.
// Each draw is centre-cut to the program window's shape, <id>.png beside raw/; building
// windows centre-crop that again. Then its two sample cards (scenes.json) at 300 dpi in
// cards/, the same cards at true size in cards.pdf (9 to an A4 page), and one preview per
// draw, scene over cards, in art-originals/picks/<picker>/. Stops if that folder isn't
// reachable or a draw is missing.
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import sharp from 'sharp';

import { ART, CARD, REPO, TOOL, layout, openPage, readJson } from './lib.mjs';
import { inset, renderCard } from '../../web/card/card.js';

const s = await readJson(path.join(TOOL, 'scenes.json'));
const NAME = `scene_style_${s.round}`;
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

const SHEET_W = l.print_px[0] * 2 + 20;
const rows = [];
const all = [];
const sceneOf = (id) => id.split('-')[1];
for (const id of s.draws) {
  const size = await cut(id);
  const pair = s.scenes[sceneOf(id)].cards;
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
  const [preamble, scene] = id.split('-');
  rows.push({
    id,
    label: `${s.scenes[scene].number}. ${s.scenes[scene].label}, style ${preamble.toUpperCase()}`,
    note: `On ${pair.map((c) => cards[c].name).join(' and ')}. Tile on: keep. Tile off: reject.`,
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

// The style row's tiles: every scene in one style, and every style side by side for the mix.
const TILE = [800, Math.round(800 / SHAPE)];
const tile = async (name, grid) => {
  const [cols, nrows] = [grid[0].length, grid.length];
  const parts = await Promise.all(
    grid.flatMap((row, y) =>
      row.map(async (id, x) => ({ input: await sharp(path.join(dir, `${id}.png`)).resize(...TILE).toBuffer(), left: x * (TILE[0] + 12), top: y * (TILE[1] + 12) })),
    ),
  );
  await sharp({ create: { width: cols * TILE[0] + (cols - 1) * 12, height: nrows * TILE[1] + (nrows - 1) * 12, channels: 3, background: '#1b1d20' } })
    .composite(parts)
    .jpeg({ quality: 86, mozjpeg: true })
    .toFile(path.join(images, `${name}.jpg`));
};
const styles = Object.keys(s.preambles);
const scenes = Object.keys(s.scenes);
for (const style of styles) await tile(`style-${style}`, [0, 2].map((i) => scenes.slice(i, i + 2).map((scene) => `${style}-${scene}`)));
await tile('style-mix', scenes.map((scene) => styles.map((style) => `${style}-${scene}`)));
rows.push({
  id: 'style',
  label: 'Which style goes forward?',
  note: `${styles.map((style) => `${style.toUpperCase()} is ${s.styles[style]}`).join(', ')}; the same four scenes each way. Mix: say in the note which goes where.`,
  kind: 'many',
  default: [],
  options: [
    ...styles.map((style) => ({ key: style, image: `../art-originals/picks/${NAME}/style-${style}.jpg`, label: style.toUpperCase() })),
    { key: 'mix', image: `../art-originals/picks/${NAME}/style-mix.jpg`, label: 'mix' },
  ],
});

await writeFile(
  path.join(REPO, 'picks', `${NAME}.json`),
  `${JSON.stringify(
    {
      name: NAME,
      title: `Card scenes, round ${s.round.slice(1)}: style test`,
      intro: `Twelve scenes to judge the look, not the subjects: four scenes in each of three illustrative styles. Each tile is the bare scene over two real cards, a program or event and a building, composed at 300 dpi with the full bleed showing: the program window is the whole scene, the building window its centre. The cards are placeholders, not assignments; scenes are a pool, and Adversary has no card tag, so its cards are the nearest fit. For true size, print art-originals/scenes/${s.round}/cards.pdf at 100%. Use the notes for what to change.`,
      rows,
    },
    null,
    1,
  )}\n`,
);
console.log(`scene-picker: ${rows.length} rows in picks/${NAME}.json, ${all.length} cards, ${sheets.length} pdf pages`);
