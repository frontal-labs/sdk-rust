# Frontal Rust SDK

![Frontal Banner](./banner.png)

[![skills.sh](https://skills.sh/b/frontal-labs/sdk-rust)](https://skills.sh/frontal-labs/sdk-rust)

Async, typed Rust client for the Frontal API. The crate includes a shared `reqwest` transport, service-scoped clients, structured errors, retries, pagination, polling, and SSE streaming.

## Install

```sh
cargo add frontal serde_json
cargo add tokio --features macros,rt-multi-thread
```

## Quickstart

Set `FRONTAL_API_KEY`, then make the first request:

```rust,no_run
# #[cfg(feature = "agents")]
# async fn quickstart() -> frontal::Result<()> {
# use frontal::Frontal;
# let frontal = Frontal::from_env()?;
let agents = frontal.agents().list_agents(&[]).await?;
println!("{agents:#?}");
# Ok(())
# }
```

All 17 supported service handles expose operation-specific methods and validate each route against its service contract before sending. Methods use caller-selected Serde request and response types when the OpenAPI snapshots do not define the wire fields. The `agents`, `ai`, `functions`, and `workflows` services also include concrete models for their primary operations. Use `post_idempotent` for a JSON POST the server supports retrying with an `Idempotency-Key` header.

```rust,no_run
# async fn configure() -> frontal::Result<()> {
# use frontal::{Frontal, RetryPolicy};
let client = Frontal::builder("frt_...")
    .base_url("https://api.frontal.dev/v1")?
    .timeout(std::time::Duration::from_secs(10))
    .retry_policy(RetryPolicy::default())
    .build()?;
# let _ = client;
# Ok(())
# }
```

The `default` feature enables all 17 supported service namespaces. Applications can disable default features and enable only the services they use, such as `frontal = { version = "1", default-features = false, features = ["functions"] }`. SSE support is enabled by the `stream` feature.

## Configuration

`Frontal::from_env()` reads `FRONTAL_API_KEY`, `FRONTAL_API_URL`, `FRONTAL_TIMEOUT` (milliseconds), `FRONTAL_ENV`, and `FRONTAL_DEBUG`. The API URL defaults to `https://api.frontal.dev/v1`; the request timeout defaults to 30 seconds. Rust does not load `.env` files automatically. Use `Frontal::builder(api_key)` to set the URL, timeout, retry policy, environment label, or debug flag directly.

## Functions API

The Functions service uses the shared `Frontal` transport and inherits its API key, base URL, and timeout. An explicitly configured retry policy applies to every service; otherwise Functions defaults to three retries and other services keep the shared two-retry default. The API URL defaults to `https://api.frontal.dev/v1`, and the timeout defaults to 30 seconds. Function definitions, pagination, versions, executions, invocation payloads, and responses have concrete types under `frontal::models::functions`.

```rust,no_run
# async fn functions_example() -> frontal::Result<()> {
use frontal::{models::functions::FunctionListOptions, Frontal, RetryPolicy};
use std::time::Duration;

let client = Frontal::builder("frt_...")
    .base_url("https://api.frontal.dev/v1")?
    .timeout(Duration::from_secs(30))
    .retry_policy(RetryPolicy {
        max_retries: 3,
        ..RetryPolicy::default()
    })
    .build()?;
let page = client
    .functions()
    .list_functions(&FunctionListOptions {
        cursor: None,
        limit: Some(20),
    })
    .await?;
println!("{page:#?}");
# Ok(())
# }
```

Use `FunctionInvocationInput` with `FunctionJsonObject` for invocation input; the SDK retains arbitrary nested object keys in inputs, results, and JSON Schema values.

## Development

The crate uses Rust 2021 and MSRV 1.75. Run the repository checks with:

```bash
cargo fmt --all -- --check
cargo build --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run
cargo test --doc
cargo xtask contract
```

HTTP behavior is tested with `wiremock`, so the test suite does not require a Frontal backend. See [`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md), [`docs/ONBOARDING.md`](./docs/ONBOARDING.md), and [`CONTRIBUTING.md`](./CONTRIBUTING.md) for details.

## Agent skills

Install this repository's Rust-specific agent skills with the [skills CLI](https://skills.sh/docs/cli):

```bash
npx skills add frontal-labs/sdk-rust
```

## License

Apache-2.0. See [`LICENSE.md`](./LICENSE.md).
