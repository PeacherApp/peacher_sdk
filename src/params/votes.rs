use crate::{paginated, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct VoteParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    /// Comma-separated vote values to filter by (e.g. "Yes,No")
    pub vote_value: Option<String>,
}
paginated!(VoteParams);
