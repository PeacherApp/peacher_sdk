use ahash::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
	prelude::{BodyBuilder, Handler, Method},
	stripe::{
		Address,
		checkout_session::{CheckoutSession, CreateLineItem, CreateShippingOption},
	},
};

/// Parameters accepted by `POST /v1/checkout/sessions/:id`.
///
/// See: <https://docs.stripe.com/api/checkout/sessions/update>
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateCheckoutSession {
	pub id: String,
	pub body: UpdateCheckoutSessionBody,
}

impl UpdateCheckoutSession {
	pub fn new(id: impl Into<String>) -> Self {
		Self {
			id: id.into(),
			body: UpdateCheckoutSessionBody::default(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCheckoutSessionBody {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collected_information: Option<UpdateCollectedInformation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub line_items: Option<Vec<CreateLineItem>>,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub metadata: HashMap<String, String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub shipping_options: Option<Vec<CreateShippingOption>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCollectedInformation {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub shipping_details: Option<UpdateShippingDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateShippingDetails {
	pub address: Address,
	pub name: String,
}

impl Handler for UpdateCheckoutSession {
	type ResponseBody = CheckoutSession;
	fn method(&self) -> Method {
		Method::Post
	}
	fn path(&self) -> std::borrow::Cow<'_, str> {
		format!("/v1/checkout/sessions/{}", self.id).into()
	}
	fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
		builder.qs(&self.body)
	}
}
