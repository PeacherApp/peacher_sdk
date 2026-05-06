use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::stripe::payment_intent::{
    AmountType, BankTransferType, CardPresentCaptureMethod, EmptyOptions, IfAvailableOrNever,
    ManualCaptureMethod, OptionSetupFutureUsage, RequestThreeDSecure, VerificationMethod,
};

/// Request-side `payment_method_options`. Mirror of the response [`PaymentMethodOptions`](crate::stripe::payment_intent::PaymentMethodOptions)
/// but with creation-only fields (e.g. `blik.code`, `card.cvc_token`, `card.three_d_secure`).
///
/// Only set the sub-hash for payment methods you want to configure.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreatePmoAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreatePmoAffirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreatePmoAfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreatePmoAlipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma: Option<CreatePmoAlma>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<CreatePmoAmazonPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreatePmoAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreatePmoBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreatePmoBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie: Option<CreatePmoBillie>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreatePmoBlik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePmoBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreatePmoCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<CreatePmoCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CreatePmoCashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto: Option<CreatePmoCrypto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreatePmoCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreatePmoEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreatePmoFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreatePmoGiropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreatePmoGrabpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreatePmoIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<CreatePmoKakaoPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreatePmoKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreatePmoKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<CreatePmoKrCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreatePmoLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<CreatePmoMbWay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<CreatePmoMobilepay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<CreatePmoMultibanco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<CreatePmoNaverPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<CreatePmoNzBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreatePmoOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreatePmoP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payco: Option<CreatePmoPayco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreatePmoPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreatePmoPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypay: Option<CreatePmoPaypay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<CreatePmoPayto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreatePmoPix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreatePmoPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CreatePmoRevolutPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<CreatePmoSamsungPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay: Option<CreatePmoSatispay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreatePmoSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreatePmoSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CreatePmoSwish>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<CreatePmoTwint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<CreatePmoUpi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreatePmoUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreatePmoWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<CreatePmoZip>,
}

// ---- ACSS Debit ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePmoAcssDebitMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoAcssDebitMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<AcssDebitPaymentSchedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<AcssDebitTransactionType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AcssDebitTransactionType {
    Business,
    Personal,
}

// ---- Shared helper macros-less structs ----

/// Just `capture_method`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CaptureOnly {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ManualCaptureMethod>,
}

/// Just `setup_future_usage`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetupOnly {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

/// `capture_method` + `setup_future_usage`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CaptureAndSetup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ManualCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- Affirm ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoAffirm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ManualCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- Afterpay / Clearpay ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoAfterpayClearpay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ManualCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- Alipay / Alma / AmazonPay / AuBecsDebit ----
pub type CreatePmoAlipay = SetupOnly;
pub type CreatePmoAlma = CaptureOnly;
pub type CreatePmoAmazonPay = CaptureAndSetup;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoAuBecsDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}

// ---- BacsDebit ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoBacsDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePmoBacsDebitMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoBacsDebitMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}

// ---- Bancontact ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoBancontact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<BancontactLanguage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum BancontactLanguage {
    De,
    En,
    Fr,
    Nl,
}

pub type CreatePmoBillie = CaptureOnly;

// ---- Blik ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoBlik {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- Boleto ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoBoleto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- Card ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ManualCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreatePmoCardInstallments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePmoCardMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization: Option<IfAvailableOrNever>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization: Option<IfAvailableOrNever>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_multicapture: Option<IfAvailableOrNever>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_overcapture: Option<IfAvailableOrNever>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<RequestThreeDSecure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_cvc_recollection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<CreatePmoCardThreeDSecure>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCardInstallments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CreatePmoCardInstallmentsPlan>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCardInstallmentsPlan {
    #[serde(rename = "type")]
    pub plan_type: InstallmentPlanType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<InstallmentPlanInterval>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum InstallmentPlanType {
    Bonus,
    FixedCount,
    Revolving,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum InstallmentPlanInterval {
    Month,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCardMandateOptions {
    pub amount: i64,
    pub amount_type: AmountType,
    pub interval: CardMandateInterval,
    pub reference: String,
    pub start_date: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types: Option<Vec<CardMandateSupportedType>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CardMandateInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CardMandateSupportedType {
    India,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCardThreeDSecure {
    pub cryptogram: String,
    pub transaction_id: String,
    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status: Option<ThreeDSecureAresTransStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator: Option<ThreeDSecureEci>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_indicator: Option<ThreeDSecureExemptionIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<CreatePmoCardThreeDSecureNetworkOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ThreeDSecureAresTransStatus {
    A,
    C,
    I,
    N,
    R,
    U,
    Y,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ThreeDSecureEci {
    #[serde(rename = "01")]
    #[strum(serialize = "01")]
    Eci01,
    #[serde(rename = "02")]
    #[strum(serialize = "02")]
    Eci02,
    #[serde(rename = "05")]
    #[strum(serialize = "05")]
    Eci05,
    #[serde(rename = "06")]
    #[strum(serialize = "06")]
    Eci06,
    #[serde(rename = "07")]
    #[strum(serialize = "07")]
    Eci07,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ThreeDSecureExemptionIndicator {
    LowRisk,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCardThreeDSecureNetworkOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<CreatePmoCardThreeDSecureCartesBancaires>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCardThreeDSecureCartesBancaires {
    pub cb_avalgo: CbAvalgo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_exemption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_score: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CbAvalgo {
    #[serde(rename = "0")]
    #[strum(serialize = "0")]
    Zero,
    #[serde(rename = "1")]
    #[strum(serialize = "1")]
    One,
    #[serde(rename = "2")]
    #[strum(serialize = "2")]
    Two,
    #[serde(rename = "3")]
    #[strum(serialize = "3")]
    Three,
    #[serde(rename = "4")]
    #[strum(serialize = "4")]
    Four,
    A,
}

// ---- Card Present ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCardPresent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CardPresentCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization_support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<CreatePmoCardPresentRouting>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCardPresentRouting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_priority: Option<CardPresentRoutingPriority>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CardPresentRoutingPriority {
    Domestic,
    International,
}

// ---- Small payment methods ----
pub type CreatePmoCashapp = CaptureAndSetup;
pub type CreatePmoCrypto = SetupOnly;
pub type CreatePmoEps = SetupOnly;
pub type CreatePmoFpx = SetupOnly;
pub type CreatePmoGiropay = SetupOnly;
pub type CreatePmoGrabpay = SetupOnly;
pub type CreatePmoIdeal = SetupOnly;
pub type CreatePmoKakaoPay = CaptureAndSetup;
pub type CreatePmoKrCard = CaptureAndSetup;
pub type CreatePmoLink = CaptureAndSetup;
pub type CreatePmoMbWay = SetupOnly;
pub type CreatePmoMobilepay = CaptureAndSetup;
pub type CreatePmoMultibanco = SetupOnly;
pub type CreatePmoNaverPay = CaptureAndSetup;
pub type CreatePmoOxxo = CreatePmoOxxoOptions;
pub type CreatePmoPaynow = SetupOnly;
pub type CreatePmoPayco = CaptureOnly;
pub type CreatePmoPaypay = CaptureOnly;
pub type CreatePmoPromptpay = SetupOnly;
pub type CreatePmoRevolutPay = CaptureAndSetup;
pub type CreatePmoSamsungPay = CaptureOnly;
pub type CreatePmoSatispay = CaptureOnly;
pub type CreatePmoTwint = SetupOnly;
pub type CreatePmoZip = SetupOnly;

// Ones with slightly special shapes below (customer_balance, klarna, konbini, etc.)

// ---- CustomerBalance ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCustomerBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<CreatePmoCustomerBalanceBankTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<CustomerBalanceFundingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCustomerBalanceBankTransfer {
    #[serde(rename = "type")]
    pub transfer_type: BankTransferType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<CreatePmoCustomerBalanceEuBankTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<Vec<RequestedAddressType>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoCustomerBalanceEuBankTransfer {
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum RequestedAddressType {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CustomerBalanceFundingType {
    BankTransfer,
}

// ---- Klarna ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoKlarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ManualCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand: Option<CreatePmoKlarnaOnDemand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<CreatePmoKlarnaSubscription>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoKlarnaOnDemand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_interval: Option<ChargeInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_interval_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChargeInterval {
    Day,
    Month,
    Week,
    Year,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoKlarnaSubscription {
    pub interval: ChargeInterval,
    pub reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing: Option<CreatePmoKlarnaSubscriptionNextBilling>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoKlarnaSubscriptionNextBilling {
    pub amount: i64,
    pub date: String,
}

// ---- Konbini ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoKonbini {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- NzBankAccount ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoNzBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}

// ---- Oxxo ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoOxxoOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- P24 ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoP24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

// ---- PayPal ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoPaypal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ManualCaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- Payto ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoPayto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePmoPaytoMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoPaytoMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<AmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<PaytoPaymentSchedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_per_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<PaytoPurpose>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PaytoPaymentSchedule {
    Adhoc,
    Annual,
    Daily,
    Fortnightly,
    Monthly,
    Quarterly,
    SemiAnnual,
    Weekly,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PaytoPurpose {
    DependantSupport,
    Government,
    Loan,
    Mortgage,
    Other,
    Pension,
    Personal,
    Retail,
    Salary,
    Tax,
    Utility,
}

// ---- Pix ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoPix {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof: Option<PixIofInclusion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePmoPixMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PixIofInclusion {
    Always,
    Never,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoPixMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof: Option<PixIofInclusion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<AmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<PixPaymentSchedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PixPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
}

// ---- SepaDebit ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePmoSepaDebitMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoSepaDebitMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}

// ---- Sofort ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoSofort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<SofortLanguage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SofortLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

// ---- Swish ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoSwish {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

// ---- Upi ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoUpi {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePmoUpiMandateOptions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoUpiMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<AmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
}

// ---- UsBankAccount ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoUsBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<CreatePmoUsBankAccountFinancialConnections>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePmoUsBankAccountMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CreatePmoUsBankAccountNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_purpose: Option<UsBankAccountTransactionPurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<UsBankAccountVerificationMethod>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoUsBankAccountFinancialConnections {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CreatePmoUsBankAccountFinancialConnectionsFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<FinancialConnectionsPrefetch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoUsBankAccountFinancialConnectionsFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum FinancialConnectionsPrefetch {
    Balances,
    Ownership,
    Transactions,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoUsBankAccountMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<UsBankAccountCollectionMethod>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountCollectionMethod {
    Paper,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoUsBankAccountNetworks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Vec<UsBankAccountNetworkRequested>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountNetworkRequested {
    Ach,
    UsDomesticWire,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountTransactionPurpose {
    Goods,
    Other,
    Services,
    Unspecified,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

// ---- WechatPay ----
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePmoWechatPay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<WechatPayClient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OptionSetupFutureUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum WechatPayClient {
    Android,
    Ios,
    Web,
}
