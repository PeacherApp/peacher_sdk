use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sdk::ChannelMessageView;

/// Server → client events for the **channel** (chat) domain. Broadcast to every
/// member of the room (Discord-style fan-out); the message carries its
/// `channel_id`, so each client routes it to the right thread locally. There is
/// no per-channel subscription.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum ChannelEvent {
    Message(ChannelMessageView),
}

/// Client → server: post a message to a channel. Persisted synchronously, then
/// broadcast to the room as [`ChannelEvent::Message`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ChannelSay {
    pub channel_id: Uuid,
    pub content: String,
}
