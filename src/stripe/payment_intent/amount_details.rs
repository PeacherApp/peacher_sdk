use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AmountDetailsError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<AmountDetailsLineItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<AmountDetailsShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<AmountDetailsTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<AmountDetailsTip>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<AmountDetailsErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AmountDetailsErrorCode {
    AmountDetailsAmountMismatch,
    AmountDetailsTaxShippingDiscountGreaterThanAmount,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsLineItems {
    pub object: String,
    pub data: Vec<AmountDetailsLineItem>,
    pub has_more: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsLineItem {
    pub id: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<AmountDetailsLineItemPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    pub product_name: String,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<AmountDetailsLineItemTax>,
    pub unit_cost: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsLineItemPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<AmountDetailsLineItemPaymentMethodOptionsPaypal>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsLineItemPaymentMethodOptionsPaypal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<AmountDetailsLineItemPaymentMethodOptionsPaypalCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sold_by: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AmountDetailsLineItemPaymentMethodOptionsPaypalCategory {
    DigitalGoods,
    Donation,
    PhysicalGoods,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsLineItemTax {
    pub total_tax_amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsShipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_postal_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsTax {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amount: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AmountDetailsTip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
