use crate::{paginated, prelude::*};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Request to create a new summary for a piece of legislation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum CreateSummaryRequest {
    Document(serde_json::Value),
    Markdown(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SummaryView {
    pub contents: ContentView,
    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Visibility {
    NotVisible,
    Public,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ModeratorSummaryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    /// note that on the legislation/{id}/summaries route,
    /// this value is overwritten.
    pub legislation_id: Option<i32>,
    /// Note that this parameter is overwritten if the viewer does not have sufficient privileges
    pub visibility: Option<Visibility>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct SummaryParams {
    pub order_by: SummaryOrder,
    pub order: Ordering,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(
    Default, Clone, Copy, EnumString, Display, Debug, PartialEq, Eq, Serialize, Deserialize,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SummaryOrder {
    #[default]
    Weight,
}

paginated!(SummaryParams);
