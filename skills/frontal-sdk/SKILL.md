---
name: frontal-sdk-rust
description: Guidance for adding or reviewing integrations using the Frontal Rust SDK.
---

# Frontal Rust SDK

This repository is a scaffold. Check the matching module under `src/resources/` and its models under `src/models/` before using an operation; an endpoint in `contracts/` does not guarantee a public method exists. Follow the idioms and toolchain documented in this repository.

## Configuration

The client configuration will use `FRONTAL_API_KEY` (`frt_...`) and `FRONTAL_API_URL` (default `https://api.frontal.dev/v1`) when implemented. See the root `.env.example`; Rust does not load `.env` files automatically.

## Modules

See [the architecture guide](../../docs/ARCHITECTURE.md) and the source modules under `../../src/`.
