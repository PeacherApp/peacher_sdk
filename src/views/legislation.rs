use crate::prelude::*;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumString};

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
pub enum LegislationOutcome {
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

impl LegislationOutcome {
    /// Returns true if this outcome represents an active/in-progress state
    pub fn is_active(&self) -> bool {
        matches!(self, LegislationOutcome::Pending)
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
    pub legislation_type: LegislationType,
    pub id: i32,
    pub name_id: String,
    pub title: String,
    pub summary: String,
    /// Current outcome of the legislation
    pub outcome: Option<LegislationOutcome>,
    /// Human-readable status text from external source
    pub status: String,
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
            id: self.id,
            name_id: self.name_id,
            title: self.title,
            summary: self.summary,
            status_updated_at: self.status_updated_at,
            legislation_type: self.legislation_type,
            outcome: self.outcome,
            status: self.status,
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
    pub summary: String,
    pub legislation_type: LegislationType,
    /// Current outcome of the legislation
    pub outcome: Option<LegislationOutcome>,
    /// Human-readable status text from external source
    pub status: String,
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
            id: self.id,
            name_id: self.name_id,
            title: self.title,
            summary: self.summary,
            legislation_type: self.legislation_type,
            status_updated_at: self.status_updated_at,
            outcome: self.outcome,
            status: self.status,
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
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LegislationDetailsResponse {
    pub id: i32,
    pub name_id: String,
    pub title: String,
    pub status: String,
    pub summary: String,
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
    pub sponsor_type: String,
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
}
