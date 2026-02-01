use crate::prelude::*;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct VoteView {
    pub legislation_vote: LegislationVote,
    pub legislation: LegislationView,
    pub vote_value: Vote,
}

/// A reference to a chamber (minimal info for display)
#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// A feed item showing votes from followed members on a single legislation vote
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GroupedVoteFeedItem {
    /// The legislation vote event
    pub legislation_vote: LegislationVote,
    /// The legislation being voted on
    pub legislation: LegislationView,
    /// Summary of votes from followed members only
    pub followed_summary: FollowedVoteSummary,
    /// Members grouped by their vote value
    pub votes_by_type: VotesByType,
}
impl GroupedVoteFeedItem {
    pub fn members(&self) -> impl Iterator<Item = &MemberView> {
        self.votes_by_type
            .yes
            .iter()
            .chain(self.votes_by_type.no.iter())
            .chain(self.votes_by_type.not_voting.iter())
            .chain(self.votes_by_type.absent.iter())
    }
}

/// Summary counts for followed members' votes
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FollowedVoteSummary {
    pub yes_count: i32,
    pub no_count: i32,
    pub absent_count: i32,
    pub not_voting_count: i32,
    pub total: i32,
}

/// Members grouped by vote type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct VotesByType {
    pub yes: Vec<MemberView>,
    pub no: Vec<MemberView>,
    pub absent: Vec<MemberView>,
    pub not_voting: Vec<MemberView>,
}
