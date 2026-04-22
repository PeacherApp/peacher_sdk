use serde::{Deserialize, Serialize};

use crate::{
    prelude::{GetHandler, SdkParams},
    stripe::checkout_session::LineItem,
};

/// Parameters accepted by `GET /v1/checkout/sessions/:id/line_items`.
///
/// See: <https://docs.stripe.com/api/checkout/sessions/line_items>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListCheckoutSessionLineItems {
    pub id: String,
    pub query: ListCheckoutSessionLineItemsQuery,
}

impl ListCheckoutSessionLineItems {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            query: ListCheckoutSessionLineItemsQuery::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListCheckoutSessionLineItemsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LineItemList {
    pub object: String,
    pub data: Vec<LineItem>,
    pub has_more: bool,
    pub url: String,
}

impl GetHandler for ListCheckoutSessionLineItems {
    type ResponseBody = LineItemList;
    fn path(&self) -> std::borrow::Cow<'_, str> {
        format!("/v1/checkout/sessions/{}/line_items", self.id).into()
    }
    fn params(&self) -> impl SdkParams {
        self.query.clone()
    }
}
