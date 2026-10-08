//! Service scoped access for the `agents` API.

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::ServiceClient;
use crate::{
    encode_path_segment,
    global::Result,
    models::agents::{
        Agent, AgentConversation, AgentDefinition, AgentExecution, AgentPage, AgentRollbackRequest,
        AgentRunRequest,
    },
};

/// Type marker for the `agents` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct AgentService;

/// Typed access to the `agents` service.
pub type Agents = ServiceClient<AgentService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "agents";

impl ServiceClient<AgentService> {
    /// List agents using the API's cursor and filter query parameters.
    pub async fn list_agents(&self, query: &[(&str, &str)]) -> Result<AgentPage> {
        let endpoint = self.validate("GET", "agents", query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Create an agent from a contract-shaped definition.
    pub async fn create_agent(&self, definition: &AgentDefinition) -> Result<Agent> {
        let endpoint = self.validate("POST", "agents", &[])?;
        self.client.post(endpoint, definition).await
    }

    /// Return the Agents service health response.
    pub async fn health(&self) -> Result<Value> {
        let endpoint = self.validate("GET", "agents/health", &[])?;
        self.client.get(endpoint).await
    }

    /// Fetch an agent by its identifier.
    pub async fn get_agent(&self, id: &str) -> Result<Agent> {
        let path = format!("agents/{}", encode_path_segment(id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Replace an agent definition.
    pub async fn update_agent(&self, id: &str, definition: &AgentDefinition) -> Result<Agent> {
        let path = format!("agents/{}", encode_path_segment(id));
        let endpoint = self.validate("PUT", &path, &[])?;
        self.client.put(endpoint, definition).await
    }

    /// Delete an agent.
    pub async fn delete_agent(&self, id: &str) -> Result<()> {
        let path = format!("agents/{}", encode_path_segment(id));
        let endpoint = self.validate("DELETE", &path, &[])?;
        self.client.delete_empty(endpoint).await
    }

    /// Roll back an agent to a requested version.
    pub async fn rollback_agent(&self, id: &str, request: &AgentRollbackRequest) -> Result<Agent> {
        let path = format!("agents/{}/rollback", encode_path_segment(id));
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post(endpoint, request).await
    }

    /// Start an agent run with an event and payload.
    pub async fn create_run(&self, id: &str, request: &AgentRunRequest) -> Result<AgentExecution> {
        let path = format!("agents/{}/runs", encode_path_segment(id));
        let endpoint = self.validate("POST", &path, &[])?;
        self.client.post(endpoint, request).await
    }

    /// Fetch an agent run by its identifier.
    pub async fn get_run(&self, run_id: &str) -> Result<AgentExecution> {
        let path = format!("agents/runs/{}", encode_path_segment(run_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Fetch the conversation transcript for an agent run.
    pub async fn conversation(&self, run_id: &str) -> Result<AgentConversation> {
        let path = format!("agents/runs/{}/conversation", encode_path_segment(run_id));
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get(endpoint).await
    }

    /// Fetch an agent's versions using caller-selected response types.
    pub async fn versions<T>(&self, id: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!("agents/{}/versions", encode_path_segment(id));
        let endpoint = self.validate("GET", &path, query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Fetch runs for an agent using caller-selected response types.
    pub async fn runs<T>(&self, id: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!("agents/{}/runs", encode_path_segment(id));
        let endpoint = self.validate("GET", &path, query)?;
        self.client.get_with_query(endpoint, query).await
    }

    /// Watch an agent run through the service's SSE endpoint.
    #[cfg(feature = "stream")]
    pub async fn watch_run(
        &self,
        run_id: &str,
    ) -> Result<impl futures_util::Stream<Item = Result<crate::api::ServerSentEvent>> + Send> {
        let path = format!("agents/runs/{}/stream", encode_path_segment(run_id));
        let endpoint = self.validate("STREAM", &path, &[])?;
        self.client.events(endpoint.as_str().to_owned()).await
    }
}
