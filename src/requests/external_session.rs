use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub struct CreateJurisdictionSession {
    body: ExternalSession,
    jurisdiction_id: i32,
}
impl CreateJurisdictionSession {
    pub fn new(jurisdiction_id: i32, body: ExternalSession) -> Self {
        Self {
            body,
            jurisdiction_id,
        }
    }
}

impl Handler for CreateJurisdictionSession {
    type ResponseBody = GetSessionResponse;
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> std::borrow::Cow<'_, str> {
        format!("/api/jurisdictions/{}/sessions", self.jurisdiction_id).into()
    }
    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExternalSession {
    pub name: String,
    pub external_id: ExternalId,
    pub url: Option<String>,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
    /// The chambers that are part of this session
    pub chambers: Vec<ExternalId>,
}
impl ExternalSession {
    pub fn new(
        external_id: impl Into<ExternalId>,
        name: impl Into<String>,
        starts_at: Option<NaiveDate>,
        ends_at: Option<NaiveDate>,
    ) -> Self {
        Self {
            name: name.into(),
            external_id: external_id.into(),
            url: None,
            chambers: Vec::new(),
            starts_at,
            ends_at,
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
    pub fn with_chamber(mut self, chamber: impl Into<ExternalId>) -> Self {
        self.chambers.push(chamber.into());
        self
    }

    pub fn with_chambers<V>(mut self, chamber: impl IntoIterator<Item = V>) -> Self
    where
        V: Into<ExternalId>,
    {
        self.chambers.extend(chamber.into_iter().map(Into::into));
        self
    }
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
    pub fn external_id(&self) -> &ExternalId {
        &self.external_id
    }
}
