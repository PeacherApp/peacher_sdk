use serde::{Deserialize, Serialize};

use crate::{paginated, prelude::*};

/// Parameters for listing chambers
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ChamberParams {
    /// Filter by external ID
    pub external_id: Option<String>,
    /// Filter by jurisdiction ID
    pub jurisdiction_id: Option<i32>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(ChamberParams);

impl ChamberParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn with_jurisdiction(mut self, jurisdiction_id: i32) -> Self {
        self.jurisdiction_id = Some(jurisdiction_id);
        self
    }
}
/// Query parameters for chamber details (session selection)
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ChamberDetailsParams {
    /// Session ID - defaults to current session if not provided
    pub session: Option<i32>,
}
