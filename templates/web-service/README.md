# Frontal Axum service starter

Use this starter when a Rust HTTP service needs to call Frontal on behalf of its own routes. It enables only the `agents` SDK feature, keeps one cloneable client in Axum router state, exposes a local readiness route, and proxies agent-list responses from the upstream API.

```bash
FRONTAL_API_KEY=frt_... BIND_ADDR=127.0.0.1:3000 cargo run
curl http://127.0.0.1:3000/healthz
curl http://127.0.0.1:3000/agents
```

The server binds to loopback by default. Set `BIND_ADDR` explicitly when the service should be reachable from other hosts, and protect its routes with your application’s authentication and authorization.
