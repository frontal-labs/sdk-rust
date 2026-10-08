# Single-crate Rust SDK restructure

## Summary

Replace the 20-member Cargo workspace with one `frontal-sdk` package and a conventional `src/` module tree. Keep contract snapshots authoritative and retain the 17 service namespaces without adding unimplemented endpoint methods.

## Context

All Rust source files under `crates/` are empty crate stubs. The root package is currently defined only as a workspace, while README files and release/CI metadata describe multiple published crates. The service names are already represented in `contracts/sdk-endpoints.json`.

## System impact

Cargo package metadata and the published unit of the SDK become the root `frontal-sdk` package. Shared API infrastructure, models, resources, and private utilities live under `src/`; service modules are the one source-code namespace per existing contract domain. Documentation and automation will target that package, while the contracts remain unchanged.

## Approach

Create `src/api`, `src/models`, `src/resources`, and private `src/utils` modules. Add one empty, documented Rust module per contract-backed service under both `models` and `resources`. Remove the obsolete `crates/` tree and update all references to the old workspace and crate publishing flow.

## Changes

- `Cargo.toml`, `Cargo.lock` — define and lock one `frontal-sdk` library package.
- `src/` — establish crate root, shared modules, and service model/resource namespaces.
- `crates/` — remove empty workspace crate scaffolding and per-crate README placeholders.
- `README.md`, `AGENTS.md`, `docs/`, `examples/`, `tests/`, `skills/`, `CODEOWNERS` — document the new layout and single-package conventions.
- `.github/workflows/` and generated `mcp.json` manifests — remove multi-crate assumptions and stale paths.

## Verification

- Run `cargo fmt --all -- --check` and `cargo check` for the new library structure.
- Run `python3 scripts/check_contracts.py` and `python3 scripts/generate_docs_manifest.py` to confirm contract readability and refresh documentation paths.
- Do not run the test suite as part of this structural change.
