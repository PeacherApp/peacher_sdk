use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::{paginated, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct CommunityReportsParams {
    /// ignored. Use the list reports route under communities.
    #[serde(skip)]
    pub community_id: Option<i32>,
    pub review_status: CommaSeparated<ReviewStatus>,
    pub created_after: Option<DateTime<FixedOffset>>,
    pub created_before: Option<DateTime<FixedOffset>>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(CommunityReportsParams);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct BasicCommunityParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub order_by: CommunityOrder,
    pub order: Ordering,
}
paginated!(BasicCommunityParams);

/// How to order the communities
#[derive(
    Serialize, Deserialize, Default, Clone, EnumString, Display, Debug, PartialEq, Eq, Copy,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CommunityOrder {
    #[default]
    Members,
    CreatedAt,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct CommunityParams {
    pub search: Option<String>,
    pub district_map_id: Option<i32>,
    pub district_id: Option<i32>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub order_by: CommunityOrder,
    pub order: Ordering,
}

paginated!(CommunityParams);

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
    type ResponseBody = Paginated<CommunityViewWithCount>;

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
    pub district_ids: Vec<DistrictId>,
    pub primary_color: String,
    pub secondary_color: String,
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
    pub district_ids: Option<Vec<(i32, i32)>>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
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

/// List communities at the viewer's location that they are NOT a member of
#[derive(Default)]
pub struct ListLocationCommunities {
    pub params: BasicCommunityParams,
}

impl GetHandler for ListLocationCommunities {
    type ResponseBody = Paginated<CommunityView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/location/communities".into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

/// List communities the authenticated user IS a member of
#[derive(Default)]
pub struct ListAccountCommunities {
    pub params: BasicCommunityParams,
}

impl GetHandler for ListAccountCommunities {
    type ResponseBody = Paginated<CommunityView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/account/communities".into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

/// Params for listing community members (mod/owner only)
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct CommunityMembersParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(CommunityMembersParams);

/// Get community membership status for the current viewer
pub struct GetCommunityMembership(pub i32);

impl GetHandler for GetCommunityMembership {
    type ResponseBody = CommunityMembershipView;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/communities/{}/members", self.0).into()
    }
}
