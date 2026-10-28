// HTML for the panels around the board: the opposition bar and loss conditions, the
// adversary pipeline, the countries strip (threats), and the seat panels' contents.
// Everything comes from the generic view; known keys (adversary.*, opposition.*,
// threats.*, ai.*, economy.income.*, cards.*) get a dedicated drawing, every other
// value, card group or pile falls back to a generic chip so nothing goes unshown.

import { aiBoard, content, type CardView, type Config, type PileView, type Value, type View } from './engine';
import { escapeHtml, strong, TERRAIN, terrainFill } from './format';

const esc = escapeHtml;

/** Global values drawn by the status strip or the pipeline; the rest are generic chips. */
const DRAWN_GLOBALS = new Set([
  'adversary.pool', 'adversary.stage', 'adversary.difficulty', 'adversary.deck', 'adversary.peek',
  'threats.tier', 'opposition.points', 'opposition.level', 'opposition.population',
  'opposition.loss_population', 'board.start_population',
]);
/** Piles drawn by the pipeline or the countries strip. */
const DRAWN_PILES = new Set(['adversary.deck', 'adversary.explore', 'adversary.build', 'adversary.ravage', 'adversary.discard', 'threats.row']);

const findValue = (vs: Value[], key: string) => vs.find((v) => v.key === key);
const pile = (v: View, key: string) => v.piles.find((p) => p.key === key);

/** A value chip; `near` marks a value close to its max (where that is a warning). */
export function valueChip(v: Value, scope: string, label = v.label, near = true): string {
  const val = v.max !== undefined ? `${v.value}<small>/${v.max}</small>` : `${v.value}`;
  const warn = near && v.max !== undefined && v.max > 0 && v.value >= v.max * 0.75 ? ' near' : '';
  return `<span class="value${warn}" data-value="${esc(v.key)}" data-scope="${scope}"><span class="label">${esc(label)}</span> <b>${val}</b></span>`;
}

export function cardChip(c: CardView, where: string): string {
  const cost = c.cost !== undefined ? ` <small title="cost">${c.cost}</small>` : '';
  const status = c.status && !['in slot', 'draft', 'drafted', 'built', 'slotted'].includes(c.status) ? `<em>${esc(c.status)}</em>` : '';
  return `<span class="card ${esc(c.status.replace(/\s+/g, '-'))}" data-card="${esc(c.id)}" data-where="${where}">${esc(c.name)}${cost}${status}</span>`;
}

const terrainChips = (ts: string[]) =>
  ts.map((t) => `<span class="terrain" style="background:${terrainFill(t)}">${esc(TERRAIN[t]?.label ?? t)}</span>`).join('');

/** Adversary pieces still on the board, by kind. */
function onBoard(v: View, kinds: string[]): Record<string, number> {
  const n: Record<string, number> = Object.fromEntries(kinds.map((k) => [k, 0]));
  for (const h of v.hexes) for (const t of h.tokens) if (t.kind in n) n[t.kind] += t.count;
  return n;
}

// ---------- status: opposition and losses ----------

export function statusHtml(v: View): string {
  const g = v.globals;
  const parts: string[] = [];
  const pts = findValue(g, 'opposition.points');
  const lvl = findValue(g, 'opposition.level');
  if (pts || lvl) {
    const level = lvl?.value ?? 0;
    const max = lvl?.max ?? 0;
    const pct = pts?.max ? Math.min(100, (100 * pts.value) / pts.max) : 0;
    const left = onBoard(v, ['drone', 'outpost', 'foundry']);
    const pieces = Object.entries(left).map(([k, n]) => `${n} ${k}${n === 1 ? '' : 's'}`).join(' · ');
    parts.push(`<div class="opposition" data-value="opposition.level" data-scope="global">
      <span class="olabel">Opposition</span>
      <span class="level">level <b>${level}</b>/${max}</span>
      <span class="bar" data-value="opposition.points" data-scope="global"><span class="fill" style="width:${pct}%"></span>
        <span class="btext">${pts ? `${pts.value} / ${pts.max ?? '?'} points${level < max ? ` → level ${level + 1}` : ''}` : ''}</span></span>
      <span class="win">${esc(lvl?.doc ?? '')}</span>
      <span class="left" title="adversary pieces on the board">on board: ${pieces}</span>
    </div>`);
  }
  const loss: string[] = [];
  const pop = findValue(g, 'opposition.population');
  const below = findValue(g, 'opposition.loss_population');
  if (pop) {
    const danger = below && pop.value - below.value <= 2 ? ' danger' : '';
    loss.push(`<span class="value${danger}" data-value="${pop.key}" data-scope="global"><span class="label">population</span> <b>${pop.value}</b>${below ? `<small> (lose below ${below.value})</small>` : ''}</span>`);
  }
  const pool = findValue(g, 'adversary.pool');
  if (pool) {
    const danger = pool.value <= 2 ? ' danger' : '';
    loss.push(`<span class="value${danger}" data-value="${pool.key}" data-scope="global"><span class="label">corruption pool</span> <b>${pool.value}</b><small>/${pool.max ?? '?'} (lose at 0)</small></span>`);
  }
  const deck = findValue(g, 'adversary.deck');
  if (deck) {
    const danger = deck.value <= 1 ? ' danger' : '';
    loss.push(`<span class="value${danger}" data-value="${deck.key}" data-scope="global"><span class="label">adversary deck</span> <b>${deck.value}</b><small> left (lose if empty)</small></span>`);
  }
  // Raw `vars` (source "engine": `ai.track.0.1`, `threats.paid.<id>.<c>.<r>`) are module
  // bookkeeping the panels already show through module values: see `varsHtml`.
  const other = g.filter((x) => !DRAWN_GLOBALS.has(x.key) && x.source !== 'engine').map((x) => valueChip(x, 'global'));
  if (loss.length || other.length) {
    parts.push(`<div class="losses">${loss.length ? `<span class="what">Lose if</span>${loss.join('')}` : ''}${other.length ? `<span class="sep"></span>${other.join('')}` : ''}</div>`);
  }
  return parts.join('');
}

// ---------- the adversary pipeline ----------

const phaseAt = (v: View, name: string) => v.round_order.findIndex((p) => p.name.split('+').includes(name));

function slotTerrains(c: CardView): string[] {
  const def = content.adversary?.find((a) => a.id === c.id);
  if (def) return def.terrains;
  const m = /:\s*([a-z +]+)/.exec(c.text ?? '');
  return m ? m[1].split('+').map((s) => s.trim()).filter(Boolean) : [];
}

function slotBox(p: PileView | undefined, title: string, hint: string, cls: string): string {
  if (!p) return '';
  const cards = p.visible.length
    ? p.visible.map((c) => {
      const esc8 = content.adversary?.find((a) => a.id === c.id)?.escalation;
      return `<span class="acard" data-card="${esc(c.id)}" data-where="pile:${esc(p.key)}">${terrainChips(slotTerrains(c))}${esc8 ? '<span class="escal">escalation</span>' : ''}</span>`;
    }).join('')
    : '<span class="empty">empty</span>';
  const names = p.visible.map((c) => esc(c.name)).join(', ');
  return `<div class="slot-box ${cls}" data-pile="${esc(p.key)}"><div class="sline"><span class="stitle">${title}</span>${names ? `<span class="sname">${names}</span>` : ''}</div><div class="sline">${cards}</div><div class="hint">${hint}</div></div>`;
}

export function pipelineHtml(v: View): string {
  const deck = pile(v, 'adversary.deck');
  const ex = pile(v, 'adversary.explore');
  const bu = pile(v, 'adversary.build');
  const ra = pile(v, 'adversary.ravage');
  const di = pile(v, 'adversary.discard');
  if (!deck && !ex && !bu && !ra) return '';
  const cur = v.phase?.index ?? v.round_order.length;
  const when = (phase: string) => {
    const i = phaseAt(v, phase);
    return i < 0 || cur <= i ? 'this round' : 'next round';
  };
  const g = v.globals;
  const head = ['adversary.stage', 'adversary.difficulty', 'threats.tier', 'adversary.pool', 'adversary.peek']
    .map((k) => findValue(g, k)).filter((x): x is Value => !!x)
    .map((x) => valueChip(x, 'global', x.key === 'adversary.pool' ? 'corruption pool' : x.label.replace('adversary ', ''), false))
    .join('');
  const ravageDone = phaseAt(v, 'adversary.ravage') >= 0 && cur > phaseAt(v, 'adversary.ravage');
  const buildDone = phaseAt(v, 'adversary.build') >= 0 && cur > phaseAt(v, 'adversary.build');
  const deckBox = deck
    ? `<div class="slot-box deck" data-pile="adversary.deck"><div class="sline"><span class="stitle">Deck</span><span class="big">${deck.count}</span><small>left</small></div>${deck.visible.length ? `<div class="sline peek">next: ${deck.visible.map((c) => `<span class="acard" data-card="${esc(c.id)}" data-where="pile:adversary.deck">${terrainChips(slotTerrains(c))}</span>`).join(' ')}</div>` : ''}<div class="hint">top card explores ${when('adversary.explore')}</div></div>`
    : '';
  const last = di?.visible[di.visible.length - 1];
  const discBox = di
    ? `<div class="slot-box discard" data-pile="adversary.discard"><div class="sline"><span class="stitle">Discard</span><span class="big">${di.count}</span></div>${last ? `<div class="hint">last: <span data-card="${esc(last.id)}" data-where="pile:adversary.discard">${esc(last.name)}</span></div>` : ''}</div>`
    : '';
  const arrow = '<span class="parrow">→</span>';
  const boxes = [
    deckBox,
    slotBox(ex, 'Explore', ex?.count ? 'builds next round' : 'the explored card waits here', 'explore'),
    slotBox(bu, 'Build', buildDone ? 'built · ravages next round' : `builds ${when('adversary.build')} · ravages next`, 'build'),
    slotBox(ra, 'Ravage', !ra?.count ? 'nothing to ravage' : ravageDone ? 'ravaged · discarded at round end' : `ravages ${when('adversary.ravage')}`, 'ravage'),
    discBox,
  ].filter(Boolean);
  return `<div class="phead"><b>Adversary</b>${head}<span class="note">cards slide one step each round</span></div><div class="prow">${boxes.join(arrow)}</div>`;
}

// ---------- countries strip: threats ----------

/** "1/2 energy, 0/3 credits" → bars. */
function progressBars(status: string): string {
  return status.split(',').map((part) => {
    const m = /(\d+)\s*\/\s*(\d+)\s*(.*)/.exec(part.trim());
    if (!m) return part.trim() ? `<span class="prog">${esc(part.trim())}</span>` : '';
    const [have, need] = [Number(m[1]), Number(m[2])];
    const pct = need ? Math.min(100, (100 * have) / need) : 100;
    return `<span class="prog${have >= need ? ' done' : ''}"><span class="fill" style="width:${pct}%"></span><span class="ptext">${have}/${need} ${esc(m[3])}</span></span>`;
  }).join('');
}

function threatCard(c: CardView, where: string): string {
  const effect = (c.text ?? '').replace(/\s*Counter:.*$/, '');
  return `<div class="threat" data-card="${esc(c.id)}" data-where="${where}"><span class="tname"><b>${esc(c.name)}</b>${progressBars(c.status)}</span><span class="ttext">${esc(effect)}</span></div>`;
}

export function countriesHtml(v: View, config: Config): string {
  const boxes = v.countries.map((c) => {
    const seat = v.seats.find((s) => s.country === c.index);
    const ai = seat ? aiBoard(config, seat.index)?.name : undefined;
    const tok = (k: string) => findValue(c.values, `tokens.${k}`)?.value ?? 0;
    const counts = `<span class="counts" title="population · adversary pieces in this country">pop ${tok('population')} · ▲${tok('drone')} ■${tok('outpost')} ⬣${tok('foundry')}${tok('corruption') ? ` · corruption ${tok('corruption')}` : ''}</span>`;
    const flags = c.values.filter((x) => !x.key.startsWith('tokens.')).map((x) => valueChip(x, `country:${c.index}`)).join('');
    const groups = c.cards.map((g) => g.key === 'threats.on'
      ? g.cards.map((x) => threatCard(x, `country:${c.index}:${g.key}`)).join('')
      : `<div class="row"><span class="what">${esc(g.label)}</span>${g.cards.map((x) => cardChip(x, `country:${c.index}:${g.key}`)).join('')}</div>`).join('');
    return `<div class="country" data-country="${c.index}" style="--tint:${strong(c.colour)};--soft:${c.colour}">
      <div class="chead"><b>C${c.index}</b>${seat ? ` seat ${seat.index}${ai ? ` · ${esc(ai)}` : ''}` : ''} ${counts}${flags}</div>
      ${groups || '<div class="none">no threat</div>'}</div>`;
  });
  const row = pile(v, 'threats.row');
  if (row) {
    boxes.push(`<div class="country shared" style="--tint:#555;--soft:#eee"><div class="chead"><b>${esc(row.label)}</b></div>${row.visible.map((x) => threatCard(x, `pile:${row.key}`)).join('') || '<div class="none">none</div>'}</div>`);
  }
  return boxes.join('');
}

/** The board's key: terrain fills and the uplink mark. */
export function legendHtml(): string {
  const t = Object.values(TERRAIN).map((x) => `<span class="terrain" style="background:${x.fill}">${x.label}</span>`).join('');
  const up = `<svg viewBox="-0.17 -0.17 0.34 0.34" width="1.2em" height="1.2em" style="vertical-align:middle"><g class="uplink">
    <circle r="0.15" class="badge"/><path d="M0,0.1 V-0.04 M-0.06,0.1 H0.06" class="mast"/><circle cy="-0.05" r="0.025" class="tip"/>
    <path d="M-0.06,-0.11 A0.08,0.08 0 0 1 0.06,-0.11 M-0.09,-0.02 A0.11,0.11 0 0 1 -0.09,-0.1 M0.09,-0.02 A0.11,0.11 0 0 0 0.09,-0.1" class="waves"/></g></svg>`;
  return `${t}<span class="lg">${up} uplink</span>`;
}

// ---------- seat panels ----------

const SEAT_DRAWN = (k: string) =>
  k.startsWith('resource.') || k.startsWith('economy.income.') || ['ai.board', 'ai.track0', 'ai.track1', 'ai.tier2', 'ai.processors', 'cards.slots'].includes(k);

/** The seat's resources with income, its AI board and tracks, then other values. */
export function seatValuesHtml(v: View, seatIndex: number): string {
  const s = v.seats[seatIndex];
  const scope = `seat:${seatIndex}`;
  const res = s.values.filter((x) => x.key.startsWith('resource.'));
  const names = new Set(res.map((x) => x.key.slice('resource.'.length)));
  for (const x of s.values) {
    const r = x.key.startsWith('economy.income.') ? x.key.slice('economy.income.'.length) : null;
    if (r && x.value !== 0) names.add(r);
  }
  const order = ['credits', 'compute', 'energy', 'insight', 'antimatter'];
  const sorted = [...names].sort((a, b) => (order.indexOf(a) + 99) % 99 - (order.indexOf(b) + 99) % 99 || a.localeCompare(b));
  const resHtml = sorted.map((r) => {
    const have = findValue(s.values, `resource.${r}`);
    const inc = findValue(s.values, `economy.income.${r}`);
    const incHtml = inc ? `<span class="inc" data-value="${inc.key}" data-scope="${scope}">${inc.value >= 0 ? '+' : ''}${inc.value}</span>` : '';
    return `<span class="res r-${esc(r)}" data-value="resource.${esc(r)}" data-scope="${scope}"><span class="label">${esc(r)}</span> <b>${have?.value ?? 0}</b>${incHtml}</span>`;
  }).join('');

  const ai: string[] = [];
  for (const t of ['ai.track0', 'ai.track1']) {
    const x = findValue(s.values, t);
    if (!x) continue;
    const under = /under processors:\s*(.*)$/.exec(x.doc ?? '')?.[1] ?? '';
    const next = under && under !== 'none' ? under.split(',').map((u) => `<i>${esc(u.trim())}</i>`).join('') : '<i>—</i>';
    ai.push(`<span class="track" data-value="${t}" data-scope="${scope}"><span class="label">${esc(x.label.replace(' track', ''))}</span> <b>${x.value}</b><span class="under" title="values still under processors">${next}</span></span>`);
  }
  const t2 = findValue(s.values, 'ai.tier2');
  if (t2) ai.push(`<span class="value t2 ${t2.value ? 'on' : 'off'}" data-value="ai.tier2" data-scope="${scope}">tier 2 ${t2.value ? 'unlocked' : 'locked'}</span>`);
  const pr = findValue(s.values, 'ai.processors');
  if (pr) ai.push(valueChip(pr, scope));
  const slots = findValue(s.values, 'cards.slots');
  const built = s.cards.find((g) => g.key === 'cards.built');
  if (slots) {
    const used = /(\d+)\s*\/\s*\d+/.exec(built?.label ?? '')?.[1] ?? String(built?.cards.length ?? 0);
    ai.push(`<span class="value" data-value="cards.slots" data-scope="${scope}"><span class="label">slots</span> <b>${used}<small>/${slots.value}</small></b></span>`);
  }
  const other = s.values.filter((x) => !SEAT_DRAWN(x.key)).map((x) => valueChip(x, scope)).join('');
  return `<div class="resources">${resHtml}</div><div class="values ai">${ai.join('')}${other}</div>`;
}

const GROUP_LABEL: Record<string, string> = {
  'cards.hand': 'hand',
  'cards.packet': 'draft hand',
  'cards.drafted': 'drafted',
};

export function seatCardsHtml(v: View, seatIndex: number): string {
  const s = v.seats[seatIndex];
  const order = ['cards.packet', 'cards.drafted', 'cards.hand', 'cards.built'];
  const groups = [...s.cards].sort((a, b) => ((order.indexOf(a.key) + 99) % 99) - ((order.indexOf(b.key) + 99) % 99));
  return groups.map((g) => {
    const label = GROUP_LABEL[g.key] ?? g.label;
    const cls = g.key.replace(/\W/g, '-');
    const body = g.cards.length ? g.cards.map((c) => cardChip(c, `seat:${seatIndex}:${g.key}`)).join('') : '<span class="slot">—</span>';
    return `<div class="row ${cls}"><span class="what" title="${esc(g.key)}">${esc(label)}</span>${body}</div>`;
  }).join('');
}

/** Raw `vars` tracks no module shows itself, folded away under the piles. */
export function varsHtml(v: View, open: boolean): string {
  const raw = v.globals.filter((x) => x.source === 'engine' && !DRAWN_GLOBALS.has(x.key));
  if (!raw.length) return '';
  return `<details id="vars"${open ? ' open' : ''}><summary>Bookkeeping (${raw.length} raw vars)</summary><div class="values">${raw.map((x) => valueChip(x, 'global', x.label, false)).join('')}</div></details>`;
}

// ---------- other piles ----------

export function pilesHtml(v: View): string {
  const rest = v.piles.filter((p) => !DRAWN_PILES.has(p.key));
  if (!rest.length) return '';
  return `<h3>Piles</h3><div class="pile-list">${rest.map((p) => {
    const shown = p.visible.slice(-3).map((c) => cardChip(c, `pile:${p.key}`)).join('');
    const more = p.visible.length > 3 ? `<small>+${p.visible.length - 3}</small>` : '';
    return `<div class="row pile" data-pile="${esc(p.key)}"><span class="what" title="${esc(p.key)}">${esc(p.label)}${p.owner !== null ? ` (seat ${p.owner})` : ''}: <b>${p.count}</b></span>${more}${shown}</div>`;
  }).join('')}</div>`;
}
