use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::views::{AttachmentResponse, MemberView};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MessageDetail {
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub created_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub updated_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>, format = DateTime))]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    pub id: i32,
    pub content: String,
    pub author: MemberView,
    pub attachments: Vec<AttachmentResponse>,
    pub likes: u64,
    pub user_liked: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MessageResponse {
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub created_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub updated_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>, format = DateTime))]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    pub id: i32,
    pub parent: Option<i32>,
    pub content: String,
    pub author: i32,
}
