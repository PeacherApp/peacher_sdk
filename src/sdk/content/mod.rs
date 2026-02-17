mod rate;
pub use rate::*;

use crate::sdk::MemberView;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ContentDetails {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub searchable_text: String,
    pub document: serde_json::Value,
    pub author: Option<MemberView>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemovedContent {
    pub reason_removed: String,
    pub removed_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
#[expect(clippy::large_enum_variant)]
pub enum ContentView {
    Removed(RemovedContent),
    Content(ContentDetails),
}
