//! Service scoped access for the `connectors` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `connectors` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct ConnectorsService;

/// Typed access to the `connectors` service.
pub type Connectors = ServiceClient<ConnectorsService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "connectors";

impl ServiceClient<ConnectorsService> {
    /// List connector definitions in the catalog.
    pub async fn list_catalog<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("connectors/catalog").await
    }

    /// Fetch a connector definition by slug.
    pub async fn get_catalog_entry<T>(&self, slug: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("connectors/catalog/{}", encode_path_segment(slug)))
            .await
    }

    /// Replay a connector sync run.
    pub async fn replay_sync_run<B, T>(&self, run_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "connectors/sync-runs/{}/replay",
                encode_path_segment(run_id)
            ),
            request,
        )
        .await
    }

    /// Return connector diagnostics.
    pub async fn diagnostics<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("diagnostics").await
    }

    /// List connector installations.
    pub async fn list_installations<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("connectors/installations", query).await
    }

    /// Create a connector installation.
    pub async fn create_installation<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("connectors/installations", request).await
    }

    /// Fetch a connector installation.
    pub async fn get_installation<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "connectors/installations/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Update a connector installation.
    pub async fn update_installation<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.patch(
            format!("connectors/installations/{}", encode_path_segment(id)),
            request,
        )
        .await
    }

    /// Delete a connector installation.
    pub async fn delete_installation(&self, id: &str) -> Result<()> {
        self.delete_empty(format!(
            "connectors/installations/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Pause a connector installation.
    pub async fn pause_installation<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("connectors/installations/{}/pause", encode_path_segment(id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// Resume a paused connector installation.
    pub async fn resume_installation<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "connectors/installations/{}/resume",
                encode_path_segment(id)
            ),
            &serde_json::json!({}),
        )
        .await
    }

    /// Fetch a connector connection test by identifier.
    pub async fn get_connection_test<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "connectors/connection-tests/{}",
            encode_path_segment(id)
        ))
        .await
    }
}
