use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BoundingBox {
    pub min: Vec2,
    pub max: Vec2,
}
impl BoundingBox {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);

        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);

        Self {
            min: Vec2 { x: min_x, y: min_y },
            max: Vec2 { x: max_x, y: max_y },
        }
    }
    pub fn from_nominatim_bb(bounding_box: [f32; 4]) -> Self {
        let p1 = Vec2 {
            x: bounding_box[2] as f64,
            y: bounding_box[0] as f64,
        };
        let p2 = Vec2 {
            x: bounding_box[3] as f64,
            y: bounding_box[1] as f64,
        };
        Self::new(p1, p2)
    }

    /// Expand this bounding box to also contain `other`.
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: Vec2 {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
            },
            max: Vec2 {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
            },
        }
    }
}
