use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    sdk::{CampaignDetails, MemberView},
    webtransport::ElementUpdate,
};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
#[expect(clippy::large_enum_variant)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ServerWebTransportMsg {
    Message { from: i32, content: String },
    Error(String),
    IdentifyYourself,
    Campaign(CampaignDetails),
    Element(ElementUpdate),
    YouAre(MemberView),
}

impl ServerWebTransportMsg {
    pub fn decode(buf: &[u8]) -> anyhow::Result<Self> {
        let payload = buf.get(4..).context("Buffer too short")?;
        let this = ciborium::from_reader(payload)?;

        Ok(this)
    }
}
