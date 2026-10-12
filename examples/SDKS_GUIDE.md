# Frontal Rust SDK guide

The shared SDK client provides async JSON requests, bearer authentication, configurable API URLs and timeouts, retry-safe backoff, pagination, polling, streaming, and structured API errors. Each supported service handle exposes named methods for its operations over the committed route inventory. Methods use caller-selected Serde request and response types where the OpenAPI snapshots do not define wire fields; the agents, AI, Functions, and Workflows services also provide concrete models for their primary operations.

## Toolchain

Rust 1.75 or newer (2021 edition). See the root README for direct build and quality commands.

## Configuration

Set `FRONTAL_API_KEY` in the process environment, then construct the client with `Frontal::from_env()`. Configure alternate base URLs, timeouts, retry behavior, environment labels, and debug logging with `Frontal::builder`. Rust does not load `.env` files by default; inject secrets through your deployment environment.

Routes are relative to the configured base URL. Keep query parameters in `get_with_query` or `post_with_query`; use `encode_path_segment` when inserting a dynamic ID into a path. The default base URL already ends in `/v1`.

## Module map

- `api` — shared HTTP transport, authentication, and response handling.
- `models` — request and response types grouped by service.
- `resources` — agents, ai, audit, auth, billing, blob, connectors, data, governance, lineage, observability, ontology, pipelines, schedules, webhooks, and workflows.

See `docs/ARCHITECTURE.md`, the [examples](./README.md), and the [application templates](../templates/README.md) for usage. Use caller-defined Serde request and response types where the contracts do not define concrete fields; do not infer request fields from an endpoint name.
