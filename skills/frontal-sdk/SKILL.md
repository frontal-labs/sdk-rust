---
name: frontal-sdk-rust
description: Guidance for building and reviewing integrations using the Frontal Rust SDK.
---

# Frontal Rust SDK

Use the published `frontal` crate. Create one `Frontal` client and access services through its feature-gated handles. The handles share one transport and use caller-defined Serde request and response types.

## Configuration

`Frontal::from_env()` reads `FRONTAL_API_KEY`, `FRONTAL_API_URL` (default `https://api.frontal.dev/v1`), `FRONTAL_ENV`, and `FRONTAL_DEBUG`. Use `Frontal::builder(api_key)` for explicit configuration. Rust does not load `.env` files automatically.

## Contracts and services

Derive HTTP methods and contract-relative paths from `contracts/sdk-endpoints.json` or `contracts/openapi/`. Do not invent operation routes or request/response fields. See [the architecture guide](../../docs/ARCHITECTURE.md), [the crate README](../../README.md), and service modules under `../../src/resources/`.
