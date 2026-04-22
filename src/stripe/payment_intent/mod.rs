mod amount_details;
use ahash::HashMap;
pub use amount_details::*;

mod hooks;
pub use hooks::*;

mod meta;
pub use meta::*;

mod payment;
pub use payment::*;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// A PaymentIntent guides you through the process of collecting a payment from your customer.
///
/// See: <https://docs.stripe.com/api/payment_intents/object>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentIntent {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub amount_capturable: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<AmountDetails>,
    pub amount_received: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods: Option<AutomaticPaymentMethods>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<CancellationReason>,
    pub capture_method: CaptureMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub confirmation_method: ConfirmationMethod,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_payment_method_types: Option<Vec<PaymentMethodType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_error: Option<LastPaymentError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_charge: Option<String>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_payments: Option<ManagedPayments>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<NextAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration_details: Option<PaymentMethodConfigurationDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    pub payment_method_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_details: Option<PresentmentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<Processing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    pub status: PaymentIntentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LastPaymentError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_advice_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_decline_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,
    #[serde(rename = "type")]
    pub error_type: ErrorType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ErrorType {
    ApiError,
    CardError,
    IdempotencyError,
    InvalidRequestError,
}
