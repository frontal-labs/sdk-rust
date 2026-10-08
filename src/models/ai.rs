//! Request and response models for the Frontal AI API.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One message in an OpenAI-compatible chat request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    /// Message role.
    pub role: String,
    /// Text content, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Optional message name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional tool calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<Value>>,
    /// Tool call ID for tool result messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// Request body for `POST /ai/chat/completions`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCompletionRequest {
    /// Model identifier.
    pub model: String,
    /// Conversation messages.
    pub messages: Vec<ChatMessage>,
    /// Sampling temperature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling probability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Number of choices to return.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Whether the API should stream completion chunks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Stop sequence or sequences.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<Value>,
    /// Maximum completion token count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Presence penalty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// Frequency penalty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// Token bias values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<std::collections::BTreeMap<String, f64>>,
    /// End-user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Structured response format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
    /// Optional seed for deterministic sampling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Tool declarations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
    /// Tool selection policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
}

/// A chat completion response message.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCompletionChoiceMessage {
    /// Message role.
    pub role: String,
    /// Returned text, if any.
    #[serde(default)]
    pub content: Option<String>,
    /// Returned tool calls, if any.
    #[serde(default)]
    pub tool_calls: Option<Vec<Value>>,
    /// Tool call ID for tool messages.
    #[serde(default)]
    pub tool_call_id: Option<String>,
}

/// One completion choice.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCompletionChoice {
    /// Choice index.
    pub index: u32,
    /// Generated message.
    pub message: ChatCompletionChoiceMessage,
    /// Why generation stopped.
    #[serde(default)]
    pub finish_reason: Option<String>,
}

/// Token usage returned by a chat completion.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCompletionUsage {
    /// Prompt token count.
    pub prompt_tokens: u64,
    /// Completion token count.
    pub completion_tokens: u64,
    /// Total token count.
    pub total_tokens: u64,
}

/// Response body for `POST /ai/chat/completions`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCompletionResponse {
    /// Completion identifier.
    pub id: String,
    /// Response object type.
    pub object: String,
    /// Creation time as a Unix timestamp.
    pub created: u64,
    /// Model identifier.
    pub model: String,
    /// Returned choices.
    pub choices: Vec<ChatCompletionChoice>,
    /// Optional token usage.
    #[serde(default)]
    pub usage: Option<ChatCompletionUsage>,
    /// Optional server fingerprint.
    #[serde(default)]
    pub system_fingerprint: Option<String>,
}

/// Request body for `POST /internal/embeddings`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddingsRequest {
    /// Text or batch of text to embed.
    pub input: Value,
    /// Model identifier.
    pub model: String,
    /// Requested output encoding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
    /// Optional embedding dimensions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    /// End-user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// One embedding vector.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Embedding {
    /// Response object type.
    pub object: String,
    /// Vector values.
    pub embedding: Vec<f64>,
    /// Index of the source input.
    pub index: u32,
}

/// Token usage returned by an embeddings request.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddingsUsage {
    /// Input token count.
    pub prompt_tokens: u64,
    /// Total token count.
    pub total_tokens: u64,
}

/// Response body for `POST /internal/embeddings`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbeddingsResponse {
    /// Response object type.
    pub object: String,
    /// Returned vectors.
    pub data: Vec<Embedding>,
    /// Model identifier.
    pub model: String,
    /// Token usage.
    pub usage: EmbeddingsUsage,
}
