"""Regenerate configs/shared_row.json from configs/baseline.json: the same config with
the threat variant `shared_row` and the name `shared_row`, nothing else. The core test
`the_shared_row_config_is_the_baseline_with_the_shared_row_threats` checks the two agree.

    python scripts/configs/shared_row.py      # from the repo root
"""

import json

with open("configs/baseline.json", encoding="utf-8") as f:
    config = json.load(f)
config["name"] = "shared_row"
threats = [m for m in config["modules"] if m["module"] == "threats"]
if len(threats) != 1:
    raise SystemExit(f"baseline.json has {len(threats)} threats modules, expected 1")
threats[0]["variant"] = "shared_row"
with open("configs/shared_row.json", "w", encoding="utf-8", newline="\n") as f:
    f.write(json.dumps(config, indent=2, ensure_ascii=False) + "\n")
