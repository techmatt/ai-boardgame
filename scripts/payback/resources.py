"""Retired: keys on the phase `adversary.ravage`, removed in 014, so on rules r6 it prints nothing useful.

Unspent resources by round from `aic-sim run --games 1 --trace` (009): each seat's
credits, energy, compute, antimatter and insight when the adversary phase begins
(after the action phase, before upkeep pays income), and what was played.

    aic-sim run CONFIG --games 1 --seed S --agent engine --trace 2>&1 | python scripts/payback/resources.py
"""

import json
import sys

RES = ("credits", "energy", "compute", "antimatter", "insight")


def main():
    have = {}
    plays = []
    rnd = 0
    shown = set()
    for line in sys.stdin:
        line = line.strip()
        if not line.startswith("{"):
            continue
        e = json.loads(line)
        act = e.get("action")
        if isinstance(act, dict) and act.get("type") in ("play", "draw", "keep"):
            plays.append(f"{act['seat']}:{act['type']}:{act.get('card', '')}")
        ev = e.get("event")
        if isinstance(ev, dict):
            if ev.get("type") == "round_started":
                rnd = ev["round"]
                plays = []
            if ev.get("type") == "resource_changed":
                have[(ev["seat"], ev["resource"])] = ev["total"]
            if ev.get("phase") == "adversary.ravage" and rnd not in shown:
                shown.add(rnd)
                row = " | ".join(
                    f"s{s} " + " ".join(f"{r[:2]} {have.get((s, r), 0)}" for r in RES) for s in (0, 1))
                print(f"r{rnd}: {row}  plays {plays}")


if __name__ == "__main__":
    main()
