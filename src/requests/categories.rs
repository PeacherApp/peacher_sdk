use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: String,
    pub primary_color: String,
    pub secondary_color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCategoryRequest {
    pub description: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}
