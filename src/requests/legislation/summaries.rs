use crate::views::MemberView;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request to create a new summary for a piece of legislation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content")]
pub enum CreateSummaryRequest {
    Document(serde_json::Value),
    Markdown(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SummaryView {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub searchable_text: String,
    pub document: serde_json::Value,
    pub visibility: Visibility,
    pub author: Option<MemberView>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListSummariesResponse {
    pub summaries: Vec<SummaryView>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Visibility {
    NotVisible,
    Public,
}
