use crate::{paginated, prelude::*};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct VoteParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    /// Comma-separated vote values to filter by (e.g. "Yes,No")
    pub vote_value: Option<String>,
}
paginated!(VoteParams);

/// Request to create a vote on legislation
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateVoteRequest {
    pub name: String,
    pub occurred_at: Option<DateTime<FixedOffset>>,
    pub member_votes: Vec<MemberVoteInput>,
    /// The chamber where the vote occurs
    pub chamber: i32,
    pub external_metadata: Option<ExternalMetadata>,
    pub vote_type: VoteType,
}

impl CreateVoteRequest {
    pub fn new(
        name: impl Into<String>,
        occurred_at: Option<DateTime<FixedOffset>>,
        member_votes: Vec<MemberVoteInput>,
        chamber: i32,
        vote_type: VoteType,
    ) -> Self {
        Self {
            name: name.into(),
            occurred_at,
            member_votes,
            chamber,
            vote_type,
            external_metadata: None,
        }
    }

    pub fn external_metadata(mut self, metadata: ExternalMetadata) -> Self {
        self.external_metadata = Some(metadata);
        self
    }
}

/// Request to update an existing vote
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateVoteRequest {
    pub name: Option<String>,
    pub occurred_at: Option<DateTime<FixedOffset>>,
    pub member_votes: Option<Vec<MemberVoteInput>>,
}

impl UpdateVoteRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn occurred_at(mut self, date: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(date);
        self
    }

    pub fn member_votes(mut self, votes: Vec<MemberVoteInput>) -> Self {
        self.member_votes = Some(votes);
        self
    }
}

/// Input for a member's vote on legislation
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberVoteInput {
    pub member_id: i32,
    pub vote: Vote,
}

impl MemberVoteInput {
    pub fn new(member_id: i32, vote: Vote) -> Self {
        Self { member_id, vote }
    }
}

use std::borrow::Cow;

/// Handler for creating a vote
pub struct CreateVote {
    legislation_id: i32,
    body: CreateVoteRequest,
}

impl CreateVote {
    pub fn new(legislation_id: i32, body: CreateVoteRequest) -> Self {
        Self {
            legislation_id,
            body,
        }
    }
}

impl Handler for CreateVote {
    type ResponseBody = i32;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}/votes", self.legislation_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler for updating a vote
pub struct UpdateVote {
    legislation_id: i32,
    vote_id: i32,
    body: UpdateVoteRequest,
}

impl UpdateVote {
    pub fn new(legislation_id: i32, vote_id: i32, body: UpdateVoteRequest) -> Self {
        Self {
            legislation_id,
            vote_id,
            body,
        }
    }
}

impl Handler for UpdateVote {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Patch
    }

    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/legislation/{}/votes/{}",
            self.legislation_id, self.vote_id
        )
        .into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct VoteView {
    pub legislation_vote: LegislationVote,
    pub legislation: LegislationView,
    pub vote_value: Vote,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Vote {
    Yes,
    No,
    Absent,
    NotVoting,
}

/// Type of a legislation vote
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum VoteType {
    #[default]
    Unknown,
    /// Final passage vote on the bill
    Passage,
    /// Procedural motions, cloture, etc
    Procedural,
    /// Vote to override executive veto
    VetoOverride,
}

impl VoteType {
    /// Parse from string reference
    pub fn from_str_ref(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }
}
impl Vote {
    pub fn from_value(value: i32) -> Option<Self> {
        let this = match value {
            0 => Vote::Yes,
            1 => Vote::No,
            2 => Vote::Absent,
            3 => Vote::NotVoting,
            _ => return None,
        };
        Some(this)
    }
    pub fn value(&self) -> i32 {
        match self {
            Vote::Yes => 0,
            Vote::No => 1,
            Vote::Absent => 2,
            Vote::NotVoting => 3,
        }
    }
}

/// A reference to a chamber (minimal info for display)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberRef {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationVote {
    pub id: i32,
    pub name: String,
    pub occurred_at: Option<DateTime<FixedOffset>>,
    pub chamber: ChamberRef,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberVoteValue {
    pub member: MemberView,
    pub vote: Vote,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MemberVoteView {
    pub member: MemberView,
    pub vote: VoteView,
}

/// A vote event on legislation with only the user's representatives' votes
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RepresentativeVoteEvent {
    /// The legislation vote event (chamber, name, date)
    pub legislation_vote: LegislationVote,
    /// How each of the user's representatives voted
    pub representative_votes: Vec<MemberVoteValue>,
}

/// A legislation-centric feed item showing all vote events with representative votes
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationUpdateItem {
    /// The legislation being tracked
    pub legislation: LegislationView,
    /// All vote events on this legislation, ordered by most recent first
    pub vote_events: Vec<RepresentativeVoteEvent>,
    /// The most recent vote date (used for feed sorting)
    pub latest_vote_at: Option<DateTime<FixedOffset>>,
}
