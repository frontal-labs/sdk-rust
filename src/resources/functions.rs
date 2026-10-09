//! Service-scoped access for the `functions` API.

use super::ServiceClient;
use crate::{
    encode_path_segment,
    global::Result,
    models::functions::{
        FunctionAsyncInvocationResponse, FunctionDefinition, FunctionDeploymentStatus,
        FunctionExecution, FunctionExecutionListOptions, FunctionExecutionListResponse,
        FunctionInvocationInput, FunctionInvocationResult, FunctionListOptions,
        FunctionListResponse, FunctionResource, FunctionVersion, FunctionVersionListResponse,
    },
};

/// Type marker for the `functions` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct FunctionsService;

/// Typed access to the `functions` service.
pub type Functions = ServiceClient<FunctionsService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "functions";

impl ServiceClient<FunctionsService> {
    /// Create a function from a contract-shaped definition.
    pub async fn create_function(
        &self,
        definition: &FunctionDefinition,
    ) -> Result<FunctionResource> {
        let endpoint = self.validate("POST", "functions", &[])?;
        self.client.post(endpoint, definition).await
    }

    /// List functions, optionally selecting a cursor and page size.
    pub async fn list_functions(
        &self,
        options: &FunctionListOptions,
    ) -> Result<FunctionListResponse> {
        let query_values = pagination_query(&options.cursor, options.limit);
        let query = query_refs(&query_values);
        let endpoint = self.validate("GET", "functions", &query)?;
        self.client.get_with_query(endpoint, &query).await
    }

    /// Fetch a function by its identifier.
    pub async fn get_function(&self, function_id: &str) -> Result<FunctionResource> {
        let path = format!("functions/{}", encode_path_segment(function_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Update a function definition, creating a new version according to the API.
    pub async fn update_function(
        &self,
        function_id: &str,
        definition: &FunctionDefinition,
    ) -> Result<FunctionResource> {
        let path = format!("functions/{}", encode_path_segment(function_id));
        let endpoint = self.validate("PATCH", &path, &[])?;
        self.client.patch(endpoint, definition).await
    }

    /// Delete a function. The endpoint returns no response body.
    pub async fn delete_function(&self, function_id: &str) -> Result<()> {
        let path = format!("functions/{}", encode_path_segment(function_id));
        let endpoint = self.validate("DELETE", &path, &[])?;
        self.client.delete_empty(endpoint).await
    }

    /// List versions of a function, optionally selecting a cursor and page size.
    pub async fn list_versions(
        &self,
        function_id: &str,
        options: &FunctionListOptions,
    ) -> Result<FunctionVersionListResponse> {
        let path = format!("functions/{}/versions", encode_path_segment(function_id));
        let query_values = pagination_query(&options.cursor, options.limit);
        let query = query_refs(&query_values);
        let endpoint = self.validate("GET", &path, &query)?;
        self.client.get_with_query(endpoint, &query).await
    }

    /// Fetch one version of a function.
    pub async fn get_version(&self, function_id: &str, version: u64) -> Result<FunctionVersion> {
        let path = version_path(function_id, version);
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Publish a function version.
    pub async fn publish_version(
        &self,
        function_id: &str,
        version: u64,
    ) -> Result<FunctionVersion> {
        let path = format!("{}/publish", version_path(function_id, version));
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post(endpoint, &EmptyBody {}).await
    }

    /// Deploy a function version. The endpoint returns no response body.
    pub async fn deploy_version(&self, function_id: &str, version: u64) -> Result<()> {
        let path = format!("{}/deploy", version_path(function_id, version));
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post_empty(endpoint, &EmptyBody {}).await
    }

    /// Fetch the deployment status of a function version.
    pub async fn deployment_status(
        &self,
        function_id: &str,
        version: u64,
    ) -> Result<FunctionDeploymentStatus> {
        let path = format!("{}/deployment/status", version_path(function_id, version));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Invoke a function and wait for its result.
    pub async fn invoke(
        &self,
        input: &FunctionInvocationInput,
    ) -> Result<FunctionInvocationResult> {
        let endpoint = self.validate("POST", "functions/invoke", &[])?;
        self.client.post(endpoint, input).await
    }

    /// Start a function invocation and return its execution identifier.
    pub async fn invoke_async(
        &self,
        input: &FunctionInvocationInput,
    ) -> Result<FunctionAsyncInvocationResponse> {
        let endpoint = self.validate("POST", "functions/invoke-async", &[])?;
        self.client.post(endpoint, input).await
    }

    /// Fetch an execution record by its identifier.
    pub async fn get_execution(&self, execution_id: &str) -> Result<FunctionExecution> {
        let path = format!("functions/executions/{}", encode_path_segment(execution_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Fetch the result of an execution.
    pub async fn get_execution_result(
        &self,
        execution_id: &str,
    ) -> Result<FunctionInvocationResult> {
        let path = format!(
            "functions/executions/{}/result",
            encode_path_segment(execution_id)
        );
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// List executions, optionally filtering by cursor, page size, function, and status.
    pub async fn list_executions(
        &self,
        options: &FunctionExecutionListOptions,
    ) -> Result<FunctionExecutionListResponse> {
        let query_values = execution_query(options);
        let query = query_refs(&query_values);
        let endpoint = self.validate("GET", "functions/executions", &query)?;
        self.client.get_with_query(endpoint, &query).await
    }

    /// Cancel an execution. The endpoint returns no response body.
    pub async fn cancel_execution(&self, execution_id: &str) -> Result<()> {
        let path = format!(
            "functions/executions/{}/cancel",
            encode_path_segment(execution_id)
        );
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post_empty(endpoint, &EmptyBody {}).await
    }
}

#[derive(serde::Serialize)]
struct EmptyBody {}

fn version_path(function_id: &str, version: u64) -> String {
    format!(
        "functions/{}/versions/{version}",
        encode_path_segment(function_id)
    )
}

fn pagination_query(cursor: &Option<String>, limit: Option<u32>) -> Vec<(&'static str, String)> {
    let mut query = Vec::with_capacity(2);
    if let Some(cursor) = cursor {
        query.push(("cursor", cursor.clone()));
    }
    if let Some(limit) = limit {
        query.push(("limit", limit.to_string()));
    }
    query
}

fn execution_query(options: &FunctionExecutionListOptions) -> Vec<(&'static str, String)> {
    let mut query = pagination_query(&options.cursor, options.limit);
    if let Some(function_id) = &options.function_id {
        query.push(("functionId", function_id.clone()));
    }
    if let Some(status) = &options.status {
        query.push(("status", status.clone()));
    }
    query
}

fn query_refs<'a>(query: &'a [(&'static str, String)]) -> Vec<(&'static str, &'a str)> {
    query
        .iter()
        .map(|(key, value)| (*key, value.as_str()))
        .collect()
}
