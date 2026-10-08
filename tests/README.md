# Rust test suite

Put focused unit tests alongside the source they exercise and cross-module integration tests in this directory. Use local HTTP mocks and synthetic data; never require live credentials for the default test suite.

Run `cargo test` for the local endpoint-path unit tests, HTTP client integration tests, and documentation examples. The integration tests use only loopback fixtures and a fixed fake API key; they never call the live API.
