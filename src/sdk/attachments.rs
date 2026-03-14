use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AttachmentResponse {
    pub id: Uuid,
    pub name: String,
    pub file_type: String,
    pub download_url: Url,
    pub preview_url: Option<String>,
}
