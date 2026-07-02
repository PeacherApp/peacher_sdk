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
    /// A message's content was edited (by its author). Carries the full updated
    /// view; clients replace the message with the matching id.
    MessageEdited(ChannelMessageView),
    /// A message was deleted (by its author or an organizer). Clients drop it
    /// from the identified channel's thread.
    MessageRemoved {
        channel_id: Uuid,
        message_id: Uuid,
    },
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

/// Client → server: delete a message. The server authorizes it (the acting
/// member must be the author, or a campaign organizer), hard-deletes the row,
/// then broadcasts [`ChannelEvent::MessageRemoved`] to the room.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ChannelDelete {
    pub channel_id: Uuid,
    pub message_id: Uuid,
}

/// Client → server: edit a message's content. The server authorizes it (only
/// the author may edit), persists the new content, then broadcasts
/// [`ChannelEvent::MessageEdited`] with the updated view.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ChannelEdit {
    pub channel_id: Uuid,
    pub message_id: Uuid,
    pub content: String,
}
