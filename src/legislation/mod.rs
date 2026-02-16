mod summaries;
pub use summaries::*;

use crate::prelude::*;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;
use strum::{Display, EnumString};
use url::Url;

use crate::{paginated, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct LegislationParams {
    #[serde(default)]
    pub id: Vec<i32>,

    pub freetext: Option<String>,
    pub legislation_type: Option<String>,

    #[serde(default)]
    pub external_id: Vec<ExternalId>,
    pub session_id: Option<i32>,
    /// Filter by active status (derived from outcome)
    pub is_active: Option<bool>,
    /// Filter by specific outcomes
    #[serde(default)]
    pub outcome: Vec<String>,
    /// id | external_id
    #[serde(default)]
    pub order_by: LegislationOrder,
    /// asc | desc
    #[serde(default)]
    pub order: Ordering,

    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(LegislationParams);

impl LegislationParams {
    pub fn set_ids(mut self, ids: impl IntoIterator<Item = i32>) -> Self {
        self.id = ids.into_iter().collect();
        self
    }
    pub fn set_external_ids(mut self, external_ids: impl IntoIterator<Item = ExternalId>) -> Self {
        self.external_id = external_ids.into_iter().collect();
        self
    }
    pub fn set_order_by(mut self, order: LegislationOrder) -> Self {
        self.order_by = order;
        self
    }
    pub fn set_order(mut self, order: Ordering) -> Self {
        self.order = order;
        self
    }

    pub fn legislation_type(&self) -> Option<LegislationType> {
        self.legislation_type
            .as_ref()
            .and_then(|t| LegislationType::from_str(t).ok())
    }

    pub fn set_is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn set_outcomes(mut self, outcomes: impl IntoIterator<Item = LegislationStatus>) -> Self {
        self.outcome = outcomes.into_iter().map(|o| o.to_string()).collect();
        self
    }

    pub fn outcomes(&self) -> Vec<LegislationStatus> {
        self.outcome
            .iter()
            .filter_map(|o| LegislationStatus::from_str(o).ok())
            .collect()
    }
}

/// How the legislation should be ordered
#[derive(
    Serialize, Deserialize, Default, Clone, EnumString, Display, Debug, PartialEq, Eq, Copy,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LegislationOrder {
    #[default]
    Id,
    ExternalId,
}

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

/// Request to create a new piece of legislation
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateLegislationRequest {
    pub name_id: String,
    pub title: String,
    pub summary: Option<CreateSummaryRequest>,
    /// When the primary source material was last updated.
    ///
    /// If your API does not provide this data, use `Local::now()`
    pub legislation_type: LegislationType,
    pub status: Option<LegislationStatus>,
    pub status_text: String,
    /// When the primary source material was last updated the legislation status.
    ///
    /// If your API does not provide this data, use `Local::now()`
    pub status_updated_at: DateTime<FixedOffset>,
    pub introduced_at: Option<DateTime<FixedOffset>>,
    pub external_metadata: Option<ExternalMetadata>,
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
    /// If some, the legislation_type is updated. If none, it remains unchanged
    pub legislation_type: Option<LegislationType>,

    pub url_set: bool,
    pub url: Option<Url>,

    /// If some, the status_updated time is updated. If none, it remains unchanged.
    pub status_updated_at: Option<DateTime<FixedOffset>>,

    /// When the external update occurred. this is a user editable field, so
    /// if an external api provides and updated_date, that should be this field.
    ///
    /// Otherwise, this is None.
    pub external_update_at: Option<DateTime<FixedOffset>>,

    /// If some, the status is updated. If none, the status is unchanged
    pub status_text: Option<String>,

    pub introduced_at_set: bool,
    /// Only applied if `introduced_at_set` is true.
    pub introduced_at: Option<DateTime<FixedOffset>>,

    pub status_set: bool,
    /// Only applied if `status_set` is true.
    pub status: Option<LegislationStatus>,
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

/// Request to replace all sponsors on legislation
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PutSponsorsRequest {
    pub sponsors: Vec<SponsorInput>,
}

/// A single sponsor entry for the PUT request
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SponsorInput {
    pub member_id: i32,
    pub sponsor_type: SponsorshipType,
    pub sponsored_at: Option<DateTime<FixedOffset>>,
}

/// Handler for replacing all sponsors on legislation
pub struct PutSponsors {
    legislation_id: i32,
    body: PutSponsorsRequest,
}

impl PutSponsors {
    pub fn new(legislation_id: i32, body: PutSponsorsRequest) -> Self {
        Self {
            legislation_id,
            body,
        }
    }
}

impl Handler for PutSponsors {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Put
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
