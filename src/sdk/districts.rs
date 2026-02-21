use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SimpleBoundaryView {
    pub id: i32,
    pub map_id: i32,
    pub geo_id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BoundaryView {
    pub id: i32,
    pub map_id: i32,
    pub geo_id: i32,
    pub name: String,
    pub chambers: Vec<BoundaryChambers>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BoundaryChambers {
    pub chamber: ChamberView,
    pub members: Vec<MemberWithPartyView>,
}
