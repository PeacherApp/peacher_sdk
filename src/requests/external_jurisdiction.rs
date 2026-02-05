use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Represents a jurisdiction from an external data source.
///
/// This is used when syncing jurisdiction data from external legislative APIs.
/// The jurisdiction includes its chambers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalJurisdiction {
    pub name: String,
    pub external_id: ExternalId,
    pub url: Option<String>,
    pub chambers: Vec<ExternalChamber>,
}

impl ExternalJurisdiction {
    pub fn new(name: impl Into<String>, external_id: impl Into<ExternalId>) -> Self {
        Self {
            name: name.into(),
            external_id: external_id.into(),
            url: None,
            chambers: Vec::new(),
        }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_chamber(mut self, chamber: ExternalChamber) -> Self {
        self.chambers.push(chamber);
        self
    }

    pub fn with_chambers(mut self, chambers: impl IntoIterator<Item = ExternalChamber>) -> Self {
        self.chambers = chambers.into_iter().collect();
        self
    }

    pub fn external_id(&self) -> &ExternalId {
        &self.external_id
    }
}

/// Represents a chamber from an external data source.
///
/// Chambers are legislative bodies within a jurisdiction (e.g., House, Senate).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalChamber {
    pub name: String,
    pub external_id: ExternalId,
    pub url: Option<String>,
}

impl ExternalChamber {
    pub fn new(name: impl Into<String>, external_id: impl Into<ExternalId>) -> Self {
        Self {
            name: name.into(),
            external_id: external_id.into(),
            url: None,
        }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}
