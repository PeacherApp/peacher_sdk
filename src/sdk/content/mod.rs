mod rate;
pub use rate::*;

use crate::sdk::MemberView;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ContentDetails {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub searchable_text: String,
    pub document: serde_json::Value,
    pub author: Option<MemberView>,
    /// This is the sum of sentiments where
    /// +1 is a positive sentiment, and -1 is a negative sentiment.
    pub rating: i32,
    pub viewer_sentiment: Option<Sentiment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemoveContentRequest {
    pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemovedContent {
    pub id: Uuid,
    pub reason_removed: String,
    pub removed_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Post,
    Comment,
    Summary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AdminContentView {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub searchable_text: String,
    pub document: serde_json::Value,
    pub kind: ContentType,
    pub author: Option<i32>,
    pub reason_removed: Option<String>,
    pub removed_by: Option<i32>,
    pub removed_at: Option<DateTime<FixedOffset>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
#[expect(clippy::large_enum_variant)]
pub enum ContentView {
    Removed(RemovedContent),
    Content(ContentDetails),
}
impl ContentView {
    pub fn id(&self) -> Uuid {
        match self {
            ContentView::Removed(removed) => removed.id,
            ContentView::Content(content) => content.id,
        }
    }
}

/// Request to create or update content.
///
/// This may be a new summary, an article, a comment, or otherwise.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum SetContentRequest {
    Document(serde_json::Value),
    Markdown(String),
}
