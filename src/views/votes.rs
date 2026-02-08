use crate::prelude::*;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumString};

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
