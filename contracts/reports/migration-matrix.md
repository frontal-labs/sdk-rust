# Rust SDK migration matrix

_Status: route-level support. This report measures source code in `sdk-rust` only._

| Metric | Value |
| --- | ---: |
| Supported endpoint entries | 383 |
| Contract endpoint entries | 386 |
| Route-level coverage | 100% of the 383 active Rust SDK routes |
| Direct OpenAPI operation overlap | 186 / 383 |

The frozen endpoint inventory contains 386 entries across 19 namespaces, including an empty React section and the excluded Sandbox routes. Sandbox is intentionally excluded from this Rust SDK, leaving 383 supported endpoint entries across 17 services. Every active inventory method is available through its feature-gated service handle and shared typed verb transport. Route-level support does not imply a concrete model type exists for every endpoint; use caller-defined Serde types where the contracts do not specify fields. Reports from other language SDKs are not evidence of this repository's coverage.
