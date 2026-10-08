---
name: rust-async-api
description: Implement safe async HTTP client behavior and structured error propagation in the Frontal Rust SDK.
---

# Rust Async API

Use this skill when changing async transport, cancellation, retries, pagination, polling, or SSE behavior in `sdk-rust`.

- Preserve async execution; do not block a Tokio worker with synchronous I/O or sleeps.
- Propagate failures through `Result` and the crate's error types; do not discard status, code, request ID, or retryability by converting errors to strings.
- Ensure dropped/cancelled futures release response resources and stop stream consumption naturally.
- Keep shared transport policy in `src/api/` and call it from service handles rather than duplicating HTTP logic.
- Retry only when the method is safe to repeat or the caller has an idempotency mechanism. Honor configured limits and delays.
- Use `wiremock` tests for offline coverage of cancellation, error mapping, and stream behavior.

Maintain Rust 2021 and MSRV 1.75; consult `AGENTS.md` before adding dependencies or public helpers.
