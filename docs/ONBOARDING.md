# Rust onboarding

1. Install Rust 1.75 or newer.
2. Add the SDK and the small dependencies used by the quickstart: `cargo add frontal serde_json`, then `cargo add tokio --features macros,rt-multi-thread`.
3. Set `FRONTAL_API_KEY` in your process environment.
4. Copy the [README quickstart](../README.md#quickstart) into an async function and choose a response type matching the endpoint contract.
5. Use `Frontal::builder(api_key)` when you need a custom base URL, timeout, retry policy, or environment label.

From a checkout, use the ordered checks in the root README. The wiremock tests and README doctests do not require a live API key or backend. Rust does not load `.env` files automatically.
