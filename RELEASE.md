# Releasing the Frontal Rust SDK

Release the single `frontal-sdk` package to crates.io using semver. Run `cargo publish --dry-run` from the repository root, then publish the package and create a matching protected Git tag.

Before release, update `CHANGELOG.md`, establish and document the supported Rust version range, review the generated contract matrix, and verify package metadata. Publishing automation is not enabled while this repository is a scaffold.
