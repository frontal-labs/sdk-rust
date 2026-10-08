---
name: frontal-sdk-rust
description: Build async Rust integrations with Frontal or change, test, and document the frontal crate.
---

# Frontal Rust SDK

Use this skill for downstream Rust applications and development of the `frontal` crate. Check actual symbols and feature flags in this checkout before writing code. The committed endpoint inventory and OpenAPI snapshots under `contracts/` define supported HTTP methods and paths; the TypeScript SDK is a behavior reference, not a replacement for Rust API contracts.

## Use the client

Create one `Frontal` client and reuse its service handles. `Frontal::from_env()` reads `FRONTAL_API_KEY`, `FRONTAL_API_URL`, `FRONTAL_ENV`, and `FRONTAL_DEBUG`; use `Frontal::builder(api_key)` for explicit configuration. Rust does not load `.env` files automatically. Service handles are feature-gated, so check `Cargo.toml` and enable only the required service features when configuring dependencies.

Use caller-defined Serde request/response types when a route has no declared SDK model. Keep route paths relative to the configured base URL exactly as represented by the contract inventory; do not guess API version prefixes, operation names, or wire fields.

## Async and errors

- Preserve async execution and propagate failures with `Result`; use the crate's categorized error types rather than flattening errors into strings.
- Keep shared transport behavior in `src/api/`, domain handles in `src/resources/`, and contract-defined caller-facing types in `src/models/`.
- Use shared pagination and polling helpers for their intended response patterns. For SSE, consume the provided stream and propagate cancellation; do not buffer an unbounded stream.
- Preserve request IDs and status/code/retryability information in diagnostics while avoiding secrets and customer payloads.
- Retry only operations that are safe to repeat or have an explicit idempotency guarantee.

## Change the crate

The workspace contains the published `frontal` package and unpublished `xtask`. Keep runtime implementation shared in `src/api`, services in matching modules under `src/resources`, and internal helpers private unless they are intentionally public. Add concrete model structs only when the contract defines their fields. Keep the Rust 2021 edition and MSRV 1.75; avoid syntax or dependencies that exceed the MSRV.

When adding or changing service behavior, add `wiremock` coverage, a rustdoc example, and a user-facing example where it clarifies usage. Update docs and templates when the public integration pattern changes. Run `cargo xtask contract` after touching contract-backed endpoint coverage. Do not edit OpenAPI snapshots as part of ordinary SDK implementation.

## Tests and quality

Use deterministic `wiremock` tests; normal tests must not need a live backend or credentials. Cover serialization, headers, typed errors, pagination, polling, SSE cancellation, and feature-gated compilation as relevant. Keep CI order aligned with the README: format, build, lint, test, examples, docs, contract.

Useful checks from the root:

```bash
cargo fmt --all -- --check
cargo build --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run
cargo test --doc
cargo xtask contract
```

Consult `AGENTS.md`, `docs/ARCHITECTURE.md`, `README.md`, and `tests/README.md` for repo-specific details.
