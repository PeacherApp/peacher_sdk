use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Legislation sponsored by one or more followed members in the feed.
/// Groups co-sponsors of the same bill into a single item.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FollowedMembersSponsored {
    pub sponsors: Vec<FeedSponsor>,
    pub legislation: LegislationView,
}

/// A member-centric feed item: a vote event on legislation showing which
/// followed members voted and how.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FollowedMembersVoted {
    /// The specific vote event (chamber, name, date)
    pub vote: LegislationVote,
    /// The legislation being voted on
    pub legislation: LegislationView,
    /// How each followed member voted
    pub member_votes: Vec<MemberVoteValue>,
}

/// A new post created by a followed member.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FeedPost {
    pub member: MemberView,
    pub community: CommunityView,
    pub post_id: i32,
    pub at: DateTime<FixedOffset>,
}

impl From<FollowedMembersVoted> for FeedItem {
    fn from(value: FollowedMembersVoted) -> Self {
        Self::FollowedMembersVoted(value)
    }
}
impl From<FollowedMembersSponsored> for FeedItem {
    fn from(value: FollowedMembersSponsored) -> Self {
        Self::FollowedMembersSponsored(value)
    }
}
