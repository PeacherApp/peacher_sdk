//! Types for the Stripe [Checkout Sessions](https://docs.stripe.com/api/checkout/sessions) API.
//!
//! The [`CheckoutSession`] struct mirrors the object Stripe returns on
//! every endpoint; the endpoint-specific request bodies live in their own
//! submodules (see [`CreateCheckoutSession`], [`RetrieveCheckoutSession`], etc.).

mod meta;
pub use meta::*;

mod line_item;
pub use line_item::*;

mod create;
pub use create::*;

mod retrieve;
pub use retrieve::*;

mod update;
pub use update::*;

mod list;
pub use list::*;

mod list_line_items;
pub use list_line_items::*;

mod expire;
pub use expire::*;

use ahash::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::stripe::{Address, PaymentMethodConfigurationDetails, Shipping};

/// A Checkout Session object returned by the Stripe API.
///
/// See: <https://docs.stripe.com/api/checkout/sessions/object>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CheckoutSession {
    pub id: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<AfterExpiration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subtotal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_total: Option<i64>,
    pub automatic_tax: AutomaticTax,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CheckoutBillingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<Consent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<ConsentCollection>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_conversion: Option<CurrencyConversion>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_fields: Vec<CustomField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<CustomText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CheckoutCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<CustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<InvoiceCreation>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    pub mode: CheckoutSessionMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<PaymentMethodCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration_details: Option<PaymentMethodConfigurationDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<serde_json::Value>,
    #[serde(default)]
    pub payment_method_types: Vec<String>,
    pub payment_status: CheckoutPaymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<PhoneNumberCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovered_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_on_completion: Option<RedirectOnCompletion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_payment_method_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<ShippingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<ShippingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<Shipping>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shipping_options: Vec<SessionShippingOption>,
    pub status: CheckoutSessionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<SubmitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<TaxIdCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_details: Option<TotalDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_mode: Option<UiMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AfterExpiration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<AfterExpirationRecovery>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AfterExpirationRecovery {
    pub allow_promotion_codes: bool,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AutomaticTax {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<TaxLiability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AutomaticTaxStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TaxLiability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "type")]
    pub liability_type: LiabilityType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Consent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<ConsentPromotions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<TermsOfService>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ConsentCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement: Option<PaymentMethodReuseAgreement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodReuseAgreement {
    pub position: ConsentPosition,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CurrencyConversion {
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub fx_rate: String,
    pub source_currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomField {
    pub key: String,
    pub label: CustomFieldLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CustomFieldDropdown>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<CustomFieldNumeric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CustomFieldText>,
    pub optional: bool,
    #[serde(rename = "type")]
    pub field_type: CustomFieldType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomFieldLabel {
    pub custom: String,
    #[serde(rename = "type")]
    pub label_type: CustomFieldLabelType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomFieldDropdown {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    pub options: Vec<CustomFieldDropdownOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomFieldDropdownOption {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomFieldNumeric {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomFieldText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<CustomTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<CustomTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<CustomTextMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomTextMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomerDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tax_ids: Vec<TaxId>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TaxId {
    #[serde(rename = "type")]
    pub tax_id_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InvoiceCreation {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<InvoiceData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InvoiceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<InvoiceDataCustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<TaxLiability>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<InvoiceRenderingOptions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InvoiceDataCustomField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InvoiceRenderingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<AmountTaxDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PhoneNumberCollection {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ShippingAddressCollection {
    pub allowed_countries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ShippingCost {
    pub amount_subtotal: i64,
    pub amount_tax: i64,
    pub amount_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taxes: Vec<ShippingCostTax>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ShippingCostTax {
    pub amount: i64,
    pub rate: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SessionShippingOption {
    pub shipping_amount: i64,
    pub shipping_rate: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TaxIdCollection {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<TaxIdRequired>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TotalDetails {
    pub amount_discount: i64,
    pub amount_shipping: i64,
    pub amount_tax: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<TotalDetailsBreakdown>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TotalDetailsBreakdown {
    pub discounts: Vec<BreakdownDiscount>,
    pub taxes: Vec<BreakdownTax>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BreakdownDiscount {
    pub amount: i64,
    pub discount: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BreakdownTax {
    pub amount: i64,
    pub rate: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
}
