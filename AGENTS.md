# Frontal Rust SDK - Agent Instructions

## Repository scope

This repository is the Rust SDK. Runtime behavior and service operations are not implemented yet; the files here establish the package structure, language tooling, shared API contracts, and release conventions. Treat the committed contract snapshots as the source for endpoint shapes.

## Layout

- `src/api/` — shared API infrastructure such as transport, authentication, errors, retries, pagination, and streaming.
- `src/models/` — request and response types grouped by service.
- `src/resources/` — service-specific API operations for the 17 contract domains.
- `src/utils/` — private implementation helpers shared within the library.
- `contracts/openapi/` — shared OpenAPI snapshots.
- `contracts/sdk-endpoints.json` — service endpoint inventory for future conformance reporting.
- `docs/`, `examples/`, and `templates/` — Rust-specific developer material.

## Rust conventions

The repository is a single Cargo package named `frontal-sdk`. Keep shared runtime behavior in `api`, service models and operations in their matching modules, and expose fallible operations with `Result`. The package uses Rust 2021; set and document an MSRV before the first release.

- Use the Rust toolchain documented in the root README and document direct native commands.
- Do not invent public method names or model fields. Derive endpoint shapes from `contracts/`.
- Add docs and runnable examples when the matching API is implemented.
- Keep internal helpers private unless they are deliberately part of the SDK's public API.
- Never check in credentials or customer data.

## Key commands

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

The contract and docs index helpers are Python 3 standard-library maintenance scripts; they do not add Python as a runtime dependency for the SDK.
