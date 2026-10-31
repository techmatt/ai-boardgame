// `npm run gen -- <direction>-<kind>-<seed>… [--pool=N]`: draw tier-1 frames.
// The image tool has no seed parameter: a "seed" is an independent draw, keyed 1, 2, …
// A redraw of `steel-program-1` replaces it; the earlier original stays as
// steel-program-1.try<n>.png. Originals and log.jsonl go to the originals folder
// (lib.mjs). Print-size PNGs go next to them in frames/; a web-size JPEG, the committed
// template, to art/frames/r3/. A draw is raw: `npm run derive` only reads master.jpg, so
// promoting one means copying it over the master.
// Stops at the first usage-limit message. Exits 1 if a draw failed or the limit left some
// undrawn.
import { appendFile, copyFile, mkdir, readFile, rename } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';

import sharp from 'sharp';

import { ART, ORIGINALS, TOOL, layout, nextTry, readJson, runCodex } from './lib.mjs';

const argv = process.argv.slice(2);
const pool = Number(argv.find((a) => a.startsWith('--pool='))?.slice(7) ?? 3);
if (!Number.isInteger(pool) || pool < 1) throw new Error('gen: --pool must be a whole number, 1 or more');
const ids = argv.filter((a) => !a.startsWith('--'));
const p = await readJson(path.join(TOOL, 'prompts.json'));
for (const id of ids) {
  const [dir, kind] = id.split('-');
  if (!p.directions[dir]) throw new Error(`gen: ${id}: no direction ${dir}: nothing drawn`);
  if (!p.kinds[kind]) throw new Error(`gen: ${id}: no kind ${kind}: nothing drawn`);
}
const l = await layout();
const [W, H] = l.print_px;
const origDir = path.join(ORIGINALS, 'card-frames-r3');
const printDir = path.join(ORIGINALS, 'card-frames-r3', 'frames');
const webDir = path.join(ART, 'frames', 'r3');
await mkdir(printDir, { recursive: true });
await mkdir(webDir, { recursive: true });

export const framePrompt = (dir, kind) =>
  [
    'Use your built-in image_gen tool ($imagegen) exactly once to generate the image below.',
    'Do not read any files, run any commands, or try to save the image anywhere: just generate it, then stop.',
    'Image shape: portrait, width to height 69:94, the attached guide\'s shape.',
    '',
    p.frame.replace('{top}', kind === 'building' ? p.top : '').replace('{kind}', p.kinds[kind].colour) + p.directions[dir].style,
  ].join('\n');

/** Crop centred to the bleed's shape, then scale: the kept original is untouched. */
async function cut(src, id) {
  const m = await sharp(src).metadata();
  const want = W / H;
  const cw = Math.min(m.width, Math.round(m.height * want));
  const ch = Math.min(m.height, Math.round(m.width / want));
  const img = sharp(src).extract({ left: Math.floor((m.width - cw) / 2), top: Math.floor((m.height - ch) / 2), width: cw, height: ch }).resize(W, H, { kernel: 'lanczos3' });
  await img.clone().png().toFile(path.join(printDir, `${id}.png`));
  await img.clone().jpeg({ quality: 88, mozjpeg: true }).toFile(path.join(webDir, `${id}.jpg`));
  return { w: m.width, h: m.height };
}

let stop = false;
let failed = 0;
async function one(id) {
  const [dir, kind] = id.split('-');
  const guide = path.join(ART, 'guides', `${kind}-t1-blocks.png`);
  let run = await runCodex(framePrompt(dir, kind), guide);
  let ref = !!run.image;
  if (!run.image && !run.limit) {
    console.error(`gen: ${id}: no image with the guide (${run.message ?? '?'}); retrying with text alone`);
    run = await runCodex(framePrompt(dir, kind));
    ref = false;
  }
  await appendFile(path.join(origDir, 'log.jsonl'), `${JSON.stringify({ id, at: new Date().toISOString(), ref, seconds: run.seconds, thread: run.thread, limit: run.limit, usage: run.usage, message: run.image ? undefined : run.message })}\n`);
  if (run.limit) {
    stop = true;
    console.error(`gen: usage limit: ${run.limit}`);
  }
  if (!run.image) {
    failed++;
    console.error(`gen: ${id}: no image after ${run.seconds}s. ${run.message ?? ''}`);
    return;
  }
  const dest = path.join(origDir, `${id}.png`);
  if (existsSync(dest)) await rename(dest, path.join(origDir, `${id}.try${await nextTry(origDir, id)}.png`));
  await copyFile(run.image, dest);
  const s = await cut(await readFile(dest), id);
  console.log(`gen: ${id}: ${s.w}×${s.h}, ${run.seconds}s, ref ${ref}, usage ${JSON.stringify(run.usage)}`);
}

const queue = [...ids];
await Promise.all(
  Array.from({ length: pool }, async () => {
    while (queue.length && !stop) await one(queue.shift());
  }),
);
if (failed || queue.length) {
  process.exitCode = 1;
  console.error(`gen: ${ids.length - failed - queue.length} of ${ids.length} drawn, ${failed} failed, ${queue.length} not started`);
}
