use std::collections::BTreeMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use frontal::models::ai::{ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use frontal::models::functions::{
    FunctionDefinition, FunctionExecutionListOptions, FunctionListOptions, FunctionPermission,
    FunctionRuntime,
};
use frontal::models::workflows::{CreateWorkflowRequest, WorkflowSpec, WorkflowStatus};
use frontal::{Frontal, FrontalError, RetryPolicy};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wiremock::{
    matchers::{
        body_json, header, header_exists, method, path, query_param, query_param_is_missing,
    },
    Mock, MockServer, Request, Respond, ResponseTemplate,
};

fn client_for(server: &MockServer) -> Frontal {
    let base_url = format!("{}/v1", server.uri());
    Frontal::builder("frt_test_key")
        .base_url(&base_url)
        .expect("valid mock server URL")
        .retry_policy(RetryPolicy {
            max_retries: 0,
            ..RetryPolicy::default()
        })
        .timeout(Duration::from_secs(5))
        .build()
        .expect("build client")
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Item {
    id: String,
}

#[derive(Serialize)]
struct CreateItem<'a> {
    name: &'a str,
}

#[tokio::test]
async fn service_handle_sends_bearer_auth_and_decodes_typed_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .and(header("authorization", "Bearer frt_test_key"))
        .and(header_exists("x-request-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id":"agent-1"})))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let agent: Item = client.agents().get("agents").await.expect("decode agent");
    assert_eq!(agent.id, "agent-1");
}

#[tokio::test]
async fn readme_quickstart_request_runs_against_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[],"pagination":{"cursor":"","hasMore":false}})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let agents = client
        .agents()
        .list_agents(&[])
        .await
        .expect("README quickstart request succeeds against a mock");
    assert!(agents.data.is_empty());
}

#[tokio::test]
async fn post_serializes_typed_json() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/agents"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({"name":"example"})))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"id":"agent-2"})))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let agent: Item = client
        .agents()
        .post("agents", &CreateItem { name: "example" })
        .await
        .expect("decode created agent");
    assert_eq!(agent.id, "agent-2");
}

#[tokio::test]
async fn ai_service_method_uses_contract_typed_models() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/ai/chat/completions"))
        .and(body_json(json!({
            "model":"frontal-test",
            "messages":[{"role":"user","content":"hello"}]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id":"completion-1",
            "object":"chat.completion",
            "created":1,
            "model":"frontal-test",
            "choices":[{
                "index":0,
                "message":{"role":"assistant","content":"hi"},
                "finishReason":"stop"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let request = ChatCompletionRequest {
        model: "frontal-test".to_owned(),
        messages: vec![ChatMessage {
            role: "user".to_owned(),
            content: Some("hello".to_owned()),
            ..ChatMessage::default()
        }],
        ..ChatCompletionRequest::default()
    };
    let response: ChatCompletionResponse = client_for(&server)
        .ai()
        .chat_completions(&request)
        .await
        .expect("decode chat completion");

    assert_eq!(response.choices[0].message.content.as_deref(), Some("hi"));
}

#[tokio::test]
async fn remaining_service_methods_use_their_contract_routes() {
    let server = MockServer::start().await;
    let json_response = || ResponseTemplate::new(200).set_body_json(json!({"ok":true}));

    for (request_method, request_path) in [
        ("GET", "/v1/audit/events"),
        ("POST", "/v1/auth/token"),
        ("GET", "/v1/auth/user"),
        ("GET", "/v1/billing/customers/customer1/usage"),
        ("GET", "/v1/blob/object/bucket1/file"),
        ("GET", "/v1/diagnostics"),
        ("GET", "/v1/data/schemas/schemas"),
        ("GET", "/v1/compliance/frameworks"),
        ("GET", "/v1/lineage/graph"),
        ("GET", "/v1/observability/dashboards"),
        ("POST", "/v1/ontology/engine/ontologies/infer-classes"),
        ("GET", "/v1/data/pipelines/capabilities"),
        ("POST", "/v1/workflows/cron/validate"),
        ("POST", "/v1/webhooks/hook1/rotate-secret"),
    ] {
        let mut mock = Mock::given(method(request_method)).and(path(request_path));
        if request_path == "/v1/auth/token" {
            mock = mock.and(query_param("grant_type", "password"));
        }
        if request_path == "/v1/auth/user" {
            mock = mock.and(header("authorization", "Bearer user-jwt"));
        }
        let response = if request_path == "/v1/blob/object/bucket1/file" {
            ResponseTemplate::new(200).set_body_bytes(b"file contents".to_vec())
        } else {
            json_response()
        };
        mock.respond_with(response).expect(1).mount(&server).await;
    }

    let client = client_for(&server);
    let _: serde_json::Value = client.audit().list_events(&[]).await.unwrap();
    let _: serde_json::Value = client
        .auth()
        .sign_in_with_password(&json!({}))
        .await
        .unwrap();
    let _: serde_json::Value = client
        .auth()
        .current_user_with_jwt("user-jwt")
        .await
        .unwrap();
    let _: serde_json::Value = client.billing().customer_usage("customer1").await.unwrap();
    assert_eq!(
        client
            .blob()
            .download_object("bucket1", "file")
            .await
            .unwrap(),
        b"file contents"
    );
    let _: serde_json::Value = client.connectors().diagnostics().await.unwrap();
    let _: serde_json::Value = client.data().list_schemas(&[]).await.unwrap();
    let _: serde_json::Value = client.governance().frameworks().await.unwrap();
    let _: serde_json::Value = client.lineage().graph(&[]).await.unwrap();
    let _: serde_json::Value = client.observability().list_dashboards().await.unwrap();
    let _: serde_json::Value = client.ontology().infer_classes(&json!({})).await.unwrap();
    let _: serde_json::Value = client.pipelines().pipeline_capabilities().await.unwrap();
    let _: serde_json::Value = client
        .schedules()
        .validate_cron(&json!({"cronExpression":"0 * * * *"}))
        .await
        .unwrap();
    let _: serde_json::Value = client.webhooks().rotate_secret("hook1").await.unwrap();
}

#[tokio::test]
async fn workflow_operations_use_contract_typed_models_and_wire_names() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/workflows"))
        .and(body_json(json!({
            "name": "Example",
            "slug": "example",
            "description": "typed workflow"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "workflow": {
                "workflowId": "wf-1",
                "tenantId": "tenant-1",
                "name": "Example",
                "slug": "example",
                "description": "typed workflow",
                "status": "DRAFT",
                "latestVersion": 0,
                "createdBy": "user-1",
                "updatedBy": "user-1",
                "createdAt": "2026-10-09T09:00:00Z",
                "updatedAt": "2026-10-09T09:00:00Z"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1/workflows/executions"))
        .and(body_json(json!({
            "workflow_id": "wf-1",
            "input": {"customer_id": "c-1"}
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "execution": {
                "execution_id": "exec-1",
                "tenant_id": "tenant-1",
                "workflow_id": "wf-1",
                "version_id": "version-1",
                "status": "RUNNING",
                "input": {"customer_id": "c-1"},
                "started_by": "user-1",
                "created_at": "2026-10-09T09:01:00Z"
            },
            "state": {
                "ready": ["step-1"],
                "completed": [],
                "failed": [],
                "total_steps": 1
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let workflows = client_for(&server).workflows();
    let workflow = workflows
        .create_workflow_record(&CreateWorkflowRequest {
            name: "Example".to_owned(),
            slug: "example".to_owned(),
            description: Some("typed workflow".to_owned()),
            actor: None,
        })
        .await
        .expect("decode workflow envelope");
    assert_eq!(workflow.workflow_id, "wf-1");
    assert_eq!(workflow.status, WorkflowStatus::Draft);

    let input = WorkflowSpec(BTreeMap::from([("customer_id".to_owned(), json!("c-1"))]));
    let execution = workflows
        .create_execution("wf-1", &input)
        .await
        .expect("decode start execution response");
    assert_eq!(execution.execution.execution_id, "exec-1");
    assert_eq!(execution.state.ready, ["step-1"]);
}

#[tokio::test]
async fn error_mapping_preserves_code_request_id_and_retryability() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("x-request-id", "req-123")
                .set_body_json(json!({"code":"rate_limited","message":"slow down"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let error = client_for(&server)
        .agents()
        .get::<Item>("agents")
        .await
        .expect_err("rate limit should fail");
    assert!(matches!(error, FrontalError::RateLimit(_)));
    assert_eq!(error.code(), Some("rate_limited"));
    assert_eq!(error.request_id(), Some("req-123"));
    assert_eq!(error.status_code(), Some(429));
    assert!(error.retryable());
}

#[tokio::test]
async fn error_mapping_categorizes_auth_validation_and_server_statuses() {
    let server = MockServer::start().await;
    for (route, status) in [("auth", 401), ("validation", 422), ("server", 500)] {
        Mock::given(method("GET"))
            .and(path(format!("/v1/{route}")))
            .respond_with(ResponseTemplate::new(status).set_body_json(json!({"message":"failed"})))
            .expect(1)
            .mount(&server)
            .await;
    }

    let client = client_for(&server);
    assert!(matches!(
        client.get::<serde_json::Value>("auth").await,
        Err(FrontalError::Auth(_))
    ));
    assert!(matches!(
        client.get::<serde_json::Value>("validation").await,
        Err(FrontalError::Validation(_))
    ));
    assert!(matches!(
        client.get::<serde_json::Value>("server").await,
        Err(FrontalError::Server(_))
    ));
}

struct FirstFailureResponder(Arc<AtomicUsize>);

impl Respond for FirstFailureResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        if self.0.fetch_add(1, Ordering::SeqCst) == 0 {
            ResponseTemplate::new(503)
        } else {
            ResponseTemplate::new(200).set_body_json(json!({"id":"agent-retried"}))
        }
    }
}

struct FailFirstNResponder {
    calls: Arc<AtomicUsize>,
    failures: usize,
    body: serde_json::Value,
}

impl Respond for FailFirstNResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        if self.calls.fetch_add(1, Ordering::SeqCst) < self.failures {
            ResponseTemplate::new(503)
        } else {
            ResponseTemplate::new(200).set_body_json(self.body.clone())
        }
    }
}

#[tokio::test]
async fn retries_transient_status_for_safe_get() {
    let server = MockServer::start().await;
    let calls = Arc::new(AtomicUsize::new(0));
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .respond_with(FirstFailureResponder(Arc::clone(&calls)))
        .expect(2)
        .mount(&server)
        .await;

    let base_url = format!("{}/v1", server.uri());
    let client = Frontal::builder("frt_test_key")
        .base_url(&base_url)
        .expect("valid mock URL")
        .retry_policy(RetryPolicy {
            max_retries: 1,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(2),
        })
        .build()
        .expect("build client");

    let agent: Item = client.agents().get("agents").await.expect("retry succeeds");
    assert_eq!(agent.id, "agent-retried");
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn functions_default_to_three_retries_without_changing_other_services() {
    let server = MockServer::start().await;
    let function_calls = Arc::new(AtomicUsize::new(0));
    let agent_calls = Arc::new(AtomicUsize::new(0));

    Mock::given(method("GET"))
        .and(path("/v1/functions/fn1"))
        .respond_with(FailFirstNResponder {
            calls: Arc::clone(&function_calls),
            failures: 3,
            body: function_resource_json("fn1"),
        })
        .expect(4)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .respond_with(FailFirstNResponder {
            calls: Arc::clone(&agent_calls),
            failures: 2,
            body: json!({"id": "agent-1"}),
        })
        .expect(3)
        .mount(&server)
        .await;

    let base_url = format!("{}/v1", server.uri());
    let client = Frontal::builder("frt_test_key")
        .base_url(&base_url)
        .expect("valid mock URL")
        .build()
        .expect("build client");

    assert_eq!(RetryPolicy::default().max_retries, 2);
    assert_eq!(
        client
            .functions()
            .get_function("fn1")
            .await
            .expect("Functions default retries succeed")
            .id,
        "fn1"
    );
    let agent: serde_json::Value = client
        .agents()
        .get("agents")
        .await
        .expect("other services retain the shared default");
    assert_eq!(agent["id"], "agent-1");
    assert_eq!(function_calls.load(Ordering::SeqCst), 4);
    assert_eq!(agent_calls.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn explicit_retry_policy_overrides_functions_service_default() {
    let server = MockServer::start().await;
    let calls = Arc::new(AtomicUsize::new(0));
    Mock::given(method("GET"))
        .and(path("/v1/functions/fn1"))
        .respond_with(FailFirstNResponder {
            calls: Arc::clone(&calls),
            failures: 1,
            body: function_resource_json("fn1"),
        })
        .expect(2)
        .mount(&server)
        .await;

    let base_url = format!("{}/v1", server.uri());
    let client = Frontal::builder("frt_test_key")
        .base_url(&base_url)
        .expect("valid mock URL")
        .retry_policy(RetryPolicy {
            max_retries: 1,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(2),
        })
        .build()
        .expect("build client");

    client
        .functions()
        .get_function("fn1")
        .await
        .expect("explicit retry policy succeeds");
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn idempotency_key_enables_post_retries_and_is_reused() {
    let server = MockServer::start().await;
    let calls = Arc::new(AtomicUsize::new(0));
    Mock::given(method("POST"))
        .and(path("/v1/agents"))
        .and(header("idempotency-key", "create-agent-42"))
        .and(body_json(json!({"name":"example"})))
        .respond_with(FirstFailureResponder(Arc::clone(&calls)))
        .expect(2)
        .mount(&server)
        .await;

    let base_url = format!("{}/v1", server.uri());
    let client = Frontal::builder("frt_test_key")
        .base_url(&base_url)
        .expect("valid mock URL")
        .retry_policy(RetryPolicy {
            max_retries: 1,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(1),
        })
        .build()
        .expect("build client");

    let agent: Item = client
        .agents()
        .post_idempotent("agents", "create-agent-42", &CreateItem { name: "example" })
        .await
        .expect("same idempotency key makes the POST retry safe");

    assert_eq!(agent.id, "agent-retried");
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn pagination_follows_cursor_until_the_last_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .and(query_param("limit", "1"))
        .and(query_param_is_missing("cursor"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[{"id":"agent-1"}],"next_cursor":"next"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .and(query_param("cursor", "next"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[{"id":"agent-2"}],"next_cursor":null})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let items: Vec<Item> = client_for(&server)
        .agents()
        .paginate("agents", Some(1))
        .await
        .expect("fetch both pages");
    assert_eq!(
        items
            .iter()
            .map(|item| item.id.as_str())
            .collect::<Vec<_>>(),
        ["agent-1", "agent-2"]
    );
}

#[tokio::test]
async fn service_handle_rejects_routes_from_another_service_before_sending() {
    let error = Frontal::new("frt_test_key")
        .expect("build client")
        .ai()
        .get::<Item>("agents")
        .await
        .expect_err("AI handle must reject an Agents route");

    assert!(matches!(error, FrontalError::InvalidPath(_)));
}

#[tokio::test]
async fn pagination_fails_when_the_server_repeats_a_cursor() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .and(query_param_is_missing("cursor"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[{"id":"agent-1"}],"next_cursor":"same"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .and(query_param("cursor", "same"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[{"id":"agent-2"}],"next_cursor":"same"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let error = client_for(&server)
        .agents()
        .paginate::<Item>("agents", None)
        .await
        .expect_err("a repeated cursor must stop pagination");
    assert!(matches!(error, FrontalError::Pagination(_)));
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn sse_stream_decodes_event_fields() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/agents/runs/run-1/stream"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_string("event: update\nid: evt-1\nretry: 1000\ndata: hello\n\n"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let mut events = client_for(&server)
        .agents()
        .events("agents/runs/run-1/stream")
        .await
        .expect("open event stream");
    let event = events
        .next()
        .await
        .expect("one SSE event")
        .expect("decode event");
    assert_eq!(event.event, "update");
    assert_eq!(event.id, "evt-1");
    assert_eq!(event.data, "hello");
    assert_eq!(event.retry, Some(Duration::from_millis(1000)));
}

#[tokio::test]
async fn invalid_configuration_and_unsafe_paths_are_rejected() {
    assert!(Frontal::new(" ").is_err());
    assert!(Frontal::builder("frt_key")
        .timeout(Duration::ZERO)
        .build()
        .is_err());
    assert!(Frontal::builder("frt_key").base_url("file:///tmp").is_err());

    let client = Frontal::new("frt_key").expect("build client");
    let error = client
        .agents()
        .get::<Item>("../other")
        .await
        .expect_err("reject traversal before sending");
    assert!(matches!(error, FrontalError::InvalidPath(_)));
}

#[test]
fn debug_output_redacts_api_key() {
    let client = Frontal::new("frt_private_value").expect("build client");
    let builder = Frontal::builder("frt_private_value");
    assert!(!format!("{client:?}").contains("frt_private_value"));
    assert!(!format!("{builder:?}").contains("frt_private_value"));
}

fn function_definition() -> FunctionDefinition {
    FunctionDefinition {
        name: "example".to_owned(),
        description: Some("Example function".to_owned()),
        runtime: FunctionRuntime::Nodejs22,
        entrypoint: "index.handler".to_owned(),
        source: Some("blob://functions/example".to_owned()),
        input_schema: Some(json_object(json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "customer.id": {
                    "type": "string",
                    "x-vendor/data": {"nested key": [1, true, null]}
                }
            },
            "x-plugin+details": {"preserved": true}
        }))),
        output_schema: Some(json_object(
            json!({"type": "object", "additionalProperties": true}),
        )),
        dependencies: Some(vec!["zod@3".to_owned()]),
        env_vars: Some(BTreeMap::from([("REGION".to_owned(), "eu".to_owned())])),
        secrets: Some(vec!["DATABASE_URL".to_owned()]),
        memory: Some(256),
        timeout: Some(30),
        permissions: Some(FunctionPermission {
            ontology: Some(vec!["Customer".to_owned()]),
            actions: Some(vec!["customer.read".to_owned()]),
        }),
    }
}

fn json_object(value: serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    value
        .as_object()
        .expect("test JSON value is an object")
        .clone()
}

fn function_resource_json(id: &str) -> serde_json::Value {
    json!({
        "id": id,
        "name": "example",
        "description": "Example function",
        "runtime": "nodejs22",
        "entrypoint": "index.handler",
        "status": "draft",
        "version": 1,
        "latestVersion": 2,
        "memory": 256,
        "timeout": 30,
        "permissions": {"ontology": ["Customer"], "actions": ["customer.read"]},
        "createdAt": "2026-10-09T10:00:00Z",
        "updatedAt": "2026-10-09T10:00:00Z"
    })
}

#[tokio::test]
async fn functions_crud_serializes_models_queries_and_escaped_ids() {
    assert_eq!(RetryPolicy::default().max_retries, 2);
    let server = MockServer::start().await;
    let definition = function_definition();
    let expected_definition = json!({
        "name": "example",
        "description": "Example function",
        "runtime": "nodejs22",
        "entrypoint": "index.handler",
        "source": "blob://functions/example",
        "inputSchema": {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "customer.id": {
                    "type": "string",
                    "x-vendor/data": {"nested key": [1, true, null]}
                }
            },
            "x-plugin+details": {"preserved": true}
        },
        "outputSchema": {"type": "object", "additionalProperties": true},
        "dependencies": ["zod@3"],
        "envVars": {"REGION": "eu"},
        "secrets": ["DATABASE_URL"],
        "memory": 256,
        "timeout": 30,
        "permissions": {"ontology": ["Customer"], "actions": ["customer.read"]}
    });

    Mock::given(method("POST"))
        .and(path("/v1/functions"))
        .and(header("authorization", "Bearer frt_test_key"))
        .and(body_json(expected_definition.clone()))
        .respond_with(ResponseTemplate::new(201).set_body_json(function_resource_json("fn-1")))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/functions"))
        .and(query_param("cursor", "after cursor/one"))
        .and(query_param("limit", "30"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "functions": [function_resource_json("fn-1")],
            "pagination": {"cursor": "next", "hasMore": true}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let escaped_path = "/v1/functions/pkg%2Fmodule%3Fentry";
    Mock::given(method("GET"))
        .and(path(escaped_path))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(function_resource_json("pkg/module?entry")),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("PATCH"))
        .and(path(escaped_path))
        .and(body_json(expected_definition))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(function_resource_json("pkg/module?entry")),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(escaped_path))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let functions = client.functions();
    let created = functions
        .create_function(&definition)
        .await
        .expect("create function");
    assert_eq!(created.id, "fn-1");
    let page = functions
        .list_functions(&FunctionListOptions {
            cursor: Some("after cursor/one".to_owned()),
            limit: Some(30),
        })
        .await
        .expect("list functions");
    assert_eq!(page.functions.len(), 1);
    assert_eq!(page.pagination.cursor.as_deref(), Some("next"));
    assert!(page.pagination.has_more);
    let existing = functions
        .get_function("pkg/module?entry")
        .await
        .expect("get function with escaped identifier");
    assert_eq!(existing.id, "pkg/module?entry");
    functions
        .update_function("pkg/module?entry", &definition)
        .await
        .expect("update function");
    functions
        .delete_function("pkg/module?entry")
        .await
        .expect("delete function");
}

#[tokio::test]
async fn function_versions_and_deployments_use_expected_routes_and_bodies() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/functions/fn%2Fone/versions"))
        .and(query_param("limit", "10"))
        .and(query_param_is_missing("cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "versions": [{"version": 3, "source": "blob://v3", "createdAt": "2026-10-09T10:00:00Z"}],
            "pagination": {"hasMore": false}
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/functions/fn%2Fone/versions/3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "version": 3,
            "source": "blob://v3",
            "createdAt": "2026-10-09T10:00:00Z",
            "publishedBy": "user-1"
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/functions/fn%2Fone/versions/3/publish"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "version": 3,
            "source": "blob://v3",
            "createdAt": "2026-10-09T10:00:00Z",
            "publishedBy": "user-1"
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/functions/fn%2Fone/versions/3/deploy"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/functions/fn%2Fone/versions/3/deployment/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "deployed",
            "details": {"region": "eu", "provider/extensions": {"ready": true}}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let functions = client_for(&server).functions();
    let versions = functions
        .list_versions(
            "fn/one",
            &FunctionListOptions {
                limit: Some(10),
                ..FunctionListOptions::default()
            },
        )
        .await
        .expect("list versions");
    assert_eq!(versions.versions[0].version, 3);
    assert_eq!(
        functions
            .get_version("fn/one", 3)
            .await
            .unwrap()
            .published_by
            .as_deref(),
        Some("user-1")
    );
    functions.publish_version("fn/one", 3).await.unwrap();
    functions.deploy_version("fn/one", 3).await.unwrap();
    let status = functions.deployment_status("fn/one", 3).await.unwrap();
    assert_eq!(status.status, "deployed");
    assert_eq!(
        status.details.unwrap()["provider/extensions"]["ready"],
        true
    );
}

#[tokio::test]
async fn function_invocations_and_executions_preserve_nested_json() {
    let server = MockServer::start().await;
    let input = json!({
        "functionId": "fn-1",
        "version": 2,
        "input": {"customer.id": "c-1", "nested/value": {"unknown+key": [1, {"space key": true}]}}
    });
    Mock::given(method("POST"))
        .and(path("/v1/functions/invoke"))
        .and(body_json(input.clone()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "executionId": "exec-sync",
            "result": {"result/key": {"nested key": [1, true, null]}},
            "status": "active"
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/functions/invoke-async"))
        .and(body_json(input))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({"executionId": "exec/one"})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/functions/executions/exec%2Fone"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "exec/one",
            "functionId": "fn-1",
            "version": 2,
            "status": "active",
            "input": {"nested/value": {"unknown+key": [1, {"space key": true}]}},
            "output": {"result/key": {"nested key": [1, true, null]}},
            "startedAt": "2026-10-09T10:00:00Z",
            "completedAt": "2026-10-09T10:00:01Z",
            "durationMs": 1000
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/functions/executions/exec%2Fone/result"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "executionId": "exec/one",
            "result": {"result/key": {"nested key": [1, true, null]}}
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/functions/executions"))
        .and(query_param("cursor", "page two"))
        .and(query_param("limit", "15"))
        .and(query_param("functionId", "fn/one"))
        .and(query_param("status", "active"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "executions": [], "pagination": {"cursor": "next", "hasMore": true}
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/functions/executions/exec%2Fone/cancel"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let functions = client_for(&server).functions();
    let invocation = frontal::models::functions::FunctionInvocationInput {
        function_id: "fn-1".to_owned(),
        version: Some(2),
        input: Some(json_object(json!({
            "customer.id": "c-1",
            "nested/value": {"unknown+key": [1, {"space key": true}]}
        }))),
    };
    let response = functions
        .invoke(&invocation)
        .await
        .expect("invoke function");
    assert_eq!(response.execution_id, "exec-sync");
    assert_eq!(
        response.result.unwrap()["result/key"]["nested key"][2],
        json!(null)
    );
    assert_eq!(
        functions
            .invoke_async(&invocation)
            .await
            .unwrap()
            .execution_id,
        "exec/one"
    );
    let execution = functions.get_execution("exec/one").await.unwrap();
    assert_eq!(
        execution.input.unwrap()["nested/value"]["unknown+key"][1]["space key"],
        true
    );
    let result = functions.get_execution_result("exec/one").await.unwrap();
    assert_eq!(result.result.unwrap()["result/key"]["nested key"][1], true);
    let page = functions
        .list_executions(&FunctionExecutionListOptions {
            cursor: Some("page two".to_owned()),
            limit: Some(15),
            function_id: Some("fn/one".to_owned()),
            status: Some("active".to_owned()),
        })
        .await
        .expect("list executions");
    assert!(page.executions.is_empty());
    functions.cancel_execution("exec/one").await.unwrap();
}

#[tokio::test]
async fn functions_errors_keep_the_shared_structured_error_details() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/functions"))
        .respond_with(
            ResponseTemplate::new(422)
                .insert_header("x-request-id", "req-functions-1")
                .set_body_json(
                    json!({"code": "INVALID_FUNCTION", "message": "runtime is unsupported"}),
                ),
        )
        .expect(1)
        .mount(&server)
        .await;

    let error = client_for(&server)
        .functions()
        .create_function(&function_definition())
        .await
        .expect_err("validation errors should use shared API error handling");
    assert!(matches!(error, FrontalError::Validation(_)));
    assert_eq!(error.status_code(), Some(422));
    assert_eq!(error.code(), Some("INVALID_FUNCTION"));
    assert_eq!(error.request_id(), Some("req-functions-1"));
    assert_eq!(error.body().unwrap()["message"], "runtime is unsupported");
}
