# Rust developer guide

## Tooling

Use stable Rust (2021 edition) and the native commands listed in [`README.md`](../README.md). The SDK runtime is written in Rust. Two small repository maintenance scripts use Python 3's standard library to parse contract JSON and build the documentation index.

## Change placement

Keep transport concerns in `core`, SDK construction and service access in `sdk`, and endpoint behavior in its service module. Update API types, examples, and the generated migration matrix with each implemented operation.

## Contract workflow

`contracts/openapi/` and `contracts/sdk-endpoints.json` are shared input snapshots. Run `python3 scripts/check_contracts.py` to check the snapshot files and `python3 scripts/generate_docs_manifest.py` after changing Markdown documentation.
