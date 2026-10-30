// A card's face, from its content entry and its face (web/card/sample/<id>.json: the icon
// lines and the rules text worded for them). Strip fields come from content, untouched.
// Extra costs come from content too, because a face must never show a different price:
// they lead the panel below the art, as minus-icons and lowered income, and the text says
// them first, so it keeps mirroring the icons.

const WORD = { credits: 'credits', chips: 'chips', energy: 'energy', antimatter: 'antimatter', insight: 'insight' };
const ONE = { credits: 'credit', chips: 'chip' };
const word = (res, n) => (n === 1 ? (ONE[res] ?? WORD[res]) : WORD[res]);

export function composeFace(card, face) {
  if (face.source_text !== card.text) throw new Error(`face: ${card.id}: its source_text no longer matches content: review the face`);
  const pays = ['chips', 'energy', 'antimatter'].filter((r) => card[r]).map((r) => [r, card[r]]);
  const lowers = card.lowers ?? [];
  const costs = [...(pays.length ? [pays.flatMap(([r, n]) => [`−${n}`, r])] : []), ...lowers.map(([r, n]) => [{ income: [`−${n}`, r] }])];
  const said = [
    pays.length ? `Pay ${pays.map(([r, n]) => `${n} ${word(r, n)}`).join(', ')}.` : '',
    ...lowers.map(([r, n]) => `−${n}\u00a0${word(r, 1)} income.`),
  ].filter(Boolean);
  return {
    ...card,
    top: face.top,
    top_text: face.top_text,
    effect: [...costs, ...(face.effect ?? [])],
    text: [...said, face.text].filter(Boolean).join(' '),
  };
}
