use crate::prelude::*;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberView {
    pub id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub name: String,
    pub jurisdiction_id: i32,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
}
impl ChamberView {
    pub fn into_get_chamber_response(self, jurisdiction: JurisdictionView) -> GetChamberView {
        debug_assert_eq!(self.jurisdiction_id, jurisdiction.id);
        GetChamberView {
            id: self.id,
            name: self.name,
            jurisdiction,
            created_at: self.created_at,
            updated_at: self.updated_at,
            external_id: self.external_id,
            external_url: self.external_url,
            created_by_id: self.created_by_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetChamberView {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub jurisdiction: JurisdictionView,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
}

/// A district within a chamber
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberDistrictView {
    pub id: i32,
    pub name: String,
    pub representatives: Vec<MemberWithPartyView>,
}
