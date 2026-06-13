use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::geometry::Vec2;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ActionItem {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub created_by: i32,
    pub updated_by: i32,
    pub offset: Vec2,
    pub extent: Vec2,
    pub parent_item: Option<Uuid>,
    pub details: ActionItemDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content")]
pub enum ActionItemDetails {
    Phonebank(PhoneBankDetails),
    Meetup(MeetupDetails),
    Custom(serde_json::Value),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PhoneBankDetails {
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MeetupDetails {
    pub location: String,
}
