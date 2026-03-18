use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SitemapData {
    pub member_handles: Vec<String>,
    pub representative_handles: Vec<String>,
    pub legislation_ids: Vec<i32>,
    pub jurisdiction_ids: Vec<i32>,
    pub chamber_pairs: Vec<(i32, i32)>,
    pub post_ids: Vec<Uuid>,
}
