---
name: rust-serde-contract-services
description: Add feature-gated Rust service operations and Serde types from Frontal API contracts.
---

# Rust Serde and Contract Services

Use this skill when adding or updating a service handle, operation, or caller-facing model in `sdk-rust`.

1. Verify method, path, and defined wire fields in `contracts/sdk-endpoints.json` and the matching OpenAPI snapshot. Never infer contract details from another language's SDK.
2. Keep a service under its matching `src/resources/` module and gate it consistently with the crate's Cargo features.
3. Add caller-facing structs under `src/models/` only for fields defined by the contract. Follow existing Serde rename/optional conventions and test serialized JSON against the wire shape.
4. Reuse shared request execution in `src/api/`; do not expose internal helpers as public API without a deliberate compatibility reason.
5. Add `wiremock` tests, a rustdoc example, and a user-facing example when adding service behavior. Run `cargo xtask contract`.

Keep the published `frontal` package and unpublished `xtask` workspace roles separate.
