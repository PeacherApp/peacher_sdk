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
