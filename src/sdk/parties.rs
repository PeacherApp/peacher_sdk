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
