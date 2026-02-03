use anyhow::{Result, bail};
use clap::{Subcommand, ValueEnum};
use std::{
    error::Error,
    fmt,
    io::ErrorKind,
    path::{Path, PathBuf},
    str::FromStr,
};
use thiserror::Error;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "snake_case")]
pub enum ConfigKey {
    /// api_key for Peacher
    ApiKey,
    /// url to the Peacher instance
    Url,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCmd {
    Show,
    Set {
        #[arg(value_enum, help = "Which config key to set")]
        key: ConfigKey,
        value: String,
    },
}

impl ConfigCmd {
    pub fn run(self, override_config_path: Option<PathBuf>) -> Result<()> {
        let mut config = match override_config_path {
            Some(path) => Config::load_or_default(path.clone())?,
            None => Config::load()?,
        };

        match self {
            ConfigCmd::Show => {
                println!(
                    "Current configuration({}):\n{}",
                    config.path.display(),
                    config.options
                );
                Ok(())
            }
            ConfigCmd::Set { key, value } => {
                match key {
                    ConfigKey::ApiKey => config.options.api_key = Some(value.to_string()),
                    ConfigKey::Url => config.options.url = Url::parse(&value)?,
                }

                config.save()?;
                println!(
                    "Updated and saved to {}:\n{}",
                    config.path.display(),
                    config.options
                );

                Ok(())
            }
        }
    }
}

fn default_url() -> Url {
    Url::from_str("https://api.peacher.app").unwrap()
}

impl Default for Config {
    fn default() -> Self {
        let path = dirs::config_dir()
            .map(|p| p.join("peacher").join("config.toml"))
            .expect("A config directory for your operating system");
        Config {
            path,
            options: ConfigOptions {
                api_key: None,
                url: default_url(),
            },
        }
    }
}

pub struct Config {
    pub path: PathBuf,
    pub options: ConfigOptions,
}

#[derive(Error, Debug)]
pub enum ConfigLoadErr {
    #[error("Config file not found")]
    NotFound,
    #[error("Failed to deserialize {0}")]
    Deserialize(toml::de::Error),
    #[error("Error while loading config from file: {0}")]
    Other(Box<dyn Error + Send + Sync>),
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        Config {
            path,
            ..Default::default()
        }
    }

    /// Attempts to load the file. Returns a default config if not found
    ///
    /// # Errors
    ///
    /// if the load error is not a deserialization or notfound issue
    pub fn load_or_default(path: PathBuf) -> Result<Self> {
        match Config::load_from_path(path.clone()) {
            Ok(config) => Ok(config),
            Err(ConfigLoadErr::NotFound) | Err(ConfigLoadErr::Deserialize(_)) => {
                Ok(Config::new(path))
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn load_from_path(path: PathBuf) -> Result<Self, ConfigLoadErr> {
        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => return Err(ConfigLoadErr::NotFound),
                _ => return Err(ConfigLoadErr::Other(err.into())),
            },
        };
        let config: ConfigOptions =
            toml::from_str(&contents).map_err(ConfigLoadErr::Deserialize)?;

        Ok(Self {
            path,
            options: config,
        })
    }

    /// Load a config from disk. Returns default if file doesn't exist or has been corrupted.
    ///
    /// # Errors
    /// if the file encountered an unexpected error
    pub fn load() -> Result<Self> {
        let default_config = Self::default();

        match Self::load_from_path(default_config.path.clone()) {
            Ok(this) => Ok(this),
            Err(e) => match e {
                ConfigLoadErr::Deserialize(_) | ConfigLoadErr::NotFound => Ok(Self::default()),
                ConfigLoadErr::Other(e) => bail!(e),
            },
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = &self.path;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(&self.options)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

/// CLI configuration stored at ~/.config/peacher/config.toml
#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigOptions {
    /// API key for authentication with Peacher API (required for sync)
    pub api_key: Option<String>,
    /// Base URL for the Peacher API
    #[serde(default = "default_url")]
    pub url: Url,
}

impl fmt::Display for ConfigOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  url:           {}", self.url)?;
        writeln!(
            f,
            "  api_key:       {}",
            self.api_key
                .as_ref()
                .map(|_| "********")
                .unwrap_or("<not set>")
        )?;
        Ok(())
    }
}
