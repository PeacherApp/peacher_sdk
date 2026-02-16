use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TagResponse {
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub created_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub updated_at: DateTime<FixedOffset>,
    pub id: i32,
    pub name: String,
    pub visible: bool,
}

/// Tag with post count for discovery page
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TagWithCount {
    pub id: i32,
    pub name: String,
    pub visible: bool,
    pub post_count: u64,
}
