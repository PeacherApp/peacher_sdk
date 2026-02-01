use crate::prelude::*;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// Response for follow/unfollow operations
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FollowResponse {
    pub followed_at: Option<DateTime<FixedOffset>>,
    pub follower_count: u64,
    pub following_count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberView {
    pub id: i32,
    pub bio: String,
    pub full_name: Option<String>,
    pub handle: String,
    pub photo: Option<String>,
    pub display_name: String,
    pub party: PartyView,
    pub auth_level: AuthLevel,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalMemberResponse {
    pub member: MemberView,
    pub external: ExternalOwner,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetMemberDetailsResponse {
    pub id: i32,
    pub bio: String,
    pub full_name: Option<String>,
    pub handle: String,
    pub photo: Option<String>,
    pub display_name: String,
    pub party: PartyView,
    pub auth_level: AuthLevel,
    pub external: Option<ExternalOwner>,
    pub ban: Option<BanInfo>,
    pub follower_data: FollowResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BanInfo {
    pub ban_date: DateTime<FixedOffset>,
    pub ban_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberMemberActivityView {
    pub member: MemberView,
    pub activity: MemberActivity,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberActivity {
    pub sponsorships: u64,
    pub total_votes: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub not_voting: u64,
    pub absent: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberActivityResponse {
    pub session: SessionView,
    pub chamber: SmallChamberView,
    pub activity: MemberActivity,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberVotesResponse {
    pub votes: Vec<VoteView>,
    pub total_votes: usize,
    pub yes_votes: usize,
    pub no_votes: usize,
    pub not_voting: usize,
    pub absent: usize,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberDistrictInfo {
    pub district: SimpleBoundaryView,
    pub chamber: ChamberView,
    pub session: SessionView,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberDistrictsResponse {
    pub districts: Vec<MemberDistrictInfo>,
}

// Note: MemberDistrictsResponse and related types are defined in the api crate
// with additional methods like `load`. Use serde_json::Value in SDK for flexibility.
