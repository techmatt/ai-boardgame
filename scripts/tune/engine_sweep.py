"""Agent sweep with strategy health (008): runs `aic-sim run --json` once per
agent spec on the same seeds and prints win %, rounds, buildings and tier-2
cards played per game. Every row is appended to out/tune/engine.jsonl.

    python scripts/tune/engine_sweep.py --games 48 --seed 0 --set adversary.difficulty=4 \
        greedy "engine" "engine:w.risk=0.2"
"""

import argparse
import json
import os
import subprocess
import sys
import time

SIM = os.environ.get("AIC_SIM", os.path.join("target", "release", "aic-sim"))


def kinds():
    with open(os.path.join("content", "cards.json"), encoding="utf-8") as f:
        d = json.load(f)
    cards = d["cards"] if isinstance(d, dict) else d
    return {c["id"]: (c["kind"], c["tier"]) for c in cards}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("agents", nargs="+")
    ap.add_argument("--config", default="configs/baseline.json")
    ap.add_argument("--games", type=int, default=48)
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--set", action="append", default=[])
    ap.add_argument("--tag", default="")
    a = ap.parse_args()
    k = kinds()
    os.makedirs(os.path.join("out", "tune"), exist_ok=True)
    log = open(os.path.join("out", "tune", "engine.jsonl"), "a", encoding="utf-8", newline="\n")
    for agent in a.agents:
        cmd = [SIM, "run", a.config, "--games", str(a.games), "--seed", str(a.seed),
               "--agent", agent, "--json", "--max-actions", "20000"]
        for s in a.set:
            cmd += ["--set", s]
        t = time.time()
        p = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
        if p.returncode != 0:
            raise SystemExit(f"aic-sim failed ({p.returncode}): {' '.join(cmd)}\n{p.stderr[-2000:]}")
        r = json.loads(p.stdout)["results"][0]
        c = r["counters"]
        played = {key[len("cards.played."):]: v for key, v in c.items() if key.startswith("cards.played.")}
        bld = sum(v for i, v in played.items() if k.get(i, ("", 0))[0] == "building")
        t2 = sum(v for i, v in played.items() if k.get(i, ("", 0))[1] == 2)
        secs = time.time() - t
        top = sorted(((v, i) for i, v in played.items() if k.get(i, ("", 0))[0] == "building"), reverse=True)[:4]
        print(f"{agent:<48} win {100 * r['win_rate']:5.1f}% ±{100 * r['win_rate_se']:4.1f} "
              f"rnd {r['rounds_mean']:4.2f} bld {bld:4.2f} t2 {t2:4.2f} "
              f"[{' '.join(f'{i}:{v:.2f}' for v, i in top)}] {secs:4.0f}s", flush=True)
        log.write(json.dumps({"tag": a.tag, "agent": agent, "games": a.games, "seed": a.seed,
                              "sets": a.set, "win_rate": r["win_rate"], "rounds": r["rounds_mean"],
                              "buildings": bld, "tier2": t2, "played": played, "secs": secs,
                              "time": time.strftime("%H:%M")}) + "\n")
        log.flush()


if __name__ == "__main__":
    sys.exit(main())
