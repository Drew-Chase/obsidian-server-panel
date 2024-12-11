use crate::server::Server;
use crate::server_database::ServerDatabase;
use crate::server_status::ServerStatus;
use crate::start_executable_type::{StartExecutableType, StartExecutableTypeExt};
use lazy_static::lazy_static;
use log::{info, warn};
use std::clone::Clone;
use std::error::Error;
use std::io::Write;
use std::io::{Error as IoError, Stdin};
use std::path::PathBuf;
use std::process::{ChildStdin, ChildStdout, Stdio};
use std::sync::{Arc, Mutex};

struct RunningServerProcess {
    /// The server id
    pub server_id: u64,
    /// The process id of the running server
    pub pid: u64,
    /// Configuration related to the server's standard input stream.
    pub stdin: Option<ChildStdin>,
    /// Configuration related to the server's standard output stream.
    pub stdout: Option<ChildStdout>,
}

lazy_static! {
    static ref RUNNING_SERVERS: Arc<Mutex<Vec<Arc<Mutex<RunningServerProcess>>>>> = Arc::new(Mutex::new(Vec::new()));
}

pub trait ServerProcess {
    fn start_server(&mut self) -> Result<u64, Box<dyn Error>>;
    fn stop_server(&mut self) -> Result<u64, Box<dyn Error>>;
    fn send_command_to_server(id: u64, command: impl AsRef<str>) -> Result<(), Box<dyn Error>>;
}

impl ServerProcess for Server<u64> {
    fn start_server(&mut self) -> Result<u64, Box<dyn Error>> {
        // Clone the `start_script` and unwrap it safely; assumes `start_script` is always `Some`.
        let start_script = &self.start_script;
        let start_script = start_script
            .clone()
            .ok_or_else(|| Box::new(IoError::new(std::io::ErrorKind::NotFound, "Start script not set")))?;

        // Determine the type of executable based on the script path and handle errors if it fails.
        let start_executable_type = StartExecutableType::from_path(&start_script)?;

        // Select the appropriate command or executable based on the determined type.
        let program: &str = match start_executable_type {
            StartExecutableType::Script => {
                // Choose the shell command based on the current operating system.
                if cfg!(target_os = "windows") {
                    "cmd"
                } else if cfg!(target_os = "linux") {
                    "sh"
                } else {
                    // Return an error if the OS is unsupported for scripting.
                    return Err(Box::new(IoError::new(
                        std::io::ErrorKind::Other,
                        "Unsupported OS for Script type",
                    )));
                }
            }
            StartExecutableType::Jar => {
                // Check if Java runtime path is provided, otherwise return an error.
                if let Some(jr) = &self.java_runtime {
                    jr.to_str().ok_or_else(|| {
                        Box::new(IoError::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid Java runtime path",
                        ))
                    })?
                } else {
                    return Err(Box::new(IoError::new(
                        std::io::ErrorKind::NotFound,
                        "Java runtime not set",
                    )));
                }
            }
            StartExecutableType::Executable =>
            // Convert the executable path to a string and handle invalid data.
            {
                start_script
                    .to_str()
                    .ok_or_else(|| Box::new(IoError::new(std::io::ErrorKind::InvalidData, "Invalid executable path")))?
            }
        };

        // Prepare to launch a new process using the determined executable or command.
        let mut process = std::process::Command::new(program);

        // Set the working directory for the process.
        process.current_dir(&self.directory);

        // Add arguments to the process based on the type of start executable.
        if start_executable_type == StartExecutableType::Script {
            process.arg(start_script);
        } else if start_executable_type == StartExecutableType::Jar {
            if let Some(java_arg) = &self.java_arguments {
                // Split Java arguments into separate tokens and handle errors.
                match shell_words::split(java_arg) {
                    Ok(args) => process.args(args),
                    Err(_) => {
                        return Err(Box::new(IoError::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid Java arguments",
                        )))
                    }
                };
            }
            process.arg(format!("-Xms{}G", self.min_ram));
            process.arg(format!("-Xmx{}G", self.max_ram));

            // Adding the -jar argument and the start script path to the command.
            process.arg("-jar");
            process.arg(start_script);
            if let Some(minecraft_args) = &self.minecraft_arguments {
                // Split Minecraft arguments into separate tokens and handle errors.
                match shell_words::split(minecraft_args) {
                    Ok(args) => process.args(args),
                    Err(_) => {
                        return Err(Box::new(IoError::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid Minecraft arguments",
                        )))
                    }
                };
            }
        }

        info!(
            "Running command: {} {}",
            process.get_program().to_str().unwrap_or("unknown.exe"),
            process
                .get_args()
                .map(|a| a.to_str().unwrap_or(""))
                .collect::<Vec<&str>>()
                .join(" ")
        );

        // Configure the process to provide input/output via pipes.
        process.stdin(Stdio::piped());
        process.stdout(Stdio::piped());

        // Spawn the process and handle potential spawning errors.
        let mut child = process.spawn()?;

        // Retrieve and return the process ID (PID) as a 64-bit integer.
        let pid = child.id();

        // Add server to the running servers list.
        let running_server_process = RunningServerProcess {
            server_id: self.id,
            pid: pid as u64,
            stdin: child.stdin.take(),
            stdout: child.stdout.take(),
        };
        match RUNNING_SERVERS.lock() {
            Ok(mut servers) => servers.push(Arc::new(Mutex::new(running_server_process))),
            Err(_) => {
                return Err(Box::new(IoError::new(
                    std::io::ErrorKind::Other,
                    "Failed to lock running servers",
                )))
            }
        }

        self.status = Some(ServerStatus::Online);
        self.update()?;

        Ok(pid as u64)
    }

    fn stop_server(&mut self) -> Result<u64, Box<dyn Error>> {
        self.status = Some(ServerStatus::Offline);
        self.update()?;
        todo!()
    }

    fn send_command_to_server(id: u64, command: impl AsRef<str>) -> Result<(), Box<dyn Error>> {
        if let Ok(servers) = RUNNING_SERVERS.lock() {
            let server = servers
                .iter()
                .find(|s| s.lock().map(|server| server.server_id == id).unwrap_or(false))
                .ok_or_else(|| IoError::new(std::io::ErrorKind::NotFound, "Server not found"))?;

            if let Ok(mut server) = server.lock() {
                if let Some(stdin) = &mut server.stdin {
                    writeln!(stdin, "{}", command.as_ref())?;
                } else {
                    return Err(Box::new(IoError::new(
                        std::io::ErrorKind::BrokenPipe,
                        "Stdin not available",
                    )));
                }
            }
        }

        Ok(())
    }
}
