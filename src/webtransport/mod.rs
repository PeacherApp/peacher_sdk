use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sdk::{CampaignDetails, MemberView};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum ClientWebTransportMsg {
    JoinRoom(i32),
    LeaveRoom,
    Iam(Uuid),
    Say { text: String },
    JoinCampaign(Uuid),
    Nothing,
}
impl ClientWebTransportMsg {
    /// this method allocated an internal vector and then extends the passed in buffer.
    ///
    /// This isn't fantastic. It's just a quick impl.
    ///
    /// Does not clear the buffer. extends it.
    pub fn append_into(&self, buf: &mut Vec<u8>) {
        let mut allocvec = Vec::with_capacity(size_of::<Self>());
        ciborium::into_writer(self, &mut allocvec).unwrap();

        let needed_cap = 4 + allocvec.len();

        if buf.capacity() < needed_cap {
            let additional_to_reserve = needed_cap - buf.capacity();
            _ = buf.try_reserve(additional_to_reserve);
        }

        buf.extend_from_slice(&(allocvec.len() as u32).to_be_bytes());
        buf.extend_from_slice(&allocvec);
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[allow(clippy::large_enum_variant)]
pub enum ServerWebTransportMsg {
    Message { from: i32, content: String },
    Error(String),
    IdentifyYourself,
    Campaign(CampaignDetails),
    YouAre(MemberView),
}

impl ServerWebTransportMsg {
    pub fn decode(buf: &[u8]) -> anyhow::Result<Self> {
        let payload = buf.get(4..).context("Buffer too short")?;
        let this = ciborium::from_reader(payload)?;

        Ok(this)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct WebTransportInfo {
    pub url: String,
    pub cert_hash: Vec<u8>,
    pub token: Uuid,
}
