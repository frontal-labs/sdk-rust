//! Typed, service-scoped access to the Frontal API.
//!
//! Each service handle shares the same client and exposes verb methods with
//! caller-selected Serde request and response types. Paths are relative to the
//! configured API base URL and should match `contracts/sdk-endpoints.json`.

use std::{collections::BTreeMap, marker::PhantomData, sync::OnceLock, time::Duration};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{api::Client, endpoints::EndpointPath, errors::FrontalError, global::Result};

#[derive(Debug, Deserialize)]
struct ContractEndpoint {
    method: String,
    path: String,
}

type ContractInventory = BTreeMap<String, Vec<ContractEndpoint>>;

static CONTRACT_INVENTORY: OnceLock<std::result::Result<ContractInventory, String>> =
    OnceLock::new();

fn contract_inventory() -> Result<&'static ContractInventory> {
    match CONTRACT_INVENTORY.get_or_init(|| {
        serde_json::from_str(include_str!("../../contracts/sdk-endpoints.json"))
            .map_err(|error| error.to_string())
    }) {
        Ok(inventory) => Ok(inventory),
        Err(error) => Err(FrontalError::Configuration(format!(
            "could not read the SDK endpoint inventory: {error}"
        ))),
    }
}

fn validate_route(
    service: &str,
    method: &str,
    path: &str,
    query: &[(&str, &str)],
) -> Result<EndpointPath> {
    let endpoint = EndpointPath::new(path)?;
    let inventory = contract_inventory()?;
    let supported = inventory.get(service).is_some_and(|routes| {
        routes.iter().any(|route| {
            route.method == method
                && route_matches(&route.path, endpoint.as_str())
                && required_query_matches(&route.path, query)
        })
    });

    if supported {
        Ok(endpoint)
    } else {
        Err(FrontalError::InvalidPath(format!(
            "the {service} service does not support {method} {} in the endpoint inventory",
            endpoint.as_str()
        )))
    }
}

fn route_matches(template: &str, path: &str) -> bool {
    let template_path = template.split_once('?').map_or(template, |(path, _)| path);
    let template_segments = template_path.trim_start_matches('/').split('/');
    let path_segments = path.split('/');
    let mut template_segments = template_segments.peekable();
    let mut path_segments = path_segments.peekable();

    loop {
        match (template_segments.next(), path_segments.next()) {
            (Some(template), Some(actual))
                if template.starts_with('{') && template.ends_with('}') && !actual.is_empty() => {}
            (Some(template), Some(actual)) if template == actual => {}
            (None, None) => return true,
            _ => return false,
        }
    }
}

fn required_query_matches(path: &str, query: &[(&str, &str)]) -> bool {
    let Some((_, required)) = path.split_once('?') else {
        return true;
    };

    required.split('&').all(|pair| {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        query.contains(&(key, value))
    })
}

/// Shared implementation behind each service-specific handle.
#[derive(Clone, Debug)]
pub struct ServiceClient<S> {
    client: Client,
    service_name: &'static str,
    service: PhantomData<fn() -> S>,
}

impl<S> ServiceClient<S> {
    pub(crate) fn new(client: Client, service_name: &'static str) -> Self {
        Self {
            client,
            service_name,
            service: PhantomData,
        }
    }

    fn validate(&self, method: &str, path: &str, query: &[(&str, &str)]) -> Result<EndpointPath> {
        validate_route(self.service_name, method, path, query)
    }

    /// Send a GET request to a contract-relative path.
    pub async fn get<T>(&self, path: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("GET", path.as_ref(), &[])?;
        self.client.get(&endpoint).await
    }

    /// Send a GET request with URL-encoded query parameters.
    pub async fn get_with_query<T>(
        &self,
        path: impl AsRef<str>,
        query: &[(&str, &str)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("GET", path.as_ref(), query)?;
        self.client.get_with_query(&endpoint, query).await
    }

    /// Send a GET request using a caller-provided bearer token.
    pub async fn get_with_bearer_auth<T>(
        &self,
        path: impl AsRef<str>,
        bearer_token: &str,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("GET", path.as_ref(), &[])?;
        self.client
            .get_with_bearer_auth(endpoint, bearer_token)
            .await
    }

    /// Send a typed JSON POST request.
    pub async fn post<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POST", path.as_ref(), &[])?;
        self.client.post(&endpoint, body).await
    }

    /// Send a JSON POST using an idempotency key for safe retries.
    pub async fn post_idempotent<B, T>(
        &self,
        path: impl AsRef<str>,
        idempotency_key: &str,
        body: &B,
    ) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POST", path.as_ref(), &[])?;
        self.client
            .post_idempotent(&endpoint, idempotency_key, body)
            .await
    }

    /// Send a typed JSON POST request with URL-encoded query parameters.
    pub async fn post_with_query<B, T>(
        &self,
        path: impl AsRef<str>,
        query: &[(&str, &str)],
        body: &B,
    ) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POST", path.as_ref(), query)?;
        self.client.post_with_query(&endpoint, query, body).await
    }

    /// Send a JSON POST with query parameters and an idempotency key.
    pub async fn post_idempotent_with_query<B, T>(
        &self,
        path: impl AsRef<str>,
        query: &[(&str, &str)],
        idempotency_key: &str,
        body: &B,
    ) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POST", path.as_ref(), query)?;
        self.client
            .post_idempotent_with_query(&endpoint, query, idempotency_key, body)
            .await
    }

    /// Send a raw POST body and return response bytes.
    pub async fn post_bytes(
        &self,
        path: impl AsRef<str>,
        content_type: &str,
        body: impl AsRef<[u8]>,
    ) -> Result<Vec<u8>> {
        let endpoint = self.validate("POSTRAW", path.as_ref(), &[])?;
        self.client.post_bytes(&endpoint, content_type, body).await
    }

    /// Send a multipart form and decode its JSON response.
    pub async fn post_form<T>(
        &self,
        path: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("POSTFORMDATA", path.as_ref(), &[])?;
        self.client.post_form(&endpoint, form).await
    }

    /// Send a multipart form whose successful response has no body.
    pub async fn post_form_empty(
        &self,
        path: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> Result<()> {
        let endpoint = self.validate("POSTFORMDATA", path.as_ref(), &[])?;
        self.client.post_form_empty(&endpoint, form).await
    }

    /// Send a typed JSON PUT request.
    pub async fn put<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("PUT", path.as_ref(), &[])?;
        self.client.put(&endpoint, body).await
    }

    /// Send a typed JSON PATCH request.
    pub async fn patch<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("PATCH", path.as_ref(), &[])?;
        self.client.patch(&endpoint, body).await
    }

    /// Send a DELETE request and decode its JSON response.
    pub async fn delete<T>(&self, path: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("DELETE", path.as_ref(), &[])?;
        self.client.delete(&endpoint).await
    }

    /// Send a DELETE request with URL-encoded query parameters.
    pub async fn delete_with_query<T>(
        &self,
        path: impl AsRef<str>,
        query: &[(&str, &str)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("DELETE", path.as_ref(), query)?;
        self.client.delete_with_query(&endpoint, query).await
    }

    /// Send a DELETE request with a JSON body.
    pub async fn delete_with_body<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let endpoint = self.validate("DELETE", path.as_ref(), &[])?;
        self.client.delete_with_body(&endpoint, body).await
    }

    /// Send a DELETE request with an empty successful response.
    pub async fn delete_empty(&self, path: impl AsRef<str>) -> Result<()> {
        let endpoint = self.validate("DELETE", path.as_ref(), &[])?;
        self.client.delete_empty(&endpoint).await
    }

    /// Send a JSON POST request with an empty successful response.
    pub async fn post_empty<B>(&self, path: impl AsRef<str>, body: &B) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        let endpoint = self.validate("POST", path.as_ref(), &[])?;
        self.client.post_empty(&endpoint, body).await
    }

    /// Send an idempotent JSON POST with an empty successful response.
    pub async fn post_empty_idempotent<B>(
        &self,
        path: impl AsRef<str>,
        idempotency_key: &str,
        body: &B,
    ) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        let endpoint = self.validate("POST", path.as_ref(), &[])?;
        self.client
            .post_empty_idempotent(&endpoint, idempotency_key, body)
            .await
    }

    /// Download a raw response body, for example a PDF or object payload.
    pub async fn get_bytes(&self, path: impl AsRef<str>) -> Result<Vec<u8>> {
        let endpoint = self.validate("GETRAW", path.as_ref(), &[])?;
        self.client.get_bytes(&endpoint).await
    }

    /// Fetch every cursor page from a `{ data, next_cursor }` response.
    pub async fn paginate<T>(&self, path: impl AsRef<str>, limit: Option<usize>) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let endpoint = self.validate("GET", path.as_ref(), &[])?;
        self.client.paginate(&endpoint, limit).await
    }

    /// Poll a resource until `ready` returns true or the attempt limit is reached.
    pub async fn poll_until<T, F>(
        &self,
        path: impl AsRef<str>,
        max_attempts: usize,
        interval: Duration,
        ready: F,
    ) -> Result<Option<T>>
    where
        T: DeserializeOwned,
        F: Fn(&T) -> bool,
    {
        let endpoint = self.validate("GET", path.as_ref(), &[])?;
        self.client
            .poll_until(&endpoint, max_attempts, interval, ready)
            .await
    }

    /// Open and decode a server-sent event stream.
    #[cfg(feature = "stream")]
    pub async fn events(
        &self,
        path: impl AsRef<str>,
    ) -> Result<impl futures_util::Stream<Item = Result<crate::api::ServerSentEvent>> + Send> {
        let endpoint = self.validate("STREAM", path.as_ref(), &[])?;
        self.client.events(endpoint.as_str().to_owned()).await
    }
}

#[cfg(feature = "agents")]
pub mod agents;
#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "audit")]
pub mod audit;
#[cfg(feature = "auth")]
pub mod auth;
#[cfg(feature = "billing")]
pub mod billing;
#[cfg(feature = "blob")]
pub mod blob;
#[cfg(feature = "connectors")]
pub mod connectors;
#[cfg(feature = "data")]
pub mod data;
#[cfg(feature = "governance")]
pub mod governance;
#[cfg(feature = "lineage")]
pub mod lineage;
#[cfg(feature = "observability")]
pub mod observability;
#[cfg(feature = "ontology")]
pub mod ontology;
#[cfg(feature = "pipelines")]
pub mod pipelines;
#[cfg(feature = "sandbox")]
pub mod sandbox;
#[cfg(feature = "schedules")]
pub mod schedules;
#[cfg(feature = "webhooks")]
pub mod webhooks;
#[cfg(feature = "workflows")]
pub mod workflows;

/// Service names surfaced by this crate and checked by `cargo xtask contract`.
pub const SDK_SERVICES: &[&str] = &[
    "agents",
    "ai",
    "audit",
    "auth",
    "billing",
    "blob",
    "connectors",
    "data",
    "governance",
    "lineage",
    "observability",
    "ontology",
    "pipelines",
    "sandbox",
    "schedules",
    "webhooks",
    "workflows",
];

#[cfg(test)]
mod tests {
    use super::validate_route;

    #[test]
    fn validates_service_method_path_and_required_query() {
        assert!(validate_route("agents", "GET", "agents", &[]).is_ok());
        assert!(validate_route("agents", "GET", "agents/agent-1", &[]).is_ok());
        assert!(validate_route("ai", "GET", "agents", &[]).is_err());
        assert!(validate_route("agents", "PATCH", "agents", &[]).is_err());
        assert!(validate_route("auth", "POST", "auth/token", &[]).is_err());
        assert!(
            validate_route("auth", "POST", "auth/token", &[("grant_type", "password")]).is_ok()
        );
    }
}
