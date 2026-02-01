use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalLegislation {
    pub external_id: ExternalId,
    pub name_id: Option<String>,
    pub title: Option<String>,
    pub legislation_type: Option<LegislationType>,
    /// Human-readable status text
    pub status: Option<String>,
    pub summary: Option<String>,
    /// Where the legislation started
    pub chamber_id: Option<ExternalId>,
    pub url: Option<String>,
    pub introduced_at: Option<Option<DateTime<FixedOffset>>>,
    /// Current outcome of the legislation (replaces active boolean)
    pub outcome: Option<Option<LegislationOutcome>>,
    /// When the legislation reached a terminal state
    pub resolved_at: Option<Option<DateTime<FixedOffset>>>,
    pub sponsors: Option<Vec<ExternalSponsor>>,
    pub votes: Option<Vec<ExternalLegislationVote>>,
}
impl ExternalLegislation {
    pub fn new(id: impl Into<ExternalId>) -> Self {
        Self {
            external_id: id.into(),
            name_id: None,
            title: None,
            legislation_type: None,
            status: None,
            summary: None,
            chamber_id: None,
            url: None,
            introduced_at: None,
            outcome: None,
            resolved_at: None,
            sponsors: None,
            votes: None,
        }
    }
    pub fn name_id(mut self, name_id: impl Into<String>) -> Self {
        self.name_id = Some(name_id.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
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

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn chamber_id(mut self, chamber_id: impl Into<ExternalId>) -> Self {
        self.chamber_id = Some(chamber_id.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn introduced_at(mut self, introduced_at: Option<DateTime<FixedOffset>>) -> Self {
        self.introduced_at = Some(introduced_at);
        self
    }

    pub fn outcome(mut self, outcome: Option<LegislationOutcome>) -> Self {
        self.outcome = Some(outcome);
        self
    }

    pub fn resolved_at(mut self, resolved_at: Option<DateTime<FixedOffset>>) -> Self {
        self.resolved_at = Some(resolved_at);
        self
    }

    pub fn sponsors(mut self, sponsors: Vec<ExternalSponsor>) -> Self {
        self.sponsors = Some(sponsors);
        self
    }
    pub fn votes(mut self, votes: impl IntoIterator<Item = ExternalLegislationVote>) -> Self {
        self.votes = Some(votes.into_iter().collect());
        self
    }

    pub fn into_create_legislation_request(self) -> CreateLegislationRequest {
        let mut req = CreateLegislationRequest::new(
            self.name_id.clone().unwrap_or_default(),
            self.title.clone().unwrap_or_default(),
            self.summary.clone().unwrap_or_default(),
            self.legislation_type.unwrap_or(LegislationType::Bill),
            self.status.clone().unwrap_or_default(),
        );

        if let Some(Some(introduced_at)) = self.introduced_at {
            req = req.introduced_at(introduced_at);
        }
        if let Some(Some(outcome)) = self.outcome {
            req = req.outcome(outcome);
        }
        if let Some(Some(resolved_at)) = self.resolved_at {
            req = req.resolved_at(resolved_at);
        }

        let mut ext_metadata = ExternalMetadata::new(self.external_id.clone());
        if let Some(ref url) = self.url {
            ext_metadata.set_url(url.clone());
        }
        req = req.external_metadata(ext_metadata);

        req
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ExternalSponsor {
    pub external_member_id: ExternalId,
    pub sponsor_type: SponsorshipType,
}
