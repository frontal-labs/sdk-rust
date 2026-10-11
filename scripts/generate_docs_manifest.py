#!/usr/bin/env python3
"""Generate simple documentation indexes from Markdown files."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
excluded = (
    ".context",
    ".git",
    ".venv",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    "node_modules",
    "target",
    "build",
    "dist",
)
paths = sorted(
    p
    for p in ROOT.rglob("*.md")
    if not any(part in excluded for part in p.parts)
    and p.name != "PLAN.md"
)
documents = [{"path": p.relative_to(ROOT).as_posix(), "title": p.stem.replace("_", " ")} for p in paths]
manifest = {"name": "Frontal Rust SDK", "language": "Rust", "version": "1.0.0", "status": "implementation", "documents": documents}
content = json.dumps(manifest, indent=2) + "\n"
(ROOT / "docs/mcp.json").write_text(content, encoding="utf-8")
(ROOT / "mcp.json").write_text(content, encoding="utf-8")
