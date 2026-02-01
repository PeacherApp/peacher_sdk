use std::borrow::Cow;

use crate::prelude::*;

/// List members with optional filters
pub struct ListMembers {
    page: u64,
    page_size: u64,
    external_id: Option<String>,
}

impl ListMembers {
    pub fn new() -> Self {
        Self {
            page: 1,
            page_size: 20,
            external_id: None,
        }
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }
}

impl Default for ListMembers {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
            external_id: None,
        }
    }
}

impl GetHandler for ListMembers {
    type ResponseBody = Paginated<MemberView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/members".into()
    }

    fn params(&self) -> impl SdkParams {
        #[derive(Serialize)]
        struct Params {
            page: u64,
            page_size: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            external_id: Option<String>,
        }
        Params {
            page: self.page,
            page_size: self.page_size,
            external_id: self.external_id.clone(),
        }
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

use serde::{Deserialize, Serialize};

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
    pub photo_url: Option<String>,
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

    pub fn photo_url(mut self, photo_url: impl Into<String>) -> Self {
        self.photo_url = Some(photo_url.into());
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
