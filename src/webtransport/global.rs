use bevy_math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::{
    sdk::{CampaignDetails, MemberView},
    webtransport::{ServerMessage, SharedEntity},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::message::Message))]
pub enum SharedEvent {
    User(UserEvent),
    Campaign(CampaignEvent),
    Element(ElementEvent),
}
impl SharedEvent {
    pub fn identify_user(entity: SharedEntity, view: MemberView) -> Self {
        Self::User(UserEvent {
            entity,
            action: UserAction::IdentifiedAs(view),
        })
    }
    pub fn join_campaign(member: SharedEntity, campaign: SharedEntity) -> Self {
        Self::User(UserEvent {
            entity: member,
            action: UserAction::JoinedCampaign(campaign),
        })
    }
    pub fn provide_campaign_details(campaign: SharedEntity, details: CampaignDetails) -> Self {
        Self::Campaign(CampaignEvent {
            entity: campaign,
            action: CampaignAction::Details(details),
        })
    }
    pub fn campaign_error(campaign: SharedEntity, msg: impl Into<String>) -> Self {
        Self::Campaign(CampaignEvent {
            entity: campaign,
            action: CampaignAction::Error(msg.into()),
        })
    }
}
impl From<SharedEvent> for ServerMessage {
    fn from(value: SharedEvent) -> Self {
        Self::Global(value)
    }
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
    Error(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementEvent {
    entity: SharedEntity,
    action: ElementAction,
}
impl ElementEvent {
    pub fn create(entity: SharedEntity, dimensions: Vec2, offset: Vec3) -> Self {
        Self {
            entity,
            action: ElementAction::Create(NewRectangle { dimensions, offset }),
        }
    }
    pub fn update(entity: SharedEntity, dimensions: Vec2, offset: Vec3) -> Self {
        Self {
            entity,
            action: ElementAction::Update(UpdateRectangle { dimensions, offset }),
        }
    }
    pub fn remove(entity: SharedEntity) -> Self {
        Self {
            entity,
            action: ElementAction::Remove,
        }
    }
    pub fn entity(&self) -> SharedEntity {
        self.entity
    }
    pub fn action(&self) -> &ElementAction {
        &self.action
    }
}

/// Some action that has occurred to an element
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ElementAction {
    Create(NewRectangle),
    Update(UpdateRectangle),
    Remove,
}
impl ElementAction {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct NewRectangle {
    dimensions: Vec2,
    offset: Vec3,
}
impl NewRectangle {
    pub fn dimensions(&self) -> Vec2 {
        self.dimensions
    }
    pub fn offset(&self) -> Vec3 {
        self.offset
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct UpdateRectangle {
    dimensions: Vec2,
    offset: Vec3,
}
impl UpdateRectangle {
    pub fn dimensions(&self) -> Vec2 {
        self.dimensions
    }
    pub fn offset(&self) -> Vec3 {
        self.offset
    }
}
