//! Request and response models for the Frontal Workflows API.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Request body used to create a workflow resource.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkflowRequest {
    /// Human-readable workflow name.
    pub name: String,
    /// URL-safe workflow identifier.
    pub slug: String,
    /// Optional description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Workflow trigger definition.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WorkflowTrigger {
    /// Triggered manually.
    Manual {
        /// Optional trigger configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        config: Option<BTreeMap<String, Value>>,
    },
    /// Triggered by a schedule.
    Schedule {
        /// Cron schedule.
        schedule: String,
        /// Optional trigger configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        config: Option<BTreeMap<String, Value>>,
    },
    /// Triggered by a named event.
    Event {
        /// Event type.
        #[serde(rename = "eventType")]
        event_type: String,
        /// Optional trigger configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        config: Option<BTreeMap<String, Value>>,
    },
    /// Triggered by a webhook URL.
    Webhook {
        /// Webhook URL.
        #[serde(rename = "webhookUrl")]
        webhook_url: String,
        /// Optional trigger configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        config: Option<BTreeMap<String, Value>>,
    },
}

/// Retry policy for a workflow step.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepRetryPolicy {
    /// Maximum attempts for the step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
    /// Backoff strategy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff: Option<String>,
}

/// Workflow step definition.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WorkflowStep {
    /// Runs an action or task.
    Task {
        /// Stable step identifier.
        id: String,
        /// Optional display name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Optional description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Dependencies that must finish first.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        depends_on: Option<Vec<String>>,
        /// Optional time limit.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout: Option<String>,
        /// Optional retry policy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        retry_policy: Option<StepRetryPolicy>,
        /// Task configuration.
        config: BTreeMap<String, Value>,
    },
    /// Pauses for human approval.
    Approval {
        /// Stable step identifier.
        id: String,
        /// Optional display name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Optional description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Dependencies that must finish first.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        depends_on: Option<Vec<String>>,
        /// Optional time limit.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout: Option<String>,
        /// Optional retry policy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        retry_policy: Option<StepRetryPolicy>,
        /// Approval configuration.
        config: ApprovalStepConfig,
    },
    /// Evaluates a condition.
    Condition {
        /// Stable step identifier.
        id: String,
        /// Optional display name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Optional description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Dependencies that must finish first.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        depends_on: Option<Vec<String>>,
        /// Optional time limit.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout: Option<String>,
        /// Optional retry policy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        retry_policy: Option<StepRetryPolicy>,
        /// Condition expression.
        condition: String,
    },
    /// Runs a set of steps in parallel.
    Parallel {
        /// Stable step identifier.
        id: String,
        /// Optional display name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Optional description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Dependencies that must finish first.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        depends_on: Option<Vec<String>>,
        /// Optional time limit.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout: Option<String>,
        /// Optional retry policy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        retry_policy: Option<StepRetryPolicy>,
        /// Parallel step configuration.
        config: ParallelStepConfig,
    },
    /// Waits for a duration.
    Delay {
        /// Stable step identifier.
        id: String,
        /// Optional display name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Optional description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Dependencies that must finish first.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        depends_on: Option<Vec<String>>,
        /// Optional time limit.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout: Option<String>,
        /// Optional retry policy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        retry_policy: Option<StepRetryPolicy>,
        /// Delay configuration.
        config: DelayStepConfig,
    },
    /// Sends a notification.
    Notification {
        /// Stable step identifier.
        id: String,
        /// Optional display name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Optional description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Dependencies that must finish first.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        depends_on: Option<Vec<String>>,
        /// Optional time limit.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout: Option<String>,
        /// Optional retry policy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        retry_policy: Option<StepRetryPolicy>,
        /// Notification configuration.
        config: NotificationStepConfig,
    },
}

/// Configuration for an approval step.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalStepConfig {
    /// Required approver identifiers.
    pub approvers: Vec<String>,
    /// Additional configuration defined by the service.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Configuration for a parallel step.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParallelStepConfig {
    /// Step identifiers to run in parallel.
    pub steps: Vec<String>,
    /// Additional configuration defined by the service.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Configuration for a delay step.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DelayStepConfig {
    /// Duration to wait.
    pub duration: String,
    /// Additional configuration defined by the service.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Configuration for a notification step.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotificationStepConfig {
    /// Message to send.
    pub message: String,
    /// Notification channels.
    pub channels: Vec<String>,
    /// Additional configuration defined by the service.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Workflow definition sent when creating a new workflow version.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowDefinition {
    /// Human-readable workflow name.
    pub name: String,
    /// Optional description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional semantic version string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Trigger definitions from the workflow contract.
    pub triggers: Vec<WorkflowTrigger>,
    /// Step definitions from the workflow contract.
    pub steps: Vec<WorkflowStep>,
    /// Optional workflow variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, Value>>,
    /// Optional labels.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Workflow resource returned by the API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    /// Workflow identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,
    /// Lifecycle status.
    pub status: String,
    /// Current workflow version.
    #[serde(default)]
    pub version: Option<u64>,
    /// Latest version recorded by the service.
    #[serde(default)]
    pub latest_version: Option<u64>,
    /// Trigger definitions when included in the response.
    #[serde(default)]
    pub triggers: Option<Vec<WorkflowTrigger>>,
    /// Step definitions when included in the response.
    #[serde(default)]
    pub steps: Option<Vec<WorkflowStep>>,
    /// Workflow variables when included in the response.
    #[serde(default)]
    pub variables: Option<BTreeMap<String, Value>>,
    /// Workflow labels.
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    /// Creation timestamp from the API.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Last update timestamp from the API.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Workflow execution returned by the API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowExecution {
    /// Execution identifier.
    pub id: String,
    /// Workflow identifier.
    pub workflow_id: String,
    /// Execution status.
    pub status: String,
    /// Workflow version used for this execution.
    #[serde(default)]
    pub workflow_version: Option<u64>,
    /// Input values.
    #[serde(default)]
    pub input: Option<BTreeMap<String, Value>>,
    /// Output values.
    #[serde(default)]
    pub output: Option<BTreeMap<String, Value>>,
    /// Runtime variables.
    #[serde(default)]
    pub variables: Option<BTreeMap<String, Value>>,
    /// Step execution records.
    #[serde(default)]
    pub step_executions: Vec<Value>,
    /// Trigger source.
    #[serde(default)]
    pub triggered_by: Option<String>,
    /// Execution start timestamp.
    #[serde(default)]
    pub started_at: Option<String>,
    /// Execution completion timestamp.
    #[serde(default)]
    pub completed_at: Option<String>,
    /// Execution duration in milliseconds.
    #[serde(default)]
    pub duration_ms: Option<u64>,
    /// Optional error details.
    #[serde(default)]
    pub error: Option<String>,
}
