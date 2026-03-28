use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum WebTransportMsg {
    Say(String),
    Nothing,
}
