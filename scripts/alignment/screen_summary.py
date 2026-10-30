"""Alignment screen summary (013): per cell, win rate, Unity and Discord tips
per game, the round of each tip, the antimatter action per game, and the
Alignment cards resolved.

    python scripts/alignment/screen_summary.py out/screen-013

Reads DIR/games.jsonl and DIR/results.json (cell labels) as written by
`aic-sim experiment`. Prints markdown.
"""
import collections
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from screens import cnt, load  # noqa: E402


def main(d):
    cells, games = load(d)
    print('| cell | variant | d | agent | games | win % | rounds | Unity tips/game | Discord tips/game | antimatter actions/game | Unity tip rounds | Discord tip rounds | ends |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    cards = collections.defaultdict(collections.Counter)
    for cell in sorted(games):
        gs = games[cell]
        c = cells[cell]
        n = len(gs)
        won = sum(g['won'] for g in gs)
        rounds = sum(g['rounds'] for g in gs) / n
        ut = sum(cnt(g, 'alignment.tips_unity') for g in gs) / n
        dt = sum(cnt(g, 'alignment.tips_discord') for g in gs) / n
        am = sum(cnt(g, 'alignment.antimatter_actions') for g in gs) / n
        by_round = {s: collections.Counter() for s in ('unity', 'discord')}
        for g in gs:
            for k, v in g['counters'].items():
                for s in by_round:
                    pre = f'alignment.tip_{s}.r'
                    if k.startswith(pre):
                        by_round[s][int(k[len(pre):])] += v
                if k.startswith('alignment.card.'):
                    cards[(c['difficulty'], c['agent'])][k[len('alignment.card.'):]] += v
        hist = lambda h: ' '.join(f'r{r}:{h[r]}' for r in sorted(h)) or '-'
        ends = collections.Counter(g['reason'] for g in gs)
        print(f"| {cell} | {c['variant']} | {c['difficulty']} | {c['agent']} | {n} | {100 * won / n:.0f} | {rounds:.1f} | {ut:.2f} | {dt:.2f} | {am:.2f} | {hist(by_round['unity'])} | {hist(by_round['discord'])} | {', '.join(f'{k} {v}' for k, v in ends.most_common())} |")
    print()
    print('Alignment cards resolved (all variants), total over the games:')
    print()
    for key in sorted(cards):
        print(f'- d{key[0]} {key[1]}: ' + ', '.join(f'{k} {v}' for k, v in cards[key].most_common()))


if __name__ == '__main__':
    main(sys.argv[1])
