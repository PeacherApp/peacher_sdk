pub mod session;

use crate::cli::fmt::{self, AsTable};
use crate::cli::resource::session::SessionAction;
use crate::prelude::*;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SessionCmd {
    /// Lists the known sessions by Peacher
    List,
    /// Syncs the API's known sessions from the external client.
    ///
    /// Only sync the existance of sessions. Syncing properties of sessions requires
    /// needs individual enumeration
    Sync,

    /// Do something with a specific session
    Id {
        id: String,
        #[command(subcommand)]
        action: SessionAction,
    },
}

impl SessionCmd {
    pub async fn run<'p, E, P>(self, mut sync: ApiSync<'p, E, P>) -> Result<()>
    where
        E: ExternalClient,
        P: Client,
    {
        let jurisdiction = sync.jurisdiction().get().await?;
        match self {
            SessionCmd::List => {
                let val = sync.sessions().list().await?;

                val.data.print();
                Ok(())
            }
            SessionCmd::Sync => {
                let spinner = fmt::spinner("Syncing sessions...");
                let result = sync.sessions().sync_sessions().await?;
                fmt::spinner_success(&spinner, "Sync complete");

                result.print();

                Ok(())
            }
            SessionCmd::Id { id, action } => action.run(id, sync).await,
        }
    }
}
