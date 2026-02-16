use crate::{paginated, prelude::*};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

/// Get the current user's feed
#[derive(Default, Clone, Debug)]
pub struct GetFeed(pub FeedParams);

impl GetHandler for GetFeed {
    type ResponseBody = Paginated<FeedItem>;

    fn path(&self) -> Cow<'_, str> {
        "/api/feed".into()
    }

    fn params(&self) -> impl SdkParams {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct FeedParams {
    // A set value here is not respected by Peacher
    #[serde(skip)]
    pub feed: Option<Uuid>,
    // A set value here is not respected by Peacher
    #[serde(skip)]
    pub member_id: Option<i32>,
    /// time
    #[serde(default)]
    pub order_by: FeedOrder,
    /// asc | desc
    #[serde(default)]
    pub order: Ordering,

    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(FeedParams);

impl FeedParams {
    pub fn set_member(mut self, member_id: i32) -> Self {
        self.member_id = Some(member_id);
        self
    }
}

/// How the feed should be ordered
#[derive(
    Serialize, Deserialize, Default, Clone, EnumString, Display, Debug, PartialEq, Eq, Copy,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FeedOrder {
    #[default]
    Time,
}
