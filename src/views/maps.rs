use serde::{Deserialize, Serialize};

/// Response after uploading a map.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MapUploadResponse {
    pub map_id: i32,
    pub name: String,
    pub district_count: usize,
    pub message: String,
}

/// Properties for a district in GeoJSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DistrictProperties {
    pub id: i32,
    pub name: String,
    pub geo_id: i32,
    pub lat: f64,
    pub lon: f64,
}

/// Response with map details.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MapDetailsResponse {
    pub id: i32,
    pub name: String,
    pub owner_id: Option<i32>,
    pub url: Option<String>,
    pub district_count: usize,
    pub created_at: String,
}

/// Response after previewing a map.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MapPreviewResponse {
    pub name: String,
    pub district_count: usize,
    pub districts: Vec<DistrictPreview>,
}

/// Preview of a single district.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DistrictPreview {
    pub id: i32,
    pub name: String,
    pub geo_id: i32,
    pub centroid_lat: f64,
    pub centroid_lon: f64,
}
