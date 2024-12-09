use std::str::FromStr;
use std::fmt::Display;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerStatus {
    Running,
    Offline,
    Restarting,
    Crashed,
}

impl Display for ServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerStatus::Running => write!(f, "Running"),
            ServerStatus::Offline => write!(f, "Offline"),
            ServerStatus::Restarting => write!(f, "Restarting"),
            ServerStatus::Crashed => write!(f, "Crashed"),
        }
    }
}

impl FromStr for ServerStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Running" => Ok(ServerStatus::Running),
            "Offline" => Ok(ServerStatus::Offline),
            "Restarting" => Ok(ServerStatus::Restarting),
            "Crashed" => Ok(ServerStatus::Crashed),
            _ => Err(format!("Unknown ServerStatus: {}", s)),
        }
    }
}