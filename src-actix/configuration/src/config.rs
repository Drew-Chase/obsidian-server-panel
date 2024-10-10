use serde_derive::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub port_forward_webui: bool,
    pub automatically_port_forward_servers: bool,
    pub servers_directory: String,
    pub backups_directory: String,
    pub java_install_directory: String,
}

impl Config {
    pub fn from_file(file: impl AsRef<Path>) -> Result<Config, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(file)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
    pub fn to_file(&self, file: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(file, contents)?;
        Ok(())
    }
}
