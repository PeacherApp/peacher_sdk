use std::borrow::Cow;

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The sentiment provided for a piece of content
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Sentiment {
    Positive,
    Negative,
}

/// Request body to rate a piece of content
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RateContentRequest {
    pub sentiment: Sentiment,
}

/// Response to rating a piece of content
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RateContentResponse {
    pub sentiment: Sentiment,
}

/// Handler to rate content
pub struct RateContent {
    content_id: Uuid,
    body: RateContentRequest,
}

impl RateContent {
    pub fn new(content_id: Uuid, sentiment: Sentiment) -> Self {
        Self {
            content_id,
            body: RateContentRequest { sentiment },
        }
    }

    pub fn positive(content_id: Uuid) -> Self {
        Self::new(content_id, Sentiment::Positive)
    }

    pub fn negative(content_id: Uuid) -> Self {
        Self::new(content_id, Sentiment::Negative)
    }
}

impl Handler for RateContent {
    type ResponseBody = RateContentResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/content/{}/rate", self.content_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler to delete a content rating
pub struct DeleteContentRating(pub Uuid);

impl Handler for DeleteContentRating {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/content/{}/rate", self.0).into()
    }
}
