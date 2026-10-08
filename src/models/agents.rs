//! Request and response models for the Frontal Agents API.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// A trigger that starts an agent run.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTrigger {
    /// Event name that starts the agent.
    pub event: String,
    /// Optional trigger filter expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Value>,
    /// Optional debounce interval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debounce: Option<String>,
}

/// Entity and action permissions granted to an agent.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentScope {
    /// Entity types the agent may read.
    #[serde(default)]
    pub read: Vec<String>,
    /// Entity types the agent may write.
    #[serde(default)]
    pub write: Vec<String>,
    /// Actions the agent may call.
    #[serde(default)]
    pub actions: Vec<String>,
    /// Escalation targets available to the agent.
    #[serde(default)]
    pub escalate: Vec<String>,
    /// Other agents this agent may invoke.
    #[serde(default)]
    pub invoke_agents: Vec<String>,
    /// Functions this agent may invoke.
    #[serde(default)]
    pub invoke_functions: Vec<String>,
}

/// Confidence thresholds for autonomous execution and escalation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfidenceConfig {
    /// Confidence above which an action may run automatically.
    pub auto_execute_above: f64,
    /// Confidence below which the agent escalates.
    pub escalate_below: f64,
    /// Whether the interval between thresholds requires review.
    pub require_review_between: bool,
}

impl Default for ConfidenceConfig {
    fn default() -> Self {
        Self {
            auto_execute_above: 0.85,
            escalate_below: 0.6,
            require_review_between: true,
        }
    }
}

/// Agent memory settings.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryConfig {
    /// Memory lifetime category.
    #[serde(rename = "type")]
    pub kind: String,
    /// Optional time to live.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    /// Optional maximum token count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            kind: "working".to_owned(),
            ttl: None,
            max_tokens: None,
        }
    }
}

/// Retry behavior for agent executions.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryConfig {
    /// Maximum retries.
    pub max_retries: u32,
    /// Delay between retries in milliseconds.
    pub retry_delay: u64,
    /// Backoff strategy.
    pub backoff: String,
    /// HTTP status codes that can be retried.
    pub retry_on: Vec<u16>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: 1_000,
            backoff: "exponential".to_owned(),
            retry_on: vec![408, 409, 425, 429, 500, 502, 503, 504],
        }
    }
}

/// Rate limits for an agent.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitConfig {
    /// Maximum executions per minute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_executions_per_minute: Option<u32>,
    /// Maximum concurrent executions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<u32>,
}

/// Definition sent when creating or updating an agent.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDefinition {
    /// Human-readable agent name.
    pub name: String,
    /// Optional description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Events that activate the agent.
    pub triggers: Vec<AgentTrigger>,
    /// Granted permissions.
    #[serde(default)]
    pub scope: AgentScope,
    /// Confidence thresholds.
    #[serde(default)]
    pub confidence: ConfidenceConfig,
    /// Memory configuration.
    #[serde(default)]
    pub memory: MemoryConfig,
    /// Retry configuration.
    #[serde(default)]
    pub retry: RetryConfig,
    /// Execution timeout.
    #[serde(default = "default_agent_timeout")]
    pub timeout: String,
    /// Optional rate limits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<RateLimitConfig>,
    /// Optional labels.
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_agent_timeout() -> String {
    "30s".to_owned()
}

/// Agent lifecycle state.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    /// The agent is not yet active.
    Draft,
    /// The agent accepts runs.
    Active,
    /// The agent is temporarily disabled.
    Paused,
    /// The agent is retired.
    Deprecated,
}

/// Agent resource returned by the API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    /// Stable agent identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,
    /// Trigger definitions.
    #[serde(default)]
    pub triggers: Vec<AgentTrigger>,
    /// Granted permissions.
    #[serde(default)]
    pub scope: AgentScope,
    /// Confidence thresholds.
    #[serde(default)]
    pub confidence: ConfidenceConfig,
    /// Memory configuration.
    #[serde(default)]
    pub memory: MemoryConfig,
    /// Retry configuration.
    #[serde(default)]
    pub retry: RetryConfig,
    /// Optional execution timeout.
    #[serde(default)]
    pub timeout: Option<String>,
    /// Current version.
    pub version: u64,
    /// Lifecycle status.
    pub status: AgentStatus,
    /// Runtime environment label.
    pub environment: String,
    /// Optional metrics summary.
    #[serde(default)]
    pub metrics: Option<AgentMetrics>,
    /// Creation timestamp from the API.
    pub created_at: String,
    /// Last update timestamp from the API.
    pub updated_at: String,
    /// Additional fields retained for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Aggregate execution metrics returned for an agent.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentMetrics {
    /// Executions started today.
    pub executions_today: u64,
    /// Fraction of executions escalated.
    pub escalation_rate: f64,
    /// Average execution time in milliseconds.
    pub avg_execution_ms: u64,
    /// Fraction of successful executions.
    pub success_rate: f64,
}

/// Agent execution (run) returned by the API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentExecution {
    /// Run identifier.
    pub id: String,
    /// Agent identifier.
    pub agent_id: String,
    /// Trigger event name.
    pub trigger_event: String,
    /// Trigger payload.
    #[serde(default)]
    pub trigger_payload: BTreeMap<String, Value>,
    /// Current run status.
    pub status: String,
    /// Optional result summary.
    #[serde(default)]
    pub outcome: Option<String>,
    /// Optional confidence score.
    #[serde(default)]
    pub confidence: Option<f64>,
    /// Start timestamp.
    pub started_at: String,
    /// Optional completion timestamp.
    #[serde(default)]
    pub completed_at: Option<String>,
    /// Optional runtime in milliseconds.
    #[serde(default)]
    pub duration_ms: Option<u64>,
    /// Optional failure message.
    #[serde(default)]
    pub error: Option<String>,
    /// Additional fields retained for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Response envelope returned by agent list operations.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPage {
    /// Agents on this page.
    #[serde(default)]
    pub data: Vec<Agent>,
    /// Cursor pagination metadata.
    #[serde(default)]
    pub pagination: Option<Value>,
}

/// Request body that starts a new agent run.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRunRequest {
    /// Event name delivered to the agent.
    pub event: String,
    /// Event payload.
    pub payload: BTreeMap<String, Value>,
}

/// Optional target version for an agent rollback.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRollbackRequest {
    /// Optional version to restore.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_version: Option<u64>,
}

/// Conversation transcript returned for an agent run.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AgentConversation {
    /// Ordered conversation messages.
    #[serde(default)]
    pub messages: Vec<Value>,
}
