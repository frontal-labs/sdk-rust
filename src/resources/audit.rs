//! Service scoped access for the `audit` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `audit` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct AuditService;

/// Typed access to the `audit` service.
pub type Audit = ServiceClient<AuditService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "audit";

impl ServiceClient<AuditService> {
    /// Record one audit event.
    pub async fn create_event<B, T>(&self, event: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("audit/events", event).await
    }

    /// Record a batch of audit events.
    pub async fn create_event_batch<B, T>(&self, events: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("audit/events/batch", events).await
    }

    /// List audit events using caller-selected filters and response types.
    pub async fn list_events<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("audit/events", query).await
    }

    /// Fetch one audit event by identifier.
    pub async fn get_event<T>(&self, event_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!("audit/events/{}", encode_path_segment(event_id));
        self.get(path).await
    }
}
