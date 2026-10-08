---
paths:
  - "**/*.rs"
---

# Rust changes

Honor the pinned Rust toolchain, Rust 2021 edition, and MSRV 1.75. Keep shared transport in src/api, service handles in src/resources, contract-backed model fields only, and internal helpers private. Cover behavior with wiremock and rustdoc examples.

Follow the repository's `AGENTS.md` and `CLAUDE.md`. Use the relevant installed language skill under `.claude/skills/`. Add deterministic, offline tests for behavior changes, and run the repository's relevant format, lint, type/build, and contract checks.
