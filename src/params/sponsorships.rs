use crate::{paginated, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct SponsorshipParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(SponsorshipParams);
