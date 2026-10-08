# Rust testing strategy

Use Rust unit and integration tests with deterministic local HTTP servers or a small mock transport. Keep tests independent of live keys; live API checks must be explicitly opt-in.

Put focused unit tests alongside the source they exercise and cross-module integration tests under the root `tests/` directory. Cover request encoding, response decoding, API errors, retry safety, timeout behavior, pagination, and streaming as those features are implemented. No SDK behavior tests exist yet.
