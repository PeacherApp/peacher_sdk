use serde::{Deserialize, Serialize};

/// The sentiment provided for a piece of content
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
