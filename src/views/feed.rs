use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::views::{LegislationUpdateItem, SponsoredLegislationView};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, EnumString, strum::Display)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    LegislationUpdate,
    Legislation,
}

/// A single item in the unified activity feed
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
#[allow(
    clippy::large_enum_variant,
    reason = "In memory, this type is almost always stored behind a pointer to the heap. This may not be true for custom usage of this type. Please submit an issue if you need these types boxed."
)]
pub enum FeedItem {
    LegislationUpdate(LegislationUpdateItem),
    Legislation(SponsoredLegislationView),
}

impl FeedItem {
    pub fn item_type(&self) -> ItemType {
        match self {
            Self::LegislationUpdate(_) => ItemType::LegislationUpdate,
            Self::Legislation(_) => ItemType::Legislation,
        }
    }
    pub fn date_occurred(&self) -> Option<DateTime<FixedOffset>> {
        match self {
            Self::LegislationUpdate(item) => item.latest_vote_at,
            Self::Legislation(leg) => leg.sponsored_at.or(leg.legislation.introduced_at),
        }
    }
    pub fn actor_id(&self) -> Option<i32> {
        match self {
            Self::LegislationUpdate(item) => item
                .vote_events
                .first()
                .and_then(|e| e.representative_votes.first())
                .map(|rv| rv.member.id),
            Self::Legislation(legislation) => Some(legislation.sponsor.id),
        }
    }
}

impl From<LegislationUpdateItem> for FeedItem {
    fn from(value: LegislationUpdateItem) -> Self {
        Self::LegislationUpdate(value)
    }
}
impl From<SponsoredLegislationView> for FeedItem {
    fn from(value: SponsoredLegislationView) -> Self {
        Self::Legislation(value)
    }
}
