mod client;
pub use client::*;

mod server;
pub use server::*;

use bevy_ecs::entity::Entity;

use bevy_math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementAction {
    Create(NewRectangle),
    Update(UpdateRectangle),
    Remove(Entity),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct NewRectangle {
    dimensions: Vec2,
    offset: Vec2,
}
impl NewRectangle {
    pub fn dimensions(&self) -> Vec2 {
        self.dimensions
    }
    pub fn offset(&self) -> Vec2 {
        self.offset
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::event::Event))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementUpdate {
    Changed(Change),
    Removed(Entity),
}
impl ElementUpdate {
    pub fn changed(entity: Entity, rect: Vec2, position: Vec3) -> Self {
        Self::Changed(Change {
            entity,
            rect,
            position,
        })
    }
    pub fn removed(id: Entity) -> Self {
        Self::Removed(id)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[allow(clippy::large_enum_variant)]
pub struct Change {
    pub entity: Entity,
    pub rect: Vec2,
    pub position: Vec3,
}
