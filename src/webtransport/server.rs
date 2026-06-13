use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    sdk::{ActionItem, CampaignDetails, MemberView},
    webtransport::global::SharedEvent,
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
    Welcome(IndividualWelcome),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct IndividualWelcome {
    campaign: CampaignDetails,
    action_items: Vec<ActionItem>,
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
