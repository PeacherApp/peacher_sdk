use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    sdk::MemberView,
    webtransport::global::{CampaignEvent, SharedEvent, UserAction, UserEvent},
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
// #[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
pub enum ServerMessage {
    /// Clients will only recieve this message for themselves.
    /// The provided sharedentity is the clients entity to record.
    Gatekeeper(GatekeeperMessage),

    /// This is an event that should be passed into the ECS for the client.
    Broadcast(SharedEvent),
    Error(String),
}

impl ServerMessage {
    pub fn user(id: i32, action: UserAction) -> Self {
        Self::Broadcast(SharedEvent::User(UserEvent { id, action }))
    }
    pub fn campaign(event: CampaignEvent) -> Self {
        Self::Broadcast(SharedEvent::Campaign(event))
    }
    pub fn decode(buf: &[u8]) -> anyhow::Result<Self> {
        let payload = buf.get(4..).context("Buffer too short")?;
        let this = ciborium::from_reader(payload)?;

        Ok(this)
    }
}
