pub mod client;
pub mod paginate;
pub mod params;
pub mod peanut;
pub mod requests;
pub mod sdk;
pub mod sync;
pub mod types;
pub mod views;

// #[cfg(feature = "cli")]
// pub mod cli;
// #[cfg(feature = "cli")]
// pub use cli::{cli, cli_with_client};

pub mod prelude {
    pub use crate::client::*;
    pub use crate::paginate::*;
    pub use crate::params::*;
    pub use crate::peanut::prelude::*;
    pub use crate::requests::*;
    pub use crate::sync::*;
    pub use crate::types::*;
    pub use crate::views::*;
}
