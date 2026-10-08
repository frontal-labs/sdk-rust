//! Defaults shared by the SDK client.

use std::time::Duration;

/// Production Frontal API base URL.
pub const DEFAULT_API_BASE_URL: &str = "https://api.frontal.dev/v1/";

/// Default total timeout for an API request.
pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
