# Frontal Rust SDK - Agent Instructions

## Repository scope

This repository contains the hand-written async Rust SDK published as the `frontal` crate. The TypeScript SDK is the frozen behavior reference; the committed contracts remain authoritative for endpoint methods and paths. Do not change the public HTTP API or OpenAPI snapshots.

## Layout

- `src/api/` — shared transport, authentication, configuration, retries, pagination, polling, and streaming.
- `src/models/` — caller-facing model types grouped by service when wire fields are defined by contracts.
- `src/resources/` — feature-gated service handles for all nonempty contract domains.
- `src/errors.rs` — categorized SDK and API errors with status, code, request ID, and retryability.
- `contracts/openapi/` and `contracts/sdk-endpoints.json` — committed API contracts and service route inventory.
- `xtask/` — `cargo xtask contract` endpoint coverage gate.
- `docs/`, `examples/`, and `templates/` — Rust-specific developer material.

## Rust conventions

The repository is a Cargo workspace with one published package named `frontal` and an unpublished `xtask`. Keep runtime behavior shared in `api`, services in their matching modules, and fallible operations on `Result`. The crate uses Rust 2021 and MSRV 1.75.

- Use the root README commands and keep CI in format → build → lint → test → examples → docs → contract order.
- Do not invent public operation names or model fields. Use HTTP verb methods and route paths from `contracts/`; add concrete request/response structs only when the contract defines the fields.
- Add `wiremock` coverage, rustdoc examples, and an example when adding service behavior.
- Keep internal helpers private unless they are deliberately part of the public API.
- Never check in credentials or customer data.

## Key commands

```bash
cargo fmt --all -- --check
cargo build --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run
cargo test --doc
cargo xtask contract
```
