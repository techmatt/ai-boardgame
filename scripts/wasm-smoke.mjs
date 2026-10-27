// Smoke test for the wasm package: plays one game through the JSON API.
// Usage (after `wasm-pack build crates/aic-wasm --target web`):
//   node scripts/wasm-smoke.mjs [config.json]
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const pkg = path.join(root, 'crates', 'aic-wasm', 'pkg');
const m = await import(pathToFileURL(path.join(pkg, 'aic_wasm.js')).href);
await m.default({ module_or_path: readFileSync(path.join(pkg, 'aic_wasm_bg.wasm')) });

const cfg = readFileSync(process.argv[2] ?? path.join(root, 'configs', 'baseline.json'), 'utf8');
const resolved = JSON.parse(m.resolve_config(cfg));
const board = JSON.parse(m.board(cfg));
let state = m.new_game(cfg, '18446744073709551615');
let actions = 0;
for (;;) {
  const legal = JSON.parse(m.legal_actions(state));
  if (legal.length === 0) break;
  state = m.apply(state, JSON.stringify(legal[0])).state;
  actions++;
}
const s = JSON.parse(state);

// Must equal GOLDEN_FINGERPRINT in crates/aic-core/src/tests.rs (same config,
// seed and policy), proving native and wasm builds play the same game.
const GOLDEN = 'dd7d9b197e69f946';
let h = 0xcbf29ce484222325n;
for (const b of new TextEncoder().encode(state)) {
  h = ((h ^ BigInt(b)) * 0x100000001b3n) & 0xffffffffffffffffn;
}
const fingerprint = h.toString(16).padStart(16, '0');
const golden = process.argv[2] ? true : fingerprint === GOLDEN;
console.log(`final state fingerprint ${fingerprint}${process.argv[2] ? '' : ` (golden ${GOLDEN})`}`);

let threw = false;
try {
  m.apply(state, JSON.stringify({ type: 'pass', seat: 0 }));
} catch (e) {
  threw = typeof e === 'string';
}
console.log(`engine ${m.version()}, config ${resolved.hash}, ${board.hexes.length} hexes`);
console.log(`seed ${s.seed}: ${actions} actions, outcome ${JSON.stringify(s.outcome)}`);
if (!s.outcome || s.seed !== '18446744073709551615' || !threw || !golden) {
  console.error('wasm smoke test FAILED');
  process.exit(1);
}
console.log('wasm smoke test ok');
