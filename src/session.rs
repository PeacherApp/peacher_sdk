use serde::{Deserialize, Serialize};

use crate::{paginated, prelude::*};

/// Parameters for listing sessions
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct SessionParams {
    /// Filter to only current sessions
    pub current: Option<bool>,
    /// Filter by jurisdiction ID
    pub jurisdiction_id: Option<i32>,
    /// Filter by external ID
    pub external_id: Option<String>,
    /// Sort order: "name", "recent", "oldest"
    pub sort: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(SessionParams);

impl SessionParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_current(mut self, current: bool) -> Self {
        self.current = Some(current);
        self
    }

    pub fn with_jurisdiction(mut self, jurisdiction_id: i32) -> Self {
        self.jurisdiction_id = Some(jurisdiction_id);
        self
    }

    pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn with_sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }
}
