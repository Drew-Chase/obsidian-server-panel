use crate::data::{JavaVersionData, OSVersions, RuntimeVersion, OS};
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct JavaVersion {
    pub operating_system: OS,
    pub runtime: RuntimeVersion,
    pub version: String,
    pub installed: bool,
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
            versions.push(JavaVersion {
                operating_system: os,
                runtime,
                version: version.version.name,
                installed: false,
            });
        }
    }

    pub async fn from_runtime(runtime: impl AsRef<str>) -> Result<Self, Box<dyn Error>> {
        let runtime = runtime.as_ref();
        let all_versions = Self::list().await?;
        for version in all_versions {
            if version.version == runtime {
                return Ok(version);
            }
        }
        Err("Version not found".into())
    }

    pub async fn install(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
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
