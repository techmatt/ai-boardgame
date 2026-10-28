// HTML for the panels around the board: the Alignment track and loss conditions, the
// adversary pipeline, the countries strip (threats), and the seat panels' contents.
// Everything comes from the generic view; known keys (adversary.*, alignment.*,
// threats.*, ai.*, economy.income.*, cards.*) get a dedicated drawing, every other
// value, card group or pile falls back to a generic chip so nothing goes unshown.

import { aiBoard, content, type CardView, type Config, type PileView, type Value, type View } from './engine';
import { escapeHtml, strong, TERRAIN, terrainFill } from './format';

const esc = escapeHtml;

/** Global values drawn by the status strip or the pipeline; the rest are generic chips. */
const DRAWN_GLOBALS = new Set([
  'adversary.pool', 'adversary.stage', 'adversary.difficulty', 'adversary.deck', 'adversary.peek',
  'threats.tier', 'alignment.undecided', 'alignment.unity', 'alignment.discord', 'alignment.unity_tips',
  'alignment.discord_tips', 'alignment.bar', 'alignment.population', 'alignment.loss_population',
  'board.start_population',
]);
/** Piles drawn by the status strip, the pipeline or the countries strip. */
const DRAWN_PILES = new Set([
  'adversary.deck', 'adversary.explore', 'adversary.build', 'adversary.ravage', 'adversary.discard', 'threats.row',
  'alignment.deck.unity', 'alignment.deck.discord', 'alignment.resolved.unity', 'alignment.resolved.discord', 'alignment.ongoing',
]);

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
  const status = c.status && !['in slot', 'draft', 'drafted', 'built', 'slotted', 'resolved', 'ongoing'].includes(c.status) ? `<em>${esc(c.status)}</em>` : '';
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

// ---------- status: alignment and losses ----------

/** A bucket of the tug of war; Unity fills toward the left, Discord toward the right. */
function bucketBar(b: Value | undefined, side: 'unity' | 'discord', name: string): string {
  if (!b) return '';
  const pct = b.max ? Math.min(100, (100 * b.value) / b.max) : 0;
  return `<span class="bar ${side}" data-value="${esc(b.key)}" data-scope="global"><span class="fill" style="width:${pct}%"></span>
    <span class="btext">${name} ${b.value}/${b.max ?? '?'}</span></span>`;
}

/** "Unity tips 1 (next at 2)". */
function tipsChip(t: Value | undefined, side: 'unity' | 'discord', name: string): string {
  if (!t) return '';
  return `<span class="tips ${side}" data-value="${esc(t.key)}" data-scope="global">${name} tips <b>${t.value}</b>${t.max !== undefined ? `<small> (next at ${t.max})</small>` : ''}</span>`;
}

export function statusHtml(v: View): string {
  const g = v.globals;
  const parts: string[] = [];
  const und = findValue(g, 'alignment.undecided');
  const unity = findValue(g, 'alignment.unity');
  const discord = findValue(g, 'alignment.discord');
  const bar = findValue(g, 'alignment.bar');
  if (und || unity || discord || bar) {
    const level = bar?.value ?? 0;
    const max = bar?.max ?? 0;
    const left = onBoard(v, ['drone', 'outpost', 'foundry']);
    const pieces = Object.entries(left).map(([k, n]) => `${n} ${k}${n === 1 ? '' : 's'}`).join(' · ');
    const undHtml = und
      ? `<span class="undecided" data-value="${esc(und.key)}" data-scope="global"><b>${und.value}</b><small>/${und.max ?? '?'}</small> undecided</span>`
      : '';
    parts.push(`<div class="alignment">
      <span class="alabel">Alignment</span>
      <span class="tug">${bucketBar(unity, 'unity', 'Unity')}${undHtml}${bucketBar(discord, 'discord', 'Discord')}</span>
      ${tipsChip(findValue(g, 'alignment.unity_tips'), 'unity', 'Unity')}${tipsChip(findValue(g, 'alignment.discord_tips'), 'discord', 'Discord')}
      ${bar ? `<span class="level" data-value="${esc(bar.key)}" data-scope="global">bar <b>${level}</b>/${max}</span><span class="win">${esc(bar.doc ?? '')}</span>` : ''}
      <span class="left" title="adversary pieces on the board">on board: ${pieces}</span>
    </div>`);
    const groups = ([['alignment.resolved.unity', 'unity'], ['alignment.resolved.discord', 'discord'], ['alignment.ongoing', 'ongoing']] as const)
      .map(([key, cls]) => [pile(v, key), cls] as const)
      .filter(([p]) => p && p.visible.length)
      .map(([p, cls]) => `<span class="agroup ${cls}" data-pile="${esc(p!.key)}"><span class="what">${esc(p!.label)}</span>${p!.visible.map((c) => cardChip(c, `pile:${p!.key}`)).join('')}</span>`);
    const decks = ['unity', 'discord'].map((s) => pile(v, `alignment.deck.${s}`)).filter((p): p is PileView => !!p)
      .map((p) => `<span class="adeck" data-pile="${esc(p.key)}">${esc(p.label)} <b>${p.count}</b></span>`);
    if (groups.length || decks.length) parts.push(`<div class="acards">${groups.join('')}${decks.join('')}</div>`);
  }
  const loss: string[] = [];
  const pop = findValue(g, 'alignment.population');
  const below = findValue(g, 'alignment.loss_population');
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
  // Unlabelled `vars` (source "engine") are folded under the piles: see `varsHtml`.
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
    const ai = seat ? seat.board?.name ?? aiBoard(config, seat.index)?.name : undefined;
    const tok = (k: string) => findValue(c.values, `tokens.${k}`)?.value ?? 0;
    const counts = `<span class="counts" title="population · adversary pieces in this country">pop ${tok('population')} · ▲${tok('drone')} ■${tok('outpost')} ⬣${tok('foundry')}${tok('corruption') ? ` · corruption ${tok('corruption')}` : ''}</span>`;
    const flags = c.values.filter((x) => !x.key.startsWith('tokens.')).map((x) => valueChip(x, `country:${c.index}`)).join('');
    const groups = c.cards.map((g) => g.key === 'threats.on'
      ? g.cards.map((x) => threatCard(x, `country:${c.index}:${g.key}`)).join('')
      : `<div class="row"><span class="what">${esc(g.label)}</span>${g.cards.map((x) => cardChip(x, `country:${c.index}:${g.key}`)).join('')}</div>`).join('');
    return `<div class="country" data-country="${c.index}" style="--tint:${strong(c.colour)};--soft:${c.colour}">
      <div class="chead"><b title="country ${c.index}">${esc(countryName(v, c.index))}</b>${seat ? ` seat ${seat.index}${ai ? ` · ${esc(ai)}` : ''}` : ''} ${counts}${flags}</div>
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
  k.startsWith('resource.') || k.startsWith('economy.income.') || ['ai.tier2', 'ai.processors', 'cards.slots'].includes(k);

/** The seat's resources with income, its board's tracks, then other values. */
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
  return `<div class="resources">${resHtml}</div>${boardHtml(v, seatIndex)}<div class="values ai">${ai.join('')}${other}</div>`;
}

/**
 * The seat's player board (`SeatView.board`): one row per track, every printed step
 * left to right. Uncovered steps are solid, covered ones dashed, the step in effect
 * is boxed; marks (`tier2`, `antimatter`, any other) sit under the step's value.
 */
export function boardHtml(v: View, seatIndex: number): string {
  const b = v.seats[seatIndex].board;
  if (!b) return '';
  const tracks = b.tracks.map((t, ti) => {
    const steps = t.steps.map((st, i) => {
      const cls = ['step', i < t.uncovered ? 'open' : 'covered', i === t.current ? 'current' : ''].filter(Boolean).join(' ');
      const marks = (st.marks ?? []).map((m) => `<span class="mark m-${esc(m.key)}" title="${esc(m.doc ?? m.key)}">${esc(m.label)}</span>`).join('');
      return `<span class="${cls}" data-track="${seatIndex}:${ti}:${i}"><b>${st.value}</b>${marks}</span>`;
    }).join('');
    return `<div class="track-row"><span class="label" title="${esc(t.label)}">${esc(t.resource || t.label)}</span>${steps}</div>`;
  }).join('');
  return `<div class="board" data-board="${seatIndex}">${tracks}</div>`;
}

/** Hover text for a track step drawn by `boardHtml` (`data-track="seat:track:step"`). */
export function trackStepDetails(v: View, ref: string): string | null {
  const [s, t, i] = ref.split(':').map(Number);
  const b = v.seats[s]?.board;
  const tr = b?.tracks[t];
  const st = tr?.steps[i];
  if (!b || !tr || !st) return null;
  const state = i === tr.current ? 'in effect' : i < tr.uncovered ? 'uncovered' : 'still covered';
  const marks = (st.marks ?? []).map((m) => `<br>${esc(m.label)}: ${esc(m.doc ?? m.key)}`).join('');
  return `<b>${esc(tr.label)}</b>, step ${i + 1} of ${tr.steps.length}: <b>${st.value}</b> (${state})${marks}<br>${tr.uncovered} uncovered · <code>${esc(b.key)}</code> · from ${esc(b.source)}`;
}

/** Hover text for a seat's board name. */
export function boardDetails(v: View, seatIndex: number): string | null {
  const b = v.seats[seatIndex]?.board;
  if (!b) return null;
  const cards = b.cards.map((c) => `${esc(c.name)} <small>(${esc(c.status)})</small>`).join('<br>');
  return `<b>${esc(b.name)}</b> <code>${esc(b.id)}</code>${b.text ? `<br>${esc(b.text)}` : ''}${b.innate ? `<hr><b>${esc(b.innate.name)}</b>${b.innate.text ? `: ${esc(b.innate.text)}` : ''}` : ''}${b.slots !== undefined ? `<br>${b.slots} slots printed` : ''}${cards ? `<hr>board cards:<br>${cards}` : ''}<br><small><code>${esc(b.key)}</code> · from ${esc(b.source)}</small>`;
}

/** "Coastal Republic", or `C<i>` if the view has no name. */
export const countryName = (v: View, i: number) => v.countries[i]?.name || `C${i}`;

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

/**
 * Game-wide `vars` tracks that no module labels (source "engine"), folded away under
 * the piles. Modules keep their per-seat and per-card bookkeeping out of the globals
 * (`ViewOut::bookkeeping` in view.rs), so in v0 this is normally empty; a new module's
 * unlabelled track shows here.
 */
export function varsHtml(v: View, open: boolean): string {
  const raw = v.globals.filter((x) => x.source === 'engine' && !DRAWN_GLOBALS.has(x.key));
  if (!raw.length) return '';
  return `<details id="vars"${open ? ' open' : ''}><summary>Unlabelled tracks (${raw.length} raw vars)</summary><div class="values">${raw.map((x) => valueChip(x, 'global', x.label, false)).join('')}</div></details>`;
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
