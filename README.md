# Frontal Rust SDK

![Frontal Banner](./banner.png)

**Frontal client library for Rust.**

This repository contains the Rust SDK package, shared API contracts, developer documentation, examples, and templates.

> **Current status:** the shared async HTTP client, authentication, JSON transport, and structured API errors are implemented. Service-specific operations and endpoint models are still being added. The committed OpenAPI files and endpoint inventory are authoritative; an endpoint in those contracts does not imply that a public SDK operation exists.

## Repository map

| Path | Purpose |
| --- | --- |
| `src/` | Rust library source, organized into API infrastructure, models, resources, and private utilities |
| `contracts/` | OpenAPI snapshots, endpoint inventory, and conformance reports |
| `docs/` | Rust architecture, onboarding, testing, and release guidance |
| `examples/` | Rust integration guide and future runnable examples |
| `templates/` | Rust application starter layouts |
| `scripts/` | Contract and documentation maintenance utilities |
| `.github/` | Rust CI, security analysis, and contribution templates |

## Install

Add this to `Cargo.toml` after the first crates.io release:

```toml
[dependencies]
frontal-sdk = "0.1"
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The package is not published yet. Tokio is the async runtime used by the example, and `serde_json` provides a convenient dynamic response type. Applications can use their own Serde types and runtime.

## Development

Requirements: Rust 1.85 or newer (2021 edition). The crate forbids unsafe code and enables Clippy's `all` lint group. Check the toolchain with `rustc --version` and `cargo --version`.

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

## Use the client

The async client reads an API key explicitly or from `FRONTAL_API_KEY`, and accepts caller-defined Serde types for endpoint payloads while endpoint schemas remain contract-driven.

```rust,no_run
use frontal_sdk::{Client, Result};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;
    let agents: Value = client.get("agents").await?;
    println!("{agents:#}");
    Ok(())
}
```

See [`examples/list_agents.rs`](./examples/list_agents.rs) for a complete example and [`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md) for client behavior.

## Application templates

The [`templates/`](./templates/README.md) directory contains standalone CLI, async worker, and Axum web-service starters. Build all three against this checkout with `cargo check --manifest-path templates/Cargo.toml --workspace`.

See [`CONTRIBUTING.md`](./CONTRIBUTING.md), [`docs/ONBOARDING.md`](./docs/ONBOARDING.md), and [`AGENTS.md`](./AGENTS.md).

## Planned configuration

The client reads `FRONTAL_API_KEY` with `Client::from_env()`. Set the API URL and request timeout with `Client::builder`; the other variables describe planned configuration only.

| Variable | Purpose |
| --- | --- |
| `FRONTAL_API_KEY` | API key (`frt_...`) |
| `FRONTAL_API_URL` | API base URL; defaults to `https://api.frontal.dev/v1` |
| `FRONTAL_ENV` | Runtime environment (`development`, `test`, or `production`) |
| `FRONTAL_DEBUG` | Enable debug logging |

Rust does not load `.env` files by default. Read environment variables explicitly or inject them through your deployment environment. The committed [`.env.example`](./.env.example) is a reference only; do not commit a populated `.env` file.

## License

Apache-2.0. See [`LICENSE.md`](./LICENSE.md).
