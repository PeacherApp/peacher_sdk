use anyhow::Context;
use futures_util::future::Shared;
use serde::{Deserialize, Serialize};

use crate::{
    sdk::{CampaignDetails, MemberView},
    webtransport::{ClientElementAction, SharedEntity},
};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
pub struct ServerMessage {
    entity: SharedEntity,
    event: ServerEvent,
}

impl ServerMessage {
    pub fn new(entity: SharedEntity, event: ServerEvent) -> Self {
        Self { entity, event }
    }
    pub fn target(&self) -> SharedEntity {
        self.entity
    }
    pub fn event(&self) -> &ServerEvent {
        &self.event
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
// #[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
#[expect(clippy::large_enum_variant)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ServerEvent {
    Message { from: i32, content: String },
    Error(String),
    IdentifyYourself,
    Campaign(CampaignDetails),
    Element(ServerElementAction),
    YouAre(MemberView),
}

impl ServerEvent {
    pub fn decode(buf: &[u8]) -> anyhow::Result<Self> {
        let payload = buf.get(4..).context("Buffer too short")?;
        let this = ciborium::from_reader(payload)?;

        Ok(this)
    }
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
