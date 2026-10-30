// `node sheet.mjs <out.png> <image>…`: a one-row inspection sheet, 390 px a tile, in out/.
import path from 'node:path';

import sharp from 'sharp';

const [out, ...images] = process.argv.slice(2);
const W = 390;
const H = Math.round((W * 1110) / 815);
const tiles = await Promise.all(images.map(async (f, i) => ({ input: await sharp(f).resize(W, H).toBuffer(), left: i * (W + 6), top: 0 })));
await sharp({ create: { width: images.length * (W + 6), height: H, channels: 3, background: '#fff' } })
  .composite(tiles)
  .png()
  .toFile(path.resolve(out));
