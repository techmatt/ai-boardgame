"""The three-clocks screen summary (034, rules r19).

    python scripts/cards/screen_034.py DIR [DIR ...]
    python scripts/cards/screen_034.py --gate DIR [DIR ...]

Each DIR is an `aic-sim experiment` output. Prints, per cell: win %, win and loss rounds, the
failure mix (Discord deck / presence / round cap, with each share of losses), the rounds
presence losses fall in, tokens left on the tracks at the end of a game (a seat, averaged),
rich actions, presence lost and frees; then the measured presence shares by source. --gate is
028's table.
"""
import collections
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
import screen_028 as old  # noqa: E402
import screens as s  # noqa: E402

CONTENT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'content', 'ais.json')
TOKENS = len(json.load(open(CONTENT, encoding='utf-8'))[0]['presence']) \
    + len(json.load(open(CONTENT, encoding='utf-8'))[0]['presence2'])


def table(cells, games):
    print('| variant | d | agent | n | win % | win rounds | loss rounds | wins: purge / Unity | losses: Discord / presence / cap (% of losses) | presence-loss rounds | tokens left a seat | rich uses a game | presence lost: wins / losses | freed |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    key = lambda c: (cells[c]['variant'], cells[c]['difficulty'], cells[c]['agent'])
    for cell in sorted(games, key=key):
        gs = games[cell]
        c = cells[cell]
        seats = 3 if c['variant'].startswith('c3') else 2
        wins = [g for g in gs if g['won']]
        losses = [g for g in gs if not g['won']]
        e = collections.Counter(g['reason'] for g in gs)
        n = len(losses) or 1
        mix = f"{e['discord_deck']} / {e['presence']} / {e['deck_empty']} ({100 * e['discord_deck'] // n} / {100 * e['presence'] // n} / {100 * e['deck_empty'] // n})"
        left = old.mean([TOKENS - 1 - old.count(g, 'board.presence_placed') / seats for g in gs])
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {len(gs)} | {100 * len(wins) / len(gs):.0f} "
              f"| {old.hist(g['rounds'] for g in wins)} | {old.hist(g['rounds'] for g in losses)} "
              f"| {e['purge']} / {e['unity_deck']} | {mix} "
              f"| {old.hist(g['rounds'] for g in gs if g['reason'] == 'presence')} "
              f"| {left:.1f} | {old.mean([old.count(g, 'board.std.presence_buy') for g in gs]):.1f} "
              f"| {old.mean([old.count(g, 'board.presence_lost') for g in wins]):.1f} / {old.mean([old.count(g, 'board.presence_lost') for g in losses]):.1f} "
              f"| {old.mean([old.count(g, 'board.cities_freed') for g in gs]):.1f} |")


if __name__ == '__main__':
    args = [a for a in sys.argv[1:] if a != '--gate']
    for d in args:
        cells, games = s.load(d)
        if '--gate' in sys.argv:
            old.gate(cells, games)
        else:
            table(cells, games)
            old.sources(games)
