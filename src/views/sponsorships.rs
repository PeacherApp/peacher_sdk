use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SponsorshipType {
    Primary,
    Cosponsor,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DetailedSponsorshipView {
    pub id: i32,
    pub member_id: i32,
    pub sponsor_type: SponsorshipType,
    pub legislation: DetailedLegislationView,
}
