mod client;
pub use client::*;

mod global;
pub use global::*;

mod shared;
pub use shared::*;

mod element;
pub use element::*;

mod server;
pub use server::*;

// mod element;
// pub use element::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum CampaignMsg {
    Join(Uuid),
    Leave,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum RoomMsg {
    Say(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::resource::Resource))]
pub struct WebTransportInfo {
    pub url: String,
    pub cert_hash: Vec<u8>,
    pub token: Uuid,
}
impl WebTransportInfo {
    pub fn token(&self) -> Uuid {
        self.token
    }

    pub fn cert_hash(&self) -> &[u8] {
        &self.cert_hash
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}
