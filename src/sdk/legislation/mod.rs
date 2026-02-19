mod summaries;
pub use summaries::*;

use crate::{paginated, prelude::*};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;
use strum::{Display, EnumString};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct LegislationParams {
    #[serde(default)]
    pub id: Vec<i32>,

    pub freetext: Option<String>,
    pub legislation_type: Option<LegislationType>,

    #[serde(default)]
    pub external_id: Vec<ExternalId>,
    pub session_id: Option<i32>,

    pub introduced_after: Option<DateTime<FixedOffset>>,
    pub introduced_before: Option<DateTime<FixedOffset>>,
    /// include or exclude null introduced at legislation
    pub introduced_at_null: bool,

    pub status_text: Option<String>,
    pub status: Option<String>,
    pub status_updated_after: Option<DateTime<FixedOffset>>,
    pub status_updated_before: Option<DateTime<FixedOffset>>,

    pub created_after: Option<DateTime<FixedOffset>>,
    pub created_before: Option<DateTime<FixedOffset>>,

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

    pub fn set_is_active(mut self) -> Self {
        todo!()
        // self.is_active = Some(is_active);
        // self
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

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString, Default, PartialEq, Eq,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum LegislationType {
    Resolution,
    Bill,
    #[default]
    Other,
}

/// Outcome of legislation - tracks what ultimately happened to a bill
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum LegislationStatus {
    ///Still in progress
    #[default]
    Pending,
    /// Passed legislature, awaiting executive action
    Passed,
    /// Did not pass (voted down, died in committee, etc.)
    Failed,
    /// Signed into law by executive
    Signed,
    /// Vetoed by executive
    Vetoed,
    /// Veto overridden by legislature
    VetoOverridden,

    /// Sponsor withdrew the legislation
    Withdrawn,
}

impl LegislationStatus {
    /// Returns true if this outcome represents an active/in-progress state
    pub fn is_active(&self) -> bool {
        matches!(self, LegislationStatus::Pending)
    }

    /// Returns true if this outcome represents a terminal state
    pub fn is_terminal(&self) -> bool {
        !self.is_active()
    }

    /// Parse from optional string, returning None for null values
    pub fn from_opt_str(s: Option<&str>) -> Option<Self> {
        s.and_then(|s| Self::from_str(s).ok())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationView {
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub introduced_at: Option<DateTime<FixedOffset>>,
    pub external_update_at: Option<DateTime<FixedOffset>>,
    pub legislation_type: LegislationType,
    pub id: i32,
    pub name_id: String,
    pub title: String,
    pub summary: Option<serde_json::Value>,
    /// Current outcome of the legislation
    pub status: Option<LegislationStatus>,
    /// Human-readable status text from external source
    pub status_text: String,
    pub status_updated_at: DateTime<FixedOffset>,
    pub external: Option<ExternalOwner>,
}

impl LegislationView {
    pub fn into_detailed(
        self,
        votes: impl IntoIterator<Item = LegislationViewVote>,
        sponsors: impl IntoIterator<Item = LegislationViewSponsor>,
    ) -> DetailedLegislationView {
        DetailedLegislationView {
            created_at: self.created_at,
            updated_at: self.updated_at,
            introduced_at: self.introduced_at,
            external_update_at: self.external_update_at,
            id: self.id,
            name_id: self.name_id,
            title: self.title,
            summary: self.summary,
            status_updated_at: self.status_updated_at,
            legislation_type: self.legislation_type,
            status: self.status,
            status_text: self.status_text,
            external: self.external,
            votes: votes.into_iter().collect(),
            sponsors: sponsors.into_iter().collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DetailedLegislationView {
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub introduced_at: Option<DateTime<FixedOffset>>,
    pub id: i32,
    pub name_id: String,
    pub title: String,
    pub summary: Option<serde_json::Value>,
    pub external_update_at: Option<DateTime<FixedOffset>>,
    pub legislation_type: LegislationType,
    /// Current outcome of the legislation
    pub status: Option<LegislationStatus>,
    /// Human-readable status text from external source
    pub status_text: String,
    pub status_updated_at: DateTime<FixedOffset>,
    pub external: Option<ExternalOwner>,
    pub votes: Vec<LegislationViewVote>,
    pub sponsors: Vec<LegislationViewSponsor>,
}

impl DetailedLegislationView {
    pub fn into_legislation_view(self) -> LegislationView {
        LegislationView {
            created_at: self.created_at,
            updated_at: self.updated_at,
            introduced_at: self.introduced_at,
            external_update_at: self.external_update_at,
            id: self.id,
            name_id: self.name_id,
            title: self.title,
            summary: self.summary,
            legislation_type: self.legislation_type,
            status_updated_at: self.status_updated_at,
            status: self.status,
            status_text: self.status_text,
            external: self.external,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationViewVote {
    pub id: i32,
    pub name: String,
    pub occurred_at: Option<DateTime<FixedOffset>>,
    pub chamber_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationViewSponsor {
    pub id: i32,
    pub member_id: i32,
    pub sponsor_type: i32,
    pub sponsored_at: Option<DateTime<FixedOffset>>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationDetailsResponse {
    pub id: i32,
    pub name_id: String,
    pub title: String,
    pub external_update_at: Option<DateTime<FixedOffset>>,
    pub status_text: String,
    pub status_updated_at: DateTime<FixedOffset>,
    pub status: Option<LegislationStatus>,
    pub legislation_type: LegislationType,
    pub external: Option<ExternalOwner>,
    pub sponsors: Vec<SponsorInfo>,
    pub chamber: Option<ChamberView>,
    pub session: Option<SessionView>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SponsorInfo {
    pub member: MemberView,
    pub sponsor_type: SponsorshipType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationVotesResponse {
    pub legislation: LegislationView,
    pub votes: Vec<LegislationVoteView>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationVoteView {
    pub id: i32,
    pub name: String,
    pub occurred_at: Option<DateTime<FixedOffset>>,
    pub chamber: ChamberRef,
    pub yes_count: i32,
    pub no_count: i32,
    pub absent_count: i32,
    pub not_voting_count: i32,
    pub total_members: i32,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct VoteSummary {
    pub yes_count: i32,
    pub no_count: i32,
    pub absent_count: i32,
    pub not_voting_count: i32,
    pub total: i32,
    pub passed: bool,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationVoteDetailsResponse {
    pub vote_id: i32,
    pub vote_name: String,
    pub occurred_at: Option<DateTime<FixedOffset>>,
    pub chamber: ChamberRef,
    pub legislation: LegislationView,
    pub member_votes: Vec<MemberVoteValue>,
    pub summary: VoteSummary,
}

/// Legislation sponsored by a member in the feed
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SponsoredLegislationView {
    pub sponsor: MemberView,
    pub legislation: LegislationView,
    pub sponsored_at: Option<DateTime<FixedOffset>>,
}
