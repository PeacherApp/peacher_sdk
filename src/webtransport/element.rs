use bevy::math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementUpdate {
    Changed(Change),
    Removed(Uuid),
}
impl ElementUpdate {
    pub fn changed(id: Uuid, rect: Vec2, position: Vec3) -> Self {
        Self::Changed(Change { id, rect, position })
    }
    pub fn removed(id: Uuid) -> Self {
        Self::Removed(id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[allow(clippy::large_enum_variant)]
pub struct Change {
    pub id: Uuid,
    pub rect: Vec2,
    pub position: Vec3,
}
