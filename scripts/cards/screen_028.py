"""The rollout-agent screen summary (028, rules r17).

    python scripts/cards/screen_028.py DIR [DIR ...]
    python scripts/cards/screen_028.py --gate DIR [DIR ...]

Each DIR is an `aic-sim experiment` output. Prints, per cell: win %, win and loss rounds,
how losses and wins split, Unity and Discord tips, presence lost (wins / losses), frees,
and the presence lost by source over all cells.

--gate prints each check's gap to its own agent: an `engine:...` check against `engine`, a
`planner:...` check against `planner` (win % points, ± one standard error).
"""
import collections
import math
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
import screens as s  # noqa: E402


def hist(rounds):
    c = collections.Counter(rounds)
    return ' '.join(f'{r}:{c[r]}' for r in sorted(c)) or '–'


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def count(g, k):
    return g['counters'].get(k, 0)


def table(cells, games):
    print('| variant | d | agent | n | win % | win rounds | loss rounds | wins: purge / Unity deck | losses: Discord / presence / deck-out | Unity / Discord tips | presence lost: wins / losses | freed | bases united |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    key = lambda c: (cells[c]['variant'], cells[c]['difficulty'], cells[c]['agent'])
    for cell in sorted(games, key=key):
        gs = games[cell]
        c = cells[cell]
        wins = [g for g in gs if g['won']]
        losses = [g for g in gs if not g['won']]
        ends = collections.Counter(g['reason'] for g in gs)
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {len(gs)} | {100 * len(wins) / len(gs):.0f} "
              f"| {hist(g['rounds'] for g in wins)} | {hist(g['rounds'] for g in losses)} "
              f"| {ends['purge']} / {ends['unity_deck']} "
              f"| {ends['discord_deck']} / {ends['presence']} / {ends['deck_empty']} "
              f"| {mean([count(g, 'alignment.tips_unity') for g in gs]):.1f} / {mean([count(g, 'alignment.tips_discord') for g in gs]):.1f} "
              f"| {mean([count(g, 'board.presence_lost') for g in wins]):.1f} / {mean([count(g, 'board.presence_lost') for g in losses]):.1f} "
              f"| {mean([count(g, 'board.cities_freed') for g in gs]):.1f} "
              f"| {mean([count(g, 'board.bases_united') for g in gs]):.1f} |")


def sources(games):
    tot = collections.Counter()
    for gs in games.values():
        for g in gs:
            for k, v in g['counters'].items():
                if k.startswith('board.presence_lost.') and not k.split('.')[-1].startswith('r'):
                    tot[k.split('.')[-1]] += v
    n = sum(tot.values()) or 1
    print('Presence lost by source: ' + ', '.join(f'{k} {v} ({100 * v / n:.0f}%)' for k, v in tot.most_common()))


def gate(cells, games):
    print('| variant | check | win % | gap to its agent |')
    print('|---|---|---|---|')
    by = collections.defaultdict(dict)
    for cell, gs in games.items():
        c = cells[cell]
        by[(c['variant'], c['difficulty'])][c['agent']] = gs
    for (v, d), agents in sorted(by.items()):
        for a, gs in sorted(agents.items()):
            base = agents.get(a.split(':')[0])
            if not base:
                continue
            p = sum(g['won'] for g in base) / len(base)
            q = sum(g['won'] for g in gs) / len(gs)
            se = math.sqrt(p * (1 - p) / len(base) + q * (1 - q) / len(gs))
            flag = '' if p - q > 0 or ':' not in a else ' X'
            print(f'| {v} d{d} | {a} | {100 * q:.0f} | {100 * (p - q):+.0f} ±{100 * se:.0f}{flag} |')


if __name__ == '__main__':
    args = [a for a in sys.argv[1:] if a != '--gate']
    for d in args:
        cells, games = s.load(d)
        if '--gate' in sys.argv:
            gate(cells, games)
        else:
            table(cells, games)
            sources(games)
