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
    pub chamber: GetChamberView,
    pub session: SessionView,
    /// members intersected with
    pub members: Vec<ChamberSessionMember>,
}
/// A simple reference to an intersecting district (map_id, district_id).
/// Used for serialization and passing between components.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DistrictId {
    pub map_id: i32,
    pub district_id: i32,
}
