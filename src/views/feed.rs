use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::views::{GroupedVoteFeedItem, SponsoredLegislationView};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, EnumString, strum::Display)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Vote,
    Legislation,
}

/// A single item in the unified activity feed
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum FeedItem {
    Vote(GroupedVoteFeedItem),
    Legislation(SponsoredLegislationView),
}

impl FeedItem {
    pub fn item_type(&self) -> ItemType {
        match self {
            Self::Vote(_) => ItemType::Vote,
            Self::Legislation(_) => ItemType::Legislation,
        }
    }
    pub fn date_occurred(&self) -> Option<DateTime<FixedOffset>> {
        match self {
            Self::Vote(grouped) => grouped.legislation_vote.occurred_at,
            Self::Legislation(legislation) => legislation.legislation.introduced_at,
        }
    }
    pub fn actor_id(&self) -> Option<i32> {
        match self {
            Self::Vote(grouped) => grouped
                .votes_by_type
                .yes
                .first()
                .or(grouped.votes_by_type.no.first())
                .or(grouped.votes_by_type.absent.first())
                .or(grouped.votes_by_type.not_voting.first())
                .map(|m| m.id),
            Self::Legislation(legislation) => Some(legislation.sponsor.id),
        }
    }
}

impl From<GroupedVoteFeedItem> for FeedItem {
    fn from(value: GroupedVoteFeedItem) -> Self {
        Self::Vote(value)
    }
}
impl From<SponsoredLegislationView> for FeedItem {
    fn from(value: SponsoredLegislationView) -> Self {
        Self::Legislation(value)
    }
}
