use serde::{Deserialize, Serialize};

use crate::{
    sdk::MemberView,
    webtransport::{RoomMessage, ServerMessage, UserCursor, UserElementEvent},
};

/// Events broadcast to every client in a room's **taskboard** domain. Presence
/// and element changes are reliable (stream); `Cursor` is delivered over an
/// unreliable datagram by the transport layer.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum TaskboardEvent {
    Presence(PresenceEvent),
    Element(UserElementEvent),
    Cursor(UserCursor),
}

/// Who is on the board right now.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum PresenceEvent {
    Joined(MemberView),
    Left(i32),
}

impl From<UserElementEvent> for TaskboardEvent {
    fn from(value: UserElementEvent) -> Self {
        TaskboardEvent::Element(value)
    }
}

impl From<PresenceEvent> for TaskboardEvent {
    fn from(value: PresenceEvent) -> Self {
        TaskboardEvent::Presence(value)
    }
}

impl From<TaskboardEvent> for ServerMessage {
    fn from(value: TaskboardEvent) -> Self {
        Self::Room(RoomMessage::Taskboard(value))
    }
}
