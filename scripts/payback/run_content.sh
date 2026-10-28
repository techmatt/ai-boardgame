#!/usr/bin/env bash
# Run content-patch experiments (009), each with the binary built for its patch,
# one after another, into out/payback-009-content/<spec>/. Each argument is
# <patch>:<spec> (binary out/patch-bin/<patch>.exe, spec
# experiments/payback-009-content/<spec>.json), or just <patch> when they match.
# --after <file> waits for that file to exist first (a running experiment's report).
#
#   bash scripts/payback/run_content.sh [--after out/x/report.md] bld_all_m3 income_x2:income_x2+keep_cost0
set -uo pipefail
if [ "${1:-}" = "--after" ]; then
    until [ -f "$2" ]; do sleep 10; done
    shift 2
fi
for arg in "$@"; do
    patch=${arg%%:*}
    spec=${arg#*:}
    echo "$(date +%H:%M:%S) start $spec (out/patch-bin/$patch.exe)"
    "out/patch-bin/$patch.exe" experiment "experiments/payback-009-content/$spec.json" \
        --out "out/payback-009-content/$spec" > "out/payback-009-content-$spec.log" 2>&1
    echo "$(date +%H:%M:%S) done $spec exit $?"
done
