use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{paginated, prelude::*};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LoginHistory {
    pub last_login: Option<DateTime<FixedOffset>>,
    pub last_auth_method: Option<String>,
    pub total_logins: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ModerationContentItem {
    pub id: Uuid,
    pub kind: ContentTypeId,
    pub searchable_text: String,
    pub created_at: DateTime<FixedOffset>,
    pub removed_at: Option<DateTime<FixedOffset>>,
    pub reason_removed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ModerationRatingItem {
    pub content_item_id: Uuid,
    pub sentiment: Sentiment,
    pub created_at: DateTime<FixedOffset>,
    pub content_excerpt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ContentCounts {
    pub posts: u64,
    pub comments: u64,
    pub summaries: u64,
    pub removed: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberModerationView {
    pub member: MemberView,
    pub email: Option<String>,
    pub trustworthy_score: i32,
    pub created_at: DateTime<FixedOffset>,
    pub login_history: LoginHistory,
    pub content_counts: ContentCounts,
    pub recent_content: Vec<ModerationContentItem>,
    pub recent_ratings: Vec<ModerationRatingItem>,
    pub reports_against: Vec<ReportView>,
    pub reports_filed: Vec<ReportView>,
    pub ban: Option<BannedMemberView>,
    pub follower_count: u64,
    pub following_count: u64,
}

/// Handler to get moderation details for a member
pub struct GetMemberModerationDetails(pub i32);

impl GetHandler for GetMemberModerationDetails {
    type ResponseBody = MemberModerationView;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/moderation/members/{}", self.0).into()
    }
}

/// Query parameters for listing bans
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct BanParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(BanParams);

/// List all bans (admin/moderator only)
pub struct ListBans {
    pub params: BanParams,
}

impl Default for ListBans {
    fn default() -> Self {
        Self {
            params: BanParams {
                page: Some(0),
                page_size: Some(20),
            },
        }
    }
}

impl ListBans {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GetHandler for ListBans {
    type ResponseBody = Paginated<BannedMemberView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/moderation/bans".into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "reason", rename_all = "snake_case")]
pub enum ToggleAccountCreation {
    Disable(String),
    Enable,
}
