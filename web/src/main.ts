// The AI Complete viewer: play a game (any seat, any order) or step through a saved
// one. Renders the engine's generic view and offers exactly the engine's
// `legal_actions`. No game rules live here.

import { renderBoard } from './board';
import {
  aiBoard, checkRecord, legalActions, load, recordText, resolveConfig, rulesRevision, version, view as engineView,
  type Action, type CardView, type GameEvent, type View,
} from './engine';
import {
  actionLabel, choiceKey, contentIds, escapeHtml, eventText, hexKey, hexesIn, isHex, kindName, strong, tokenColour, tokenName,
  type LabelContext,
} from './format';
import {
  boardDetails, countriesHtml, countryName, legendHtml, pilesHtml, pipelineHtml, seatCardsHtml, seatValuesHtml,
  statusHtml, trackStepDetails, varsHtml,
} from './panels';
import { Timeline, parseSeed, randomSeed, readRecord, storageGet, storageSet } from './session';
import './style.css';

const $ = <T extends HTMLElement = HTMLElement>(id: string) => document.getElementById(id) as T;

const ui = {
  version: $('version'),
  newForm: $<HTMLFormElement>('new-form'),
  config: $<HTMLSelectElement>('config'),
  seed: $<HTMLInputElement>('seed'),
  error: $('error'),
  source: $('source'),
  phaseBar: $('phase-bar'),
  board: document.getElementById('board') as unknown as SVGSVGElement,
  prompt: $('prompt'),
  status: $('status'),
  pipeline: $('pipeline'),
  countries: $('countries'),
  seats: $('seats'),
  piles: $('piles'),
  log: $('tab-log'),
  state: $('tab-state'),
  view: $('tab-view'),
  configTab: $('tab-config'),
  scrub: $<HTMLInputElement>('scrub'),
  step: $('step'),
  play: $<HTMLButtonElement>('play'),
  speed: $<HTMLSelectElement>('speed'),
  tip: $('tip'),
  recordDialog: $<HTMLDialogElement>('record-dialog'),
  recordText: $<HTMLTextAreaElement>('record-text'),
  recordNote: $<HTMLInputElement>('record-note'),
  recordMsg: $('record-msg'),
  recordFile: $<HTMLInputElement>('record-file'),
};

/** Every file in configs/, read at build time (the dev server reloads on changes). */
const CONFIGS: Record<string, string> = Object.fromEntries(
  Object.entries(import.meta.glob('../../configs/*.json', { query: '?raw', import: 'default', eager: true }))
    .map(([path, text]) => [path.split('/').pop() as string, text as string])
    .sort(([a], [b]) => a.localeCompare(b)),
);

let tl: Timeline;
let shown: View;
let legal: Action[] = [];
/** The choice (actions differing only in target hex) waiting for a hex click. */
let armed: { seat: number; key: string } | null = null;
let autoplay: number | null = null;
let activeTab = 'log';
/** Whether the raw-vars section under the piles is unfolded. */
let varsOpen = false;

// ---------- errors ----------

function showError(msg: string | null) {
  ui.error.hidden = !msg;
  ui.error.textContent = msg ?? '';
}

function guard(f: () => void) {
  try {
    f();
  } catch (e) {
    showError(e instanceof Error ? e.message : String(e));
  }
}

// ---------- sessions ----------

function setTimeline(t: Timeline) {
  stopAutoplay();
  tl = t;
  armed = null;
  showError(t.replayError);
  tl.save();
  render();
}

function newGame(file: string, seed: string) {
  const text = CONFIGS[file];
  if (text === undefined) throw new Error(`no config ${file}`);
  setTimeline(Timeline.start(text, seed, { kind: 'play', file }));
}

function loadRecordText(text: string, file: string) {
  const rec = readRecord(text);
  const t = Timeline.fromRecord(rec, file);
  setTimeline(t);
  // A record dropped or opened under the dialog: the dialog's old text would be stale.
  if (ui.recordDialog.open) ui.recordDialog.close();
  const check = checkRecord(text);
  const warn = [...(check.warnings ?? []), ...(check.error ? [check.error] : [])];
  if (warn.length && !t.replayError) showError(`record loaded with warnings: ${warn.join('; ')}`);
  // A record that stops replaying: say first if it is from another rules revision (020:
  // only the rejected action showed).
  else if (t.replayError && rec.rules_revision && rec.rules_revision !== rulesRevision)
    showError(`this record is from rules revision ${rec.rules_revision} and this engine plays revision ${rulesRevision}: ${t.replayError}`);
}

function act(a: Action) {
  const later = tl.last - tl.cursor;
  if (later > 0 && !confirm(`Acting here drops the ${later} later action(s). Go on?`)) return;
  armed = null;
  // Acting from a focused action button (the keyboard): keep focus in that seat's
  // buttons across the re-render (020: it fell back to the page body every time).
  const fromButton = !!(document.activeElement as Element | null)?.closest?.('#seats .actions');
  guard(() => {
    tl.act(a);
    showError(null);
  });
  tl.save();
  render();
  if (fromButton) {
    const next = (ui.seats.querySelector(`.seat[data-seat="${a.seat}"] .actions button`) ?? ui.seats.querySelector('.actions button')) as HTMLElement | null;
    next?.focus();
  }
}

function seek(i: number) {
  if (!tl) return; // still loading
  tl.seek(i);
  armed = null;
  tl.save();
  render();
}

// ---------- rendering ----------

function render() {
  const f = tl.frame;
  shown = engineView(f.snap);
  legal = legalActions(f.snap);
  if (armed && !legal.some((a) => a.seat === armed!.seat && choiceKey(a) === armed!.key)) armed = null;
  // An open choice answered by picking a hex arms that pick at once.
  if (!armed && shown.choice) {
    const pick = legal.find((a) => a.seat === shown.choice!.seat && a.type === 'choose' && isHex(a.target));
    if (pick) armed = { seat: pick.seat, key: choiceKey(pick) };
  }

  const src = tl.source;
  const cfgName = f.snap.state.config.name || 'config';
  ui.source.innerHTML = src.kind === 'record'
    ? `<b>Watching</b> ${escapeHtml(src.file)} · ${escapeHtml(cfgName)} · seed ${tl.seed}${src.note ? ` · <i>${escapeHtml(src.note)}</i>` : ''}`
    : `<b>Playing</b> ${escapeHtml(src.file)} · ${escapeHtml(cfgName)} · seed ${tl.seed}`;
  // A long record note is cut off with an ellipsis: the hover has it whole.
  ui.source.title = ui.source.textContent ?? '';

  renderPhase();
  const targets = new Set<string>();
  if (armed) {
    for (const a of legal) if (a.seat === armed.seat && choiceKey(a) === armed.key && isHex(a.target)) targets.add(hexKey(a.target));
  }
  const changed = new Set<string>();
  // Setup touches every hex: only mark what an action changed.
  if (f.action) {
    for (const e of f.events) for (const h of hexesIn(e)) changed.add(hexKey(h));
    for (const h of hexesIn(f.action)) changed.add(hexKey(h));
  }
  renderBoard(ui.board, shown, { targets, changed });
  renderPrompt(targets.size);
  ui.status.innerHTML = statusHtml(shown);
  ui.pipeline.innerHTML = pipelineHtml(shown);
  ui.countries.innerHTML = countriesHtml(shown, f.snap.state.config);
  renderSeats();
  renderPiles();
  renderLog();
  renderTimeline();
  renderTab();
}

/** "cards.play+threats.counter" → "play + counter". */
const shortPhase = (name: string) => name.split('+').map((x) => x.split('.').pop()).join(' + ');

/** Who the game waits for, and the open choice's question, above the board. */
function renderPrompt(targets: number) {
  const v = shown;
  const seatName = (s: number) => {
    const ai = v.seats[s]?.board?.name ?? aiBoard(tl.frame.snap.state.config, s)?.name;
    return `Seat ${s}${ai ? ` (${escapeHtml(ai)})` : ''}`;
  };
  ui.prompt.className = '';
  if (v.outcome) {
    ui.prompt.innerHTML = '';
    return;
  }
  if (v.choice) {
    ui.prompt.className = 'choice';
    const how = targets ? `click one of the ${targets} highlighted hexes` : `answer in ${seatName(v.choice.seat)}'s panel`;
    ui.prompt.innerHTML = `<b>${seatName(v.choice.seat)} decides:</b> ${escapeHtml(contentIds(v.choice.prompt.replace(/^Seat \d+:\s*/, '')))} <span class="how">— ${how}</span>`;
    return;
  }
  const acting = [...new Set(legal.map((a) => a.seat))];
  ui.prompt.innerHTML = v.phase && acting.length
    ? `<b>${escapeHtml(shortPhase(v.phase.name))}:</b> ${acting.map(seatName).join(', ')} to act`
    : '';
}

function labels(): LabelContext {
  return { config: tl.frame.snap.state.config, view: shown };
}

function renderPhase() {
  const v = shown;
  const order = v.round_order
    .map((p, i) => `<span class="phase ${p.kind}${v.phase?.index === i ? ' now' : ''}" title="${escapeHtml(p.name)} (${p.kind})">${escapeHtml(shortPhase(p.name))}</span>`)
    .join('<span class="arrow">›</span>');
  let status: string;
  if (v.outcome) {
    const o = v.outcome;
    // The sentence first, the reason code (what `--save-when` takes) after it.
    status = `<span class="outcome ${o.won ? 'won' : 'lost'}">${o.won ? 'WON' : 'LOST'} in round ${o.round}: ${escapeHtml(o.detail)} <code>${escapeHtml(o.reason)}</code></span>`;
  } else status = ''; // who is to act is in #prompt; a war round is marked on the adversary row
  ui.phaseBar.innerHTML = `<b>Round ${v.round}</b> <span class="order">${order}</span> ${status}`;
}

function renderSeats() {
  const v = shown;
  const config = tl.frame.snap.state.config;
  ui.seats.innerHTML = '';
  for (const seat of v.seats) {
    const country = v.countries[seat.country];
    const mine = legal.filter((a) => a.seat === seat.index);
    const waiting = v.phase?.waiting.includes(seat.index);
    const choosing = v.choice?.seat === seat.index;
    const box = document.createElement('div');
    box.className = `seat${seat.passed && !choosing ? ' passed' : ''}${mine.length ? ' active' : ''}`;
    box.dataset.seat = String(seat.index);
    box.style.setProperty('--tint', strong(country?.colour ?? '#cccccc'));
    const status = v.outcome ? '' : choosing ? 'deciding' : mine.length ? 'to act' : seat.passed ? 'passed' : waiting ? 'waiting' : '';
    // The board the engine shows for this seat; content's board by config as a fallback.
    const fallback = aiBoard(config, seat.index);
    const ai = seat.board ?? (fallback && { name: fallback.name, text: fallback.text, innate: fallback.innate });
    box.innerHTML = `
      <h3><span class="swatch"></span>Seat ${seat.index}${ai ? ` · <span class="board-name" data-board-seat="${seat.index}">${escapeHtml(ai.name)}</span>` : ''} <small title="country ${seat.country}">${escapeHtml(countryName(v, seat.country))}</small><span class="status">${status}</span></h3>
      ${ai?.innate ? `<div class="innate" title="${escapeHtml(ai.text ?? '')}"><b>${escapeHtml(ai.innate.name)}:</b> ${escapeHtml(ai.innate.text ?? '')}</div>` : ''}
      ${seatValuesHtml(v, seat.index)}
      ${seatCardsHtml(v, seat.index)}`;
    if (mine.length) box.append(actionButtons(mine));
    ui.seats.append(box);
  }
  const passers = (v.phase?.waiting ?? []).filter((s) => legal.some((a) => a.seat === s && a.type === 'pass'));
  if (!v.outcome && !v.choice && passers.length > 1) {
    const all = document.createElement('button');
    all.type = 'button';
    all.className = 'pass-all';
    all.textContent = `Pass every seat still to act (${passers.join(', ')})`;
    all.onclick = () => passAll();
    ui.seats.append(all);
  }
}

/**
 * One button per choice. Only `choose` actions have a target hex, and `render` arms
 * such a choice at once: it shows as a count here and is answered on the board.
 */
function actionButtons(mine: Action[]): HTMLElement {
  const row = document.createElement('div');
  row.className = 'actions';
  const groups = new Map<string, Action[]>();
  const sorted = [...mine].sort((a, b) => typeRank(a.type) - typeRank(b.type));
  for (const a of sorted) {
    const k = choiceKey(a);
    groups.set(k, [...(groups.get(k) ?? []), a]);
  }
  const cx = labels();
  for (const acts of groups.values()) {
    if (acts.every((a) => isHex(a.target))) {
      const count = document.createElement('span');
      count.className = 'hex-pick';
      count.textContent = `pick a hex ▸ (${acts.length})`;
      count.dataset.hexes = acts.map((a) => hexKey(a.target!)).join(' ');
      count.dataset.action = JSON.stringify(acts[0]);
      row.append(count);
      // Two answers on one hex (a sacrifice: the ambassador or the presence there) can't
      // both be a click on it: each gets its own button (020: only the first was reachable).
      const perHex = new Map<string, Action[]>();
      for (const a of acts) perHex.set(hexKey(a.target!), [...(perHex.get(hexKey(a.target!)) ?? []), a]);
      for (const same of perHex.values()) {
        if (same.length < 2) continue;
        for (const a of same) {
          const one = document.createElement('button');
          one.type = 'button';
          one.textContent = actionLabel(a, false, false, cx);
          one.className = `a-${a.type}`;
          one.dataset.action = JSON.stringify(a);
          one.dataset.hexes = hexKey(a.target!);
          one.onclick = () => act(a);
          row.append(one);
        }
      }
      continue;
    }
    // Anything else is one button per action, labelled in full.
    for (const a of acts) {
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = actionLabel(a, false, true, cx);
      b.className = `a-${a.type}${a.type === 'pass' ? ' pass' : ''}`;
      b.dataset.action = JSON.stringify(a);
      b.onclick = () => act(a);
      row.append(b);
    }
  }
  return row;
}

/** Button order: answers and decisions first, standard actions, threat counters and gifts next, selling a card and pass last. */
const TYPE_ORDER = ['choose', 'growth', 'tier', 'draft', 'keep', 'play', 'use', 'draw', 'standard', 'pay', 'withdraw', 'give', 'antimatter', 'sell'];
const typeRank = (t: string) => (t === 'pass' ? 99 : TYPE_ORDER.indexOf(t) < 0 ? 50 : TYPE_ORDER.indexOf(t));

function passAll() {
  const later = tl.last - tl.cursor;
  if (later > 0 && !confirm(`Acting here drops the ${later} later action(s). Go on?`)) return;
  guard(() => {
    for (const s of shown.phase?.waiting ?? []) {
      const p = legalActions(tl.frame.snap).find((a) => a.seat === s && a.type === 'pass');
      if (p) tl.act(p);
    }
  });
  armed = null;
  tl.save();
  render();
}

function renderPiles() {
  ui.piles.innerHTML = pilesHtml(shown) + varsHtml(shown, varsOpen);
  ui.piles.querySelector('#vars')?.addEventListener('toggle', (e) => {
    varsOpen = (e.target as HTMLDetailsElement).open;
  });
}

/** The timeline and frames whose entries the log shows (see `renderLog`). */
let logBuilt: { tl: Timeline; frames: unknown[] } | null = null;

const evItem = (e: GameEvent) => {
  const hs = hexesIn(e).map(hexKey).join(' ');
  return `<li class="ev ${escapeHtml(e.type)}"${hs ? ` data-hexes="${hs}"` : ''}>${escapeHtml(eventText(e))}</li>`;
};

/**
 * A step's events by phase (022, D7): the engine marks each phase it enters with a
 * `phase_started` event and each new round with `round_started`; the events before the
 * first mark belong to the phase the action was taken in (`before`, the step's phase).
 */
function phaseGroups(events: GameEvent[], before: string): { label: string; round: boolean; events: GameEvent[] }[] {
  const groups = [{ label: before, round: false, events: [] as GameEvent[] }];
  for (const e of events) {
    if (e.type === 'phase_started') groups.push({ label: shortPhase(String(e.phase)), round: false, events: [] });
    else if (e.type === 'round_started') groups.push({ label: `Round ${e.round}`, round: true, events: [] });
    else groups[groups.length - 1].events.push(e);
  }
  if (!groups[0].events.length) groups.shift();
  return groups;
}

function logEntry(f: Timeline['frames'][number], i: number, prev: Timeline['frames'][number] | undefined): string {
  const head = f.action ? `#${i} ${escapeHtml(actionLabel(f.action, true, true, { config: f.snap.state.config }))}` : '#0 setup';
  const hexes = [...(f.action ? hexesIn(f.action) : []), ...f.events.flatMap(hexesIn)].map(hexKey);
  const ps = prev?.snap.state;
  const before = ps ? shortPhase(ps.config.round[ps.phase] ?? 'setup') : 'setup';
  const groups = phaseGroups(f.events, before);
  let body: string;
  // A step that stays in one phase lists its events as they are; one that runs through
  // phases (the adversary's turn after the last pass) gets a folded group per phase,
  // whose heading outlines the hexes that phase changed when hovered.
  if (groups.length <= 1) body = f.events.length ? `<ul>${f.events.map(evItem).join('')}</ul>` : '';
  else {
    body = groups.map((g) => {
      if (!g.events.length) return `<div class="pgroup empty${g.round ? ' round' : ''}">${escapeHtml(g.label)}</div>`;
      const hs = [...new Set(g.events.flatMap(hexesIn).map(hexKey))].join(' ');
      const n = g.events.length;
      return `<details class="pgroup${g.round ? ' round' : ''}"${hs ? ` data-hexes="${hs}"` : ''}><summary>${escapeHtml(g.label)} <small>${n} event${n === 1 ? '' : 's'}${hs ? ` · ${hs.split(' ').length} hex${hs.split(' ').length === 1 ? '' : 'es'}` : ''}</small></summary><ul>${g.events.map(evItem).join('')}</ul></details>`;
    }).join('');
  }
  return `<div class="entry" data-step="${i}" data-hexes="${[...new Set(hexes)].join(' ')}"><div class="head">${head}</div>${body}</div>`;
}

/**
 * The log keeps the entries of the frames it already shows and only adds or drops
 * the ones that changed (020: rebuilding a long game's log on every step or action
 * cost ~130 ms a step at 400 actions, ~30 ms an action at 1,000).
 */
function renderLog() {
  if (logBuilt?.tl !== tl) {
    ui.log.innerHTML = '';
    logBuilt = { tl, frames: [] };
  }
  const had = logBuilt.frames;
  let keep = 0;
  while (keep < had.length && keep < tl.frames.length && had[keep] === tl.frames[keep]) keep++;
  while (ui.log.children.length > keep) ui.log.lastElementChild!.remove();
  if (tl.frames.length > keep) {
    ui.log.insertAdjacentHTML('beforeend', tl.frames.slice(keep).map((f, j) => logEntry(f, keep + j, tl.frames[keep + j - 1])).join(''));
  }
  logBuilt.frames = [...tl.frames];
  const entries = ui.log.children;
  for (let i = 0; i < entries.length; i++) {
    entries[i].classList.toggle('now', i === tl.cursor);
    entries[i].classList.toggle('future', i > tl.cursor);
  }
  entries[tl.cursor]?.scrollIntoView({ block: 'nearest' });
}

function renderTimeline() {
  ui.scrub.max = String(tl.last);
  ui.scrub.value = String(tl.cursor);
  const a = tl.frame.action;
  ui.step.textContent = `step ${tl.cursor} / ${tl.last}${a ? ` · ${actionLabel(a, true, true, { config: tl.frame.snap.state.config })}` : ' · setup'}`;
  ui.play.textContent = autoplay !== null ? '⏸ pause' : '▶ play';
}

function renderTab() {
  for (const t of ['log', 'state', 'view', 'config']) {
    $(`tab-${t}`).hidden = t !== activeTab;
    document.querySelector(`#tabs [data-tab="${t}"]`)?.classList.toggle('on', t === activeTab);
  }
  if (!tl) return; // still loading (020: Raw state and Config threw)
  if (activeTab === 'state') ui.state.textContent = JSON.stringify(tl.frame.snap.state, null, 1);
  if (activeTab === 'view') ui.view.textContent = JSON.stringify(shown, null, 1);
  if (activeTab === 'config') {
    let hash = '?';
    try {
      hash = resolveConfig(tl.configJson).hash;
    } catch (e) {
      hash = String(e);
    }
    ui.configTab.textContent = `config hash ${hash}\n${JSON.stringify(tl.frame.snap.state.config, null, 1)}`;
  }
}

// ---------- hover details ----------

function details(t: HTMLElement | SVGElement): string | null {
  const d = (t as HTMLElement).dataset;
  if (d.token !== undefined && d.hex) {
    const h = shown.hexes.find((x) => hexKey(x.hex) === d.hex);
    const tok = h?.tokens.find((x) => x.kind === d.token);
    return `<b>${escapeHtml(tokenName(d.token))}</b> × ${tok?.count ?? 0}<br><code>${escapeHtml(d.token)}</code> on ${escapeHtml(countryName(shown, h?.country ?? -1))} #${h?.number} (${escapeHtml(kindName(h?.kind ?? ''))}, hex ${d.hex})`;
  }
  if (d.hex && d.index !== undefined) {
    const h = shown.hexes[Number(d.index)];
    // A seat's piece in its seat's colour, as the board draws it (020: grey).
    const dot = (k: string) => {
      const seat = /:(\d+)$/.exec(k)?.[1];
      const c = seat !== undefined ? shown.countries[shown.seats[Number(seat)]?.country ?? -1]?.colour : undefined;
      return c ? strong(c) : tokenColour(k);
    };
    const toks = h.tokens.map((x) => `<span class="dot" style="background:${dot(x.kind)}"></span>${escapeHtml(tokenName(x.kind))} × ${x.count}`).join('<br>') || 'no tokens';
    const here = legal.filter((a) => isHex(a.target) && hexKey(a.target) === d.hex).map((a) => escapeHtml(actionLabel(a, true, true, labels())));
    const facing = h.facing !== undefined ? ` · <b>front</b> facing ${escapeHtml(countryName(shown, h.facing))}` : '';
    const arrows = (shown.arrows ?? []).filter((a) => a.from === h.index || a.to === h.index).map((a) => escapeHtml(contentIds(a.doc)));
    return `<b>${escapeHtml(countryName(shown, h.country))} #${h.number}</b> · ${escapeHtml(kindName(h.kind))}${facing}<br>hex ${d.hex} · index ${h.index} · piece position ${h.piece_pos.join(',')} · ${h.neighbours.length} neighbours<hr>${toks}${arrows.length ? `<hr>${arrows.join('<br>')}` : ''}${here.length ? `<hr>legal here:<br>${here.join('<br>')}` : ''}`;
  }
  if (d.pile) {
    const p = shown.piles.find((x) => x.key === d.pile);
    if (!p || d.card) return null;
    return `<b>${escapeHtml(p.label)}</b>: ${p.count} card${p.count === 1 ? '' : 's'}<br><code>${escapeHtml(p.key)}</code> · from ${escapeHtml(p.source)}${p.visible.length ? `<hr>${p.visible.map((c) => `${escapeHtml(c.name)}${c.text ? ` <small>${escapeHtml(c.text)}</small>` : ''}`).join('<br>')}` : ''}`;
  }
  if (d.value) {
    const [scope, idx] = (d.scope ?? 'global').split(':');
    const list = scope === 'seat' ? shown.seats[Number(idx)].values : scope === 'country' ? shown.countries[Number(idx)].values : shown.globals;
    const v = list.find((x) => x.key === d.value);
    // A resource another seat has but this one never held (drawn as 0 since 020).
    if (!v && d.value.startsWith('resource.')) return `<b>${escapeHtml(d.value.slice('resource.'.length).replace(/_/g, ' '))}</b>: 0<br><code>${escapeHtml(d.value)}</code> · ${scope}${idx ? ` ${idx}` : ''} (none yet)`;
    if (!v) return null;
    return `<b>${escapeHtml(v.label)}</b>: ${v.value}${v.max !== undefined ? ` / ${v.max}` : ''}<br><code>${escapeHtml(v.key)}</code> · ${scope}${idx ? ` ${idx}` : ''} · from ${escapeHtml(v.source)}${v.doc ? `<br>${escapeHtml(v.doc)}` : ''}`;
  }
  if (d.card) {
    // The same card id can sit with several seats, each with its own status: look
    // in the place the chip was drawn from.
    const [kind, a, ...rest] = (d.where ?? '').split(':');
    const key = rest.join(':');
    const pool: (CardView | null)[] =
      kind === 'seat' ? shown.seats[Number(a)]?.cards.find((g) => g.key === key)?.cards ?? []
      : kind === 'country' ? shown.countries[Number(a)]?.cards.find((g) => g.key === key)?.cards ?? []
      : kind === 'pile' ? shown.piles.find((p) => p.key === [a, ...rest].join(':'))?.visible ?? []
      : [];
    const c = pool.find((x) => x?.id === d.card);
    if (!c) return null;
    return `<b>${escapeHtml(c.name)}</b> <code>${escapeHtml(c.id)}</code>${c.cost !== undefined ? ` · cost ${c.cost}` : ''}<br>${escapeHtml(c.status)}${c.text ? `<br>${escapeHtml(c.text)}` : ''}<br><small>${escapeHtml(d.where ?? '')}</small>`;
  }
  if (d.track) return trackStepDetails(shown, d.track);
  if (d.boardSeat !== undefined) return boardDetails(shown, Number(d.boardSeat));
  if (d.action) return `<code>${escapeHtml(d.action)}</code>`;
  return null;
}

function highlight(hexes: string[]) {
  ui.board.querySelectorAll('.hex.hl').forEach((e) => e.classList.remove('hl'));
  for (const h of hexes) ui.board.querySelector(`.hex[data-hex="${h}"]`)?.classList.add('hl');
}

document.addEventListener('mousemove', (e) => {
  const target = (e.target as Element).closest?.('[data-token],[data-hex][data-index],[data-value],[data-card],[data-pile],[data-track],[data-board-seat],[data-action]') as HTMLElement | null;
  const html = target ? details(target) : null;
  const hl = (e.target as Element).closest?.('[data-hexes]') as HTMLElement | null;
  highlight(hl?.dataset.hexes ? hl.dataset.hexes.split(' ').filter(Boolean) : []);
  if (!html) {
    ui.tip.hidden = true;
    return;
  }
  ui.tip.innerHTML = html;
  ui.tip.hidden = false;
  const pad = 14;
  const w = ui.tip.offsetWidth;
  const h = ui.tip.offsetHeight;
  const x = e.clientX + pad + w > innerWidth ? e.clientX - pad - w : e.clientX + pad;
  const y = e.clientY + pad + h > innerHeight ? Math.max(4, e.clientY - pad - h) : e.clientY + pad;
  ui.tip.style.left = `${x}px`;
  ui.tip.style.top = `${y}px`;
});

// ---------- input ----------

ui.board.addEventListener('click', (e) => {
  const g = (e.target as Element).closest('[data-hex]') as SVGElement | null;
  if (!g || !armed) return;
  const key = g.dataset.hex;
  // Two answers on one hex: the click takes the first, and each has its own button.
  const a = legal.find((x) => x.seat === armed!.seat && choiceKey(x) === armed!.key && isHex(x.target) && hexKey(x.target) === key);
  if (a) act(a);
});

ui.log.addEventListener('click', (e) => {
  // A phase heading folds or unfolds its group; it doesn't move the timeline.
  if ((e.target as Element).closest('summary')) return;
  const entry = (e.target as Element).closest('[data-step]') as HTMLElement | null;
  if (entry) seek(Number(entry.dataset.step));
});

document.getElementById('tabs')!.addEventListener('click', (e) => {
  const b = (e.target as Element).closest('[data-tab]') as HTMLElement | null;
  if (!b) return;
  activeTab = b.dataset.tab!;
  renderTab();
});

ui.newForm.addEventListener('submit', (e) => {
  e.preventDefault();
  const seed = parseSeed(ui.seed.value);
  if (seed === null) return showError('the seed must be a whole number from 0 to 18446744073709551615');
  guard(() => newGame(ui.config.value, seed));
});

$('random-seed').onclick = () => {
  ui.seed.value = randomSeed();
  guard(() => newGame(ui.config.value, ui.seed.value));
};

$('undo').onclick = () => {
  if (!tl) return;
  tl.undo();
  armed = null;
  tl.save();
  render();
};

/** One uniformly random legal action at the last step; false once the game is over. */
function randomAction(): boolean {
  tl.seek(tl.last);
  const moves = legalActions(tl.frame.snap);
  if (!moves.length) return false;
  tl.act(moves[Math.floor(Math.random() * moves.length)]);
  return true;
}

function randomly(times: number) {
  if (!tl) return;
  armed = null;
  guard(() => {
    for (let i = 0; i < times && randomAction(); i++);
  });
  tl.save();
  render();
}

$('random-action').onclick = () => randomly(1);
$('random-end').onclick = () => randomly(100_000);

function stopAutoplay() {
  if (autoplay !== null) clearInterval(autoplay);
  autoplay = null;
}

$('first').onclick = () => seek(0);
$('back').onclick = () => tl && seek(tl.cursor - 1);
$('fwd').onclick = () => tl && seek(tl.cursor + 1);
$('end').onclick = () => tl && seek(tl.last);
ui.scrub.oninput = () => seek(Number(ui.scrub.value));

/** The step where the round shown at `i` began (its `round_started` frame, or 0). */
function roundStart(i: number): number {
  const r = tl.frames[i].snap.state.round;
  while (i > 0 && tl.frames[i - 1].snap.state.round === r) i--;
  return i;
}

/**
 * Page Up / Page Down (022, D4): back to the start of the shown round, or of the round
 * before when already there; on to the start of the next round, or the last step.
 */
function jumpRound(dir: -1 | 1) {
  if (dir < 0) {
    const s = roundStart(tl.cursor);
    seek(s < tl.cursor || s === 0 ? s : roundStart(s - 1));
  } else {
    const r = tl.frame.snap.state.round;
    let i = tl.cursor;
    while (i < tl.last && tl.frames[i].snap.state.round === r) i++;
    seek(i);
  }
}

/** Autoplay speed (022, D4): 1x is 4 steps a second; 16x moves 4 steps a tick. */
const SPEED_KEY = 'aic.speed';
let speed = Number(storageGet(SPEED_KEY)) || 1;
if (![1, 4, 16].includes(speed)) speed = 1;
ui.speed.value = String(speed);

function startAutoplay() {
  const perTick = Math.max(1, speed / 4);
  autoplay = window.setInterval(() => {
    if (tl.atEnd) stopAutoplay();
    else {
      tl.seek(Math.min(tl.last, tl.cursor + perTick));
      tl.save();
    }
    render();
  }, 250 / Math.min(speed, 4));
}

ui.speed.onchange = () => {
  speed = Number(ui.speed.value) || 1;
  storageSet(SPEED_KEY, String(speed));
  if (autoplay !== null) {
    stopAutoplay();
    startAutoplay();
  }
  // Give the keys (Space, arrows, Page Up/Down) back to the timeline.
  ui.speed.blur();
};

ui.play.onclick = () => {
  if (!tl) return;
  if (autoplay !== null) stopAutoplay();
  else {
    if (tl.atEnd) tl.seek(0);
    startAutoplay();
  }
  render();
};

document.addEventListener('keydown', (e) => {
  // Keys and the toolbar do nothing until the engine has loaded (020: they threw).
  if (!tl) return;
  const target = e.target as Element;
  // The slider keeps focus after a drag: it moves itself with the arrows, Home and End,
  // and the other keys still work (020: Space, R and Ctrl+Z stopped after using it).
  if (target.closest('input[type=range]') && ['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(e.key)) return;
  if (target.closest('input:not([type=range]), textarea, select, dialog')) return;
  // Space on a focused button presses that button (020: it started autoplay instead).
  if (e.key === ' ' && target.closest('button')) return;
  const keys: Record<string, () => void> = {
    ArrowLeft: () => seek(tl.cursor - 1),
    ArrowRight: () => seek(tl.cursor + 1),
    Home: () => seek(0),
    End: () => seek(tl.last),
    PageUp: () => jumpRound(-1),
    PageDown: () => jumpRound(1),
    ' ': () => ui.play.click(),
    r: () => $('random-action').click(),
  };
  if (e.key === 'z' && (e.ctrlKey || e.metaKey)) {
    e.preventDefault();
    $('undo').click();
    return;
  }
  // Leave browser shortcuts (Ctrl+R, Alt+←, ...) alone.
  if (e.ctrlKey || e.metaKey || e.altKey) return;
  const f = keys[e.key];
  if (f) {
    e.preventDefault();
    f();
  }
});

// ---------- records ----------

function currentRecord(toCursor: boolean): string {
  const actions = toCursor ? tl.actionsToCursor : tl.actions;
  const snap = toCursor ? tl.frame.snap : tl.frames[tl.last].snap;
  return recordText(snap, actions, ui.recordNote.value.trim());
}

async function copy(text: string) {
  ui.recordText.value = text;
  try {
    await navigator.clipboard.writeText(text);
    ui.recordMsg.textContent = `Copied (${text.length} characters).`;
  } catch {
    ui.recordText.select();
    ui.recordMsg.textContent = 'Clipboard unavailable: the record is selected below, copy it with Ctrl+C.';
  }
}

$('record').onclick = () => {
  if (!tl) return;
  ui.recordNote.value = tl.source.kind === 'record' ? tl.source.note : '';
  ui.recordText.value = currentRecord(false);
  ui.recordMsg.textContent = '';
  ui.recordDialog.showModal();
};
$('record-copy').onclick = () => void copy(currentRecord(false));
$('record-copy-cursor').onclick = () => void copy(currentRecord(true));
$('record-close').onclick = () => ui.recordDialog.close();
$('record-load').onclick = () => {
  try {
    loadRecordText(ui.recordText.value, 'pasted record');
    ui.recordDialog.close();
  } catch (e) {
    ui.recordMsg.textContent = e instanceof Error ? e.message : String(e);
  }
};

$('open-record').onclick = () => ui.recordFile.click();
ui.recordFile.onchange = async () => {
  const f = ui.recordFile.files?.[0];
  if (f) guard(() => void f.text().then((t) => guard(() => loadRecordText(t, f.name)), (e) => showError(`could not read ${f.name}: ${e}`)));
  ui.recordFile.value = '';
};
document.addEventListener('dragover', (e) => e.preventDefault());
document.addEventListener('drop', (e) => {
  e.preventDefault();
  const f = e.dataTransfer?.files?.[0];
  // An unreadable drop (a folder, a locked file) says so (020: nothing happened).
  if (f) void f.text().then((t) => guard(() => loadRecordText(t, f.name)), (e) => showError(`could not read ${f.name}: ${e}`));
});

// ---------- start ----------

/** Whether two JSON texts hold the same value (whitespace and key order aside). */
function sameJson(a: string, b: string): boolean {
  const norm = (v: unknown): unknown =>
    Array.isArray(v) ? v.map(norm)
    : v && typeof v === 'object' ? Object.fromEntries(Object.entries(v).sort(([x], [y]) => x.localeCompare(y)).map(([k, x]) => [k, norm(x)]))
    : v;
  try {
    return JSON.stringify(norm(JSON.parse(a))) === JSON.stringify(norm(JSON.parse(b)));
  } catch {
    return a === b;
  }
}

async function main() {
  await load();
  ui.version.textContent = `${version} · rules r${rulesRevision}`;
  $('legend').innerHTML = legendHtml();
  ui.config.innerHTML = Object.keys(CONFIGS).map((f) => `<option>${escapeHtml(f)}</option>`).join('');
  let restored: Timeline | null = null;
  let restoreError: string | null = null;
  try {
    restored = Timeline.restore();
  } catch (e) {
    restoreError = `could not restore the saved session (a config from an older engine?): ${e}`;
  }
  if (restored) {
    const file = restored.source.file;
    if (CONFIGS[file] !== undefined) ui.config.value = file;
    ui.seed.value = restored.seed;
    setTimeline(restored);
    if (restored.replayError) showError(`saved session: ${restored.replayError}`);
    // The session keeps the config it started with: say so when the file has changed
    // since (a retuned baseline), or the game shown is not what the file now plays.
    else if (restored.source.kind === 'play' && CONFIGS[file] !== undefined && !sameJson(CONFIGS[file], restored.configJson))
      showError(`saved session: this game was started with an older copy of ${file}; New game plays the current one`);
    else if (restored.source.kind === 'play' && CONFIGS[file] === undefined && file.endsWith('.json'))
      showError(`saved session: ${file} is no longer in configs/; this game keeps the copy it started with`);
  } else {
    ui.config.value = CONFIGS['baseline.json'] !== undefined ? 'baseline.json' : Object.keys(CONFIGS)[0];
    ui.seed.value = '1';
    newGame(ui.config.value, '1');
    // After the new game, which clears the error bar (020: the reason was wiped at once).
    if (restoreError) showError(restoreError);
  }
}

main().catch((e) => showError(`failed to start: ${e instanceof Error ? e.stack : e}`));

// For scripts/e2e.mjs, scripts/sweep.mjs and the console. The helpers go through the
// same paths as the buttons (render included); `play` takes a config's text, so a script
// can start a game on any override of a config file.
Object.assign(window, {
  aic: {
    get timeline() { return tl; },
    get view() { return shown; },
    get legal() { return legal; },
    play: (configJson: string, seed: string, file = 'script') => setTimeline(Timeline.start(configJson, seed, { kind: 'play', file })),
    open: (text: string, file = 'script') => loadRecordText(text, file),
    act,
    seek,
    get error() { return ui.error.hidden ? null : ui.error.textContent; },
  },
});
