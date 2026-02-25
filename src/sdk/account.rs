use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::peanut::multipart::{MultipartForm, Part};
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
    pub trust: Trust,
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

pub struct ClearEmail;

impl Handler for ClearEmail {
    type ResponseBody = AccountView;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/account/email".into()
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

/// Upload a profile photo via multipart form data.
pub struct UploadAccountPhoto {
    file_data: Vec<u8>,
    file_name: String,
}

impl UploadAccountPhoto {
    pub fn new(file_name: impl Into<String>, file_data: Vec<u8>) -> Self {
        Self {
            file_name: file_name.into(),
            file_data,
        }
    }
}

impl Handler for UploadAccountPhoto {
    type ResponseBody = MemberWithPartyView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/account/photo".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        let form = MultipartForm::new().add_part(
            "file",
            Part::bytes(self.file_data.clone()).file_name(&self.file_name),
        );
        builder.multipart(form)
    }
}

pub struct DeleteAccountPhoto;

impl Handler for DeleteAccountPhoto {
    type ResponseBody = MemberWithPartyView;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/account/photo".into()
    }
}
