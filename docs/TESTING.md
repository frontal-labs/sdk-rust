# Rust testing strategy

Use Rust unit tests and `wiremock` integration fixtures at the HTTP transport. Tests must not require live Frontal credentials or a backend. Keep any live API smoke tests explicitly opt-in.

The client integration suite covers typed request/response encoding, bearer auth, request IDs, error classification, transient retries, pagination, SSE event decoding, invalid paths, and API key redaction. The README is included in crate-level rustdoc, so its Rust snippets compile as doctests.

Run `cargo nextest run` for unit and integration tests and `cargo test --doc` for README and API doctests. CI checks formatting, build, lint, tests, examples, docs, and contracts in that order. The template workspace is compile-checked and linted separately; it has no standalone test suite.
