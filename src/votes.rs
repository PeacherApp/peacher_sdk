use crate::{paginated, prelude::*};
use serde::{Deserialize, Serialize};

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
