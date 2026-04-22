use ahash::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::stripe::payment_intent::{
    AllowRedisplay, EmptyOptions, PaymentMethodBillingDetails, PaymentMethodType,
};

/// Request-side payload for `payment_method_data`.
///
/// Only the sub-hash matching [`Self::data_type`] should be populated. Types that
/// Stripe documents as empty objects (e.g. `affirm`, `alipay`, `giropay`, …) don't
/// need their sub-hash set at all — the `type` field is what selects the payment method.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodData {
    #[serde(rename = "type")]
    pub data_type: PaymentMethodType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreatePaymentMethodDataAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<AllowRedisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreatePaymentMethodDataAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreatePaymentMethodDataBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<PaymentMethodBillingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePaymentMethodDataBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreatePaymentMethodDataEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreatePaymentMethodDataFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreatePaymentMethodDataIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreatePaymentMethodDataKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<EmptyOptions>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<CreatePaymentMethodDataNaverPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<CreatePaymentMethodDataNzBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreatePaymentMethodDataP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payco: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<CreatePaymentMethodDataPayto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreatePaymentMethodDataRadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreatePaymentMethodDataSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreatePaymentMethodDataSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<CreatePaymentMethodDataUpi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreatePaymentMethodDataUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<EmptyOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<EmptyOptions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataAcssDebit {
    pub account_number: String,
    pub institution_number: String,
    pub transit_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataAuBecsDebit {
    pub account_number: String,
    pub bsb_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataBacsDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataBoleto {
    pub tax_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataEps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataFpx {
    pub bank: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataIdeal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataKlarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreatePaymentMethodDataKlarnaDob>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataKlarnaDob {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataNaverPay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<NaverPayFunding>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum NaverPayFunding {
    Card,
    Points,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataNzBankAccount {
    pub account_number: String,
    pub bank_code: String,
    pub branch_code: String,
    pub suffix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataP24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataPayto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataRadarOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataSepaDebit {
    pub iban: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataSofort {
    pub country: SofortCountry,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SofortCountry {
    AT,
    BE,
    DE,
    ES,
    IT,
    NL,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataUpi {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePaymentMethodDataUpiMandateOptions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataUpiMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<super::create::AmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentMethodDataUsBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UsBankAccountHolderTypeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UsBankAccountTypeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountHolderTypeInput {
    Company,
    Individual,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsBankAccountTypeInput {
    Checking,
    Savings,
}
