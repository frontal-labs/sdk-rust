//! Shared HTTP transport, configuration, retry, pagination, and error types.

mod client;

pub use crate::{errors::ApiError, errors::Error, errors::FrontalError, global::Result};
#[cfg(feature = "stream")]
pub use client::ServerSentEvent;
pub use client::{Client, ClientBuilder, Frontal, Page, RetryPolicy};
