"""What the screen summaries share: reading an `aic-sim experiment` output directory
(DIR/results.json for the cell labels, DIR/games.jsonl for the games) and the two
helpers every table uses. A summary one folder down puts scripts/ on `sys.path` to
import this.
"""
import collections
import json
import sys


def load(d):
    """(cells by id, games by cell), without the games' action lists."""
    cells = {c['cell']: c for c in json.load(open(f'{d}/results.json', encoding='utf-8'))['cells']}
    games = collections.defaultdict(list)
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        next(f)
        for line in f:
            g = json.loads(line)
            g.pop('actions', None)
            games[g['cell']].append(g)
    return cells, games


def cnt(g, k):
    return g['counters'].get(k, 0)


def mean(gs, f):
    return sum(f(g) for g in gs) / len(gs) if gs else 0.0


def checked_cnt(games):
    """`cnt` for a script on older rules: a key in no game warns once on stderr, since
    it may be a counter that is gone and its 0 would pass for a measurement."""
    seen = {k for gs in games.values() for g in gs for k in g['counters']}
    warned = set()

    def checked(g, k):
        if k not in seen and k not in warned:
            warned.add(k)
            print(f'warning: no game has counter {k}: it may be gone, so its 0 means nothing', file=sys.stderr)
        return cnt(g, k)
    return checked
