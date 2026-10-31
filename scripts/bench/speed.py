"""Games per minute for each agent, and a standing check that agents have not slowed.

    python scripts/bench/speed.py table SIM [--out FILE] [--only "AGENT[ cN]"]...
        # the full table: every agent at c2 and c3, d2, one thread and every thread;
        # about 20 min, mostly `rollout`, so `--only` and an `--out` that keeps its rows
    python scripts/bench/speed.py pair OLD NEW [--out FILE] [--only "AGENT[ cN]"]...
        # the same rows on two binaries in turn: use this for a before and after, because
        # the ratio holds on a busy machine and two `table` runs at different times don't
    python scripts/bench/speed.py compare BEFORE.json AFTER.json   # two `table` files
    python scripts/bench/speed.py check SIM                        # about 40 s, see below
    python scripts/bench/speed.py check SIM --update               # after a change meant to cost time

`check` runs a small fixed set on one thread, best of three, and fails if an agent's ms/game
is more than TOLERANCE over `scripts/bench/speed_baseline.json`. It needs a quiet machine:
anything heavy alongside reads as a slowdown. `same_play.py` is the check that play held.
"""

import json
import os
import subprocess
import sys
import time

CONFIG = "configs/baseline.json"
SEED = "1000"
# Agent, then games on one thread and on every thread. Enough to fill 10-60 s each;
# rollout takes minutes, see the README.
TABLE = [
    ("random", 400, 4000),
    ("greedy", 16, 200),
    ("engine", 4, 48),
    ("rollout", 2, 12),
    ("planner", 2, 24),
]
CHECK = [("random", 200, 2), ("greedy", 6, 2), ("engine", 2, 2), ("planner", 1, 2)]
TOLERANCE = 1.3
BASELINE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "speed_baseline.json")


def timed(sim, agent, games, countries, threads):
    """Wall seconds for `games` games."""
    cmd = [os.path.abspath(sim), "run", CONFIG, "--seed", SEED, "--agent", agent, "--games", str(games),
           "--threads", str(threads), "--json", "--set", f"countries={countries}",
           "--set", "adversary.difficulty=2"]
    t = time.perf_counter()
    subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
    return time.perf_counter() - t


def rows_to_run(only, cores):
    """Each (agent, countries, games on one thread, games on every thread) `only` lets through."""
    for agent, one, many in TABLE:
        for c in (2, 3):
            if only and agent not in only and f"{agent} c{c}" not in only:
                continue
            # Three countries take about three times as long: fewer games, a like wait.
            yield agent, c, max(1, one * 2 // c // (c - 1)), max(cores, many * 2 // c // (c - 1))


def saved(out):
    """The rows `out` already holds."""
    if out and os.path.exists(out):
        with open(out, encoding="utf-8") as f:
            return json.load(f)
    return {}


def save(out, rows):
    if out:
        with open(out, "w", encoding="utf-8") as f:
            json.dump(rows, f, indent=1)


def table(sim, out, only):
    """Saves after each row, into what `out` already holds, so a stopped run keeps its rows."""
    rows = saved(out)
    cores = os.cpu_count()
    for agent, c, n1, nn in rows_to_run(only, cores):
        s1 = timed(sim, agent, n1, c, 1)
        sn = timed(sim, agent, nn, c, cores)
        rows[f"{agent} c{c}"] = {"gpm_1": 60 * n1 / s1, "gpm_all": 60 * nn / sn, "games_1": n1, "games_all": nn}
        print(f"{agent} c{c}: {60 * n1 / s1:.1f} games/min on 1 thread ({n1} in {s1:.1f} s), "
              f"{60 * nn / sn:.1f} on {cores} ({nn} in {sn:.1f} s)", flush=True)
        save(out, rows)


def pair(old, new, out, only):
    """Each row on both binaries in turn, old-new-new-old, so both see the same machine: a
    busy box moves the rates but not the ratio. Saves after each row, as `table`."""
    rows = saved(out)
    cores = os.cpu_count()
    for agent, c, n1, nn in rows_to_run(only, cores):
        r = {}
        for label, games, threads in (("1", n1, 1), ("all", nn, cores)):
            first = timed(old, agent, games, c, threads)
            b = timed(new, agent, games, c, threads) + timed(new, agent, games, c, threads)
            a = first + timed(old, agent, games, c, threads)
            r[f"old_{label}"], r[f"new_{label}"] = 120 * games / a, 120 * games / b
        rows[f"{agent} c{c}"] = r
        print(f"{agent} c{c}: 1 thread {r['old_1']:.1f} to {r['new_1']:.1f} ({r['new_1'] / r['old_1']:.2f} x), "
              f"{cores} threads {r['old_all']:.1f} to {r['new_all']:.1f} "
              f"({r['new_all'] / r['old_all']:.2f} x)", flush=True)
        save(out, rows)


def compare(before, after):
    with open(before, encoding="utf-8") as f:
        a = json.load(f)
    with open(after, encoding="utf-8") as f:
        b = json.load(f)
    print("| agent | 1 thread before | after | x | all threads before | after | x |")
    print("|---|---|---|---|---|---|---|")
    for k in a:
        if k in b:
            x, y = a[k], b[k]
            print(f"| {k} | {x['gpm_1']:.1f} | {y['gpm_1']:.1f} | {y['gpm_1'] / x['gpm_1']:.2f} "
                  f"| {x['gpm_all']:.1f} | {y['gpm_all']:.1f} | {y['gpm_all'] / x['gpm_all']:.2f} |")


def check(sim, update):
    now = {}
    for agent, games, c in CHECK:
        now[agent] = 1000 * min(timed(sim, agent, games, c, 1) for _ in range(3)) / games
    if update:
        with open(BASELINE, "w", encoding="utf-8", newline="\n") as f:
            json.dump({k: round(v, 1) for k, v in now.items()}, f, indent=1)
            f.write("\n")
    with open(BASELINE, encoding="utf-8") as f:
        base = json.load(f)
    bad = 0
    for agent, ms in now.items():
        ratio = ms / base[agent]
        slow = ratio > TOLERANCE
        bad += slow
        print(f"{agent}: {ms:.1f} ms/game, {ratio:.2f} x baseline{'  SLOWER' if slow else ''}")
    return 1 if bad else 0


if __name__ == "__main__":
    argv = sys.argv[1:]
    out = argv[argv.index("--out") + 1] if "--out" in argv else None
    only = [argv[i + 1] for i, a in enumerate(argv) if a == "--only"]
    if argv[:1] == ["table"] and len(argv) >= 2:
        table(argv[1], out, only)
    elif argv[:1] == ["pair"] and len(argv) >= 3:
        pair(argv[1], argv[2], out, only)
    elif argv[:1] == ["compare"] and len(argv) == 3:
        compare(argv[1], argv[2])
    elif argv[:1] == ["check"] and len(argv) >= 2:
        sys.exit(check(argv[1], "--update" in argv))
    else:
        sys.exit(__doc__)
