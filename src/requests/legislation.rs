use std::borrow::Cow;

use crate::prelude::*;

impl GetHandler for LegislationParams {
    type ResponseBody = Paginated<DetailedLegislationView>;
    fn path(&self) -> Cow<'_, str> {
        "/api/legislation".into()
    }
    fn params(&self) -> impl SdkParams {
        self.clone()
    }
}

/// Get details for a specific piece of legislation
pub struct GetLegislationDetails(pub i32);

impl GetHandler for GetLegislationDetails {
    type ResponseBody = LegislationDetailsResponse;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}", self.0).into()
    }
}

/// Get votes for a specific piece of legislation
pub struct GetLegislationVotes(pub i32);

impl GetHandler for GetLegislationVotes {
    type ResponseBody = LegislationVotesResponse;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}/votes", self.0).into()
    }
}

/// Get details for a specific vote on legislation
pub struct GetLegislationVoteDetails {
    pub legislation_id: i32,
    pub vote_id: i32,
}

impl GetLegislationVoteDetails {
    pub fn new(legislation_id: i32, vote_id: i32) -> Self {
        Self {
            legislation_id,
            vote_id,
        }
    }
}

impl GetHandler for GetLegislationVoteDetails {
    type ResponseBody = LegislationVoteDetailsResponse;

    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/legislation/{}/votes/{}",
            self.legislation_id, self.vote_id
        )
        .into()
    }
}

use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};

/// Request to create a new piece of legislation
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateLegislationRequest {
    pub name_id: String,
    pub title: String,
    pub summary: String,
    /// When the primary source material was last updated.
    ///
    /// If your API does not provide this data, use `Local::now()`
    pub external_update_at: DateTime<FixedOffset>,
    pub legislation_type: LegislationType,
    pub status_text: String,
    /// When the primary source material was last updated the legislation status.
    ///
    /// If your API does not provide this data, use `Local::now()`
    pub status_updated_at: DateTime<FixedOffset>,
    pub introduced_at: Option<DateTime<FixedOffset>>,
    pub status: Option<LegislationStatus>,
    pub external_metadata: Option<ExternalMetadata>,
}

impl CreateLegislationRequest {
    pub fn new(
        name_id: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        legislation_type: LegislationType,
        status_text: impl Into<String>,
    ) -> Self {
        Self {
            name_id: name_id.into(),
            title: title.into(),
            summary: summary.into(),
            external_update_at: Local::now().into(),
            legislation_type,
            status_text: status_text.into(),
            introduced_at: None,
            status: None,
            status_updated_at: Local::now().into(),
            external_metadata: None,
        }
    }

    pub fn introduced_at(mut self, date: DateTime<FixedOffset>) -> Self {
        self.introduced_at = Some(date);
        self
    }

    pub fn outcome(mut self, outcome: LegislationStatus) -> Self {
        self.status = Some(outcome);
        self
    }

    pub fn status_updated_at(mut self, date: DateTime<FixedOffset>) -> Self {
        self.status_updated_at = date;
        self
    }

    pub fn external_metadata(mut self, metadata: ExternalMetadata) -> Self {
        self.external_metadata = Some(metadata);
        self
    }
}

/// Handler for creating legislation
pub struct CreateLegislation {
    chamber_id: i32,
    session_id: i32,
    body: CreateLegislationRequest,
}

impl CreateLegislation {
    pub fn new(chamber_id: i32, session_id: i32, body: CreateLegislationRequest) -> Self {
        Self {
            chamber_id,
            session_id,
            body,
        }
    }
}

impl Handler for CreateLegislation {
    type ResponseBody = LegislationView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/sessions/{}/chambers/{}/legislation",
            self.session_id, self.chamber_id
        )
        .into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}
/// Request to update an existing piece of legislation
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateLegislationRequest {
    /// If some, the name_id is updated. If none, it remains unchanged
    pub name_id: Option<String>,
    /// If some, the title is updated. If none, it remains unchanged
    pub title: Option<String>,
    /// If some, the summary is updated. If none, it remains unchanged
    pub summary: Option<String>,
    /// If some, the legislation_type is updated. If none, it remains unchanged
    pub legislation_type: Option<LegislationType>,
    /// When the external update occurred. this is a user editable field, so
    /// if an external api provides and updated_date, that should be this field.
    ///
    /// Otherwise, this is None.
    pub external_update_at: Option<DateTime<FixedOffset>>,
    /// If some, the status is updated. If none, the status is unchanged
    pub status: Option<String>,

    pub introduced_at_set: bool,
    /// Only applied if `introduced_at_set` is true.
    pub introduced_at: Option<DateTime<FixedOffset>>,
    pub outcome_set: bool,
    /// Only applied if `outcome_set` is true.
    pub outcome: Option<LegislationStatus>,
    pub status_updated_set: bool,
    /// Only applied if `status_updated_set` is true.
    pub status_updated_at: Option<DateTime<FixedOffset>>,
}

impl UpdateLegislationRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name_id(mut self, name_id: impl Into<String>) -> Self {
        self.name_id = Some(name_id.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn legislation_type(mut self, legislation_type: LegislationType) -> Self {
        self.legislation_type = Some(legislation_type);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn introduced_at(mut self, date: DateTime<FixedOffset>) -> Self {
        self.introduced_at = Some(date);
        self
    }

    pub fn outcome(mut self, outcome: LegislationStatus) -> Self {
        self.outcome = Some(outcome);
        self
    }
}

/// Request to add a sponsor to legislation
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AddSponsorRequest {
    pub member_id: i32,
    pub sponsor_type: i32,
}

impl AddSponsorRequest {
    pub fn new(member_id: i32, sponsor_type: i32) -> Self {
        Self {
            member_id,
            sponsor_type,
        }
    }
}

/// Handler for updating legislation
pub struct UpdateLegislation {
    id: i32,
    body: UpdateLegislationRequest,
}

impl UpdateLegislation {
    pub fn new(id: i32, body: UpdateLegislationRequest) -> Self {
        Self { id, body }
    }
}

impl Handler for UpdateLegislation {
    type ResponseBody = LegislationView;

    fn method(&self) -> Method {
        Method::Patch
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}", self.id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler for adding a sponsor
pub struct AddSponsor {
    legislation_id: i32,
    body: AddSponsorRequest,
}

impl AddSponsor {
    pub fn new(legislation_id: i32, body: AddSponsorRequest) -> Self {
        Self {
            legislation_id,
            body,
        }
    }
}

impl Handler for AddSponsor {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}/sponsors", self.legislation_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

#[test]
fn test_query_params_behavior() {
    use pretty_assertions::assert_eq;
    let list_session_legislation = ListSessionLegislation::new(3);

    // ListSessionLegislation returns default pagination params
    let params = Handler::params(&list_session_legislation)
        .into_params()
        .unwrap();
    assert_eq!("order_by=id&order=desc&page=1&page_size=10", &params);
    let params = LegislationParams {
        page: Some(2),
        page_size: Some(13),
        id: vec![2, 4, 3],
        order: Ordering::Desc,
        order_by: LegislationOrder::ExternalId,
        ..Default::default()
    };
    let ser_params = serde_qs::to_string(&params).unwrap();

    assert_eq!(
        "id[0]=2&id[1]=4&id[2]=3&order_by=external_id&order=desc&page=2&page_size=13",
        &ser_params
    );

    let de_params: LegislationParams = serde_qs::from_str(&ser_params).unwrap();

    assert_eq!(params, de_params);
}
