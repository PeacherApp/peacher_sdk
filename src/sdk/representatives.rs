use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::{paginated, prelude::*};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct RepresentativeParams {
    #[serde(default)]
    pub order_by: RepresentativeOrder,
    #[serde(default)]
    pub order: Ordering,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(RepresentativeParams);

#[derive(
    Default, Clone, Copy, EnumString, Display, Debug, PartialEq, Eq, Serialize, Deserialize,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RepresentativeOrder {
    #[default]
    RecentAction,
    Id,
}
