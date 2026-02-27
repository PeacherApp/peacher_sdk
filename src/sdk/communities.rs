use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{paginated, prelude::*};

// --- View types ---

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommunityView {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub member_count: u64,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommunityDetailView {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub rules: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub member_count: u64,
    pub created_by: MemberView,
    pub districts: Vec<CommunityDistrictView>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommunityDistrictView {
    pub map_id: i32,
    pub district_id: i32,
    pub district_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommunityMembershipView {
    pub community_id: i32,
    pub member_id: Option<i32>,
    pub role: Option<CommunityMemberRole>,
    pub joined_at: Option<DateTime<FixedOffset>>,
    pub member_count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CommunityMemberRole {
    Member,
    Moderator,
}

// --- Params ---

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct CommunityParams {
    pub search: Option<String>,
    pub district_map_id: Option<i32>,
    pub district_id: Option<i32>,
    pub member_id: Option<i32>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(CommunityParams);

// --- Request types ---

/// List communities with optional filters
#[derive(Default)]
pub struct ListCommunities {
    pub params: CommunityParams,
}

impl ListCommunities {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GetHandler for ListCommunities {
    type ResponseBody = Paginated<CommunityView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/communities".into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

/// Get community details by ID
pub struct GetCommunity(pub i32);

impl GetHandler for GetCommunity {
    type ResponseBody = CommunityDetailView;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/communities/{}", self.0).into()
    }
}

/// Request body for creating a community
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCommunityRequest {
    pub name: String,
    pub description: Option<String>,
    pub rules: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    /// Vec of (map_id, district_id) tuples
    pub district_ids: Vec<(i32, i32)>,
}

/// Create a new community
pub struct CreateCommunity {
    body: CreateCommunityRequest,
}

impl CreateCommunity {
    pub fn new(body: CreateCommunityRequest) -> Self {
        Self { body }
    }
}

impl Handler for CreateCommunity {
    type ResponseBody = CommunityDetailView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/communities".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Request body for updating a community
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCommunityRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rules: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub district_ids: Option<Vec<(i32, i32)>>,
}

/// Update an existing community
pub struct UpdateCommunity {
    id: i32,
    body: UpdateCommunityRequest,
}

impl UpdateCommunity {
    pub fn new(id: i32, body: UpdateCommunityRequest) -> Self {
        Self { id, body }
    }
}

impl Handler for UpdateCommunity {
    type ResponseBody = CommunityDetailView;

    fn method(&self) -> Method {
        Method::Put
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/communities/{}", self.id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Join a community
pub struct JoinCommunity(pub i32);

impl Handler for JoinCommunity {
    type ResponseBody = CommunityMembershipView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/communities/{}/members", self.0).into()
    }
}

/// Leave a community
pub struct LeaveCommunity(pub i32);

impl Handler for LeaveCommunity {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/communities/{}/members", self.0).into()
    }
}

/// Delete a community
pub struct DeleteCommunity(pub i32);

impl Handler for DeleteCommunity {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/communities/{}", self.0).into()
    }
}

/// Get community membership status for the current viewer
pub struct GetCommunityMembership(pub i32);

impl GetHandler for GetCommunityMembership {
    type ResponseBody = CommunityMembershipView;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/communities/{}/members", self.0).into()
    }
}
