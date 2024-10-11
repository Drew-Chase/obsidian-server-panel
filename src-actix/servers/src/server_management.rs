use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::server_db;
use crate::server_process::ServerProcess;

lazy_static! {
    pub static ref SERVER_MANAGER: Mutex<ServerManager> = Mutex::new(ServerManager::new());
}

pub struct ServerManager {
    pub servers: Vec<ServerProcess>,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
        }
    }
    pub fn refresh_servers(&mut self) {
        let servers = server_db::get_servers();
        self.servers = servers
            .iter()
            .map(|server| ServerProcess::new(server.clone()))
            .collect();
    }
    
    pub fn add_server(&mut self, server: server_db::Server) {
        self.servers.push(ServerProcess::new(server));
    }
    
    pub fn remove_server(&mut self, server_id: u32) {
        self.servers.retain(|server| server.server.id != server_id);
    }
    
    pub fn update_server(&mut self, id: u32)->Result<(), Box<dyn std::error::Error>>
    {
        let server = server_db::get_server_by_id(id).ok_or("Server not found")?;
        self.remove_server(id);
        self.add_server(server);
        
        
        Ok(())
    }
    

}