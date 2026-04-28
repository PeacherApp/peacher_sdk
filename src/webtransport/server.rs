use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    sdk::{CampaignDetails, MemberView},
    webtransport::{
        ClientElementAction, SharedEntity,
        global::{CampaignAction, CampaignEvent, SharedEvent, UserAction, UserEvent},
    },
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
// #[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
#[expect(clippy::large_enum_variant)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ServerEvent {
    Message {
        from: i32,
        content: String,
    },
    Error(String),
    /// Clients will only recieve this message for themselves. So it is implied
    /// that the entity in [`ServerMessage`] is their provided [`SharedEntity`] identifier.
    IdentifyYourself,
    Campaign(CampaignDetails),
    Element(ServerElementAction),
    ThisIs(MemberView),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::event::Event))]
pub struct ServerElementAction {
    action: ClientElementAction,
}
impl ServerElementAction {
    pub fn new(action: ClientElementAction) -> Self {
        Self { action }
    }
    pub fn action(&self) -> &ClientElementAction {
        &self.action
    }
}
