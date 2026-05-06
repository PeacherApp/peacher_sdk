use ahash::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::stripe::{
    AllowRedirects, AllowRedisplay, CardReadMethod, PaymentMethodCashapp, PaymentMethodKlarna,
    PaymentMethodLink, PaymentMethodRadarOptions, PaymentMethodSepaDebit,
    PaymentMethodUsBankAccount, RegulatedStatus,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethod {
    pub id: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<AllowRedisplay>,
    pub billing_details: PaymentMethodBillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PaymentMethodCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodCashapp>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodLink>,
    pub livemode: bool,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<PaymentMethodRadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodUsBankAccount>,
    #[serde(rename = "type")]
    pub payment_method_type: PaymentMethodType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AutomaticPaymentMethods {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<AllowRedirects>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CancellationReason {
    Abandoned,
    Automatic,
    Duplicate,
    Expired,
    FailedInvoice,
    Fraudulent,
    RequestedByCustomer,
    VoidInvoice,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ConfirmationMethod {
    Automatic,
    Manual,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PaymentIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PaymentMethodType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Blik,
    Boleto,
    Card,
    CardPresent,
    Cashapp,
    Crypto,
    Custom,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    MbWay,
    Mobilepay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    Oxxo,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    SepaDebit,
    Sofort,
    Sunbit,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
    Zip,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodBillingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCard {
    pub brand: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<PaymentMethodCardChecks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_brand: Option<String>,
    pub exp_month: i32,
    pub exp_year: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    pub funding: String,
    pub last4: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<PaymentMethodCardNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulated_status: Option<RegulatedStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure_usage: Option<PaymentMethodCardThreeDSecureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<PaymentMethodCardWallet>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCardChecks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCardNetworks {
    pub available: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCardThreeDSecureUsage {
    pub supported: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCardWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<PaymentMethodCardWalletMasterpass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub wallet_type: PaymentMethodCardWalletType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<PaymentMethodCardWalletVisaCheckout>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PaymentMethodCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCardWalletMasterpass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCardWalletVisaCheckout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCardPresent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub exp_month: i32,
    pub exp_year: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<PaymentMethodCardNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<PaymentMethodCardPresentOffline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_method: Option<CardReadMethod>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCardPresentOffline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub offline_type: Option<OfflineType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum OfflineType {
    Deferred,
}
