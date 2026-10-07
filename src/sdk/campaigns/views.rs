use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{prelude::*, tippytappy};

/// Where a campaign is in its lifecycle.
///
/// Campaigns build behind a closed door: while `Building`, the demand and the
/// supporter count are public but supporter names are not. When the supporter
/// count reaches the threshold the campaign reveals — everyone's names go
/// public together and the phase becomes `Live`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum CampaignPhase {
    /// Gathering private commitments toward the reveal threshold.
    #[default]
    Building,
    /// Revealed — the war room is open and supporter names are public.
    Live,
    /// Ended by its organizers (won, or died quietly).
    Closed,
    /// Suspended by platform admins; visible with a public reason.
    Suspended,
    /// Removed from discovery by platform admins.
    Archived,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CampaignView {
    pub id: Uuid,
    /// The demand — the thing these people want to happen.
    pub name: String,
    pub region: DistrictView,
    pub phase: CampaignPhase,
    /// Active supporters (private commitments while building).
    pub supporters: u64,
    pub threshold: i32,
    pub primary_color: String,
    pub secondary_color: String,
    pub icon_url: Option<Url>,
    pub banner_url: Option<Url>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CampaignDetails {
    pub id: Uuid,
    /// The demand — the thing these people want to happen.
    pub name: String,
    pub region: DistrictView,
    pub phase: CampaignPhase,
    /// Active supporters (private commitments while building).
    pub supporters: u64,
    pub threshold: i32,
    pub revealed_at: Option<DateTime<FixedOffset>>,
    /// Present when the campaign is suspended; shown publicly.
    pub suspended_reason: Option<String>,
    /// The officeholders this campaign is pressuring.
    pub targets: Vec<MemberView>,
    /// The bill this campaign is anchored to, if any.
    pub legislation_id: Option<i32>,
    pub description: tippytappy::DocumentView,
    pub primary_color: String,
    pub secondary_color: String,
    pub icon_url: Option<Url>,
    pub banner_url: Option<Url>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
