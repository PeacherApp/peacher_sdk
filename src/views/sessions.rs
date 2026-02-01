use crate::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A session view with jurisdiction and chamber details
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetSessionResponse {
    pub id: i32,
    pub name: String,
    pub current: bool,
    pub external: Option<ExternalOwner>,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
    pub jurisdiction: BasicJurisdictionView,
    pub chambers: Vec<ChamberSessionView>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SessionView {
    pub id: i32,
    pub name: String,
    pub current: bool,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
    pub jurisdiction_id: i32,
}
