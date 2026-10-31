"""Kept for the docs that cite it (docs/qa-findings.md, screen-017-projects/projects.md, screen-018-calibration/); new work: scripts/cards/screen.py.

Adversary projects / Discord pressure screen summary (017, rules r8): per
cell, win rate, rounds, tips and how games ended; then per game, as means
over the cell's games: projects resolved (cheap, big), adversary compute made
and blocked (a player ambassador on a node), Discord and Unity tips, troop
advances, advance strikes and their destructions, production strikes'
destructions, and the round of the first project (mean, range, and the games
with none); then standard actions by kind (the strategy-health gate reads
these).

    python scripts/board/screen_017.py out/screen-017

Reads DIR/games.jsonl and DIR/results.json (cell labels) as written by
`aic-sim experiment`. Prints markdown.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from screens import checked_cnt, ends_text, load, mean  # noqa: E402


def main(d):
    sys.stdout.reconfigure(encoding='utf-8')
    cells, games = load(d)
    cnt = checked_cnt(games)

    print('| cell | variant | d | agent | games | win % | rounds | Unity tips | Discord tips | ends |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        n = len(gs)
        won = sum(g['won'] for g in gs)
        print(f"| {cell} | {c['variant']} | {c['difficulty']} | {c['agent']} | {n} | {100 * won / n:.0f} "
              f"| {mean(gs, lambda g: g['rounds']):.1f} | {mean(gs, lambda g: cnt(g, 'alignment.tips_unity')):.2f} "
              f"| {mean(gs, lambda g: cnt(g, 'alignment.tips_discord')):.2f} "
              f"| {ends_text(gs)} |")
    print()
    print('Per game (means over the cell): projects resolved cheap / big; adversary compute made / blocked; '
          'Discord / Unity tips; games with a Unity tip; troop advances (moves) / advance strikes / their '
          'destructions; production-strike destructions; first project round (mean over games with one, '
          'range; games with none).')
    print()
    print('| cell | variant | d | agent | projects cheap / big | compute made / blocked | compute a round '
          '| Discord / Unity tips | games with Unity tip | advances / strikes / destroyed | strike destroyed '
          '| first project round |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games):
        gs, c = games[cell], cells[cell]
        firsts = [cnt(g, 'adversary.first_project_round') for g in gs if cnt(g, 'adversary.first_project_round')]
        first = (f"{sum(firsts) / len(firsts):.1f} ({min(firsts)}–{max(firsts)}); none {len(gs) - len(firsts)}"
                 if firsts else f"none in {len(gs)}")
        per_round = mean(gs, lambda g: cnt(g, 'adversary.compute_made') / max(g['rounds'], 1))
        print(f"| {cell} | {c['variant']} | {c['difficulty']} | {c['agent']} "
              f"| {mean(gs, lambda g: cnt(g, 'adversary.projects_resolved.cheap')):.2f} / "
              f"{mean(gs, lambda g: cnt(g, 'adversary.projects_resolved.big')):.2f} "
              f"| {mean(gs, lambda g: cnt(g, 'adversary.compute_made')):.1f} / "
              f"{mean(gs, lambda g: cnt(g, 'adversary.compute_blocked')):.1f} "
              f"| {per_round:.1f} "
              f"| {mean(gs, lambda g: cnt(g, 'alignment.tips_discord')):.2f} / "
              f"{mean(gs, lambda g: cnt(g, 'alignment.tips_unity')):.2f} "
              f"| {sum(1 for g in gs if cnt(g, 'alignment.tips_unity') > 0)}/{len(gs)} "
              f"| {mean(gs, lambda g: cnt(g, 'board.troop_advances')):.1f} / "
              f"{mean(gs, lambda g: cnt(g, 'board.advance_strikes')):.1f} / "
              f"{mean(gs, lambda g: cnt(g, 'board.advance_destroyed')):.1f} "
              f"| {mean(gs, lambda g: cnt(g, 'board.troop_destroyed')):.1f} "
              f"| {first} |")
    print()
    print('Standard actions by kind and cards played, per game (the strategy-health gate):')
    print()
    kinds = sorted({k for gs in games.values() for g in gs for k in g['counters'] if k.startswith('board.std.')})
    print('| cell | ' + ' | '.join(k[len('board.std.'):] for k in kinds) + ' | cards played | drain / discard |')
    print('|---|' + '---|' * len(kinds) + '---|---|')
    for cell in sorted(games):
        gs = games[cell]
        print(f'| {cell} | ' + ' | '.join(f"{mean(gs, lambda g: cnt(g, k)):.1f}" for k in kinds)
              + f" | {mean(gs, lambda g: sum(v for k, v in g['counters'].items() if k.startswith('cards.played.'))):.1f}"
              + f" | {mean(gs, lambda g: cnt(g, 'cards.played.security_patch')):.2f} / "
              f"{mean(gs, lambda g: cnt(g, 'cards.played.zero_day_wipe')):.2f} |")


if __name__ == '__main__':
    main(sys.argv[1])
