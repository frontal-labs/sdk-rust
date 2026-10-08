//! Service scoped access for the `ai` API.

use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use super::ServiceClient;
use crate::{
    global::Result,
    models::ai::{
        ChatCompletionRequest, ChatCompletionResponse, EmbeddingsRequest, EmbeddingsResponse,
    },
};

/// Type marker for the `ai` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct AiService;

/// Typed access to the `ai` service.
pub type Ai = ServiceClient<AiService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "ai";

impl ServiceClient<AiService> {
    /// Send an OpenAI-compatible chat completion request.
    pub async fn chat_completions(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let endpoint = self.validate("POST", "ai/chat/completions", &[])?;
        self.client.post(endpoint, request).await
    }

    /// Create embeddings for text or a batch of text inputs.
    pub async fn embeddings(&self, request: &EmbeddingsRequest) -> Result<EmbeddingsResponse> {
        let endpoint = self.validate("POST", "internal/embeddings", &[])?;
        self.client.post(endpoint, request).await
    }

    /// Return the AI gateway health response.
    pub async fn health(&self) -> Result<Value> {
        let endpoint = self.validate("GET", "health", &[])?;
        self.client.get(endpoint).await
    }

    /// Return the available AI model catalog.
    pub async fn models(&self) -> Result<Value> {
        let endpoint = self.validate("GET", "internal/models", &[])?;
        self.client.get(endpoint).await
    }

    /// Return the configured default model for each supported task.
    pub async fn model_defaults(&self) -> Result<BTreeMap<String, String>> {
        let endpoint = self.validate("GET", "internal/models/defaults", &[])?;
        self.client.get(endpoint).await
    }

    /// Run a prediction request with a typed JSON payload and response.
    pub async fn predict<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POST", "internal/predictions", &[])?;
        self.client.post(endpoint, request).await
    }

    /// Run a prediction using a raw request payload.
    pub async fn predict_raw(&self, content_type: &str, body: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        let endpoint = self.validate("POSTRAW", "internal/predictions", &[])?;
        self.client.post_bytes(endpoint, content_type, body).await
    }

    /// Run a prediction with a multipart form.
    pub async fn predict_form<T>(&self, form: reqwest::multipart::Form) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POSTFORMDATA", "internal/predictions", &[])?;
        self.client.post_form(endpoint, form).await
    }

    /// Rerank documents using a typed JSON payload and response.
    pub async fn rerank<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POST", "internal/rerank", &[])?;
        self.client.post(endpoint, request).await
    }
}
