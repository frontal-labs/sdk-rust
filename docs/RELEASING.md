# Rust release checklist

1. Use Conventional Commit subjects and include a changelog-worthy commit for user-visible changes.
2. Wait for CI on `main` to pass, including MSRV, cross-platform nextest, SemVer, and contract checks.
3. Review the release-plz pull request for the version bump and generated changelog.
4. Merge the release pull request after required reviews and checks pass.
5. Confirm the tag-triggered workflow creates the crate archive, CycloneDX SBOM, provenance and SBOM attestations, then publishes the version to crates.io.

The workflow uses the protected `crates-io` GitHub Environment and OIDC trusted publishing; it does not use a long-lived registry token. The initial crate version must be published manually once before configuring the crates.io trusted publisher. For the required environment, GitHub App, tag ruleset, and registry setup, follow [the repository settings checklist](../.github/BRANCH_PROTECTION.md).
