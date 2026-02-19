use std::borrow::Cow;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::{paginated, prelude::*};

/// Parameters for listing jurisdictions
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct JurisdictionParams {
    /// Filter by external ID
    pub external_id: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(JurisdictionParams);

impl JurisdictionParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }
}

/// Query parameters for jurisdiction details (session selection)
#[derive(Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct JurisdictionDetailsParams {
    /// Session ID - defaults to current session if not provided
    pub session: Option<i32>,
}

/// List jurisdictions with optional filters
pub struct ListJurisdictions {
    page: u64,
    page_size: u64,
    external_id: Option<String>,
}

impl ListJurisdictions {
    pub fn new() -> Self {
        Self {
            page: 0,
            page_size: 20,
            external_id: None,
        }
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }
}

impl Default for ListJurisdictions {
    fn default() -> Self {
        Self::new()
    }
}

impl GetHandler for ListJurisdictions {
    type ResponseBody = Paginated<GetJurisdictionResponse>;

    fn path(&self) -> Cow<'_, str> {
        "/api/jurisdictions".into()
    }

    fn params(&self) -> impl SdkParams {
        #[derive(Serialize)]
        struct Params {
            page: u64,
            page_size: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            external_id: Option<String>,
        }
        Params {
            page: self.page,
            page_size: self.page_size,
            external_id: self.external_id.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateJurisdiction {
    pub name: String,
    pub external_metadata: Option<ExternalMetadata>,
}

impl CreateJurisdiction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            external_metadata: None,
        }
    }
    pub fn external_metadata(mut self, metadata: ExternalMetadata) -> Self {
        self.external_metadata = Some(metadata);
        self
    }
}

impl Handler for CreateJurisdiction {
    type ResponseBody = GetJurisdictionResponse;

    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> Cow<'_, str> {
        "/api/jurisdictions".into()
    }
    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(self)
    }
}

pub struct GetAccountJurisdictions;
impl GetHandler for GetAccountJurisdictions {
    type ResponseBody = Vec<JurisdictionView>;
    fn path(&self) -> Cow<'_, str> {
        "/api/account/jurisdictions".into()
    }
}

// #[derive(Serialize, Deserialize)]
// #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
// pub struct CreateSession {
//     pub name: String,
//     pub current: bool,
//     pub external_id: ExternalId,
//     pub url: Option<String>,
// }

// pub struct CreateSessionMember {
//     chamber_id: i32,
//     session_id: i32,
//     member: ExternalMember,
// }
// impl CreateSessionMember {
//     pub fn new(chamber_id: i32, session_id: i32, member: ExternalMember) -> Self {
//         Self {
//             chamber_id,
//             session_id,
//             member,
//         }
//     }
// }

// impl Handler for CreateSessionMember {
//     type ResponseBody = ChamberSessionMember;
//     fn method(&self) -> Method {
//         Method::Post
//     }
//     fn path(&self) -> Cow<'_, str> {
//         format!(
//             "/chamber/{}/session/{}/members",
//             self.chamber_id, self.session_id
//         )
//         .into()
//     }

//     fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
//         builder.json(&self.member)
//     }
// }

/// Get a jurisdiction by ID
pub struct GetJurisdiction(pub i32);

impl GetHandler for GetJurisdiction {
    type ResponseBody = GetJurisdictionResponse;
    fn path(&self) -> Cow<'_, str> {
        format!("/api/jurisdictions/{}", self.0).into()
    }
}

/// Get jurisdiction details with session-aware data
pub struct GetJurisdictionDetails {
    pub id: i32,
    pub session: Option<i32>,
}

impl GetJurisdictionDetails {
    pub fn new(id: i32) -> Self {
        Self { id, session: None }
    }

    pub fn with_session(mut self, session_id: i32) -> Self {
        self.session = Some(session_id);
        self
    }
}

impl GetHandler for GetJurisdictionDetails {
    type ResponseBody = GetJurisdictionDetailsResponse;
    fn path(&self) -> Cow<'_, str> {
        format!("/api/jurisdictions/{}/details", self.id).into()
    }
    fn params(&self) -> impl SdkParams {
        #[derive(Serialize)]
        struct Params {
            #[serde(skip_serializing_if = "Option::is_none")]
            session: Option<i32>,
        }
        Params {
            session: self.session,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JurisdictionChamber {
    pub name: String,
    pub external_id: ExternalId,
    pub url: Option<String>,
}

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
