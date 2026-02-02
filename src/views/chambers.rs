use serde::{Deserialize, Serialize};

use crate::prelude::*;

// TODO: this will be removed when the N+1 query problem is resolved
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SmallChamberView {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberView {
    pub id: i32,
    pub name: String,
    pub jurisdiction: BasicJurisdictionView,
}
impl ChamberView {
    pub fn into_get_chamber_response(self, external: Option<ExternalOwner>) -> GetChamberResponse {
        GetChamberResponse {
            id: self.id,
            name: self.name,
            jurisdiction: self.jurisdiction,
            external,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetChamberResponse {
    pub id: i32,
    pub name: String,
    pub jurisdiction: BasicJurisdictionView,
    pub external: Option<ExternalOwner>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListChamberResponse {
    pub id: i32,
    pub name: String,
    pub external: Option<ExternalOwner>,
}

/// A member within a chamber session
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberSessionMember {
    pub member: MemberView,
    /// This does fit. We should definitely have this value in the get session response.
    /// makes life way easier.
    pub external: Option<ExternalOwner>,
    pub district_id: Option<i32>,
}

/// A chamber within a session
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberSessionView {
    pub chamber_id: i32,
    pub chamber_name: String,
    pub external: Option<ExternalOwner>,
    pub members: Vec<ChamberSessionMember>,
}

/// A district within a chamber
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberDistrictView {
    pub id: i32,
    pub name: String,
    pub representative: Option<MemberView>,
}

/// Response for getting chamber details with session support
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetChamberDetailsResponse {
    pub id: i32,
    pub name: String,
    pub jurisdiction: BasicJurisdictionView,
    pub external: Option<ExternalOwner>,
    /// All sessions available for this chamber
    pub sessions: Vec<SessionSummary>,
    /// The currently selected session
    pub current_session: Option<SessionSummary>,
    /// Members with their activity for the selected session
    pub members: Vec<ChamberMemberActivityView>,
    /// Districts in this chamber for the selected session
    pub districts: Vec<ChamberDistrictView>,
    /// Map ID for the chamber in the selected session
    pub map_id: Option<i32>,
}
