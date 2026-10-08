//! Service scoped access for the `governance` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `governance` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct GovernanceService;

/// Typed access to the `governance` service.
pub type Governance = ServiceClient<GovernanceService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "governance";

impl ServiceClient<GovernanceService> {
    /// List governance policies.
    pub async fn list_policies<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("policies", query).await
    }

    /// Create a governance policy.
    pub async fn create_policy<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("policies", request).await
    }

    /// Fetch a governance policy.
    pub async fn get_policy<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("policies/{}", encode_path_segment(id)))
            .await
    }

    /// Update a governance policy.
    pub async fn update_policy<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.put(format!("policies/{}", encode_path_segment(id)), request)
            .await
    }

    /// Delete a governance policy.
    pub async fn delete_policy(&self, id: &str) -> Result<()> {
        self.delete_empty(format!("policies/{}", encode_path_segment(id)))
            .await
    }

    /// List versions of a governance policy.
    pub async fn policy_versions<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("policies/{}/versions", encode_path_segment(id)))
            .await
    }

    /// List policy templates.
    pub async fn policy_templates<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("policies/templates", query).await
    }

    /// Create a policy from a template.
    pub async fn create_policy_from_template<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("policies/from-template", request).await
    }

    /// Validate a policy request.
    pub async fn validate_policy<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("policies/validate", request).await
    }

    /// List compliance frameworks.
    pub async fn frameworks<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("compliance/frameworks").await
    }

    /// Start a compliance assessment.
    pub async fn run_assessment<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("compliance/assessments", request).await
    }

    /// List compliance assessments.
    pub async fn list_assessments<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("compliance/assessments", query).await
    }

    /// Fetch a compliance assessment.
    pub async fn get_assessment<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "compliance/assessments/{}",
            encode_path_segment(id)
        ))
        .await
    }

    /// List compliance violations.
    pub async fn list_violations<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("compliance/violations", query).await
    }

    /// Resolve a compliance violation.
    pub async fn resolve_violation<B, T>(&self, id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!("compliance/violations/{}/resolve", encode_path_segment(id)),
            request,
        )
        .await
    }

    /// Fetch a compliance score.
    pub async fn compliance_score<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("compliance/score", query).await
    }

    /// List governance roles.
    pub async fn list_roles<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("roles", query).await
    }

    /// Fetch a governance role.
    pub async fn get_role<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("roles/{}", encode_path_segment(id))).await
    }

    /// Create a governance role.
    pub async fn create_role<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("roles", request).await
    }

    /// Delete a governance role.
    pub async fn delete_role(&self, id: &str) -> Result<()> {
        self.delete_empty(format!("roles/{}", encode_path_segment(id)))
            .await
    }

    /// List governance permissions.
    pub async fn list_permissions<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("permissions", query).await
    }

    /// Fetch a governance permission.
    pub async fn get_permission<T>(&self, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("permissions/{}", encode_path_segment(id)))
            .await
    }

    /// Create a governance permission.
    pub async fn create_permission<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("permissions", request).await
    }

    /// Check whether a subject has access to a resource.
    pub async fn check_access<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("access/check", request).await
    }
}
