//! Service scoped access for the `workflows` API.

use serde::Serialize;

use super::ServiceClient;
use crate::{
    encode_path_segment,
    global::Result,
    models::workflows::{
        CancelWorkflowTaskRequest, CreateWorkflowRequest, CreateWorkflowTemplateRequest,
        CreateWorkflowVersion, InstantiateTemplateRequest, InstantiateTemplateResponse,
        ListExecutionsResponse, ListWorkflowApprovalsResponse, ListWorkflowRunStepsResponse,
        ListWorkflowTasksResponse, ListWorkflowTemplatesResponse, ListWorkflowsResponse,
        RetryWorkflowTaskRequest, StartWorkflowExecutionRequest, StartWorkflowExecutionResponse,
        UpdateWorkflowRequest, Workflow, WorkflowApproval, WorkflowApprovalDecisionRequest,
        WorkflowEnvelope, WorkflowExecutionResponse, WorkflowSpec, WorkflowTask,
        WorkflowTaskEnvelope, WorkflowTemplate, WorkflowTemplateEnvelope, WorkflowVersion,
    },
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

impl ServiceClient<WorkflowsService> {
    /// List workflows using supported filters and cursor values.
    pub async fn list_workflows(&self, query: &[(&str, &str)]) -> Result<ListWorkflowsResponse> {
        let endpoint = self.validate("GET", "workflows", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Create a workflow resource. Add a version with [`Self::add_workflow_version`].
    pub async fn create_workflow_record(
        &self,
        request: &CreateWorkflowRequest,
    ) -> Result<Workflow> {
        let endpoint = self.validate("POST", "workflows", &[])?;
        let response: WorkflowEnvelope = self.client.post(endpoint, request).await?;
        Ok(response.workflow)
    }

    /// Add a version containing the workflow definition.
    pub async fn add_workflow_version(
        &self,
        workflow_id: &str,
        definition: &WorkflowSpec,
    ) -> Result<WorkflowVersion> {
        let path = format!("workflows/{}/versions", encode_path_segment(workflow_id));
        let endpoint = self.validate("POST", &path, &[])?;
        let request = CreateWorkflowVersion {
            spec: definition.clone(),
            actor: None,
        };
        let response: crate::models::workflows::WorkflowVersionEnvelope =
            self.client.post(endpoint, &request).await?;
        Ok(response.version)
    }

    /// Fetch a workflow by identifier.
    pub async fn get_workflow(&self, workflow_id: &str) -> Result<Workflow> {
        let path = format!("workflows/{}", encode_path_segment(workflow_id));
        let endpoint = self.validate("GET", &path, &[])?;
        let response: WorkflowEnvelope = self.client.get(endpoint).await?;
        Ok(response.workflow)
    }

    /// Update a workflow definition.
    pub async fn update_workflow(
        &self,
        workflow_id: &str,
        request: &UpdateWorkflowRequest,
    ) -> Result<Workflow> {
        let path = format!("workflows/{}", encode_path_segment(workflow_id));
        let endpoint = self.validate("PATCH", &path, &[])?;
        let response: WorkflowEnvelope = self.client.patch(endpoint, request).await?;
        Ok(response.workflow)
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
    pub async fn list_executions(&self, query: &[(&str, &str)]) -> Result<ListExecutionsResponse> {
        let endpoint = self.validate("GET", "workflows/executions", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Start a workflow execution.
    pub async fn create_execution(
        &self,
        workflow_id: &str,
        input: &WorkflowSpec,
    ) -> Result<StartWorkflowExecutionResponse> {
        let endpoint = self.validate("POST", "workflows/executions", &[])?;
        let request = StartWorkflowExecutionRequest {
            workflow_id: workflow_id.to_owned(),
            version_id: None,
            input: Some(input.clone()),
            actor: None,
        };
        self.client.post(endpoint, &request).await
    }

    /// Fetch a workflow execution by identifier.
    pub async fn get_execution(&self, execution_id: &str) -> Result<WorkflowExecutionResponse> {
        let path = format!("workflows/executions/{}", encode_path_segment(execution_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Fetch tasks for an execution.
    pub async fn execution_tasks(
        &self,
        execution_id: &str,
        query: &[(&str, &str)],
    ) -> Result<ListWorkflowTasksResponse> {
        let path = format!(
            "workflows/executions/{}/tasks",
            encode_path_segment(execution_id)
        );
        let endpoint = self.validate("GET", &path, query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch steps for a workflow run.
    pub async fn run_steps(
        &self,
        run_id: &str,
        query: &[(&str, &str)],
    ) -> Result<ListWorkflowRunStepsResponse> {
        let path = format!("workflows/runs/{}/steps", encode_path_segment(run_id));
        let endpoint = self.validate("GET", &path, query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch a task by identifier.
    pub async fn get_task(&self, task_id: &str) -> Result<WorkflowTask> {
        let path = format!("workflows/tasks/{}", encode_path_segment(task_id));
        let endpoint = self.validate("GET", &path, &[])?;
        let response: WorkflowTaskEnvelope = self.client.get(endpoint).await?;
        Ok(response.task)
    }

    /// Retry a workflow task.
    pub async fn retry_task(
        &self,
        task_id: &str,
        request: &RetryWorkflowTaskRequest,
    ) -> Result<WorkflowTask> {
        self.task_action(task_id, "retry", request).await
    }

    /// Cancel a workflow task with an optional reason.
    pub async fn cancel_task(
        &self,
        task_id: &str,
        request: &CancelWorkflowTaskRequest,
    ) -> Result<WorkflowTask> {
        self.task_action(task_id, "cancel", request).await
    }

    /// List workflow approvals.
    pub async fn list_approvals(
        &self,
        query: &[(&str, &str)],
    ) -> Result<ListWorkflowApprovalsResponse> {
        let endpoint = self.validate("GET", "workflows/approvals", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch an approval by identifier.
    pub async fn get_approval(&self, approval_id: &str) -> Result<WorkflowApproval> {
        let path = format!("workflows/approvals/{}", encode_path_segment(approval_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Approve a workflow request with an optional comment.
    pub async fn approve(
        &self,
        approval_id: &str,
        request: &WorkflowApprovalDecisionRequest,
    ) -> Result<WorkflowApproval> {
        self.approval_action(approval_id, "approve", request).await
    }

    /// Reject a workflow request with an optional comment.
    pub async fn reject(
        &self,
        approval_id: &str,
        request: &WorkflowApprovalDecisionRequest,
    ) -> Result<WorkflowApproval> {
        self.approval_action(approval_id, "reject", request).await
    }

    /// List workflow templates.
    pub async fn list_templates(
        &self,
        query: &[(&str, &str)],
    ) -> Result<ListWorkflowTemplatesResponse> {
        let endpoint = self.validate("GET", "workflows/templates", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch a workflow template by identifier.
    pub async fn get_template(&self, template_id: &str) -> Result<WorkflowTemplate> {
        let path = format!("workflows/templates/{}", encode_path_segment(template_id));
        let endpoint = self.validate("GET", &path, &[])?;
        let response: WorkflowTemplateEnvelope = self.client.get(endpoint).await?;
        Ok(response.template)
    }

    /// Create a workflow template using a contract-defined request type.
    pub async fn create_template(
        &self,
        template: &CreateWorkflowTemplateRequest,
    ) -> Result<WorkflowTemplate> {
        let endpoint = self.validate("POST", "workflows/templates", &[])?;
        let response: WorkflowTemplateEnvelope = self.client.post(endpoint, template).await?;
        Ok(response.template)
    }

    /// Instantiate a template as a named workflow.
    pub async fn instantiate_template(
        &self,
        template_id: &str,
        request: &InstantiateTemplateRequest,
    ) -> Result<InstantiateTemplateResponse> {
        let path = format!(
            "workflows/templates/{}/instantiate",
            encode_path_segment(template_id)
        );
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post(endpoint, request).await
    }

    async fn workflow_action(&self, workflow_id: &str, action: &str) -> Result<Workflow> {
        let path = format!("workflows/{}/{action}", encode_path_segment(workflow_id));
        let endpoint = self.validate("POST", &path, &[])?;
        let response: WorkflowEnvelope = self.client.post(endpoint, &EmptyBody {}).await?;
        Ok(response.workflow)
    }

    async fn task_action<B: Serialize + ?Sized>(
        &self,
        task_id: &str,
        action: &str,
        body: &B,
    ) -> Result<WorkflowTask> {
        let path = format!("workflows/tasks/{}/{action}", encode_path_segment(task_id));
        let endpoint = self.validate("POST", &path, &[])?;
        let response: WorkflowTaskEnvelope = self.client.post(endpoint, body).await?;
        Ok(response.task)
    }

    async fn approval_action(
        &self,
        approval_id: &str,
        action: &str,
        request: &WorkflowApprovalDecisionRequest,
    ) -> Result<WorkflowApproval> {
        let path = format!(
            "workflows/approvals/{}/{action}",
            encode_path_segment(approval_id)
        );
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post(endpoint, request).await
    }
}
