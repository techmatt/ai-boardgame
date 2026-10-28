"""Write one experiment spec per content patch (009) into experiments/payback-009-content/.
Run each with the binary built for that patch (scripts/payback/build_patches.sh):

    out/patch-bin/<patch>.exe experiment experiments/payback-009-content/<patch>.json

    python scripts/payback/make_content_specs.py <patch> [<patch>=<set>;<set> ...]

`<patch>=a.b=1;c.d=2` adds config overrides on top of the patch (a combination).
"""

import json
import os
import sys

DOC = {
    "bld_all_m3": "every building costs 3 less",
    "econ_half": "economy buildings cost half",
    "income_p1": "economy buildings' income +1",
    "income_x2": "economy buildings' income doubled",
    "t2_half+t2_noam": "tier 2 costs half its credits and no antimatter",
    "unlock_m1": "the AI tracks' tier-2 unlock one value earlier",
    "antimatter": "Refinery converts 2 energy, tracks' second-to-last value +1 antimatter",
    "econ_noinsight": "economy buildings cost no insight",
    "insight_inc": "the income buildings also give +1 insight income",
    "insight_credit": "Fab Line, Trade Office and Megastructure also give +1 insight income",
    "overclock_cheap":"Overclock Rig (+1 insight) costs 6, no upkeep",
    "econ_noinsight+econ_half": "economy buildings cost no insight and half their credits",
    "econ_noinsight+income_x2": "economy buildings cost no insight and give double income",
}


def main():
    os.makedirs(os.path.join("experiments", "payback-009-content"), exist_ok=True)
    for arg in sys.argv[1:]:
        patch, _, sets = arg.partition("=")
        sets = [s for s in sets.split(";") if s]
        name = patch + ("+" + "+".join(s.split(".", 1)[1].replace("=", "") for s in sets) if sets else "")
        doc = " + ".join(DOC.get(p, p) for p in [patch]) + (f"; with {', '.join(sets)}" if sets else "")
        spec = {
            "name": f"payback-009-c-{name}",
            "note": f"009 content lever (separate binary, committed content untouched; "
                    f"scripts/payback/patch_content.py {patch}): {doc}. Engine agent against greedy, same seeds as payback-009-singles.",
            "configs": ["configs/baseline.json"],
            "variants": [{"name": name, "set": sets}],
            "difficulties": [0, 4],
            "agents": ["greedy", "engine"],
            "games": 96,
            "first_seed": 300000,
            "winprob": {"games": 0},
            "closeness": {"every": 0},
            "records": 2,
        }
        path = os.path.join("experiments", "payback-009-content", f"{name}.json")
        with open(path, "w", encoding="utf-8", newline="\n") as f:
            json.dump(spec, f, indent=2)
            f.write("\n")
        print(path)


if __name__ == "__main__":
    main()
