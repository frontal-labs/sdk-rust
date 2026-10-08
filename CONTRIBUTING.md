# Contributing to the Frontal Rust SDK

## Set up

Install Rust 1.85 or newer (2021 edition). The crate's MSRV is declared in `Cargo.toml`; follow [`docs/ONBOARDING.md`](./docs/ONBOARDING.md) to prepare the toolchain and local environment.

## Make a change

- Put shared transport and error behavior in the core module.
- Put each service's models and operations in its corresponding module.
- Use the native Rust formatter, linter, build tool, and test runner.
- Keep contracts and generated reports synchronized when public endpoint coverage changes.
- Add API documentation and a runnable Rust example for each public operation.
- Record user-visible changes in `CHANGELOG.md` and use `type(scope): summary` commit subjects.

## Before opening a pull request

Run the commands in the root README and report any contract or public API changes. Do not include live credentials in tests or examples.
