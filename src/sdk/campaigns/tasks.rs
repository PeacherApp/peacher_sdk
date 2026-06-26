use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::geometry::{Vec2, Vec3};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct NewCampaignTask {
    pub dimensions: Vec2,
    pub offset: Vec3,
    pub details: ActionItemDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct MoveCampaignTask {
    pub id: Uuid,
    pub dimensions: Vec2,
    pub offset: Vec3,
}

impl MoveCampaignTask {
    pub fn update(&self, item: &mut ActionItem) {
        debug_assert_eq!(self.id, item.id);
        item.offset = self.offset.clone();
        item.dimensions = self.dimensions.clone();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct EditCampaignTask {
    pub id: Uuid,
    pub details: ActionItemDetails,
}

impl EditCampaignTask {
    pub fn update(&self, item: &mut ActionItem) {
        debug_assert_eq!(self.id, item.id);
        item.details = self.details.clone();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ActionItem {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub created_by: i32,
    pub updated_by: i32,
    pub dimensions: Vec2,
    pub offset: Vec3,
    pub status: TaskStatus,
    pub details: ActionItemDetails,
}

/// A task's workflow status. A fixed, static set (no per-campaign customization).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    #[default]
    Todo,
    InProgress,
    Blocked,
    Done,
}

/// Set a task's status.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct SetTaskStatus {
    pub id: Uuid,
    pub status: TaskStatus,
}

impl SetTaskStatus {
    pub fn update(&self, item: &mut ActionItem) {
        debug_assert_eq!(self.id, item.id);
        item.status = self.status;
    }
}

/// The kind of link between two tasks (drawn node-to-node on the canvas).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(rename_all = "snake_case")]
pub enum TaskEdgeKind {
    Dependency,
    Subtask,
}

/// A persisted edge between two tasks. Identity is the `(from_task, to_task)`
/// pair — the table has a composite primary key, so there is no surrogate id.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct TaskEdgeView {
    pub from_task: Uuid,
    pub to_task: Uuid,
    pub kind: TaskEdgeKind,
}

/// Request to create a task edge.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct NewTaskEdge {
    pub from_task: Uuid,
    pub to_task: Uuid,
    pub kind: TaskEdgeKind,
}

/// Identifies an edge by its `(from_task, to_task)` pair — used to remove an
/// edge (client → server) and to announce a removal (server → client).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct TaskEdgeRef {
    pub from_task: Uuid,
    pub to_task: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content")]
pub enum ActionItemDetails {
    Phonebank(PhoneBankDetails),
    Meetup(MeetupDetails),
    Custom(serde_json::Value),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum ActionItemKind {
    Phonebank,
    Meetup,
    Custom,
}
impl ActionItemKind {
    pub fn into_default_details(self) -> ActionItemDetails {
        match self {
            ActionItemKind::Custom => {
                ActionItemDetails::Custom(serde_json::Value::String(String::new()))
            }
            ActionItemKind::Meetup => ActionItemDetails::Meetup(MeetupDetails {
                location: String::new(),
            }),
            ActionItemKind::Phonebank => ActionItemDetails::Phonebank(PhoneBankDetails {
                phone_number: String::new(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PhoneBankDetails {
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MeetupDetails {
    pub location: String,
}
