use serde::{Deserialize, Serialize};

use crate::{
    prelude::{GetHandler, SdkParams},
    stripe::checkout_session::CheckoutSession,
};

/// Parameters accepted by `GET /v1/checkout/sessions/:id`.
///
/// See: <https://docs.stripe.com/api/checkout/sessions/retrieve>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveCheckoutSession {
    pub id: String,
    pub query: RetrieveCheckoutSessionQuery,
}

impl RetrieveCheckoutSession {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            query: RetrieveCheckoutSessionQuery::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RetrieveCheckoutSessionQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

impl GetHandler for RetrieveCheckoutSession {
    type ResponseBody = CheckoutSession;
    fn path(&self) -> std::borrow::Cow<'_, str> {
        format!("/v1/checkout/sessions/{}", self.id).into()
    }
    fn params(&self) -> impl SdkParams {
        self.query.clone()
    }
}
