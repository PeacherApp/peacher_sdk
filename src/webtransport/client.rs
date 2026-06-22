use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::webtransport::ClientElementEvent;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum GatekeeperClientMessage {
    Iam(Uuid),
    JoinCampaign(Uuid),
}

/// Client → server messages sent while inside a room. Split into two
/// compile-time domains: the always-on **taskboard** (canvas), and an optional
/// single-**channel** chat subscription.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum RoomClientMessage {
    Taskboard(TaskboardClientMessage),
    Channel(ChannelClientMessage),
    Leave,
}

/// Taskboard (canvas) operations. High-frequency cursor updates travel over
/// datagrams, not here.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum TaskboardClientMessage {
    Element(ClientElementEvent),
}

/// Channel (chat) operations. A client is subscribed to at most one channel at
/// a time; `Subscribe` replaces any prior subscription.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ChannelClientMessage {
    Subscribe(Uuid),
    Unsubscribe,
    Say(ChannelSay),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ChannelSay {
    pub channel_id: Uuid,
    pub content: String,
}

impl From<ClientElementEvent> for RoomClientMessage {
    fn from(value: ClientElementEvent) -> Self {
        RoomClientMessage::Taskboard(TaskboardClientMessage::Element(value))
    }
}

impl From<TaskboardClientMessage> for RoomClientMessage {
    fn from(value: TaskboardClientMessage) -> Self {
        RoomClientMessage::Taskboard(value)
    }
}

impl From<ChannelClientMessage> for RoomClientMessage {
    fn from(value: ChannelClientMessage) -> Self {
        RoomClientMessage::Channel(value)
    }
}

/// While this derives bevy message, the shared lib does not add it as an event.
///
/// clients should use this to process messages.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ClientMessage {
    Gatekeeper(GatekeeperClientMessage),
    Room(RoomClientMessage),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn cbor_roundtrip(msg: &ClientMessage) -> ClientMessage {
        let mut buf = Vec::new();
        ciborium::into_writer(msg, &mut buf).unwrap();
        ciborium::from_reader(buf.as_slice()).unwrap()
    }

    /// The Taskboard/Channel nesting must survive a CBOR round-trip identically,
    /// since server (peachtree) and client (wasm) decode with the same derives.
    #[test]
    fn client_message_nested_domains_roundtrip() {
        let id = Uuid::from_u128(1);
        let cases = vec![
            ClientMessage::Room(RoomClientMessage::Taskboard(
                TaskboardClientMessage::Element(ClientElementEvent::Remove(id)),
            )),
            ClientMessage::Room(RoomClientMessage::Channel(ChannelClientMessage::Subscribe(
                id,
            ))),
            ClientMessage::Room(RoomClientMessage::Channel(
                ChannelClientMessage::Unsubscribe,
            )),
            ClientMessage::Room(RoomClientMessage::Channel(ChannelClientMessage::Say(
                ChannelSay {
                    channel_id: id,
                    content: "hello".into(),
                },
            ))),
            ClientMessage::Room(RoomClientMessage::Leave),
            ClientMessage::Gatekeeper(GatekeeperClientMessage::JoinCampaign(id)),
        ];
        for case in &cases {
            // The protocol enums don't derive PartialEq; debug equality is enough
            // to catch a changed serde representation.
            assert_eq!(format!("{case:?}"), format!("{:?}", cbor_roundtrip(case)));
        }
    }

    /// `From<ClientElementEvent>` must route through the Taskboard domain so the
    /// wasm `RuntimeState::send` path stays correct.
    #[test]
    fn element_event_wraps_into_taskboard() {
        let msg: RoomClientMessage = ClientElementEvent::Remove(Uuid::from_u128(2)).into();
        assert!(matches!(
            msg,
            RoomClientMessage::Taskboard(TaskboardClientMessage::Element(_))
        ));
    }
}
