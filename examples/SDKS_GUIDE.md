# Frontal Rust SDK guide

The shared SDK client provides async JSON requests, bearer authentication, configurable API URLs and timeouts, and structured API errors. Service-specific models and operation methods are added only when their request and response shapes are supported by the committed contracts.

## Toolchain

Rust 1.85 or newer (2021 edition). See the root README for direct build and quality commands.

## Configuration

Set `FRONTAL_API_KEY` to your API key, then construct the client with `Client::from_env()`. Configure alternate base URLs and timeouts with `Client::builder`. Rust does not load `.env` files by default; inject secrets through your deployment environment.

## Module map

- `api` — shared HTTP transport, authentication, and response handling.
- `models` — request and response types grouped by service.
- `resources` — agents, ai, audit, auth, billing, blob, connectors, data, governance, lineage, observability, ontology, pipelines, sandbox, schedules, webhooks, and workflows.
- `utils` — private implementation helpers.

See `docs/ARCHITECTURE.md`, the [examples](./README.md), and the [application templates](../templates/README.md) for usage. Use caller-defined Serde types with the generic client until typed operation methods are available for a service.
