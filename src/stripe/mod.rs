mod payment_intent;
pub use payment_intent::*;

mod checkout_session;
pub use checkout_session::*;

mod webhook;
pub use webhook::*;

pub mod client;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Deserializer};

/// Deserializes a JSON `null` as `T::default()`.
///
/// Stripe occasionally returns `null` for fields our types model as
/// sequences/maps (e.g. `customer_details.tax_ids: null` instead of `[]`).
/// Pair with `#[serde(default, deserialize_with = "null_as_default")]`
/// so both an absent key and an explicit `null` decode to the default.
pub(crate) fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: Default + Deserialize<'de>,
{
	Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

// use crate::prelude::*;
// use serde::{Deserialize, Serialize};

// /// `POST /v1/payment_intents` — carries [`CreatePaymentIntentBody`] and decodes to [`PaymentIntent`].
// ///
// /// Stripe expects `application/x-www-form-urlencoded` with bracket-notation nesting,
// /// so the body is serialized via [`BodyBuilder::qs`].
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
// pub struct CreatePaymentIntent {
//     pub body: CreatePaymentIntentBody,
// }

// impl CreatePaymentIntent {
//     pub fn new(body: CreatePaymentIntentBody) -> Self {
//         Self { body }
//     }
// }
