//! Request and response types for the Frontal API.
//!
//! Keep wire models grouped by service and derive their fields from the
//! committed OpenAPI snapshots. No endpoint models have been implemented yet.

pub mod agents;
pub mod ai;
pub mod audit;
pub mod auth;
pub mod billing;
pub mod blob;
pub mod connectors;
pub mod data;
pub mod governance;
pub mod lineage;
pub mod observability;
pub mod ontology;
pub mod pipelines;
pub mod sandbox;
pub mod schedules;
pub mod webhooks;
pub mod workflows;
