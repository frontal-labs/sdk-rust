# Rust SDK migration matrix

_Status: route-level support. This report measures source code in `sdk-rust` only._

| Metric | Value |
| --- | ---: |
| Supported endpoint entries | 370 |
| Contract endpoint entries | 370 |
| Route-level coverage | 100% |
| Direct OpenAPI operation overlap | 151 / 370 |

Every inventory method is available through its feature-gated service handle and shared typed verb transport. Route-level support does not imply a concrete model type exists for every endpoint; use caller-defined Serde types where the contracts do not specify fields. Reports from other language SDKs are not evidence of this repository's coverage.
