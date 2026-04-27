use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};
use url::Url;
use uuid::Uuid;

use crate::prelude::*;

/// Response for follow/unfollow operations
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub handle: Slug,
    pub photo: Option<Url>,
    pub display_name: String,
    pub party_id: Option<i32>,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
    pub auth_level: AuthLevel,
}

impl MemberView {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn with_party(self, party: Option<PartyView>) -> MemberWithPartyView {
        debug_assert_eq!(self.party_id, party.as_ref().map(|p| p.id));
        MemberWithPartyView {
            id: self.id,
            handle: self.handle,
            party,
            photo: self.photo,
            display_name: self.display_name,
            external_id: self.external_id,
            external_url: self.external_url,
            created_by_id: self.created_by_id,
            auth_level: self.auth_level,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CompactRepresentativeView {
    pub id: i32,
    pub display_name: String,
    pub handle: Slug,
    pub photo: Option<Url>,
    pub party: Option<PartyView>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberWithPartyView {
    pub id: i32,
    pub handle: Slug,
    pub party: Option<PartyView>,
    pub photo: Option<Url>,
    pub display_name: String,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
    pub auth_level: AuthLevel,
}

impl MemberWithPartyView {
    pub fn into_member_view(self) -> MemberView {
        MemberView {
            id: self.id,
            handle: self.handle,
            party_id: self.party.map(|p| p.id),
            photo: self.photo,
            display_name: self.display_name,
            external_id: self.external_id,
            external_url: self.external_url,
            created_by_id: self.created_by_id,
            auth_level: self.auth_level,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetMemberDetailsResponse {
    pub id: i32,
    pub bio: String,
    pub full_name: Option<String>,
    pub handle: Slug,
    pub photo: Option<Url>,
    pub display_name: String,
    pub party: Option<PartyView>,
    pub auth_level: AuthLevel,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub ban: Option<BanInfo>,
    pub created_by_id: Option<i32>,
    pub follower_data: FollowResponse,
    pub trust: Trust,
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
pub struct RepresentativeActivityResponse {
    pub session: SessionView,
    pub chamber: ChamberView,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RepresentativeMember {
    pub member: MemberWithPartyView,
    pub appointed_at: Option<NaiveDate>,
    pub vacated_at: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberDistrictInfo {
    pub district: SimpleBoundaryView,
    pub chamber: GetChamberView,
    pub session: SessionView,
    pub appointed_at: Option<NaiveDate>,
    pub vacated_at: Option<NaiveDate>,
}

/// returns the past data about a member's election/appointment to jurisdictions
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RepresentativeDistrictsResponse {
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

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
    VariantArray,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Trust {
    Untrusted,
    NewMember,
    Standard,
    Privileged,
    Moderator,
    Admin,
}
#[cfg(feature = "content")]
impl Trust {
    pub fn initial_summary_review_state(&self) -> ReviewState {
        match self {
            Trust::Untrusted | Trust::NewMember => ReviewState::UnderReview,
            _ => ReviewState::Public,
        }
    }
    pub fn initial_post_review_state(&self) -> ReviewState {
        match self {
            Trust::Untrusted => ReviewState::UnderReview,
            _ => ReviewState::Public,
        }
    }

    pub fn initial_comment_review_state(&self) -> ReviewState {
        match self {
            Trust::Untrusted => ReviewState::UnderReview,
            _ => ReviewState::Public,
        }
    }

    pub fn hide_on_report_threshold(&self) -> u32 {
        match self {
            Trust::Untrusted | Trust::NewMember => 1,
            Trust::Standard => 2,
            Trust::Privileged | Trust::Moderator | Trust::Admin => 10,
        }
    }
}

/// A single item in a member's activity feed.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum MemberActivityItemView {
    Post(PostActivityView),
    Comment(CommentActivityView),
    Summary(SummaryActivityView),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostActivityView {
    pub title: String,
    pub district_id: i32,
    pub content: ContentView,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommentActivityView {
    pub post_id: Uuid,
    pub post_title: String,
    pub content: ContentView,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SummaryActivityView {
    pub legislation_id: i32,
    pub legislation_title: String,
    pub kind: SummaryKind,
    pub content: ContentView,
}
