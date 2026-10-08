//! Service scoped access for the `sandbox` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::global::Result;

/// Type marker for the `sandbox` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct SandboxService;

/// Typed access to the `sandbox` service.
pub type Sandbox = ServiceClient<SandboxService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "sandbox";

impl ServiceClient<SandboxService> {
    /// List languages available to sandbox jobs.
    pub async fn languages<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("sandbox/languages").await
    }

    /// Compile and run code with one standard input value.
    pub async fn self_test<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("sandbox/self-test", request).await
    }

    /// Submit code to the sandbox judge.
    pub async fn submit<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("sandbox/submit", request).await
    }
}
