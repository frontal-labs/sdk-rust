//! Request and response models for the Frontal Workflows API.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// A workflow specification is an open JSON object in the API contract.
///
/// The service accepts arbitrary keys and values, so the SDK preserves that
/// contract while guaranteeing the top-level value is an object.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WorkflowSpec(pub BTreeMap<String, Value>);

/// Actor attribution accepted by workflow engine write requests.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowActor {
    /// User or service principal to record as the actor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    /// Tenant attribution used only by template instantiation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

/// Request to create a workflow resource.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateWorkflowRequest {
    /// Human-readable workflow name.
    pub name: String,
    /// URL-safe workflow identifier.
    pub slug: String,
    /// Optional description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional actor attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<WorkflowActor>,
}

/// Request to partially update a workflow.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateWorkflowRequest {
    /// Replacement human-readable name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replacement description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional actor attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<WorkflowActor>,
}

/// Workflow lifecycle state defined by the API.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WorkflowStatus {
    /// Workflow is being drafted.
    #[serde(rename = "DRAFT")]
    Draft,
    /// Workflow is active.
    #[serde(rename = "ACTIVE")]
    Active,
    /// Workflow is archived.
    #[serde(rename = "ARCHIVED")]
    Archived,
}

/// Workflow resource returned by the API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    /// Workflow identifier.
    pub workflow_id: String,
    /// Tenant that owns the workflow.
    pub tenant_id: String,
    /// Human-readable workflow name.
    pub name: String,
    /// URL-safe workflow identifier.
    pub slug: String,
    /// Workflow description, empty when omitted at creation.
    pub description: String,
    /// Lifecycle status.
    pub status: WorkflowStatus,
    /// Latest workflow version number.
    pub latest_version: u64,
    /// Creator principal identifier.
    pub created_by: String,
    /// Last updater principal identifier.
    pub updated_by: String,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
    /// Last update timestamp in RFC 3339 format.
    pub updated_at: String,
}

/// Envelope returned by workflow creation, retrieval, and update operations.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowEnvelope {
    /// Workflow resource.
    pub workflow: Workflow,
}

/// Response returned when listing workflows.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWorkflowsResponse {
    /// Workflows in this page.
    pub workflows: Vec<Workflow>,
    /// Token for the next page, or an empty string when finished.
    pub next_page_token: String,
}

/// Request to add a workflow version.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateWorkflowVersion {
    /// Open workflow specification.
    pub spec: WorkflowSpec,
    /// Optional actor attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<WorkflowActor>,
}

/// Immutable workflow version returned by the API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowVersion {
    /// Version identifier.
    pub version_id: String,
    /// Parent workflow identifier.
    pub workflow_id: String,
    /// Version number.
    pub version_number: u64,
    /// Authored workflow specification.
    pub spec: WorkflowSpec,
    /// Compiled plan when the spec contains executable steps.
    #[serde(default)]
    pub compiled_plan: Option<WorkflowSpec>,
    /// Specification checksum, empty when the spec is not compiled.
    pub checksum: String,
    /// Whether this version is active.
    pub is_active: bool,
    /// Principal that created the version.
    pub created_by: String,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
    /// Activation timestamp, when activated.
    pub published_at: Option<String>,
}

/// Envelope returned by workflow version operations.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowVersionEnvelope {
    /// Workflow version resource.
    pub version: WorkflowVersion,
}

/// Workflow execution lifecycle state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WorkflowExecutionStatus {
    /// Execution has not started.
    #[serde(rename = "PENDING")]
    Pending,
    /// Execution is running.
    #[serde(rename = "RUNNING")]
    Running,
    /// Execution is paused.
    #[serde(rename = "PAUSED")]
    Paused,
    /// Execution completed successfully.
    #[serde(rename = "COMPLETED")]
    Completed,
    /// Execution failed.
    #[serde(rename = "FAILED")]
    Failed,
    /// Execution was cancelled.
    #[serde(rename = "CANCELLED")]
    Cancelled,
    /// Execution was terminated.
    #[serde(rename = "TERMINATED")]
    Terminated,
}

/// Request to start a workflow execution.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct StartWorkflowExecutionRequest {
    /// Workflow identifier.
    pub workflow_id: String,
    /// Optional workflow version identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Optional JSON input object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<WorkflowSpec>,
    /// Optional actor attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<WorkflowActor>,
}

/// Execution fields returned directly after starting a workflow.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct StartedWorkflowExecution {
    /// Execution identifier.
    pub execution_id: String,
    /// Owning tenant identifier.
    pub tenant_id: String,
    /// Workflow identifier.
    pub workflow_id: String,
    /// Version identifier.
    pub version_id: String,
    /// State-machine status immediately after starting.
    pub status: String,
    /// Input object, present when supplied by the caller.
    #[serde(default)]
    pub input: Option<WorkflowSpec>,
    /// Principal that started the execution.
    pub started_by: String,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
}

/// Initial execution state returned after starting a workflow.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct StartedWorkflowExecutionState {
    /// Steps ready to run.
    pub ready: Vec<String>,
    /// Completed steps.
    pub completed: Vec<String>,
    /// Failed steps.
    pub failed: Vec<String>,
    /// Number of steps in the workflow.
    pub total_steps: u64,
}

/// Response returned after starting a workflow execution.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartWorkflowExecutionResponse {
    /// Newly created execution.
    pub execution: StartedWorkflowExecution,
    /// Initial step state.
    pub state: StartedWorkflowExecutionState,
}

/// Execution summary returned in a list response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecutionSummary {
    /// Execution identifier.
    pub execution_id: String,
    /// Workflow identifier.
    pub workflow_id: String,
    /// Execution status.
    pub status: WorkflowExecutionStatus,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
}

/// Response returned when listing executions.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ListExecutionsResponse {
    /// Executions in this page.
    pub executions: Vec<WorkflowExecutionSummary>,
    /// Token for the next page, or an empty string when finished.
    pub next_page_token: String,
    /// Total matching executions.
    pub total_count: u64,
}

/// Detailed workflow execution returned by the API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecution {
    /// Execution identifier.
    pub execution_id: String,
    /// Owning tenant identifier.
    pub tenant_id: String,
    /// Workflow identifier.
    pub workflow_id: String,
    /// Version identifier.
    pub version_id: String,
    /// Execution status.
    pub status: WorkflowExecutionStatus,
    /// Input object, absent when no input was supplied.
    #[serde(default)]
    pub input: Option<WorkflowSpec>,
    /// Output object, absent until the execution produces output.
    #[serde(default)]
    pub output: Option<WorkflowSpec>,
    /// Failure message, when present.
    pub error_message: Option<String>,
    /// Principal that started the execution.
    pub started_by: String,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
    /// Last update timestamp in RFC 3339 format.
    pub updated_at: String,
    /// Start timestamp, or null before execution begins.
    pub started_at: Option<String>,
    /// Completion timestamp, or null while execution remains active.
    pub completed_at: Option<String>,
}

/// Step identifiers grouped by execution state.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecutionState {
    /// Steps ready to run.
    pub ready: Vec<String>,
    /// Steps currently running.
    pub in_progress: Vec<String>,
    /// Completed steps.
    pub completed: Vec<String>,
    /// Failed steps.
    pub failed: Vec<String>,
    /// Skipped steps.
    pub skipped: Vec<String>,
    /// Steps blocked by dependencies.
    pub blocked: Vec<String>,
}

/// Optional progress projection attached to a detailed execution response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecutionProjection {
    /// Percent of the workflow completed.
    pub progress_percent: f64,
    /// Total number of steps.
    pub total_steps: u64,
    /// Number of completed steps.
    pub completed_steps: u64,
    /// Number of failed steps.
    pub failed_steps: u64,
    /// Current execution phase, or null when no phase is active.
    pub current_phase: Option<String>,
}

/// Detailed execution response containing its state projection.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowExecutionResponse {
    /// Execution resource.
    pub execution: WorkflowExecution,
    /// Step identifiers grouped by state.
    pub state: WorkflowExecutionState,
    /// Optional progress summary.
    #[serde(default)]
    pub projection: Option<WorkflowExecutionProjection>,
}

/// State of an individual workflow step run.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WorkflowStepRunStatus {
    /// Step has not started.
    #[serde(rename = "PENDING")]
    Pending,
    /// Step is ready to run.
    #[serde(rename = "READY")]
    Ready,
    /// Step is running.
    #[serde(rename = "RUNNING")]
    Running,
    /// Step completed successfully.
    #[serde(rename = "COMPLETED")]
    Completed,
    /// Step failed.
    #[serde(rename = "FAILED")]
    Failed,
    /// Step was skipped.
    #[serde(rename = "SKIPPED")]
    Skipped,
    /// Step was cancelled.
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

/// One task associated with a workflow execution.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowTask {
    /// Task identifier.
    pub task_id: String,
    /// Workflow step identifier.
    pub step_id: String,
    /// Execution identifier.
    pub execution_id: String,
    /// Task state.
    pub status: WorkflowStepRunStatus,
    /// Step type; the API currently returns `unknown`.
    #[serde(rename = "type")]
    pub task_type: String,
    /// Task input, when recorded.
    #[serde(default)]
    pub input: Option<WorkflowSpec>,
    /// Task output, when recorded.
    #[serde(default)]
    pub output: Option<WorkflowSpec>,
    /// Structured failure details, when present.
    pub error: Option<BTreeMap<String, Value>>,
    /// Failure message, empty when no error message exists.
    pub error_message: String,
    /// Current attempt number.
    pub attempt: u64,
    /// Maximum attempts configured for the task.
    pub max_attempts: u64,
    /// Start timestamp, or null before the task starts.
    pub started_at: Option<String>,
    /// Completion timestamp, or null until the task completes.
    pub completed_at: Option<String>,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
    /// Last update timestamp in RFC 3339 format.
    pub updated_at: String,
}

/// Envelope returned by task retrieval and task actions.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowTaskEnvelope {
    /// Task resource.
    pub task: WorkflowTask,
}

/// Response returned when listing workflow execution tasks.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ListWorkflowTasksResponse {
    /// Tasks in this page.
    pub tasks: Vec<WorkflowTask>,
    /// Token for the next page, or an empty string when finished.
    pub next_page_token: String,
}

/// One step row returned by the workflow run read surface.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowRunStep {
    /// Workflow step identifier.
    pub step_id: String,
    /// Step position in creation order.
    pub step_index: u64,
    /// Step state.
    pub status: WorkflowStepRunStatus,
    /// Step input, when recorded.
    #[serde(default)]
    pub input: Option<WorkflowSpec>,
    /// Step output, when recorded.
    #[serde(default)]
    pub output: Option<WorkflowSpec>,
    /// Structured error details, when present.
    pub error: Option<BTreeMap<String, Value>>,
    /// Current attempt number.
    pub attempt: u64,
    /// Maximum attempts configured for the step.
    pub max_attempts: u64,
    /// Step duration in milliseconds, or null before the step starts.
    pub latency_ms: Option<u64>,
    /// Start timestamp, or null before the step starts.
    pub started_at: Option<String>,
    /// Completion timestamp, or null until the step completes.
    pub completed_at: Option<String>,
}

/// Response returned when listing steps for a workflow run.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ListWorkflowRunStepsResponse {
    /// Steps in this page.
    pub steps: Vec<WorkflowRunStep>,
    /// Token for the next page.
    pub next_page_token: String,
}

/// Workflow task retry request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct RetryWorkflowTaskRequest {
    /// Whether to reset the task's dependencies before retrying.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_dependencies: Option<bool>,
    /// Optional caller attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<WorkflowExecutionActor>,
}

/// Workflow task cancellation request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct CancelWorkflowTaskRequest {
    /// Cancellation reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Optional caller attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<WorkflowExecutionActor>,
}

/// Actor attribution used by execution and task operations.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct WorkflowExecutionActor {
    /// User or service principal to record as the actor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    /// Tenant attribution, stripped by the API middleware.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

/// Workflow approval lifecycle state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WorkflowApprovalStatus {
    /// Approval is waiting for a decision.
    #[serde(rename = "PENDING")]
    Pending,
    /// Approval was accepted.
    #[serde(rename = "APPROVED")]
    Approved,
    /// Approval was rejected.
    #[serde(rename = "REJECTED")]
    Rejected,
    /// Approval was cancelled.
    #[serde(rename = "CANCELLED")]
    Cancelled,
    /// Approval expired.
    #[serde(rename = "EXPIRED")]
    Expired,
}

/// A human approval gate on a workflow execution.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowApproval {
    /// Approval identifier.
    pub id: String,
    /// Owning tenant identifier.
    pub tenant_id: String,
    /// Execution identifier.
    pub execution_id: String,
    /// Workflow step identifier.
    pub step_id: String,
    /// Signal name used to deliver the approval decision.
    pub signal_name: String,
    /// Approval title.
    pub title: String,
    /// Approval description.
    pub description: String,
    /// Approval state.
    pub status: WorkflowApprovalStatus,
    /// Principals eligible to approve.
    pub required_approvers: Vec<String>,
    /// Principals who approved.
    pub approved_by: Vec<String>,
    /// Principal who rejected, when rejected.
    pub rejected_by: Option<String>,
    /// Cancellation reason, when cancelled.
    pub cancel_reason: Option<String>,
    /// Decision comment, when supplied.
    pub comment: Option<String>,
    /// Expiration timestamp, when configured.
    pub expires_at: Option<String>,
    /// Resolution timestamp, when resolved.
    pub resolved_at: Option<String>,
    /// Creator principal identifier.
    pub created_by: String,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
    /// Last update timestamp in RFC 3339 format.
    pub updated_at: String,
}

/// Response returned when listing workflow approvals.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListWorkflowApprovalsResponse {
    /// Approval rows in this page.
    pub items: Vec<WorkflowApproval>,
    /// Next-page token, when present.
    #[serde(default, rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// Total matching approvals, when present.
    #[serde(default, rename = "totalCount")]
    pub total_count: Option<u64>,
}

/// Approval decision request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct WorkflowApprovalDecisionRequest {
    /// Approver principal; defaults to the authenticated caller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    /// Optional decision comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Request to instantiate a workflow template.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstantiateTemplateRequest {
    /// Name for the new workflow.
    pub name: String,
    /// Optional slug; the API derives one from the name when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Optional variable values merged into the workflow spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_overrides: Option<BTreeMap<String, String>>,
    /// Optional actor attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<WorkflowActor>,
}

/// Response returned when creating a workflow from a template.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstantiateTemplateResponse {
    /// Identifier of the new workflow.
    pub workflow_id: String,
    /// Created workflow resource.
    pub workflow: Workflow,
}

/// A reusable workflow template.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowTemplate {
    /// Template identifier.
    pub template_id: String,
    /// Template name.
    pub name: String,
    /// Template description.
    pub description: String,
    /// Open workflow definition.
    pub definition: WorkflowSpec,
    /// Template category.
    pub category: String,
    /// Template tags.
    pub tags: Vec<String>,
    /// Creator principal identifier.
    pub created_by: String,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: String,
    /// Last update timestamp in RFC 3339 format.
    pub updated_at: String,
}

/// Envelope returned by template creation and retrieval.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowTemplateEnvelope {
    /// Template resource.
    pub template: WorkflowTemplate,
}

/// Request to create a reusable workflow template.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateWorkflowTemplateRequest {
    /// Template name.
    pub name: String,
    /// Template description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Open workflow definition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<WorkflowSpec>,
    /// Optional template category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Template tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Optional actor attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<WorkflowActor>,
}

/// Response returned when listing templates.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWorkflowTemplatesResponse {
    /// Templates in this page.
    pub templates: Vec<WorkflowTemplate>,
    /// Token for the next page, or an empty string when finished.
    pub next_page_token: String,
}
