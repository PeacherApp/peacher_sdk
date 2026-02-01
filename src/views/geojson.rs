use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type")]
pub enum GeoJsonFeatureVariant<T> {
    Feature(GeoJsonFeature<T>),
}

impl<T> From<GeoJsonFeature<T>> for GeoJsonFeatureVariant<T> {
    fn from(value: GeoJsonFeature<T>) -> Self {
        GeoJsonFeatureVariant::Feature(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type")]
pub enum GeoJson<T> {
    Feature(GeoJsonFeature<T>),
    FeatureCollection(GeoJsonFeatureCollection<T>),
}
impl<T> GeoJson<T> {
    pub fn one(feature: GeoJsonFeature<T>) -> Self {
        Self::Feature(feature)
    }
    pub fn many(features: impl IntoIterator<Item = GeoJsonFeature<T>>) -> Self {
        Self::FeatureCollection(GeoJsonFeatureCollection {
            features: features
                .into_iter()
                .map(GeoJsonFeatureVariant::Feature)
                .collect(),
        })
    }
}

/// This is a GeoJSON feature. Perfectly fine as a GeoJSON itself.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GeoJsonFeature<T> {
    //can have id
    pub geometry: Geometry,
    pub properties: T,
}

impl<T> GeoJsonFeature<T> {
    pub fn new(geometry: impl Into<Geometry>, properties: T) -> Self {
        Self {
            geometry: geometry.into(),
            properties,
        }
    }
}

impl<T> From<GeoJsonFeature<T>> for GeoJson<T> {
    fn from(value: GeoJsonFeature<T>) -> Self {
        GeoJson::Feature(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GeoJsonFeatureCollection<T> {
    pub features: Vec<GeoJsonFeatureVariant<T>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "coordinates")]
pub enum Geometry {
    MultiPolygon(Vec<Vec<Vec<Vec<f64>>>>),
    Polygon(Vec<Vec<Vec<f64>>>),
    // Point(Vec<f64>),
    // MultiPoint(Vec<Vec<f64>>),
    // LineString(Vec<Vec<f64>>),
    // MultiLineString(Vec<LineStringType>),
    // Polygon(Vec<Vec<f64>>),
    // GeometryCollection(Vec<Geometry>),
}

#[cfg(feature = "geo")]
impl From<geo::Geometry> for Geometry {
    fn from(value: geo::Geometry) -> Self {
        let value = geojson::Value::from(&value);
        match value {
            geojson::Value::MultiPolygon(p) => Geometry::MultiPolygon(p),
            geojson::Value::Polygon(p) => Geometry::Polygon(p),
            _ => panic!("hit unimplmeneted conversion"),
        }
    }
}
