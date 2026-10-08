//! Service resource namespaces.
//!
//! Each module corresponds to a service in `contracts/sdk-endpoints.json`.
//! Implement service operations here when their behavior and wire shapes are
//! grounded in the shared API contracts.

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
