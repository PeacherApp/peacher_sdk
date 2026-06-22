use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;

/// Who is allowed to join a campaign and how.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum CampaignVisibility {
    /// Anyone may find the campaign and join instantly.
    #[default]
    PublicOpen,
    /// Anyone may find the campaign and request to join; an organizer approves.
    PublicRequest,
    /// Only invited members may join.
    InviteOnly,
}

/// A member's role within a campaign.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum CampaignRole {
    Member,
    Organizer,
}

/// Request body for joining a campaign. `invite_token` is required for
/// `invite_only` campaigns; `message` is attached to a pending request for
/// `public_request` campaigns.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JoinCampaignRequest {
    pub invite_token: Option<Uuid>,
    pub message: Option<String>,
}

/// The result of attempting to join a campaign.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum JoinOutcome {
    /// The caller is now a member.
    Joined,
    /// A join request was recorded and awaits organizer approval.
    RequestPending,
    /// The campaign is invite-only and a valid invite is required.
    NeedsInvite,
}

/// A pending join request, shown to organizers.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JoinRequestView {
    pub id: Uuid,
    pub member: MemberView,
    pub message: Option<String>,
    pub created_at: DateTime<FixedOffset>,
}

/// Request body for creating an invite. With no `invited_handle` the invite is
/// a shareable link; with one it targets a specific member by handle.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateInviteRequest {
    pub invited_handle: Option<String>,
    pub expires_in_secs: Option<i64>,
    pub max_uses: Option<i32>,
}

/// An invite to a campaign.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InviteView {
    pub token: Uuid,
    pub invited_member_id: Option<i32>,
    pub created_at: DateTime<FixedOffset>,
    pub expires_at: Option<DateTime<FixedOffset>>,
    pub max_uses: Option<i32>,
    pub uses: i32,
}

/// A member of a campaign, with their role.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CampaignMemberView {
    pub member: MemberView,
    pub role: CampaignRole,
    pub joined_at: DateTime<FixedOffset>,
}

/// Request body for changing a member's role.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetRoleRequest {
    pub role: CampaignRole,
}
