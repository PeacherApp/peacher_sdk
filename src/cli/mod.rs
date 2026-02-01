use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};
use url::Url;

mod commands;
mod config;
mod test_client;

pub use config::CliConfig;
pub use test_client::*;

use crate::prelude::*;

pub use commands::config_cmd::ConfigCmd;
pub use commands::create_cmd::CreateResource;
pub use commands::list_cmd::ListResource;
pub use commands::sync_cmd::SyncTarget;

/// The Peacher SDK CLI tool for external clients
#[derive(Parser, Debug)]
#[command(version, about = "Peacher SDK CLI - Sync legislative data", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Action,

    /// Override the default path of the config
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// Override your member id
    #[arg(short, long, global = true)]
    member: Option<i32>,

    /// Override your api key
    #[arg(short, long, global = true)]
    api_key: Option<String>,

    /// Override the API base URL (default: from config or https://peacher.app)
    #[arg(short, long, global = true)]
    url: Option<String>,

    /// Override the jurisdiction ID (default: from config)
    #[arg(short = 'j', long, global = true)]
    jurisdiction: Option<i32>,

    /// Override the session ID (default: from config)
    #[arg(short = 's', long, global = true)]
    session: Option<i32>,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Manage CLI configuration
    Config {
        #[command(subcommand)]
        cmd: ConfigCmd,
    },

    /// List resources (jurisdictions, sessions, chambers)
    List {
        #[command(subcommand)]
        resource: ListResource,
    },

    /// Sync data from external API to Peacher
    Sync {
        #[command(subcommand)]
        target: SyncTarget,
    },

    /// Create resources (jurisdictions, etc.)
    Create {
        #[command(subcommand)]
        resource: CreateResource,
    },

    /// Run E2E test suite for the external client implementation
    Test {},
}

/// Resolved configuration combining CLI args and config file
pub struct ResolvedConfig {
    pub base_url: Url,
    pub member_id: i32,
    pub api_key: String,
    pub jurisdiction_id: Option<i32>,
    pub session_id: Option<i32>,
}

impl ResolvedConfig {
    pub fn from_args_and_config(args: &Args, config: CliConfig) -> anyhow::Result<Self> {
        let url_str = args.url.as_ref().unwrap_or(&config.url);
        let url = Url::parse(url_str)?;
        let member_id = args
            .member
            .or(config.member_id)
            .context("Member ID required!")?;

        let api_key = args
            .api_key
            .as_ref()
            .or(config.api_key.as_ref())
            .context("API key required")?;

        let jurisdiction_id = args.jurisdiction.or(config.jurisdiction_id);
        let session_id = args.session.or(config.session_id);

        Ok(Self {
            base_url: url,
            member_id,
            api_key: api_key.to_string(),
            jurisdiction_id,
            session_id,
        })
    }

    pub fn to_peacher(&self) -> PeacherClient {
        PeacherClient::new(self.member_id, self.api_key.clone())
            .with_jurisdiction_id(self.jurisdiction_id)
            .with_session_id(self.session_id)
            .with_base_url(self.base_url.clone())
    }

    pub fn require_jurisdiction(&self) -> anyhow::Result<i32> {
        self.jurisdiction_id.ok_or_else(|| {
            anyhow::anyhow!(
                "No jurisdiction specified. Use --jurisdiction or set default with: \
                 config set jurisdiction <id>"
            )
        })
    }

    #[allow(dead_code)]
    pub fn require_session(&self) -> anyhow::Result<i32> {
        self.session_id.ok_or_else(|| {
            anyhow::anyhow!(
                "No session specified. Use --session or set default with: \
                 config set session <id>"
            )
        })
    }
}

/// Main CLI entry point - generic over external client
pub async fn cli<C: ExternalClient>(client: C) {
    let args = Args::parse();

    let config = match match args.config {
        Some(ref path) => {
            let pathbuf = PathBuf::from(path);
            CliConfig::load_from_path(&pathbuf)
        }
        None => CliConfig::load(),
    } {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Config error: {e:?}");
            std::process::exit(1);
        }
    };

    // Load config file (or use defaults if missing)

    match run(args, config, client, None).await {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e:?}");
            std::process::exit(1);
        }
    }
}

/// CLI entry point with a pre-authenticated Peacher client.
/// Use this for testing when you have an already-authenticated client.
pub async fn cli_with_client<C: ExternalClient>(
    external_client: C,
    peacher_client: PeacherClient,
    args: Vec<String>,
) -> anyhow::Result<()> {
    let args = Args::try_parse_from(args)?;

    // Create a minimal config for testing
    let config = CliConfig::default();

    run(args, config, external_client, Some(peacher_client)).await
}

async fn run<C: ExternalClient>(
    args: Args,
    config: CliConfig,
    client: C,
    peacher_client: Option<PeacherClient>,
) -> anyhow::Result<()> {
    let resolved = ResolvedConfig::from_args_and_config(&args, config.clone())?;

    match args.command {
        Action::Config { cmd } => commands::config_cmd::run(cmd, &config),
        Action::List { resource } => {
            commands::list_cmd::run(resource, &resolved, peacher_client.as_ref()).await
        }
        Action::Sync { target } => {
            commands::sync_cmd::run(target, &resolved, client, peacher_client).await
        }
        Action::Create { resource } => {
            commands::create_cmd::run(resource, &resolved, peacher_client.as_ref()).await
        }
        Action::Test {} => match peacher_client {
            Some(pc) => {
                let mut test = ClientTest::new_from_parts(client, pc).await?;
                test.run_test().await
            }
            None => test_client(client, resolved.to_peacher()).await,
        },
    }
}
