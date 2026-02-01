use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PartyView {
    pub id: i32,
    pub name: String,
}
impl Default for PartyView {
    fn default() -> Self {
        Self::unaffiliated()
    }
}

impl PartyView {
    pub fn unaffiliated() -> Self {
        Self {
            id: -1,
            name: "Unaffiliated".to_string(),
        }
    }
}
