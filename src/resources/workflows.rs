//! Service scoped access for the `workflows` API.

use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use super::ServiceClient;
use crate::{
    encode_path_segment,
    global::Result,
    models::workflows::{CreateWorkflowRequest, Workflow, WorkflowDefinition, WorkflowExecution},
};

/// Type marker for the `workflows` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct WorkflowsService;

/// Typed access to the `workflows` service.
pub type Workflows = ServiceClient<WorkflowsService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "workflows";

#[derive(Serialize)]
struct EmptyBody {}

#[derive(Serialize)]
struct CreateWorkflowVersion<'a> {
    spec: &'a WorkflowDefinition,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateExecution<'a> {
    workflow_id: &'a str,
    input: &'a BTreeMap<String, Value>,
}

#[derive(Serialize)]
struct ApprovalDecision<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,
}

#[derive(Serialize)]
struct CancelTask<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'a str>,
}

#[derive(Serialize)]
struct InstantiateTemplate<'a> {
    name: &'a str,
}

impl ServiceClient<WorkflowsService> {
    /// List workflows using supported filters and cursor values.
    pub async fn list_workflows<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("GET", "workflows", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Create a workflow resource. Add a version with [`Self::add_workflow_version`].
    pub async fn create_workflow_record(
        &self,
        request: &CreateWorkflowRequest,
    ) -> Result<Workflow> {
        let endpoint = self.validate("POST", "workflows", &[])?;
        self.client.post(endpoint, request).await
    }

    /// Add a version containing the workflow definition.
    pub async fn add_workflow_version(
        &self,
        workflow_id: &str,
        definition: &WorkflowDefinition,
    ) -> Result<Workflow> {
        let path = format!("workflows/{}/versions", encode_path_segment(workflow_id));
        let endpoint = self.validate("POST", &path, &[])?;
        self.client
            .post(endpoint, &CreateWorkflowVersion { spec: definition })
            .await
    }

    /// Fetch a workflow by identifier.
    pub async fn get_workflow(&self, workflow_id: &str) -> Result<Workflow> {
        let path = format!("workflows/{}", encode_path_segment(workflow_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Update a workflow definition.
    pub async fn update_workflow<B>(&self, workflow_id: &str, definition: &B) -> Result<Workflow>
    where
        B: Serialize + ?Sized,
    {
        let path = format!("workflows/{}", encode_path_segment(workflow_id));
        let endpoint = self.validate("PATCH", &path, &[])?;
        self.client.patch(endpoint, definition).await
    }

    /// Delete a workflow.
    pub async fn delete_workflow(&self, workflow_id: &str) -> Result<()> {
        let path = format!("workflows/{}", encode_path_segment(workflow_id));
        let endpoint = self.validate("DELETE", &path, &[])?;
        self.client.delete_empty(endpoint).await
    }

    /// Publish a workflow.
    pub async fn publish_workflow(&self, workflow_id: &str) -> Result<Workflow> {
        self.workflow_action(workflow_id, "publish").await
    }

    /// Archive a workflow.
    pub async fn archive_workflow(&self, workflow_id: &str) -> Result<Workflow> {
        self.workflow_action(workflow_id, "archive").await
    }

    /// Restore an archived workflow.
    pub async fn restore_workflow(&self, workflow_id: &str) -> Result<Workflow> {
        self.workflow_action(workflow_id, "restore").await
    }

    /// List workflow executions using supported filters and cursor values.
    pub async fn list_executions<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("GET", "workflows/executions", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Start a workflow execution.
    pub async fn create_execution(
        &self,
        workflow_id: &str,
        input: &BTreeMap<String, Value>,
    ) -> Result<WorkflowExecution> {
        let endpoint = self.validate("POST", "workflows/executions", &[])?;
        self.client
            .post(endpoint, &CreateExecution { workflow_id, input })
            .await
    }

    /// Fetch a workflow execution by identifier.
    pub async fn get_execution(&self, execution_id: &str) -> Result<WorkflowExecution> {
        let path = format!("workflows/executions/{}", encode_path_segment(execution_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Fetch tasks for an execution.
    pub async fn execution_tasks<T>(&self, execution_id: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!(
            "workflows/executions/{}/tasks",
            encode_path_segment(execution_id)
        );
        let endpoint = self.validate("GET", &path, query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch steps for a workflow run.
    pub async fn run_steps<T>(&self, run_id: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!("workflows/runs/{}/steps", encode_path_segment(run_id));
        let endpoint = self.validate("GET", &path, query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch a task by identifier.
    pub async fn get_task<T>(&self, task_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!("workflows/tasks/{}", encode_path_segment(task_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Retry a workflow task.
    pub async fn retry_task<T>(&self, task_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.task_action(task_id, "retry", &EmptyBody {}).await
    }

    /// Cancel a workflow task with an optional reason.
    pub async fn cancel_task<T>(&self, task_id: &str, reason: Option<&str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.task_action(task_id, "cancel", &CancelTask { reason })
            .await
    }

    /// List workflow approvals.
    pub async fn list_approvals<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("GET", "workflows/approvals", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch an approval by identifier.
    pub async fn get_approval<T>(&self, approval_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!("workflows/approvals/{}", encode_path_segment(approval_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Approve a workflow request with an optional comment.
    pub async fn approve<T>(&self, approval_id: &str, comment: Option<&str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.approval_action(approval_id, "approve", comment).await
    }

    /// Reject a workflow request with an optional comment.
    pub async fn reject<T>(&self, approval_id: &str, comment: Option<&str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.approval_action(approval_id, "reject", comment).await
    }

    /// List workflow templates.
    pub async fn list_templates<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("GET", "workflows/templates", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch a workflow template by identifier.
    pub async fn get_template<T>(&self, template_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!("workflows/templates/{}", encode_path_segment(template_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Create a workflow template using a contract-defined request type.
    pub async fn create_template<B, T>(&self, template: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POST", "workflows/templates", &[])?;
        self.client.post(endpoint, template).await
    }

    /// Instantiate a template as a named workflow.
    pub async fn instantiate_template<T>(&self, template_id: &str, name: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!(
            "workflows/templates/{}/instantiate",
            encode_path_segment(template_id)
        );
        let endpoint = self.validate("POST", &path, &[])?;
        self.client
            .post(endpoint, &InstantiateTemplate { name })
            .await
    }

    async fn workflow_action(&self, workflow_id: &str, action: &str) -> Result<Workflow> {
        let path = format!("workflows/{}/{action}", encode_path_segment(workflow_id));
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post(endpoint, &EmptyBody {}).await
    }

    async fn task_action<T, B>(&self, task_id: &str, action: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let path = format!("workflows/tasks/{}/{action}", encode_path_segment(task_id));
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post(endpoint, body).await
    }

    async fn approval_action<T>(
        &self,
        approval_id: &str,
        action: &str,
        comment: Option<&str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!(
            "workflows/approvals/{}/{action}",
            encode_path_segment(approval_id)
        );
        let endpoint = self.validate("POST", &path, &[])?;
        self.client
            .post(endpoint, &ApprovalDecision { comment })
            .await
    }
}
