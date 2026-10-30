// `npm run scene-gen -- [a-science …] [--pool=N]`: draw card scenes (scenes.json), all of
// them with no ids named. An id is <preamble>-<scene>. A redraw replaces it; the earlier
// original stays as <id>.try<n>.png. Originals and log.jsonl go to
// art-originals/scenes/<round>/raw/, never committed. Stops at the first usage-limit message.
import { appendFile, copyFile, mkdir, readdir, rename } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';

import { ORIGINALS, TOOL, readJson, runCodex } from './lib.mjs';

const argv = process.argv.slice(2);
const pool = Number(argv.find((a) => a.startsWith('--pool='))?.slice(7) ?? 3);
const named = argv.filter((a) => !a.startsWith('--'));
const s = await readJson(path.join(TOOL, 'scenes.json'));
const ids = named.length ? named : s.draws;
const dir = path.join(ORIGINALS, 'scenes', s.round, 'raw');
await mkdir(dir, { recursive: true });

const scenePrompt = (preamble, scene) =>
  [
    'Use your built-in image_gen tool ($imagegen) exactly once to generate the image below.',
    'Do not read any files, run any commands, or try to save the image anywhere: just generate it, then stop.',
    'Image shape: landscape, 3:2.',
    '',
    `${s.preambles[preamble]} ${s.scenes[scene].subject}`,
  ].join('\n');

let stop = false;
async function one(id) {
  const [preamble, scene] = id.split('-');
  if (!s.preambles[preamble]) throw new Error(`no preamble ${preamble}`);
  if (!s.scenes[scene]) throw new Error(`no scene ${scene}`);
  const run = await runCodex(scenePrompt(preamble, scene));
  await appendFile(path.join(dir, 'log.jsonl'), `${JSON.stringify({ id, at: new Date().toISOString(), seconds: run.seconds, thread: run.thread, limit: run.limit, usage: run.usage, message: run.image ? undefined : run.message })}\n`);
  if (run.limit) {
    stop = true;
    console.error(`scene-gen: usage limit: ${run.limit}`);
  }
  if (!run.image) {
    console.error(`scene-gen: ${id}: no image after ${run.seconds}s. ${run.message ?? ''}`);
    return;
  }
  const dest = path.join(dir, `${id}.png`);
  if (existsSync(dest)) {
    const tries = (await readdir(dir)).filter((f) => f.startsWith(`${id}.try`)).length;
    await rename(dest, path.join(dir, `${id}.try${tries + 1}.png`));
  }
  await copyFile(run.image, dest);
  console.log(`scene-gen: ${id}: ${run.seconds}s, usage ${JSON.stringify(run.usage)}`);
}

const queue = [...ids];
await Promise.all(
  Array.from({ length: pool }, async () => {
    while (queue.length && !stop) await one(queue.shift());
  }),
);
