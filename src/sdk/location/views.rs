use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DistrictRepresentative {
    pub district: DistrictView,
    pub member: RepresentativeMember,
    pub session: SessionView,
    pub chamber: GetChamberView,
    pub is_following: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViewerIntersectionResponse {
    pub location: ViewerLocationResponse,
    pub representatives: Vec<DistrictRepresentative>,
    pub map: GeoJson<DistrictIntersectionInfo>,
}

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
// pub struct ViewerIntersectionReponse {
//     pub location: ViewerLocationResponse,
//     pub maps: Vec<PoliticalIntersectionMap>,
//     /// Note that this data are also embedded within the maps. However,
//     /// we duplicate the data here, as it's much easier to work with/worth the cost.
//     pub intersections: Vec<BoundaryView>,
// }

impl ViewerIntersectionResponse {
    /// count the number of representatives in this response (just `self.representatives.len`)
    pub fn num_representatives(&self) -> usize {
        self.representatives.len()
    }
    /// get the representatives in the representative field
    pub fn representatives(&self) -> impl Iterator<Item = &RepresentativeMember> {
        self.representatives.iter().map(|r| &r.member)
    }

    /// get the number of geojson members (which should always equal [`Self::num_representatives`])
    pub fn num_geojson_members(&self) -> usize {
        self.map.iter_props().map(|prop| prop.num_members()).sum()
    }
    /// iterate through the geojson members (which should always yield the same number of members as [`Self::representatives`])
    pub fn geojson_members(&self) -> impl Iterator<Item = &RepresentativeMember> {
        self.map.iter_props().flat_map(|prop| prop.members())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DistrictIntersectionInfo {
    /// the district of the original map this intersection corresponds with
    pub id: i32,
    /// the name of the district
    pub name: String,
    /// The jurisdictions that are represented by this boundary
    pub intersecting_jurisdictions: Vec<JurisdictionIntersection>,
}
impl DistrictIntersectionInfo {
    pub fn num_members(&self) -> usize {
        self.intersecting_jurisdictions
            .iter()
            .map(|j| j.num_members())
            .sum()
    }
    pub fn members(&self) -> impl Iterator<Item = &RepresentativeMember> {
        self.intersecting_jurisdictions
            .iter()
            .flat_map(|j| j.members())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JurisdictionIntersection {
    pub jurisdiction: JurisdictionView,
    /// The chambers that intersect with this boundary
    pub intersecting_chambers: Vec<ChamberIntersection>,
}
impl JurisdictionIntersection {
    pub fn num_members(&self) -> usize {
        self.intersecting_chambers
            .iter()
            .map(|c| c.num_members())
            .sum()
    }
    pub fn members(&self) -> impl Iterator<Item = &RepresentativeMember> {
        self.intersecting_chambers
            .iter()
            .flat_map(|mem| mem.members())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChamberIntersection {
    /// The chamber intersected with
    pub chamber: ChamberView,
    /// The representatives that have represented this district
    pub intersecting_representatives: Vec<RepresentativeMember>,
}
impl ChamberIntersection {
    pub fn num_members(&self) -> usize {
        self.intersecting_representatives.len()
    }
    pub fn members(&self) -> impl Iterator<Item = &RepresentativeMember> {
        self.intersecting_representatives.iter()
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
