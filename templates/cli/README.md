# Frontal CLI starter

Use this starter for one-shot API tasks, local scripts, or an internal operations command. It demonstrates environment-based auth and both JSON reads and writes without tying the project to a CLI framework.

```bash
FRONTAL_API_KEY=frt_... cargo run -- list-agents
FRONTAL_API_KEY=frt_... cargo run -- create-agent '{"name":"example"}'
```

The create command forwards caller-provided JSON to `POST /v1/agents`. Supply fields defined by the current API contract for your account.
