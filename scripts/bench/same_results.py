"""Checks that two experiment output directories hold the same games.

Usage: python scripts/bench/same_results.py DIR_A DIR_B [--ignore-content-hash]

Compares games.csv row by row and results.json field by field, ignoring
what is timing (any `ms` field or column, wall seconds, threads), record
paths and how many games a resumed run read back. Exits 1 on any
difference. For checking that a speed change left play unchanged, or that
a stopped and resumed run matches an unbroken one (012).
--ignore-content-hash is for a content refactor that renames ops but must
play the same (025).
"""

import csv
import json
import sys

TIMING = ("ms", "wall", "threads", "seconds")
# Run bookkeeping, not play (012 added the games_* counts).
IGNORED = ["path", "games_resumed", "games_played", "games_planned"]


def timing(key):
    k = key.lower()
    return any(t in k.split("_") or k.startswith(t) or k.endswith(t) for t in TIMING)


def strip(v):
    if isinstance(v, dict):
        return {
            k: strip(x)
            for k, x in v.items()
            if not timing(k) and k not in IGNORED
        }
    if isinstance(v, list):
        return [strip(x) for x in v]
    return v


def rows(path):
    with open(path, encoding="utf-8", newline="") as f:
        return [
            {k: v for k, v in r.items() if not timing(k) and k not in IGNORED}
            for r in csv.DictReader(f)
        ]


def diff(a, b, where=""):
    if type(a) is not type(b):
        return [f"{where}: {a!r} != {b!r}"]
    if isinstance(a, dict):
        out = []
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                out.append(f"{where}.{k}: only in {'B' if k not in a else 'A'}")
            else:
                out += diff(a[k], b[k], f"{where}.{k}")
        return out
    if isinstance(a, list):
        if len(a) != len(b):
            return [f"{where}: length {len(a)} != {len(b)}"]
        out = []
        for i, (x, y) in enumerate(zip(a, b)):
            out += diff(x, y, f"{where}[{i}]")
        return out
    return [] if a == b else [f"{where}: {a!r} != {b!r}"]


def main():
    args = [x for x in sys.argv[1:] if x != "--ignore-content-hash"]
    if len(args) != len(sys.argv) - 1:
        IGNORED.append("content_hash")
    a, b = args[0], args[1]
    problems = []
    ra, rb = rows(f"{a}/games.csv"), rows(f"{b}/games.csv")
    if len(ra) != len(rb):
        problems.append(f"games.csv: {len(ra)} rows != {len(rb)}")
    for i, (x, y) in enumerate(zip(ra, rb)):
        if x != y:
            problems.append(f"games.csv row {i + 2}: {x} != {y}")
    ja = strip(json.load(open(f"{a}/results.json", encoding="utf-8")))
    jb = strip(json.load(open(f"{b}/results.json", encoding="utf-8")))
    problems += diff(ja, jb, "results.json")
    for p in problems[:20]:
        print(p)
    if problems:
        print(f"{len(problems)} difference(s)")
        sys.exit(1)
    print(f"same: {len(ra)} games")


if __name__ == "__main__":
    main()
