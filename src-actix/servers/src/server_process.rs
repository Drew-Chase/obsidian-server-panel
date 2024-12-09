use crate::server_db::Server;
use std::io::Stdin;
use std::process::Stdio;

pub struct ServerProcess {
    pub stdin: Stdin,
    pub stdout: Stdin,
    pub stderr: Stdin,
    pub pid: i32,
    pub exit_code: Option<i32>,
}

impl ServerProcess {
    pub fn start(server: &Server){
        
    }
}
