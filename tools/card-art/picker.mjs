// `npm run picker`: picks/card_frames_r1.json plus small composed previews, from
// out/cards/ (run `npm run render` first). One row per image: composed card, bare frame.
// The picker's rows are single-choice, so the bare-frame tile is the mark: selecting
// it means carry forward. The previews exist only for this picker: they are disposable
// once Matt has picked. The bare frames are the committed templates.
import { mkdir, writeFile, readdir } from 'node:fs/promises';
import path from 'node:path';

import sharp from 'sharp';

import { ART, OUT, REPO, TOOL, readJson } from './lib.mjs';

const prompts = await readJson(path.join(TOOL, 'prompts.json'));
const order = Object.keys(prompts.directions);
const framesDir = path.join(ART, 'frames', 'r1');
const ids = (await readdir(framesDir))
  .filter((f) => f.endsWith('.jpg'))
  .map((f) => f.slice(0, -4))
  .sort((a, b) => order.indexOf(a.split('-')[0]) - order.indexOf(b.split('-')[0]) || a.localeCompare(b));
const previews = path.join(ART, 'picks', 'card_frames_r1');
await mkdir(previews, { recursive: true });

const rows = [];
for (const id of ids) {
  await sharp(path.join(OUT, `${id}.png`)).resize(520).jpeg({ quality: 82, mozjpeg: true }).toFile(path.join(previews, `${id}.jpg`));
  const [dir, seed] = id.split('-');
  rows.push({
    id,
    label: `${prompts.directions[dir].label}, seed ${seed}`,
    options: [
      { key: 'composed', image: `../art/picks/card_frames_r1/${id}.jpg`, label: 'composed sample' },
      { key: 'carry', image: `../art/frames/r1/${id}.jpg`, label: 'bare frame: select to carry forward' },
    ],
  });
}
await mkdir(path.join(REPO, 'picks'), { recursive: true });
await writeFile(
  path.join(REPO, 'picks', 'card_frames_r1.json'),
  `${JSON.stringify(
    {
      name: 'card_frames_r1',
      title: 'Card frames, round 1: style directions',
      intro: 'One row per painted frame: the composed Sentiment Mining sample on the left, the bare frame on the right. To carry a frame forward, select its bare-frame tile; leave the left tile selected to drop it. Use the note for what to change. The "changed" count is how many you are carrying. Art windows are empty on purpose; icons are placeholders.',
      rows,
    },
    null,
    1,
  )}\n`,
);
console.log(`picker: ${rows.length} rows in picks/card_frames_r1.json`);
