use crate::data::{JavaVersionData, Manifest, OSVersions, RuntimeVersion, OS};
use futures::stream::StreamExt;
use log::{debug, error, info, warn};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{fs::File, io::Write, sync::Arc};
use futures::stream;
use shellwords::split;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use common_lib::traits::TransformPath;

#[derive(Serialize, Deserialize)]
pub struct JavaVersion {
    pub operating_system: OS,
    pub runtime: RuntimeVersion,
    pub version: String,
    pub installed: bool,
    #[serde(skip_serializing)]
    manifest: Manifest,
    executable: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadProgress {
    pub file: String,
    pub completed: bool,
}

impl JavaVersion {
    pub async fn list() -> Result<Vec<Self>, Box<dyn Error>> {
        let mut versions: Vec<Self> = vec![];
        let response = reqwest::get("https://piston-meta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json").await?;
        let data: OSVersions = response.json().await?;
        let current_os = get_current_os().ok_or("Unsupported OS")?;
        let current_os_versions = match current_os {
            OS::Linux => data.linux,
            OS::LinuxI386 => data.linux_i386,
            OS::MacOS => data.macos,
            OS::MacOSArm64 => data.macos_arm64,
            OS::WindowsArm64 => data.windows_arm64,
            OS::WindowsX64 => data.windows_x64,
            OS::WindowsX86 => data.windows_x86,
        };

        Self::append_versions(
            &mut versions,
            current_os_versions.alpha,
            current_os,
            RuntimeVersion::Alpha,
        );
        Self::append_versions(
            &mut versions,
            current_os_versions.beta,
            current_os,
            RuntimeVersion::Beta,
        );
        Self::append_versions(
            &mut versions,
            current_os_versions.delta,
            current_os,
            RuntimeVersion::Delta,
        );
        Self::append_versions(
            &mut versions,
            current_os_versions.gamma,
            current_os,
            RuntimeVersion::Gamma,
        );
        Self::append_versions(
            &mut versions,
            current_os_versions.gamma_snapshot,
            current_os,
            RuntimeVersion::GammaSnapshot,
        );
        Self::append_versions(
            &mut versions,
            current_os_versions.legacy,
            current_os,
            RuntimeVersion::Legacy,
        );

        Ok(versions)
    }

    fn append_versions(
        versions: &mut Vec<Self>,
        version_data: Vec<JavaVersionData>,
        os: OS,
        runtime: RuntimeVersion,
    ) {
        for version in version_data {
            let mut vcard = JavaVersion {
                operating_system: os,
                runtime,
                version: version.version.name,
                installed: false,
                manifest: version.manifest,
                executable: None,
            };
            vcard.check_if_version_installed();
            versions.push(vcard);
        }
    }

    pub async fn from_runtime(runtime: impl AsRef<str>) -> Result<Self, Box<dyn Error>> {
        let runtime = runtime.as_ref();
        let all_versions = Self::list().await?;
        for version in all_versions {
            if version.runtime.to_string() == runtime {
                return Ok(version);
            }
        }
        Err("Version not found".into())
    }

    pub async fn from_version(version: impl AsRef<str>) -> Result<Self, Box<dyn Error>> {
        let version_id = version.as_ref();
        let all_versions = Self::list().await?;
        for version in all_versions {
            if version.version == version_id {
                return Ok(version);
            }
        }
        Err("Version not found".into())
    }

    fn check_if_version_installed(&mut self) -> bool {
        self.installed = self.get_executable().exists();
        self.installed
    }

    fn get_installation_directory(&self) -> PathBuf {
        let path =
            std::fs::canonicalize("meta/java").unwrap_or_else(|_| PathBuf::from("meta/java"));
        let path = path.join(format!("{}-{}", self.runtime, self.version));
        path
    }

    fn get_executable(&mut self) -> PathBuf {
        let path = self.get_installation_directory();
        let executable = if cfg!(windows) {
            path.join("bin/java.exe")
        } else {
            path.join("bin/java")
        };
        if !executable.exists() {
            self.executable = None;
        } else {
            self.executable = Some(executable.clone());
        }
        executable
    }

    pub async fn get_installed_versions() -> Result<Vec<Self>, Box<dyn Error>> {
        let mut versions: Vec<Self> = vec![];
        let all_versions = Self::list().await?;
        for mut version in all_versions {
            if version.check_if_version_installed() {
                versions.push(version);
            }
        }
        Ok(versions)
    }

    pub async fn execute_command<F>(
        &mut self,
        args: impl AsRef<str>,
        callback: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(String) + Send + 'static,
    {
        let executable = self.get_executable().to_path_buf().normalize();
        if !executable.exists() {
            return Err("Java executable not found".into());
        }
        
        println!("Executing command: {} {}", executable.display(), args.as_ref());

        let args = args.as_ref();
        let mut command = Command::new(executable)
            .args(split(args)?)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        // Create and write the PID to a PID file
        let pid = command.id().ok_or("Failed to get process ID")?;
        let pid_file_path = format!("./{}.pid", pid);
        let mut pid_file = File::create(&pid_file_path)?;
        writeln!(pid_file, "{}", pid)?;

        let stdout = command.stdout.take().ok_or("Failed to capture stdout")?;
        let stderr = command.stderr.take().ok_or("Failed to capture stderr")?;

        let (tx, rx) = mpsc::channel(100);
        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    line = stdout_reader.next_line() => {
                        match line {
                            Ok(Some(content)) => {
                                if tx.send(content).await.is_err() {
                                    break;
                                }
                            },
                            _ => break,
                        }
                    },
                    line = stderr_reader.next_line() => {
                        match line {
                            Ok(Some(content)) => {
                                if tx.send(content).await.is_err() {
                                    break;
                                }
                            },
                            _ => break,
                        }
                    },
                }
            }
        });

        let mut stream = ReceiverStream::new(rx);
        while let Some(line) = stream.next().await {
            callback(line);
        }

        Ok(())
    }

    pub async fn install<F>(&self, callback: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(Vec<DownloadProgress>) + 'static + Send + Sync,
    {
        let now = std::time::Instant::now();
        warn!("Installing Java {}, this may take some time!", self.version);
        let url = &self.manifest.url;
        let client = Client::new();
        let response = client.get(url).send().await?;
        let json = response.json::<Value>().await?;
        let files = json.get("files").ok_or("No files found")?;
        let keys = files
            .as_object()
            .ok_or("Files not an object")?
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        let files = Arc::new(files.clone());
        let client = Arc::new(Client::new());
        let progress: Vec<DownloadProgress> = keys
            .iter()
            .map(|key| DownloadProgress {
                file: key.clone(),
                completed: false,
            })
            .collect();

        let callback = Arc::new(Mutex::new(callback));
        let progress = Arc::new(Mutex::new(progress));

        // Using a concurrent stream to download files.
        stream::iter(keys)
            .for_each_concurrent(10, move |key| {
                let files = Arc::clone(&files);
                let client = Arc::clone(&client);
                let progress = Arc::clone(&progress);
                let callback = Arc::clone(&callback);

                async move {
                    if let Err(e) = self
                        .download_installation_file(client, files, &key, move |item| {
                            let mut progress = progress.lock().unwrap();
                            let index = progress.iter().position(|x| x.file == item.file).unwrap();
                            progress[index] = item;

                            let callback = callback.lock().unwrap();
                            callback(progress.clone());
                        })
                        .await
                    {
                        error!("Error downloading file {}: {:?}", key, e);
                    }
                }
            })
            .await;

        let duration = now.elapsed();
        info!("Java {} installed in {:?}", self.version, duration);
        Ok(())
    }

    pub fn uninstall(&self) -> Result<(), Box<dyn Error>> {
        let path = self.get_installation_directory();
        if path.exists() {
            std::fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    async fn download_installation_file<F>(
        &self,
        client: Arc<Client>,
        files: Arc<Value>,
        key: &String,
        callback: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(DownloadProgress) + 'static + Send + Sync,
    {
        let file = files.get(key).ok_or("File not found")?;
        let downloads = file.get("downloads").ok_or("No downloads found")?;
        let r#type = file.get("type").ok_or("No type found")?;
        let is_directory = r#type == "directory";
        let raw = downloads.get("raw").ok_or("No raw download found")?;
        let url = raw.get("url").ok_or("No url found")?;
        let url = url.as_str().ok_or("Url not a string")?;

        let path = self.get_installation_directory().join(key);
        let response = client.get(url).send().await?;

        info!("Creating {} {}", r#type, path.display());

        if is_directory {
            tokio::fs::create_dir_all(&path).await?;
        } else {
            debug!("Downloading file {}", key);
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    tokio::fs::create_dir_all(parent).await?;
                }
            }
            let mut file = File::create(&path)?;
            let content = response.bytes().await?;
            file.write_all(&content)?;
        }

        callback(DownloadProgress {
            file: key.clone(),
            completed: true,
        });

        Ok(())
    }

    pub async fn get_installation_files(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let response = reqwest::get(&self.manifest.url).await?;
        let json = response.json::<Value>().await?;
        let files = json.get("files").ok_or("No files found")?;
        Ok(files
            .as_object()
            .ok_or("Files not an object")?
            .iter()
            .filter(|(_, v)| v.get("type") != Some(&Value::String("directory".to_string())))
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>())
    }
}

fn get_current_os() -> Option<OS> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86") {
            return Some(OS::LinuxI386);
        }
        return Some(OS::Linux);
    }
    if cfg!(target_os = "macos") {
        if cfg!(target_arch = "arm") {
            return Some(OS::MacOSArm64);
        }
        return Some(OS::MacOS);
    }
    if cfg!(target_os = "windows") {
        if cfg!(target_arch = "x86") {
            return Some(OS::WindowsX86);
        }
        if cfg!(target_arch = "arm") {
            return Some(OS::WindowsArm64);
        }
        return Some(OS::WindowsX64);
    }
    None
}
