// Screenshots of 021's cards and resources in a baseline (d0, 2 countries) game.
//   AIC_URL=http://localhost:<port>/ AIC_SHOTS=<dir> node scripts/shots-021.mjs
// Plays baseline games with the viewer's Random action, seed by seed, and saves at
// 1366x768 the first moment each of these shows:
//   viewer-021-1-opening.png  round 1's keep: the opening hand of 8
//   viewer-021-2-tier.png     a draft hand being built card by card (tier picks)
//   viewer-021-3-passed.png   the draft after a pick: hands passed to the next seat
//   viewer-021-4-stacked.png  a building built over another (status "stacked")
//   viewer-021-5-tokens.png   a token card with 2 or more tokens on it
//   viewer-021-6-power.png    an AI-board power used this round (paid by sacrificing tags)
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

// What the 021 scenes need, read off the view.
const STATE = `(() => {
  const v = window.aic.view;
  const group = (s, k) => (v.seats[s].cards.find((g) => g.key === k) ?? { cards: [] }).cards;
  const seats = v.seats.map((_, s) => ({
    drafted: group(s, 'cards.drafted').length,
    packet: group(s, 'cards.packet').length,
    stacked: group(s, 'cards.built').some((c) => c.status === 'stacked'),
    tokens: Math.max(0, ...[...group(s, 'cards.built'), ...group(s, 'cards.played')].map((c) => {
      const m = /\\((\\d+) (datasets|followers|capital|agents) on it\\)/.exec(c.text ?? '');
      return m ? Number(m[1]) : 0;
    })),
    powerUsed: (v.seats[s].board?.cards ?? []).some((c) => c.id.includes('.power.') && c.status === 'used'),
  }));
  return { round: v.round, outcome: !!v.outcome, phase: v.phase?.name ?? null, seats };
})()`;

const want = {
  'viewer-021-1-opening': (s) => s.round === 1 && s.phase === 'cards.keep' && s.seats[0].drafted === 8,
  'viewer-021-2-tier': (s) => s.phase === 'cards.tier' && s.seats.some((x) => x.packet >= 1 && x.packet <= 3),
  'viewer-021-3-passed': (s) => s.phase === 'cards.draft' && s.seats[0].drafted >= 1 && s.seats[0].packet >= 1,
  'viewer-021-4-stacked': (s) => s.seats.some((x) => x.stacked),
  'viewer-021-5-tokens': (s) => s.seats.some((x) => x.tokens >= 2),
  'viewer-021-6-power': (s) => s.seats.some((x) => x.powerUsed),
};
const saved = {};
try {
  await send('Page.enable');
  await send('Runtime.enable');
  await send('Emulation.setDeviceMetricsOverride', { width: 1366, height: 768, deviceScaleFactor: 1, mobile: false });
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start');
  for (let seed = 1; seed <= 20 && Object.keys(saved).length < Object.keys(want).length; seed++) {
    await js(`(() => { document.getElementById('config').value = 'baseline.json'; document.getElementById('seed').value = '${seed}';
                        document.getElementById('new-form').requestSubmit(); })()`);
    await until(`window.aic.timeline.last === 0 && window.aic.timeline.seed === '${seed}'`, `the seed-${seed} game`);
    for (let i = 0; i < 4000; i++) {
      const s = await js(STATE);
      if (s.outcome) break;
      for (const [name, f] of Object.entries(want)) {
        if (!saved[name] && f(s)) {
          await shot(name);
          saved[name] = { seed, round: s.round, phase: s.phase };
          console.log(name, JSON.stringify(saved[name]));
        }
      }
      if (Object.keys(saved).length === Object.keys(want).length) break;
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
process.exit(Object.keys(saved).length === Object.keys(want).length ? 0 : 1);
