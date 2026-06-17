use serde::{Deserialize, Serialize};

use crate::sdk::ChannelMessageView;

/// Server → client events for the **channel** (chat) domain. Routed only to the
/// clients currently subscribed to the message's channel.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub enum ChannelEvent {
    Message(ChannelMessageView),
}
