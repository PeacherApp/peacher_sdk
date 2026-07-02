use crate::{paginated, prelude::*};

use serde::{Deserialize, Serialize};

/// The smallest reveal threshold a campaign may be created with.
pub const MIN_CAMPAIGN_THRESHOLD: i32 = 5;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCampaignRequest {
    /// The demand — the thing these people want to happen.
    pub name: String,
    pub region_id: i32,
    pub body: SetContentRequest,
    pub primary_color: String,
    pub secondary_color: String,
    /// Supporters needed for the campaign to reveal. At least
    /// [`MIN_CAMPAIGN_THRESHOLD`].
    pub threshold: i32,
    /// Officeholders the campaign pressures. Each must hold an active seat in
    /// the campaign's region. At least one is required.
    pub target_ids: Vec<i32>,
    /// Anchor the demand to a bill.
    pub legislation_id: Option<i32>,
}

/// Partial update of a campaign's settings. Only the `Some(_)` fields are
/// applied; the rest are left unchanged. Organizers only.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCampaignRequest {
    pub name: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct CampaignParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    /// Only campaigns rooted in this region.
    pub region_id: Option<i32>,
}

paginated!(CampaignParams);
