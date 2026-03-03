use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DistrictRepresentative {
    pub district: DistrictView,
    pub member: RepresentativeMember,
    pub session: SessionView,
    pub chamber: GetChamberView,
}

pub struct NewViewerIntersectionResponse {
    pub representatives: Vec<DistrictRepresentative>,
    pub map: GeoJson<DistrictIntersectionInfo>,
}

impl NewViewerIntersectionResponse {
    pub fn num_map_members(&self) -> usize {
        todo!()
        //self.map.iter
    }
}

pub struct DistrictIntersectionInfo {
    /// the original map this intersection corresponds with
    pub map_id: i32,
    /// the district of the original map this intersection corresponds with
    pub district_id: i32,
    /// the name of the district
    pub name: String,
    /// The jurisdictions that are represented by this boundary
    pub intersecting_jurisdictions: Vec<JurisdictionIntersection>,
}
pub struct JurisdictionIntersection {
    pub jurisdiction: JurisdictionView,
    /// The chambers that intersect with this boundary
    pub intersecting_chambers: Vec<ChamberIntersection>,
}
pub struct ChamberIntersection {
    /// The chamber intersected with
    pub chamber: ChamberView,
    /// The representatives that have represented this district
    pub intersecting_representatives: Vec<RepresentativeMember>,
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
    pub map: SmallMapView,
    pub boundaries: GeoJson<BoundaryView>,
}
