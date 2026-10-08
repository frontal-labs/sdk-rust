# Rust application templates

These are small Rust applications showing how to use `frontal` in different setups. Each uses only the `agents` feature, and they share a nested Cargo workspace so they can build against this checkout.

| Template | Use it for | Main pieces |
| --- | --- | --- |
| [`cli/`](./cli/) | One-shot developer and operations commands | Tokio, environment-based auth, typed JSON input/output |
| [`async-worker/`](./async-worker/) | Periodic background jobs | Tokio interval, health polling, graceful Ctrl-C shutdown |
| [`web-service/`](./web-service/) | A Rust HTTP service that proxies SDK calls | Axum router state, JSON responses, upstream error mapping |

## Check the templates

From the repository root:

```bash
cargo check --manifest-path templates/Cargo.toml --workspace
cargo fmt --manifest-path templates/Cargo.toml --all -- --check
cargo clippy --manifest-path templates/Cargo.toml --workspace --all-targets -- -D warnings
```

Run one with a Frontal API key:

```bash
FRONTAL_API_KEY=frt_... cargo run --manifest-path templates/Cargo.toml -p frontal-cli-template -- list-agents
FRONTAL_API_KEY=frt_... cargo run --manifest-path templates/Cargo.toml -p frontal-worker-template
FRONTAL_API_KEY=frt_... cargo run --manifest-path templates/Cargo.toml -p frontal-web-template
```

The manifests use a local path dependency so the examples build against the SDK source in this repository. When copying a template into a standalone project after release, use `cargo add frontal --features agents`. The API key is read from the process environment; Rust does not load `.env` files automatically. Use request bodies and response types supported by the contract and your account.
