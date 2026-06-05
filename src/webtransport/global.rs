use serde::{Deserialize, Serialize};

use crate::{
    sdk::{CampaignDetails, MemberView},
    webtransport::{ServerMessage, UserElementEvent},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
pub enum SharedEvent {
    User(UserEvent),
    Campaign(CampaignEvent),
    Element(UserElementEvent),
}
impl SharedEvent {
    pub fn user_joined(view: MemberView) -> Self {
        Self::User(UserEvent {
            id: view.id,
            action: UserAction::IdentifiedAs(view),
        })
    }

    pub fn user_disconnected(id: i32) -> Self {
        Self::User(UserEvent {
            id,
            action: UserAction::Disconnected,
        })
    }

    pub fn campaign_details(details: CampaignDetails) -> Self {
        Self::Campaign(CampaignEvent::Details(details))
    }
    pub fn campaign_error(msg: impl Into<String>) -> Self {
        Self::Campaign(CampaignEvent::Error(msg.into()))
    }
}
impl From<SharedEvent> for ServerMessage {
    fn from(value: SharedEvent) -> Self {
        Self::Global(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEvent {
    pub id: i32,
    pub action: UserAction,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum UserAction {
    IdentifiedAs(MemberView),
    Disconnected,
    Says(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum CampaignEvent {
    Details(CampaignDetails),
    Error(String),
}
