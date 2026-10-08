# Rust onboarding

1. Install stable Rust (2021 edition).
2. Clone this repository and install or resolve its dependencies using the Rust-native toolchain.
3. Run the format, lint, build, and test commands in the root README.
4. Review `AGENTS.md`, `docs/ARCHITECTURE.md`, and `contracts/README.md`.
5. Select a service module under `src/resources/` and compare its planned operations with `contracts/sdk-endpoints.json`.
6. Add implementation, Rust tests, documentation, and examples together.

Install stable Rust with rustup and the rustfmt/clippy components. From the repository root, use `cargo check`, `cargo fmt --all -- --check`, and `cargo test`. Export environment variables in your shell; Rust does not load `.env` files automatically.
