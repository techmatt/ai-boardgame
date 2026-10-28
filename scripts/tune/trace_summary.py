"""Per-round summary of an `aic-sim run --games 1 --trace` stream (stdin):
adversary pieces on the board, pool, opposition, population, and what the
players played. Usage:

    aic-sim run CONFIG --games 1 --seed S --agent A --trace 2>/dev/null | python scripts/tune/trace_summary.py
"""

import collections
import json
import sys

ADV = ("drone", "outpost", "foundry", "cluster", "rogue_factory", "rogue_data_centre")


def main():
    tokens = collections.Counter()
    vars_ = {}
    plays = []
    rnd = 0
    ops = collections.Counter()

    def report(label):
        adv = {t: tokens[t] for t in ADV if tokens[t]}
        print(f"{label:<10} adv {adv} corr {tokens['corruption']} pool {vars_.get('adversary.pool')} "
              f"opp {vars_.get('opposition.points', 0)} pop {tokens['population']} "
              f"fac {tokens['factory']} plays {plays}")

    for line in sys.stdin:
        line = line.strip()
        if not line.startswith("{"):
            continue
        d = json.loads(line)
        if "action" in d:
            a = d["action"]
            if a.get("type") in ("play", "use"):
                plays.append(a.get("card"))
            continue
        e = d.get("event")
        if not e:
            continue
        t = e.get("type")
        if t == "tokens_changed":
            tokens[e["token"]] += e["delta"]
        elif t == "var_changed":
            vars_[e["key"]] = e["total"]
        elif t == "round_started":
            report(f"r{rnd} end")
            rnd = e["round"]
            plays = []
        elif t == "game_over" or t == "game_ended":
            report(f"r{rnd} END")
            print(json.dumps(e))
        elif t == "note":
            pass
    report(f"r{rnd} last")


if __name__ == "__main__":
    main()
