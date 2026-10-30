"""Troop and node screen summary (019, rules r10): one row per cell with the
win rate, rounds, how games ended and what was left on the board at the end
(the purge's blockers); then per game the adversary troops (built, steps,
moves, strikes, destructions by target, absorbed), presence lost to troops
and placed back from the reserve; nodes made, removed and their peak, and
the rounds that ended with only nodes blocking the purge; then the
strategy-health gate (presence, factories, hardware, nodes removed, cards,
tier-2 node removal) with a pass/fail line per cell.

    python scripts/board/screen_019.py out/screen-019 [--short]

Reads DIR/games.jsonl and DIR/results.json (cell labels) as written by
`aic-sim experiment`. Prints markdown. `--short` prints only the first table.

The gate (019): in every greedy and engine cell, per game on average,
presence placed >= 5, player factories built >= 1, hardware made >= 1, cards
played >= 5 (018's gate); and in the engine cells at d0, some node removed
(the tier-2 removal is reachable and used: removal is meant to be rare, so
the bar is above zero, not one a game) and some presence placed back from the
reserve (troops bite and players recover).
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
    print('Adversary troops and nodes, per game (means over the cell): troops built (factories, projects); '
          'march steps / moves / strikes; destroyed presence / station / factory; strikes absorbed; '
          'presence lost to troops / placed back from the reserve; nodes made / removed / peak on the board; '
          'rounds ending with only nodes blocking the purge, and the share of games with one.')
    print()
    print('| variant | d | agent | troops built | steps / moves / strikes | destroyed P / S / F | absorbed '
          '| presence lost / replaced | nodes made / removed / peak | only-nodes rounds (games %) |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        m = lambda k: mean(gs, lambda g: cnt(g, k))
        blocked = 100 * sum(cnt(g, 'alignment.purge_blocked_by_nodes') > 0 for g in gs) / len(gs)
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} "
              # Troops from factories, projects and threats (020: threats' were left out).
              f"| {m('board.troops_built') + m('adversary.project_troops') + m('threats.troops'):.1f} "
              f"| {m('board.troop_steps'):.1f} / {m('board.troop_moves'):.1f} / {m('board.troop_strikes'):.1f} "
              f"| {m('board.troop_destroyed.presence'):.1f} / {m('board.troop_destroyed.station'):.1f} / "
              f"{m('board.troop_destroyed.factory'):.1f} | {m('board.troop_absorbed'):.1f} "
              f"| {m('board.troop_destroyed.presence'):.1f} / {m('board.presence_from_reserve'):.1f} "
              f"| {m('adversary.nodes_made'):.1f} / {m('adversary.nodes_removed'):.1f} / {m('adversary.nodes_peak'):.1f} "
              f"| {m('alignment.purge_blocked_by_nodes'):.1f} ({blocked:.0f}%) |")
    print()
    print('Strategy-health gate, per game (means over the cell): presence placed; factories built '
          'player / adversary / neutral; hardware made / spent; Node Takedown played; Decommission drawn; '
          'cities freed; cards played; gate (rules in the script docstring).')
    print()
    print('| variant | d | agent | presence | factories P / A / N | hardware made / spent | Node Takedown '
          '| Decommission | freed | cards | gate |')
    print('|---|---|---|---|---|---|---|---|---|---|---|')
    fails = []
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        m = lambda k: mean(gs, lambda g: cnt(g, k))
        cards = mean(gs, lambda g: sum(v for k, v in g['counters'].items() if k.startswith('cards.played.')))
        gate = '–'
        if c['agent'] != 'random':
            ok = (m('board.presence_placed') >= 5 and m('board.factories_built.player') >= 1
                  and m('board.hardware_made') >= 1 and cards >= 5)
            if c['agent'] == 'engine' and c['difficulty'] == 0:
                ok = ok and m('adversary.nodes_removed') > 0 and m('board.presence_from_reserve') > 0
            gate = 'pass' if ok else 'FAIL'
            if not ok:
                fails.append(f"{c['variant']} d{c['difficulty']} {c['agent']}")
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {m('board.presence_placed'):.1f} "
              f"| {m('board.factories_built.player'):.1f} / {m('board.factories_built.adversary'):.1f} / "
              f"{m('board.factories_built.neutral'):.1f} "
              f"| {m('board.hardware_made'):.1f} / {m('board.hardware_spent'):.1f} "
              f"| {m('cards.played.node_takedown'):.2f} | {m('alignment.card.decommission'):.2f} "
              f"| {m('board.cities_freed'):.2f} | {cards:.1f} | {gate} |")
    print()
    print('Gate: ' + ('pass in every greedy and engine cell.' if not fails else 'FAIL in ' + ', '.join(fails) + '.'))


if __name__ == '__main__':
    main(sys.argv[1], '--short' in sys.argv[2:])
