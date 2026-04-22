use ahash::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::stripe::payment_intent::{
    Address, AllowRedirects, AmountDetailsLineItemPaymentMethodOptionsPaypalCategory, CaptureMethod,
    ConfirmationMethod, CreatePaymentMethodData, CreatePaymentMethodOptions, Hooks, PaymentDetails,
    PaymentMethodType, SetupFutureUsage, TransferData,
};

/// Parameters accepted by `POST /v1/payment_intents`.
///
/// See: <https://docs.stripe.com/api/payment_intents/create>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentIntentBody {
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<CreateAmountDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods: Option<CreateAutomaticPaymentMethods>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_method: Option<ConfirmationMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_on_requires_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_payment_method_types: Option<Vec<PaymentMethodType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<MandateData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<OffSession>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<CreatePaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreatePaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<PaymentMethodType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreateRadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<bool>,
}

impl CreatePaymentIntentBody {
    /// Minimal constructor for the two required parameters.
    pub fn new(amount: i64, currency: impl Into<String>) -> Self {
        Self {
            amount,
            currency: currency.into(),
            amount_details: None,
            application_fee_amount: None,
            automatic_payment_methods: None,
            capture_method: None,
            confirm: None,
            confirmation_method: None,
            confirmation_token: None,
            customer: None,
            customer_account: None,
            description: None,
            error_on_requires_action: None,
            excluded_payment_method_types: None,
            hooks: None,
            mandate: None,
            mandate_data: None,
            metadata: HashMap::default(),
            off_session: None,
            on_behalf_of: None,
            payment_details: None,
            payment_method: None,
            payment_method_configuration: None,
            payment_method_data: None,
            payment_method_options: None,
            payment_method_types: None,
            radar_options: None,
            receipt_email: None,
            return_url: None,
            setup_future_usage: None,
            shipping: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_data: None,
            transfer_group: None,
            use_stripe_sdk: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_arithmetic_validation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateAmountDetailsLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateAmountDetailsShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<CreateAmountDetailsTax>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsLineItem {
    pub product_name: String,
    pub quantity: i64,
    pub unit_cost: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateAmountDetailsLineItemPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<CreateAmountDetailsLineItemTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsLineItemPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateAmountDetailsLineItemPaymentMethodOptionsCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<CreateAmountDetailsLineItemPaymentMethodOptionsCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateAmountDetailsLineItemPaymentMethodOptionsKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreateAmountDetailsLineItemPaymentMethodOptionsPaypal>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsLineItemPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsLineItemPaymentMethodOptionsCardPresent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsLineItemPaymentMethodOptionsKlarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_reference: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsLineItemPaymentMethodOptionsPaypal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<AmountDetailsLineItemPaymentMethodOptionsPaypalCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sold_by: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsLineItemTax {
    pub total_tax_amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsShipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_postal_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAmountDetailsTax {
    pub total_tax_amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAutomaticPaymentMethods {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<AllowRedirects>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MandateData {
    pub customer_acceptance: MandateDataCustomerAcceptance,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MandateDataCustomerAcceptance {
    #[serde(rename = "type")]
    pub acceptance_type: MandateDataCustomerAcceptanceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<i64>,
    /// Stripe documents this as an empty object; sending an empty hash is sufficient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<MandateDataCustomerAcceptanceOffline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<MandateDataCustomerAcceptanceOnline>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum MandateDataCustomerAcceptanceType {
    Online,
    Offline,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MandateDataCustomerAcceptanceOffline {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MandateDataCustomerAcceptanceOnline {
    pub ip_address: String,
    pub user_agent: String,
}

/// `off_session` accepts either a boolean or a reason string; this mirrors that union.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum OffSession {
    Bool(bool),
    Reason(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateShipping {
    pub address: Address,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateRadarOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

/// Shared `setup_future_usage` enum for sub-options, including the `none` override.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum OptionSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

/// `manual` capture override used across many payment method options.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ManualCaptureMethod {
    Manual,
}

/// `manual` / `manual_preferred` capture override for card-present flows.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CardPresentCaptureMethod {
    Manual,
    ManualPreferred,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum IfAvailableOrNever {
    IfAvailable,
    Never,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum VerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AmountType {
    Fixed,
    Maximum,
}

/// Empty placeholder for payment-method sub-hashes that currently carry no options.
///
/// Sending `{}` for these sub-hashes is equivalent to omitting them — Stripe just
/// uses the top-level `type` discriminator. Kept as a single shared struct to avoid
/// declaring 30+ identical empty types.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct EmptyOptions {}
