# Rust SDK architecture

The Rust SDK is one Cargo package with shared runtime infrastructure and service modules. This keeps authentication, transport policy, errors, retries, pagination, and streaming consistent across all service operations.

## Module layout

- `src/api/` owns the reusable HTTP client and request behavior.
- `src/constants.rs` defines shared defaults such as the API URL and request timeout.
- `src/endpoints.rs` validates relative endpoint paths and joins them to the API URL.
- `src/errors.rs` defines SDK and API response errors; `src/global.rs` defines shared crate types.
- `src/models/` owns request and response types grouped by service.
- `src/resources/` owns service-specific API operations: agents, ai, audit, auth, billing, blob, connectors, data, governance, lineage, observability, ontology, pipelines, sandbox, schedules, webhooks, and workflows.
- `src/utils/` contains private implementation helpers.
- `contracts/` stores shared API snapshots and the Rust SDK's conformance reports.

## Rust mapping

The repository root is the `frontal-sdk` Cargo package. Service areas are Rust modules inside that package rather than independently published crates. The package uses Rust 2021 and declares Rust 1.85 as its minimum supported version (`rust-version` in `Cargo.toml`).

Keep service models and operations in the matching service module. Do not duplicate authentication or transport defaults in service resources. Public method names and wire fields must follow the committed OpenAPI and endpoint snapshots.

## Intended request flow

`Application → Client → contract-defined endpoint path → shared HTTP transport → Frontal API`

Service resource methods can be added as request and response schemas become stable in the contracts.

## Current implementation status

The shared async HTTP client supports bearer authentication, JSON requests and responses, configurable base URLs and timeouts, and structured HTTP errors. Service-specific request and response models and endpoint methods remain unimplemented where the contracts do not define stable wire shapes. The endpoint inventory and OpenAPI snapshots remain authoritative.

`Client` owns a reusable reqwest connection pool and applies authentication to each request. Callers provide endpoint paths and Serde payload types. API errors retain the status, optional request ID, parsed JSON body, and a readable message; API keys are redacted from `Debug` output.
