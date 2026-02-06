use crate::{cli::fmt, prelude::*};
use anyhow::{Context, Result};
use clap::{Subcommand, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Details {
    /// Only fetch Legislation from the endpoint, but don't look for details
    None,
    /// fetch Legislation from the endpoint, and sync details by calling each
    /// legislative item individually
    ///
    /// Note: this will not apply if your config does not identify `get_legislation`
    /// as having more information
    All,

    /// Fetch legislation details from the endpoint only for legislation peacher knows about.
    OnlyKnownLegislation,
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum SyncType {
    /// Sync only members
    Members,
    /// Sync only legislation
    Legislation,
    /// Sync both members and legislation
    All,
}

#[derive(Subcommand, Debug)]
pub enum SessionAction {
    /// Delete this session. May fail if you already have members and/or legislation associated
    /// with this session
    Delete,
    /// Details about this session
    Get,
    /// Sync data from a specific chamber or all chambers for this session
    Sync {
        /// Sync only a specific chamber. The default behavior
        ///
        /// will sync all members/legislation of all chambers of this session.
        #[arg(long)]
        chamber: Option<String>,
        #[command(subcommand)]
        sync_type: SyncType,
    },
}

impl SessionAction {
    pub async fn run<'p, E, P>(self, session_id: String, mut sync: ApiSync<'p, E, P>) -> Result<()>
    where
        E: ExternalClient,
        P: Client,
    {
        let id = ExternalId::new(&session_id);
        let mut session = sync.sessions().session(&id);

        match self {
            SessionAction::Delete => {
                let spinner = fmt::spinner(format!("Deleting session {session_id}"));
                session.delete().await?;
                fmt::spinner_success(&spinner, "Sync complete");
                Ok(())
            }
            SessionAction::Get => {
                let session = session.get().await?;
                session.print();
                Ok(())
            }
            SessionAction::Sync { chamber, sync_type } => {
                match sync_type {
                    SyncType::All | SyncType::Members => match chamber.as_deref() {
                        Some(id) => {
                            let id = ExternalId::new(id);
                            let result = session.members(&id).sync().await?;
                            result.print()
                        }
                        None => {
                            let results = session.sync_all_members().await?;
                            for result in results {
                                result.print();
                            }
                        }
                    },
                    SyncType::Legislation => {}
                }

                match sync_type {
                    SyncType::All | SyncType::Legislation => {
                        let value = session.legislation().sync().await?;
                        value.print();
                    }
                    SyncType::Members => {}
                }

                Ok(())
            }
        }
    }
}
