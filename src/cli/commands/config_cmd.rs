use crate::cli::config::CliConfig;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ConfigCmd {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// The key to set (api-key, jurisdiction, session, url, member-id)
        key: String,
        /// The value to set
        value: String,
    },
    /// Get a configuration value
    Get {
        /// The key to get
        key: String,
    },
    /// Show the config file path
    Path,
}

pub fn run(cmd: ConfigCmd, config: &CliConfig) -> Result<()> {
    match cmd {
        ConfigCmd::Show => show_config(config),
        ConfigCmd::Set { key, value } => set_config(config, &key, &value),
        ConfigCmd::Get { key } => get_config(config, &key),
        ConfigCmd::Path => show_path(),
    }
}

fn show_config(config: &CliConfig) -> Result<()> {
    println!("Current configuration:");
    println!("  url:           {}", config.url);
    println!(
        "  api-key:       {}",
        config
            .api_key
            .as_ref()
            .map(|_| "********")
            .unwrap_or("<not set>")
    );
    println!(
        "  member-id:     {}",
        config
            .member_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "<not set>".into())
    );
    println!(
        "  jurisdiction:  {}",
        config
            .jurisdiction_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "<not set>".into())
    );
    Ok(())
}

fn set_config(config: &CliConfig, key: &str, value: &str) -> Result<()> {
    let mut config = config.clone();

    match key {
        "api-key" => config.api_key = Some(value.to_string()),
        "url" => config.url = value.to_string(),
        "jurisdiction" => config.jurisdiction_id = Some(value.parse()?),
        "member-id" => config.member_id = Some(value.parse()?),
        _ => anyhow::bail!(
            "Unknown config key: {}. Valid keys: api-key, url, jurisdiction, session, member-id",
            key
        ),
    }

    config.save()?;
    println!(
        "Set {} = {}",
        key,
        if key == "api-key" { "********" } else { value }
    );
    Ok(())
}

fn get_config(config: &CliConfig, key: &str) -> Result<()> {
    let value = match key {
        "api-key" => config.api_key.clone().unwrap_or_default(),
        "url" => config.url.clone(),
        "jurisdiction" => config
            .jurisdiction_id
            .map(|id| id.to_string())
            .unwrap_or_default(),
        "member-id" => config
            .member_id
            .map(|id| id.to_string())
            .unwrap_or_default(),
        _ => anyhow::bail!("Unknown config key: {}", key),
    };
    println!("{}", value);
    Ok(())
}

fn show_path() -> Result<()> {
    match CliConfig::config_path() {
        Some(path) => println!("{}", path.display()),
        None => println!("<could not determine>"),
    }
    Ok(())
}
