use ahash::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::{
    prelude::{BodyBuilder, Handler, Method},
    stripe::{
        Address, CaptureMethod, PaymentMethodType, SetupFutureUsage, TransferData,
        checkout_session::{
            AmountTaxDisplay, CheckoutBillingAddressCollection, CheckoutCustomerCreation,
            CheckoutSession, CheckoutSessionMode, ConsentPosition, CustomFieldLabelType,
            CustomFieldType, DeliveryUnit, LiabilityType, OriginContext, PaymentMethodCollection,
            RecurringInterval, RedirectOnCompletion, ShippingRateType, SubmitType, TaxBehavior,
            TaxIdRequired, UiMode,
        },
    },
};

/// Parameters accepted by `POST /v1/checkout/sessions`.
///
/// See: <https://docs.stripe.com/api/checkout/sessions/create>
///
/// Stripe expects `application/x-www-form-urlencoded` with bracket-notation nesting,
/// so the body is serialized via [`BodyBuilder::qs`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCheckoutSession {
    pub mode: CheckoutSessionMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_pricing: Option<CreateAdaptivePricing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<CreateAfterExpiration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CheckoutBillingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<CreateConsentCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateCustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<CreateCustomText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CheckoutCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CreateCustomerUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<CreateDiscount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_payment_method_types: Option<Vec<PaymentMethodType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<CreateInvoiceCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_payments: Option<CreateManagedPayments>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_collection: Option<CreateNameCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_items: Option<Vec<CreateOptionalItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_context: Option<OriginContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CreatePaymentIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<PaymentMethodCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<CreateSessionPaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<PaymentMethodType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<CreatePermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<CreatePhoneNumberCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_on_completion: Option<RedirectOnCompletion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_payment_method_options: Option<CreateSavedPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent_data: Option<CreateSetupIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreateShippingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<CreateShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<SubmitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreateSubscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreateTaxIdCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_mode: Option<UiMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_options: Option<CreateWalletOptions>,
}

impl CreateCheckoutSession {
    /// Minimal constructor that only sets the single required parameter.
    pub fn new(mode: CheckoutSessionMode) -> Self {
        Self {
            mode,
            adaptive_pricing: None,
            after_expiration: None,
            allow_promotion_codes: None,
            automatic_tax: None,
            billing_address_collection: None,
            cancel_url: None,
            client_reference_id: None,
            consent_collection: None,
            currency: None,
            custom_fields: None,
            custom_text: None,
            customer: None,
            customer_account: None,
            customer_creation: None,
            customer_email: None,
            customer_update: None,
            discounts: None,
            excluded_payment_method_types: None,
            expires_at: None,
            integration_identifier: None,
            invoice_creation: None,
            line_items: None,
            locale: None,
            managed_payments: None,
            metadata: HashMap::default(),
            name_collection: None,
            optional_items: None,
            origin_context: None,
            payment_intent_data: None,
            payment_method_collection: None,
            payment_method_configuration: None,
            payment_method_data: None,
            payment_method_options: None,
            payment_method_types: None,
            permissions: None,
            phone_number_collection: None,
            redirect_on_completion: None,
            return_url: None,
            saved_payment_method_options: None,
            setup_intent_data: None,
            shipping_address_collection: None,
            shipping_options: None,
            submit_type: None,
            subscription_data: None,
            success_url: None,
            tax_id_collection: None,
            ui_mode: None,
            wallet_options: None,
        }
    }
}

impl Handler for CreateCheckoutSession {
    type ResponseBody = CheckoutSession;
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/v1/checkout/sessions".into()
    }
    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.qs(self)
    }
}

// ---------------------------------------------------------------------------
// Request sub-types
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAdaptivePricing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAfterExpiration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<CreateAfterExpirationRecovery>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAfterExpirationRecovery {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAutomaticTax {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateLiability>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateLiability {
    #[serde(rename = "type")]
    pub liability_type: LiabilityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateConsentCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement: Option<CreatePaymentMethodReuseAgreement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreateConsentPromotion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<CreateConsentTermsOfService>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodReuseAgreement {
    pub position: ConsentPosition,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateConsentPromotion {
    Auto,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateConsentTermsOfService {
    None,
    Required,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomField {
    pub key: String,
    pub label: CreateCustomFieldLabel,
    #[serde(rename = "type")]
    pub field_type: CustomFieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CreateCustomFieldDropdown>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<CreateCustomFieldNumeric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CreateCustomFieldText>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomFieldLabel {
    pub custom: String,
    #[serde(rename = "type")]
    pub label_type: CustomFieldLabelType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomFieldDropdown {
    pub options: Vec<CreateCustomFieldDropdownOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomFieldDropdownOption {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomFieldNumeric {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomFieldText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<CreateCustomTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreateCustomTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<CreateCustomTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<CreateCustomTextMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomTextMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCustomerUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateCustomerUpdateBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CreateCustomerUpdateBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCustomerUpdateBehavior>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateCustomerUpdateBehavior {
    Auto,
    Never,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateDiscount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateInvoiceCreation {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreateInvoiceData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateInvoiceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateInvoiceDataCustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateLiability>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateInvoiceRenderingOptions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateInvoiceDataCustomField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateInvoiceRenderingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<AmountTaxDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

// ---------------------------------------------------------------------------
// Line items
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateAdjustableQuantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tax_rates: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreatePriceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAdjustableQuantity {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePriceData {
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreateProductData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreatePriceDataRecurring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateProductData {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<CreateProductTaxDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateProductTaxDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePriceDataRecurring {
    pub interval: RecurringInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i64>,
}

// ---------------------------------------------------------------------------
// Optional items
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateOptionalItem {
    pub price: String,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateAdjustableQuantity>,
}

// ---------------------------------------------------------------------------
// Payment-intent & subscription & setup-intent data
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentIntentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateShippingInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateShippingInfo {
    pub address: Address,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSubscriptionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<CreateBillingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateSubscriptionInvoiceSettings>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<CreatePendingInvoiceItemInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateSubscriptionProration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateSubscriptionTransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<CreateTrialSettings>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBillingMode {
    #[serde(rename = "type")]
    pub billing_mode_type: CreateBillingModeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible: Option<CreateBillingModeFlexible>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateBillingModeType {
    Classic,
    Flexible,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBillingModeFlexible {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_discounts: Option<CreateProrationDiscounts>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateProrationDiscounts {
    Included,
    Itemized,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSubscriptionInvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateLiability>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePendingInvoiceItemInterval {
    pub interval: RecurringInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateSubscriptionProration {
    CreateProrations,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSubscriptionTransferData {
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateTrialSettings {
    pub end_behavior: CreateTrialEndBehavior,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateTrialEndBehavior {
    pub missing_payment_method: CreateMissingPaymentMethod,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSetupIntentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
}

// ---------------------------------------------------------------------------
// Name / phone collection, managed payments, permissions
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateManagedPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateNameCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<CreateNameCollectionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<CreateNameCollectionConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateNameCollectionConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePhoneNumberCollection {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_shipping_details: Option<CreateUpdateShippingDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateUpdateShippingDetails {
    ClientOnly,
    ServerOnly,
}

// ---------------------------------------------------------------------------
// Saved payment method options / payment_method_data
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSavedPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay_filters: Option<Vec<CreateAllowRedisplayFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_remove: Option<CreatePaymentMethodToggle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save: Option<CreatePaymentMethodToggle>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateAllowRedisplayFilter {
    Always,
    Limited,
    Unspecified,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreatePaymentMethodToggle {
    Disabled,
    Enabled,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSessionPaymentMethodData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<CreateAllowRedisplay>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}

// ---------------------------------------------------------------------------
// Shipping collection + options
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateShippingAddressCollection {
    pub allowed_countries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateShippingOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateShippingRateData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateShippingRateData {
    pub display_name: String,
    #[serde(rename = "type")]
    pub shipping_type: ShippingRateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<CreateDeliveryEstimate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateFixedAmount>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateDeliveryEstimate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateDeliveryBound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateDeliveryBound>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateDeliveryBound {
    pub unit: DeliveryUnit,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateFixedAmount {
    pub amount: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<HashMap<String, CreateCurrencyOption>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCurrencyOption {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
}

// ---------------------------------------------------------------------------
// Tax-id collection, wallet options
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateTaxIdCollection {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<TaxIdRequired>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateWalletOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateWalletLink>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateWalletLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<CreateWalletDisplay>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateWalletDisplay {
    Auto,
    Never,
}
