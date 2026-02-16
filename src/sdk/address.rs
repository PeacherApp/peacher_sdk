use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateAccountRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub full_name: Option<String>,
    pub handle: Option<String>,
    pub address: Option<SetLocation>,
    pub public: Option<bool>,
    pub email: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct AddressSearchParams {
    pub input: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BoundingBox {
    min: Vec2,
    max: Vec2,
}
impl BoundingBox {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);

        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);

        Self {
            min: Vec2 { x: min_x, y: min_y },
            max: Vec2 { x: max_x, y: max_y },
        }
    }
    pub fn from_nominatim_bb(bounding_box: [f32; 4]) -> Self {
        let p1 = Vec2 {
            x: bounding_box[2],
            y: bounding_box[0],
        };
        let p2 = Vec2 {
            x: bounding_box[3],
            y: bounding_box[1],
        };
        Self::new(p1, p2)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AddressSearchResponse {
    pub addresses: Vec<Address>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Address {
    pub address_type: String,
    pub bounding_box: BoundingBox,
    pub class: String,
    pub display_name: String,
    pub importance: f32,
    pub lat: f64,
    pub lon: f64,
    pub license: String,
    pub name: String,
    pub osm_id: i64,
    pub osm_type: String,
    pub place_id: i64,
    pub place_rank: i64,
    pub type_type: String,
}
