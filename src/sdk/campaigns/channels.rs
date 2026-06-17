use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;

/// A chat channel within a campaign. `action_item_id` links the channel to a
/// specific task; `None` is a general campaign channel.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChannelView {
    pub id: Uuid,
    pub name: String,
    pub action_item_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NewChannelRequest {
    pub name: String,
    pub action_item_id: Option<Uuid>,
}

/// A chat message. Crosses both the REST history boundary and the realtime
/// (wasm) boundary, so it carries both derives.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ChannelMessageView {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub author: MemberView,
    pub content: String,
    pub created_at: DateTime<FixedOffset>,
}

/// A page of channel history, newest-first, with a keyset cursor for the next
/// (older) page.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MessagePage {
    pub messages: Vec<ChannelMessageView>,
    /// Pass as `before` to fetch the next older page; `None` when exhausted.
    pub next_cursor: Option<Uuid>,
}

/// Query params for paginating channel history (keyset on the uuidv7 message id).
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct MessageHistoryParams {
    pub before: Option<Uuid>,
    pub limit: Option<u64>,
}
