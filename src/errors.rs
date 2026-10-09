//! Structured errors returned while configuring or using the Frontal SDK.

use reqwest::StatusCode;
use serde_json::Value;

/// Broad category for a [`FrontalError`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorCategory {
    /// Invalid local client configuration.
    Configuration,
    /// Authentication or authorization was rejected.
    Authentication,
    /// The request exceeded a rate or usage limit.
    RateLimit,
    /// The API rejected the request as invalid.
    Validation,
    /// The API returned a server error.
    Server,
    /// The request failed before receiving an API response.
    Network,
    /// A request or response could not be encoded or decoded.
    Serialization,
    /// An API response did not match an expected category.
    Api,
}

/// Errors returned while configuring the client or making an API request.
#[derive(Debug, thiserror::Error)]
pub enum FrontalError {
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

    /// An SSE response could not be parsed.
    #[error("could not parse server-sent event: {0}")]
    Stream(String),

    /// Cursor pagination returned a repeated cursor and could not make progress.
    #[error("pagination failed: {0}")]
    Pagination(String),

    /// The API rejected authentication or authorization.
    #[error("authentication failed: {0}")]
    Auth(ApiError),

    /// The API rate limited the request.
    #[error("API rate limit exceeded: {0}")]
    RateLimit(ApiError),

    /// The API rejected the request as invalid.
    #[error("API validation failed: {0}")]
    Validation(ApiError),

    /// The API returned a server error.
    #[error("API server error: {0}")]
    Server(ApiError),

    /// The API returned another non-success status code.
    #[error("API returned HTTP {}: {}", .0.status, .0.message)]
    Api(#[from] ApiError),
}

impl FrontalError {
    /// Return the broad error category.
    #[must_use]
    pub const fn category(&self) -> ErrorCategory {
        match self {
            Self::Configuration(_) | Self::InvalidPath(_) => ErrorCategory::Configuration,
            Self::Transport(_) => ErrorCategory::Network,
            Self::Serialization(_) | Self::Decode(_) => ErrorCategory::Serialization,
            Self::Stream(_) => ErrorCategory::Network,
            Self::Pagination(_) => ErrorCategory::Api,
            Self::Auth(_) => ErrorCategory::Authentication,
            Self::RateLimit(_) => ErrorCategory::RateLimit,
            Self::Validation(_) => ErrorCategory::Validation,
            Self::Server(_) => ErrorCategory::Server,
            Self::Api(_) => ErrorCategory::Api,
        }
    }

    /// Machine-readable API error code, when the server provided one.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.api_error().and_then(ApiError::code)
    }

    /// Server request identifier, when available.
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        self.api_error().and_then(ApiError::request_id)
    }

    /// HTTP status code, when the server returned a response.
    #[must_use]
    pub fn status_code(&self) -> Option<u16> {
        self.api_error().map(|error| error.status.as_u16())
    }

    /// Whether the operation may succeed if retried.
    #[must_use]
    pub fn retryable(&self) -> bool {
        match self {
            Self::Transport(error) => error.is_timeout() || error.is_connect(),
            Self::RateLimit(_) | Self::Server(_) => true,
            Self::Api(error) => error.retryable(),
            Self::Configuration(_)
            | Self::InvalidPath(_)
            | Self::Serialization(_)
            | Self::Decode(_)
            | Self::Stream(_)
            | Self::Pagination(_)
            | Self::Auth(_)
            | Self::Validation(_) => false,
        }
    }

    /// Parsed JSON error body, when one was returned.
    #[must_use]
    pub fn body(&self) -> Option<&Value> {
        self.api_error().and_then(ApiError::body)
    }

    fn api_error(&self) -> Option<&ApiError> {
        match self {
            Self::Auth(error)
            | Self::RateLimit(error)
            | Self::Validation(error)
            | Self::Server(error)
            | Self::Api(error) => Some(error),
            Self::Configuration(_)
            | Self::InvalidPath(_)
            | Self::Transport(_)
            | Self::Serialization(_)
            | Self::Decode(_) => None,
            Self::Stream(_) | Self::Pagination(_) => None,
        }
    }
}

/// Structured error response returned by the API.
#[derive(Debug, thiserror::Error)]
#[error("{status}: {message}")]
pub struct ApiError {
    status: StatusCode,
    code: Option<String>,
    message: String,
    request_id: Option<String>,
    body: Option<Value>,
}

impl ApiError {
    pub(crate) fn new(status: StatusCode, request_id: Option<String>, body: Option<Value>) -> Self {
        let code = body
            .as_ref()
            .and_then(|value| value.get("code").or_else(|| value.get("error_code")))
            .and_then(Value::as_str)
            .map(str::to_owned);
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
            code,
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

    /// Numeric HTTP status code returned by the API.
    #[must_use]
    pub const fn status_code(&self) -> u16 {
        self.status.as_u16()
    }

    /// Machine-readable error code from the response body, when present.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Human-readable message from the API.
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

    /// Whether the server status is normally safe to retry.
    #[must_use]
    pub fn retryable(&self) -> bool {
        self.status == StatusCode::REQUEST_TIMEOUT
            || self.status == StatusCode::TOO_EARLY
            || self.status == StatusCode::TOO_MANY_REQUESTS
            || self.status.is_server_error()
    }
}

pub(crate) fn classify_api_error(error: ApiError) -> FrontalError {
    match error.status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => FrontalError::Auth(error),
        StatusCode::UNPROCESSABLE_ENTITY => FrontalError::Validation(error),
        StatusCode::TOO_MANY_REQUESTS => FrontalError::RateLimit(error),
        status if status.is_server_error() => FrontalError::Server(error),
        _ => FrontalError::Api(error),
    }
}
