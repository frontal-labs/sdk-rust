# Rust SDK roadmap

- [x] Establish the Rust 2021 crate, Rust 1.75 MSRV, community files, and CI toolchain.
- [x] Add one async client with shared auth, configuration, retries, pagination, polling, streaming, and errors.
- [x] Expose 16 supported service namespaces behind Cargo features with operation-specific methods mapped to the endpoint inventory; Sandbox is intentionally excluded.
- [x] Add wiremock tests, README doctests, examples, application templates, and contract drift checks.
- [x] Configure release-plz and cargo-semver-checks for the 1.0.0 release line.
- [ ] Publish 1.0.0 after the release pull request is merged and `CARGO_REGISTRY_TOKEN` is configured.
- [ ] Add endpoint-specific model types when the shared contracts define request and response fields.
