# Frontal async worker starter

Use this starter for scheduled checks, queue consumers, or periodic synchronization work. The example uses the `agents` feature to poll the inventoried `GET /v1/agents/health` route at a configurable interval and exits cleanly on Ctrl-C.

```bash
FRONTAL_API_KEY=frt_... FRONTAL_POLL_SECONDS=30 cargo run
```

Set `FRONTAL_POLL_SECONDS` to a positive integer. This template intentionally leaves job scheduling and persistence to the application.
