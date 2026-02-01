use serde::{Deserialize, Serialize};

use crate::prelude::ExternalId;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalOwner {
    pub owner_id: i32,
    pub external_id: ExternalId,
    pub url: Option<String>,
}
impl ExternalOwner {
    pub fn new(owner_id: i32, external_id: impl ToString, url: Option<String>) -> Self {
        Self {
            owner_id,
            external_id: ExternalId::new(external_id),
            url,
        }
    }
}
