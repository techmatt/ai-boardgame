// Shared by the card-art scripts: paths, the one browser, Codex's image tool.
import { spawn } from 'node:child_process';
import { readdir, readFile, stat, mkdir } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { chromium } from 'playwright-core';

export const TOOL = path.dirname(fileURLToPath(import.meta.url));
export const REPO = path.resolve(TOOL, '..', '..');
export const CARD = path.join(REPO, 'web', 'card');
export const ART = path.join(REPO, 'art');
export const OUT = path.join(REPO, 'out', 'cards');
export const DRIVE_ORIGINALS = 'C:\\Code\\gdrive-sync\\aicomplete\\art-originals';
// The junction when a checkout has one, else the Drive folder it points at.
export const ORIGINALS = process.env.AIC_ART_ORIGINALS ?? (existsSync(path.join(REPO, 'art-originals')) ? path.join(REPO, 'art-originals') : DRIVE_ORIGINALS);

const CODEX_HOME = process.env.CODEX_HOME ?? path.join(os.homedir(), '.codex');
const TIMEOUT_MS = 6 * 60_000;

export const readJson = async (f) => JSON.parse(await readFile(f, 'utf8'));
export const layout = () => readJson(path.join(CARD, 'layout.json'));

/** One headless browser (system Chrome), one page, for every render. */
export async function openPage(width, height) {
  const browser = await chromium.launch({ channel: 'chrome' });
  const page = await browser.newPage({ viewport: { width, height } });
  return { page, close: () => browser.close() };
}

/** Draw one image with Codex's built-in image tool; same route as wordwarriors' art:gen. */
export async function runCodex(prompt, image) {
  const workdir = path.join(os.tmpdir(), 'aic-codex');
  await mkdir(workdir, { recursive: true });
  const started = Date.now();
  // codex is an npm .cmd shim, which Node only spawns through a shell. The prompt goes on stdin.
  const child = spawn(`codex exec ${image ? `-i "${image}" ` : ''}--json --skip-git-repo-check -s read-only -C "${workdir}" -`, { shell: true, stdio: ['pipe', 'pipe', 'pipe'] });
  child.stdin.end(prompt);
  let out = '';
  let err = '';
  child.stdout.on('data', (d) => (out += d.toString('utf8')));
  child.stderr.on('data', (d) => (err += d.toString('utf8')));
  const timer = setTimeout(() => child.kill(), TIMEOUT_MS);
  const code = await new Promise((resolve) => child.on('close', resolve));
  clearTimeout(timer);
  const run = { seconds: Math.round((Date.now() - started) / 100) / 10 };
  for (const line of out.split('\n')) {
    let ev;
    try {
      ev = JSON.parse(line);
    } catch {
      continue;
    }
    if (ev.type === 'thread.started') run.thread = ev.thread_id;
    if (ev.item?.type === 'agent_message' && ev.item.text) run.message = ev.item.text.trim();
    const msg = ev.message ?? ev.error?.message;
    if ((ev.type === 'error' || ev.type === 'turn.failed') && msg) run.message = msg;
  }
  const limit = `${run.message ?? ''}\n${err}`.match(/[^\n]*(usage limit|rate limit|limit reached|try again (at|in))[^\n]*/i);
  if (limit) run.limit = limit[0].trim();
  if (run.thread) {
    const dir = path.join(CODEX_HOME, 'generated_images', run.thread);
    if (existsSync(dir)) {
      const pngs = (await readdir(dir)).filter((f) => f.endsWith('.png'));
      const timed = await Promise.all(pngs.map(async (f) => ({ f, t: (await stat(path.join(dir, f))).mtimeMs })));
      const newest = timed.sort((a, b) => b.t - a.t)[0];
      if (newest) run.image = path.join(dir, newest.f);
    }
    run.usage = await rateLimits(run.thread);
  }
  if (!run.image && code !== 0 && !run.message) run.message = `codex exited ${code}: ${err.trim().split('\n').slice(-3).join(' | ')}`;
  return run;
}

/** The allowance Codex logged for this thread (the last `rate_limits` in its session file). Nothing else is read. */
async function rateLimits(thread) {
  const find = async (dir) => {
    for (const e of await readdir(dir, { withFileTypes: true }).catch(() => [])) {
      const p = path.join(dir, e.name);
      if (e.isDirectory()) {
        const hit = await find(p);
        if (hit) return hit;
      } else if (e.name.endsWith(`${thread}.jsonl`)) return p;
    }
    return undefined;
  };
  const file = await find(path.join(CODEX_HOME, 'sessions'));
  if (!file) return undefined;
  let last;
  for (const line of (await readFile(file, 'utf8')).split('\n')) {
    if (!line.includes('"rate_limits"')) continue;
    try {
      const rl = JSON.parse(line)?.payload?.rate_limits;
      if (rl) last = { primary: rl.primary?.used_percent, secondary: rl.secondary?.used_percent };
    } catch {
      /* partial line */
    }
  }
  return last;
}
