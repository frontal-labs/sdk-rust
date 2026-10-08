//! Service scoped access for the `observability` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `observability` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct ObservabilityService;

/// Typed access to the `observability` service.
pub type Observability = ServiceClient<ObservabilityService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "observability";

impl ServiceClient<ObservabilityService> {
    /// Query structured logs.
    pub async fn query_logs<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("observability/logs/query", request).await
    }

    /// Ingest structured log entries.
    pub async fn ingest_logs<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("observability/logs/ingest", request).await
    }

    /// Watch the live log stream using server-sent events.
    #[cfg(feature = "stream")]
    pub async fn watch_logs(
        &self,
    ) -> Result<impl futures_util::Stream<Item = Result<crate::api::ServerSentEvent>> + Send> {
        self.events("observability/logs/stream").await
    }

    /// Fetch a metrics summary.
    pub async fn metrics<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("observability/metrics", query).await
    }

    /// List metrics.
    pub async fn list_metrics<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("observability/metrics/list", query)
            .await
    }

    /// Ingest metric points.
    pub async fn ingest_metrics<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("observability/metrics/ingest", request).await
    }

    /// Fetch a trace by identifier.
    pub async fn get_trace<T>(&self, trace_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "observability/traces/{}",
            encode_path_segment(trace_id)
        ))
        .await
    }

    /// List traces.
    pub async fn list_traces<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("observability/traces", query).await
    }

    /// Query traces using a request filter.
    pub async fn query_traces<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("observability/traces/query", request).await
    }

    /// List alert rules.
    pub async fn list_alerts<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("observability/alerts", query).await
    }

    /// Create an alert rule.
    pub async fn create_alert<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("observability/alerts", request).await
    }

    /// Update an alert rule.
    pub async fn update_alert<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.put(
            format!("observability/alerts/{}", encode_path_segment(id)),
            request,
        )
        .await
    }

    /// Delete an alert rule.
    pub async fn delete_alert(&self, id: &str) -> Result<()> {
        self.delete_empty(format!("observability/alerts/{}", encode_path_segment(id)))
            .await
    }

    /// Enable an alert rule.
    pub async fn enable_alert<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("observability/alerts/{}/enable", encode_path_segment(id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// Disable an alert rule.
    pub async fn disable_alert<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("observability/alerts/{}/disable", encode_path_segment(id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// List alert incidents.
    pub async fn list_incidents<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("observability/alerts/incidents", query)
            .await
    }

    /// List observability dashboards.
    pub async fn list_dashboards<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("observability/dashboards").await
    }

    /// Fetch a dashboard by identifier.
    pub async fn get_dashboard<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "observability/dashboards/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Create a dashboard.
    pub async fn create_dashboard<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("observability/dashboards", request).await
    }

    /// Update a dashboard.
    pub async fn update_dashboard<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.put(
            format!("observability/dashboards/{}", encode_path_segment(id)),
            request,
        )
        .await
    }

    /// Delete a dashboard.
    pub async fn delete_dashboard(&self, id: &str) -> Result<()> {
        self.delete_empty(format!(
            "observability/dashboards/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Create a share link for a dashboard.
    pub async fn share_dashboard<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!("observability/dashboards/{}/share", encode_path_segment(id)),
            request,
        )
        .await
    }

    /// Report an observability event.
    pub async fn report_event<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("observability/events", request).await
    }

    /// Report a batch of observability events.
    pub async fn report_event_batch<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("observability/events/batch", request).await
    }

    /// Fetch observability event statistics.
    pub async fn event_stats<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("observability/events/stats", query)
            .await
    }
}
