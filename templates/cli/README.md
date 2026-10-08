# Frontal CLI starter

Use this starter for one-shot API tasks, local scripts, or an internal operations command. It demonstrates environment-based auth and JSON reads and writes without tying the project to a CLI framework. It enables only the `agents` SDK feature.

```bash
FRONTAL_API_KEY=frt_... cargo run -- list-agents
FRONTAL_API_KEY=frt_... cargo run -- create-agent "$REQUEST_JSON"
```

The create command forwards caller-provided JSON to `POST /v1/agents`. Set `REQUEST_JSON` to a body supported by your API; the shared contracts do not define a request schema for this route.
