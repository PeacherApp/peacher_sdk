use serde::{Deserialize, Serialize};

use crate::{
    sdk::MemberView,
    webtransport::{RoomMessage, ServerMessage, UserElementEvent},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum SharedEvent {
    User(UserEvent),
    Element(UserElementEvent),
}
impl From<UserEvent> for SharedEvent {
    fn from(value: UserEvent) -> Self {
        SharedEvent::User(value)
    }
}
impl From<UserElementEvent> for SharedEvent {
    fn from(value: UserElementEvent) -> Self {
        SharedEvent::Element(value)
    }
}
impl SharedEvent {
    pub fn user_joined(view: MemberView) -> Self {
        Self::User(UserEvent {
            id: view.id,
            action: UserAction::IdentifiedAs(view),
        })
    }

    pub fn user_disconnected(id: i32) -> Self {
        Self::User(UserEvent {
            id,
            action: UserAction::Disconnected,
        })
    }
}
impl From<SharedEvent> for ServerMessage {
    fn from(value: SharedEvent) -> Self {
        Self::Room(RoomMessage::Broadcast(value))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEvent {
    pub id: i32,
    pub action: UserAction,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum UserAction {
    IdentifiedAs(MemberView),
    Disconnected,
    Says(String),
}
