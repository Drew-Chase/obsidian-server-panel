use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(windows)]
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE, CloseHandle};
#[cfg(windows)]
use windows::Win32::System::Pipes::{
	CreateNamedPipeA, PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE, PIPE_READMODE_BYTE,
	PIPE_WAIT, ConnectNamedPipe
};
#[cfg(windows)]
use windows::Win32::Storage::FileSystem::{
	CreateFileA, OPEN_EXISTING, FILE_SHARE_READ, FILE_SHARE_WRITE,
	GENERIC_READ, GENERIC_WRITE,
};
#[cfg(windows)]
use windows::Win32::System::Threading::{
	OpenProcess, PROCESS_ALL_ACCESS,
};
#[cfg(windows)]
use std::os::windows::io::{FromRawHandle, IntoRawHandle};

#[derive(Debug)]
pub enum ProcessError {
	NotFound,
	AccessDenied,
	IoError(std::io::Error),
	PipeError,
	ExecutableNotFound,
	ProcessCreationFailed,
	StdioCaptureFailed,
	PidWriteFailed,
	#[cfg(windows)]
	WindowsError(windows::core::Error),
}

impl std::fmt::Display for ProcessError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ProcessError::NotFound => write!(f, "Process not found"),
			ProcessError::AccessDenied => write!(f, "Access denied to process"),
			ProcessError::IoError(e) => write!(f, "IO error: {}", e),
			ProcessError::PipeError => write!(f, "Pipe error"),
			ProcessError::ExecutableNotFound => write!(f, "Executable not found"),
			ProcessError::ProcessCreationFailed => write!(f, "Failed to create process"),
			ProcessError::StdioCaptureFailed => write!(f, "Failed to capture stdio"),
			ProcessError::PidWriteFailed => write!(f, "Failed to write PID file"),
			#[cfg(windows)]
			ProcessError::WindowsError(e) => write!(f, "Windows error: {}", e),
		}
	}
}

impl std::error::Error for ProcessError {}

impl From<std::io::Error> for ProcessError {
	fn from(error: std::io::Error) -> Self {
		ProcessError::IoError(error)
	}
}

/// Configuration options for process management
#[derive(Debug, Clone)]
pub struct ProcessConfig {
	pub buffer_size: usize,
	pub read_timeout: Option<std::time::Duration>,
	pub write_timeout: Option<std::time::Duration>,
}

impl Default for ProcessConfig {
	fn default() -> Self {
		Self {
			buffer_size: 4096,
			read_timeout: Some(std::time::Duration::from_secs(30)),
			write_timeout: Some(std::time::Duration::from_secs(30)),
		}
	}
}

/// Manages a newly created process
pub struct ProcessManager {
	child: Child,
	pid: u32,
	pid_file: PathBuf,
	config: ProcessConfig,
}

impl ProcessManager {
	/// Creates a new process with the given executable and arguments
	pub async fn new(
		executable: PathBuf,
		args: Vec<String>,
		config: Option<ProcessConfig>,
	) -> Result<Self, ProcessError> {
		let config = config.unwrap_or_default();

		if !executable.exists() {
			return Err(ProcessError::ExecutableNotFound);
		}

		let mut command = Command::new(executable);
		let child = command
			.args(args)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
			.map_err(|_| ProcessError::ProcessCreationFailed)?;

		let pid = child.id().ok_or(ProcessError::ProcessCreationFailed)?;
		let pid_file = PathBuf::from(format!("./{}.pid", pid));

		// Write PID to file
		let mut file = File::create(&pid_file).map_err(|_| ProcessError::PidWriteFailed)?;
		writeln!(file, "{}", pid).map_err(|_| ProcessError::PidWriteFailed)?;

		Ok(Self {
			child,
			pid,
			pid_file,
			config,
		})
	}

	/// Returns the process ID
	pub fn pid(&self) -> u32 {
		self.pid
	}

	/// Starts handling process output with the given callback
	pub async fn start_output_handling<F>(&mut self, callback: F) -> Result<(), ProcessError>
	                                      where
		                                      F: Fn(String) + Send + Sync + 'static,
	{
		let stdout = self.child.stdout.take()
		                 .ok_or(ProcessError::StdioCaptureFailed)?;
		let stderr = self.child.stderr.take()
		                 .ok_or(ProcessError::StdioCaptureFailed)?;

		let (tx, rx) = mpsc::channel(self.config.buffer_size);
		let callback = Arc::new(callback);

		// Handle stdout
		let tx_clone = tx.clone();
		let callback_clone = callback.clone();
		let read_timeout = self.config.read_timeout;
		tokio::spawn(async move {
			let mut reader = BufReader::new(stdout).lines();
			loop {
				let line = if let Some(timeout) = read_timeout {
					tokio::time::timeout(timeout, reader.next_line()).await
				} else {
					Ok(reader.next_line().await)
				};

				match line {
					Ok(Ok(Some(content))) => {
						callback_clone(content.clone());
						if tx_clone.send(content).await.is_err() {
							break;
						}
					},
					_ => break,
				}
			}
		});

		// Handle stderr
		let read_timeout = self.config.read_timeout;
		tokio::spawn(async move {
			let mut reader = BufReader::new(stderr).lines();
			loop {
				let line = if let Some(timeout) = read_timeout {
					tokio::time::timeout(timeout, reader.next_line()).await
				} else {
					Ok(reader.next_line().await)
				};

				match line {
					Ok(Ok(Some(content))) => {
						callback(content.clone());
						if tx.send(content).await.is_err() {
							break;
						}
					},
					_ => break,
				}
			}
		});

		// Process the stream
		let mut stream = ReceiverStream::new(rx);
		while let Some(_) = stream.next().await {}

		Ok(())
	}

	/// Writes input to the process
	pub async fn write_input(&mut self, input: &str) -> Result<(), ProcessError> {
		let stdin = self.child.stdin.as_mut()
		                .ok_or(ProcessError::StdioCaptureFailed)?;

		if let Some(timeout) = self.config.write_timeout {
			tokio::time::timeout(
				timeout,
				async {
					stdin.write_all(input.as_bytes()).await?;
					stdin.write_all(b"\n").await?;
					stdin.flush().await?;
					Ok::<(), std::io::Error>(())
				},
			).await.map_err(|_| ProcessError::IoError(
				std::io::Error::new(std::io::ErrorKind::TimedOut, "Write timed out")
			))??;
		} else {
			stdin.write_all(input.as_bytes()).await?;
			stdin.write_all(b"\n").await?;
			stdin.flush().await?;
		}

		Ok(())
	}

	/// Kills the process and cleans up resources
	pub async fn kill(&mut self) -> Result<(), ProcessError> {
		self.child.kill().await?;
		if self.pid_file.exists() {
			std::fs::remove_file(&self.pid_file)?;
		}
		Ok(())
	}
}

/// Represents a process that we've attached to
pub struct AttachedProcess {
	pid: u32,
	config: ProcessConfig,
	#[cfg(unix)]
	stdin_path: PathBuf,
	#[cfg(unix)]
	stdout_path: PathBuf,
	#[cfg(windows)]
	process_handle: HANDLE,
	#[cfg(windows)]
	stdin_pipe: Option<HANDLE>,
	#[cfg(windows)]
	stdout_pipe: Option<HANDLE>,
}

impl AttachedProcess {
	/// Attaches to an existing process
	pub async fn new(pid: u32, config: Option<ProcessConfig>) -> Result<Self, ProcessError> {
		let config = config.unwrap_or_default();

		#[cfg(unix)]
		{
			let stdin_path = PathBuf::from(format!("/proc/{}/fd/0", pid));
			let stdout_path = PathBuf::from(format!("/proc/{}/fd/1", pid));

			if !stdin_path.exists() || !stdout_path.exists() {
				return Err(ProcessError::NotFound);
			}

			Ok(Self {
				pid,
				config,
				stdin_path,
				stdout_path,
			})
		}

		#[cfg(windows)]
		{
			unsafe {
				let process_handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid)
					.map_err(|e| ProcessError::WindowsError(e))?;

				if process_handle == HANDLE(0) {
					return Err(ProcessError::NotFound);
				}

				let stdin_pipe_name = format!("\\\\.\\pipe\\rust_stdin_{}", pid);
				let stdout_pipe_name = format!("\\\\.\\pipe\\rust_stdout_{}", pid);

				let stdin_pipe = create_named_pipe(&stdin_pipe_name)?;
				let stdout_pipe = create_named_pipe(&stdout_pipe_name)?;

				connect_pipe(stdin_pipe)?;
				connect_pipe(stdout_pipe)?;

				Ok(Self {
					pid,
					config,
					process_handle,
					stdin_pipe: Some(stdin_pipe),
					stdout_pipe: Some(stdout_pipe),
				})
			}
		}
	}

	/// Starts handling process output with the given callback
	#[cfg(unix)]
	pub async fn start_output_handling<F>(&self, callback: F) -> Result<(), ProcessError>
	                                      where
		                                      F: Fn(String) + Send + Sync + 'static,
	{
		use tokio::fs::File;
		use std::os::unix::fs::OpenOptionsExt;

		let file = tokio::fs::OpenOptions::new()
			.read(true)
			.custom_flags(libc::O_NONBLOCK)
			.open(&self.stdout_path)
			.await?;

		let callback = Arc::new(Mutex::new(callback));
		let reader = BufReader::new(file);
		let mut lines = reader.lines();
		let read_timeout = self.config.read_timeout;

		tokio::spawn(async move {
			loop {
				let line = if let Some(timeout) = read_timeout {
					tokio::time::timeout(timeout, lines.next_line()).await
				} else {
					Ok(lines.next_line().await)
				};

				match line {
					Ok(Ok(Some(content))) => {
						let callback = callback.lock().await;
						callback(content);
					},
					_ => break,
				}
			}
		});

		Ok(())
	}

	/// Starts handling process output with the given callback
	#[cfg(windows)]
	pub async fn start_output_handling<F>(&self, callback: F) -> Result<(), ProcessError>
	                                      where
		                                      F: Fn(String) + Send + Sync + 'static,
	{
		let stdout_pipe = self.stdout_pipe.ok_or(ProcessError::PipeError)?;

		unsafe {
			let file = File::from_raw_handle(stdout_pipe.0 as *mut _);
			let file = tokio::fs::File::from_std(file);

			let callback = Arc::new(Mutex::new(callback));
			let reader = BufReader::new(file);
			let mut lines = reader.lines();
			let read_timeout = self.config.read_timeout;

			tokio::spawn(async move {
				loop {
					let line = if let Some(timeout) = read_timeout {
						tokio::time::timeout(timeout, lines.next_line()).await
					} else {
						Ok(lines.next_line().await)
					};

					match line {
						Ok(Ok(Some(content))) => {
							let callback = callback.lock().await;
							callback(content);
						},
						_ => break,
					}
				}
			});

			// Don't drop the file handle
			std::mem::forget(file);
		}

		Ok(())
	}

	/// Writes input to the process
	#[cfg(unix)]
	pub async fn write_input(&self, input: &str) -> Result<(), ProcessError> {
		use tokio::fs::File;

		let mut file = tokio::fs::OpenOptions::new()
			.write(true)
			.open(&self.stdin_path)
			.await?;

		if let Some(timeout) = self.config.write_timeout {
			tokio::time::timeout(
				timeout,
				async {
					file.write_all(input.as_bytes()).await?;
					file.write_all(b"\n").await?;
					file.flush().await?;
					Ok::<(), std::io::Error>(())
				},
			).await.map_err(|_| ProcessError::IoError(
				std::io::Error::new(std::io::ErrorKind::TimedOut, "Write timed out")
			))??;
		} else {
			file.write_all(input.as_bytes()).await?;
			file.write_all(b"\n").await?;
			file.flush().await?;
		}

		Ok(())
	}

	/// Writes input to the process
	#[cfg(windows)]
	pub async fn write_input(&self, input: &str) -> Result<(), ProcessError> {
		let stdin_pipe = self.stdin_pipe.ok_or(ProcessError::PipeError)?;

		unsafe {
			let file = File::from_raw_handle(stdin_pipe.0 as *mut _);
			let mut file = tokio::fs::File::from_std(file);

			if let Some(timeout) = self.config.write_timeout {
				tokio::time::timeout(
					timeout,
					async {
						file.write_all(input// Continuing from the write_input Windows implementation...
							.as_bytes()).await?;
						file.write_all(b"\n").await?;
						file.flush().await?;
						Ok::<(), std::io::Error>(())
					},
				).await.map_err(|_| ProcessError::IoError(
					std::io::Error::new(std::io::ErrorKind::TimedOut, "Write timed out")
				))??;
			} else {
				file.write_all(input.as_bytes()).await?;
				file.write_all(b"\n").await?;
				file.flush().await?;
			}

			// Don't drop the file handle
			std::mem::forget(file);
		}

		Ok(())
	}

	/// Closes the process handle and cleans up resources
	#[cfg(windows)]
	pub async fn close(&mut self) -> Result<(), ProcessError> {
		unsafe {
			if let Some(stdin_pipe) = self.stdin_pipe.take() {
				CloseHandle(stdin_pipe);
			}
			if let Some(stdout_pipe) = self.stdout_pipe.take() {
				CloseHandle(stdout_pipe);
			}
			CloseHandle(self.process_handle);
		}
		Ok(())
	}
}

#[cfg(windows)]
unsafe fn create_named_pipe(name: &str) -> Result<HANDLE, ProcessError> {
	use std::ffi::CString;
	use windows::core::PCSTR;

	let name = CString::new(name).map_err(|_| ProcessError::PipeError)?;
	let pipe = CreateNamedPipeA(
		PCSTR(name.as_ptr() as *const u8),
		PIPE_ACCESS_DUPLEX,
		PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
		1,
		4096,
		4096,
		0,
		None,
	);

	if pipe == INVALID_HANDLE_VALUE {
		Err(ProcessError::PipeError)
	} else {
		Ok(pipe)
	}
}

#[cfg(windows)]
unsafe fn connect_pipe(pipe: HANDLE) -> Result<(), ProcessError> {
	if ConnectNamedPipe(pipe, None).is_ok() {
		Ok(())
	} else {
		Err(ProcessError::PipeError)
	}
}

/// Helper function to attach to an existing process
pub async fn attach_to_process(
	pid: u32,
	config: Option<ProcessConfig>,
) -> Result<AttachedProcess, ProcessError> {
	AttachedProcess::new(pid, config).await
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Duration;
	use tokio::time::sleep;

	#[tokio::test]
	async fn test_process_manager_creation() {
		let config = ProcessConfig {
			buffer_size: 8192,
			read_timeout: Some(Duration::from_secs(5)),
			write_timeout: Some(Duration::from_secs(5)),
		};

		#[cfg(windows)]
		let executable = "cmd.exe";
		#[cfg(unix)]
		let executable = "bash";

		let process = ProcessManager::new(
			PathBuf::from(executable),
			vec![],
			Some(config),
		).await;

		assert!(process.is_ok());
	}

	#[tokio::test]
	async fn test_process_io() {
		let mut process = ProcessManager::new(
			#[cfg(windows)]
			PathBuf::from("cmd.exe"),
			#[cfg(unix)]
			PathBuf::from("bash"),
			vec![],
			None,
		).await.unwrap();

		let (tx, mut rx) = mpsc::channel(100);
		let tx = Arc::new(Mutex::new(tx));

		process.start_output_handling(move |line| {
			let tx = tx.clone();
			tokio::spawn(async move {
				let tx = tx.lock().await;
				let _ = tx.send(line).await;
			});
		}).await.unwrap();

		// Give the process time to start
		sleep(Duration::from_millis(100)).await;

		#[cfg(windows)]
		process.write_input("echo test").await.unwrap();
		#[cfg(unix)]
		process.write_input("echo test").await.unwrap();

		// Wait for output
		let received = rx.recv().await;
		assert!(received.is_some());

		process.kill().await.unwrap();
	}
}