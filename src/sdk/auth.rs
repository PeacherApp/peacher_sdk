use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct TokenLoginRequest {
    pub member_id: i32,
    pub token: String,
}

pub struct TokenLogin {
    pub body: TokenLoginRequest,
}

impl TokenLogin {
    pub fn new(member_id: i32, token: impl Into<String>) -> Self {
        Self {
            body: TokenLoginRequest {
                member_id,
                token: token.into(),
            },
        }
    }
}

impl Handler for TokenLogin {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/auth/token".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

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

impl AuthLevel {
    pub fn elevated(&self) -> bool {
        matches!(self, AuthLevel::Admin | AuthLevel::Moderator)
    }
}
