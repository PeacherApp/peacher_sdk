use serde::{Deserialize, Serialize};

use crate::{paginated, prelude::*};

/// Parameters for listing jurisdictions
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct JurisdictionParams {
    /// Filter by external ID
    pub external_id: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(JurisdictionParams);

impl JurisdictionParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }
}

/// Query parameters for jurisdiction details (session selection)
#[derive(Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct JurisdictionDetailsParams {
    /// Session ID - defaults to current session if not provided
    pub session: Option<i32>,
}
