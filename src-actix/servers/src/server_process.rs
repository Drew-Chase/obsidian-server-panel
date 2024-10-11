use crate::server_db::Server;
use std::error::Error;

pub enum ServerStatus {
    Running,
    Stopped,
    Restarting,
    Crashed,
}
pub struct ServerProcess {
    pub server: Server,
    pid: u32,
    process_stdout: Option<String>,
    pub status: ServerStatus,
}

impl ServerProcess {
    pub fn new(server: Server) -> Self {
        Self {
            server,
            pid: 0,
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
