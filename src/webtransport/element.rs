use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sdk::{ActionItem, MoveActionItem, NewActionItem};

/// Wraps an element event with the actioner of the event
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserElementEvent {
    user: i32,
    element_event: ElementEvent,
}
impl UserElementEvent {
    pub fn wrap(user: i32, element_event: ElementEvent) -> Self {
        Self {
            user,
            element_event,
        }
    }
    pub fn user(&self) -> i32 {
        self.user
    }
    // pub fn element(&self) -> Uuid {
    //     self.element_event.id
    // }
    pub fn event(&self) -> &ElementEvent {
        &self.element_event
    }
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct ElementEvent {
//     pub action: ElementAction,
// }
// impl ElementEvent {
//     pub fn create(dimensions: Vec2, offset: Vec3) -> Self {
//         Self {
//             action: ElementAction::Create(NewRectangle { dimensions, offset }),
//         }
//     }
//     pub fn update(id: Uuid, dimensions: Vec2, offset: Vec3) -> Self {
//         Self {
//             id,
//             action: ElementAction::Update(UpdateRectangle { dimensions, offset }),
//         }
//     }
//     pub fn remove(id: Uuid) -> Self {
//         Self {
//             id,
//             action: ElementAction::Remove,
//         }
//     }
// }

/// Some action that has occurred to an element
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ClientElementEvent {
    Create(NewActionItem),
    Update(MoveActionItem),
    Remove(Uuid),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementEvent {
    Created(ActionItem),
    Updated(MoveActionItem),
    Removed(RemovedActionItem),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct RemovedActionItem {
    pub user: Uuid,
    pub action_item_id: Uuid,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[cfg_attr(feature = "web", derive(tsify::Tsify))]
// #[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
// pub struct NewRectangle {
//     dimensions: Vec2,
//     offset: Vec3,
// }
// impl NewRectangle {
//     pub fn dimensions(&self) -> Vec2 {
//         self.dimensions
//     }
//     pub fn offset(&self) -> Vec3 {
//         self.offset
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[cfg_attr(feature = "web", derive(tsify::Tsify))]
// #[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
// pub struct UpdateRectangle {
//     dimensions: Vec2,
//     offset: Vec3,
// }
// impl UpdateRectangle {
//     pub fn dimensions(&self) -> Vec2 {
//         self.dimensions
//     }
//     pub fn offset(&self) -> Vec3 {
//         self.offset
//     }
// }
