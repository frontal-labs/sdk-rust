//! Service scoped access for the `billing` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `billing` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct BillingService;

/// Typed access to the `billing` service.
pub type Billing = ServiceClient<BillingService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "billing";

impl ServiceClient<BillingService> {
    /// Fetch a billing customer's entitlements.
    pub async fn customer_entitlements<T>(&self, customer_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/customers/{}/entitlements",
            encode_path_segment(customer_id)
        ))
        .await
    }

    /// Fetch a billing customer's usage summary.
    pub async fn customer_usage<T>(&self, customer_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/customers/{}/usage",
            encode_path_segment(customer_id)
        ))
        .await
    }

    /// List a billing customer's wallets.
    pub async fn customer_wallets<T>(&self, customer_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/customers/{}/wallets",
            encode_path_segment(customer_id)
        ))
        .await
    }

    /// Fetch a billing customer's invoice summary.
    pub async fn customer_invoice_summary<T>(&self, customer_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/customers/{}/invoices/summary",
            encode_path_segment(customer_id)
        ))
        .await
    }

    /// Create a customer portal session for an external customer identifier.
    pub async fn customer_portal_session<T>(&self, external_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/customers/portal/{}",
            encode_path_segment(external_id)
        ))
        .await
    }

    /// Clone a billing plan.
    pub async fn clone_plan<B, T>(&self, plan_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!("billing/plans/{}/clone", encode_path_segment(plan_id)),
            request,
        )
        .await
    }

    /// Fetch a billing plan's entitlements.
    pub async fn plan_entitlements<T>(&self, plan_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/plans/{}/entitlements",
            encode_path_segment(plan_id)
        ))
        .await
    }

    /// Activate a subscription.
    pub async fn activate_subscription<T>(&self, subscription_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "billing/subscriptions/{}/activate",
                encode_path_segment(subscription_id)
            ),
            &serde_json::json!({}),
        )
        .await
    }

    /// Cancel a subscription.
    pub async fn cancel_subscription<B, T>(&self, subscription_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "billing/subscriptions/{}/cancel",
                encode_path_segment(subscription_id)
            ),
            request,
        )
        .await
    }

    /// Pause a subscription.
    pub async fn pause_subscription<B, T>(&self, subscription_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "billing/subscriptions/{}/pause",
                encode_path_segment(subscription_id)
            ),
            request,
        )
        .await
    }

    /// Resume a paused subscription.
    pub async fn resume_subscription<B, T>(&self, subscription_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "billing/subscriptions/{}/resume",
                encode_path_segment(subscription_id)
            ),
            request,
        )
        .await
    }

    /// Fetch a subscription's entitlements.
    pub async fn subscription_entitlements<T>(&self, subscription_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/subscriptions/{}/entitlements",
            encode_path_segment(subscription_id)
        ))
        .await
    }

    /// Finalize an invoice.
    pub async fn finalize_invoice<T>(&self, invoice_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "billing/invoices/{}/finalize",
                encode_path_segment(invoice_id)
            ),
            &serde_json::json!({}),
        )
        .await
    }

    /// Void an invoice.
    pub async fn void_invoice<T>(&self, invoice_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("billing/invoices/{}/void", encode_path_segment(invoice_id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// Preview an invoice from a request payload.
    pub async fn preview_invoice<B, T>(&self, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post("billing/invoices/preview", request).await
    }

    /// Download an invoice PDF.
    pub async fn invoice_pdf(&self, invoice_id: &str) -> Result<Vec<u8>> {
        self.get_bytes(format!(
            "billing/invoices/{}/pdf",
            encode_path_segment(invoice_id)
        ))
        .await
    }

    /// List transactions in a wallet.
    pub async fn wallet_transactions<T>(&self, wallet_id: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query(
            format!(
                "billing/wallets/{}/transactions",
                encode_path_segment(wallet_id)
            ),
            query,
        )
        .await
    }

    /// Add funds to a wallet.
    pub async fn top_up_wallet<B, T>(&self, wallet_id: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.post(
            format!("billing/wallets/{}/top-up", encode_path_segment(wallet_id)),
            request,
        )
        .await
    }

    /// Terminate a wallet.
    pub async fn terminate_wallet<T>(&self, wallet_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!(
                "billing/wallets/{}/terminate",
                encode_path_segment(wallet_id)
            ),
            &serde_json::json!({}),
        )
        .await
    }

    /// Fetch a wallet's real-time balance.
    pub async fn real_time_wallet_balance<T>(&self, wallet_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/wallets/{}/balance/real-time",
            encode_path_segment(wallet_id)
        ))
        .await
    }

    /// Disable a billing meter.
    pub async fn disable_meter<T>(&self, meter_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.post(
            format!("billing/meters/{}/disable", encode_path_segment(meter_id)),
            &serde_json::json!({}),
        )
        .await
    }

    /// Look up a billing price by lookup key.
    pub async fn lookup_price<T>(&self, lookup_key: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/prices/lookup/{}",
            encode_path_segment(lookup_key)
        ))
        .await
    }

    /// Fetch an add-on's entitlements.
    pub async fn addon_entitlements<T>(&self, addon_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(format!(
            "billing/addons/{}/entitlements",
            encode_path_segment(addon_id)
        ))
        .await
    }
}
