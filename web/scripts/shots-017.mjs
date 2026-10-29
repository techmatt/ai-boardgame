// Screenshots of the adversary's project row (017) in a baseline (d0, 2 countries) game.
//   AIC_URL=http://localhost:<port>/ AIC_SHOTS=<dir> node scripts/shots-017.mjs
// Plays baseline games with the viewer's Random action, seed by seed, and saves at 1366x768:
//   viewer-projects-r2.png    round 2, a visible project part-filled with compute
//   viewer-projects-r<n>.png  a later round (4+), a project part-filled and at least one resolved
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

// What the project row shows now: each visible project's fill, the resolved count, the board size.
const STATE = `(() => {
  const v = window.aic.view;
  const pile = (k) => v.piles.find((p) => p.key === k);
  const row = (pile('adversary.projects')?.visible ?? []).map((c) => {
    const m = /(\\d+)\\s*\\/\\s*(\\d+)/.exec(c.status ?? '');
    return { name: c.name, have: m ? +m[1] : 0, cost: m ? +m[2] : 0, status: c.status };
  });
  const r = document.getElementById('board').getBoundingClientRect();
  return { round: v.round, outcome: !!v.outcome, row, resolved: pile('adversary.projects_done')?.count ?? 0,
           ongoing: (pile('adversary.projects_ongoing')?.visible ?? []).map((c) => c.name),
           compute: v.globals.find((g) => g.key === 'adversary.compute')?.value,
           drawn: document.querySelectorAll('#pipeline .projects-row .project .prog').length,
           board: [Math.round(r.width), Math.round(r.height)] };
})()`;
const midFill = (s) => s.row.some((p) => p.have > 0 && p.have < p.cost);

const saved = {};
try {
  await send('Page.enable');
  await send('Runtime.enable');
  await send('Emulation.setDeviceMetricsOverride', { width: 1366, height: 768, deviceScaleFactor: 1, mobile: false });
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start');
  for (let seed = 1; seed <= 30 && !(saved.r2 && saved.late); seed++) {
    await js(`(() => { document.getElementById('config').value = 'baseline.json'; document.getElementById('seed').value = '${seed}';
                        document.getElementById('new-form').requestSubmit(); })()`);
    await until(`window.aic.timeline.last === 0 && window.aic.timeline.seed === '${seed}'`, `the seed-${seed} game`);
    for (let i = 0; i < 2000; i++) {
      const s = await js(STATE);
      if (s.outcome) { console.log(`seed ${seed}: ended in round ${s.round}, ${s.resolved} resolved`); break; }
      if (!saved.r2 && s.round === 2 && midFill(s)) {
        await shot('viewer-projects-r2');
        saved.r2 = { seed, ...s };
      }
      // A resolved project leaves its slot empty until all three have gone (rules.md), so
      // this row has fewer than three.
      if (!saved.late && s.round >= 4 && s.resolved > 0 && midFill(s)) {
        await shot(`viewer-projects-r${s.round}`);
        saved.late = { seed, ...s };
        break;
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
process.exit(saved.r2 && saved.late ? 0 : 1);
