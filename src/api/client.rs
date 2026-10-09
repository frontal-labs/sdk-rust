//! Async HTTP client, configuration, retries, pagination, and streaming.

use std::{
    collections::HashSet,
    env,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime},
};

use reqwest::{header, Method, Response, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    constants::{DEFAULT_API_BASE_URL, DEFAULT_REQUEST_TIMEOUT},
    endpoints::EndpointPath,
    errors::{classify_api_error, ApiError, FrontalError},
    global::Result,
};

static REQUEST_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy)]
struct RetrySettings<'a> {
    idempotency_key: Option<&'a str>,
    retry_safe: bool,
    bearer_token: Option<&'a str>,
}

/// Retry configuration for retry-safe requests.
#[derive(Clone, Copy, Debug)]
pub struct RetryPolicy {
    /// Maximum additional attempts after the initial request.
    pub max_retries: u32,
    /// Delay before the first retry.
    pub initial_backoff: Duration,
    /// Upper bound for exponential and `Retry-After` delays.
    pub max_backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 2,
            initial_backoff: Duration::from_millis(200),
            max_backoff: Duration::from_secs(2),
        }
    }
}

/// Configurable Frontal API client builder.
#[derive(Clone)]
pub struct ClientBuilder {
    api_key: String,
    base_url: Url,
    timeout: Duration,
    retry_policy: RetryPolicy,
    environment: Option<String>,
    debug: bool,
}

impl ClientBuilder {
    /// Create a builder using the production API URL and default transport settings.
    #[must_use]
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: Url::parse(DEFAULT_API_BASE_URL).expect("valid default API URL"),
            timeout: DEFAULT_REQUEST_TIMEOUT,
            retry_policy: RetryPolicy::default(),
            environment: None,
            debug: false,
        }
    }

    /// Read `FRONTAL_API_KEY` and supported options from the process environment.
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("FRONTAL_API_KEY").map_err(|_| {
            FrontalError::Configuration(
                "FRONTAL_API_KEY is not set or is not valid Unicode".to_owned(),
            )
        })?;
        let mut builder = Self::new(api_key);
        if let Ok(base_url) = env::var("FRONTAL_API_URL") {
            builder = builder.base_url(&base_url)?;
        }
        if let Ok(timeout) = env::var("FRONTAL_TIMEOUT") {
            let millis = timeout.parse::<u64>().map_err(|_| {
                FrontalError::Configuration(
                    "FRONTAL_TIMEOUT must be an unsigned number of milliseconds".to_owned(),
                )
            })?;
            builder = builder.timeout(Duration::from_millis(millis));
        }
        builder.environment = env::var("FRONTAL_ENV").ok();
        builder.debug = env::var("FRONTAL_DEBUG")
            .map(|value| matches!(value.to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);
        Ok(builder)
    }

    /// Set the API base URL. It must be HTTP(S), without credentials, query, or fragment.
    pub fn base_url(mut self, base_url: &str) -> Result<Self> {
        let parsed = Url::parse(base_url).map_err(|error| {
            FrontalError::Configuration(format!("invalid API base URL: {error}"))
        })?;
        if !matches!(parsed.scheme(), "http" | "https")
            || parsed.host_str().is_none()
            || !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(FrontalError::Configuration(
                "API base URL must be an HTTP(S) URL without credentials, query, or fragment"
                    .to_owned(),
            ));
        }
        self.base_url = ensure_trailing_slash(parsed);
        Ok(self)
    }

    /// Set the total request timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the retry policy used for safe methods and JSON POST requests with an idempotency key.
    #[must_use]
    pub fn retry_policy(mut self, retry_policy: RetryPolicy) -> Self {
        self.retry_policy = retry_policy;
        self
    }

    /// Set the runtime environment label, usually `development`, `test`, or `production`.
    #[must_use]
    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// Enable the debug configuration flag.
    #[must_use]
    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    /// Build a client and its reusable connection pool.
    pub fn build(self) -> Result<Frontal> {
        if self.api_key.trim().is_empty() {
            return Err(FrontalError::Configuration(
                "API key must not be empty".to_owned(),
            ));
        }
        if self.timeout.is_zero() {
            return Err(FrontalError::Configuration(
                "timeout must be greater than zero".to_owned(),
            ));
        }
        if self.retry_policy.max_retries > 10 {
            return Err(FrontalError::Configuration(
                "max_retries must not exceed 10".to_owned(),
            ));
        }

        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .user_agent(concat!("frontal-rust/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Frontal {
            api_key: self.api_key,
            base_url: self.base_url,
            http,
            retry_policy: self.retry_policy,
            environment: self.environment,
            debug: self.debug,
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
            .field("retry_policy", &self.retry_policy)
            .field("environment", &self.environment)
            .field("debug", &self.debug)
            .finish()
    }
}

/// Shared async Frontal client. Clones share reqwest's connection pool.
pub struct Frontal {
    api_key: String,
    base_url: Url,
    http: reqwest::Client,
    retry_policy: RetryPolicy,
    environment: Option<String>,
    debug: bool,
}

impl Frontal {
    /// Create a builder for the supplied API key.
    #[must_use]
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    /// Create a client using the default API URL and timeout.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key).build()
    }

    /// Create a client from `FRONTAL_API_KEY` and optional environment settings.
    pub fn from_env() -> Result<Self> {
        ClientBuilder::from_env()?.build()
    }

    /// Configured API base URL.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Environment label loaded from the builder or `FRONTAL_ENV`.
    #[must_use]
    pub fn environment(&self) -> Option<&str> {
        self.environment.as_deref()
    }

    /// Whether debug mode was enabled through the builder or `FRONTAL_DEBUG`.
    #[must_use]
    pub const fn is_debug(&self) -> bool {
        self.debug
    }

    /// Send a GET request and decode its JSON response.
    pub async fn get<T>(&self, path: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.send_json(Method::GET, path.as_ref(), None, None, None, true)
            .await
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
        self.send_json(Method::GET, path.as_ref(), Some(query), None, None, true)
            .await
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
        let response = self
            .send(
                Method::GET,
                path.as_ref(),
                None,
                None,
                None,
                RetrySettings {
                    idempotency_key: None,
                    retry_safe: true,
                    bearer_token: Some(bearer_token),
                },
            )
            .await?;
        let bytes = response.bytes().await?;
        serde_json::from_slice(&bytes).map_err(FrontalError::Decode)
    }

    /// Send a typed JSON POST request.
    pub async fn post<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.send_json(
            Method::POST,
            path.as_ref(),
            None,
            Some(serialize(body)?),
            None,
            false,
        )
        .await
    }

    /// Send a JSON POST request with an idempotency key so transient failures can be retried.
    ///
    /// The same key is sent on every attempt. Use a key that uniquely identifies this logical
    /// operation and reuse it only when retrying that operation.
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
        self.send_json(
            Method::POST,
            path.as_ref(),
            None,
            Some(serialize(body)?),
            Some(idempotency_key),
            true,
        )
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
        self.send_json(
            Method::POST,
            path.as_ref(),
            Some(query),
            Some(serialize(body)?),
            None,
            false,
        )
        .await
    }

    /// Send a JSON POST request with query parameters and an idempotency key.
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
        self.send_json(
            Method::POST,
            path.as_ref(),
            Some(query),
            Some(serialize(body)?),
            Some(idempotency_key),
            true,
        )
        .await
    }

    /// Send a typed JSON PUT request.
    pub async fn put<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.send_json(
            Method::PUT,
            path.as_ref(),
            None,
            Some(serialize(body)?),
            None,
            true,
        )
        .await
    }

    /// Send a typed JSON PATCH request.
    pub async fn patch<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.send_json(
            Method::PATCH,
            path.as_ref(),
            None,
            Some(serialize(body)?),
            None,
            false,
        )
        .await
    }

    /// Send a DELETE request and decode its JSON response.
    pub async fn delete<T>(&self, path: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.send_json(Method::DELETE, path.as_ref(), None, None, None, true)
            .await
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
        self.send_json(Method::DELETE, path.as_ref(), Some(query), None, None, true)
            .await
    }

    /// Send a DELETE request with a JSON body.
    pub async fn delete_with_body<B, T>(&self, path: impl AsRef<str>, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.send_json(
            Method::DELETE,
            path.as_ref(),
            None,
            Some(serialize(body)?),
            None,
            true,
        )
        .await
    }

    /// Send a DELETE request whose successful response has no body.
    pub async fn delete_empty(&self, path: impl AsRef<str>) -> Result<()> {
        self.send_empty(Method::DELETE, path.as_ref(), None, None, None, true)
            .await
    }

    /// Send a JSON POST request whose successful response has no body.
    pub async fn post_empty<B>(&self, path: impl AsRef<str>, body: &B) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        self.send_empty(
            Method::POST,
            path.as_ref(),
            None,
            Some(serialize(body)?),
            None,
            false,
        )
        .await
    }

    /// Send an idempotent JSON POST request whose successful response has no body.
    pub async fn post_empty_idempotent<B>(
        &self,
        path: impl AsRef<str>,
        idempotency_key: &str,
        body: &B,
    ) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        self.send_empty(
            Method::POST,
            path.as_ref(),
            None,
            Some(serialize(body)?),
            Some(idempotency_key),
            true,
        )
        .await
    }

    /// Send a raw POST body and return the response body as bytes.
    pub async fn post_bytes(
        &self,
        path: impl AsRef<str>,
        content_type: &str,
        body: impl AsRef<[u8]>,
    ) -> Result<Vec<u8>> {
        let response = self
            .send(
                Method::POST,
                path.as_ref(),
                None,
                Some(body.as_ref().to_vec()),
                Some(content_type),
                RetrySettings {
                    idempotency_key: None,
                    retry_safe: false,
                    bearer_token: None,
                },
            )
            .await?;
        Ok(response.bytes().await?.to_vec())
    }

    /// Send a multipart form and decode the JSON response.
    pub async fn post_form<T>(
        &self,
        path: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.endpoint_url(path.as_ref())?;
        let request_id = next_request_id();
        let response = self
            .http
            .post(url)
            .bearer_auth(&self.api_key)
            .header("x-request-id", request_id.clone())
            .multipart(form)
            .send()
            .await?;
        let response = checked_response(response, request_id).await?;
        let bytes = response.bytes().await?;
        serde_json::from_slice(&bytes).map_err(FrontalError::Decode)
    }

    /// Send a multipart form whose successful response has no body.
    pub async fn post_form_empty(
        &self,
        path: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> Result<()> {
        let url = self.endpoint_url(path.as_ref())?;
        let request_id = next_request_id();
        let response = self
            .http
            .post(url)
            .bearer_auth(&self.api_key)
            .header("x-request-id", request_id.clone())
            .multipart(form)
            .send()
            .await?;
        checked_response(response, request_id).await?;
        Ok(())
    }

    /// Download the response body as bytes, for raw endpoints such as invoice PDFs.
    pub async fn get_bytes(&self, path: impl AsRef<str>) -> Result<Vec<u8>> {
        let response = self
            .send(
                Method::GET,
                path.as_ref(),
                None,
                None,
                None,
                RetrySettings {
                    idempotency_key: None,
                    retry_safe: true,
                    bearer_token: None,
                },
            )
            .await?;
        Ok(response.bytes().await?.to_vec())
    }

    /// Fetch every cursor page from a response shaped as `{ data, next_cursor }`.
    pub async fn paginate<T>(&self, path: impl AsRef<str>, limit: Option<usize>) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let path = path.as_ref();
        let mut cursor: Option<String> = None;
        let mut seen_cursors = HashSet::new();
        let mut items = Vec::new();
        loop {
            let mut owned_query = Vec::new();
            if let Some(limit) = limit {
                owned_query.push(("limit".to_owned(), limit.to_string()));
            }
            if let Some(value) = cursor.as_ref() {
                owned_query.push(("cursor".to_owned(), value.clone()));
            }
            let query = owned_query
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str()))
                .collect::<Vec<_>>();
            let page: Page<T> = self
                .send_json(Method::GET, path, Some(&query), None, None, true)
                .await?;
            items.extend(page.data);
            match page.next_cursor.filter(|value| !value.is_empty()) {
                Some(next) if seen_cursors.insert(next.clone()) => cursor = Some(next),
                Some(next) => {
                    return Err(FrontalError::Pagination(format!(
                        "server repeated the pagination cursor {next:?}"
                    )))
                }
                None => return Ok(items),
            }
        }
    }

    /// Poll a JSON resource until `ready` returns true or the attempt limit is reached.
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
        for attempt in 0..max_attempts {
            let value: T = self.get(path.as_ref()).await?;
            if ready(&value) {
                return Ok(Some(value));
            }
            if attempt + 1 < max_attempts {
                tokio::time::sleep(interval).await;
            }
        }
        Ok(None)
    }

    /// Open an SSE response and decode its events as a fallible async stream.
    #[cfg(feature = "stream")]
    pub async fn events(
        &self,
        path: impl AsRef<str>,
    ) -> Result<impl futures_util::Stream<Item = Result<ServerSentEvent>> + Send> {
        use eventsource_stream::Eventsource;
        use futures_util::StreamExt;

        let response = self
            .send(
                Method::GET,
                path.as_ref(),
                None,
                None,
                None,
                RetrySettings {
                    idempotency_key: None,
                    retry_safe: true,
                    bearer_token: None,
                },
            )
            .await?;
        let events = response.bytes_stream().eventsource().map(|item| {
            item.map(|event| ServerSentEvent {
                data: event.data,
                event: event.event,
                id: event.id,
                retry: event.retry,
            })
            .map_err(|error| FrontalError::Stream(error.to_string()))
        });
        Ok(events)
    }

    async fn send_json<T>(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<Vec<u8>>,
        idempotency_key: Option<&str>,
        retry_safe: bool,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let content_type = body.as_ref().map(|_| "application/json");
        let response = self
            .send(
                method,
                path,
                query,
                body,
                content_type,
                RetrySettings {
                    idempotency_key,
                    retry_safe,
                    bearer_token: None,
                },
            )
            .await?;
        let bytes = response.bytes().await?;
        serde_json::from_slice(&bytes).map_err(FrontalError::Decode)
    }

    async fn send_empty(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<Vec<u8>>,
        idempotency_key: Option<&str>,
        retry_safe: bool,
    ) -> Result<()> {
        let content_type = body.as_ref().map(|_| "application/json");
        self.send(
            method,
            path,
            query,
            body,
            content_type,
            RetrySettings {
                idempotency_key,
                retry_safe,
                bearer_token: None,
            },
        )
        .await?;
        Ok(())
    }

    async fn send(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<Vec<u8>>,
        content_type: Option<&str>,
        retry_settings: RetrySettings<'_>,
    ) -> Result<Response> {
        let url = self.endpoint_url(path)?;
        let request_id = next_request_id();
        let idempotency_key = retry_settings.idempotency_key;
        if idempotency_key.is_some_and(|key| key.trim().is_empty()) {
            return Err(FrontalError::Configuration(
                "idempotency key must not be empty".to_owned(),
            ));
        }
        let retry_safe = retry_settings.retry_safe
            || idempotency_key.is_some()
            || method == Method::GET
            || method == Method::HEAD;
        if self.debug {
            tracing::debug!(%method, %url, %request_id, "sending Frontal API request");
        }
        for attempt in 0..=self.retry_policy.max_retries {
            let mut request = self.http.request(method.clone(), url.clone());
            request = request.bearer_auth(retry_settings.bearer_token.unwrap_or(&self.api_key));
            request = request.header("x-request-id", &request_id);
            if let Some(idempotency_key) = idempotency_key {
                request = request.header("idempotency-key", idempotency_key);
            }
            if let Some(query) = query {
                request = request.query(query);
            }
            if let Some(body) = body.as_ref() {
                if let Some(content_type) = content_type {
                    request = request.header(header::CONTENT_TYPE, content_type);
                }
                request = request.body(body.clone());
            }

            match request.send().await {
                Ok(response) if response.status().is_success() => {
                    if self.debug {
                        tracing::debug!(%method, status = %response.status(), %request_id, "Frontal API request succeeded");
                    }
                    return Ok(response);
                }
                Ok(response) => {
                    let status = response.status();
                    let retry_after = retry_after(&response);
                    if self.debug {
                        tracing::debug!(%method, %status, %request_id, attempt, "Frontal API request returned an error status");
                    }
                    if retry_safe
                        && is_transient_status(status)
                        && attempt < self.retry_policy.max_retries
                    {
                        drop(response);
                        tokio::time::sleep(self.retry_delay(attempt, retry_after)).await;
                        continue;
                    }
                    return checked_response(response, request_id.clone()).await;
                }
                Err(error)
                    if retry_safe
                        && (error.is_timeout() || error.is_connect())
                        && attempt < self.retry_policy.max_retries =>
                {
                    tokio::time::sleep(self.retry_delay(attempt, None)).await;
                }
                Err(error) => return Err(FrontalError::Transport(error)),
            }
        }
        unreachable!("retry loop always returns or continues within its bound")
    }

    fn retry_delay(&self, attempt: u32, retry_after: Option<Duration>) -> Duration {
        if let Some(delay) = retry_after {
            return delay.min(self.retry_policy.max_backoff);
        }
        let multiplier = 1u32.checked_shl(attempt.min(31)).unwrap_or(u32::MAX);
        self.retry_policy
            .initial_backoff
            .checked_mul(multiplier)
            .unwrap_or(self.retry_policy.max_backoff)
            .min(self.retry_policy.max_backoff)
    }

    fn endpoint_url(&self, path: &str) -> Result<Url> {
        let endpoint = EndpointPath::new(path)?;
        crate::endpoints::join(&self.base_url, &endpoint)
    }
}

impl Clone for Frontal {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            base_url: self.base_url.clone(),
            http: self.http.clone(),
            retry_policy: self.retry_policy,
            environment: self.environment.clone(),
            debug: self.debug,
        }
    }
}

impl std::fmt::Debug for Frontal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Frontal")
            .field("api_key", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("retry_policy", &self.retry_policy)
            .field("environment", &self.environment)
            .field("debug", &self.debug)
            .finish_non_exhaustive()
    }
}

/// Cursor page shape used by [`Frontal::paginate`].
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Page<T> {
    /// Items returned for this page.
    pub data: Vec<T>,
    /// Cursor for the next page. An absent or empty cursor ends pagination.
    #[serde(default)]
    pub next_cursor: Option<String>,
}

/// One event received from an SSE endpoint.
#[cfg(feature = "stream")]
#[derive(Clone, Debug, PartialEq)]
pub struct ServerSentEvent {
    /// Event data field.
    pub data: String,
    /// Optional event type.
    pub event: String,
    /// Optional event identifier.
    pub id: String,
    /// Optional reconnection delay.
    pub retry: Option<Duration>,
}

fn serialize<B>(body: &B) -> Result<Vec<u8>>
where
    B: Serialize + ?Sized,
{
    serde_json::to_vec(body).map_err(FrontalError::Serialization)
}

async fn api_error(response: Response, fallback_request_id: String) -> FrontalError {
    let status = response.status();
    let request_id = response
        .headers()
        .get("x-request-id")
        .or_else(|| response.headers().get("request-id"))
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
        .or(Some(fallback_request_id));
    let body = response
        .bytes()
        .await
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok());
    classify_api_error(ApiError::new(status, request_id, body))
}

async fn checked_response(response: Response, request_id: String) -> Result<Response> {
    if response.status().is_success() {
        Ok(response)
    } else {
        Err(api_error(response, request_id).await)
    }
}

fn retry_after(response: &Response) -> Option<Duration> {
    response
        .headers()
        .get(header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| parse_retry_after(value, SystemTime::now()))
}

fn parse_retry_after(value: &str, now: SystemTime) -> Option<Duration> {
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let retry_at = httpdate::parse_http_date(value).ok()?;
    Some(retry_at.duration_since(now).unwrap_or_default())
}

fn is_transient_status(status: reqwest::StatusCode) -> bool {
    status == reqwest::StatusCode::REQUEST_TIMEOUT
        || status == reqwest::StatusCode::TOO_EARLY
        || status == reqwest::StatusCode::TOO_MANY_REQUESTS
        || status.is_server_error()
}

fn next_request_id() -> String {
    format!(
        "frontal-{}-{}",
        std::process::id(),
        REQUEST_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    )
}

fn ensure_trailing_slash(mut url: Url) -> Url {
    if !url.path().ends_with('/') {
        url.path_segments_mut()
            .expect("HTTP URLs support path segments")
            .push("");
    }
    url
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};

    use super::parse_retry_after;

    #[test]
    fn parses_retry_after_delta_seconds() {
        let now = UNIX_EPOCH + Duration::from_secs(1_000);
        assert_eq!(parse_retry_after("12", now), Some(Duration::from_secs(12)));
    }

    #[test]
    fn parses_retry_after_http_date() {
        let now = UNIX_EPOCH + Duration::from_secs(1_000);
        let retry_at = now + Duration::from_secs(12);
        let value = httpdate::fmt_http_date(retry_at);
        assert_eq!(
            parse_retry_after(&value, now),
            Some(Duration::from_secs(12))
        );
    }

    #[test]
    fn treats_past_retry_after_date_as_immediate() {
        let now = UNIX_EPOCH + Duration::from_secs(1_000);
        let retry_at = now - Duration::from_secs(12);
        let value = httpdate::fmt_http_date(retry_at);
        assert_eq!(parse_retry_after(&value, now), Some(Duration::ZERO));
    }
}
