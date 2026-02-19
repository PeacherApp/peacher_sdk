use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PartyView {
    pub id: i32,
    pub name: String,
    pub photo_url: Option<Url>,
    pub primary_color: String,
    pub secondary_color: String,
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
            photo_url: None,
            primary_color: "#AAA".to_string(),
            secondary_color: "#AAA".to_string(),
        }
    }
}
