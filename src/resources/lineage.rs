//! Service scoped access for the `lineage` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `lineage` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct LineageService;

/// Typed access to the `lineage` service.
pub type Lineage = ServiceClient<LineageService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "lineage";

impl ServiceClient<LineageService> {
    /// Query the lineage graph.
    pub async fn graph<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("lineage/graph", query).await
    }

    /// List lineage nodes.
    pub async fn list_nodes<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("lineage/nodes", query).await
    }

    /// Fetch a lineage node.
    pub async fn get_node<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("lineage/nodes/{}", encode_path_segment(id)))
            .await
    }

    /// Trace dependencies from a lineage node.
    pub async fn trace_node<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("lineage/nodes/{}/trace", encode_path_segment(id)))
            .await
    }

    /// List lineage edges.
    pub async fn list_edges<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("lineage/edges", query).await
    }

    /// Fetch a lineage edge.
    pub async fn get_edge<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("lineage/edges/{}", encode_path_segment(id)))
            .await
    }

    /// Analyze the impact of a proposed change.
    pub async fn analyze_impact<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("lineage/impact", request).await
    }
}
