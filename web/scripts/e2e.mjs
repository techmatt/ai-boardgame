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
/** Reloads the page and waits for the new document's viewer (not the old one's) and `expr`. */
async function reload(expr, what) {
  await js('(window.__aicOld = true)');
  await send('Page.reload');
  await until(`!window.__aicOld && window.aic && window.aic.view && (${expr})`, what, 30000);
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
  await reload('window.aic.timeline.last === 0', 'a fresh game');

  // Board: one data-hex per view hex, in the view's coordinates.
  const board = await js(`(() => {
    const v = window.aic.view;
    const keys = [...document.querySelectorAll('#board .hex')].map((e) => e.dataset.hex);
    return { n: v.hexes.length, keys, want: v.hexes.map((h) => h.hex.join(',')),
             labels: document.querySelectorAll('#board .country-label').length, countries: v.countries.length,
             borders: document.querySelectorAll('#board .border').length,
             terrains: new Set(v.hexes.map((h) => h.terrain)).size,
             fills: new Set([...document.querySelectorAll('#board .hex .cell')].map((p) => p.getAttribute('fill'))).size,
             uplinks: v.hexes.filter((h) => h.uplink).length, antennas: document.querySelectorAll('#board .uplink').length };
  })()`);
  check(board.n > 0 && JSON.stringify(board.keys) === JSON.stringify(board.want), `board has ${board.n} hexes with data-hex="q,r"`);
  check(board.labels === board.countries && board.borders > 0, `${board.countries} countries labelled, ${board.borders} border edges`);
  check(board.fills === board.terrains, `hexes filled by terrain (${board.fills} fills for ${board.terrains} terrains)`);
  check(board.uplinks === board.antennas, `${board.antennas} uplinks marked`);
  const panels = await js(`({ pipeline: document.querySelectorAll('#pipeline .slot-box').length, opposition: !!document.querySelector('#status .opposition .bar'),
                             threats: document.querySelectorAll('#countries .threat').length })`);
  check(panels.pipeline >= 4 && panels.opposition, `adversary pipeline (${panels.pipeline} boxes) and opposition bar drawn`);
  check(panels.threats > 0, `${panels.threats} threats shown with their countries`);

  // Country names, the AI boards' tracks, and no per-seat bookkeeping among the globals.
  const named = await js(`(() => {
    const v = window.aic.view;
    const label = [...document.querySelectorAll('#board .country-label')].map((g) => g.textContent).join('|');
    const strip = [...document.querySelectorAll('#countries .chead > b')].map((b) => b.textContent);
    const boards = v.seats.map((s) => s.board ? s.board.tracks.map((t) => t.steps.length) : null);
    const rows = [...document.querySelectorAll('.seat .board')].map((b) => ({
      rows: b.querySelectorAll('.track-row').length, steps: b.querySelectorAll('.step').length,
      open: b.querySelectorAll('.step.open').length, current: b.querySelectorAll('.step.current').length,
      t2: b.querySelectorAll('.mark.m-tier2').length }));
    const keys = v.globals.map((x) => x.key);
    return { names: v.countries.map((c) => c.name), label, strip, boards, rows, keys,
             raw: document.querySelectorAll('#vars .value').length };
  })()`);
  check(named.names.every((n) => n && named.label.includes(n.split(' ')[0]) && named.strip.includes(n)),
    `countries named on the board and strip (${named.names.join(', ')})`);
  check(named.boards.every((b) => b && b.length === 2) && named.rows.length === named.boards.length
    && named.rows.every((r) => r.rows === 2 && r.current === 2 && r.open >= 2 && r.steps > r.open && r.t2 > 0),
    `each seat's AI board drawn with 2 tracks (${named.rows.map((r) => `${r.open}/${r.steps} uncovered`).join(', ')})`);
  const bookkeeping = /^(ai\.track\.|threats\.paid\.|threats\.jam\.|cards\.(tier|picked|next_discount|civic_used|unpowered)\.|adversary\.killed_foundry\.|board\.(blockade|sabotage|shelter)\.)/;
  check(!named.keys.some((k) => bookkeeping.test(k)) && named.raw === 0,
    `no bookkeeping vars among the globals (${named.keys.length} globals)`);

  // Hover a hex and a token: the details card shows.
  const hover = async (sel) => {
    const pt = await js(`(() => { const r = document.querySelector('${sel}')?.getBoundingClientRect(); return r && [r.x + r.width / 2, r.y + r.height / 2]; })()`);
    if (!pt) return '';
    await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: pt[0], y: pt[1] });
    await sleep(50);
    return js(`document.getElementById('tip').hidden ? '' : document.getElementById('tip').textContent`);
  };
  check((await hover('#board .hex polygon')).includes('hex'), 'hovering a hex shows its details');
  check((await hover('#board .token .hit')).includes('×'), 'hovering a token shows its details');
  check((await hover('.value')).length > 0, 'hovering a value shows its details');
  check((await hover('#pipeline .slot-box.build')).length > 0, 'hovering an adversary slot shows its pile');
  check((await hover('.seat .board .step')).includes('step'), 'hovering a track step shows its details');
  check((await hover('.seat .board-name')).length > 0, 'hovering a board name shows the board');
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
  await shot('play-start-1366');

  // Play the whole game through the UI: random clicks on what the seat panels offer
  // (any action button: growth and draft phases offer no pass). An open choice answered
  // by a hex must highlight its hexes without a click first.
  let clicks = 0;
  let hexChoices = 0;
  let armedAtOnce = 0;
  const shots = new Set();
  while (clicks < 3000 && !(await js(`!!document.querySelector('#phase-bar .outcome')`))) {
    const st = await js(`(() => {
      const v = window.aic.view;
      const hexPick = !!v.choice && window.aic.legal.some((a) => a.type === 'choose' && a.target);
      return { hexPick, targets: document.querySelectorAll('#board .hex.target').length, prompt: document.querySelector('#prompt.choice') ? 1 : 0,
               phase: v.phase?.name ?? '', round: v.round };
    })()`);
    if (st.hexPick) {
      hexChoices++;
      if (st.targets > 0 && st.prompt) armedAtOnce++;
      if (!shots.has('choice')) { shots.add('choice'); await shot('play-choice-1366'); }
    }
    if (st.round >= 2 && st.phase.includes('cards.play') && !shots.has('mid')) { shots.add('mid'); await shot('play-mid-1366'); }
    if (st.phase === 'cards.draft' && !shots.has('draft')) { shots.add('draft'); await shot('play-draft-1366'); }
    if (st.phase === 'ai.growth' && !shots.has('growth')) { shots.add('growth'); await shot('play-growth-1366'); }
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
  check(armedAtOnce === hexChoices, `hex choices highlight their hexes at once (${armedAtOnce}/${hexChoices})`);
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
  await reload('true', 'reload');
  check((await js('window.aic.timeline.last')) === afterUndo, 'the session is restored from localStorage after a reload');

  // Bug report: the record dialog holds the game in the record format.
  await js(`document.getElementById('record').click()`);
  const rec = await js(`JSON.parse(document.getElementById('record-text').value)`);
  check(rec.format === 'aic-game-record/1' && rec.actions.length === afterUndo && !!rec.config && /^\d+$/.test(rec.seed),
    `record dialog: ${rec.format}, seed ${rec.seed}, ${rec.actions.length} actions`);
  await shot('record-dialog');
  await js(`document.getElementById('record-close').click()`);

  // Four countries (64 hexes): the default config with `countries: 4`, played a while
  // with random actions, still fits without page scroll and keeps a usable board.
  const four = await js(`(() => {
    const t = window.aic.timeline;
    const cfg = JSON.parse(t.configJson);
    cfg.countries = 4;
    cfg.name = (cfg.name ?? 'config') + ' (4 countries, e2e)';
    localStorage.setItem('aic.session', JSON.stringify({ config: JSON.stringify(cfg), seed: '7', actions: [], cursor: 0, source: { kind: 'play', file: 'e2e-4-countries' } }));
    return cfg.countries;
  })()`);
  await reload(`window.aic.view.hexes.length === ${four * 16}`, 'the 4-country game');
  for (let i = 0; i < 40; i++) await js(`document.getElementById('random-action').click()`);
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
  for (const [w, h] of [[1366, 768], [1920, 1080]]) {
    await size(w, h);
    const svg = await js(`(() => { const r = document.getElementById('board').getBoundingClientRect(); return [r.width, r.height]; })()`);
    check(await noScroll() && svg[1] > h * 0.4, `4 countries: no page scroll, board ${svg.map(Math.round).join('x')} at ${w}x${h}`);
    await shot(`four-countries-${w}x${h}`);
  }
  await size(1366, 768);

  // The shared-row threat variant shows its threats in their own box by the countries.
  if (await js(`[...document.querySelectorAll('#config option')].some((o) => o.value === 'shared_row.json')`)) {
    await js(`(() => { document.getElementById('config').value = 'shared_row.json'; document.getElementById('seed').value = '3';
                        document.getElementById('new-form').requestSubmit(); })()`);
    await until(`window.aic.timeline.source.file === 'shared_row.json'`, 'the shared-row game');
    for (let i = 0; i < 12; i++) await js(`document.getElementById('random-action').click()`);
    const row = await js(`({ pile: !!window.aic.view.piles.find((p) => p.key === 'threats.row'), shown: document.querySelectorAll('#countries .shared .threat').length })`);
    check(!row.pile || row.shown > 0, `shared threat row shown (${row.shown} threats)`);
    await shot('shared-row-1366');
  }

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
