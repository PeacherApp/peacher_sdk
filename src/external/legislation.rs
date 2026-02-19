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

    /// Current outcome of the legislation (replaces active boolean)
    pub status: Option<LegislationStatus>,
    /// Human-readable status text
    pub status_text: String,
    /// When the status was last updated
    pub status_updated_at: Option<DateTime<FixedOffset>>,

    pub summary: Option<SetContentRequest>,
    /// Where the legislation started
    pub chamber_id: ExternalId,
    pub url: Option<Url>,
    pub introduced_at: Option<DateTime<FixedOffset>>,
    pub sponsors: Vec<ExternalSponsor>,
    pub votes: Vec<ExternalLegislationVote>,
}
impl ExternalLegislation {
    pub fn into_create_legislation_request(self) -> CreateLegislationRequest {
        CreateLegislationRequest {
            name_id: self.name_id.clone(),
            title: self.title.clone(),
            summary: self.summary.map(|summary| CreateSummaryRequest {
                kind: CreateSummaryKind::PrimarySource,
                content: summary,
            }),
            legislation_type: self.legislation_type,
            status_text: self.status_text.clone(),
            status: self.status,
            status_updated_at: self.status_updated_at.unwrap_or(self.external_update_at),
            introduced_at: self.introduced_at,
            external_metadata: Some(ExternalMetadata {
                external_id: self.external_id.clone(),
                url: self.url.clone(),
                externally_updated_at: Some(self.external_update_at),
            }),
        }
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn needs_update(&self, view: &LegislationView) -> bool {
        view.external
            .as_ref()
            .is_some_and(|val| val.external_id == self.external_id)
            && (self.status != view.status
                || self.title != view.title
                || self.status_text != view.status_text
                || self.status != view.status
                || self
                    .status_updated_at
                    .is_some_and(|status| status != view.status_updated_at)
                || view
                    .external
                    .as_ref()
                    .is_some_and(|val| val.url != self.url)
                || view.legislation_type != self.legislation_type)
    }

    pub fn into_update_legislation_request(self) -> UpdateLegislationRequest {
        UpdateLegislationRequest {
            name_id: Some(self.name_id),
            title: Some(self.title),
            legislation_type: Some(self.legislation_type),
            status_text: Some(self.status_text),

            introduced_at_set: true,
            introduced_at: self.introduced_at,

            url_set: true,
            url: self.url,

            status_set: true,
            status: self.status,

            external_update_at: Some(self.external_update_at),

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
