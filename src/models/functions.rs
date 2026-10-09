//! Request and response models for the Frontal Functions API.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Open JSON object used by function schemas and invocation payloads.
pub type FunctionJsonObject = Map<String, Value>;

/// Supported runtimes for a Frontal function.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FunctionRuntime {
    /// Node.js 20 runtime.
    #[serde(rename = "nodejs20")]
    Nodejs20,
    /// Node.js 22 runtime.
    #[serde(rename = "nodejs22")]
    Nodejs22,
    /// Python 3.11 runtime.
    #[serde(rename = "python311")]
    Python311,
}

/// Lifecycle state returned for a function or invocation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionStatus {
    /// The function has not been activated.
    Draft,
    /// The function is active.
    Active,
    /// The function is deprecated.
    Deprecated,
    /// The function failed.
    Failed,
}

/// Ontology and action permissions granted to a function.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionPermission {
    /// Ontology entities the function may access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ontology: Option<Vec<String>>,
    /// Actions the function may invoke.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
}

/// Definition used to create or update a function.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDefinition {
    /// Human-readable function name.
    pub name: String,
    /// Optional function description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Runtime that executes this function.
    pub runtime: FunctionRuntime,
    /// Runtime entrypoint, such as `index.handler`.
    pub entrypoint: String,
    /// Optional source reference, such as a blob path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Optional open JSON Schema object. Nested and extension keys are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<FunctionJsonObject>,
    /// Optional open JSON Schema object. Nested and extension keys are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<FunctionJsonObject>,
    /// Optional package dependencies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    /// Optional environment variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<BTreeMap<String, String>>,
    /// Optional names of secrets made available to the function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<String>>,
    /// Optional memory limit in MB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<u32>,
    /// Optional execution timeout in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// Optional ontology and action permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FunctionPermission>,
}

/// Function resource returned by the API.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResource {
    /// Stable function identifier.
    pub id: String,
    /// Human-readable function name.
    pub name: String,
    /// Optional function description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Runtime that executes this function.
    pub runtime: FunctionRuntime,
    /// Runtime entrypoint.
    pub entrypoint: String,
    /// Current lifecycle state.
    pub status: FunctionStatus,
    /// Current function version.
    pub version: u64,
    /// Optional latest available version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<u64>,
    /// Optional memory limit in MB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<u32>,
    /// Optional execution timeout in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// Optional ontology and action permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FunctionPermission>,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
    /// Last update timestamp in RFC 3339 format.
    pub updated_at: String,
}

/// A published or deployable version of a function.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionVersion {
    /// Version number.
    pub version: u64,
    /// Source reference for this version.
    pub source: String,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
    /// Optional identity of the publisher.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_by: Option<String>,
}

/// Execution record returned by the Functions API.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionExecution {
    /// Stable execution identifier.
    pub id: String,
    /// Identifier of the invoked function.
    pub function_id: String,
    /// Function version used for this execution.
    pub version: u64,
    /// Execution state as defined by the Functions API.
    pub status: FunctionStatus,
    /// Optional input object. Arbitrary nested JSON keys are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<FunctionJsonObject>,
    /// Optional output object. Arbitrary nested JSON keys are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<FunctionJsonObject>,
    /// Optional error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Optional start timestamp in RFC 3339 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Optional completion timestamp in RFC 3339 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Optional execution duration in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

/// Input to a synchronous or asynchronous function invocation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionInvocationInput {
    /// Identifier of the function to invoke.
    pub function_id: String,
    /// Optional version to invoke; omitted to use the current version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
    /// Optional input object. Arbitrary nested JSON keys are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<FunctionJsonObject>,
}

/// Result returned by a synchronous invocation or execution result lookup.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionInvocationResult {
    /// Identifier of the execution that produced this result.
    pub execution_id: String,
    /// Optional result object. Arbitrary nested JSON keys are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<FunctionJsonObject>,
    /// Optional error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Optional function status associated with the result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionStatus>,
}

/// Response returned when an asynchronous invocation starts.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionAsyncInvocationResponse {
    /// Identifier used to track the asynchronous execution.
    pub execution_id: String,
}

/// Optional cursor and page-size parameters for function and version lists.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FunctionListOptions {
    /// Cursor returned by the previous page.
    pub cursor: Option<String>,
    /// Maximum number of resources to return.
    pub limit: Option<u32>,
}

/// Optional cursor, page-size, and filter parameters for execution lists.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FunctionExecutionListOptions {
    /// Cursor returned by the previous page.
    pub cursor: Option<String>,
    /// Maximum number of executions to return.
    pub limit: Option<u32>,
    /// Optional function identifier filter.
    pub function_id: Option<String>,
    /// Optional execution status filter.
    pub status: Option<String>,
}

/// Cursor metadata shared by Functions list responses.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionPagination {
    /// Optional cursor for the next page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Whether another page is available.
    pub has_more: bool,
}

/// Paginated function list response.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionListResponse {
    /// Functions returned for this page.
    pub functions: Vec<FunctionResource>,
    /// Pagination cursor and continuation flag.
    pub pagination: FunctionPagination,
}

/// Paginated function-version list response.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionVersionListResponse {
    /// Versions returned for this page.
    pub versions: Vec<FunctionVersion>,
    /// Pagination cursor and continuation flag.
    pub pagination: FunctionPagination,
}

/// Paginated execution list response.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionExecutionListResponse {
    /// Executions returned for this page.
    pub executions: Vec<FunctionExecution>,
    /// Pagination cursor and continuation flag.
    pub pagination: FunctionPagination,
}

/// Deployment status response for a function version.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionDeploymentStatus {
    /// Current deployment state.
    pub status: String,
    /// Optional service-defined deployment details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}
