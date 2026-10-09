Review the current diff and identify affected modules, public behavior, and relevant contract entries. This SDK is unreleased, so preserve one canonical v1 API and remove dead compatibility aliases instead of maintaining parallel versions. Derive public wire types from authoritative backend contracts; update snapshots and recorded hashes when those contracts change. Run relevant checks and report their results. When asked to implement or fix, make the necessary edits as well as running the checks.

```sh
cargo fmt --all -- --check
cargo build --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run
cargo test --doc
cargo xtask contract
```
