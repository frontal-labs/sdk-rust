# Rust examples

Examples in this repository use Rust project conventions and avoid hard-coded credentials. `list_agents.rs` and `list_functions.rs` demonstrate environment-based authentication, service-scoped async requests, and JSON response handling. From the repository root, run them with `FRONTAL_API_KEY=... cargo run --example list_agents` or `FRONTAL_API_KEY=... cargo run --example list_functions`.

The route passed to a service handle is relative to `FRONTAL_API_URL`, which defaults to `https://api.frontal.dev/v1/`. Use the HTTP verb and path listed for that service in `contracts/sdk-endpoints.json`. Request and response types must match the API contract; use `serde_json::Value` where the shared snapshots do not define fields.
