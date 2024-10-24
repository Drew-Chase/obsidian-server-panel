use crate::server_db::Server;
use lazy_static::lazy_static;
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::unistd::Pid;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::os::unix::io::FromRawFd;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;

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

    pub fn pipe_stdio_from_pid<F>(pid: u32, callback: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let pid = Pid::from_raw(pid as i32);

        // Open the /proc/<pid>/fd/1 file for stdout of the process
        let path = format!("./{}", pid);
        let stdout_fd = nix::fcntl::open(
            path.as_str(), // Convert `String` to `&str`
            OFlag::O_RDONLY,
            nix::sys::stat::Mode::empty(),
        )?;

        // Make the file descriptor non-blocking
        fcntl(stdout_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))?;

        let async_file = unsafe { File::from_raw_fd(stdout_fd) };
        let reader = BufReader::new(async_file);

        let callback = Arc::new(Mutex::new(callback));

        tokio::spawn(async move {
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let callback = Arc::clone(&callback);
                let line = line.to_string();
                tokio::spawn(async move {
                    let callback = callback.lock().await;
                    (callback)(line);
                });
            }
        });

        Ok(())
    }
    pub async fn write_to_stdin(pid: u32, data: String) -> Result<(), Box<dyn Error>> {
        let pid = Pid::from_raw(pid as i32);

        // Open the /proc/<pid>/fd/0 file for stdin of the process
        let path = format!("./{}", pid);
        let stdin_fd = nix::fcntl::open(
            path.as_str(), // Convert `String` to `&str`
            OFlag::O_WRONLY,
            nix::sys::stat::Mode::empty(),
        )?;

        // Make the file descriptor non-blocking
        fcntl(stdin_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))?;

        let async_file = unsafe { File::from_raw_fd(stdin_fd) };
        let mut writer = tokio::io::BufWriter::new(async_file);

        writer.write_all(data.as_bytes()).await?;

        Ok(())
    }
}
