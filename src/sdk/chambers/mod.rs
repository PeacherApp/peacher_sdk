mod views;
pub use views::*;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{paginated, prelude::*};

use std::borrow::Cow;

use chrono::NaiveDate;

/// Parameters for listing chambers
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ChamberParams {
    /// Filter by external ID
    pub external_id: Option<ExternalId>,
    /// Filter by jurisdiction ID
    pub jurisdiction_id: Option<i32>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(ChamberParams);

impl ChamberParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_external_id(mut self, external_id: impl Into<ExternalId>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn with_jurisdiction(mut self, jurisdiction_id: i32) -> Self {
        self.jurisdiction_id = Some(jurisdiction_id);
        self
    }
}
/// Query parameters for chamber details (session selection)
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ChamberDetailsParams {
    /// Session ID - defaults to current session if not provided
    pub session: Option<i32>,
}

/// List chambers with optional filters
pub struct ListChambers {
    page: u64,
    page_size: u64,
    external_id: Option<ExternalId>,
    jurisdiction_id: Option<i32>,
}

impl ListChambers {
    pub fn new() -> Self {
        Self {
            page: 0,
            page_size: 20,
            external_id: None,
            jurisdiction_id: None,
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

    pub fn with_external_id(mut self, external_id: impl Into<ExternalId>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn with_jurisdiction(mut self, jurisdiction_id: i32) -> Self {
        self.jurisdiction_id = Some(jurisdiction_id);
        self
    }
}

impl Default for ListChambers {
    fn default() -> Self {
        Self::new()
    }
}

impl GetHandler for ListChambers {
    type ResponseBody = Paginated<GetChamberView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/chambers".into()
    }

    fn params(&self) -> impl SdkParams {
        #[derive(Serialize)]
        struct Params {
            page: u64,
            page_size: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            external_id: Option<ExternalId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            jurisdiction_id: Option<i32>,
        }
        Params {
            page: self.page,
            page_size: self.page_size,
            external_id: self.external_id.clone(),
            jurisdiction_id: self.jurisdiction_id,
        }
    }
}

/// Get a chamber by ID
pub struct GetChamber(pub i32);

impl GetHandler for GetChamber {
    type ResponseBody = GetChamberView;
    fn path(&self) -> Cow<'_, str> {
        format!("/api/chambers/{}", self.0).into()
    }
}

/// Request to create a new chamber
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateChamberRequest {
    pub name: String,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
}

impl CreateChamberRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            external_id: None,
            external_url: None,
        }
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

pub struct CreateChamber {
    pub jurisdiction_id: i32,
    pub request: CreateChamberRequest,
}
impl CreateChamber {
    pub fn new(jurisdiction_id: i32, chamber_name: impl Into<String>) -> Self {
        Self {
            jurisdiction_id,
            request: CreateChamberRequest::new(chamber_name),
        }
    }
    pub fn external_id(mut self, id: impl Into<ExternalId>) -> Self {
        self.request.external_id = Some(id.into());
        self
    }
    pub fn external_url(mut self, url: Url) -> Self {
        self.request.external_url = Some(url);
        self
    }
}

impl Handler for CreateChamber {
    type ResponseBody = ChamberView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/jurisdictions/{}/chambers", self.jurisdiction_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.request)
    }
}

/// Request to update an existing chamber
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateChamberRequest {
    pub name: Option<String>,
}

impl UpdateChamberRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Request to link a member to a chamber in a session context
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LinkMemberToChamberRequest {
    pub member_id: i32,
    pub district_id: Option<i32>,
    pub appointed_at_set: bool,
    pub appointed_at: Option<NaiveDate>,
    pub expunged_at_set: bool,
    pub expunged_at: Option<NaiveDate>,
}

/// Handler for updating a chamber
pub struct UpdateChamber {
    id: i32,
    body: UpdateChamberRequest,
}

impl UpdateChamber {
    pub fn new(id: i32, body: UpdateChamberRequest) -> Self {
        Self { id, body }
    }
}

impl Handler for UpdateChamber {
    type ResponseBody = ChamberView;

    fn method(&self) -> Method {
        Method::Patch
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/chambers/{}", self.id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler for linking a member to a chamber
pub struct LinkMemberToChamber {
    chamber_id: i32,
    session_id: i32,
    request: LinkMemberToChamberRequest,
}

impl LinkMemberToChamber {
    pub fn new(chamber_id: i32, session_id: i32, member_id: i32) -> Self {
        Self {
            chamber_id,
            session_id,
            request: LinkMemberToChamberRequest {
                member_id,
                district_id: None,
                appointed_at_set: false,
                appointed_at: None,
                expunged_at_set: false,
                expunged_at: None,
            },
        }
    }
    pub fn appointed_at(mut self, appointed_at: Option<NaiveDate>) -> Self {
        self.request.appointed_at_set = true;
        self.request.appointed_at = appointed_at;
        self
    }
    pub fn expunged_at(mut self, expunged_at: Option<NaiveDate>) -> Self {
        self.request.expunged_at_set = true;
        self.request.expunged_at = expunged_at;
        self
    }
    pub fn district(mut self, district_id: Option<i32>) -> Self {
        self.request.district_id = district_id;
        self
    }
    pub fn set_district(&mut self, district_id: Option<i32>) {
        self.request.district_id = district_id;
    }
}

impl Handler for LinkMemberToChamber {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/sessions/{}/chambers/{}/members",
            self.session_id, self.chamber_id
        )
        .into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.request)
    }
}

/// Request to vacate a member from a chamber in a session context
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct VacateMemberRequest {
    pub date_expunged: NaiveDate,
}

/// Handler for vacating a member from a chamber
pub struct VacateMemberFromChamber {
    chamber_id: i32,
    session_id: i32,
    member_id: i32,
    date: Option<NaiveDate>,
}

impl VacateMemberFromChamber {
    pub fn new(chamber_id: i32, session_id: i32, member_id: i32) -> Self {
        Self {
            chamber_id,
            session_id,
            member_id,
            date: None,
        }
    }
    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }
    pub fn set_date(&mut self, date: Option<NaiveDate>) {
        self.date = date;
    }
}

impl Handler for VacateMemberFromChamber {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/sessions/{}/chambers/{}/members/{}",
            self.session_id, self.chamber_id, self.member_id
        )
        .into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        let request_body = self.date.map(|d| VacateMemberRequest { date_expunged: d });
        builder.json(&request_body)
    }
}
