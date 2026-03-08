use std::borrow::Cow;

use crate::{paginated, prelude::*};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::EnumString;

mod variants;
pub use variants::*;

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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, EnumString, strum::Display)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    FollowedMembersVoted,
    FollowedMembersSponsored,
    CommunityPost,
    FollowedMemberPosted,
}

/// A single item in the unified activity feed
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
// #[expect(
//     clippy::large_enum_variant,
//     reason = "In memory, this type is almost always stored behind a pointer to the heap. This may not be true for custom usage of this type. Please submit an issue if you need these types boxed."
// )]
pub enum FeedItem {
    FollowedMembersVoted(FollowedMembersVoted),
    FollowedMembersSponsored(FollowedMembersSponsored),
    CommunityPost(FeedPost),
    FollowedMemberPosted(FeedPost),
}

impl FeedItem {
    pub fn item_type(&self) -> ItemType {
        match self {
            Self::FollowedMembersVoted(_) => ItemType::FollowedMembersVoted,
            Self::FollowedMembersSponsored(_) => ItemType::FollowedMembersSponsored,
            Self::CommunityPost(_) => ItemType::CommunityPost,
            Self::FollowedMemberPosted(_) => ItemType::FollowedMemberPosted,
        }
    }
    pub fn date_occurred(&self) -> Option<DateTime<FixedOffset>> {
        match self {
            Self::FollowedMembersVoted(item) => item.vote.occurred_at,
            Self::FollowedMembersSponsored(leg) => leg
                .sponsors
                .iter()
                .filter_map(|s| s.sponsored_at)
                .max()
                .or(leg.legislation.introduced_at),
            Self::CommunityPost(post) | Self::FollowedMemberPosted(post) => Some(post.at),
        }
    }
    pub fn actor_id(&self) -> Option<i32> {
        match self {
            Self::FollowedMembersVoted(item) => item.member_votes.first().map(|mv| mv.member.id),
            Self::FollowedMembersSponsored(leg) => leg.sponsors.first().map(|s| s.member.id),
            Self::CommunityPost(post) | Self::FollowedMemberPosted(post) => Some(post.member.id),
        }
    }
}
