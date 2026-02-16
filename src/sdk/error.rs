/// Common error response structure from the api
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ErrorResponse {
    #[cfg_attr(feature = "utoipa", schema(example = "not_found"))]
    pub error: String,
    #[cfg_attr(feature = "utoipa", schema(example = "Resource was not found"))]
    pub description: String,
}
