# Frontal async worker starter

Use this starter for scheduled checks, queue consumers, or periodic synchronization work. The example polls the contract-backed `GET /v1/agents/health` endpoint at a configurable interval and exits cleanly on Ctrl-C.

```bash
FRONTAL_API_KEY=frt_... FRONTAL_POLL_SECONDS=30 cargo run
```

Set `FRONTAL_POLL_SECONDS` to a positive integer. This template intentionally leaves job scheduling and persistence to the application.
