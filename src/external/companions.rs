use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Request body for creating a companion relationship between legislation
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalCompanionRequest {
    /// External ID of the companion legislation
    pub companion_external_id: ExternalId,
}

/// Response for companion creation
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CompanionResponse {
    pub legislation_id: i32,
    pub companion_id: i32,
}
