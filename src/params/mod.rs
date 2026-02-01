mod address;
pub use address::*;

mod chamber;
pub use chamber::*;

mod jurisdiction;
pub use jurisdiction::*;

mod legislation;
pub use legislation::*;

mod sponsorships;
pub use sponsorships::*;

mod votes;
pub use votes::*;

mod member;
pub use member::*;

mod feed;
pub use feed::*;

mod session;
pub use session::*;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

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
