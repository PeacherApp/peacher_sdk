use bevy_ecs::entity::Entity;
use bevy_math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementUpdate {
    Changed(Change),
    Removed(Entity),
}

impl ElementUpdate {
    pub fn changed(entity: Entity, rect: Vec2, position: Vec3, client_nonce: Option<Uuid>) -> Self {
        Self::Changed(Change {
            entity,
            rect,
            position,
            client_nonce,
        })
    }
    pub fn removed(id: Entity) -> Self {
        Self::Removed(id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[allow(clippy::large_enum_variant)]
pub struct Change {
    pub entity: Entity,
    pub rect: Vec2,
    pub position: Vec3,
    /// Echoed back on the first broadcast for an entity that originated from a
    /// client's optimistic spawn. The client uses this to bind its temporary
    /// entity to the server-assigned `entity`.
    pub client_nonce: Option<Uuid>,
}
