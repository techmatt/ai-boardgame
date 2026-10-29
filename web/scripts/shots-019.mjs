// Screenshots of the adversary troops' march (019) in a baseline (d0, 2 countries) game.
//   AIC_URL=http://localhost:<port>/ AIC_SHOTS=<dir> node scripts/shots-019.mjs
// Plays baseline games with the viewer's Random action, seed by seed, and saves at 1366x768:
//   viewer-troops-1-far.png   the action phase, a troop's dashed arrow on a target two
//                             or more marches away
//   viewer-troops-2-near.png  a later round's action phase: a solid arrow on a presence
//                             (the troop strikes it in the next march)
//   viewer-troops-3-hit.png   the round after: the march destroyed that presence (unless
//                             a base troop, hardening or an ambassador absorbed the hit)
// Same DevTools approach as e2e.mjs (headless Edge or Chrome, no extra packages).

import { spawn } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const url = process.env.AIC_URL ?? 'http://localhost:5173/';
const outDir = process.env.AIC_SHOTS;
if (!outDir) throw new Error('set AIC_SHOTS to the folder for the screenshots');
mkdirSync(outDir, { recursive: true });

const exe = [
  process.env.AIC_BROWSER,
  'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe',
  'C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe',
  'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
  '/usr/bin/google-chrome',
].filter(Boolean).find((p) => existsSync(p));
if (!exe) throw new Error('no browser found; set AIC_BROWSER');

const profile = mkdtempSync(join(tmpdir(), 'aic-shots-'));
const browser = spawn(exe, [
  '--headless=new', '--remote-debugging-port=0', `--user-data-dir=${profile}`,
  '--no-first-run', '--no-default-browser-check', '--window-size=1366,768', 'about:blank',
], { stdio: 'ignore' });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

let port;
for (let i = 0; i < 100 && !port; i++) {
  const f = join(profile, 'DevToolsActivePort');
  if (existsSync(f)) port = readFileSync(f, 'utf8').split('\n')[0].trim();
  if (!port) await sleep(100);
}
if (!port) throw new Error('browser did not open a DevTools port');
const pages = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
const ws = new WebSocket(pages.find((p) => p.type === 'page').webSocketDebuggerUrl);
await new Promise((r, j) => { ws.onopen = r; ws.onerror = j; });
let nextId = 1;
const waiting = new Map();
ws.onmessage = (m) => {
  const msg = JSON.parse(m.data);
  if (msg.id && waiting.has(msg.id)) { waiting.get(msg.id)(msg); waiting.delete(msg.id); }
};
const send = (method, params = {}) => new Promise((r, j) => {
  const id = nextId++;
  waiting.set(id, (msg) => (msg.error ? j(new Error(`${method}: ${msg.error.message}`)) : r(msg.result)));
  ws.send(JSON.stringify({ id, method, params }));
});
async function js(expr) {
  const r = await send('Runtime.evaluate', { expression: `(async () => (${expr}))()`, awaitPromise: true, returnByValue: true });
  if (r.exceptionDetails) throw new Error(`${expr.slice(0, 80)}: ${r.exceptionDetails.exception?.description ?? r.exceptionDetails.text}`);
  return r.result.value;
}
async function until(expr, what, ms = 30000) {
  for (let t = 0; t < ms; t += 100) {
    if (await js(expr).catch(() => false)) return;
    await sleep(100);
  }
  throw new Error(`timed out waiting for ${what}`);
}
async function shot(name) {
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
  const r = await send('Page.captureScreenshot', { format: 'png' });
  writeFileSync(join(outDir, `${name}.png`), Buffer.from(r.data, 'base64'));
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
const ACTIONS = 'cards.play+threats.counter+economy.give+alignment.act+board.act';

const saved = {};
try {
  await send('Page.enable');
  await send('Runtime.enable');
  await send('Emulation.setDeviceMetricsOverride', { width: 1366, height: 768, deviceScaleFactor: 1, mobile: false });
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start');
  for (let seed = 1; seed <= 30 && !saved.hit; seed++) {
    await js(`(() => { document.getElementById('config').value = 'baseline.json'; document.getElementById('seed').value = '${seed}';
                        document.getElementById('new-form').requestSubmit(); })()`);
    await until(`window.aic.timeline.last === 0 && window.aic.timeline.seed === '${seed}'`, `the seed-${seed} game`);
    let far = null;
    let near = null;
    for (let i = 0; i < 3000; i++) {
      const s = await js(STATE);
      if (s.outcome) { console.log(`seed ${seed}: ended in round ${s.round}`); break; }
      if (s.phase === ACTIONS) {
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
  ws.close();
  browser.kill();
  await sleep(300);
  try { rmSync(profile, { recursive: true, force: true }); } catch { /* the browser may still hold it */ }
}
console.log(JSON.stringify(saved, null, 1));
process.exit(saved.hit ? 0 : 1);
