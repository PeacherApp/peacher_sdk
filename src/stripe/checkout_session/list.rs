use serde::{Deserialize, Serialize};

use crate::{
	prelude::{GetHandler, SdkParams},
	stripe::checkout_session::{CheckoutSession, CheckoutSessionStatus},
};

/// Parameters accepted by `GET /v1/checkout/sessions`.
///
/// See: <https://docs.stripe.com/api/checkout/sessions/list>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListCheckoutSessions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub customer: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub customer_account: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub customer_details: Option<ListCustomerDetailsFilter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ending_before: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub payment_intent: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub payment_link: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub starting_after: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<CheckoutSessionStatus>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expand: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListCustomerDetailsFilter {
	pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CheckoutSessionList {
	pub object: String,
	pub data: Vec<CheckoutSession>,
	pub has_more: bool,
	pub url: String,
}

impl GetHandler for ListCheckoutSessions {
	type ResponseBody = CheckoutSessionList;
	fn path(&self) -> std::borrow::Cow<'_, str> {
		"/v1/checkout/sessions".into()
	}
	fn params(&self) -> impl SdkParams {
		self.clone()
	}
}
