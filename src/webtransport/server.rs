use ahash::HashMap;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    sdk::{ActionItem, CampaignDetails, MemberView, TaskEdgeView},
    webtransport::{ChannelEvent, UserCursor, UserElementEvent},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
// #[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
pub enum ServerMessage {
    /// The authorization message state machine
    Gatekeeper(GatekeeperMessage),
    /// Events sent to members of a room. Only received after passing the gate keeper
    Room(RoomMessage),
    /// Global errors
    Error(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum GatekeeperMessage {
    /// Asks for identification.
    ///
    /// Provides a unique ID of the connection.
    IdentifyYourself(u64),
    /// The user has been identified and is being transferred to the campaign room
    MovingToRoom,
    /// Token was invalid
    InvalidToken,
    /// User does not exist?
    InvalidUser,
    /// Campaign doesn't exist?
    InvalidCampaign,
    /// Catchall error
    Error(String),
}

impl From<GatekeeperMessage> for ServerMessage {
    fn from(value: GatekeeperMessage) -> Self {
        Self::Gatekeeper(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum RoomMessage {
    /// Events sent to a specific client
    Client(IndividualEvent),
    /// Events broadcast to members of a room
    Broadcast(SharedEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum IndividualEvent {
    Welcome(WelcomeMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct WelcomeMessage {
    /// The current campaign state
    pub state: CampaignState,
    /// The user's identification
    pub identified_as: MemberView,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct CampaignState {
    pub campaign: CampaignDetails,
    /// individuals need this initial context always
    pub action_items: HashMap<Uuid, ActionItem>,
    /// drawn task-to-task links
    pub edges: Vec<TaskEdgeView>,
    /// who is currently on the board, so a joiner sees the existing roster
    pub participants: Vec<MemberView>,
}

impl ServerMessage {
    pub fn broadcast(shared_event: SharedEvent) -> Self {
        Self::Room(RoomMessage::Broadcast(shared_event))
    }
    pub fn client(indiv_event: IndividualEvent) -> Self {
        Self::Room(RoomMessage::Client(indiv_event))
    }

    pub fn decode(buf: &[u8]) -> anyhow::Result<Self> {
        let payload = buf.get(4..).context("Buffer too short")?;
        let this = ciborium::from_reader(payload)?;

        Ok(this)
    }
}

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
    /// this is a separate enum from [`SharedEvent::User`]
    ///
    /// as this typically is sent over a webtransport datagram (as opposed to a bidirectional stream).
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
