@AGENTS.md

# Rust SDK repository instructions

Read `AGENTS.md` first for repository layout, project conventions, and contract rules. Read `README.md` and the relevant design docs before changing public SDK behavior.

## Contract and compatibility

Honor the pinned Rust toolchain, Rust 2021 edition, and MSRV 1.75. Keep shared transport in src/api, service handles in src/resources, contract-backed model fields only, and internal helpers private. Cover behavior with wiremock and rustdoc examples.

Do not add public endpoints, wire fields, or behavior unsupported by the committed contracts. Keep credentials, tokens, and customer data out of source, logs, fixtures, and examples. Make the smallest compatible change and update documentation/examples when public behavior changes.

## Skills

Use the matching skill in `.claude/skills/` when its topic applies. Skills are also linked from `.agents/skills/`; source files live in `skills/` and selected upstream sources are recorded in `skills/SOURCES.md`.

## Verification commands

Run only the commands relevant to the change; do not claim verification unless it was run.

```sh
cargo fmt --all -- --check
cargo build --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run
cargo test --doc
cargo xtask contract
```
