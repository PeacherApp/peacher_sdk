mod config;
pub use config::*;

mod jurisdiction;
pub use jurisdiction::*;

mod sessions;
pub use sessions::*;

use anyhow::Result;
use clap::Subcommand;

use crate::sync::{ApiSync, ExternalClient};

#[derive(Subcommand, Debug)]
pub enum Resource {
    /// Commands to modify your config
    Config {
        #[command(subcommand)]
        cmd: ConfigCmd,
    },
    /// Commands to list and sync chambers according to your external client implementation
    Jurisdiction {
        #[command(subcommand)]
        cmd: JurisdictionCmd,
    },

    /// Commands for managing your sessions
    Sessions {
        #[command(subcommand)]
        cmd: SessionCmd,
    },
}
