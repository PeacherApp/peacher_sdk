use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::webtransport::{CampaignMsg, ElementEvent, RoomMsg};

/// While this derives bevy message, the shared lib does not add it as an event.
///
/// clients should use this to process messages.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ClientMessage {
    Iam(Uuid),
    Campaign(CampaignMsg),
    Room(RoomMsg),
    Element(ElementEvent),
    Nothing,
}

impl ClientMessage {
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
