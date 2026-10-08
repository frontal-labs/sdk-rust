# Rust SDK architecture

The published `frontal` crate provides one `Frontal` client and one shared reqwest connection pool. Service handles are typed namespaces over that transport, so authentication, timeouts, request IDs, retries, and error mapping stay consistent.

## Request flow

`Application → Frontal → typed service handle → validated contract-relative path → shared HTTP transport → Frontal API`

All 17 service handles expose operation-specific methods. Agents, AI, and Workflows also provide concrete models for their primary operations. Other operations use caller-selected Serde request and response types where the OpenAPI snapshots do not define wire fields. Before sending, every method checks that its service, verb, path template, and any required query values match `contracts/sdk-endpoints.json`.

## Configuration and transport

`Frontal::builder(api_key)` configures an HTTP(S) base URL, request timeout, retry policy, environment label, and debug flag. `Frontal::from_env()` reads `FRONTAL_API_KEY`, `FRONTAL_API_URL`, `FRONTAL_TIMEOUT` (milliseconds), `FRONTAL_ENV`, and `FRONTAL_DEBUG`. The default base URL is `https://api.frontal.dev/v1`, and the default timeout is 30 seconds. Rust does not load `.env` files.

The transport sends bearer authentication and an `x-request-id` for each logical request. GET, PUT, and DELETE may retry on connection/timeouts and transient HTTP statuses. POST is retried only through the JSON idempotency-key methods, which reuse the same key for every attempt; ordinary POST and PATCH calls are not retried automatically. `Retry-After` is honored up to the configured maximum delay. Multipart and streaming requests share auth and response error mapping.

## Errors and long-running operations

`FrontalError` implements `std::error::Error` and exposes category, API error code, request ID, status code, and retryability. API responses map authentication, rate limit, validation, server, and other status errors into distinct variants. The shared client also offers cursor pagination, bounded polling, raw response bytes, and feature-gated SSE streams.

## Services and features

The default `full` feature enables `ai`, `agents`, `audit`, `auth`, `billing`, `blob`, `connectors`, `data`, `governance`, `lineage`, `observability`, `ontology`, `pipelines`, `sandbox`, `schedules`, `webhooks`, `workflows`, and `stream`. Turn off default features to compile only the required service modules.

## Contracts

`contracts/sdk-endpoints.json` inventories 370 routes across 17 services. The committed OpenAPI snapshots directly describe 151 of those routes; the other inventory entries remain available through the shared verb transport because the snapshots are partial. `cargo xtask contract` checks service features, supported verbs, OpenAPI coverage, and the exact list of inventory routes absent from the snapshots, so route drift fails CI. `python3 scripts/check_contracts.py` verifies the snapshots and their recorded SHA-256 hashes. Neither command rewrites the contracts.
