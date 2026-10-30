// native = wasm across the variants (020): aic-sim plays and saves games in every
// variant of the core's variant matrix, then the wasm package replays each record
// (`check_record`) and must reach the saved final fingerprint with no warnings.
// Usage (after `wasm-pack build crates/aic-wasm --target web` and a release aic-sim):
//   node scripts/wasm-parity.mjs [games per variant, default 3]
//   AIC_SIM  aic-sim binary (default target/release/aic-sim[.exe])
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readdirSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const pkg = path.join(root, 'crates', 'aic-wasm', 'pkg');
const m = await import(pathToFileURL(path.join(pkg, 'aic_wasm.js')).href);
await m.default({ module_or_path: readFileSync(path.join(pkg, 'aic_wasm_bg.wasm')) });
const sim = process.env.AIC_SIM ?? path.join(root, 'target', 'release', process.platform === 'win32' ? 'aic-sim.exe' : 'aic-sim');
const games = process.argv[2] ?? '3';

// The variant matrix of crates/aic-core/src/tests/mod.rs, played by greedy and random.
const VARIANTS = [
  [], ['threats.variant=shared_row'], ['threats.variant=off'], ['alignment.variant=still'],
  ['alignment.variant=off'], ['alignment.momentum_every=2', 'alignment.antimatter_discord=0'],
  ['adversary.difficulty=4'], ['cards.variant=free_keep', 'cards.free_keeps=1'], ['cards.variant=off'],
  ['board.clock=3'], ['countries=1'], ['countries=3'], ['countries=4'],
  ['adversary.difficulty=4', 'adversary.counter_bonus_all=1', 'threats.variant=shared_row'],
  ['countries=3', 'adversary.difficulty=4', 'alignment.variant=still'],
];
const out = mkdtempSync(path.join(tmpdir(), 'aic-parity-'));
let checked = 0;
let bad = 0;
try {
  for (const [i, sets] of VARIANTS.entries()) {
    for (const agent of ['random', 'greedy']) {
      const dir = path.join(out, `${i}-${agent}`);
      const r = spawnSync(sim, ['run', path.join(root, 'configs', 'baseline.json'), '--games', games, '--seed', '50',
        '--agent', agent, ...sets.flatMap((s) => ['--set', s]), '--save', dir, '--save-when', 'all'], { encoding: 'utf8' });
      if (r.error) throw new Error(`cannot run aic-sim at ${sim} (${r.error.code}): cargo build --release -p aic-sim, or set AIC_SIM`);
      if (r.status !== 0) {
        console.log(`FAIL aic-sim ${sets.join(' ')} ${agent}: ${(r.stderr ?? '').slice(0, 300)}`);
        bad++;
        continue;
      }
      for (const f of readdirSync(dir)) {
        const c = JSON.parse(m.check_record(readFileSync(path.join(dir, f), 'utf8')));
        checked++;
        if (!c.ok || c.warnings.length) {
          bad++;
          console.log(`FAIL ${sets.join(' ') || 'baseline'} ${agent} ${f}: ${c.error ?? c.warnings.join('; ')}`);
        }
      }
    }
  }
} finally {
  rmSync(out, { recursive: true, force: true });
}
console.log(`${checked} records over ${VARIANTS.length} variants replayed in wasm: ${bad ? `${bad} FAILED` : 'native = wasm'}`);
process.exit(bad ? 1 : 0);
