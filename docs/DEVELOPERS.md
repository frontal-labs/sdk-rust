# Rust developer guide

## Tooling

Use stable Rust (2021 edition) and the native commands listed in [`README.md`](../README.md). The SDK runtime is written in Rust. Two small repository maintenance scripts use Python 3's standard library to parse contract JSON and build the documentation index.

## Change placement

Keep transport concerns in `src/api`, SDK construction in `Frontal`, and service-scoped access in `src/resources`. Add model fields only when the contracts define them. Update examples and contract coverage with each added service behavior.

## Contract workflow

`contracts/openapi/` and `contracts/sdk-endpoints.json` are shared input snapshots. Run `cargo xtask contract` to check the service and route mapping; it fails if the reported OpenAPI overlap changes. Run `python3 scripts/check_contracts.py` to validate snapshot JSON and hashes. Do not edit OpenAPI snapshots as part of an SDK implementation change. After changing Markdown documentation, run `python3 scripts/generate_docs_manifest.py` to refresh the root and docs indexes.
