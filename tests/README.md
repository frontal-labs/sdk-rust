# Tests

Run `cargo nextest run` for local unit and integration tests. The HTTP fixtures use `wiremock` at the transport layer and a fixed fake API key; they do not contact a live backend. `cargo test --doc` compiles the README examples through the crate-level `include_str!` documentation.

Coverage includes typed JSON requests, bearer authentication, request IDs, error classification, retry backoff, cursor pagination, SSE parsing, invalid client configuration, and key redaction.
