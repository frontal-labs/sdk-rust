#!/usr/bin/env python3
"""Validate that committed OpenAPI and endpoint contract snapshots are readable."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FILES = [
    ROOT / "contracts/sdk-endpoints.json",
    ROOT / "contracts/references/typescript-sdk-endpoints.json",
    ROOT / "contracts/coverage-floor.json",
    ROOT / "contracts/openapi/api.openapi.json",
    ROOT / "contracts/openapi/ai.openapi.generated.json",
    ROOT / "contracts/openapi/manifest.json",
]
for path in FILES:
    if not path.is_file():
        raise SystemExit(f"missing contract file: {path.relative_to(ROOT)}")
    with path.open(encoding="utf-8") as stream:
        json.load(stream)

with (ROOT / "contracts/sdk-endpoints.json").open(encoding="utf-8") as stream:
    endpoint_inventory = json.load(stream)
with (ROOT / "contracts/references/typescript-sdk-endpoints.json").open(
    encoding="utf-8"
) as stream:
    typescript_reference = json.load(stream)
if endpoint_inventory != typescript_reference:
    raise SystemExit("Rust endpoint inventory differs from the frozen TypeScript reference")

manifest_path = ROOT / "contracts/openapi/manifest.json"
with manifest_path.open(encoding="utf-8") as stream:
    manifest = json.load(stream)

for contract in manifest["contracts"].values():
    snapshot = ROOT / "contracts/openapi" / contract["file"]
    digest = hashlib.sha256(snapshot.read_bytes()).hexdigest()
    if digest != contract["sha256"]:
        raise SystemExit(f"OpenAPI snapshot hash drift: {snapshot.relative_to(ROOT)}")

print(f"validated {len(FILES)} contract snapshots")
