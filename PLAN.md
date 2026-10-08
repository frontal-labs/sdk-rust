# Async SDK client and application templates

## Summary

Implement an async, contract-neutral HTTP client for the Rust SDK, then provide runnable starter applications tailored to common Rust setups. The client provides reusable JSON requests, bearer authentication, configurable API URL and timeout, typed API errors, and deterministic local-server integration tests.

## Context

`src/api`, `src/models`, and `src/resources` currently contain only documented module placeholders. The OpenAPI and endpoint inventory are authoritative, but most endpoint request and response schemas are open JSON objects or omitted. This implementation therefore supplies the shared client foundation without fabricating service models or endpoint signatures.

## System impact

`Client` owns the reqwest connection pool and immutable configuration. Each request builds its URL from the configured API root, attaches the API key as a bearer token, serializes an optional body, and maps HTTP failures into a stable SDK error. Callers own endpoint-specific Serde types; the contract snapshots remain the source of truth for endpoint paths and fields.

## Changes

- `Cargo.toml` — add async HTTP, Serde, and error dependencies while preserving the declared MSRV.
- `src/api/` — implement client configuration and request transport.
- `src/constants.rs`, `src/global.rs`, `src/errors.rs`, `src/endpoints.rs` — centralize defaults, shared result type, structured errors, and validated endpoint paths.
- `src/lib.rs` — expose the supported client and error API.
- `tests/client.rs` — test auth headers, JSON encoding/decoding, status errors, query values, URL validation, and empty responses against local HTTP fixtures.
- `README.md`, `docs/ARCHITECTURE.md`, `docs/TESTING.md`, `examples/` — document setup and provide a runnable usage example.
- `templates/` — add a CLI, async polling worker, and Axum web-service starter in a nested workspace, with independent manifests and setup guides.
- `.github/workflows/ci.yml` — build, format, lint, and test each starter in CI.
- `PLAN.md` — track this implementation.

## Verification

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo doc --no-deps`
- `python3 scripts/check_contracts.py`
