#![allow(unused)]

pub mod args;
pub mod fmt;
pub mod resource;

use std::path::PathBuf;

use crate::{
    cli::resource::{Config, ConfigOptions, Resource},
    client::PeacherClient,
    sync::{ApiSync, ExternalClient},
};
use anyhow::Result;
use clap::Parser;
use url::Url;

/// The Peacher SDK CLI tool for external clients
#[derive(Parser, Debug)]
#[command(version, about = "Peacher SDK CLI - Sync legislative data", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    resource: Resource,

    /// Override the default path of the config
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// Override your api key
    #[arg(short, long, global = true)]
    api_key: Option<String>,

    /// Override the API base URL (default: from config or https://api.peacher.app)
    #[arg(short, long, global = true)]
    url: Option<String>,
}

#[derive(Default)]
pub struct CliOpts {
    /// Override the default path of the config
    pub config: Option<String>,

    /// Override your api key
    pub api_key: Option<String>,

    /// Override the API base URL (default: from config or https://api.peacher.app)
    pub url: Option<String>,
}

pub async fn cli<E: ExternalClient>(client: E) -> Result<()> {
    cli_with_opts(client, Default::default()).await
}

pub async fn cli_with_opts<E: ExternalClient>(client: E, options: CliOpts) -> Result<()> {
    let mut args = Args::parse();

    if args.config.is_none()
        && let Some(config) = options.config
    {
        args.config = Some(config);
    }

    if args.api_key.is_none()
        && let Some(api_key) = options.api_key
    {
        args.api_key = Some(api_key);
    }
    if args.config.is_none()
        && let Some(url) = options.url
    {
        args.url = Some(url);
    }

    if let Resource::Config { cmd } = args.resource {
        let override_config_path = args.config.map(PathBuf::from);

        return cmd.run(override_config_path);
    }

    let mut config = match args.config {
        Some(path) => Config::load_from_path(path.into())?,
        None => Config::load()?,
    };
    if let Some(api_key) = args.api_key {
        config.options.api_key = Some(api_key);
    }
    if let Some(url) = args.url {
        let url = Url::parse(&url)?;
        config.options.url = url;
    }
    let peacher_client = PeacherClient {
        base: config.options.url,
        api_key: config.options.api_key,
        ..Default::default()
    };

    let api_sync = ApiSync::new(client, &peacher_client);

    match args.resource {
        Resource::Jurisdiction { cmd } => cmd.run(api_sync).await,
        Resource::Sessions { cmd } => cmd.run(api_sync).await,
        Resource::Config { cmd } => unreachable!(),
    }
}

pub async fn cli_with_client<E: ExternalClient>(
    external_client: E,
    peacher_client: &PeacherClient,
    args: Args,
) -> anyhow::Result<()> {
    // let args = Args::parse();

    // let api_sync = ApiSync::new(external_client, peacher_client);

    // args.run(api_sync);

    todo!()
}
