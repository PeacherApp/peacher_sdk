mod payment_intent;
pub use payment_intent::*;

pub mod client;

#[cfg(test)]
mod tests;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// `POST /v1/payment_intents` — carries [`CreatePaymentIntentBody`] and decodes to [`PaymentIntent`].
///
/// Stripe expects `application/x-www-form-urlencoded` with bracket-notation nesting,
/// so the body is serialized via [`BodyBuilder::qs`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentIntent {
    pub body: CreatePaymentIntentBody,
}

impl CreatePaymentIntent {
    pub fn new(body: CreatePaymentIntentBody) -> Self {
        Self { body }
    }
}

impl Handler for CreatePaymentIntent {
    type ResponseBody = PaymentIntent;
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/v1/payment_intents".into()
    }
    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.qs(&self.body)
    }
}
