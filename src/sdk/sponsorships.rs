use crate::{paginated, prelude::*};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct SponsorshipParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(SponsorshipParams);

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Display,
    EnumString,
    PartialEq,
    Eq,
    Hash,
    VariantArray,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
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
