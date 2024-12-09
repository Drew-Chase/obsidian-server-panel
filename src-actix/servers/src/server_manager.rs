use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use crate::server_db::Server;
use crate::server_process::ServerProcess;

pub struct ServerManager {
	servers: HashMap<u64, ServerProcess>
}

impl Default for ServerManager {
	fn default() -> Self {
		ServerManager::new()
	}
}

impl ServerManager {
	pub fn new() -> Self {
		ServerManager {
			servers: HashMap::new()
		}
	}
	
	pub fn start(&mut self, server: &Server)-> Result<(), Box<dyn Error>>
	{
		
		Ok(())
	}
	
}