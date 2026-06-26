use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sdk::{
    ActionItem, EditCampaignTask, MoveCampaignTask, NewCampaignTask, NewTaskEdge, SetTaskStatus,
    TaskEdgeRef, TaskEdgeView,
};

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

    pub fn event(&self) -> &ElementEvent {
        &self.element_event
    }
}

/// Some action that has occurred to an element
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ClientElementEvent {
    Create(NewCampaignTask),
    Update(MoveCampaignTask),
    Edit(EditCampaignTask),
    Remove(Uuid),
    SetStatus(SetTaskStatus),
    CreateEdge(NewTaskEdge),
    RemoveEdge(TaskEdgeRef),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementEvent {
    Created(ActionItem),
    Updated(MoveCampaignTask),
    Edited(EditCampaignTask),
    Removed(RemovedActionItem),
    StatusChanged(SetTaskStatus),
    EdgeCreated(TaskEdgeView),
    EdgeRemoved(TaskEdgeRef),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct RemovedActionItem {
    pub user: i32,
    pub action_item_id: Uuid,
}
