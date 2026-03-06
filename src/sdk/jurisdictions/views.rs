use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JurisdictionView {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
}
impl JurisdictionView {
    pub fn into_get_view(
        self,
        sessions: impl IntoIterator<Item = SessionView>,
        chambers: impl IntoIterator<Item = ChamberView>,
    ) -> GetJurisdictionView {
        GetJurisdictionView {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            external_url: self.external_url,
            external_id: self.external_id,
            created_by_id: self.created_by_id,
            sessions: sessions.into_iter().collect(),
            chambers: chambers.into_iter().collect(),
        }
    }
    pub fn with_chambers(
        self,
        chambers: impl IntoIterator<Item = ChamberView>,
    ) -> JurisdictionWithChambers {
        JurisdictionWithChambers {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            external_url: self.external_url,
            external_id: self.external_id,
            created_by_id: self.created_by_id,
            chambers: chambers.into_iter().collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetJurisdictionView {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
    pub sessions: Vec<SessionView>,
    pub chambers: Vec<ChamberView>,
}

impl GetJurisdictionView {
    pub fn into_jurisdiction_view(self) -> JurisdictionView {
        JurisdictionView {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            external_url: self.external_url,
            external_id: self.external_id,
            created_by_id: self.created_by_id,
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
// #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
// pub struct SessionJurisdictionView {
//     pub id: i32,
//     pub name: String,
//     pub created_at: DateTime<FixedOffset>,
//     pub updated_at: DateTime<FixedOffset>,
//     pub external_id: Option<ExternalId>,
//     pub external_url: Option<Url>,
//     pub created_by_id: Option<i32>,
//     pub current_session: Option<SessionView>,
//     pub chambers: Vec<ChamberViewWithPartyBreakdown>,
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JurisdictionWithChambers {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
    pub chambers: Vec<ChamberView>,
}
