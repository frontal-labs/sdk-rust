//! Service scoped access for the `webhooks` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `webhooks` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct WebhooksService;

/// Typed access to the `webhooks` service.
pub type Webhooks = ServiceClient<WebhooksService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "webhooks";

impl ServiceClient<WebhooksService> {
    /// List webhook endpoints.
    pub async fn list_webhooks<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("webhooks", query).await
    }

    /// Create a webhook endpoint.
    pub async fn create_webhook<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("webhooks", request).await
    }

    /// Fetch one webhook endpoint.
    pub async fn get_webhook<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("webhooks/{}", encode_path_segment(id)))
            .await
    }

    /// Update a webhook endpoint.
    pub async fn update_webhook<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.put(format!("webhooks/{}", encode_path_segment(id)), request)
            .await
    }

    /// Delete a webhook endpoint.
    pub async fn delete_webhook(&self, id: &str) -> Result<()> {
        self.delete_empty(format!("webhooks/{}", encode_path_segment(id)))
            .await
    }

    /// Rotate a webhook endpoint's signing secret.
    pub async fn rotate_secret<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("webhooks/{}/rotate-secret", encode_path_segment(id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// List webhook delivery attempts.
    pub async fn list_deliveries<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("webhooks/deliveries", query).await
    }

    /// Fetch a webhook delivery attempt.
    pub async fn get_delivery<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("webhooks/deliveries/{}", encode_path_segment(id)))
            .await
    }

    /// Retry a webhook delivery attempt.
    pub async fn retry_delivery<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("webhooks/deliveries/{}/retry", encode_path_segment(id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// Fetch webhook delivery statistics.
    pub async fn webhook_stats<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("webhooks/stats", query).await
    }
}
