# API resources

`Frontal` exposes each supported service namespace as a handle method. The default feature enables all 17 handles; disable default features and enable only the services you use, such as `frontal = { version = "1", default-features = false, features = ["functions"] }`.

| Handle | API domain |
| --- | --- |
| `agents()` | Agent definitions, runs, and versions |
| `ai()` | AI generation and gateway operations |
| `functions()` | Function definitions, versions, deployments, and executions |
| `workflows()` | Workflow definitions, executions, tasks, approvals, and templates |
| `audit()`, `auth()`, `billing()` | Audit events, authentication, and billing |
| `blob()`, `connectors()`, `data()` | Files, integrations, and data operations |
| `governance()`, `lineage()`, `observability()` | Governance, lineage, and telemetry |
| `ontology()`, `pipelines()` | Ontology and pipelines operations |
| `schedules()`, `webhooks()` | Schedules and webhook operations |

## AI operations

The AI service provides typed methods for its contract-defined operations and generic passthrough for the rest. `chat_completions` takes a `ChatCompletionRequest` and returns a `ChatCompletionResponse`; `embeddings` takes an `EmbeddingsRequest` and returns an `EmbeddingsResponse`. `predict` accepts a caller-selected serializable request and response type, `predict_raw` streams raw bytes, and `predict_form` sends a multipart form. `models` and `model_defaults` expose gateway metadata.

```rust,ignore
# use frontal::{models::ai::ChatCompletionRequest, Frontal};
# async fn example(client: &Frontal) -> frontal::Result<()> {
let request = ChatCompletionRequest {
    model: "frontal-ai-fast".to_string(),
    messages: Vec::new(),
    ..ChatCompletionRequest::default()
};
let response = client.ai().chat_completions(&request).await?;
# let _ = response;
# Ok(())
# }
```

## Agents and workflows

Agents expose `list_agents`, `create_agent`, `health`, `get_agent`, `update_agent`, `delete_agent`, `rollback_agent`, run operations (`create_run`, `get_run`, `conversation`), and the generic `versions` and `runs` methods that take a caller-selected response type. `watch_run` streams server-sent events; it requires the `stream` feature.

Workflows expose definition operations (`list_workflows`, `create_workflow_record`, `add_workflow_version`, `get_workflow`, `update_workflow`, `delete_workflow`, `publish_workflow`, `archive_workflow`, `restore_workflow`), execution operations (`list_executions`, `create_execution`, `get_execution`, `execution_tasks`, `run_steps`), task operations (`get_task`, `retry_task`, `cancel_task`), approvals (`list_approvals`, `get_approval`, `approve`, `reject`), and templates (`list_templates`, `get_template`, `create_template`, `instantiate_template`).

## Functions

See the [Functions API guide](FUNCTIONS.md) for definitions, invocations, versions, executions, errors, and an end-to-end example.

## Endpoint methods

Each service handle exposes HTTP verb methods backed by the shared transport. Read methods include `get`, `get_with_query`, `get_with_bearer_auth`, and `get_bytes`. Write methods include `post`, `post_with_query`, `post_idempotent`, `post_idempotent_with_query`, `post_empty`, `post_empty_idempotent`, `post_bytes`, `post_form`, and `post_form_empty`. Update methods include `put` and `patch`. Delete methods include `delete`, `delete_with_query`, `delete_with_body`, and `delete_empty`. Helper methods include `paginate`, `poll_until`, and `events` (the latter requires the `stream` feature). Where the contract does not define the wire fields, methods and response decoding use caller-selected Serde types:

```rust,ignore
# use frontal::Frontal;
# async fn example(client: &Frontal) -> frontal::Result<()> {
#[derive(serde::Deserialize)]
struct Page {
    data: Vec<serde_json::Value>,
    #[serde(rename = "nextCursor", default)]
    next_cursor: Option<String>,
}
let page: Page = client
    .ontology()
    .get_with_query("ontology/objects/objects", &[("limit", "20")])
    .await?;
# let _ = page;
# Ok(())
# }
```

All 17 supported service handles validate every route against its service contract before sending. The endpoint inventory in `contracts/sdk-endpoints.json` and the OpenAPI snapshots remain authoritative; `cargo xtask contract` fails if a route is not supported by its service and HTTP operation. Run it after adding or changing endpoint-backed methods.

Query parameters use the API's documented names; for example, pagination methods pass `cursor` and `limit`. SSE streaming returns `ServerSentEvent` values from the `stream` feature. Raw response-body methods return bytes, and multipart uploads use `reqwest::multipart::Form`.

## Pagination, polling, and streaming

`Frontal::paginate` walks every cursor page of a response shaped as `{ data, next_cursor }` and returns the collected items:

```rust,ignore
# use frontal::Frontal;
# async fn example(client: &Frontal) -> frontal::Result<()> {
#[derive(serde::Deserialize)]
struct Row {
    id: String,
}
let rows: Vec<Row> = client
    .paginate("ontology/objects/objects", Some(100))
    .await?;
# let _ = rows;
# Ok(())
# }
```

It fails with `FrontalError::Pagination` if the server repeats a cursor. `Frontal::poll_until` polls a JSON resource until a readiness closure returns true or the attempt limit is reached. With the `stream` feature, `Frontal::events` and service streaming methods such as `agents().watch_run` decode server-sent events as `ServerSentEvent` streams. Configure the timeout on the client, and remember that a polling deadline cannot interrupt an in-flight request:

```rust,ignore
# use frontal::{Frontal, RetryPolicy};
# use std::time::Duration;
# async fn example() -> frontal::Result<()> {
let client = Frontal::builder("frt_...")
    .base_url("https://api.frontal.dev/v1/")?
    .timeout(Duration::from_secs(10))
    .retry_policy(RetryPolicy::default())
    .build()?;
# let _ = client;
# Ok(())
# }
```

Errors are categorized `FrontalError` variants with an `ApiError` payload that exposes `status`, `code`, `request_id`, and `retryable()`. Streaming POST requests are not retried by default because a connection failure can occur after the server has accepted the request.
