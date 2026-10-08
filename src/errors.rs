//! Errors returned by the SDK client.

use reqwest::StatusCode;
use serde_json::Value;

/// Errors returned while configuring the client or making an API request.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The client configuration is invalid.
    #[error("invalid client configuration: {0}")]
    Configuration(String),

    /// The endpoint path cannot be safely joined to the configured API URL.
    #[error("invalid endpoint path: {0}")]
    InvalidPath(String),

    /// The HTTP request failed before a valid response was received.
    #[error("HTTP request failed: {0}")]
    Transport(#[from] reqwest::Error),

    /// The request body could not be serialized as JSON.
    #[error("could not serialize request body: {0}")]
    Serialization(#[source] serde_json::Error),

    /// The response body could not be decoded as JSON or the requested type.
    #[error("could not decode API response: {0}")]
    Decode(#[source] serde_json::Error),

    /// The API returned a non-success status code.
    #[error("API returned HTTP {}: {}", .0.status, .0.message)]
    Api(#[from] ApiError),
}

/// Structured error response returned by the API.
#[derive(Debug, thiserror::Error)]
#[error("{status}: {message}")]
pub struct ApiError {
    status: StatusCode,
    message: String,
    request_id: Option<String>,
    body: Option<Value>,
}

impl ApiError {
    pub(crate) fn new(status: StatusCode, request_id: Option<String>, body: Option<Value>) -> Self {
        let message = body
            .as_ref()
            .and_then(|value| {
                value
                    .get("message")
                    .or_else(|| value.get("error"))
                    .and_then(Value::as_str)
            })
            .map(str::to_owned)
            .unwrap_or_else(|| {
                status
                    .canonical_reason()
                    .unwrap_or("request failed")
                    .to_owned()
            });

        Self {
            status,
            message,
            request_id,
            body,
        }
    }

    /// HTTP status code returned by the API.
    #[must_use]
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    /// Human-readable message from the API, if present.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Server request identifier, when the response provides one.
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// Parsed JSON error body, when the server returned JSON.
    #[must_use]
    pub fn body(&self) -> Option<&Value> {
        self.body.as_ref()
    }
}
