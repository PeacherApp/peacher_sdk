use crate::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct DiscordAuthRequest {
    /// code exists if they accept
    pub code: Option<String>,

    /// state exists in all cases
    pub state: Option<String>,

    /// if they cancel
    pub error: Option<String>,
    /// if they cancel
    pub error_discription: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct AuthRequest {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AccountView {
    pub member: MemberView,
    pub member_location: Option<ViewerLocationResponse>,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Display,
    EnumString,
    Default,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AuthLevel {
    #[default]
    Member,
    Moderator,
    Admin,
}
