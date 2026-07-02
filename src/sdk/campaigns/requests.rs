use crate::{paginated, prelude::*};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCampaignRequest {
    pub name: String,
    pub region_id: i32,
    pub body: SetContentRequest,
    pub primary_color: String,
    pub secondary_color: String,
    pub visibility: CampaignVisibility,
}

/// Partial update of a campaign's settings. Only the `Some(_)` fields are
/// applied; the rest are left unchanged. Organizers only.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCampaignRequest {
    pub name: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub visibility: Option<CampaignVisibility>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct CampaignParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(CampaignParams);
