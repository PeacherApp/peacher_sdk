use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::webtransport::{
    SharedEntity,
    global::{CampaignAction, CampaignEvent, SharedEvent, UserAction, UserEvent},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
// #[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
pub enum ServerMessage {
    /// Clients will only recieve this message for themselves.
    /// The provided sharedentity is the clients entity to record.
    IdentifyYourself(SharedEntity),
    /// This is an event that should be passed into the ECS for the client.
    Global(SharedEvent),
    Error(String),
}

impl ServerMessage {
    pub fn user(entity: SharedEntity, action: UserAction) -> Self {
        Self::Global(SharedEvent::User(UserEvent { entity, action }))
    }
    pub fn campaign(entity: SharedEntity, action: CampaignAction) -> Self {
        Self::Global(SharedEvent::Campaign(CampaignEvent { entity, action }))
    }
    pub fn decode(buf: &[u8]) -> anyhow::Result<Self> {
        let payload = buf.get(4..).context("Buffer too short")?;
        let this = ciborium::from_reader(payload)?;

        Ok(this)
    }
}
