use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use url::Url;

use crate::client::PeacherClient;

fn default_url() -> String {
    "https://peacher.app".to_string()
}

/// CLI configuration stored at ~/.config/peacher/config.toml
#[derive(Serialize, Deserialize, Clone)]
pub struct CliConfig {
    /// API key for authentication with Peacher API (required for sync)
    pub api_key: Option<String>,
    /// Default jurisdiction ID for commands
    pub jurisdiction_id: Option<i32>,
    /// Default session id for commands,
    pub session_id: Option<i32>,
    /// Base URL for the Peacher API
    #[serde(default = "default_url")]
    pub url: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            session_id: None,
            jurisdiction_id: None,
            url: default_url(),
        }
    }
}

impl CliConfig {
    /// Returns the config file path: ~/.config/peacher/config.toml
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("peacher").join("config.toml"))
    }

    pub fn load_from_path(path: &Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: CliConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Load config from disk. Returns default if file doesn't exist.
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        if !path.exists() {
            return Ok(Self::default());
        }

        Self::load_from_path(&path)
    }

    /// Save config to disk. Creates parent directories if needed.
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&path, contents)?;
        Ok(())
    }

    /// Convert to Peacher SDK config
    pub fn to_peacher(&self) -> Option<PeacherClient> {
        let api_key = self.api_key.clone()?;
        let mut peacher = PeacherClient::new(api_key);

        if let Ok(url) = Url::parse(&self.url) {
            peacher.base = url;
        }
        if let Some(jurisdiction_id) = self.jurisdiction_id {
            peacher.jurisdiction_id = Some(jurisdiction_id);
        }
        if let Some(session_id) = self.session_id {
            peacher.session_id = Some(session_id);
        }
        Some(peacher)
    }
}
