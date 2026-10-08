# Rust testing strategy

Use Rust unit and integration tests with deterministic local HTTP servers or a small mock transport. Keep tests independent of live keys; live API checks must be explicitly opt-in. The client integration tests use a one-request local TCP server and a fixed test token.

Put focused unit tests alongside the source they exercise and cross-module integration tests under the root `tests/` directory. Cover request encoding, response decoding, API errors, retry safety, timeout behavior, pagination, and streaming as those features are implemented. Current tests cover auth, query encoding, JSON request/response handling, API errors, empty responses, and invalid configuration/path handling.
