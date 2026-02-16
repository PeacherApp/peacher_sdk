pub mod address;
pub mod chamber;
pub mod client;
pub mod feed;
pub mod jurisdiction;
pub mod legislation;
pub mod member;
pub mod paginate;
pub mod params;
pub mod peanut;
pub mod requests;
pub mod session;
pub mod sponsorships;
pub mod sync;
pub mod views;
pub mod votes;

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "cli")]
pub use cli::{cli, cli_with_client};

pub mod prelude {
    pub use crate::client::*;
    pub use crate::paginate::*;
    pub use crate::params::*;
    pub use crate::peanut::prelude::*;
    pub use crate::requests::*;
    pub use crate::sync::*;
    pub use crate::views::*;

    pub use crate::address::*;
    pub use crate::chamber::*;
    pub use crate::feed::*;
    pub use crate::jurisdiction::*;
    pub use crate::legislation::*;
    pub use crate::member::*;
    pub use crate::session::*;
    pub use crate::sponsorships::*;
    pub use crate::votes::*;
}
