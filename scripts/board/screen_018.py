"""Calibration screen summary (018, rules r9): one row per cell with the win
rate, rounds, how games ended and what was left on the board at the end
(corrupted cities, nodes, troops; the purge's blockers), then the
strategy-health gate per game: presence placed, factories built per side
and player-owned at the end, hardware made and spent, nodes removed, compute
blocked by a player ambassador on a node, Security Patch played, cities
freed, Unity and Discord tips.

    python scripts/board/screen_018.py out/screen-018 [--short]

Reads DIR/games.jsonl and DIR/results.json (cell labels) as written by
`aic-sim experiment`. Prints markdown. `--short` prints only the first table.
"""
import collections
import json
import sys


def main(d, short):
    sys.stdout.reconfigure(encoding='utf-8')
    cells = {c['cell']: c for c in json.load(open(f'{d}/results.json', encoding='utf-8'))['cells']}
    games = collections.defaultdict(list)
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        next(f)
        for line in f:
            g = json.loads(line)
            games[g['cell']].append(g)
    cnt = lambda g, k: g['counters'].get(k, 0)
    mar = lambda g, k: g['margin'].get(k, 0)
    mean = lambda gs, f: sum(f(g) for g in gs) / len(gs)

    print('| variant | d | agent | games | win % | rounds | left: corrupted / nodes / troops | Unity / Discord tips | ends |')
    print('|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        n = len(gs)
        won = sum(g['won'] for g in gs)
        ends = collections.Counter(('win ' if g['won'] else 'loss ') + g['reason'] for g in gs)
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {n} | {100 * won / n:.0f} "
              f"| {mean(gs, lambda g: g['rounds']):.1f} "
              f"| {mean(gs, lambda g: mar(g, 'cities_corrupted_end')):.1f} / "
              f"{mean(gs, lambda g: mar(g, 'nodes_end')):.1f} / {mean(gs, lambda g: mar(g, 'troops_end')):.1f} "
              f"| {mean(gs, lambda g: cnt(g, 'alignment.tips_unity')):.1f} / "
              f"{mean(gs, lambda g: cnt(g, 'alignment.tips_discord')):.1f} "
              f"| {', '.join(f'{k} {v}' for k, v in ends.most_common())} |")
    if short:
        return
    print()
    print('Strategy-health gate, per game (means over the cell): presence placed; factories built '
          'player / adversary / neutral; player-owned factories at the end; hardware made / spent; '
          'nodes made / removed; adversary compute made / blocked (a player ambassador on a node); '
          'Security Patch played; corrupted cities freed; cards played.')
    print()
    print('| variant | d | agent | presence | factories P / A / N | owned at end | hardware made / spent '
          '| nodes made / removed | compute made / blocked | Security Patch | freed | cards |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        m = lambda k: mean(gs, lambda g: cnt(g, k))
        cards = mean(gs, lambda g: sum(v for k, v in g['counters'].items() if k.startswith('cards.played.')))
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {m('board.presence_placed'):.1f} "
              f"| {m('board.factories_built.player'):.1f} / {m('board.factories_built.adversary'):.1f} / "
              f"{m('board.factories_built.neutral'):.1f} | {m('board.factories_end.player'):.1f} "
              f"| {m('board.hardware_made'):.1f} / {m('board.hardware_spent'):.1f} "
              f"| {m('adversary.nodes_made'):.1f} / {m('adversary.nodes_removed'):.1f} "
              f"| {m('adversary.compute_made'):.1f} / {m('adversary.compute_blocked'):.1f} "
              f"| {m('cards.played.security_patch'):.2f} | {m('board.cities_freed'):.2f} | {cards:.1f} |")


if __name__ == '__main__':
    main(sys.argv[1], '--short' in sys.argv[2:])
