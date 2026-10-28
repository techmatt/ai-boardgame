#!/usr/bin/env bash
# Build one aic-sim per content patch (009), from a copy of the repo under out/,
# never touching the committed content. Each binary lands in out/patch-bin/<patch>.exe.
#
#   bash scripts/payback/build_patches.sh bld_all_m3 econ_half income_p1+econ_half ...
set -euo pipefail
root=$(pwd)
src=out/patch-src
mkdir -p "$src" out/patch-bin
cp Cargo.toml Cargo.lock "$src"/
rm -rf "$src/crates"
cp -r crates "$src"/
rm -rf "$src/crates/aic-wasm/pkg"
for p in "$@"; do
    rm -rf "$src/content"
    cp -r content "$src"/
    python scripts/payback/patch_content.py "$src" "$p"
    (cd "$src" && CARGO_TARGET_DIR="$root/out/patch-target" cargo build --release -p aic-sim -q)
    cp out/patch-target/release/aic-sim.exe "out/patch-bin/$p.exe"
    echo "built $p"
done
