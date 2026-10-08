---
name: frontal-rust-sdk
description: Build async Rust integrations with the Frontal API using the frontal crate.
---

# Frontal Rust SDK

Use the `frontal` crate for typed async Rust access to Frontal services. Construct one `Frontal` client and get a service handle with methods such as `client.ai()`, `client.agents()`, or `client.workflows()`.

## Usage rules

- Read `contracts/sdk-endpoints.json` or the OpenAPI snapshots for supported paths and HTTP methods.
- Use a caller-defined Serde request or response type when a contract does not define a model schema. Do not invent model fields or operation routes.
- Pass the path relative to the configured base URL, including the API version only when it appears in the contract route inventory.
- Prefer `Frontal::from_env()` for `FRONTAL_API_KEY`, `FRONTAL_API_URL`, `FRONTAL_ENV`, and `FRONTAL_DEBUG`; use `Frontal::builder` for explicit settings.
- Use the shared pagination, polling, raw-body, and SSE helpers when they fit the endpoint behavior.
- Test HTTP behavior with `wiremock`; live credentials are not required for normal tests.

See [`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md) and [`README.md`](./README.md) for examples and crate setup.
