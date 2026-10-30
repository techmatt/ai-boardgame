"""Card and resource screen summary (021, rules r12): one row per cell with the
win rate, rounds and how games ended; then per game the card economy (cards
kept and played, hand size, insight and credits left unspent, powers used and
tags sacrificed, building replacements, tokens gained and spent), the same by
round (rounds 1-6, both seats summed, means over games that reached the
round), and the strategy-health gate with a pass/fail line per cell.

    python scripts/cards/screen_021.py out/screen-021 [--short]

Reads DIR/games.jsonl and DIR/results.json (cell labels) as written by
`aic-sim experiment` (r12: per-round card counters appear as `<key>.rNN`).

The gate (021): in every greedy and engine cell, per game on average,
presence placed >= 5, cards played >= 5 and cards kept >= 8 (018-019's gate,
less hardware: a factory's hardware now only pays buildings); and in the
engine cells at d0, some AI power used and some building replaced (the new
primitives are reachable and used).
"""
import collections
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from screens import checked_cnt, load, mean  # noqa: E402

ROUND = re.compile(r'\.r\d\d$')


def main(d, short):
    sys.stdout.reconfigure(encoding='utf-8')
    cells, games = load(d)
    cnt = checked_cnt(games)
    played = lambda g: sum(v for k, v in g['counters'].items() if k.startswith('cards.played.') and not ROUND.search(k))
    kept = lambda g: sum(v for k, v in g['counters'].items() if k.startswith('cards.kept.') and not ROUND.search(k))

    print('| variant | d | agent | games | win % | rounds | ends |')
    print('|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        n = len(gs)
        won = sum(g['won'] for g in gs)
        ends = collections.Counter(('win ' if g['won'] else 'loss ') + g['reason'] for g in gs)
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {n} | {100 * won / n:.0f} "
              f"| {mean(gs, lambda g: g['rounds']):.1f} | {', '.join(f'{k} {v}' for k, v in ends.most_common())} |")
    if short:
        return
    print()
    print('Card economy, per game (means over the cell): cards kept / played (tier 2 dealt); powers used / '
          'tags sacrificed; buildings replaced; tokens gained / spent; presence placed; gate.')
    print()
    print('| variant | d | agent | kept / played | tier-2 dealt | powers / tags | replaced | tokens + / - | presence | gate |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        m = lambda k: mean(gs, lambda g: cnt(g, k))
        gate = '–'
        if c['agent'] != 'random':
            ok = m('board.presence_placed') >= 5 and mean(gs, played) >= 5 and mean(gs, kept) >= 8
            if c['agent'].startswith('engine') and c['difficulty'] == 0:
                ok = ok and m('ai.powers_used') > 0 and m('cards.replaced') > 0
            gate = 'pass' if ok else 'FAIL'
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {mean(gs, kept):.1f} / {mean(gs, played):.1f} "
              f"| {m('cards.dealt_tier2'):.1f} | {m('ai.powers_used'):.1f} / {m('ai.tags_sacrificed'):.1f} "
              f"| {m('cards.replaced'):.1f} | {m('cards.tokens_gained'):.1f} / {m('cards.tokens_spent'):.1f} "
              f"| {m('board.presence_placed'):.1f} | {gate} |")
    print()
    print('By round (rounds 1-6; both seats summed; means over the games that reached the round): '
          'hand size after keeping, cards kept, cards played, insight / credits left unspent at upkeep, '
          'powers used. Engine and greedy at d0, 2 countries.')
    print()
    print('| agent | measure | r1 | r2 | r3 | r4 | r5 | r6 |')
    print('|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        c = cells[cell]
        if c['difficulty'] != 0 or c['variant'] != 'c2' or c['agent'] == 'random':
            continue
        gs = games[cell]
        for key, label in [('cards.hand_size', 'hand size'), ('cards.kept', 'kept'), ('cards.played', 'played'),
                           ('economy.insight_unspent', 'insight unspent'),
                           ('economy.credits_unspent', 'credits unspent'), ('ai.powers_used', 'powers')]:
            row = []
            for r in range(1, 7):
                reach = [g for g in gs if g['rounds'] >= r]
                row.append(f"{mean(reach, lambda g: cnt(g, f'{key}.r{r:02}')):.1f}" if reach else '–')
            print(f"| {c['agent']} | {label} | {' | '.join(row)} |")


if __name__ == '__main__':
    main(sys.argv[1], '--short' in sys.argv)
