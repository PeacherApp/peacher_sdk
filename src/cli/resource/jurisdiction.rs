use crate::cli::fmt::{self, AsTable};
use crate::prelude::*;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum JurisdictionCmd {
    /// Lists the jurisdictions used by the client and its implementation
    List,
    /// Syncs the jurisdiction from the external client
    Sync,
}

impl JurisdictionCmd {
    pub async fn run<'p, E, P>(self, mut sync: ApiSync<'p, E, P>) -> Result<()>
    where
        E: ExternalClient,
        P: Client,
    {
        match self {
            JurisdictionCmd::List => {
                let ext_jurisdiction = sync.external().get_jurisdiction();
                let existing_jurisdictions = ListJurisdictions(
                    JurisdictionParams::default()
                        .with_external_id(ext_jurisdiction.external_id.val_str()),
                )
                .request(sync.peacher())
                .await?;
                existing_jurisdictions.data.print();
                Ok(())
            }
            JurisdictionCmd::Sync => {
                let spinner = fmt::spinner("Syncing jurisdiction and chambers...");

                let result = sync.jurisdiction().sync().await?;
                fmt::spinner_success(&spinner, "Sync complete");
                result.print();
                Ok(())
            }
        }
    }
}
