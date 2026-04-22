use ahash::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::stripe::checkout_session::{RecurringInterval, TaxBehavior};

/// A line item returned from Stripe.
///
/// See: <https://docs.stripe.com/api/checkout/sessions/line_items>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LineItem {
    pub id: String,
    pub object: String,
    pub amount_discount: i64,
    pub amount_subtotal: i64,
    pub amount_tax: i64,
    pub amount_total: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub discounts: Vec<LineItemDiscount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    pub quantity: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taxes: Vec<LineItemTax>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LineItemDiscount {
    pub amount: i64,
    pub discount: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LineItemTax {
    pub amount: i64,
    pub rate: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
}

/// The Price object — only the fields we typically need. Other payment detail
/// fields remain on the raw JSON response and can be parsed as needed.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Price {
    pub id: String,
    pub object: String,
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<BillingScheme>,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub product: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<PriceRecurring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<TiersMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<TransformQuantity>,
    #[serde(rename = "type")]
    pub price_type: PriceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum BillingScheme {
    PerUnit,
    Tiered,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PriceType {
    OneTime,
    Recurring,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TiersMode {
    Graduated,
    Volume,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CustomUnitAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PriceRecurring {
    pub interval: RecurringInterval,
    pub interval_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<UsageType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UsageType {
    Licensed,
    Metered,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TransformQuantity {
    pub divide_by: i64,
    pub round: TransformRound,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TransformRound {
    Down,
    Up,
}
