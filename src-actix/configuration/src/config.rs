use env::current_exe;
use lazy_static::lazy_static;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;

lazy_static! {
	/// Static instance of `ObsidianConfig`, loaded at runtime.
	pub static ref CONFIG: ObsidianConfig = ObsidianConfig::new().expect("Failed to load config");
}

/// Configuration struct for the application.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObsidianConfig {
	/// Port number for the application.
	pub port: u16,
	/// Flag to enable or disable port forwarding for the web UI.
	pub port_forward_webui: bool,
	/// Flag to enable or disable automatic port forwarding for servers.
	pub automatically_port_forward_servers: bool,
	/// Directory where servers are stored.
	pub servers_directory: String,
	/// Directory where backups are stored.
	pub backups_directory: String,
	/// Directory where Java is installed.
	pub java_install_directory: String,
}

impl ObsidianConfig {
	const CONFIG_FILE: &'static str = "app_settings.json";

	/// Creates a new `ObsidianConfig` instance.
	///
	/// This function loads the configuration from `app_settings.json`
	/// or generates a default configuration if the file does not exist or has errors.
	pub fn new() -> Result<Self, Box<dyn Error>> {
		let mut config = ObsidianConfig::get_default_config()?;
		config.load()?;
		config.save()?;
		Ok(config)
	}

	/// Saves the current configuration to `app_settings.json`.
	pub fn save(&self) -> Result<(), Box<dyn Error>> {
		let file = File::create(Self::CONFIG_FILE)?;
		serde_json::to_writer_pretty(file, self)?;
		Ok(())
	}

	/// Loads the configuration from `app_settings.json`.
	///
	/// If the file does not exist or contains errors, the default configuration is used.
	pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
		if std::fs::exists(Self::CONFIG_FILE)? {
			if let Ok(file) = File::open(Self::CONFIG_FILE) {
				let config: ObsidianConfig = match serde_json::from_reader(file) {
					Ok(config) => config,
					Err(e) => {
						error!("Failed to parse config file: {}", e);
						warn!("Using default config");
						Self::get_default_config()?
					}
				};
				*self = config;
				return Ok(());
			}
		}
		self.save()
	}

	/// Generates the default configuration.
	///
	/// The default configuration is based on the current executable's directory.
	pub fn get_default_config() -> Result<Self, Box<dyn Error>> {
		let current_exe = current_exe()?;
		let directory = current_exe
			.parent()
			.ok_or("Could not get current directory")?;
		let default_java_path = directory.join("java");
		Ok(ObsidianConfig {
			port: 1420,
			port_forward_webui: false,
			automatically_port_forward_servers: false,
			servers_directory: "servers".to_string(),
			backups_directory: "backups".to_string(),
			java_install_directory: default_java_path
				.to_str()
				.ok_or("Unable to convert default_java_path to str")?
				.to_string(),
		})
	}
}
