# Frontal Rust SDK guide

This guide uses Rust terms and layout. Client construction and service operations are not available yet, so it intentionally contains no guessed method calls.

## Toolchain

Stable Rust (2021 edition). See the root README for direct build and quality commands.

## Configuration

Set `FRONTAL_API_KEY` to a key beginning with `frt_`. `FRONTAL_API_URL` defaults to `https://api.frontal.dev/v1`. Rust does not load `.env` files by default. Read environment variables explicitly or inject them through your deployment environment.

## Module map

- `api` — shared transport, authentication, errors, retries, and response handling.
- `models` — request and response types grouped by service.
- `resources` — agents, ai, audit, auth, billing, blob, connectors, data, governance, lineage, observability, ontology, pipelines, sandbox, schedules, webhooks, and workflows.
- `utils` — private implementation helpers.

See `docs/ARCHITECTURE.md` and the matching source module under `src/` for status. Public examples will be added when the matching Rust methods compile and are covered by tests.
