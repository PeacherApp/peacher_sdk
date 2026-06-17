use ahash::HashMap;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    sdk::{ActionItem, CampaignDetails, MemberView, TaskEdgeView},
    webtransport::{ChannelEvent, global::TaskboardEvent},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum GatekeeperMessage {
    IdentifyYourself(u64),
    AuthenticatedAs(MemberView),
}

impl From<GatekeeperMessage> for ServerMessage {
    fn from(value: GatekeeperMessage) -> Self {
        Self::Gatekeeper(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum IndividualEvent {
    Welcome(CampaignState),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum RoomMessage {
    /// Events sent to a specific client (e.g. the initial Welcome snapshot).
    Client(IndividualEvent),
    /// Taskboard events broadcast to every client in the room.
    Taskboard(TaskboardEvent),
    /// Channel (chat) events, routed only to the channel's current subscribers.
    Channel(ChannelEvent),
}

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

impl ServerMessage {
    pub fn taskboard(event: TaskboardEvent) -> Self {
        Self::Room(RoomMessage::Taskboard(event))
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
