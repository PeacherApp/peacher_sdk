pub mod props_iter;

use serde::{Deserialize, Serialize};

use crate::geojson::props_iter::RefPropsIter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(untagged)]
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
            features: features.into_iter().collect(),
        })
    }

    pub fn iter_props(&self) -> RefPropsIter<'_, T> {
        match self {
            Self::Feature(feature) => RefPropsIter::one(&feature.properties),
            Self::FeatureCollection(collection) => RefPropsIter::many(collection),
        }
    }

    pub fn num_features(&self) -> usize {
        match self {
            Self::Feature(_) => 1,
            Self::FeatureCollection(collection) => collection.features.len(),
        }
    }

    pub fn map_props<F, U>(self, mut func: F) -> GeoJson<U>
    where
        F: FnMut(T) -> U,
    {
        match self {
            GeoJson::Feature(features) => {
                let props = func(features.properties);

                GeoJson::Feature(GeoJsonFeature {
                    geometry: features.geometry,
                    properties: props,
                })
            }
            GeoJson::FeatureCollection(collection) => {
                let collection = collection.features.into_iter().map(|feature| {
                    let props = func(feature.properties);
                    GeoJsonFeature {
                        geometry: feature.geometry,
                        properties: props,
                    }
                });
                GeoJson::FeatureCollection(GeoJsonFeatureCollection {
                    features: collection.collect(),
                })
            }
        }
    }
}

/// This is a GeoJSON feature. Perfectly fine as a GeoJSON itself.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename = "Feature")]
pub struct GeoJsonFeature<T> {
    //can have id
    pub geometry: Geometry,
    pub properties: T,
}

#[cfg(feature = "utoipa")]
fn build_geojson_feature_schema<T: utoipa::PartialSchema>()
-> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
    use utoipa::openapi::schema::Type;
    utoipa::openapi::ObjectBuilder::new()
        .property(
            "type",
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(Type::String)
                .enum_values::<_, &str>(Some(["Feature"])),
        )
        .required("type")
        .property("geometry", <Geometry as utoipa::PartialSchema>::schema())
        .property("properties", T::schema())
        .description(Some("A GeoJSON Feature"))
        .into()
}

#[cfg(feature = "utoipa")]
impl<T: utoipa::ToSchema> utoipa::ToSchema for GeoJsonFeature<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        "GeoJsonFeature".into()
    }

    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        T::schemas(schemas);
        // Register the concrete feature schema so $ref works
        let schema_name = format!("{}{}", Self::name(), T::name());
        schemas.push((schema_name, build_geojson_feature_schema::<T>()));
    }
}

/// Returns a `$ref` to the named schema (registered via `schemas()`), so that
/// containers like `Vec<GeoJsonFeature<T>>` produce `Array<$ref>` instead of inlining.
///
/// We must manually perform this implementation since `utoipa` does not support
/// internally tagged structs (serde(tag))
#[cfg(feature = "utoipa")]
impl<T: utoipa::ToSchema> utoipa::__dev::ComposeSchema for GeoJsonFeature<T> {
    fn compose(
        _generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let schema_name = format!("{}{}", <Self as utoipa::ToSchema>::name(), T::name());
        utoipa::openapi::Ref::from_schema_name(schema_name).into()
    }
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
#[serde(tag = "type", rename = "FeatureCollection")]
pub struct GeoJsonFeatureCollection<T> {
    pub features: Vec<GeoJsonFeature<T>>,
}

#[cfg(feature = "utoipa")]
impl<T: utoipa::ToSchema> utoipa::ToSchema for GeoJsonFeatureCollection<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        "GeoJsonFeatureCollection".into()
    }

    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        T::schemas(schemas);
    }
}

/// We must manually perform this implementation since `utoipa` does not support
/// internally tagged structs (serde(tag))
#[cfg(feature = "utoipa")]
impl<T: utoipa::ToSchema> utoipa::__dev::ComposeSchema for GeoJsonFeatureCollection<T> {
    fn compose(
        _generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        use utoipa::openapi::schema::Type;
        utoipa::openapi::ObjectBuilder::new()
            .property(
                "type",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(Type::String)
                    .enum_values::<_, &str>(Some(["FeatureCollection"])),
            )
            .required("type")
            .property(
                "features",
                <Vec<GeoJsonFeature<T>> as utoipa::PartialSchema>::schema(),
            )
            .description(Some("A GeoJSON Feature Collection"))
            .into()
    }
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
