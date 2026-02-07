use serde::{Deserialize, Serialize};

use crate::views::{BoundaryView, GeoJson, MapView};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViewerLocationResponse {
    pub lat: f64,
    pub lng: f64,
    pub name: String,
}
impl ViewerLocationResponse {
    pub fn name_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViewerIntersectionReponse {
    pub location: ViewerLocationResponse,
    pub intersections: Vec<IntersectionView>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Intersections {
    inner: Vec<IntersectionView>,
}
impl Intersections {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }
    pub fn extend(&mut self, views: Vec<IntersectionView>) {
        self.inner.extend(views);
    }
}

/// Contains data about all intersections, either
/// districts with political data, or districts without political data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LocationIntersectionResponse {
    pub intersections: Vec<IntersectionView>,
}

/// This is an intersection with some map type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct IntersectionView {
    pub map: MapView,
    pub boundaries: GeoJson<BoundaryView>,
}
