use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AttachmentResponse {
    pub id: i32,
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub created_at: DateTime<FixedOffset>,
    pub name: String,
    pub size: i64,
    pub file_type: String,
    pub download_url: String,
    pub preview_url: Option<String>,
}
