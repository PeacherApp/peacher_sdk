use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid Json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid markdown: {0}")]
    Markdown(markdown::message::Message),
    #[error("Something went wrong: {0}")]
    Msg(String),
}
impl ParseError {
    pub fn other(msg: impl fmt::Display) -> Self {
        Self::Msg(msg.to_string())
    }
}
impl From<markdown::message::Message> for ParseError {
    fn from(value: markdown::message::Message) -> Self {
        Self::Markdown(value)
    }
}

#[cfg(feature = "loco")]
impl From<ParseError> for loco_rs::Error {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::Json(_) => loco_rs::Error::BadRequest("Invalid Json!".to_string()),
            ParseError::Markdown(_) => loco_rs::Error::BadRequest("Invalid Markdown!".to_string()),
            ParseError::Msg(_) => loco_rs::Error::InternalServerError,
        }
    }
}
