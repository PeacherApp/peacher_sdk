use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

use crate::prelude::*;

/// Why a campaign is being reported. Campaigns get categorical reasons
/// (unlike content's free text) because each maps to a distinct trust &
/// safety lane: a demand that doesn't match what's actually being organized,
/// pressure aimed at someone outside their official capacity, inauthentic
/// coordination, or plain spam.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum CampaignReportReason {
    /// The stated demand doesn't match what the campaign actually organizes.
    MisrepresentsTarget,
    /// Targets an individual rather than an officeholder in their official
    /// capacity.
    Harassment,
    /// Suspected inauthentic coordination.
    Astroturfing,
    Spam,
}

/// Request body for reporting a campaign.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCampaignReportRequest {
    pub reason: CampaignReportReason,
    /// Free-text context for moderators.
    pub details: String,
}

/// Request body for suspending a campaign (admins). The reason is shown
/// publicly on the campaign page.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SuspendCampaignRequest {
    pub reason: String,
}

/// Request body for archiving a campaign (admins) — removes it from
/// discovery entirely.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ArchiveCampaignRequest {
    pub reason: Option<String>,
}

/// Organizer-level changes and lifecycle transitions, recorded so a
/// campaign's history is inspectable — a campaign whose demand silently
/// shifts mid-flight has a problem, and participants deserve to see that.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CampaignAuditAction {
    Created,
    Revealed,
    Closed,
    Suspended,
    Unsuspended,
    Archived,
    ReviewApproved,
    RoleChanged,
    MemberRemoved,
    MemberBanned,
    MemberUnbanned,
    DemandEdited,
    /// The bill anchor was set, changed, or cleared.
    AnchorChanged,
}

/// One entry in a campaign's audit history.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CampaignAuditEntry {
    pub id: Uuid,
    /// `None` means the platform itself acted — the reveal, notably, is
    /// system-actored so the supporter whose commitment tipped the threshold
    /// is never named.
    pub actor: Option<MemberView>,
    pub action: CampaignAuditAction,
    pub detail: Option<serde_json::Value>,
    pub created_at: DateTime<FixedOffset>,
}
