use crate::cli::ResolvedConfig;
use crate::prelude::*;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ListResource {
    /// List jurisdictions for the authenticated account
    Jurisdictions,
    /// List sessions for a jurisdiction
    Sessions {
        /// Jurisdiction ID (uses default from config if not specified)
        #[arg(short, long)]
        jurisdiction: Option<i32>,
    },
    /// List chambers for a jurisdiction
    Chambers {
        /// Jurisdiction ID (uses default from config if not specified)
        #[arg(short, long)]
        jurisdiction: Option<i32>,
    },
}

pub async fn run(
    resource: ListResource,
    resolved: &ResolvedConfig,
    injected_client: Option<&PeacherClient>,
) -> Result<()> {
    // Use injected client if provided, otherwise create from config
    let owned_client;
    let peacher: &PeacherClient = match injected_client {
        Some(client) => client,
        None => {
            owned_client = resolved.to_peacher();
            &owned_client
        }
    };

    match resource {
        ListResource::Jurisdictions => list_jurisdictions(peacher).await,
        ListResource::Sessions { jurisdiction } => {
            let jid = jurisdiction.or(resolved.jurisdiction_id).ok_or_else(|| {
                anyhow::anyhow!(
                    "No jurisdiction specified. Use --jurisdiction or set default with: config set jurisdiction <id>"
                )
            })?;
            list_sessions(peacher, jid).await
        }
        ListResource::Chambers { jurisdiction } => {
            let jid = jurisdiction.or(resolved.jurisdiction_id).ok_or_else(|| {
                anyhow::anyhow!(
                    "No jurisdiction specified. Use --jurisdiction or set default with: config set jurisdiction <id>"
                )
            })?;
            list_chambers(peacher, jid).await
        }
    }
}

async fn list_jurisdictions(peacher: &PeacherClient) -> Result<()> {
    todo!()
    // let jurisdictions = peacher.request(&GetAccountJurisdictions).await?;

    // if jurisdictions.is_empty() {
    //     println!("No jurisdictions found for this account.");
    //     return Ok(());
    // }

    // println!("{:<6} {:<30} {:<20}", "ID", "NAME", "EXTERNAL_ID");
    // println!("{}", "-".repeat(60));

    // for j in jurisdictions {
    //     let ext_id = j
    //         .external
    //         .as_ref()
    //         .map(|e| e.external_id.to_string())
    //         .unwrap_or_else(|| "-".to_string());
    //     println!("{:<6} {:<30} {:<20}", j.id, j.name, ext_id);
    // }

    // Ok(())
}

async fn list_sessions(peacher: &PeacherClient, jurisdiction_id: i32) -> Result<()> {
    todo!()
    // let sessions = peacher
    //     .request(&ListJurisdictionSessions(jurisdiction_id))
    //     .await?;

    // if sessions.is_empty() {
    //     println!("No sessions found for jurisdiction {}.", jurisdiction_id);
    //     return Ok(());
    // }

    // println!(
    //     "{:<6} {:<30} {:<10} {:<20}",
    //     "ID", "NAME", "CURRENT", "EXTERNAL_ID"
    // );
    // println!("{}", "-".repeat(70));

    // for s in sessions {
    //     let ext_id = s
    //         .external
    //         .as_ref()
    //         .map(|e| e.external_id.to_string())
    //         .unwrap_or_else(|| "-".to_string());
    //     let current = if s.current { "yes" } else { "no" };
    //     println!("{:<6} {:<30} {:<10} {:<20}", s.id, s.name, current, ext_id);
    // }

    // Ok(())
}

async fn list_chambers(peacher: &PeacherClient, jurisdiction_id: i32) -> Result<()> {
    todo!()
    // Get jurisdiction details which include chambers
    // let jurisdictions = peacher.request(&GetAccountJurisdictions).await?;
    // let jurisdiction = jurisdictions
    //     .into_iter()
    //     .find(|j| j.id == jurisdiction_id)
    //     .ok_or_else(|| anyhow::anyhow!("Jurisdiction {} not found", jurisdiction_id))?;

    // if jurisdiction.chambers.is_empty() {
    //     println!("No chambers found for jurisdiction {}.", jurisdiction_id);
    //     return Ok(());
    // }

    // println!("{:<6} {:<30}", "ID", "NAME");
    // println!("{}", "-".repeat(40));

    // for c in jurisdiction.chambers {
    //     println!("{:<6} {:<30}", c.id, c.name);
    // }

    //Ok(())
}
