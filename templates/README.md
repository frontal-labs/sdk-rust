# Rust application templates

These are small, complete Rust applications showing how to use `frontal-sdk` in different setups. They share one nested Cargo workspace so each example can be built against this checkout.

| Template | Use it for | Main pieces |
| --- | --- | --- |
| [`cli/`](./cli/) | One-shot developer and operations commands | Tokio, environment-based auth, typed JSON input/output |
| [`async-worker/`](./async-worker/) | Periodic background jobs | Tokio interval, health polling, graceful Ctrl-C shutdown |
| [`web-service/`](./web-service/) | A Rust HTTP service that proxies SDK calls | Axum router state, JSON responses, upstream error mapping |

## Build the templates

From the repository root:

```bash
cargo check --manifest-path templates/Cargo.toml --workspace
cargo test --manifest-path templates/Cargo.toml --workspace
```

Run one with a Frontal API key:

```bash
FRONTAL_API_KEY=frt_... cargo run --manifest-path templates/Cargo.toml -p frontal-cli-template -- list-agents
FRONTAL_API_KEY=frt_... cargo run --manifest-path templates/Cargo.toml -p frontal-worker-template
FRONTAL_API_KEY=frt_... cargo run --manifest-path templates/Cargo.toml -p frontal-web-template
```

The manifests use a local path dependency so the examples build against the SDK source in this repository. When copying a template into a standalone project after release, change that dependency to the published `frontal-sdk` version. The API key is read from the process environment; Rust does not load `.env` files automatically.
