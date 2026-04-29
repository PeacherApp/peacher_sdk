use bevy_math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::webtransport::SharedEntity;

/// Wraps an element event with the actioner of the event
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserElementEvent {
    user: SharedEntity,
    element_event: ElementEvent,
}
impl UserElementEvent {
    pub fn wrap(user: SharedEntity, element_event: ElementEvent) -> Self {
        Self {
            user,
            element_event,
        }
    }
    pub fn user(&self) -> SharedEntity {
        self.user
    }
    pub fn element(&self) -> SharedEntity {
        self.element_event.entity()
    }
    pub fn action(&self) -> &ElementAction {
        self.element_event.action()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementEvent {
    element_entity: SharedEntity,
    action: ElementAction,
}
impl ElementEvent {
    pub fn create(element_entity: SharedEntity, dimensions: Vec2, offset: Vec3) -> Self {
        Self {
            element_entity,
            action: ElementAction::Create(NewRectangle { dimensions, offset }),
        }
    }
    pub fn update(element_entity: SharedEntity, dimensions: Vec2, offset: Vec3) -> Self {
        Self {
            element_entity,
            action: ElementAction::Update(UpdateRectangle { dimensions, offset }),
        }
    }
    pub fn remove(element_entity: SharedEntity) -> Self {
        Self {
            element_entity,
            action: ElementAction::Remove,
        }
    }
    pub fn entity(&self) -> SharedEntity {
        self.element_entity
    }
    pub fn action(&self) -> &ElementAction {
        &self.action
    }
}

/// Some action that has occurred to an element
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementAction {
    Create(NewRectangle),
    Update(UpdateRectangle),
    Remove,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct NewRectangle {
    dimensions: Vec2,
    offset: Vec3,
}
impl NewRectangle {
    pub fn dimensions(&self) -> Vec2 {
        self.dimensions
    }
    pub fn offset(&self) -> Vec3 {
        self.offset
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct UpdateRectangle {
    dimensions: Vec2,
    offset: Vec3,
}
impl UpdateRectangle {
    pub fn dimensions(&self) -> Vec2 {
        self.dimensions
    }
    pub fn offset(&self) -> Vec3 {
        self.offset
    }
}
