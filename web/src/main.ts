// The AI Complete viewer: play a game (any seat, any order) or step through a saved
// one. Renders the engine's generic view and offers exactly the engine's
// `legal_actions`. No game rules live here.

import { renderBoard } from './board';
import {
  aiBoard, checkRecord, legalActions, load, recordText, resolveConfig, version, view as engineView,
  type Action, type CardView, type View,
} from './engine';
import {
  actionLabel, choiceKey, escapeHtml, eventText, hexKey, hexesIn, isHex, strong, tokenColour, tokenName,
  type LabelContext,
} from './format';
import { cardChip, countriesHtml, legendHtml, pilesHtml, pipelineHtml, seatCardsHtml, seatValuesHtml, statusHtml, varsHtml } from './panels';
import { Timeline, parseSeed, randomSeed, readRecord } from './session';
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
  const check = checkRecord(text);
  const warn = [...(check.warnings ?? []), ...(check.error ? [check.error] : [])];
  if (warn.length && !t.replayError) showError(`record loaded with warnings: ${warn.join('; ')}`);
}

function act(a: Action) {
  const later = tl.last - tl.cursor;
  if (later > 0 && !confirm(`Acting here drops the ${later} later action(s). Go on?`)) return;
  armed = null;
  guard(() => {
    tl.act(a);
    showError(null);
  });
  tl.save();
  render();
}

function seek(i: number) {
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
    const ai = aiBoard(tl.frame.snap.state.config, s)?.name;
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
    ui.prompt.innerHTML = `<b>${seatName(v.choice.seat)} decides:</b> ${escapeHtml(v.choice.prompt.replace(/^Seat \d+:\s*/, ''))} <span class="how">— ${how}</span>`;
    return;
  }
  if (armed) {
    ui.prompt.className = 'choice';
    const a = legal.find((x) => x.seat === armed!.seat && choiceKey(x) === armed!.key)!;
    ui.prompt.innerHTML = `<b>${seatName(armed.seat)}:</b> pick a highlighted hex for ${escapeHtml(actionLabel(a, false, false, labels()))} <span class="how">(Esc cancels)</span>`;
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
    status = `<span class="outcome ${o.won ? 'won' : 'lost'}">${o.won ? 'WON' : 'LOST'} in round ${o.round}: ${escapeHtml(o.reason)} (${escapeHtml(o.detail)})</span>`;
  } else status = ''; // who is to act is in #prompt
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
    const ai = aiBoard(config, seat.index);
    const slots = country?.slots.length
      ? `<div class="row"><span class="what">card strip</span>${country.slots.map((s, i) => (s ? cardChip(s, `slot:${seat.country}:${i}`) : '<span class="slot">—</span>')).join('')}</div>`
      : '';
    box.innerHTML = `
      <h3><span class="swatch"></span>Seat ${seat.index}${ai ? ` · <span data-value="ai.board" data-scope="seat:${seat.index}">${escapeHtml(ai.name)}</span>` : ''} <small>C${seat.country}</small><span class="status">${status}</span></h3>
      ${ai ? `<div class="innate" title="${escapeHtml(ai.text)}"><b>${escapeHtml(ai.innate.name)}:</b> ${escapeHtml(ai.innate.text)}</div>` : ''}
      ${seatValuesHtml(v, seat.index)}
      ${seatCardsHtml(v, seat.index)}
      ${slots}`;
    if (mine.length) box.append(actionButtons(seat.index, mine));
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

/** One button per choice; a choice whose actions differ only by target hex arms a hex pick. */
function actionButtons(seat: number, mine: Action[]): HTMLElement {
  const row = document.createElement('div');
  row.className = 'actions';
  const groups = new Map<string, Action[]>();
  const sorted = [...mine].sort((a, b) => typeRank(a.type) - typeRank(b.type));
  for (const a of sorted) {
    const k = choiceKey(a);
    groups.set(k, [...(groups.get(k) ?? []), a]);
  }
  const cx = labels();
  for (const [key, acts] of groups) {
    if (acts.every((a) => isHex(a.target))) {
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = acts[0].type === 'choose'
        ? `pick a hex ▸ (${acts.length})`
        : `${actionLabel(acts[0], false, false, cx)} ▸ hex (${acts.length})`;
      b.className = armed?.seat === seat && armed.key === key ? 'armed' : '';
      b.dataset.hexes = acts.map((a) => hexKey(a.target!)).join(' ');
      b.dataset.action = JSON.stringify(acts[0]);
      b.onclick = () => {
        armed = armed?.seat === seat && armed.key === key ? null : { seat, key };
        render();
      };
      row.append(b);
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

/** Button order: answers and decisions first, threat counters and gifts next, pass last. */
const TYPE_ORDER = ['choose', 'growth', 'tier', 'draft', 'keep', 'play', 'use', 'draw', 'pay', 'withdraw', 'give'];
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

function renderLog() {
  const parts: string[] = [];
  tl.frames.forEach((f, i) => {
    const cls = `entry${i === tl.cursor ? ' now' : ''}${i > tl.cursor ? ' future' : ''}`;
    const head = f.action ? `#${i} ${escapeHtml(actionLabel(f.action, true, true, { config: f.snap.state.config }))}` : '#0 setup';
    const hexes = [...(f.action ? hexesIn(f.action) : []), ...f.events.flatMap(hexesIn)].map(hexKey);
    const evs = f.events
      .map((e) => {
        const hs = hexesIn(e).map(hexKey).join(' ');
        return `<li class="ev ${escapeHtml(e.type)}"${hs ? ` data-hexes="${hs}"` : ''}>${escapeHtml(eventText(e))}</li>`;
      })
      .join('');
    parts.push(`<div class="${cls}" data-step="${i}" data-hexes="${[...new Set(hexes)].join(' ')}"><div class="head">${head}</div>${evs ? `<ul>${evs}</ul>` : ''}</div>`);
  });
  ui.log.innerHTML = parts.join('');
  ui.log.querySelector('.entry.now')?.scrollIntoView({ block: 'nearest' });
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
    return `<b>${escapeHtml(tokenName(d.token))}</b> × ${tok?.count ?? 0}<br><code>${escapeHtml(d.token)}</code> on C${h?.country} #${h?.number} (${escapeHtml(h?.terrain ?? '')}, hex ${d.hex})`;
  }
  if (d.hex && d.index !== undefined) {
    const h = shown.hexes[Number(d.index)];
    const toks = h.tokens.map((x) => `<span class="dot" style="background:${tokenColour(x.kind)}"></span>${escapeHtml(tokenName(x.kind))} × ${x.count}`).join('<br>') || 'no tokens';
    const here = legal.filter((a) => isHex(a.target) && hexKey(a.target) === d.hex).map((a) => escapeHtml(actionLabel(a, true, true, labels())));
    return `<b>C${h.country} #${h.number}</b> · ${escapeHtml(h.terrain)}${h.uplink ? ' · <b>uplink</b> (the adversary enters here)' : ''}<br>hex ${d.hex} · index ${h.index} · piece position ${h.piece_pos.join(',')} · ${h.neighbours.length} neighbours<hr>${toks}${here.length ? `<hr>legal here:<br>${here.join('<br>')}` : ''}`;
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
      : kind === 'slot' ? [shown.countries[Number(a)]?.slots[Number(key)] ?? null]
      : kind === 'country' ? shown.countries[Number(a)]?.cards.find((g) => g.key === key)?.cards ?? []
      : kind === 'pile' ? shown.piles.find((p) => p.key === [a, ...rest].join(':'))?.visible ?? []
      : [];
    const c = pool.find((x) => x?.id === d.card);
    if (!c) return null;
    return `<b>${escapeHtml(c.name)}</b> <code>${escapeHtml(c.id)}</code>${c.cost !== undefined ? ` · cost ${c.cost}` : ''}<br>${escapeHtml(c.status)}${c.text ? `<br>${escapeHtml(c.text)}` : ''}<br><small>${escapeHtml(d.where ?? '')}</small>`;
  }
  if (d.action) return `<code>${escapeHtml(d.action)}</code>`;
  return null;
}

function highlight(hexes: string[]) {
  ui.board.querySelectorAll('.hex.hl').forEach((e) => e.classList.remove('hl'));
  for (const h of hexes) ui.board.querySelector(`.hex[data-hex="${h}"]`)?.classList.add('hl');
}

document.addEventListener('mousemove', (e) => {
  const target = (e.target as Element).closest?.('[data-token],[data-hex][data-index],[data-value],[data-card],[data-pile],[data-action]') as HTMLElement | null;
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
  const a = legal.find((x) => x.seat === armed!.seat && choiceKey(x) === armed!.key && isHex(x.target) && hexKey(x.target) === key);
  if (a) act(a);
});

ui.log.addEventListener('click', (e) => {
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
$('back').onclick = () => seek(tl.cursor - 1);
$('fwd').onclick = () => seek(tl.cursor + 1);
$('end').onclick = () => seek(tl.last);
ui.scrub.oninput = () => seek(Number(ui.scrub.value));
ui.play.onclick = () => {
  if (autoplay !== null) stopAutoplay();
  else {
    if (tl.atEnd) tl.seek(0);
    autoplay = window.setInterval(() => {
      if (tl.atEnd) stopAutoplay();
      else tl.seek(tl.cursor + 1);
      render();
    }, 250);
  }
  render();
};

document.addEventListener('keydown', (e) => {
  if ((e.target as Element).closest('input, textarea, select, dialog')) return;
  const keys: Record<string, () => void> = {
    ArrowLeft: () => seek(tl.cursor - 1),
    ArrowRight: () => seek(tl.cursor + 1),
    Home: () => seek(0),
    End: () => seek(tl.last),
    ' ': () => ui.play.click(),
    Escape: () => {
      armed = null;
      render();
    },
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
  if (f) guard(() => void f.text().then((t) => guard(() => loadRecordText(t, f.name))));
  ui.recordFile.value = '';
};
document.addEventListener('dragover', (e) => e.preventDefault());
document.addEventListener('drop', (e) => {
  e.preventDefault();
  const f = e.dataTransfer?.files?.[0];
  if (f) void f.text().then((t) => guard(() => loadRecordText(t, f.name)));
});

// ---------- start ----------

async function main() {
  await load();
  ui.version.textContent = version;
  $('legend').innerHTML = legendHtml();
  ui.config.innerHTML = Object.keys(CONFIGS).map((f) => `<option>${escapeHtml(f)}</option>`).join('');
  let restored: Timeline | null = null;
  try {
    restored = Timeline.restore();
  } catch (e) {
    showError(`could not restore the saved session: ${e}`);
  }
  if (restored) {
    const file = restored.source.file;
    if (CONFIGS[file] !== undefined) ui.config.value = file;
    ui.seed.value = restored.seed;
    setTimeline(restored);
    if (restored.replayError) showError(`saved session: ${restored.replayError}`);
  } else {
    ui.config.value = CONFIGS['baseline.json'] !== undefined ? 'baseline.json' : Object.keys(CONFIGS)[0];
    ui.seed.value = '1';
    newGame(ui.config.value, '1');
  }
}

main().catch((e) => showError(`failed to start: ${e instanceof Error ? e.stack : e}`));

// For scripts/e2e.mjs and the console.
Object.assign(window, { aic: { get timeline() { return tl; }, get view() { return shown; }, get legal() { return legal; } } });
