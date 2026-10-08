# Rust SDK overview

The Rust SDK is published as one crate named `frontal`. Create a `Frontal` client and use service handles such as `client.ai()`, `client.agents()`, and `client.workflows()`; they share authentication, timeout, retry, and connection-pool settings.

Each handle provides typed Serde methods for the HTTP verbs in the endpoint inventory and validates its service, verb, path template, and required query values before sending. Pass the contract-relative path to the matching service handle and provide request and response types that match the API schema. Many inventory routes do not have request or response schemas in the committed OpenAPI snapshots, so use a caller-defined Serde type or `serde_json::Value` for those routes.

All 17 service features are enabled by default. Set `default-features = false` and enable the service features your application uses to omit other service modules. See the [README quickstart](../README.md#quickstart) for installation and a first call.
