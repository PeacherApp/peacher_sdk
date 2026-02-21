use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub struct GetAccount;

impl GetHandler for GetAccount {
    type ResponseBody = AccountView;
    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/account".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AccountView {
    pub member: MemberWithPartyView,
    pub member_location: Option<ViewerLocationResponse>,
    pub ban: Option<BanInfo>,
    pub email: Option<String>,
    #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>, format = DateTime))]
    pub email_verified_at: Option<DateTime<FixedOffset>>,
    pub pending_email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RequestEmailVerificationRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RequestEmailVerificationResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct VerifyEmailResponse {
    pub message: String,
}

pub struct RequestEmailVerification {
    pub body: RequestEmailVerificationRequest,
}

impl RequestEmailVerification {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            body: RequestEmailVerificationRequest {
                email: email.into(),
            },
        }
    }
}

impl Handler for RequestEmailVerification {
    type ResponseBody = RequestEmailVerificationResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/account/email".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

pub struct VerifyEmail {
    pub token: String,
}

impl VerifyEmail {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

impl GetHandler for VerifyEmail {
    type ResponseBody = VerifyEmailResponse;

    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/account/email/verify".into()
    }

    fn params(&self) -> impl SdkParams {
        Params::new(format!("token={}", self.token))
    }
}
