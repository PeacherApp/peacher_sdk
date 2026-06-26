use serde::{Deserialize, Serialize};

use crate::{
    sdk::MemberView,
    webtransport::{ChannelEvent, RoomMessage, ServerMessage, UserCursor, UserElementEvent},
};

/// An event broadcast to every client in a room. `Cursor` is delivered over an
/// unreliable datagram by the transport layer; everything else rides the
/// reliable stream. `Channel` (chat) fans out to the whole room, tagged by
/// `channel_id`, so each client routes it to the right thread locally.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum SharedEvent {
    User(UserEvent),
    Element(UserElementEvent),
    Cursor(UserCursor),
    Channel(ChannelEvent),
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
impl From<UserCursor> for SharedEvent {
    fn from(value: UserCursor) -> Self {
        SharedEvent::Cursor(value)
    }
}
impl From<ChannelEvent> for SharedEvent {
    fn from(value: ChannelEvent) -> Self {
        SharedEvent::Channel(value)
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
