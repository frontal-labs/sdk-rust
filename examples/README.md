# Rust examples

Examples in this repository use Rust project conventions and avoid hard-coded credentials. `list_agents.rs` demonstrates environment-based authentication, a service-scoped async request, and JSON response handling. From the repository root, run it with `FRONTAL_API_KEY=... cargo run --example list_agents`.

The route passed to a service handle is relative to `FRONTAL_API_URL`, which defaults to `https://api.frontal.dev/v1`. Use the HTTP verb and path listed for that service in `contracts/sdk-endpoints.json`. Request and response types must match the API contract; use `serde_json::Value` where the shared snapshots do not define fields.
