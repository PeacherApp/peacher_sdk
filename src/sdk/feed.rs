use std::borrow::Cow;

use crate::{paginated, prelude::*};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Get the current user's feed
#[derive(Default, Clone, Debug)]
pub struct GetFeed(pub FeedParams);

impl GetHandler for GetFeed {
    type ResponseBody = Paginated<FeedItem>;

    fn path(&self) -> Cow<'_, str> {
        "/api/feed".into()
    }

    fn params(&self) -> impl SdkParams {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct FeedParams {
    // A set value here is not respected by Peacher's API.
    #[serde(skip)]
    pub location_id: Option<u64>,
    // A set value here is not respected by Peacher's API.
    #[serde(skip)]
    pub member_id: Option<i32>,
    /// time
    pub order_by: FeedOrder,
    /// asc | desc
    pub order: Ordering,

    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(FeedParams);

impl FeedParams {
    pub fn set_member(mut self, member_id: i32) -> Self {
        self.member_id = Some(member_id);
        self
    }
}

/// How the feed should be ordered
#[derive(
    Serialize, Deserialize, Default, Clone, EnumString, Display, Debug, PartialEq, Eq, Copy,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FeedOrder {
    #[default]
    Time,
}

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
#[expect(
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
            Self::LegislationUpdate(item) => item.vote.occurred_at,
            Self::Legislation(leg) => leg
                .sponsors
                .iter()
                .filter_map(|s| s.sponsored_at)
                .max()
                .or(leg.legislation.introduced_at),
        }
    }
    pub fn actor_id(&self) -> Option<i32> {
        match self {
            Self::LegislationUpdate(item) => {
                item.member_votes.first().map(|mv| mv.member.id)
            }
            Self::Legislation(leg) => leg.sponsors.first().map(|s| s.member.id),
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
