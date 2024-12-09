use miow::pipe::NamedPipe;
use std::ffi::OsStr;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION};

pub struct Server {
    pipe_name: String,
    process_handle: HANDLE,
}

#[derive(Debug)]
pub enum Error {
    ProcessError,
    IoError(io::Error),
    WinApiError,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

pub fn create_server() -> Result<Server, Error> {
    let pipe_name = r"\\.\pipe\minecraft_pipe";
    let pipe = NamedPipe::new(OsStr::new(pipe_name))?;

    let child = Command::new("java")
        .current_dir(r"F:\JetBrains\RustRover\obsidian-server-panel\tests\start_minecraft_server_with_piped_process_test\dev-env")
        .args(["-jar", "server.jar", "nogui"])
        .spawn()
        .map_err(|_| Error::ProcessError)?;

    let process_handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, 0, child.id()) };
    if process_handle.is_null() {
        return Err(Error::WinApiError);
    }

    let pipe = Arc::new(pipe);

    thread::spawn(move || loop {
        if NamedPipe::connect(&pipe).is_ok() {
            let mut buf = [0; 512];
            let mut handle = pipe.as_ref();
            while let Ok(bytes_read) = handle.read(&mut buf) {
                if bytes_read == 0 {
                    break;
                }
                let command = String::from_utf8_lossy(&buf[..bytes_read]);
                println!("Received command: {}", command);
            }
        }
    });

    Ok(Server {
        pipe_name: pipe_name.to_string(),
        process_handle,
    })
}

pub fn send_command(server: &Server, command: &str) -> Result<(), Error> {
    let mut pipe = NamedPipe::new(OsStr::new(server.pipe_name.as_str()))?;
    pipe.connect().map_err(|_| Error::WinApiError)?;
    pipe.write_all(command.as_bytes())?;
    pipe.flush()?;
    pipe.write_all(b"\n")?;
    pipe.flush().map_err(|_| Error::WinApiError)?;
    pipe.disconnect().map_err(|_| Error::WinApiError)?;
    Ok(())
}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.process_handle);
        }
    }
}
