use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateAccountRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub full_name: Option<String>,
    pub handle: Option<String>,
    pub follow_representatives: bool,
    pub address: Option<SetLocation>,
    pub public: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct AddressSearchParams {
    pub input: String,
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
