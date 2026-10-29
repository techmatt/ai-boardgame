"""Board-model screen summary (014): per cell, win and loss rates and causes,
the round of the first new corruption, adversary ambassadors on the board by
round, cities corrupted at the end, destructions per war round, Unity and
Discord tips, and standard-action use.

    python scripts/board/screen_summary.py out/screen-014-board

Reads DIR/games.jsonl and DIR/results.json (cell labels) as written by
`aic-sim experiment`. Prints markdown.
"""
import collections
import json
import sys


def main(d):
    cells = {c['cell']: c for c in json.load(open(f'{d}/results.json', encoding='utf-8'))['cells']}
    games = collections.defaultdict(list)
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        next(f)
        for line in f:
            g = json.loads(line)
            games[g['cell']].append(g)
    print('| cell | variant | d | agent | games | win % | rounds | first new corruption (mean round; % of games) | cities corrupted at end | destroyed per war round | Unity tips | Discord tips | std actions/game | ends |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    amb = {}
    std = {}
    for cell in sorted(games):
        gs = games[cell]
        c = cells[cell]
        n = len(gs)
        cnt = lambda g, k: g['counters'].get(k, 0)
        won = sum(g['won'] for g in gs)
        rounds = sum(g['rounds'] for g in gs) / n
        firsts = [cnt(g, 'adversary.first_corruption_round') for g in gs if cnt(g, 'adversary.first_corruption_round') > 0]
        first = f"{sum(firsts) / len(firsts):.1f}; {100 * len(firsts) / n:.0f}%" if firsts else '-'
        end = [g.get('margin', {}).get('cities_corrupted_end') for g in gs]
        end = [e for e in end if e is not None]
        end_s = f"{sum(end) / len(end):.2f}" if end else '?'
        wr = sum(cnt(g, 'board.war_rounds') for g in gs)
        wd = sum(cnt(g, 'board.war_destroyed') for g in gs)
        per_war = f"{wd / wr:.2f}" if wr else '-'
        ut = sum(cnt(g, 'alignment.tips_unity') for g in gs) / n
        dt = sum(cnt(g, 'alignment.tips_discord') for g in gs) / n
        sa = sum(cnt(g, 'board.std_actions') for g in gs) / n
        ends = collections.Counter(('win ' if g['won'] else 'loss ') + g['reason'] for g in gs)
        print(f"| {cell} | {c['variant']} | {c['difficulty']} | {c['agent']} | {n} | {100 * won / n:.0f} | {rounds:.1f} | {first} | {end_s} | {per_war} | {ut:.2f} | {dt:.2f} | {sa:.2f} | {', '.join(f'{k} {v}' for k, v in ends.most_common())} |")
        by_round = collections.defaultdict(list)
        for g in gs:
            for k, v in g['counters'].items():
                if k.startswith('adversary.ambassadors_r') and k[len('adversary.ambassadors_r'):].isdigit():
                    by_round[int(k[len('adversary.ambassadors_r'):])].append(v)
        amb[cell] = (c, {r: (sum(v) / len(v), len(v)) for r, v in by_round.items()})
        acts = collections.Counter()
        for g in gs:
            for k, v in g['counters'].items():
                if k.startswith('board.std.'):
                    acts[k[len('board.std.'):]] += v
        std[cell] = (c, {k: v / n for k, v in acts.items()})
    print()
    print('Adversary ambassadors on the board after the corrupt step, mean over games still running (games):')
    print()
    rounds = sorted({r for _, h in amb.values() for r in h})
    print('| cell | variant | d | agent | ' + ' | '.join(f'r{r}' for r in rounds) + ' |')
    print('|---|---|---|---|' + '---|' * len(rounds))
    for cell, (c, h) in sorted(amb.items()):
        row = ' | '.join(f"{h[r][0]:.1f} ({h[r][1]})" if r in h else '' for r in rounds)
        print(f"| {cell} | {c['variant']} | {c['difficulty']} | {c['agent']} | {row} |")
    print()
    print('Standard actions per game, by kind:')
    print()
    for cell, (c, a) in sorted(std.items()):
        print(f"- {cell} {c['variant']} d{c['difficulty']} {c['agent']}: " + (', '.join(f'{k} {v:.2f}' for k, v in sorted(a.items(), key=lambda x: -x[1])) or 'none'))


if __name__ == '__main__':
    main(sys.argv[1])
