# Rust release checklist

Release the single `frontal-sdk` package to crates.io using semver. Update the changelog, confirm the supported Rust version range, review `contracts/reports/migration-matrix.md`, and verify package metadata and artifact contents.

Run the Rust CI checks and `cargo publish --dry-run` from the repository root. Create the matching protected Git tag before publishing. Publishing automation is not enabled while this repository is a scaffold.
