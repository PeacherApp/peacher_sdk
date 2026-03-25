use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::prelude::*;

/// The external interface for identifying a legislative vote on some piece of legislation.
///
/// **IMPORTANT**
/// Most apis do not provide unique identifiers for their id. You will, most likely, need
/// to derive an external id from the legislation id and the vote id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalLegislationVote {
    pub vote_name: String,
    pub url: Option<Url>,
    pub date_occurred: Option<DateTime<FixedOffset>>,
    pub chamber_id: ExternalId,
    /// Note that many APIs duplicate their external ID as a composite
    /// primary key of the legislation and another value.
    /// you will have to augment this conversion.
    pub external_id: ExternalId,
    /// Type of vote: Passage, Procedural, or VetoOverride
    pub vote_type: VoteType,
    pub votes: Vec<ExternalMemberVote>,
    /// Did the vote succeed? It depends on the rules of the chamber.
    ///
    /// You can use the [`VoteSuccess`] and [`VoteSuccessExt`] traits here for ease
    /// of use.
    /// ```rust
    /// use peacher_sdk::prelude::{Vote, VoteSuccessExt, SimpleMajority};
    /// let votes = [Vote::Yes, Vote::Yes, Vote::No];
    /// assert!(votes.succeeds(SimpleMajority));
    /// ```
    pub succeeded: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalMemberVote {
    pub member_id: ExternalId,
    pub vote: Vote,
}
impl ExternalMemberVote {
    pub fn new(member_id: impl Into<ExternalId>, vote: Vote) -> Self {
        Self {
            member_id: member_id.into(),
            vote,
        }
    }
}
