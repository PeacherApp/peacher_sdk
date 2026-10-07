use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;

/// A member's role within a campaign.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum CampaignRole {
    Member,
    Organizer,
}

/// Request body for committing to a campaign. While a campaign is building,
/// committing is the private act of support that counts toward the reveal
/// threshold. `invite_token` marks the commitment as arriving through an
/// organizer's recruitment link.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommitToCampaignRequest {
    pub invite_token: Option<Uuid>,
}

/// The result of committing to a campaign.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommitOutcome {
    /// Active supporters after this commitment.
    pub supporters: u64,
    pub threshold: i32,
    /// True once the campaign has revealed — possibly triggered by this very
    /// commitment.
    pub revealed: bool,
}

/// The caller's own standing in a campaign.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommitmentStatus {
    pub committed: bool,
    pub role: Option<CampaignRole>,
    pub banned: bool,
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

/// Request body for banning a member from a campaign. A ban removes the member
/// and blocks them from recommitting, even with an invite. The `reason` is
/// optional and shown to organizers.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BanCampaignMemberRequest {
    pub reason: Option<String>,
}

/// A member who has been banned from a campaign, shown to organizers.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CampaignBanView {
    pub member: MemberView,
    pub reason: Option<String>,
    pub banned_at: DateTime<FixedOffset>,
}
