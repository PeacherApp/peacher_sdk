use std::borrow::Cow;

use crate::peanut::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Get the viewer's current location
pub struct GetLocation;

impl GetHandler for GetLocation {
    type ResponseBody = ViewerLocationResponse;

    fn path(&self) -> Cow<'_, str> {
        "/api/location".into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum SetLocation {
    Address(String),
    Coords(LocationRequestCoords),
}
impl SetLocation {
    pub fn addr(address: impl Into<String>) -> Self {
        Self::Address(address.into())
    }
    pub fn coords(lat: f64, lng: f64) -> Self {
        Self::Coords(LocationRequestCoords { lat, lng })
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LocationRequestCoords {
    pub lat: f64,
    pub lng: f64,
}

impl Handler for SetLocation {
    type ResponseBody = ViewerLocationResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/location".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(self)
    }
}

/// Unset/clear the viewer's location
pub struct UnsetLocation;

impl Handler for UnsetLocation {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/location".into()
    }
}

/// Get detailed location information with map intersections
pub struct GetLocationDetails;

impl GetHandler for GetLocationDetails {
    type ResponseBody = ViewerIntersectionReponse;

    fn path(&self) -> Cow<'_, str> {
        "/api/location/details".into()
    }
}

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
    pub maps: Vec<PoliticalIntersectionMap>,
    /// Note that this data are also embedded within the maps. However,
    /// we duplicate the data here, as it's much easier to work with/worth the cost.
    pub intersections: Vec<BoundaryView>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Intersections {
    inner: Vec<PoliticalIntersectionMap>,
}
impl Intersections {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }
    pub fn extend(&mut self, views: Vec<PoliticalIntersectionMap>) {
        self.inner.extend(views);
    }
}

/// Contains data about all intersections, either
/// districts with political data, or districts without political data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LocationIntersectionResponse {
    pub intersections: Vec<PoliticalIntersectionMap>,
}

/// This is an intersection with some map type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PoliticalIntersectionMap {
    pub map: MapView,
    pub boundaries: GeoJson<BoundaryView>,
}
