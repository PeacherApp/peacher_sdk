mod element;
use bevy_ecs::entity::Entity;
pub use element::{Change, ElementUpdate};

use anyhow::Context;
use bevy_math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sdk::{CampaignDetails, MemberView};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementAction {
    Create(NewRectangle),
    Update(UpdateRectangle),
    Remove(Entity),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct NewRectangle {
    client_nonce: Uuid,
    dimensions: Vec2,
    offset: Vec2,
}
impl NewRectangle {
    pub fn new(client_nonce: Uuid, dimensions: Vec2, offset: Vec2) -> Self {
        Self {
            client_nonce,
            dimensions,
            offset,
        }
    }
    pub fn client_nonce(&self) -> Uuid {
        self.client_nonce
    }
    pub fn dimensions(&self) -> Vec2 {
        self.dimensions
    }
    pub fn offset(&self) -> Vec2 {
        self.offset
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct UpdateRectangle {
    entity: Entity,
    dimensions: Vec2,
    offset: Vec2,
}
impl UpdateRectangle {
    pub fn entity(&self) -> Entity {
        self.entity
    }
    pub fn dimensions(&self) -> Vec2 {
        self.dimensions
    }
    pub fn offset(&self) -> Vec2 {
        self.offset
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum CampaignMsg {
    Join(Uuid),
    Leave,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum RoomMsg {
    Say(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ClientWebTransportMsg {
    Iam(Uuid),
    Campaign(CampaignMsg),
    Room(RoomMsg),
    Element(ElementAction),
    Nothing,
}
impl ClientWebTransportMsg {
    /// this method allocated an internal vector and then extends the passed in buffer.
    ///
    /// This isn't fantastic. It's just a quick impl.
    ///
    /// Does not clear the buffer. extends it.
    pub fn append_into(&self, buf: &mut Vec<u8>) {
        let mut allocvec = Vec::with_capacity(size_of::<Self>());
        ciborium::into_writer(self, &mut allocvec).unwrap();

        let needed_cap = 4 + allocvec.len();

        if buf.capacity() < needed_cap {
            let additional_to_reserve = needed_cap - buf.capacity();
            _ = buf.try_reserve(additional_to_reserve);
        }

        buf.extend_from_slice(&(allocvec.len() as u32).to_be_bytes());
        buf.extend_from_slice(&allocvec);
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[expect(clippy::large_enum_variant)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ServerWebTransportMsg {
    Message { from: i32, content: String },
    Error(String),
    IdentifyYourself,
    Campaign(CampaignDetails),
    Element(ElementUpdate),
    YouAre(MemberView),
}

impl ServerWebTransportMsg {
    pub fn decode(buf: &[u8]) -> anyhow::Result<Self> {
        let payload = buf.get(4..).context("Buffer too short")?;
        let this = ciborium::from_reader(payload)?;

        Ok(this)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct WebTransportInfo {
    pub url: String,
    pub cert_hash: Vec<u8>,
    pub token: Uuid,
}
