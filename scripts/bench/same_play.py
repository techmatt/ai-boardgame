"""Proves a refactor left play unchanged: same games, state by state where it can tell.

    python scripts/bench/same_play.py run SIM DIR       # play the matrix on binary SIM into DIR
    python scripts/bench/same_play.py compare SIM BASE NEW   # SIM: the new binary

The matrix covers both configs, 1-4 countries, three difficulties, the off and still
variants, with the random, greedy and engine agents, saving every game. `compare` checks:

- summaries match, timing and the content hash aside (renaming ops moves the hash);
- first-game traces (actions and events) match, wall time and the content hash aside;
- every game saved by the old binary replays on the new one to the same fingerprint, so
  option labels, counters and vars all survived.

Exits 1 on any difference. About 15 min for each `run` on 12 threads (025); build the old
binary first and copy it aside. For experiment specs use `same_results.py`.
"""

import glob
import json
import os
import subprocess
import sys

CONFIGS = ["baseline", "shared_row"]
# Fields and trace lines that differ between runs of the same play.
IGNORED = ("content_hash", "ms_per_game", "paths")
TRACE_NOISE = ("wall ms/game", "seeds ")


def play(sim, out, label, agent, games, config, sets):
    base = [os.path.abspath(sim), "run", f"configs/{config}.json", "--seed", "1000", "--agent", agent]
    extra = [x for s in sets for x in ("--set", s)]
    rec = os.path.join(out, "rec", label)
    with open(os.path.join(out, label + ".json"), "w", encoding="utf-8") as f:
        subprocess.run(base + extra + ["--games", str(games), "--json", "--save", rec,
                                       "--save-when", "all", "--save-max", "1000"],
                       stdout=f, stderr=subprocess.DEVNULL, check=True)
    with open(os.path.join(out, label + ".trace"), "wb") as f:
        subprocess.run(base + extra + ["--games", "1", "--threads", "1", "--trace"],
                       stdout=f, stderr=subprocess.DEVNULL, check=True)


def run(sim, out):
    os.makedirs(out, exist_ok=True)
    for cfg in CONFIGS:
        for c in (1, 2, 3, 4):
            for d in (0, 2, 4):
                sets = [f"countries={c}", f"adversary.difficulty={d}"]
                play(sim, out, f"{cfg}-c{c}-d{d}-random", "random", 12, cfg, sets)
                play(sim, out, f"{cfg}-c{c}-d{d}-greedy", "greedy", 8, cfg, sets)
        play(sim, out, f"{cfg}-c2-d2-engine", "engine", 6, cfg, ["countries=2", "adversary.difficulty=2"])
        play(sim, out, f"{cfg}-c3-d1-engine", "engine", 4, cfg, ["countries=3", "adversary.difficulty=1"])
    for v in ("threats.variant=off", "alignment.variant=still", "alignment.variant=off", "cards.variant=off"):
        play(sim, out, "v-" + v, "greedy", 6, "baseline", [v])


def clean(v):
    if isinstance(v, dict):
        return {k: clean(x) for k, x in v.items()
                if k not in IGNORED}
    if isinstance(v, list):
        return [clean(x) for x in v]
    return v


def compare(sim, base, new):
    bad = 0
    summaries = sorted(glob.glob(os.path.join(base, "*.json")))
    for f in summaries:
        g = os.path.join(new, os.path.basename(f))
        with open(f, encoding="utf-8") as a, open(g, encoding="utf-8") as b:
            if clean(json.load(a)) != clean(json.load(b)):
                print("summary differs:", os.path.basename(f))
                bad += 1
    traces = sorted(glob.glob(os.path.join(base, "*.trace")))
    for f in traces:
        with open(f, encoding="utf-8") as a, open(os.path.join(new, os.path.basename(f)), encoding="utf-8") as b:
            keep = lambda h: [x for x in h if not x.startswith(TRACE_NOISE)]
            if keep(a) != keep(b):
                print("trace differs:", os.path.basename(f))
                bad += 1
    records = sorted(glob.glob(os.path.join(base, "rec", "*", "*.json")))
    for i in range(0, len(records), 40):
        p = subprocess.run([os.path.abspath(sim), "replay", *records[i:i + 40]], capture_output=True, text=True)
        lines = (p.stdout + p.stderr).splitlines()
        for line in lines:
            if ("WARNING" in line and "content hash" not in line) or "panicked" in line or "FAIL" in line:
                print(line[:200])
                bad += 1
        # Exit 1 is expected for content-hash warnings alone; anything else is a crash.
        warned = sum("content hash" in line for line in lines)
        if p.returncode not in (0, 1) or (p.returncode == 1 and warned == 0):
            print(f"replay exited {p.returncode}: {records[i]}...")
            bad += 1
    print(f"{len(summaries)} summaries, {len(traces)} traces, {len(records)} records: {bad} difference(s)")
    return 1 if bad else 0


if __name__ == "__main__":
    if sys.argv[1:2] == ["run"] and len(sys.argv) == 4:
        run(sys.argv[2], sys.argv[3])
    elif sys.argv[1:2] == ["compare"] and len(sys.argv) == 5:
        sys.exit(compare(*sys.argv[2:5]))
    else:
        sys.exit(__doc__)
