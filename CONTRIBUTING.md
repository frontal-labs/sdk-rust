# Contributing to the Frontal Rust SDK

## Set up

Install Rust 1.75 or newer and Node.js 24 or newer. Run `npm ci` once to install the local Lefthook and commitlint tools. Follow [`docs/ONBOARDING.md`](./docs/ONBOARDING.md) for the Rust setup.

## Make a change

- Put shared transport and error behavior in the core module.
- Put each service's models and operations in its corresponding module.
- Keep contracts and generated reports synchronized when public endpoint coverage changes.
- Add API documentation and a runnable Rust example for each public operation.
- Record user-visible changes in `CHANGELOG.md` and use Conventional Commit subjects such as `feat(ai): add streaming support`.
- Do not include credentials, customer data, or populated `.env` files.

## Before opening a pull request

Run the checks listed in the root README. CI checks formatting, build, lint, tests, examples, docs, contract coverage, MSRV, SemVer, and supported operating systems. The pull request checklist covers changelog and documentation updates.
