# Publishing to crates.io

The package is `frontal`. `release-plz` opens release pull requests from Conventional Commits and updates `CHANGELOG.md`; after a green CI run and merge, it creates a version tag and GitHub Release. The tag-triggered publish workflow verifies the tag against `Cargo.toml`, builds the crate, generates a CycloneDX SBOM and attestations, and publishes through crates.io trusted publishing.

No long-lived crates.io API token is stored in GitHub. The `crates-io` environment must have required reviewers and a tag deployment rule, and the crate must have a trusted publisher configured for `frontal-labs/sdk-rust`, `.github/workflows/publish.yml`, and that environment.

crates.io requires a maintainer to publish a crate's first version manually before trusted publishing can be configured. The publish workflow detects an existing version and skips duplicate uploads; it still packages and attests that tag.

See [the release runbook](./RELEASING.md) and [the repository settings checklist](../.github/BRANCH_PROTECTION.md) for the manual setup steps. `cargo-semver-checks` runs as part of CI and compares against the latest version tag after the first release exists.
