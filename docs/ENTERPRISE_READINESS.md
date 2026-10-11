# Enterprise readiness

The crate includes a shared async `reqwest` transport, validated client configuration, structured typed errors, bounded retries with a configurable `RetryPolicy`, cursor pagination handling, request-ID propagation, Serde boundary models, and methods for the complete supported endpoint inventory. It is tested offline with `wiremock`, gates every route against the committed contracts through `cargo xtask contract`, enforces MSRV 1.75 and public API stability in CI, and publishes through a GitHub OIDC crates.io workflow with CycloneDX SBOM generation and artifact attestations. It supports Linux, macOS, and Windows.

Remaining readiness work is:

- Obtain verified operation-specific request and response schemas from API owners. The committed API snapshot has no request-body schemas and uses generic object schemas for successful responses; the AI snapshot has no operation response schemas.
- Configure the crates.io trusted publisher for this repository, the `publish.yml` workflow, and the `crates-io` environment. crates.io does not allow a crate's first version through trusted publishing, so a maintainer must publish the initial `frontal` version once with `cargo publish`. See [PUBLISHING.md](./PUBLISHING.md).
- Review security controls, supported authentication flows, and service-specific operational guidance during release review. The first v1.0.0 release is unreleased; `cargo-semver-checks` starts enforcing the public API once the baseline tag exists.
