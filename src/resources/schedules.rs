//! Service scoped access for the `schedules` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `schedules` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct SchedulesService;

/// Typed access to the `schedules` service.
pub type Schedules = ServiceClient<SchedulesService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "schedules";

impl ServiceClient<SchedulesService> {
    /// List workflow schedules.
    pub async fn list_schedules<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("workflows/schedules", query).await
    }

    /// Create a workflow schedule.
    pub async fn create_schedule<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("workflows/schedules", request).await
    }

    /// Fetch one schedule by identifier.
    pub async fn get_schedule<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("workflows/schedules/{}", encode_path_segment(id)))
            .await
    }

    /// Update a schedule.
    pub async fn update_schedule<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.patch(
            format!("workflows/schedules/{}", encode_path_segment(id)),
            request,
        )
        .await
    }

    /// Delete a schedule.
    pub async fn delete_schedule(&self, id: &str) -> Result<()> {
        self.delete_empty(format!("workflows/schedules/{}", encode_path_segment(id)))
            .await
    }

    /// Pause a schedule.
    pub async fn pause_schedule<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("workflows/schedules/{}/pause", encode_path_segment(id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// Resume a paused schedule.
    pub async fn resume_schedule<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("workflows/schedules/{}/resume", encode_path_segment(id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// Trigger a schedule immediately.
    pub async fn trigger_schedule<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("workflows/schedules/{}/trigger", encode_path_segment(id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// Validate a cron expression.
    pub async fn validate_cron<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("workflows/cron/validate", request).await
    }

    /// Parse a cron expression and return its upcoming schedule.
    pub async fn parse_cron<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("workflows/cron/parse", request).await
    }
}
