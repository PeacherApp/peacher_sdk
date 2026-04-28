use serde::{Deserialize, Serialize};

use crate::{
    sdk::{CampaignDetails, MemberView},
    webtransport::SharedEntity,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
pub enum SharedEvent {
    User(UserEvent),
    Campaign(CampaignEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEvent {
    pub entity: SharedEntity,
    pub action: UserAction,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum UserAction {
    IdentifiedAs(MemberView),
    JoinedCampaign(SharedEntity),
    LeftCampaign(SharedEntity),
    Says(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CampaignEvent {
    pub entity: SharedEntity,
    pub action: CampaignAction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum CampaignAction {
    Details(CampaignDetails),
}
