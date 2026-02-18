use std::borrow::Cow;

use crate::{paginated, prelude::*};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use url::Url;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct MemberParams {
    /// Free text search across display_name, handle, and full_name
    pub freetext: Option<String>,
    /// A query for members following this member
    pub members_following: Option<i32>,
    /// A query for members that are followed by this member
    pub members_followed_by: Option<i32>,
    /// Filter by external ID
    pub external_id: Option<String>,

    #[serde(default)]
    pub order_by: MemberOrder,
    #[serde(default)]
    pub order: Ordering,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(MemberParams);

impl MemberParams {
    pub fn freetext(&self) -> Option<&str> {
        self.freetext.as_deref()
    }
    pub fn external_id(&self) -> Option<&str> {
        self.external_id.as_deref()
    }
    pub fn order(&self) -> Ordering {
        self.order
    }
    pub fn order_by(&self) -> MemberOrder {
        self.order_by
    }
}

#[derive(
    Default, Clone, Copy, EnumString, Display, Debug, PartialEq, Eq, Serialize, Deserialize,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MemberOrder {
    #[default]
    Id,
}

/// List members with optional filters
#[derive(Default)]
pub struct ListMembers {
    pub params: MemberParams,
}

impl ListMembers {
    pub fn new() -> Self {
        Self {
            params: MemberParams {
                page: Some(1),
                page_size: Some(20),
                external_id: None,
                freetext: None,
                ..Default::default()
            },
        }
    }

    pub fn page(mut self, page: u64) -> Self {
        self.params.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.params.page_size = Some(page_size);
        self
    }

    pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
        self.params.external_id = Some(external_id.into());
        self
    }

    pub fn freetext(mut self, freetext: impl Into<String>) -> Self {
        self.params.freetext = Some(freetext.into());
        self
    }
}

impl GetHandler for ListMembers {
    type ResponseBody = Paginated<MemberView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/members".into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

/// Get details for a specific member by ID
pub struct GetMemberDetails(pub i32);

impl GetHandler for GetMemberDetails {
    type ResponseBody = GetMemberDetailsResponse;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/members/{}", self.0).into()
    }
}

/// Get details for a specific member by handle
pub struct GetMemberByHandle(pub String);

impl GetMemberByHandle {
    pub fn new(handle: impl Into<String>) -> Self {
        Self(handle.into())
    }
}

impl GetHandler for GetMemberByHandle {
    type ResponseBody = GetMemberDetailsResponse;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/members/@{}", self.0).into()
    }
}

/// Get districts for a specific member
pub struct GetMemberDistricts(pub i32);

impl GetHandler for GetMemberDistricts {
    // Use Value for flexibility - actual type is MemberDistrictsResponse (defined in api crate)
    type ResponseBody = serde_json::Value;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/members/{}/districts", self.0).into()
    }
}

/// Follow a member (requires authentication)
pub struct FollowMember(pub i32);

impl Handler for FollowMember {
    type ResponseBody = FollowResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/members/{}/followers", self.0).into()
    }
}

/// Unfollow a member (requires authentication)
pub struct UnfollowMember(pub i32);

impl Handler for UnfollowMember {
    type ResponseBody = FollowResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/members/{}/followers", self.0).into()
    }
}

/// Get follower data for a member
pub struct GetMemberFollowerData(pub i32);

impl GetHandler for GetMemberFollowerData {
    type ResponseBody = FollowResponse;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/members/{}/followers", self.0).into()
    }
}

/// Request to create a new member
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateMemberRequest {
    pub display_name: String,
    pub full_name: Option<String>,
    pub bio: String,
    pub party: String,
    pub photo_url: Option<String>,
    pub external_metadata: Option<ExternalMetadata>,
}

impl NewMember for CreateMemberRequest {
    fn display_name(&self) -> String {
        self.display_name.clone()
    }
    fn full_name(&self) -> Option<String> {
        self.full_name.clone()
    }
    fn bio(&self) -> String {
        self.bio.clone()
    }
    fn email(&self) -> Option<String> {
        None
    }
    fn party(&self) -> Option<String> {
        Some(self.party.clone())
    }
    fn photo_url(&self) -> Option<String> {
        self.photo_url.clone()
    }
    fn public(&self) -> bool {
        true
    }
}

impl CreateMemberRequest {
    pub fn new(
        display_name: impl Into<String>,
        bio: impl Into<String>,
        party: impl Into<String>,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            full_name: None,
            bio: bio.into(),
            party: party.into(),
            photo_url: None,
            external_metadata: None,
        }
    }

    pub fn full_name(mut self, full_name: impl Into<String>) -> Self {
        self.full_name = Some(full_name.into());
        self
    }

    pub fn photo_url(mut self, photo_url: impl Into<String>) -> Self {
        self.photo_url = Some(photo_url.into());
        self
    }

    pub fn external_metadata(mut self, metadata: ExternalMetadata) -> Self {
        self.external_metadata = Some(metadata);
        self
    }
}

/// Request to update an existing member
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateMemberRequest {
    pub display_name: Option<String>,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub party: Option<String>,
    pub photo_url: Option<Url>,
}

impl UpdateMemberRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    pub fn full_name(mut self, full_name: impl Into<String>) -> Self {
        self.full_name = Some(full_name.into());
        self
    }

    pub fn bio(mut self, bio: impl Into<String>) -> Self {
        self.bio = Some(bio.into());
        self
    }

    pub fn party(mut self, party: impl Into<String>) -> Self {
        self.party = Some(party.into());
        self
    }

    pub fn photo_url(mut self, photo_url: Url) -> Self {
        self.photo_url = Some(photo_url);
        self
    }
}

/// Handler for creating a member
pub struct CreateMember {
    body: CreateMemberRequest,
}

impl CreateMember {
    pub fn new(body: CreateMemberRequest) -> Self {
        Self { body }
    }
}

impl Handler for CreateMember {
    type ResponseBody = MemberView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/members".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler for updating a member
pub struct UpdateMember {
    id: i32,
    body: UpdateMemberRequest,
}

impl UpdateMember {
    pub fn new(id: i32, body: UpdateMemberRequest) -> Self {
        Self { id, body }
    }
}

impl Handler for UpdateMember {
    type ResponseBody = MemberView;

    fn method(&self) -> Method {
        Method::Patch
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/members/{}", self.id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Ban a member with a reason
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BanMemberRequest {
    /// This is the reason for the ban
    pub reason: String,

    /// This field is for interior use only, meant to share
    /// additional context with administrators of the site.
    ///
    /// This is not visible to users without moderator capabilities.
    pub context: String,
}

/// Handler to ban a member
pub struct BanMember {
    member_id: i32,
    body: BanMemberRequest,
}

impl BanMember {
    pub fn new(member_id: i32, reason: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            member_id,
            body: BanMemberRequest {
                reason: reason.into(),
                context: context.into(),
            },
        }
    }
}

impl Handler for BanMember {
    type ResponseBody = BanInfo;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/members/{}/bans", self.member_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

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
    pub handle: Slug,
    pub photo: Option<String>,
    pub display_name: String,
    pub party: PartyView,
    pub auth_level: AuthLevel,
    pub external: Option<ExternalOwner>,
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
