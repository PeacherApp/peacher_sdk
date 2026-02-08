use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::views::{ExternalOwner, ListChamberResponse, SmallChamberView};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetJurisdictionResponse {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub external: Option<ExternalOwner>,
    pub chambers: Vec<ListChamberResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BasicJurisdictionView {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JurisdictionView {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub external: Option<ExternalOwner>,
    pub chambers: Vec<SmallChamberView>,
}

/// Summary view of a session for session pickers
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SessionSummary {
    pub id: i32,
    pub name: String,
    pub current: bool,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
}

/// Summary of a chamber within a jurisdiction
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JurisdictionChamberView {
    pub id: i32,
    pub name: String,
    pub external: Option<ExternalOwner>,
    pub member_count: i32,
}

/// Response for getting jurisdiction details with session support
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetJurisdictionDetailsResponse {
    pub id: i32,
    pub name: String,
    pub external: Option<ExternalOwner>,
    /// All sessions for this jurisdiction (for session picker)
    pub sessions: Vec<SessionSummary>,
    /// The currently selected session (defaults to current session)
    pub current_session: Option<SessionSummary>,
    /// Chambers with member counts for the selected session
    pub chambers: Vec<JurisdictionChamberView>,
}
