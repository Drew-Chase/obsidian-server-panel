use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub port_forward_webui: bool,
    pub automatically_port_forward_servers: bool,
    pub servers_directory: String,
    pub backups_directory: String,
    pub java_install_directory: String,
}
