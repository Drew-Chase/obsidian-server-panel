use crate::server_db::Server;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerStatus {
    Running,
    Stopped,
    Restarting,
    Crashed,
}

impl Display for ServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerStatus::Running => write!(f, "Running"),
            ServerStatus::Stopped => write!(f, "Stopped"),
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
            "Stopped" => Ok(ServerStatus::Stopped),
            "Restarting" => Ok(ServerStatus::Restarting),
            "Crashed" => Ok(ServerStatus::Crashed),
            _ => Err(format!("Unknown ServerStatus: {}", s)),
        }
    }
}

pub struct ServerProcess {
    pub server: Server,
    pid: Option<u32>,
    process_stdout: Option<String>,
    pub status: ServerStatus,
}

trait ToProcess {
    fn to_process(&self) -> Result<ServerProcess, Box<dyn Error>>;
}

impl ToProcess for Server {
    fn to_process(&self) -> Result<ServerProcess, Box<dyn Error>> {
        Ok(ServerProcess {
            server: self.clone(),
            pid: None,
            process_stdout: None,
            status: ServerStatus::Stopped,
        })
    }
}

lazy_static! {
    static ref SERVERS: Vec<ServerProcess> = vec![];
}

impl ServerProcess {
    pub fn new(server: Server) -> Self {
        Self {
            server,
            pid: None,
            process_stdout: None,
            status: ServerStatus::Stopped,
        }
    }

    pub fn start_server(&mut self) -> Result<(), Box<dyn Error>> {
        todo!("Implement the start_server function")
    }
    pub fn stop_server(&mut self) -> Result<(), Box<dyn Error>> {
        todo!("Implement the stop_server function")
    }

    pub fn restart_server(&mut self) -> Result<(), Box<dyn Error>> {
        todo!("Implement the restart_server function")
    }

    pub fn kill_server(&mut self) -> Result<(), Box<dyn Error>> {
        todo!("Implement the kill_server function")
    }
}
