# Functions API

The crate exposes the Functions service as `client.functions()`. It uses the shared `reqwest` transport and inherits its API key, base URL, and timeout: the API URL defaults to `https://api.frontal.dev/v1/`, and the timeout defaults to 30 seconds. The API key is sent as Bearer authentication. An explicitly configured `RetryPolicy` applies to every service; otherwise Functions defaults to three retries and other services keep the shared two-retry default.

```rust,ignore
# use frontal::{models::functions::FunctionListOptions, Frontal};
# async fn example(client: &Frontal) -> frontal::Result<()> {
let page = client.functions().list_functions(&FunctionListOptions::default()).await?;
println!("{page:#?}");
# Ok(())
# }
```

## Create and invoke

`FunctionDefinition` requires `name`, `runtime` (`FunctionRuntime::Nodejs20`, `FunctionRuntime::Nodejs22`, or `FunctionRuntime::Python311`), and `entrypoint`. Optional fields include `description`, `source`, `input_schema`, `output_schema`, `dependencies`, `env_vars`, `secrets`, `memory` in MB, `timeout` in seconds, and `permissions` with `ontology` and `actions` string arrays. Schema fields use `FunctionJsonObject`, so arbitrary nested JSON keys are preserved.

```rust,ignore
# use frontal::{
#     models::functions::{
#         FunctionDefinition, FunctionInvocationInput, FunctionJsonObject, FunctionRuntime,
#     },
#     Frontal, RetryPolicy,
# };
# async fn example() -> frontal::Result<()> {
let client = Frontal::builder("frt_your_api_key")
    .retry_policy(RetryPolicy {
        max_retries: 3,
        ..RetryPolicy::default()
    })
    .build()?;

let mut input_schema = FunctionJsonObject::new();
input_schema.insert("type".to_string(), serde_json::json!("object"));
input_schema.insert(
    "properties".to_string(),
    serde_json::json!({"name": {"type": "string"}}),
);

let definition = FunctionDefinition {
    name: "hello-world".to_string(),
    description: None,
    runtime: FunctionRuntime::Nodejs22,
    entrypoint: "index.handler".to_string(),
    source: None,
    input_schema: Some(input_schema),
    output_schema: None,
    dependencies: None,
    env_vars: None,
    secrets: None,
    memory: None,
    timeout: None,
    permissions: None,
};

let function = client.functions().create_function(&definition).await?;

let invocation = client
    .functions()
    .invoke(&FunctionInvocationInput {
        function_id: function.id.clone(),
        version: None,
        input: None,
    })
    .await?;
println!("{:#?}", invocation.result);
# Ok(())
# }
```

## Service methods

Methods take semantic path arguments; versions and executions are addressed with `function_id`, `version`, and `execution_id` arguments rather than sub-accessors:

| Group | Methods |
| --- | --- |
| Definitions | `create_function`, `list_functions`, `get_function`, `update_function`, `delete_function` |
| Versions | `list_versions`, `get_version`, `publish_version`, `deploy_version`, `deployment_status` |
| Executions | `invoke`, `invoke_async`, `get_execution`, `get_execution_result`, `list_executions`, `cancel_execution` |

List options are typed structs. `FunctionListOptions` takes `cursor` and `limit` for function and version lists; `FunctionExecutionListOptions` adds `function_id` and `status` filters for execution lists.

```rust,ignore
# use frontal::{models::functions::{FunctionExecutionListOptions, FunctionListOptions}, Frontal};
# async fn example(client: &Frontal) -> frontal::Result<()> {
let functions = client
    .functions()
    .list_functions(&FunctionListOptions {
        cursor: None,
        limit: Some(20),
        ..FunctionListOptions::default()
    })
    .await?;

let executions = client
    .functions()
    .list_executions(&FunctionExecutionListOptions {
        function_id: None,
        status: Some("succeeded".to_string()),
        ..FunctionExecutionListOptions::default()
    })
    .await?;
# let _ = (functions, executions);
# Ok(())
# }
```

Versions are addressed by number: `get_version(function_id, 3)`, `publish_version(function_id, 3)`, `deploy_version(function_id, 3)`, and `deployment_status(function_id, 3)`.

## Errors

Functions requests use the shared typed errors. HTTP 401 and 403 responses raise `FrontalError::Auth`; 422 raises `FrontalError::Validation`; 429 raises `FrontalError::RateLimit`; and 5xx responses raise `FrontalError::Server`. All other non-success statuses, including 400, 404, and 409, raise `FrontalError::Api`. Connection and timeout failures raise `FrontalError::Transport`. Every API error variant carries an `ApiError` that exposes `status`, `code`, `request_id`, `body`, and `retryable()`. The SDK does not automatically retry function write requests when replay could repeat an accepted operation; use `post_idempotent` on the shared transport when the server supports an `Idempotency-Key`.

The Functions routes are recorded in `contracts/sdk-endpoints.json` from the TypeScript Functions package. The committed OpenAPI snapshots do not contain these routes.
