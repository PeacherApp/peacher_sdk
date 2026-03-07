use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SmallCommunityView {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommunityView {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
impl CommunityView {
    pub fn with_count(self, member_count: u64) -> CommunityViewWithCount {
        CommunityViewWithCount {
            id: self.id,
            name: self.name,
            description: self.description,
            icon_url: self.icon_url,
            banner_url: self.banner_url,
            member_count,
            created_at: self.created_at,
        }
    }
    pub fn with_join_date(self, join_date: DateTime<FixedOffset>) -> CommunityViewWithJoinDate {
        CommunityViewWithJoinDate {
            id: self.id,
            name: self.name,
            description: self.description,
            icon_url: self.icon_url,
            banner_url: self.banner_url,
            join_date,
            created_at: self.created_at,
        }
    }
    pub fn with_details(
        self,
        rules: Option<String>,
        member_count: u64,
        created_by: MemberView,
        districts: impl IntoIterator<Item = DistrictView>,
    ) -> CommunityDetailView {
        CommunityDetailView {
            id: self.id,
            name: self.name,
            rules,
            updated_at: self.updated_at,
            description: self.description,
            icon_url: self.icon_url,
            banner_url: self.banner_url,
            member_count,
            created_by,
            districts: districts.into_iter().collect(),
            created_at: self.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommunityViewWithJoinDate {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub join_date: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommunityViewWithCount {
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
    pub districts: Vec<DistrictView>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
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
