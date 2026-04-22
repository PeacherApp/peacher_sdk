use ahash::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::stripe::payment_intent::Address;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AllowRedirects {
    Always,
    Never,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SetupFutureUsage {
    OffSession,
    OnSession,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AllowRedisplay {
    Always,
    Limited,
    Unspecified,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum RegulatedStatus {
    Regulated,
    Unregulated,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CardReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodCashapp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodKlarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<PaymentMethodKlarnaDob>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodKlarnaDob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodRadarOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodUsBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UsBankAccountHolderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UsBankAccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<PaymentMethodUsBankAccountNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<PaymentMethodUsBankAccountStatusDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountHolderType {
    Company,
    Individual,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountType {
    Checking,
    Savings,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodUsBankAccountNetworks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<String>,
    pub supported: Vec<UsBankAccountNetwork>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodUsBankAccountStatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<PaymentMethodUsBankAccountBlocked>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodUsBankAccountBlocked {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_code: Option<UsBankAccountBlockedNetworkCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<UsBankAccountBlockedReason>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountBlockedNetworkCode {
    R02,
    R03,
    R04,
    R05,
    R07,
    R08,
    R10,
    R11,
    R16,
    R20,
    R29,
    R31,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountBlockedReason {
    BankAccountClosed,
    BankAccountFrozen,
    BankAccountInvalidDetails,
    BankAccountRestricted,
    BankAccountUnusable,
    DebitNotAuthorized,
    TokenizedAccountNumberDeactivated,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ErrorSource {
    pub id: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<AllowRedisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<PayoutMethod>>,
    pub brand: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    pub exp_month: i32,
    pub exp_year: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    pub funding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    pub last4: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulated_status: Option<RegulatedStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<ErrorSourceWallet>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PayoutMethod {
    Instant,
    Standard,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ErrorSourceWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub wallet_type: ErrorSourceWalletType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ErrorSourceWalletType {
    ApplePay,
    Link,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ManagedPayments {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay_handle_redirect: Option<NextActionAlipayHandleRedirect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_display_details: Option<NextActionBoletoDisplayDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_await_notification: Option<NextActionCardAwaitNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_handle_redirect_or_display_qr_code: Option<NextActionCashappQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_bank_transfer_instructions: Option<NextActionDisplayBankTransferInstructions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_display_details: Option<NextActionKonbiniDisplayDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco_display_details: Option<NextActionMultibancoDisplayDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_display_details: Option<NextActionOxxoDisplayDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_display_qr_code: Option<NextActionPaynowDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix_display_qr_code: Option<NextActionPixDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_display_qr_code: Option<NextActionPromptpayDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<NextActionRedirectToUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish_handle_redirect_or_display_qr_code: Option<NextActionSwishQrCode>,
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi_handle_redirect_or_display_qr_code: Option<NextActionUpiQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits: Option<NextActionVerifyWithMicrodeposits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_display_qr_code: Option<NextActionWechatPayDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_android_app: Option<NextActionWechatPayRedirectToAndroidApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_ios_app: Option<NextActionWechatPayRedirectToIosApp>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionAlipayHandleRedirect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionBoletoDisplayDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_voucher_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionCardAwaitNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_attempt_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_approval_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionCashappQrCode {
    pub hosted_instructions_url: String,
    pub mobile_auth_url: String,
    pub qr_code: QrCodeData,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct QrCodeData {
    pub expires_at: i64,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionDisplayBankTransferInstructions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_remaining: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<Vec<FinancialAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_instructions_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "type")]
    pub transfer_type: BankTransferType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum BankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<FinancialAddressAba>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<FinancialAddressIban>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<FinancialAddressSortCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<FinancialAddressSpei>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<FinancialAddressSwift>,
    #[serde(rename = "type")]
    pub address_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<FinancialAddressZengin>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FinancialAddressAba {
    pub account_holder_address: Address,
    pub account_holder_name: String,
    pub account_number: String,
    pub account_type: String,
    pub bank_address: Address,
    pub bank_name: String,
    pub routing_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FinancialAddressIban {
    pub account_holder_address: Address,
    pub account_holder_name: String,
    pub bank_address: Address,
    pub bic: String,
    pub country: String,
    pub iban: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FinancialAddressSortCode {
    pub account_holder_address: Address,
    pub account_holder_name: String,
    pub account_number: String,
    pub bank_address: Address,
    pub sort_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FinancialAddressSpei {
    pub account_holder_address: Address,
    pub account_holder_name: String,
    pub bank_address: Address,
    pub bank_code: String,
    pub bank_name: String,
    pub clabe: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FinancialAddressSwift {
    pub account_holder_address: Address,
    pub account_holder_name: String,
    pub account_number: String,
    pub account_type: String,
    pub bank_address: Address,
    pub bank_name: String,
    pub swift_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FinancialAddressZengin {
    pub account_holder_address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub bank_address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionKonbiniDisplayDetails {
    pub expires_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_voucher_url: Option<String>,
    pub stores: KonbiniStores,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KonbiniStores {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub familymart: Option<KonbiniStoreInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lawson: Option<KonbiniStoreInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ministop: Option<KonbiniStoreInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seicomart: Option<KonbiniStoreInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KonbiniStoreInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
    pub payment_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionMultibancoDisplayDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_voucher_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionOxxoDisplayDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_voucher_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionPaynowDisplayQrCode {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_instructions_url: Option<String>,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionPixDisplayQrCode {
    pub data: String,
    pub expires_at: i64,
    pub hosted_instructions_url: String,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionPromptpayDisplayQrCode {
    pub data: String,
    pub hosted_instructions_url: String,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionRedirectToUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionSwishQrCode {
    pub hosted_instructions_url: String,
    pub qr_code: SwishQrCodeContent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SwishQrCodeContent {
    pub data: String,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionUpiQrCode {
    pub hosted_instructions_url: String,
    pub qr_code: QrCodeData,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionVerifyWithMicrodeposits {
    pub arrival_date: i64,
    pub hosted_verification_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microdeposit_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionWechatPayDisplayQrCode {
    pub data: String,
    pub hosted_instructions_url: String,
    pub image_data_url: String,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionWechatPayRedirectToAndroidApp {
    pub app_id: String,
    pub nonce_str: String,
    pub package: String,
    pub partner_id: String,
    pub prepay_id: String,
    pub sign: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NextActionWechatPayRedirectToIosApp {
    pub native_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodConfigurationDetails {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}

/// Payment-method-specific configuration for this PaymentIntent.
///
/// Only the most commonly used payment methods are typed here; other payment method
/// hashes are preserved as raw JSON via [`serde_json::Value`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodOptionsCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodOptionsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<serde_json::Value>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<PaymentMethodOptionsCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_multicapture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_overcapture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<RequestThreeDSecure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_cvc_recollection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surcharge: Option<PaymentMethodOptionsCardSurcharge>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PaymentMethodOptionsCaptureMethod {
    Manual,
    ManualPreferred,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum RequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodOptionsCardSurcharge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<i64>,
    pub status: SurchargeStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SurchargeStatus {
    Available,
    Unavailable,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentMethodOptionsLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<PaymentMethodOptionsCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PresentmentDetails {
    pub presentment_amount: i64,
    pub presentment_currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Processing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<ProcessingCard>,
    #[serde(rename = "type")]
    pub processing_type: ProcessingType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ProcessingType {
    Card,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ProcessingCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notification: Option<ProcessingCardCustomerNotification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ProcessingCardCustomerNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_requested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completes_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Shipping {
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    pub destination: String,
}
