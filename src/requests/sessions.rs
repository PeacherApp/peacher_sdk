use std::borrow::Cow;

use crate::prelude::*;

/// Get a session by ID
pub struct GetSession(pub i32);

impl GetHandler for GetSession {
    type ResponseBody = GetSessionResponse;
    fn path(&self) -> Cow<'_, str> {
        format!("/api/sessions/{}", self.0).into()
    }
}

/// List all sessions with pagination and filtering
#[derive(Default)]
pub struct ListSessions(pub SessionParams);

impl GetHandler for ListSessions {
    type ResponseBody = Paginated<GetSessionResponse>;
    fn path(&self) -> Cow<'_, str> {
        "/api/sessions".into()
    }
    fn params(&self) -> impl SdkParams {
        self.0.clone()
    }
}

/// Get members of a chamber for a specific session
pub struct GetChamberSession {
    session_id: i32,
    chamber_id: i32,
}

impl GetChamberSession {
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

impl GetHandler for GetChamberSession {
    type ResponseBody = ChamberSessionView;
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
            page: 1,
            page_size: 10,
        }
    }
    /// Note that pages start at 1
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

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Request to create a new session
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSessionRequest {
    pub name: String,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
    pub external_metadata: Option<ExternalMetadata>,
}

impl CreateSessionRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            starts_at: None,
            ends_at: None,
            external_metadata: None,
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

    pub fn external_metadata(mut self, metadata: ExternalMetadata) -> Self {
        self.external_metadata = Some(metadata);
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
    type ResponseBody = GetSessionResponse;

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
    type ResponseBody = GetSessionResponse;

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
