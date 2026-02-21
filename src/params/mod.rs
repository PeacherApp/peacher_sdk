use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

mod comma_separated;
pub use comma_separated::*;

/// The order in which the query should be returned
#[derive(
    Serialize, Debug, PartialEq, Eq, Deserialize, Default, Clone, Copy, EnumString, Display,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Ordering {
    /// Ascending order
    Asc,
    /// Descending order
    #[default]
    Desc,
}
