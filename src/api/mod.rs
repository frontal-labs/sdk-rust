//! Shared HTTP client, configuration, and error types for the Frontal SDK.

mod client;

pub use crate::{errors::ApiError, errors::Error, global::Result};
pub use client::{Client, ClientBuilder};
