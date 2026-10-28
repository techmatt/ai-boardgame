"""Content patches for the 009 payback sweep's levers that config can't reach
(card costs and income, tier-2 costs, the AI tracks' tier-2 unlock, antimatter).

Patches a *copy* of the repo, never the committed content, as 007's cards-t2cut did:

    python scripts/payback/patch_content.py <copy dir> <patch>[+<patch>...]

then build that copy into its own target dir and run experiments with that binary.
Results carry a different content hash, so their records replay only on a binary
built from the same patch. `--list` prints the patches.
"""

import json
import os
import sys

# 008's economy buildings (scripts/payback/summarize.py uses the same set).
ECONOMY = {
    "fab_line", "trade_office", "solar_field", "microgrid", "overclock_rig",
    "assembly_cluster", "grid_battery",
    "fusion_plant", "quantum_core", "megastructure", "antimatter_refinery",
}


def bld_all_m3(cards, ais):
    """Every building (tier 1, tier 2 and AI unique) costs 3 less, at least 1."""
    for c in cards:
        if c["kind"] == "building":
            c["cost"] = max(1, c["cost"] - 3)


def econ_half(cards, ais):
    """Economy buildings cost half (rounded up)."""
    for c in cards:
        if c["id"] in ECONOMY:
            c["cost"] = (c["cost"] + 1) // 2


def income_p1(cards, ais):
    """Each income passive on an economy building gives 1 more."""
    for c in cards:
        if c["id"] in ECONOMY:
            for p in c.get("passives", []):
                if p["op"] in ("income", "income_if_tags"):
                    p["amount"] += 1


def income_x2(cards, ais):
    """Each income passive on an economy building gives double."""
    for c in cards:
        if c["id"] in ECONOMY:
            for p in c.get("passives", []):
                if p["op"] in ("income", "income_if_tags"):
                    p["amount"] *= 2


def t2_half(cards, ais):
    """Tier-2 cards cost half their credits (rounded up)."""
    for c in cards:
        if c["tier"] == 2:
            c["cost"] = (c["cost"] + 1) // 2


def t2_noam(cards, ais):
    """Tier-2 cards need no antimatter."""
    for c in cards:
        if c["tier"] == 2 and "antimatter" in c:
            del c["antimatter"]


def unlock_m1(cards, ais):
    """The tier-2 mark moves one value earlier on each AI track (the Protector's to its
    first, always uncovered, value: tier 2 from the start)."""
    for a in ais:
        for t in a["tracks"]:
            vals = t["values"]
            for i, v in enumerate(vals):
                if v.get("tier2") and i > 0:
                    del v["tier2"]
                    vals[i - 1]["tier2"] = True
                    break


def antimatter(cards, ais):
    """More antimatter: the Refinery converts 2 energy (not 4), and each track's
    second-to-last value also gives 1 antimatter income."""
    for c in cards:
        if c["id"] == "antimatter_refinery":
            for p in c["passives"]:
                if p["op"] == "convert_upkeep":
                    p["amount"] = 2
    for a in ais:
        for t in a["tracks"]:
            if any(v.get("antimatter") for v in t["values"]):
                t["values"][-2]["antimatter"] = 1


def econ_noinsight(cards, ais):
    """Economy buildings cost no insight to play (so they don't take a card play)."""
    for c in cards:
        if c["id"] in ECONOMY:
            c["insight"] = 0


def insight_inc(cards, ais):
    """The income buildings (Fab Line, Trade Office, Solar Field, Microgrid, Fusion
    Plant, Megastructure) also give +1 insight income: income in the resource that
    caps card plays, not only in credits, which pile up unspent."""
    for c in cards:
        if c["id"] in ("fab_line", "trade_office", "solar_field", "microgrid",
                       "fusion_plant", "megastructure"):
            c["passives"].append({"op": "income", "resource": "insight", "amount": 1})


def insight_credit(cards, ais):
    """Weaker insight_inc: only the credit buildings (Fab Line, Trade Office,
    Megastructure) also give +1 insight income."""
    for c in cards:
        if c["id"] in ("fab_line", "trade_office", "megastructure"):
            c["passives"].append({"op": "income", "resource": "insight", "amount": 1})


def overclock_cheap(cards, ais):
    """Overclock Rig (+1 insight income) costs 6 and has no energy upkeep."""
    for c in cards:
        if c["id"] == "overclock_rig":
            c["cost"] = 6
            c.pop("upkeep", None)


PATCHES = {f.__name__: f for f in
           [bld_all_m3, econ_half, income_p1, income_x2, t2_half, t2_noam, unlock_m1, antimatter,
            econ_noinsight, insight_inc, insight_credit, overclock_cheap]}


def main():
    if "--list" in sys.argv:
        for n, f in PATCHES.items():
            print(f"{n}: {f.__doc__}")
        return
    root, spec = sys.argv[1], sys.argv[2]
    cp = os.path.join(root, "content", "cards.json")
    ap = os.path.join(root, "content", "ais.json")
    cards = json.load(open(cp, encoding="utf-8"))
    ais = json.load(open(ap, encoding="utf-8"))
    for name in spec.split("+"):
        PATCHES[name](cards, ais)
    for path, data in ((cp, cards), (ap, ais)):
        with open(path, "w", encoding="utf-8", newline="\n") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
            f.write("\n")


if __name__ == "__main__":
    main()
