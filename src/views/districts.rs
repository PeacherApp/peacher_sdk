use serde::{Deserialize, Serialize};

use crate::views::MemberView;

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
    pub representative: Option<MemberView>,
}
