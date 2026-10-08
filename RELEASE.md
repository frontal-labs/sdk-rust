# Releasing the Frontal Rust SDK

The `frontal` crate uses `release-plz` for Conventional Commit based version bumps, changelog updates, git tags, and GitHub Releases. The workflow runs only after the matching `main` CI run succeeds. `cargo-semver-checks` is part of CI and blocks incompatible releases.

After a release pull request is merged, release-plz creates the `vX.Y.Z` tag and GitHub Release. The tag workflow builds the crate, generates a CycloneDX SBOM and GitHub artifact attestations, then publishes to crates.io through trusted publishing. `release-plz.toml` disables its direct registry publish to avoid duplicate uploads.

Before releasing, review the generated release pull request and changelog, check the version and MSRV, and confirm the CI, docs, examples, and contract checks are green. The publish workflow also runs `cargo package` on the release tag.

Repository setup requirements are in [the GitHub settings checklist](.github/BRANCH_PROTECTION.md). They include the release GitHub App, the protected `crates-io` environment, and the crates.io trusted publisher. The first crate version must be published once by a maintainer before crates.io trusted publishing can be configured.
