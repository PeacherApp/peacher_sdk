use crate::{
	prelude::{Handler, Method},
	stripe::checkout_session::CheckoutSession,
};

/// Parameters accepted by `POST /v1/checkout/sessions/:id/expire`.
///
/// See: <https://docs.stripe.com/api/checkout/sessions/expire>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpireCheckoutSession {
	pub id: String,
}

impl ExpireCheckoutSession {
	pub fn new(id: impl Into<String>) -> Self {
		Self { id: id.into() }
	}
}

impl Handler for ExpireCheckoutSession {
	type ResponseBody = CheckoutSession;
	fn method(&self) -> Method {
		Method::Post
	}
	fn path(&self) -> std::borrow::Cow<'_, str> {
		format!("/v1/checkout/sessions/{}/expire", self.id).into()
	}
}
