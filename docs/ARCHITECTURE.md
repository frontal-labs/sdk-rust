# Rust SDK architecture

The Rust SDK is one Cargo package with shared runtime infrastructure and service modules. This keeps authentication, transport policy, errors, retries, pagination, and streaming consistent across all service operations.

## Module layout

- `src/api/` owns shared HTTP behavior and protocol concerns.
- `src/models/` owns request and response types grouped by service.
- `src/resources/` owns service-specific API operations: agents, ai, audit, auth, billing, blob, connectors, data, governance, lineage, observability, ontology, pipelines, sandbox, schedules, webhooks, and workflows.
- `src/utils/` contains private implementation helpers.
- `contracts/` stores shared API snapshots and the Rust SDK's conformance reports.

## Rust mapping

The repository root is the `frontal-sdk` Cargo package. Service areas are Rust modules inside that package rather than independently published crates. The package uses Rust 2021. Set and document an MSRV before the first release.

Keep service models and operations in the matching service module. Do not duplicate authentication or transport defaults in service resources. Public method names and wire fields must follow the committed OpenAPI and endpoint snapshots.

## Intended request flow

`Application → SDK entry point → service resource → shared API infrastructure → Frontal API`

The runtime pieces in this flow are not implemented yet.

## Current implementation status

The source module layout and contract snapshots are present. No runtime client operations, endpoint models, or passing endpoint-coverage report are claimed yet.
