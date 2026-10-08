//! Frontal SDK for Rust.
//!
//! Async Rust client for the Frontal API.
//!
//! The SDK provides shared HTTP transport, bearer authentication, JSON handling,
//! and structured errors. Endpoint paths and wire types remain grounded in the
//! committed API contracts.

#![forbid(unsafe_code)]

pub mod api;
pub mod constants;
pub mod endpoints;
pub mod errors;
pub mod global;
pub mod models;
pub mod resources;

pub use api::{Client, ClientBuilder};
pub use endpoints::EndpointPath;
pub use errors::{ApiError, Error};
pub use global::Result;

mod utils;
