use std::borrow::Cow;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{paginated, prelude::*};

/// Parameters for listing sessions
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct SessionParams {
    /// Filter to only current sessions
    pub current: Option<bool>,
    /// Filter by jurisdiction ID
    pub jurisdiction_id: Option<i32>,
    /// Filter by external ID
    pub external_id: Option<ExternalId>,
    /// Sort order: "name", "recent", "oldest"
    pub sort: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(SessionParams);

impl SessionParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_current(mut self, current: bool) -> Self {
        self.current = Some(current);
        self
    }

    pub fn with_jurisdiction(mut self, jurisdiction_id: i32) -> Self {
        self.jurisdiction_id = Some(jurisdiction_id);
        self
    }

    pub fn with_external_id(mut self, external_id: impl Into<ExternalId>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn with_sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }
}

/// Get a session by ID
pub struct GetSession(pub i32);

impl GetHandler for GetSession {
    type ResponseBody = GetSessionView;
    fn path(&self) -> Cow<'_, str> {
        format!("/api/sessions/{}", self.0).into()
    }
}

/// List all sessions with pagination and filtering
#[derive(Default)]
pub struct ListSessions(pub SessionParams);

impl GetHandler for ListSessions {
    type ResponseBody = Paginated<GetSessionView>;
    fn path(&self) -> Cow<'_, str> {
        "/api/sessions".into()
    }
    fn params(&self) -> impl SdkParams {
        self.0.clone()
    }
}

/// Get members of a chamber for a specific session
pub struct GetSessionChamber {
    session_id: i32,
    chamber_id: i32,
}

impl GetSessionChamber {
    pub fn new(chamber_id: i32, session_id: i32) -> Self {
        Self {
            session_id,
            chamber_id,
        }
    }
}

pub struct DeleteSession(pub i32);

impl Handler for DeleteSession {
    type ResponseBody = NoResponse;
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> Cow<'_, str> {
        format!("/api/sessions/{}", self.0).into()
    }
}

impl GetHandler for GetSessionChamber {
    type ResponseBody = GetSessionChamberResponse;
    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/sessions/{}/chambers/{}",
            self.session_id, self.chamber_id
        )
        .into()
    }
}

/// Get member activity for a chamber in a specific session
pub struct GetChamberSessionActivity {
    session_id: i32,
    chamber_id: i32,
}

impl GetChamberSessionActivity {
    pub fn new(session_id: i32, chamber_id: i32) -> Self {
        Self {
            session_id,
            chamber_id,
        }
    }
}

impl GetHandler for GetChamberSessionActivity {
    type ResponseBody = Vec<ChamberMemberActivityView>;
    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/sessions/{}/chambers/{}/activity",
            self.session_id, self.chamber_id
        )
        .into()
    }
}

/// Get the map for a chamber in a specific session
pub struct GetChamberSessionMap {
    session_id: i32,
    chamber_id: i32,
}

impl GetChamberSessionMap {
    pub fn new(session_id: i32, chamber_id: i32) -> Self {
        Self {
            session_id,
            chamber_id,
        }
    }
}

impl GetHandler for GetChamberSessionMap {
    // Use serde_json::Value for flexibility with GeoJson types
    type ResponseBody = serde_json::Value;
    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/sessions/{}/chambers/{}/map",
            self.session_id, self.chamber_id
        )
        .into()
    }
}

pub struct ListSessionLegislation {
    id: i32,
    page: u64,
    page_size: u64,
}
impl ListSessionLegislation {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            page: 0,
            page_size: 10,
        }
    }
    /// Note that pages start at 0
    pub fn page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }
    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = page_size;
        self
    }
}

impl GetHandler for ListSessionLegislation {
    type ResponseBody = Paginated<DetailedLegislationView>;
    fn path(&self) -> std::borrow::Cow<'_, str> {
        format!("/api/sessions/{}/legislation", self.id).into()
    }
    fn params(&self) -> impl SdkParams {
        LegislationParams {
            page: Some(self.page),
            page_size: Some(self.page_size),
            ..Default::default()
        }
    }
}

/// Request to create a new session
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSessionRequest {
    pub name: String,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
}

impl CreateSessionRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            starts_at: None,
            ends_at: None,
            external_id: None,
            external_url: None,
        }
    }

    pub fn starts_at(mut self, date: NaiveDate) -> Self {
        self.starts_at = Some(date);
        self
    }

    pub fn ends_at(mut self, date: NaiveDate) -> Self {
        self.ends_at = Some(date);
        self
    }

    pub fn external_id(mut self, id: impl Into<ExternalId>) -> Self {
        self.external_id = Some(id.into());
        self
    }

    pub fn external_url(mut self, url: Url) -> Self {
        self.external_url = Some(url);
        self
    }
}

/// Request to update an existing session
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateSessionRequest {
    pub name: Option<String>,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
}

impl UpdateSessionRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn starts_at(mut self, date: NaiveDate) -> Self {
        self.starts_at = Some(date);
        self
    }

    pub fn ends_at(mut self, date: NaiveDate) -> Self {
        self.ends_at = Some(date);
        self
    }
}

/// Request to link a chamber to a session with an optional map id
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LinkSessionToChamberRequest {
    pub chamber_id: i32,
    pub map_id: Option<i32>,
}

impl LinkSessionToChamberRequest {
    pub fn new(chamber_id: i32) -> Self {
        Self {
            chamber_id,
            map_id: None,
        }
    }

    pub fn map_id(mut self, map_id: i32) -> Self {
        self.map_id = Some(map_id);
        self
    }
}

/// Handler for creating a session
pub struct CreateSession {
    jurisdiction_id: i32,
    body: CreateSessionRequest,
}

impl CreateSession {
    pub fn new(jurisdiction_id: i32, body: CreateSessionRequest) -> Self {
        Self {
            jurisdiction_id,
            body,
        }
    }
}

impl Handler for CreateSession {
    type ResponseBody = GetSessionView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/jurisdictions/{}/sessions", self.jurisdiction_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler for updating a session
pub struct UpdateSession {
    id: i32,
    body: UpdateSessionRequest,
}

impl UpdateSession {
    pub fn new(id: i32, body: UpdateSessionRequest) -> Self {
        Self { id, body }
    }
}

impl Handler for UpdateSession {
    type ResponseBody = GetSessionView;

    fn method(&self) -> Method {
        Method::Patch
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/sessions/{}", self.id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler for linking a chamber to a session
pub struct LinkChamberToSession {
    session_id: i32,
    body: LinkSessionToChamberRequest,
}

impl LinkChamberToSession {
    pub fn new(session_id: i32, body: LinkSessionToChamberRequest) -> Self {
        Self { session_id, body }
    }
}

impl Handler for LinkChamberToSession {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/sessions/{}/chambers", self.session_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// A session view with jurisdiction and chamber details
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetSessionView {
    pub id: i32,
    pub name: String,
    pub current: bool,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
    pub jurisdiction: JurisdictionView,
    pub chambers: Vec<ChamberView>,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
}

/// A chamber within a session
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetSessionChamberResponse {
    // the session has a top level jurisdiction view
    pub chamber: GetChamberView,
    pub session: SessionView,
    pub members: Vec<ChamberSessionMember>,
}

// /// A chamber within a session
// #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
// #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
// pub struct ChamberSession {
//     // the session has a top level jurisdiction view
//     pub chamber: ChamberView,
//     pub session: SessionView,
//     pub members: Vec<ChamberSessionMember>,
// }

/// A member within a chamber session
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberSessionMember {
    pub member: MemberWithPartyView,
    pub district_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SessionView {
    pub id: i32,
    pub name: String,
    pub current: bool,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
    pub jurisdiction_id: i32,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
    pub created_by_id: Option<i32>,
}
impl SessionView {
    pub fn into_get_session_view(
        self,
        jurisdiction: JurisdictionView,
        chambers: impl IntoIterator<Item = ChamberView>,
    ) -> GetSessionView {
        debug_assert_eq!(self.jurisdiction_id, jurisdiction.id);
        GetSessionView {
            id: self.id,
            name: self.name,
            current: self.current,
            external_id: self.external_id,
            external_url: self.external_url,
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            created_by_id: self.created_by_id,
            jurisdiction,
            chambers: chambers.into_iter().collect(),
        }
    }
}
