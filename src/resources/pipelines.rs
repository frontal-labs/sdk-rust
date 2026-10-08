//! Service scoped access for the `pipelines` API.

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `pipelines` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct PipelinesService;

/// Typed access to the `pipelines` service.
pub type Pipelines = ServiceClient<PipelinesService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "pipelines";

fn pipeline_command<B: Serialize + ?Sized>(
    operation: &str,
    pipeline_id: Option<&str>,
    payload: &B,
) -> Result<Value> {
    let mut command =
        match serde_json::to_value(payload).map_err(crate::FrontalError::Serialization)? {
            Value::Object(fields) => fields,
            _ => Map::new(),
        };
    command.insert("operation".to_owned(), Value::String(operation.to_owned()));
    if let Some(pipeline_id) = pipeline_id {
        command.insert(
            "pipelineId".to_owned(),
            Value::String(pipeline_id.to_owned()),
        );
    }
    Ok(Value::Object(command))
}

impl ServiceClient<PipelinesService> {
    /// List pipeline definitions.
    pub async fn list_pipelines<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut options = vec![("operation", "pipelines.list")];
        options.extend_from_slice(query);
        self.get_with_query("data/pipelines/pipelines", &options)
            .await
    }

    /// Create a pipeline from its definition.
    pub async fn create_pipeline<B, T>(&self, definition: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let body = serde_json::json!({"definition": definition});
        let command = pipeline_command("pipelines.create", None, &body)?;
        self.post("data/pipelines/pipelines", &command).await
    }

    /// Fetch pipeline service capabilities.
    pub async fn pipeline_capabilities<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("data/pipelines/capabilities").await
    }

    /// Fetch a pipeline by identifier.
    pub async fn get_pipeline<T>(&self, pipeline_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let query = [("operation", "pipelines.get")];
        self.get_with_query(
            format!(
                "data/pipelines/pipelines/{}",
                encode_path_segment(pipeline_id)
            ),
            &query,
        )
        .await
    }

    /// Update a pipeline definition.
    pub async fn update_pipeline<B, T>(&self, pipeline_id: &str, definition: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let payload = serde_json::json!({"definition": definition});
        let command = pipeline_command("pipelines.update", Some(pipeline_id), &payload)?;
        self.post("data/pipelines/pipelines", &command).await
    }

    /// Delete a pipeline.
    pub async fn delete_pipeline(&self, pipeline_id: &str) -> Result<()> {
        let command = pipeline_command(
            "pipelines.delete",
            Some(pipeline_id),
            &serde_json::json!({}),
        )?;
        self.post_empty("data/pipelines/runs", &command).await?;
        Ok(())
    }

    /// List runs for one pipeline.
    pub async fn list_pipeline_runs<T>(
        &self,
        pipeline_id: &str,
        query: &[(&str, &str)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut options = vec![
            ("operation", "pipelines.runs.list"),
            ("pipelineId", pipeline_id),
        ];
        options.extend_from_slice(query);
        self.get_with_query("data/pipelines/pipeline-runs", &options)
            .await
    }

    /// Fetch one pipeline run.
    pub async fn get_pipeline_run<T>(&self, pipeline_id: &str, run_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let query = [
            ("operation", "pipelines.runs.get"),
            ("pipelineId", pipeline_id),
        ];
        self.get_with_query(
            format!(
                "data/pipelines/pipeline-runs/{}",
                encode_path_segment(run_id)
            ),
            &query,
        )
        .await
    }

    /// Trigger a new pipeline run.
    pub async fn trigger_pipeline_run<B, T>(&self, pipeline_id: &str, input: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let command = pipeline_command("pipelines.runs.trigger", Some(pipeline_id), input)?;
        self.post("data/pipelines/runs", &command).await
    }

    /// Create and activate a pipeline.
    pub async fn activate_pipeline<T>(&self, pipeline_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let command = pipeline_command(
            "pipelines.activate",
            Some(pipeline_id),
            &serde_json::json!({}),
        )?;
        self.post("data/pipelines/runs", &command).await
    }

    /// List backfills for one pipeline.
    pub async fn list_backfills<T>(&self, pipeline_id: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut options = vec![
            ("operation", "pipelines.backfills.list"),
            ("pipelineId", pipeline_id),
        ];
        options.extend_from_slice(query);
        self.get_with_query("data/pipelines/runs", &options).await
    }

    /// Create a backfill for one pipeline.
    pub async fn create_backfill<B, T>(&self, pipeline_id: &str, input: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let command = pipeline_command("pipelines.backfills.create", Some(pipeline_id), input)?;
        self.post("data/pipelines/runs", &command).await
    }

    /// Fetch pipeline health.
    pub async fn pipeline_health<T>(&self, pipeline_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let query = [
            ("operation", "pipelines.health"),
            ("pipelineId", pipeline_id),
        ];
        self.get_with_query("data/pipelines/health", &query).await
    }

    /// Fetch pipeline service information, including lineage graph data.
    pub async fn pipeline_info<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("data/pipelines/info", query).await
    }

    /// Watch a pipeline run with server-sent events.
    #[cfg(feature = "stream")]
    pub async fn watch_pipeline_run(
        &self,
        run_id: &str,
    ) -> Result<impl futures_util::Stream<Item = Result<crate::api::ServerSentEvent>> + Send> {
        self.events(format!(
            "data/pipelines/pipeline-runs/{}",
            encode_path_segment(run_id)
        ))
        .await
    }
}
