use crate::{cli::fmt, prelude::*};
use anyhow::{Context, Result};
use clap::{Subcommand, ValueEnum};

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum SyncType {
    /// Sync only members
    Members,
    /// Sync only legislation
    Legislation {
        /// Also sync the detailed information of the legislation
        #[arg(short, long, default_value_t = true)]
        details: bool,
    },
    /// Sync both members and legislation
    All {
        #[arg(short, long, default_value_t = true)]
        details: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum SessionAction {
    Delete,
    Get,
    Sync {
        /// Sync only a specific chamber. The default behavior
        ///
        /// will sync all members/legislation of all chambers of this session.
        #[arg(long)]
        chamber_id: Option<i32>,
        #[command(subcommand)]
        sync_type: SyncType,
    },
}

impl SessionAction {
    pub async fn run<'p, E, P>(self, session_id: i32, mut sync: ApiSync<'p, E, P>) -> Result<()>
    where
        E: ExternalClient,
        P: Client,
    {
        match self {
            SessionAction::Delete => {
                let spinner = fmt::spinner(format!("Deleting session {session_id}"));
                DeleteSession(session_id).request(sync.peacher()).await?;
                fmt::spinner_success(&spinner, "Sync complete");
                Ok(())
            }
            SessionAction::Get => {
                let session = GetSession(session_id).request(sync.peacher()).await?;
                session.print();
                Ok(())
            }
            SessionAction::Sync {
                chamber_id,
                sync_type,
            } => {
                let session = GetSession(session_id).request(sync.peacher()).await?;
                let session_ext_id = session
                    .external
                    .context("The provided session id does not have external associated data!")?
                    .external_id;

                let chambers = match chamber_id {
                    Some(value) => {
                        let chamber = GetChamber(value).request(sync.peacher()).await?;
                        let chamber_ext_id = chamber.external.map(|v| v.external_id).context(
                            "The provided chamber id does not have external associated data!",
                        )?;
                        vec![(chamber.id, chamber_ext_id, chamber.name)]
                    }
                    None => session
                        .chambers
                        .into_iter()
                        .filter_map(|c| {
                            let chamber_ext_id = c.external?.external_id;
                            Some((c.chamber_id, chamber_ext_id, c.chamber_name))
                        })
                        .collect(),
                };

                for (chamber_id, chamber_ext_id, chamber_name) in chambers {
                    match sync_type {
                        SyncType::All { .. } | SyncType::Members => {
                            match sync
                                .sync_members(
                                    session_id,
                                    &session_ext_id,
                                    chamber_id,
                                    &chamber_ext_id,
                                )
                                .await
                            {
                                Ok(result) => {
                                    result.print();
                                }
                                Err(e) => {
                                    println!(
                                        "Something happened when syncing Chamber {}: {e}",
                                        chamber_name
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }

                match sync_type {
                    SyncType::All { details } | SyncType::Legislation { details } => {
                        let result = match sync.sync_legislation(session_id, &session_ext_id).await
                        {
                            Ok(result) => {
                                result.print();
                                result
                            }
                            Err(e) => {
                                println!("Something happened when syncing legislation: {e}");
                                return Ok(());
                            }
                        };

                        if sync.config().get_legislation_has_details && !details {
                            println!(
                                "{}",
                                fmt::yellow(
                                    "Legislation has been synced, but some legislative items need additional information."
                                )
                            );
                        } else if !sync.config().get_legislation_has_details {
                            return Ok(());
                        }

                        for legislation in result.created.into_iter().chain(result.updated) {
                            if let Some(external) = legislation.external {
                                match sync.sync_votes(legislation.id, &external.external_id).await {
                                    Ok(result) => {
                                        result.print();
                                    }
                                    Err(e) => {
                                        println!(
                                            "Something happened when syncing votes for legislation {} ({}): {e}",
                                            legislation.name_id, legislation.id
                                        );
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }

                Ok(())
            }
        }
    }
}
