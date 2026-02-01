use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Vote {
    Yes,
    No,
    Absent,
    NotVoting,
}

/// Outcome of legislation - tracks what ultimately happened to a bill
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum LegislationOutcome {
    #[default]
    Pending, // Still in progress
    Passed,         // Passed legislature, awaiting executive action
    Failed,         // Did not pass (voted down, died in committee, etc.)
    Signed,         // Signed into law by executive
    Vetoed,         // Vetoed by executive
    VetoOverridden, // Veto overridden by legislature
    Withdrawn,      // Sponsor withdrew the legislation
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

/// Type of a legislation vote
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum VoteType {
    #[default]
    Unknown,
    Passage,      // Final passage vote on the bill
    Procedural,   // Procedural motions, cloture, etc.
    VetoOverride, // Vote to override executive veto
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
