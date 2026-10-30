// Placeholder icons, drawn by the locked iconography rules: resources are square
// tiles, card tokens are discs, board pieces are bare silhouettes, tags are ringed
// circles. A cost is a minus before the icon, the income box is brown and the
// adversary frame is dark (not on the sample card). The rendered set replaces these.

const svg = (body, cls = '') => `<svg class="ico ${cls}" viewBox="0 0 24 24" aria-hidden="true">${body}</svg>`;

const ICONS = {
  // Resource: a square tile.
  insight: () => '<rect x="2" y="2" width="20" height="20" rx="3" fill="#6d3fa8"/><circle cx="12" cy="12" r="4" fill="none" stroke="#fff" stroke-width="2"/><path d="M4 12 Q12 3 20 12 Q12 21 4 12Z" fill="none" stroke="#fff" stroke-width="2"/>',
  // Card token: a disc.
  dataset: () => '<circle cx="12" cy="12" r="10.5" fill="#1f6f9c" stroke="#0d3a54" stroke-width="1.5"/><g fill="none" stroke="#fff" stroke-width="1.8"><ellipse cx="12" cy="7.5" rx="5.5" ry="2.2"/><path d="M6.5 7.5v9c0 1.2 2.5 2.2 5.5 2.2s5.5-1 5.5-2.2v-9M6.5 12c0 1.2 2.5 2.2 5.5 2.2s5.5-1 5.5-2.2"/></g>',
  // "Any of your cards": a card with a dashed outline.
  anycard: () => '<rect x="5" y="1.5" width="14" height="21" rx="2" fill="none" stroke="#222" stroke-width="2" stroke-dasharray="3.2 2.2"/>',
  // Board piece: a bare silhouette, no tile.
  ambassador: () => '<circle cx="12" cy="5.5" r="4.2" fill="#222"/><path d="M4 23c0-8 3-12 8-12s8 4 8 12z" fill="#222"/>',
  arrow: () => '<path d="M2 12h16M13 6l6 6-6 6" fill="none" stroke="#222" stroke-width="2.6"/>',
  hex: () => '<path d="M12 1.5 21.5 7v10L12 22.5 2.5 17V7z" fill="none" stroke="#222" stroke-width="2.2"/>',
};

const TAGS = {
  science: { ring: '#2c7fb8', glyph: '<path d="M9 3h6M10 3v7l-5.5 9.5c-.5 1 .2 2 1.2 2h12.6c1 0 1.7-1 1.2-2L14 10V3" fill="none" stroke="#fff" stroke-width="2"/><path d="M7.5 16h9" stroke="#fff" stroke-width="2"/>' },
  diplomacy: { ring: '#b8862c', glyph: '<path d="M2 11l5-4 5 2 5-2 5 4-8 8z" fill="none" stroke="#fff" stroke-width="2" stroke-linejoin="round"/><path d="M8 13l4 4" stroke="#fff" stroke-width="2"/>' },
};

export function icon(name, opts = {}) {
  const draw = ICONS[name];
  if (!draw) throw new Error(`no icon ${name}`);
  return svg(draw(), `${name}${opts.small ? ' small' : ''}`);
}

export function tagMedallion(tag) {
  const t = TAGS[tag];
  if (!t) throw new Error(`no tag ${tag}`);
  return `<svg class="medallion" viewBox="0 0 28 28" aria-label="${tag}"><circle cx="14" cy="14" r="13" fill="#1b1f24" stroke="${t.ring}" stroke-width="2"/><g transform="translate(2 2)">${t.glyph}</g></svg>`;
}
