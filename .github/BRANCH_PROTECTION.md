# GitHub repository settings to apply

These settings require repository or organization permissions and cannot be enforced by committed files.

## Default branch ruleset

In **Settings → Rules → Rulesets**, create an active branch ruleset targeting the default branch (`main`):

- Require a pull request before merging, with at least one approval.
- Require review from Code Owners and dismiss stale approvals after new commits.
- Require all review conversations to be resolved.
- Require these status checks: `CI / quality`, `CI / msrv`, `CI / platform (ubuntu-latest)`, `CI / platform (macos-latest)`, `CI / platform (windows-latest)`, `CI / commitlint`, and `CI / semver`. Also require `CodeQL / analyze` and `Dependency review / review` when the repository is public or private-repository Code Security is enabled with `CODE_SECURITY_ENABLED=true`; both jobs skip otherwise.
- Require branches to be up to date before merging.
- Require linear history; block force pushes and branch deletion.
- Do not allow bypasses except for a documented break-glass administrator.

Check names come from the workflow and job names. GitHub may show matrix checks with their runner in parentheses.
Before enabling required Code Owner reviews, replace the existing `@sdk-team/core` and `@sdk-team/maintainers` entries in `CODEOWNERS` with valid users or teams that have write access to this repository.

## Release tag ruleset

Create an active tag ruleset for `v*` that prevents tag updates and deletion. Allow creation only to the release automation app and designated maintainers.

## Environment and crates.io trusted publisher

Create the GitHub Environment **`crates-io`**. Add required reviewers, prevent self-review, and limit deployments to tags matching `v*`. Configure the crates.io trusted publisher for owner `frontal-labs`, repository `sdk-rust`, workflow `.github/workflows/publish.yml`, and environment `crates-io`.

crates.io does not allow a crate's first version to be published through trusted publishing. Publish the initial `frontal` version once through a maintainer-controlled `cargo publish`, then configure the trusted publisher. The tag workflow detects an already-published version and skips a duplicate publish while still generating its SBOM and attestations.

The release-plz workflow uses a GitHub App token so release PRs run CI and release tags trigger the publish workflow. Create an app installed only on this repository with Contents read/write and Pull requests read/write permissions; store its numeric ID as repository variable `RELEASE_APP_ID` and its private key as repository secret `RELEASE_APP_PRIVATE_KEY`.

## Security features and action policy

In **Settings → Code security and analysis**, enable Code Security, Dependabot alerts, Dependabot security updates, secret scanning, and push protection where available. For a private repository, set the repository Actions variable `CODE_SECURITY_ENABLED=true` under **Settings → Secrets and variables → Actions → Variables** only after Code Security is enabled. This enables the CodeQL and dependency review jobs. Availability depends on repository visibility and the organization's GitHub plan; verify the features show as enabled.

Enable Discussions in **Settings → General → Features** if the issue-form contact link should route questions there.

In **Settings → Actions → General**, restrict allowed actions to trusted publishers and require actions to be pinned to full-length commit SHAs. Dependabot checks the pinned action references weekly.

If the repository is private, confirm the GitHub plan supports Code Security, Dependency Review, and artifact attestations. Without Code Security, CodeQL and dependency review are skipped; enable the feature and set `CODE_SECURITY_ENABLED=true` before requiring their checks. Artifact attestations are skipped for private repositories unless the plan supports them.
