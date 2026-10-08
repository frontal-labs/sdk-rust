# Frontal Rust SDK

![Frontal Banner](./banner.png)

**Frontal client library for Rust.**

This repository contains the Rust SDK package, shared API contracts, developer documentation, examples, and templates.

> **Current status:** this is a repository scaffold. The runtime client, service operations, and endpoint models have not been implemented. The committed OpenAPI files and endpoint inventory are the source of truth for future Rust API work; an endpoint in those contracts does not imply that a public SDK operation exists.

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
```

The package is not published yet. Follow the setup instructions below to work from this checkout.

## Development

Requirements: stable Rust (2021 edition). Check the toolchain with `rustc --version` and `cargo --version`.

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

See [`CONTRIBUTING.md`](./CONTRIBUTING.md), [`docs/ONBOARDING.md`](./docs/ONBOARDING.md), and [`AGENTS.md`](./AGENTS.md).

## Planned configuration

These variables describe the intended configuration surface; the SDK does not yet include a configuration loader or runtime client.

| Variable | Purpose |
| --- | --- |
| `FRONTAL_API_KEY` | API key (`frt_...`) |
| `FRONTAL_API_URL` | API base URL; defaults to `https://api.frontal.dev/v1` |
| `FRONTAL_ENV` | Runtime environment (`development`, `test`, or `production`) |
| `FRONTAL_DEBUG` | Enable debug logging |

Rust does not load `.env` files by default. Read environment variables explicitly or inject them through your deployment environment. The committed [`.env.example`](./.env.example) is a reference only; do not commit a populated `.env` file.

## License

Apache-2.0. See [`LICENSE.md`](./LICENSE.md).
