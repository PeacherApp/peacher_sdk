use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{paginated, prelude::*};

/// Parameters for listing jurisdictions
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct JurisdictionParams {
    /// Filter by external ID
    pub external_id: Option<ExternalId>,
    pub created_by_id: Option<i32>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(JurisdictionParams);

impl JurisdictionParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_external_id(mut self, external_id: impl Into<ExternalId>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }
}

/// List jurisdictions with optional filters
#[derive(Default)]
pub struct ListJurisdictions(pub JurisdictionParams);

impl GetHandler for ListJurisdictions {
    type ResponseBody = Paginated<GetJurisdictionView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/jurisdictions".into()
    }

    fn params(&self) -> impl SdkParams {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateJurisdiction {
    pub name: String,
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
}

impl CreateJurisdiction {
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

impl Handler for CreateJurisdiction {
    type ResponseBody = JurisdictionView;

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

/// Get a jurisdiction by ID
pub struct GetJurisdiction(pub i32);

impl GetHandler for GetJurisdiction {
    type ResponseBody = GetJurisdictionView;
    fn path(&self) -> Cow<'_, str> {
        format!("/api/jurisdictions/{}", self.0).into()
    }
}

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
    pub fn into_get_jurisdiction_view(
        self,
        current_session: Option<SessionView>,
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
            current_session,
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
    pub current_session: Option<SessionView>,
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
