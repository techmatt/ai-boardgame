// End-to-end check of the viewer in a real browser (Edge or Chrome, headless, driven over
// the DevTools protocol; no extra npm packages). Needs the dev server running.
//   node scripts/e2e.mjs [record.json]
//   AIC_URL     viewer URL (default http://localhost:5173/)
//   AIC_BROWSER browser executable (default: Edge, then Chrome, at their usual paths)
// Plays a game to the end through the UI, checks undo, the record dialog, the saved
// session, hover details and the no-scroll layout at 1366x768 and 2560x1440, then loads
// the record (if given) and steps through it. Screenshots go to web/e2e-out/.

import { spawn } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const url = process.env.AIC_URL ?? 'http://localhost:5173/';
const recordPath = process.argv[2];
const out = join(web, 'e2e-out');
mkdirSync(out, { recursive: true });

const candidates = [
  process.env.AIC_BROWSER,
  'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe',
  'C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe',
  'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
  '/usr/bin/google-chrome',
  '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
].filter(Boolean);
const exe = candidates.find((p) => existsSync(p));
if (!exe) throw new Error('no browser found; set AIC_BROWSER');

const profile = mkdtempSync(join(tmpdir(), 'aic-e2e-'));
const browser = spawn(exe, [
  '--headless=new', '--remote-debugging-port=0', `--user-data-dir=${profile}`,
  '--no-first-run', '--no-default-browser-check', '--window-size=1366,768', 'about:blank',
], { stdio: 'ignore' });

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
let failures = 0;
const check = (ok, what) => {
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}`);
  if (!ok) failures++;
};

async function devtoolsPort() {
  const file = join(profile, 'DevToolsActivePort');
  for (let i = 0; i < 100; i++) {
    if (existsSync(file)) {
      const port = readFileSync(file, 'utf8').split('\n')[0].trim();
      if (port) return port;
    }
    await sleep(100);
  }
  throw new Error('browser did not open a DevTools port');
}

const port = await devtoolsPort();
const pages = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
const ws = new WebSocket(pages.find((p) => p.type === 'page').webSocketDebuggerUrl);
await new Promise((r, j) => { ws.onopen = r; ws.onerror = j; });
let nextId = 1;
const waiting = new Map();
const consoleErrors = [];
ws.onmessage = (m) => {
  const msg = JSON.parse(m.data);
  if (msg.id && waiting.has(msg.id)) {
    waiting.get(msg.id)(msg);
    waiting.delete(msg.id);
  } else if (msg.method === 'Runtime.exceptionThrown') {
    consoleErrors.push(msg.params.exceptionDetails.exception?.description ?? msg.params.exceptionDetails.text);
  } else if (msg.method === 'Runtime.consoleAPICalled' && msg.params.type === 'error') {
    consoleErrors.push(msg.params.args.map((a) => a.value ?? a.description).join(' '));
  }
};
const send = (method, params = {}) => new Promise((r, j) => {
  const id = nextId++;
  waiting.set(id, (msg) => (msg.error ? j(new Error(`${method}: ${msg.error.message}`)) : r(msg.result)));
  ws.send(JSON.stringify({ id, method, params }));
});
/** Evaluates `expr` (an expression, awaited) in the page and returns its JSON value. */
async function js(expr) {
  const r = await send('Runtime.evaluate', { expression: `(async () => (${expr}))()`, awaitPromise: true, returnByValue: true });
  if (r.exceptionDetails) throw new Error(`${expr.slice(0, 80)}: ${r.exceptionDetails.exception?.description ?? r.exceptionDetails.text}`);
  return r.result.value;
}
async function until(expr, what, ms = 10000) {
  for (let t = 0; t < ms; t += 100) {
    if (await js(expr).catch(() => false)) return true;
    await sleep(100);
  }
  throw new Error(`timed out waiting for ${what}`);
}
async function size(w, h) {
  await send('Emulation.setDeviceMetricsOverride', { width: w, height: h, deviceScaleFactor: 1, mobile: false });
  await sleep(150);
}
async function shot(name) {
  const r = await send('Page.captureScreenshot', { format: 'png' });
  writeFileSync(join(out, `${name}.png`), Buffer.from(r.data, 'base64'));
}
const noScroll = () => js(`document.documentElement.scrollHeight <= innerHeight && document.documentElement.scrollWidth <= innerWidth
  && document.body.scrollHeight <= innerHeight && document.body.scrollWidth <= innerWidth`);

try {
  await send('Page.enable');
  await send('Runtime.enable');
  await size(1366, 768);
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start', 30000);
  await js('localStorage.clear()');
  await send('Page.reload');
  await until('window.aic && window.aic.view && window.aic.timeline.last === 0', 'a fresh game');

  // Board: one data-hex per view hex, in the view's coordinates.
  const board = await js(`(() => {
    const v = window.aic.view;
    const keys = [...document.querySelectorAll('#board .hex')].map((e) => e.dataset.hex);
    return { n: v.hexes.length, keys, want: v.hexes.map((h) => h.hex.join(',')),
             labels: document.querySelectorAll('#board .country-label').length, countries: v.countries.length,
             fills: new Set([...document.querySelectorAll('#board .hex polygon')].map((p) => p.getAttribute('fill'))).size };
  })()`);
  check(board.n > 0 && JSON.stringify(board.keys) === JSON.stringify(board.want), `board has ${board.n} hexes with data-hex="q,r"`);
  check(board.labels === board.countries && board.fills === board.countries, `${board.countries} countries labelled and tinted`);

  // Hover a hex and a token: the details card shows.
  const hover = async (sel) => {
    const pt = await js(`(() => { const r = document.querySelector('${sel}')?.getBoundingClientRect(); return r && [r.x + r.width / 2, r.y + r.height / 2]; })()`);
    if (!pt) return '';
    await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: pt[0], y: pt[1] });
    await sleep(50);
    return js(`document.getElementById('tip').hidden ? '' : document.getElementById('tip').textContent`);
  };
  check((await hover('#board .hex polygon')).includes('hex'), 'hovering a hex shows its details');
  check((await hover('#board .token circle')).includes('×'), 'hovering a token shows its details');
  check((await hover('.value')).length > 0, 'hovering a value shows its details');
  await shot('play-start-1366');

  // Play the whole game through the UI: random clicks on what the seat panels offer.
  let clicks = 0;
  while (clicks < 3000 && !(await js(`!!document.querySelector('#phase-bar .outcome')`))) {
    const did = await js(`(() => {
      const tgt = document.querySelectorAll('#board .hex.target polygon');
      if (tgt.length) { tgt[Math.floor(Math.random() * tgt.length)].dispatchEvent(new MouseEvent('click', { bubbles: true })); return 'hex'; }
      const bs = [...document.querySelectorAll('#seats .actions button, #seats .pass-all')];
      if (!bs.length) return '';
      // Favour plays over passes so games reach the cards.
      const plays = bs.filter((b) => !b.classList.contains('pass') && !b.classList.contains('pass-all'));
      const pool = plays.length && Math.random() < 0.7 ? plays : bs;
      pool[Math.floor(Math.random() * pool.length)].click();
      return 'button';
    })()`);
    if (!did) break;
    clicks++;
  }
  const end = await js(`({ outcome: window.aic.view.outcome, n: window.aic.timeline.last, legal: window.aic.legal.length,
                          text: document.querySelector('#phase-bar .outcome')?.textContent })`);
  check(!!end.outcome && end.legal === 0, `played to the end in ${clicks} clicks, ${end.n} actions: ${end.text}`);
  const played = await js(`window.aic.timeline.actions.filter((a) => a.type !== 'pass').length`);
  check(played > 0, `${played} non-pass actions taken through the UI`);
  await shot('play-end-1366');

  // Layout: no page scroll at both ends of the supported range.
  for (const [w, h] of [[1366, 768], [1920, 1080], [2560, 1440]]) {
    await size(w, h);
    check(await noScroll(), `no page scroll at ${w}x${h}`);
    const svg = await js(`(() => { const r = document.getElementById('board').getBoundingClientRect(); return [r.width, r.height]; })()`);
    check(svg[0] > w * 0.3 && svg[1] > h * 0.45, `board fills its pane at ${w}x${h} (${svg.map(Math.round).join('x')})`);
    await shot(`layout-${w}x${h}`);
  }
  await size(1366, 768);

  // Undo by replay, and the saved session survives a reload.
  await js(`document.getElementById('undo').click()`);
  const afterUndo = await js('window.aic.timeline.last');
  check(afterUndo === end.n - 1 && !(await js('window.aic.view.outcome')), 'undo drops the last action and reopens the game');
  await send('Page.reload');
  await until('window.aic && window.aic.view', 'reload');
  check((await js('window.aic.timeline.last')) === afterUndo, 'the session is restored from localStorage after a reload');

  // Bug report: the record dialog holds the game in the record format.
  await js(`document.getElementById('record').click()`);
  const rec = await js(`JSON.parse(document.getElementById('record-text').value)`);
  check(rec.format === 'aic-game-record/1' && rec.actions.length === afterUndo && !!rec.config && /^\d+$/.test(rec.seed),
    `record dialog: ${rec.format}, seed ${rec.seed}, ${rec.actions.length} actions`);
  await shot('record-dialog');
  await js(`document.getElementById('record-close').click()`);

  // Watch a record from the simulator: load, step forward and back, reach its end.
  if (recordPath) {
    const text = readFileSync(recordPath, 'utf8');
    const want = JSON.parse(text);
    await js(`(() => { document.getElementById('record').click(); document.getElementById('record-text').value = ${JSON.stringify(text)};
                        document.getElementById('record-load').click(); })()`);
    await until(`window.aic.timeline.source.kind === 'record'`, 'the record to load');
    const loaded = await js(`({ n: window.aic.timeline.last, cursor: window.aic.timeline.cursor, err: document.getElementById('error').hidden ? '' : document.getElementById('error').textContent })`);
    check(loaded.n === want.actions.length && loaded.cursor === 0 && !loaded.err, `record loaded: ${loaded.n} actions, at step 0 ${loaded.err}`);
    await send('Input.dispatchKeyEvent', { type: 'keyDown', key: 'ArrowRight', code: 'ArrowRight', windowsVirtualKeyCode: 39 });
    await send('Input.dispatchKeyEvent', { type: 'keyUp', key: 'ArrowRight', code: 'ArrowRight', windowsVirtualKeyCode: 39 });
    await js(`document.getElementById('fwd').click()`);
    check((await js('window.aic.timeline.cursor')) === 2, 'steps forward (key and button)');
    await js(`document.getElementById('back').click()`);
    check((await js('window.aic.timeline.cursor')) === 1, 'steps back');
    await js(`document.getElementById('end').click()`);
    const fin = await js(`({ o: window.aic.view.outcome, cursor: window.aic.timeline.cursor, now: document.querySelector('#tab-log .entry.now')?.dataset.step })`);
    const same = (a, b) => (a && b ? ['won', 'round', 'reason', 'detail'].every((k) => a[k] === b[k]) : a == b);
    check(same(fin.o, want.outcome) && fin.cursor === want.actions.length && Number(fin.now) === fin.cursor,
      `end of the record matches its saved outcome (${fin.o?.reason}) and the log follows the cursor`);
    await js(`document.querySelector('#tab-log .entry[data-step="3"]').click()`);
    check((await js('window.aic.timeline.cursor')) === 3, 'clicking a log entry jumps to that step');
    await shot('watch-record');
    await js(`document.getElementById('first').click()`);
    check((await js('window.aic.timeline.cursor')) === 0, 'back to the start');
  } else {
    console.log('skip watching a record (no record path given)');
  }

  check(consoleErrors.length === 0, `no page errors${consoleErrors.length ? `: ${consoleErrors.join(' | ')}` : ''}`);
} catch (e) {
  console.error(e);
  failures++;
} finally {
  ws.close();
  browser.kill();
  await sleep(300);
  try { rmSync(profile, { recursive: true, force: true }); } catch { /* the browser may still hold it */ }
}
console.log(failures ? `e2e FAILED (${failures})` : 'e2e ok');
process.exit(failures ? 1 : 0);
