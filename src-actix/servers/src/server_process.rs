use crate::server::Server;
use crate::start_executable_type::{StartExecutableType, StartExecutableTypeExt};
use lazy_static::lazy_static;
use std::error::Error;
use std::io::Error as IoError;
use std::io::Write;
use std::process::Stdio;
use std::thread;

lazy_static! {
    static ref running_servers: Vec<Server<u64>> = Vec::new();
}

pub trait ServerProcess {
    fn start_server(&mut self) -> Result<u64, Box<dyn Error>>;
    fn stop_server(&mut self) -> Result<u64, Box<dyn Error>>;
    fn send_command(&mut self, command: impl AsRef<str>) -> Result<(), Box<dyn Error>>;
}

impl ServerProcess for Server<u64> {
    fn start_server(&mut self) -> Result<u64, Box<dyn Error>> {
        // Clone the `start_script` and unwrap it safely; assumes `start_script` is always `Some`.
        let start_script = &self.start_script;
        let start_script = start_script.clone().unwrap();

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
                    return Err(Box::new(IoError::new(std::io::ErrorKind::Other, "Unsupported OS for Script type")));
                }
            }
            StartExecutableType::Jar => {
                // Check if Java runtime path is provided, otherwise return an error.
                if let Some(jr) = &self.java_runtime {
                    jr.to_str().ok_or_else(|| Box::new(IoError::new(std::io::ErrorKind::InvalidData, "Invalid Java runtime path")))?
                } else {
                    return Err(Box::new(IoError::new(std::io::ErrorKind::NotFound, "Java runtime not set")));
                }
            }
            StartExecutableType::Executable =>
            // Convert the executable path to a string and handle invalid data.
                start_script.to_str().ok_or_else(|| Box::new(IoError::new(std::io::ErrorKind::InvalidData, "Invalid executable path")))?,
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
                    Err(_) => return Err(Box::new(IoError::new(std::io::ErrorKind::InvalidData, "Invalid Java arguments"))),
                };
            }
            // Adding the -jar argument and the start script path to the command.
            process.arg("-jar");
            process.arg(start_script);
            if let Some(minecraft_args) = &self.minecraft_arguments {
                // Split Minecraft arguments into separate tokens and handle errors.
                match shell_words::split(minecraft_args) {
                    Ok(args) => process.args(args),
                    Err(_) => return Err(Box::new(IoError::new(std::io::ErrorKind::InvalidData, "Invalid Minecraft arguments"))),
                };
            }
        }

        // Configure the process to provide input/output via pipes.
        process.stdin(Stdio::piped());
        process.stdout(Stdio::piped());

        // Spawn the process and handle potential spawning errors.
        let mut child = process.spawn()?;

        // Assign the stdin and stdout of the spawned process to the server fields.
        self.stdin = child.stdin.take();
        self.stdout = child.stdout.take();

        // Retrieve and return the process ID (PID) as a 64-bit integer.
        let pid = child.id();

        Ok(pid as u64)
    }

    fn stop_server(&mut self) -> Result<u64, Box<dyn Error>> {
        todo!()
    }

    fn send_command(&mut self, command: impl AsRef<str>) -> Result<(), Box<dyn Error>> {
        if let Some(stdin) = &mut self.stdin {
            writeln!(stdin, "{}", command.as_ref())?;
        } else {
            return Err(Box::new(IoError::new(std::io::ErrorKind::BrokenPipe, "Stdin not available")));
        }

        Ok(())
    }
}
