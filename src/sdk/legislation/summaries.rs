use crate::{paginated, prelude::*};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SummaryKind {
    User,
    PrimarySource,
    Ai,
}

/// The type of summary for this content
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum CreateSummaryKind {
    PrimarySource,
    User,
}
impl CreateSummaryKind {
    pub fn to_summary_kind(&self) -> SummaryKind {
        match self {
            CreateSummaryKind::PrimarySource => SummaryKind::PrimarySource,
            CreateSummaryKind::User => SummaryKind::User,
        }
    }
}

/// Request to create a new summary.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSummaryRequest {
    pub kind: CreateSummaryKind,
    pub content: SetContentRequest,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SummaryView {
    pub legislation_id: i32,
    pub contents: ContentView,
    pub visibility: Visibility,
    pub kind: SummaryKind,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Visibility {
    NotVisible,
    Public,
}

/// Request to review (approve/reject) a summary as a moderator.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewSummaryRequest {
    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ModeratorSummaryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub legislation_id: Option<i32>,
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
paginated!(ModeratorSummaryParams);
