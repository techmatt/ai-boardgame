// `npm run picker`: picks/card_frames_r2.json, with its images in the shared folder
// (art-originals/picks/card_frames_r2/), never committed: composed previews from
// out/cards/ (run `npm run render` first) and a copy of each bare frame. One row per
// draw, by direction then kind. The picker's rows are single-choice, so the bare-frame
// tile is the mark: selecting it means carry forward. Stops if the folder isn't reachable.
import { copyFile, mkdir, readdir, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';

import sharp from 'sharp';

import { ART, OUT, REPO, TOOL, readJson } from './lib.mjs';

const prompts = await readJson(path.join(TOOL, 'prompts.json'));
const directions = Object.keys(prompts.directions);
const kinds = Object.keys(prompts.kinds);
const framesDir = path.join(ART, 'frames', 'r2');
const rank = (id) => {
  const [dir, kind, seed] = id.split('-');
  return directions.indexOf(dir) * 100 + kinds.indexOf(kind) * 10 + Number(seed);
};
const ids = (await readdir(framesDir))
  .filter((f) => f.endsWith('.jpg'))
  .map((f) => f.slice(0, -4))
  .sort((a, b) => rank(a) - rank(b));

const shared = path.join(REPO, 'art-originals');
if (!existsSync(shared)) throw new Error(`picker: ${shared} is not reachable: no picker written`);
const images = path.join(shared, 'picks', 'card_frames_r2');
await mkdir(images, { recursive: true });

const rows = [];
for (const id of ids) {
  await sharp(path.join(OUT, `${id}.png`)).resize(520).jpeg({ quality: 82, mozjpeg: true }).toFile(path.join(images, `${id}.jpg`));
  await copyFile(path.join(framesDir, `${id}.jpg`), path.join(images, `${id}-frame.jpg`));
  const [dir, kind, seed] = id.split('-');
  rows.push({
    id,
    label: `${prompts.directions[dir].label}, ${prompts.kinds[kind].label}, draw ${seed}`,
    options: [
      { key: 'composed', image: `../art-originals/picks/card_frames_r2/${id}.jpg`, label: 'composed sample' },
      { key: 'carry', image: `../art-originals/picks/card_frames_r2/${id}-frame.jpg`, label: 'bare frame: select to carry forward' },
    ],
  });
}
await mkdir(path.join(REPO, 'picks'), { recursive: true });
await writeFile(
  path.join(REPO, 'picks', 'card_frames_r2.json'),
  `${JSON.stringify(
    {
      name: 'card_frames_r2',
      title: 'Card frames, round 2: Lab glass and Brushed steel, in three kinds',
      intro: 'One row per draw, by direction, then kind: the composed sample on the left, the bare frame on the right. To carry a frame forward, select its bare-frame tile; leave the left tile selected to drop it. Use the note for what to change. Art windows are empty on purpose; icons are placeholders. The building and event samples are tier-2 cards on tier-1 frames (tier 2 comes once a direction is final); the event has its insight cost removed to show the octagon alone, and the building leaves off its chip and antimatter costs.',
      rows,
    },
    null,
    1,
  )}\n`,
);
console.log(`picker: ${rows.length} rows in picks/card_frames_r2.json`);
