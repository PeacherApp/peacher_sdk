use crate::{
    peanut::multipart::{MultipartForm, Part},
    prelude::*,
};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, path::Path};

/// List all maps.
pub struct ListMaps;

impl GetHandler for ListMaps {
    type ResponseBody = Vec<MapDetailsResponse>;

    fn path(&self) -> Cow<'_, str> {
        "/api/maps".into()
    }
}

/// Get a map by ID.
pub struct GetMap(pub i32);

impl GetHandler for GetMap {
    type ResponseBody = MapDetailsResponse;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/maps/{}", self.0).into()
    }
}

/// Delete a map by ID.
pub struct DeleteMap(pub i32);

impl Handler for DeleteMap {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/maps/{}", self.0).into()
    }
}

/// Get a map as GeoJSON FeatureCollection.
pub struct GetMapGeojson(pub i32);

impl GetHandler for GetMapGeojson {
    type ResponseBody = GeoJson<DistrictProperties>;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/maps/{}/geojson", self.0).into()
    }
}

/// Configuration for how to extract district metadata from shapefile records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapefileFieldMapping {
    /// Field name for district ID (e.g., "DISTRICT", "CD119FP", "SLDUST")
    pub id_field: String,
    /// Optional field name for district name
    pub name_field: Option<String>,
    /// Optional field name for geo_id
    pub geo_id_field: Option<String>,
}

impl Default for ShapefileFieldMapping {
    fn default() -> Self {
        Self {
            id_field: "DISTRICT".to_string(),
            name_field: None,
            geo_id_field: None,
        }
    }
}

/// Upload a new map via multipart form data.
pub struct UploadMap {
    name: String,
    file_data: Vec<u8>,
    file_name: String,
    field_mapping: Option<ShapefileFieldMapping>,
}

impl UploadMap {
    pub fn new(name: impl Into<String>, file_name: impl Into<String>, file_data: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            file_name: file_name.into(),
            file_data,
            field_mapping: None,
        }
    }

    pub fn read(name: impl Into<String>, path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();

        let file_name = path
            .file_name()
            .context("Path is missing file name!")?
            .to_string_lossy()
            .into_owned();

        let bytes = std::fs::read(path)?;

        Ok(Self::new(name, file_name, bytes))
    }

    pub fn with_mapping(mut self, field_mapping: ShapefileFieldMapping) -> Self {
        self.field_mapping = Some(field_mapping);
        self
    }
}

impl Handler for UploadMap {
    type ResponseBody = MapUploadResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/maps".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        let mut form = MultipartForm::new().add_text("name", &self.name).add_part(
            "file",
            Part::bytes(self.file_data.to_vec()).file_name(&self.file_name),
        );

        if let Some(mapping) = &self.field_mapping
            && let Ok(serialized) = serde_json::to_string(mapping)
        {
            form = form.add_text("field_mapping", serialized);
        }

        builder.multipart(form)
    }
}

/// Contains a map id and name
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MapView {
    pub id: i32,
    pub name: String,
}

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
