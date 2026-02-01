use crate::cli::ResolvedConfig;
use crate::cli::config::CliConfig;
use crate::prelude::*;
use anyhow::Result;
use clap::Subcommand;
use dialoguer::{Confirm, Input};

#[derive(Subcommand, Debug)]
pub enum CreateResource {
    /// Create a new jurisdiction interactively
    Jurisdiction,
}

pub async fn run(
    resource: CreateResource,
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
        CreateResource::Jurisdiction => create_jurisdiction(peacher).await,
    }
}

async fn create_jurisdiction(peacher: &PeacherClient) -> Result<()> {
    println!("Creating a new jurisdiction...\n");

    // 1. Prompt for jurisdiction fields
    let name: String = Input::new()
        .with_prompt("Jurisdiction name")
        .interact_text()?;

    let external_id: String = Input::new()
        .with_prompt("External ID (e.g., state abbreviation like 'GA')")
        .interact_text()?;

    let url: String = Input::new()
        .with_prompt("URL (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;
    let url = if url.is_empty() { None } else { Some(url) };

    // 2. Prompt for chambers (loop until user is done)
    let mut chambers = Vec::new();
    loop {
        let add_chamber = if chambers.is_empty() {
            Confirm::new()
                .with_prompt("Add a chamber?")
                .default(true)
                .interact()?
        } else {
            Confirm::new()
                .with_prompt("Add another chamber?")
                .default(false)
                .interact()?
        };

        if !add_chamber {
            break;
        }

        let chamber_name: String = Input::new().with_prompt("Chamber name").interact_text()?;

        let chamber_external_id: String = Input::new()
            .with_prompt("Chamber external ID")
            .interact_text()?;

        let chamber_url: String = Input::new()
            .with_prompt("Chamber URL (optional, press Enter to skip)")
            .allow_empty(true)
            .interact_text()?;
        let chamber_url = if chamber_url.is_empty() {
            None
        } else {
            Some(chamber_url)
        };

        chambers.push(CreateChamber {
            name: chamber_name,
            external_id: ExternalId::new(chamber_external_id),
            url: chamber_url,
        });

        println!("  Added chamber: {}", chambers.last().unwrap().name);
    }

    // 3. Build and send request
    let req = CreateJurisdiction {
        name: name.clone(),
        external_id: ExternalId::new(external_id),
        url,
        chambers,
    };

    println!("\nCreating jurisdiction '{}'...", name);
    todo!()
    // let jurisdiction = peacher.request(&req).await?;

    // println!(
    //     "\nSuccessfully created jurisdiction '{}' (ID: {})",
    //     jurisdiction.name, jurisdiction.id
    // );

    // // 4. Show chambers if any
    // if !jurisdiction.chambers.is_empty() {
    //     println!("Chambers:");
    //     for chamber in &jurisdiction.chambers {
    //         println!("  - {} (ID: {})", chamber.name, chamber.id);
    //     }
    // }

    // // 5. Offer to set as default
    // let set_default = Confirm::new()
    //     .with_prompt(format!(
    //         "Set jurisdiction '{}' as your default?",
    //         jurisdiction.name
    //     ))
    //     .default(true)
    //     .interact()?;

    // if set_default {
    //     let mut config = CliConfig::load().unwrap_or_default();
    //     config.jurisdiction_id = Some(jurisdiction.id);
    //     config.save()?;
    //     println!(
    //         "Default jurisdiction set to {} (ID: {})",
    //         jurisdiction.name, jurisdiction.id
    //     );
    // }

    //Ok(())
}
