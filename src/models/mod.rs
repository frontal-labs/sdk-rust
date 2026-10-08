//! Request and response types for the Frontal API.
//!
//! Keep wire models grouped by service and derive their fields from the
//! committed OpenAPI snapshots and frozen TypeScript reference contracts.

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
