//! Service scoped access for the `data` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `data` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct DataService;

/// Typed access to the `data` service.
pub type Data = ServiceClient<DataService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "data";

impl ServiceClient<DataService> {
    /// List configured aggregation definitions.
    pub async fn list_aggregations<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/aggregations/aggregations", query)
            .await
    }

    /// Create an aggregation definition.
    pub async fn create_aggregation<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/aggregations/aggregations", request).await
    }

    /// Fetch an aggregation definition.
    pub async fn get_aggregation<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "data/aggregations/aggregations/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Execute an aggregation.
    pub async fn execute_aggregation<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/aggregations/aggregations/{}/executions",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }

    /// List archival policies.
    pub async fn list_archival_policies<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/archival/archival/policies", query)
            .await
    }

    /// Create an archival policy.
    pub async fn create_archival_policy<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/archival/archival/policies", request).await
    }

    /// Fetch an archival policy.
    pub async fn get_archival_policy<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "data/archival/archival/policies/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Execute an archival policy.
    pub async fn execute_archival_policy<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/archival/archival/policies/{}/executions",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }

    /// List enrichment profiles.
    pub async fn list_enrichment_profiles<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/enrichment/enrichment/profiles", query)
            .await
    }

    /// Create an enrichment profile.
    pub async fn create_enrichment_profile<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/enrichment/enrichment/profiles", request)
            .await
    }

    /// Fetch an enrichment profile.
    pub async fn get_enrichment_profile<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "data/enrichment/enrichment/profiles/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Execute an enrichment profile.
    pub async fn execute_enrichment_profile<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/enrichment/enrichment/profiles/{}/executions",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }

    /// List data export definitions.
    pub async fn list_exports<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/exports/exports", query).await
    }

    /// Create a data export.
    pub async fn create_export<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/exports/exports", request).await
    }

    /// Fetch a data export.
    pub async fn get_export<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("data/exports/exports/{}", encode_path_segment(id)))
            .await
    }

    /// Execute a data export.
    pub async fn execute_export<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/exports/exports/{}/executions",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }

    /// List normalization profiles.
    pub async fn list_normalization_profiles<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/normalization/normalization/profiles", query)
            .await
    }

    /// Create a normalization profile.
    pub async fn create_normalization_profile<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/normalization/normalization/profiles", request)
            .await
    }

    /// Fetch a normalization profile.
    pub async fn get_normalization_profile<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "data/normalization/normalization/profiles/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Execute a normalization profile.
    pub async fn execute_normalization_profile<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/normalization/normalization/profiles/{}/executions",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }

    /// List quality rulesets.
    pub async fn list_quality_rulesets<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/quality/quality/rulesets", query)
            .await
    }

    /// Create a quality ruleset.
    pub async fn create_quality_ruleset<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/quality/quality/rulesets", request).await
    }

    /// Fetch a quality ruleset.
    pub async fn get_quality_ruleset<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "data/quality/quality/rulesets/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Evaluate a quality ruleset.
    pub async fn evaluate_quality_ruleset<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/quality/quality/rulesets/{}/evaluations",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }

    /// Run a federated data query.
    pub async fn federated_query<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/query/query/federated", request).await
    }

    /// List data schemas.
    pub async fn list_schemas<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/schemas/schemas", query).await
    }

    /// Create a data schema.
    pub async fn create_schema<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/schemas/schemas", request).await
    }

    /// Resolve a schema reference.
    pub async fn resolve_schema<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/schemas/schemas/resolve", request).await
    }

    /// Fetch a schema by reference.
    pub async fn get_schema<T>(&self, schema_ref: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "data/schemas/schemas/{}",
            encode_path_segment(schema_ref)
        ))
        .await
    }

    /// List serving products.
    pub async fn list_serving_products<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/serving/serving/products", query)
            .await
    }

    /// Create a serving product.
    pub async fn create_serving_product<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/serving/serving/products", request).await
    }

    /// Fetch a serving product.
    pub async fn get_serving_product<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "data/serving/serving/products/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Refresh a serving product.
    pub async fn refresh_serving_product<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/serving/serving/products/{}/refreshes",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }

    /// List data streams.
    pub async fn list_streams<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/streams/streams", query).await
    }

    /// Create a data stream.
    pub async fn create_stream<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/streams/streams", request).await
    }

    /// Fetch a data stream.
    pub async fn get_stream<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("data/streams/streams/{}", encode_path_segment(id)))
            .await
    }

    /// Create deliveries for a data stream.
    pub async fn create_stream_deliveries<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/streams/streams/{}/deliveries",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }

    /// List data synchronization jobs.
    pub async fn list_sync_jobs<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/sync/sync/jobs", query).await
    }

    /// Create a data synchronization job.
    pub async fn create_sync_job<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/sync/sync/jobs", request).await
    }

    /// Fetch a data synchronization job.
    pub async fn get_sync_job<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("data/sync/sync/jobs/{}", encode_path_segment(id)))
            .await
    }

    /// Execute a data synchronization job.
    pub async fn execute_sync_job<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!("data/sync/sync/jobs/{}/executions", encode_path_segment(id)),
            request,
        )
        .await
    }

    /// List data transformations.
    pub async fn list_transformations<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/transformations/transformations", query)
            .await
    }

    /// Create a data transformation.
    pub async fn create_transformation<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("data/transformations/transformations", request)
            .await
    }

    /// Fetch a data transformation.
    pub async fn get_transformation<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "data/transformations/transformations/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// Execute a data transformation.
    pub async fn execute_transformation<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "data/transformations/transformations/{}/executions",
                encode_path_segment(id)
            ),
            request,
        )
        .await
    }
}
