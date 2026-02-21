use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

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
    pub handle: Slug,
    pub photo: Option<String>,
    pub display_name: String,
    pub party_id: Option<i32>,
    pub auth_level: AuthLevel,
}

impl MemberView {
    pub fn with_party(self, party: Option<PartyView>) -> MemberWithPartyView {
        debug_assert_eq!(self.party_id, party.as_ref().map(|p| p.id));
        MemberWithPartyView {
            id: self.id,
            bio: self.bio,
            full_name: self.full_name,
            handle: self.handle,
            party,
            photo: self.photo,
            display_name: self.display_name,
            auth_level: self.auth_level,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberWithPartyView {
    pub id: i32,
    pub bio: String,
    pub full_name: Option<String>,
    pub handle: Slug,
    pub party: Option<PartyView>,
    pub photo: Option<String>,
    pub display_name: String,
    pub auth_level: AuthLevel,
}

impl MemberWithPartyView {
    pub fn into_member_view(self) -> MemberView {
        MemberView {
            id: self.id,
            bio: self.bio,
            full_name: self.full_name,
            handle: self.handle,
            party_id: self.party.map(|p| p.id),
            photo: self.photo,
            display_name: self.display_name,
            auth_level: self.auth_level,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetMemberDetailsResponse {
    pub id: i32,
    pub bio: String,
    pub full_name: Option<String>,
    pub handle: Slug,
    pub photo: Option<String>,
    pub display_name: String,
    pub party: Option<PartyView>,
    pub auth_level: AuthLevel,
    pub external_id: Option<String>,
    pub external_url: Option<String>,
    pub ban: Option<BanInfo>,
    pub follower_data: FollowResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BanInfo {
    pub ban_date: DateTime<FixedOffset>,
    pub ban_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberMemberActivityView {
    pub member: MemberWithPartyView,
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

/// View for a banned member in the admin ban log
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BannedMemberView {
    pub member: MemberView,
    pub banned_by: MemberView,
    pub ban_date: DateTime<FixedOffset>,
    pub ban_reason: String,
    pub admin_context: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Trust {
    Untrusted,
    NewMember,
    Standard,
    Privileged,
    Moderator,
    Admin,
}
impl Trust {
    pub fn initial_summary_visibility(&self) -> Visibility {
        match self {
            Trust::Untrusted | Trust::NewMember => Visibility::NotVisible,
            _ => Visibility::Public,
        }
    }
}
