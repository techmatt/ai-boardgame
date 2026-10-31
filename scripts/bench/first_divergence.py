"""Where two runs of the same spec first play differently, game by game (046).

    python scripts/bench/first_divergence.py DIR_OLD DIR_NEW

Each DIR is an `aic-sim experiment --out` directory on the same spec and seeds. For each game it
finds the first action that differs and classes it: `growth` (the option picked, or a choice inside
a growth option), or the kind of action otherwise; `length` is the same actions, one game longer.
A last line a killed run cut short is dropped. An agent change meant to touch only growth
should print `same` for an agent with no rollouts, and `growth` first wherever it first bites.
"""
import collections
import json
import sys


def games(d):
    out = {}
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        for line in f:
            try:
                g = json.loads(line)
            except json.JSONDecodeError:
                continue
            if 'seed' in g:
                out[(g['cell'], g['seed'])] = g['actions']
    return out


def main():
    old, new = games(sys.argv[1]), games(sys.argv[2])
    tally = collections.defaultdict(collections.Counter)
    for key in sorted(old.keys() & new.keys()):
        a, b = old[key], new[key]
        n = next((i for i, (x, y) in enumerate(zip(a, b)) if x != y), None)
        if n is None:
            tally[key[0]]['same' if len(a) == len(b) else 'length'] += 1
            continue
        k = sorted({a[n]['type'], b[n]['type']})
        last = next((x['type'] for x in reversed(a[:n]) if x['type'] != 'choose'), '')
        if last == 'growth' and k == ['choose']:
            k = ['growth']
        tally[key[0]]['/'.join(k)] += 1
    for cell in sorted(tally):
        print(f'cell {cell}:', dict(tally[cell]))


if __name__ == '__main__':
    main()
