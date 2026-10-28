"""Engine-payback summary (009): one row per variant and difficulty from experiment
result directories (games.csv + results.json), engine agent against greedy on the
same seeds.

    python scripts/payback/summarize.py out/payback-009-singles [more dirs...] [--base base]
        [--engine engine] [--greedy greedy] [--picks N]

Columns: win % of greedy and engine; engine minus greedy (paired, ± standard error,
exact sign-test p); the engine's buildings played, economy buildings standing at the
end and tier-2 cards played per game; the share of the engine's wins with 2+ economy
buildings standing, and its win rate with 2+ vs fewer; rounds; the seed's share of
win/loss variance (luck). With --picks, lists seeds where the engine won with 2+
economy buildings standing and greedy lost (candidate game records).
"""

import argparse
import csv
import json
import math
import os
import sys

# 008's economy buildings: income or economy effects, tier 1 and tier 2.
ECONOMY = {
    "fab_line", "trade_office", "solar_field", "microgrid", "overclock_rig",
    "assembly_cluster", "grid_battery",
    "fusion_plant", "quantum_core", "megastructure", "antimatter_refinery",
}


def econ_standing(build):
    n = 0
    for seat in build.split("|"):
        if ":" not in seat:
            continue
        cards = seat.split(":", 1)[1]
        n += sum(1 for c in cards.split("+") if c in ECONOMY)
    return n


def sign_p(a, b):
    """Two-sided exact sign test on a vs b discordant pairs."""
    n = a + b
    if n == 0:
        return 1.0
    k = min(a, b)
    p = sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n
    return min(1.0, 2 * p)


def load(d):
    rows = list(csv.DictReader(open(os.path.join(d, "games.csv"), encoding="utf-8")))
    res = json.load(open(os.path.join(d, "results.json"), encoding="utf-8"))
    return rows, res


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dirs", nargs="+")
    ap.add_argument("--engine", default="engine")
    ap.add_argument("--greedy", default="greedy")
    ap.add_argument("--picks", type=int, default=0)
    ap.add_argument("--json", default="")
    a = ap.parse_args()

    games = {}  # (variant, d, agent) -> {seed: row}
    luck = {}
    order = []
    for d in a.dirs:
        rows, res = load(d)
        for r in rows:
            key = (r["variant"], int(r["difficulty"]), r["agent"])
            games.setdefault(key, {})[int(r["seed"])] = r
            if r["variant"] not in order:
                order.append(r["variant"])
        for g in res["groups"]:
            luck[(g["variant"], g["difficulty"])] = g["luck_win"][0]

    out = []
    print("| variant | d | greedy win % | engine win % | engine − greedy | p | bld played | econ standing | tier 2 played | engine wins with 2+ econ | engine win % 2+ econ / fewer | rounds e / g | luck (win) |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    picks = []
    for v in order:
        for d in (0, 4):
            ge = games.get((v, d, a.greedy))
            en = games.get((v, d, a.engine))
            if not ge or not en:
                continue
            seeds = sorted(set(ge) & set(en))
            n = len(seeds)
            gw = [int(ge[s]["won"]) for s in seeds]
            ew = [int(en[s]["won"]) for s in seeds]
            diffs = [e - g for e, g in zip(ew, gw)]
            mean = sum(diffs) / n
            sd = math.sqrt(sum((x - mean) ** 2 for x in diffs) / (n - 1)) if n > 1 else 0.0
            se = sd / math.sqrt(n)
            only_e = sum(1 for x in diffs if x > 0)
            only_g = sum(1 for x in diffs if x < 0)
            p = sign_p(only_e, only_g)
            er = [en[s] for s in seeds]
            econ = [econ_standing(r["build"]) for r in er]
            bld = sum(float(r["buildings_played"]) for r in er) / n
            t2 = sum(float(r["tier2_played"]) for r in er) / n
            ec = sum(econ) / n
            wins = [i for i in range(n) if ew[i]]
            wins2 = sum(1 for i in wins if econ[i] >= 2)
            hi = [ew[i] for i in range(n) if econ[i] >= 2]
            lo = [ew[i] for i in range(n) if econ[i] < 2]
            wr_hi = 100 * sum(hi) / len(hi) if hi else float("nan")
            wr_lo = 100 * sum(lo) / len(lo) if lo else float("nan")
            rnd_e = sum(int(r["rounds"]) for r in er) / n
            rnd_g = sum(int(ge[s]["rounds"]) for s in seeds) / n
            lk = luck.get((v, d), float("nan"))
            row = dict(variant=v, difficulty=d, games=n, greedy_win=100 * sum(gw) / n,
                       engine_win=100 * sum(ew) / n, diff=100 * mean, diff_se=100 * se, p=p,
                       buildings_played=bld, econ_standing=ec, tier2_played=t2,
                       engine_wins_2plus_econ=(100 * wins2 / len(wins)) if wins else None,
                       engine_win_2plus=wr_hi, engine_win_fewer=wr_lo, n_2plus=len(hi),
                       rounds_engine=rnd_e, rounds_greedy=rnd_g, luck_win=lk)
            out.append(row)
            star = "**" if p < 0.05 else ""
            print(f"| {v} | {d} | {row['greedy_win']:.1f} | {row['engine_win']:.1f} | "
                  f"{star}{row['diff']:+.1f} ±{row['diff_se']:.1f}{star} | {p:.2f} | {bld:.2f} | {ec:.2f} | {t2:.2f} | "
                  f"{row['engine_wins_2plus_econ'] or 0:.0f}% | {wr_hi:.0f} / {wr_lo:.0f} (n {len(hi)}) | "
                  f"{rnd_e:.2f} / {rnd_g:.2f} | {100 * lk:.0f}% |")
            for i, s in enumerate(seeds):
                if ew[i] and not gw[i] and econ[i] >= 2:
                    picks.append((v, d, s, econ[i], er[i]["build"], er[i]["rounds"]))
    if a.picks:
        print("\nCandidate records (engine won with 2+ economy buildings standing, greedy lost):")
        for p_ in picks[: a.picks]:
            print(" ", *p_)
    if a.json:
        with open(a.json, "w", encoding="utf-8", newline="\n") as f:
            json.dump({"rows": out, "picks": picks}, f, indent=1)


if __name__ == "__main__":
    sys.exit(main())
