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

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// Request to create a new piece of legislation
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateLegislationRequest {
    pub name_id: String,
    pub title: String,
    pub summary: String,
    pub legislation_type: LegislationType,
    pub status: String,
    pub introduced_at: Option<DateTime<FixedOffset>>,
    pub outcome: Option<LegislationOutcome>,
    pub resolved_at: Option<DateTime<FixedOffset>>,
    pub external_metadata: Option<ExternalMetadata>,
}

impl CreateLegislationRequest {
    pub fn new(
        name_id: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        legislation_type: LegislationType,
        status: impl Into<String>,
    ) -> Self {
        Self {
            name_id: name_id.into(),
            title: title.into(),
            summary: summary.into(),
            legislation_type,
            status: status.into(),
            introduced_at: None,
            outcome: None,
            resolved_at: None,
            external_metadata: None,
        }
    }

    pub fn introduced_at(mut self, date: DateTime<FixedOffset>) -> Self {
        self.introduced_at = Some(date);
        self
    }

    pub fn outcome(mut self, outcome: LegislationOutcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    pub fn resolved_at(mut self, date: DateTime<FixedOffset>) -> Self {
        self.resolved_at = Some(date);
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
    pub name_id: Option<String>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub legislation_type: Option<LegislationType>,
    pub status: Option<String>,
    pub introduced_at: Option<DateTime<FixedOffset>>,
    pub outcome: Option<LegislationOutcome>,
    pub resolved_at: Option<DateTime<FixedOffset>>,
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

    pub fn outcome(mut self, outcome: LegislationOutcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    pub fn resolved_at(mut self, date: DateTime<FixedOffset>) -> Self {
        self.resolved_at = Some(date);
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
