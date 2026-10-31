// `npm run scene-gen -- [scene_id …] [--pool=N]`: draw card scenes (scenes.json), the whole
// batch with no ids named. A redraw replaces it; the earlier original stays as
// <id>.try<n>.png. Originals and log.jsonl go to art-originals/scenes/<round>/raw/, never
// committed. Stops at the first usage-limit message. Exits 1 if a draw failed or the limit
// left some undrawn.
import { appendFile, copyFile, mkdir, rename } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';

import { ORIGINALS, TOOL, nextTry, readJson, runCodex } from './lib.mjs';

const argv = process.argv.slice(2);
const pool = Number(argv.find((a) => a.startsWith('--pool='))?.slice(7) ?? 3);
if (!Number.isInteger(pool) || pool < 1) throw new Error('scene-gen: --pool must be a whole number, 1 or more');
const named = argv.filter((a) => !a.startsWith('--'));
const s = await readJson(path.join(TOOL, 'scenes.json'));
const ids = named.length ? named : Object.keys(s.batch);
const scenes = Object.fromEntries(Object.entries(s.scenes).flatMap(([tag, group]) => Object.entries(group).map(([id, scene]) => [id, { ...scene, tag }])));
const unknown = ids.filter((id) => !scenes[id]);
if (unknown.length) throw new Error(`scene-gen: no scene ${unknown.join(', ')}: nothing drawn`);
const dir = path.join(ORIGINALS, 'scenes', s.round, 'raw');
await mkdir(dir, { recursive: true });

const scenePrompt = (scene) =>
  [
    'Use your built-in image_gen tool ($imagegen) exactly once to generate the image below.',
    'Do not read any files, run any commands, or try to save the image anywhere: just generate it, then stop.',
    'Image shape: landscape, 3:2.',
    '',
    `${s.preamble} ${s.emphasis} ${s.lighting} ${s.framing} ${scene.subject} The tag colour is ${s.tags[scene.tag].accent}.`,
  ].join('\n');

let stop = false;
let failed = 0;
async function one(id) {
  const run = await runCodex(scenePrompt(scenes[id]));
  await appendFile(path.join(dir, 'log.jsonl'), `${JSON.stringify({ id, at: new Date().toISOString(), seconds: run.seconds, thread: run.thread, limit: run.limit, usage: run.usage, message: run.image ? undefined : run.message })}\n`);
  if (run.limit) {
    stop = true;
    console.error(`scene-gen: usage limit: ${run.limit}`);
  }
  if (!run.image) {
    failed++;
    console.error(`scene-gen: ${id}: no image after ${run.seconds}s. ${run.message ?? ''}`);
    return;
  }
  const dest = path.join(dir, `${id}.png`);
  if (existsSync(dest)) await rename(dest, path.join(dir, `${id}.try${await nextTry(dir, id)}.png`));
  await copyFile(run.image, dest);
  console.log(`scene-gen: ${id}: ${run.seconds}s, usage ${JSON.stringify(run.usage)}`);
}

const queue = [...ids];
await Promise.all(
  Array.from({ length: pool }, async () => {
    while (queue.length && !stop) await one(queue.shift());
  }),
);
if (failed || queue.length) {
  process.exitCode = 1;
  console.error(`scene-gen: ${ids.length - failed - queue.length} of ${ids.length} drawn, ${failed} failed, ${queue.length} not started`);
}
