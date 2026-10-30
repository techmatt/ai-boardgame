"""Presence / ladder / factories screen summary (016, rules r7): per cell,
win rate, rounds and how games ended; then per game, as means over the
cell's games: presence placed, node peaks per board, nodes made and
removed, factories built per side, destroyed and owned per side at the end,
hardware made, bought and spent, and adversary compute made; then
standard actions by kind (the strategy-health gate reads these).

    python scripts/board/screen_016.py out/screen-016

Reads DIR/games.jsonl and DIR/results.json (cell labels) as written by
`aic-sim experiment`. Prints markdown.
"""
import collections
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from screens import checked_cnt, load, mean  # noqa: E402


def main(d):
    cells, games = load(d)
    cnt = checked_cnt(games)

    print('| cell | variant | d | agent | games | win % | rounds | Unity tips | Discord tips | ends |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        n = len(gs)
        won = sum(g['won'] for g in gs)
        ends = collections.Counter(('win ' if g['won'] else 'loss ') + g['reason'] for g in gs)
        print(f"| {cell} | {c['variant']} | {c['difficulty']} | {c['agent']} | {n} | {100 * won / n:.0f} "
              f"| {mean(gs, lambda g: g['rounds']):.1f} | {mean(gs, lambda g: cnt(g, 'alignment.tips_unity')):.2f} "
              f"| {mean(gs, lambda g: cnt(g, 'alignment.tips_discord')):.2f} "
              f"| {', '.join(f'{k} {v}' for k, v in ends.most_common())} |")
    print()
    print('Per game (means over the cell): presence placed (std action / all routes); node peak per board '
          '(mean of each board\'s peak; the highest board); nodes made / removed; factories built player / '
          'adversary / neutral; destroyed; owned at the end player / adversary / neutral; hardware made / '
          'bought / spent; adversary compute made.')
    print()
    print('| cell | variant | d | agent | presence | node peak/board (max) | nodes made / removed | factories built P/A/N '
          '| destroyed | owned at end P/A/N | hardware made / bought / spent | adv. compute |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        boards = int(c['variant'][1:]) if c['variant'][1:].isdigit() else 2

        def peaks(g):
            return [cnt(g, f'adversary.nodes_peak.c{b}') for b in range(boards)]

        peak = mean(gs, lambda g: sum(peaks(g)) / boards)
        peak_max = max(max(peaks(g)) for g in gs)
        m = lambda k: mean(gs, lambda g: cnt(g, k))
        print(f"| {cell} | {c['variant']} | {c['difficulty']} | {c['agent']} "
              f"| {m('board.std.presence'):.2f} / {m('board.presence_placed'):.2f} "
              f"| {peak:.2f} ({peak_max}) | {m('adversary.nodes_made'):.2f} / {m('adversary.nodes_removed'):.2f} "
              f"| {m('board.factories_built.player'):.2f} / {m('board.factories_built.adversary'):.2f} / {m('board.factories_built.neutral'):.2f} "
              f"| {m('board.factories_destroyed'):.2f} "
              f"| {m('board.factories_end.player'):.2f} / {m('board.factories_end.adversary'):.2f} / {m('board.factories_end.neutral'):.2f} "
              f"| {m('board.hardware_made'):.2f} / {m('board.hardware_bought'):.2f} / {m('board.hardware_spent'):.2f} "
              f"| {m('adversary.compute_made'):.1f} |")
    print()
    print('Standard actions per game, by kind:')
    print()
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        acts = collections.Counter()
        for g in gs:
            for k, v in g['counters'].items():
                if k.startswith('board.std.'):
                    acts[k[len('board.std.'):]] += v
        n = len(gs)
        print(f"- {cell} {c['variant']} d{c['difficulty']} {c['agent']}: "
              + (', '.join(f'{k} {v / n:.2f}' for k, v in sorted(acts.items(), key=lambda x: -x[1])) or 'none'))


if __name__ == '__main__':
    main(sys.argv[1])
