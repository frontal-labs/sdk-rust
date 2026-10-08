//! Service scoped access for the `auth` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `auth` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct AuthService;

/// Typed access to the `auth` service.
pub type Auth = ServiceClient<AuthService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "auth";

impl ServiceClient<AuthService> {
    /// List users through the admin API.
    pub async fn list_admin_users<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("auth/admin/users", query).await
    }

    /// Fetch an admin-managed user.
    pub async fn get_admin_user<T>(&self, user_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!("auth/admin/users/{}", encode_path_segment(user_id)))
            .await
    }

    /// Create an admin-managed user.
    pub async fn create_admin_user<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/admin/users", request).await
    }

    /// Update an admin-managed user.
    pub async fn update_admin_user<B, T>(&self, user_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.put(
            format!("auth/admin/users/{}", encode_path_segment(user_id)),
            request,
        )
        .await
    }

    /// Delete an admin-managed user with the API's delete options.
    pub async fn delete_admin_user<B, T>(&self, user_id: &str, options: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.delete_with_body(
            format!("auth/admin/users/{}", encode_path_segment(user_id)),
            options,
        )
        .await
    }

    /// Invite a user by email.
    pub async fn invite_user<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/invite", request).await
    }

    /// Generate an admin invitation, signup, or recovery link.
    pub async fn generate_admin_link<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/admin/generate_link", request).await
    }

    /// Sign out an admin-selected user session.
    pub async fn admin_sign_out<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/admin/logout", request).await
    }

    /// List MFA factors for an admin-managed user.
    pub async fn list_admin_user_factors<T>(&self, user_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "auth/admin/users/{}/factors",
            encode_path_segment(user_id)
        ))
        .await
    }

    /// Delete an MFA factor from an admin-managed user.
    pub async fn delete_admin_user_factor<T>(&self, user_id: &str, factor_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.delete(format!(
            "auth/admin/users/{}/factors/{}",
            encode_path_segment(user_id),
            encode_path_segment(factor_id)
        ))
        .await
    }

    /// Enroll an MFA factor for the current user.
    pub async fn enroll_factor<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/factors", request).await
    }

    /// List MFA factors for the current user.
    pub async fn list_factors<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/factors").await
    }

    /// Start a challenge for an MFA factor.
    pub async fn challenge_factor<B, T>(&self, factor_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!("auth/factors/{}/challenge", encode_path_segment(factor_id)),
            request,
        )
        .await
    }

    /// Verify an MFA factor challenge.
    pub async fn verify_factor<B, T>(&self, factor_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!("auth/factors/{}/verify", encode_path_segment(factor_id)),
            request,
        )
        .await
    }

    /// Remove an MFA factor from the current user.
    pub async fn delete_factor(&self, factor_id: &str) -> Result<()> {
        self.delete_empty(format!("auth/factors/{}", encode_path_segment(factor_id)))
            .await
    }

    /// Fetch the current user's profile.
    pub async fn profile<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/account/profile").await
    }

    /// Update the current user's profile.
    pub async fn update_profile<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.put("auth/account/profile", request).await
    }

    /// Delete the current user's profile.
    pub async fn delete_profile(&self) -> Result<()> {
        self.delete_empty("auth/account/profile").await
    }

    /// Update the current user's password.
    pub async fn update_password<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/account/password", request).await
    }

    /// Fetch the authenticated account audit log.
    pub async fn account_audit_log<T>(&self, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query("auth/account/audit-log", query).await
    }

    /// List API keys for the authenticated account.
    pub async fn list_api_keys<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/account/security/api-keys").await
    }

    /// Create an API key for the authenticated account.
    pub async fn create_api_key<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/account/security/api-keys", request).await
    }

    /// Fetch an API key by identifier.
    pub async fn get_api_key<T>(&self, key_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "auth/account/security/api-keys/{}",
            encode_path_segment(key_id)
        ))
        .await
    }

    /// Update an API key.
    pub async fn update_api_key<B, T>(&self, key_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.put(
            format!(
                "auth/account/security/api-keys/{}",
                encode_path_segment(key_id)
            ),
            request,
        )
        .await
    }

    /// Delete an API key.
    pub async fn delete_api_key(&self, key_id: &str) -> Result<()> {
        self.delete_empty(format!(
            "auth/account/security/api-keys/{}",
            encode_path_segment(key_id)
        ))
        .await
    }

    /// List devices registered to the authenticated account.
    pub async fn list_devices<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/account/security/devices").await
    }

    /// Register a device for the authenticated account.
    pub async fn register_device<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/account/security/devices", request).await
    }

    /// Fetch a registered device.
    pub async fn get_device<T>(&self, device_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "auth/account/security/devices/{}",
            encode_path_segment(device_id)
        ))
        .await
    }

    /// Delete a registered device.
    pub async fn delete_device(&self, device_id: &str) -> Result<()> {
        self.delete_empty(format!(
            "auth/account/security/devices/{}",
            encode_path_segment(device_id)
        ))
        .await
    }

    /// Mark a registered device as trusted.
    pub async fn trust_device<T>(&self, device_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "auth/account/security/devices/{}/trust",
                encode_path_segment(device_id)
            ),
            &serde_json::json!({}),
        )
        .await
    }

    /// List sessions for the authenticated account.
    pub async fn list_sessions<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/account/sessions").await
    }

    /// Extend an account session.
    pub async fn extend_session<T>(&self, session_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "auth/account/sessions/{}/extend",
                encode_path_segment(session_id)
            ),
            &serde_json::json!({}),
        )
        .await
    }

    /// Revoke an account session.
    pub async fn revoke_session(&self, session_id: &str) -> Result<()> {
        self.delete_empty(format!(
            "auth/account/sessions/{}",
            encode_path_segment(session_id)
        ))
        .await
    }

    /// List MFA factors for the authenticated account.
    pub async fn list_account_factors<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/account/mfa").await
    }

    /// Enroll an MFA factor for the authenticated account.
    pub async fn enroll_account_factor<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/account/mfa", request).await
    }

    /// Fetch an account MFA factor.
    pub async fn get_account_factor<T>(&self, factor_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "auth/account/mfa/{}",
            encode_path_segment(factor_id)
        ))
        .await
    }

    /// Remove an account MFA factor.
    pub async fn delete_account_factor(&self, factor_id: &str) -> Result<()> {
        self.delete_empty(format!(
            "auth/account/mfa/{}",
            encode_path_segment(factor_id)
        ))
        .await
    }

    /// Verify an account MFA factor.
    pub async fn verify_account_factor<B, T>(&self, factor_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!("auth/account/mfa/{}/verify", encode_path_segment(factor_id)),
            request,
        )
        .await
    }

    /// Start an account MFA challenge.
    pub async fn challenge_account_factor<T>(&self, factor_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "auth/account/mfa/{}/challenge",
                encode_path_segment(factor_id)
            ),
            &serde_json::json!({}),
        )
        .await
    }

    /// Sign up a user.
    pub async fn sign_up<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/signup", request).await
    }

    /// Sign in with password credentials.
    pub async fn sign_in_with_password<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post_with_query("auth/token", &[("grant_type", "password")], request)
            .await
    }

    /// Sign in with an OAuth provider.
    pub async fn sign_in_with_oauth<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/authorize", request).await
    }

    /// Request a one-time password.
    pub async fn sign_in_with_otp<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/otp", request).await
    }

    /// Sign in with a single sign-on provider.
    pub async fn sign_in_with_sso<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/sso", request).await
    }

    /// Sign in with a third-party identity token.
    pub async fn sign_in_with_id_token<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post_with_query("auth/token", &[("grant_type", "id_token")], request)
            .await
    }

    /// Create an anonymous user.
    pub async fn sign_in_anonymously<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/signup", request).await
    }

    /// Verify a one-time password.
    pub async fn verify_otp<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/verify", request).await
    }

    /// Exchange an authorization code for a session.
    pub async fn exchange_code_for_session<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post_with_query("auth/token", &[("grant_type", "pkce")], request)
            .await
    }

    /// Fetch the current authentication session.
    pub async fn session<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/auth/session").await
    }

    /// Fetch the current authenticated user.
    pub async fn current_user<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/user").await
    }

    /// Fetch the current authenticated user with a caller-provided JWT.
    pub async fn current_user_with_jwt<T>(&self, jwt: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_bearer_auth("auth/user", jwt).await
    }

    /// Update the current authenticated user.
    pub async fn update_current_user<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.put("auth/user", request).await
    }

    /// Set the current session from access and refresh tokens.
    pub async fn set_session<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/auth/session", request).await
    }

    /// Refresh an authentication session.
    pub async fn refresh_session<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post_with_query("auth/token", &[("grant_type", "refresh_token")], request)
            .await
    }

    /// Sign out the current authenticated user.
    pub async fn sign_out<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/logout", request).await
    }

    /// Send a password recovery request.
    pub async fn recover_password<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/recover", request).await
    }

    /// Reauthenticate the current user.
    pub async fn reauthenticate<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post("auth/reauthenticate", &serde_json::json!({}))
            .await
    }

    /// Resend a verification email or one-time password.
    pub async fn resend_verification<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/resend", request).await
    }

    /// List identities linked to the current user.
    pub async fn list_identities<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get("auth/user/identities").await
    }

    /// Link an external identity to the current user.
    pub async fn link_identity<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("auth/user/identities", request).await
    }

    /// Unlink an identity from the current user.
    pub async fn unlink_identity(&self, identity_id: &str) -> Result<()> {
        self.delete_empty(format!(
            "auth/user/identities/{}",
            encode_path_segment(identity_id)
        ))
        .await
    }
}
