// Screenshots of the adversary troops' march (019) in a baseline (d0, 2 countries) game.
//   AIC_URL=http://localhost:<port>/ AIC_SHOTS=<dir> node scripts/shots-019.mjs
// Plays baseline games with the viewer's Random action, seed by seed, and saves at 1366x768:
//   viewer-troops-1-far.png   the action phase, a troop's dashed arrow on a target two
//                             or more marches away
//   viewer-troops-2-near.png  a later round's action phase: a solid arrow on a presence
//                             (the troop strikes it in the next march)
//   viewer-troops-3-hit.png   the round after: the march destroyed that presence (unless
//                             a base troop, hardening or an ambassador absorbed the hit)
// Drives the page with cdp.mjs (headless Edge or Chrome, no extra packages).

import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { openPage } from './cdp.mjs';

const url = process.env.AIC_URL ?? 'http://localhost:5173/';
const outDir = process.env.AIC_SHOTS;
if (!outDir) throw new Error('set AIC_SHOTS to the folder for the screenshots');
mkdirSync(outDir, { recursive: true });

const page = await openPage();
const { send, js, until } = page;
async function shot(name) {
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
  writeFileSync(join(outDir, `${name}.png`), await page.shot());
}

// The troops, their arrows and the presence on the board now.
const STATE = `(() => {
  const v = window.aic.view;
  const presence = v.hexes.filter((h) => h.tokens.some((t) => t.kind.startsWith('presence'))).map((h) => h.index);
  const troops = v.hexes.filter((h) => h.tokens.some((t) => t.kind === 'troop')).map((h) => h.index);
  return { round: v.round, outcome: !!v.outcome, phase: v.phase?.name ?? null, presence, troops,
           arrows: (v.arrows ?? []).map((a) => ({ from: a.from, to: a.to, rounds: a.rounds, doc: a.doc })),
           drawn: document.querySelectorAll('#board .arrows .arrow').length,
           soon: document.querySelectorAll('#board .arrows .arrow.soon').length };
})()`;

const saved = {};
try {
  await page.size(1366, 768);
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start', 30000);
  for (let seed = 1; seed <= 30 && !saved.hit; seed++) {
    await js(`(() => { document.getElementById('config').value = 'baseline.json'; document.getElementById('seed').value = '${seed}';
                        document.getElementById('new-form').requestSubmit(); })()`);
    await until(`window.aic.timeline.last === 0 && window.aic.timeline.seed === '${seed}'`, `the seed-${seed} game`, 30000);
    let far = null;
    let near = null;
    for (let i = 0; i < 3000; i++) {
      const s = await js(STATE);
      if (s.outcome) { console.log(`seed ${seed}: ended in round ${s.round}`); break; }
      // The action phase, whichever modules share it with cards.play.
      if (s.phase?.startsWith('cards.play')) {
        if (!far) {
          const a = s.arrows.find((x) => x.rounds >= 2);
          if (a) {
            await shot('viewer-troops-1-far');
            far = { seed, round: s.round, arrow: a, troops: s.troops };
          }
        } else if (!near && s.round > far.round) {
          const a = s.arrows.find((x) => x.rounds === 1 && x.doc.includes('presence'));
          if (a) {
            await shot('viewer-troops-2-near');
            near = { round: s.round, arrow: a, troops: s.troops };
          }
        } else if (near && s.round === near.round + 1) {
          await shot('viewer-troops-3-hit');
          Object.assign(saved, { far, near, hit: { round: s.round, presenceStillThere: s.presence.includes(near.arrow.to), troops: s.troops } });
          break;
        }
      }
      await js(`document.getElementById('random-action').click()`);
    }
  }
} finally {
  await page.close();
}
console.log(JSON.stringify(saved, null, 1));
process.exit(saved.hit ? 0 : 1);
