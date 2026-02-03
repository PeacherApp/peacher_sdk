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
        id: i32,
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
        let jurisdiction_id = sync.resolve_internal_jurisdiction().await?;
        match self {
            SessionCmd::List => {
                let sessions =
                    ListSessions(SessionParams::default().with_jurisdiction(jurisdiction_id))
                        .request(sync.peacher())
                        .await?;

                sessions.data.print();
                Ok(())
            }
            SessionCmd::Sync => {
                let spinner = fmt::spinner("Syncing sessions...");
                let result = sync.sync_sessions(jurisdiction_id).await?;
                fmt::spinner_success(&spinner, "Sync complete");

                result.print();

                Ok(())
            }
            SessionCmd::Id { id, action } => action.run(id, sync).await,
        }
    }
}
