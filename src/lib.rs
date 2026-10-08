#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub mod api;
pub mod constants;
pub mod endpoints;
pub mod errors;
pub mod global;
pub mod models;
pub mod resources;

#[cfg(feature = "stream")]
pub use api::ServerSentEvent;
pub use api::{Client, ClientBuilder, Frontal, Page, RetryPolicy};
pub use endpoints::{encode_path_segment, EndpointPath};
pub use errors::{ApiError, Error, ErrorCategory, FrontalError};
pub use global::Result;

impl Frontal {
    #[cfg(feature = "agents")]
    /// Access the Agents service using this client's shared transport.
    #[must_use]
    pub fn agents(&self) -> resources::agents::Agents {
        resources::ServiceClient::new(self.clone(), "agents")
    }

    #[cfg(feature = "ai")]
    /// Access the AI service using this client's shared transport.
    #[must_use]
    pub fn ai(&self) -> resources::ai::Ai {
        resources::ServiceClient::new(self.clone(), "ai")
    }

    #[cfg(feature = "audit")]
    /// Access the Audit service using this client's shared transport.
    #[must_use]
    pub fn audit(&self) -> resources::audit::Audit {
        resources::ServiceClient::new(self.clone(), "audit")
    }

    #[cfg(feature = "auth")]
    /// Access the Auth service using this client's shared transport.
    #[must_use]
    pub fn auth(&self) -> resources::auth::Auth {
        resources::ServiceClient::new(self.clone(), "auth")
    }

    #[cfg(feature = "billing")]
    /// Access the Billing service using this client's shared transport.
    #[must_use]
    pub fn billing(&self) -> resources::billing::Billing {
        resources::ServiceClient::new(self.clone(), "billing")
    }

    #[cfg(feature = "blob")]
    /// Access the Blob service using this client's shared transport.
    #[must_use]
    pub fn blob(&self) -> resources::blob::Blob {
        resources::ServiceClient::new(self.clone(), "blob")
    }

    #[cfg(feature = "connectors")]
    /// Access the Connectors service using this client's shared transport.
    #[must_use]
    pub fn connectors(&self) -> resources::connectors::Connectors {
        resources::ServiceClient::new(self.clone(), "connectors")
    }

    #[cfg(feature = "data")]
    /// Access the Data service using this client's shared transport.
    #[must_use]
    pub fn data(&self) -> resources::data::Data {
        resources::ServiceClient::new(self.clone(), "data")
    }

    #[cfg(feature = "governance")]
    /// Access the Governance service using this client's shared transport.
    #[must_use]
    pub fn governance(&self) -> resources::governance::Governance {
        resources::ServiceClient::new(self.clone(), "governance")
    }

    #[cfg(feature = "lineage")]
    /// Access the Lineage service using this client's shared transport.
    #[must_use]
    pub fn lineage(&self) -> resources::lineage::Lineage {
        resources::ServiceClient::new(self.clone(), "lineage")
    }

    #[cfg(feature = "observability")]
    /// Access the Observability service using this client's shared transport.
    #[must_use]
    pub fn observability(&self) -> resources::observability::Observability {
        resources::ServiceClient::new(self.clone(), "observability")
    }

    #[cfg(feature = "ontology")]
    /// Access the Ontology service using this client's shared transport.
    #[must_use]
    pub fn ontology(&self) -> resources::ontology::Ontology {
        resources::ServiceClient::new(self.clone(), "ontology")
    }

    #[cfg(feature = "pipelines")]
    /// Access the Pipelines service using this client's shared transport.
    #[must_use]
    pub fn pipelines(&self) -> resources::pipelines::Pipelines {
        resources::ServiceClient::new(self.clone(), "pipelines")
    }

    #[cfg(feature = "sandbox")]
    /// Access the Sandbox service using this client's shared transport.
    #[must_use]
    pub fn sandbox(&self) -> resources::sandbox::Sandbox {
        resources::ServiceClient::new(self.clone(), "sandbox")
    }

    #[cfg(feature = "schedules")]
    /// Access the Schedules service using this client's shared transport.
    #[must_use]
    pub fn schedules(&self) -> resources::schedules::Schedules {
        resources::ServiceClient::new(self.clone(), "schedules")
    }

    #[cfg(feature = "webhooks")]
    /// Access the Webhooks service using this client's shared transport.
    #[must_use]
    pub fn webhooks(&self) -> resources::webhooks::Webhooks {
        resources::ServiceClient::new(self.clone(), "webhooks")
    }

    #[cfg(feature = "workflows")]
    /// Access the Workflows service using this client's shared transport.
    #[must_use]
    pub fn workflows(&self) -> resources::workflows::Workflows {
        resources::ServiceClient::new(self.clone(), "workflows")
    }
}

mod utils;
