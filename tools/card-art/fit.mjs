// `node fit.mjs`: measure where each painted frame really put its plates, into
// art/frames/r1/fit.json (mm boxes the renderer uses instead of the guide's). The image
// tool lands within a millimetre or two, and not uniformly, so one offset cannot fix a
// frame. Each plate is light inside a darker rim: flood from the guide's centre over the
// lightly blurred image (texture would stop it), take the bounding box. When no
// threshold gives a box of about the right size, the guide's stays.
import { writeFile, readdir } from 'node:fs/promises';
import path from 'node:path';

import sharp from 'sharp';

import { ART, layout } from './lib.mjs';

const l = await layout();
const mm = l.print_px[0] / l.bleed_mm[0];
// A pixel below this share of the seed's brightness ends the plate. Shiny rims leak at
// the lower ones, textured plates stop short at the higher: first size that fits wins.
const DARK = [0.92, 0.88, 0.84, 0.8, 0.74, 0.66];
const SIZE_SLACK = 1.8; // mm a plate may differ from the guide in width or height

function flood(data, W, H, [cx, cy], [x0, y0, x1, y1], dark) {
  const at = (x, y) => (y * W + x) * 3;
  const sx = Math.round(cx * mm);
  const sy = Math.round(cy * mm);
  const lum = (i) => data[i] + data[i + 1] + data[i + 2];
  const floor = lum(at(sx, sy)) * dark;
  const lim = [Math.max(0, Math.floor((x0 - 3) * mm)), Math.max(0, Math.floor((y0 - 3) * mm)), Math.min(W - 1, Math.ceil((x1 + 3) * mm)), Math.min(H - 1, Math.ceil((y1 + 3) * mm))];
  const seen = new Uint8Array(W * H);
  const stack = [[sx, sy]];
  let [bx0, by0, bx1, by1] = [sx, sy, sx, sy];
  while (stack.length) {
    const [x, y] = stack.pop();
    if (x < lim[0] || y < lim[1] || x > lim[2] || y > lim[3] || seen[y * W + x]) continue;
    seen[y * W + x] = 1;
    const i = at(x, y);
    if (lum(i) < floor) continue;
    if (x < bx0) bx0 = x;
    if (x > bx1) bx1 = x;
    if (y < by0) by0 = y;
    if (y > by1) by1 = y;
    stack.push([x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]);
  }
  return [bx0 / mm, by0 / mm, (bx1 + 1) / mm, (by1 + 1) / mm];
}

const r2 = (b) => b.map((v) => Math.round(v * 100) / 100);

function measure(data, W, H, zone, seed = [(zone[0] + zone[2]) / 2, (zone[1] + zone[3]) / 2]) {
  for (const dark of DARK) {
    const b = flood(data, W, H, seed, zone, dark);
    if (Math.abs(b[2] - b[0] - (zone[2] - zone[0])) <= SIZE_SLACK && Math.abs(b[3] - b[1] - (zone[3] - zone[1])) <= SIZE_SLACK) return { box: r2(b), ok: true };
  }
  return { box: zone, ok: false };
}

const c = l.common;
const d = c.tags[3] - c.tags[1];
const circles = [2, 1, 0].map((i) => {
  const x1 = c.tags[2] - i * (d + 0.5);
  return [x1 - d, c.tags[1], x1, c.tags[3]];
});

const dir = path.join(ART, 'frames', 'r1');
const out = {};
for (const f of (await readdir(dir)).filter((n) => n.endsWith('.jpg')).sort()) {
  const id = f.slice(0, -4);
  const { data, info } = await sharp(path.join(dir, f)).blur(1.2).removeAlpha().raw().toBuffer({ resolveWithObject: true });
  const m = {
    cost: measure(data, info.width, info.height, c.cost),
    insight: measure(data, info.width, info.height, c.insight),
    banner: measure(data, info.width, info.height, c.banner),
    number: measure(data, info.width, info.height, c.number, [(c.number[0] + c.number[2]) / 2, 88]),
  };
  const tags = circles.map((z) => measure(data, info.width, info.height, z));
  const missed = [...Object.entries(m).filter(([, v]) => !v.ok).map(([k]) => k), ...tags.flatMap((t, i) => (t.ok ? [] : [`tag${i}`]))];
  out[id] = { ...Object.fromEntries(Object.entries(m).map(([k, v]) => [k, v.box])), tags: tags.map((t) => t.box) };
  console.log(`fit: ${id}: ${missed.length ? `guide kept for ${missed.join(', ')}` : 'all measured'}`);
}
await writeFile(path.join(dir, 'fit.json'), `${JSON.stringify(out, null, 1)}\n`);
