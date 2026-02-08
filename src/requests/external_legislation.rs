use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalLegislation {
    pub external_id: ExternalId,
    pub name_id: String,
    pub title: String,
    /// When the primary source material was last updated.
    ///
    /// If your API does not provide this data, use `Local::now()`.
    pub external_update_at: DateTime<FixedOffset>,
    pub legislation_type: LegislationType,
    /// Human-readable status text
    pub status_text: String,
    /// When the status was last updated
    pub status_updated_at: Option<DateTime<FixedOffset>>,

    pub summary: String,
    /// Where the legislation started
    pub chamber_id: ExternalId,
    pub url: Option<Url>,
    pub introduced_at: Option<DateTime<FixedOffset>>,
    /// Current outcome of the legislation (replaces active boolean)
    pub status: Option<LegislationOutcome>,
    pub sponsors: Vec<ExternalSponsor>,
    pub votes: Vec<ExternalLegislationVote>,
}
impl ExternalLegislation {
    pub fn into_create_legislation_request(self) -> CreateLegislationRequest {
        let mut req = CreateLegislationRequest::new(
            self.name_id.clone(),
            self.title.clone(),
            self.summary.clone(),
            self.legislation_type,
            self.status_text.clone(),
        );

        if let Some(introduced_at) = self.introduced_at {
            req = req.introduced_at(introduced_at);
        }
        if let Some(outcome) = self.status {
            req = req.outcome(outcome);
        }
        if let Some(status_updated_at) = self.status_updated_at {
            req = req.status_updated_at(status_updated_at);
        }

        let mut ext_metadata = ExternalMetadata::new(self.external_id.clone());
        if let Some(ref url) = self.url {
            ext_metadata.set_url(url.clone());
        }
        req = req.external_metadata(ext_metadata);

        req
    }

    pub fn needs_update(&self, view: &LegislationView) -> bool {
        self.status == view.outcome
            && self.title == view.title
            && self.status_text == view.status
            && view
                .external
                .as_ref()
                .is_some_and(|val| val.external_id == self.external_id && val.url == self.url)
            && view.summary == self.summary
            && view.legislation_type == self.legislation_type
    }

    pub fn into_update_legislation_request(self) -> UpdateLegislationRequest {
        UpdateLegislationRequest {
            name_id: Some(self.name_id),
            title: Some(self.title),
            summary: Some(self.summary),
            legislation_type: Some(self.legislation_type),
            status: Some(self.status_text),
            introduced_at_set: true,
            introduced_at: self.introduced_at,
            outcome_set: true,
            outcome: self.status,
            external_update_at: Some(self.external_update_at),
            status_updated_set: true,
            status_updated_at: self.status_updated_at,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ExternalSponsor {
    pub external_member_id: ExternalId,
    pub sponsor_type: SponsorshipType,
    pub sponsored_at: Option<DateTime<FixedOffset>>,
}
