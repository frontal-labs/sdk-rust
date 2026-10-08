//! Async HTTP client and configuration.

use std::{env, time::Duration};

use reqwest::{header, Method, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
    constants::{DEFAULT_API_BASE_URL, DEFAULT_REQUEST_TIMEOUT},
    endpoints::EndpointPath,
    errors::{ApiError, Error},
    global::Result,
};

/// Configurable Frontal API client builder.
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> frontal_sdk::Result<()> {
/// let client = frontal_sdk::Client::builder("frt_your_api_key")
///     .timeout(std::time::Duration::from_secs(10))
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub struct ClientBuilder {
    api_key: String,
    base_url: Url,
    timeout: Duration,
}

impl ClientBuilder {
    /// Create a builder using the production API URL and default timeout.
    #[must_use]
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            // This constant is a valid URL.
            base_url: Url::parse(DEFAULT_API_BASE_URL).expect("valid default API URL"),
            timeout: DEFAULT_REQUEST_TIMEOUT,
        }
    }

    /// Read the API key from `FRONTAL_API_KEY` and create a builder.
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("FRONTAL_API_KEY").map_err(|_| {
            Error::Configuration("FRONTAL_API_KEY is not set or is not valid Unicode".to_owned())
        })?;
        Ok(Self::new(api_key))
    }

    /// Set the API base URL. The URL must use HTTP or HTTPS and must not have a query or fragment.
    pub fn base_url(mut self, base_url: &str) -> Result<Self> {
        let parsed = Url::parse(base_url)
            .map_err(|error| Error::Configuration(format!("invalid API base URL: {error}")))?;

        if !matches!(parsed.scheme(), "http" | "https")
            || parsed.host_str().is_none()
            || !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(Error::Configuration(
                "API base URL must be an HTTP(S) URL without credentials, query, or fragment"
                    .to_owned(),
            ));
        }

        self.base_url = ensure_trailing_slash(parsed);
        Ok(self)
    }

    /// Set the total request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build the configured client and its reusable connection pool.
    pub fn build(self) -> Result<Client> {
        if self.api_key.trim().is_empty() {
            return Err(Error::Configuration("API key must not be empty".to_owned()));
        }
        if self.timeout.is_zero() {
            return Err(Error::Configuration(
                "timeout must be greater than zero".to_owned(),
            ));
        }

        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .user_agent(concat!("frontal-sdk-rust/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Client {
            api_key: self.api_key,
            base_url: self.base_url,
            http,
        })
    }
}

impl std::fmt::Debug for ClientBuilder {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ClientBuilder")
            .field("api_key", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .finish()
    }
}

/// Async Frontal API client.
///
/// The client is cloneable and shares an underlying connection pool. Endpoint-specific
/// request and response types are supplied by callers and should follow the committed API
/// contracts.
pub struct Client {
    api_key: String,
    base_url: Url,
    http: reqwest::Client,
}

impl Client {
    /// Create a client builder for the supplied API key.
    #[must_use]
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    /// Create a client using the default API URL and timeout.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key).build()
    }

    /// Create a client using `FRONTAL_API_KEY`.
    pub fn from_env() -> Result<Self> {
        ClientBuilder::from_env()?.build()
    }

    /// Send a GET request and decode its JSON response.
    pub async fn get<T>(&self, path: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.send_json(Method::GET, path.as_ref(), None, None).await
    }

    /// Send a GET request with URL-encoded query parameters and decode its JSON response.
    pub async fn get_with_query<T>(
        &self,
        path: impl AsRef<str>,
        query: &[(&str, &str)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.send_json(Method::GET, path.as_ref(), Some(query), None)
            .await
    }

    /// Send a JSON POST request and decode its JSON response.
    pub async fn post<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.send_json(Method::POST, path.as_ref(), None, Some(serialize(body)?))
            .await
    }

    /// Send a JSON PUT request and decode its JSON response.
    pub async fn put<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.send_json(Method::PUT, path.as_ref(), None, Some(serialize(body)?))
            .await
    }

    /// Send a JSON PATCH request and decode its JSON response.
    pub async fn patch<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.send_json(Method::PATCH, path.as_ref(), None, Some(serialize(body)?))
            .await
    }

    /// Send a DELETE request and decode its JSON response.
    pub async fn delete<T>(&self, path: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.send_json(Method::DELETE, path.as_ref(), None, None)
            .await
    }

    /// Send a DELETE request whose response has no body, such as HTTP 204.
    pub async fn delete_empty(&self, path: impl AsRef<str>) -> Result<()> {
        self.send_empty(Method::DELETE, path.as_ref(), None, None)
            .await
    }

    /// Send a request whose successful response has no body.
    pub async fn post_empty<B>(&self, path: impl AsRef<str>, body: &B) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        self.send_empty(Method::POST, path.as_ref(), None, Some(serialize(body)?))
            .await
    }

    async fn send_json<T>(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<Vec<u8>>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.send(method, path, query, body).await?;
        let bytes = response.bytes().await?;
        serde_json::from_slice(&bytes).map_err(Error::Decode)
    }

    async fn send_empty(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<Vec<u8>>,
    ) -> Result<()> {
        let response = self.send(method, path, query, body).await?;
        drop(response);
        Ok(())
    }

    async fn send(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<Vec<u8>>,
    ) -> Result<reqwest::Response> {
        let url = self.endpoint_url(path)?;
        let mut request = self.http.request(method, url).bearer_auth(&self.api_key);

        if let Some(query) = query {
            request = request.query(query);
        }

        if let Some(body) = body {
            request = request
                .header(header::CONTENT_TYPE, "application/json")
                .body(body);
        }

        let response = request.send().await?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(api_error(response).await)
        }
    }

    fn endpoint_url(&self, path: &str) -> Result<Url> {
        let endpoint = EndpointPath::new(path)?;
        crate::endpoints::join(&self.base_url, &endpoint)
    }
}

fn serialize<B>(body: &B) -> Result<Vec<u8>>
where
    B: Serialize + ?Sized,
{
    serde_json::to_vec(body).map_err(Error::Serialization)
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            base_url: self.base_url.clone(),
            http: self.http.clone(),
        }
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Client")
            .field("api_key", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .finish_non_exhaustive()
    }
}

async fn api_error(response: reqwest::Response) -> Error {
    let status = response.status();
    let request_id = response
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let body = response
        .bytes()
        .await
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok());
    Error::Api(ApiError::new(status, request_id, body))
}

fn ensure_trailing_slash(mut url: Url) -> Url {
    if !url.path().ends_with('/') {
        url.path_segments_mut()
            .expect("HTTP URLs support path segments")
            .push("");
    }
    url
}
