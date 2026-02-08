use std::convert::Infallible;

use http::StatusCode;
use thiserror::Error;

pub type SdkResult<T> = Result<T, SdkError>;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("{0} -- {1}")]
    Status(StatusCode, String),
    #[error("Request Error")]
    Request(Option<reqwest::Error>),
    #[error("Could not get body from response")]
    DeserializeBody(Option<reqwest::Error>),
    #[error("Could not get text body from response")]
    ResponseText(Option<reqwest::Error>),
    #[error("Could not deserialize value: {0}")]
    DeserializeValue(serde_json::Error),
    #[error("The requested isn't available: {0}")]
    Unsupported(String),
    #[error("{0}")]
    Other(String),
}
impl SdkError {
    pub fn status(status: StatusCode, msg: impl Into<String>) -> Self {
        Self::Status(status, msg.into())
    }
    pub fn unsupported(msg: impl Into<String>) -> Self {
        Self::Unsupported(msg.into())
    }
    pub fn message(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
    pub fn request(err: reqwest::Error) -> Self {
        Self::Request(Some(err))
    }
    pub fn debody(err: reqwest::Error) -> Self {
        Self::DeserializeBody(Some(err))
    }
}
impl From<anyhow::Error> for SdkError {
    fn from(value: anyhow::Error) -> Self {
        Self::Other(value.to_string())
    }
}
impl From<reqwest::Error> for SdkError {
    fn from(value: reqwest::Error) -> Self {
        if let Some(status) = value.status() {
            SdkError::Status(status, value.to_string())
        } else {
            SdkError::Other(value.to_string())
        }
    }
}

impl From<Infallible> for SdkError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
