use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{prelude::*, tippytappy};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CampaignView {
    pub id: Uuid,
    pub name: String,
    pub region: DistrictView,
    pub primary_color: String,
    pub secondary_color: String,
    pub icon_url: Option<Url>,
    pub banner_url: Option<Url>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "bevy", derive(bevy::ecs::component::Component))]
pub struct CampaignDetails {
    pub id: Uuid,
    pub name: String,
    pub region: DistrictView,
    pub description: tippytappy::DocumentView,
    pub primary_color: String,
    pub secondary_color: String,
    pub icon_url: Option<Url>,
    pub banner_url: Option<Url>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
