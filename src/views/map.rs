use serde::{Deserialize, Serialize};

/// Contains a map id and name
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MapView {
    pub id: i32,
    pub name: String,
}
