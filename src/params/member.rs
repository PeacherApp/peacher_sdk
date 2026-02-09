use crate::{paginated, prelude::*};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct MemberParams {
    /// Free text search across display_name, handle, and full_name
    pub freetext: Option<String>,
    /// A query for members following this member
    pub members_following: Option<i32>,
    /// A query for members that are followed by this member
    pub members_followed_by: Option<i32>,
    /// Filter by external ID
    pub external_id: Option<String>,

    #[serde(default)]
    pub order_by: MemberOrder,
    #[serde(default)]
    pub order: Ordering,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(MemberParams);

impl MemberParams {
    pub fn freetext(&self) -> Option<&str> {
        self.freetext.as_deref()
    }
    pub fn external_id(&self) -> Option<&str> {
        self.external_id.as_deref()
    }
    pub fn order(&self) -> Ordering {
        self.order
    }
    pub fn order_by(&self) -> MemberOrder {
        self.order_by
    }
}

#[derive(
    Default, Clone, Copy, EnumString, Display, Debug, PartialEq, Eq, Serialize, Deserialize,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MemberOrder {
    #[default]
    Id,
}
