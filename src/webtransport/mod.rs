use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientWebTransportMsg {
    Join(i32),
    Leave,
    Say { text: String },
    Nothing,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerWebTransportMsg {
    Message { from: i32, content: String },
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct WebTransportInfo {
    pub url: String,
    pub cert_hash: Vec<u8>,
}
