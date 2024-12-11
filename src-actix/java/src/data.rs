use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone)]
pub struct Version {
    pub name: String,
    pub released: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub sha1: String,
    pub size: i64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Availability {
    pub group: i64,
    pub progress: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JavaVersionData {
    pub availability: Availability,
    #[serde(rename = "manifest")]
    pub manifest: Manifest,
    #[serde(rename = "version")]
    pub version: Version,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JavaVersions {
    #[serde(rename = "java-runtime-alpha")]
    pub alpha: Vec<JavaVersionData>,
    #[serde(rename = "java-runtime-beta")]
    pub beta: Vec<JavaVersionData>,
    #[serde(rename = "java-runtime-delta")]
    pub delta: Vec<JavaVersionData>,
    #[serde(rename = "java-runtime-gamma")]
    pub gamma: Vec<JavaVersionData>,
    #[serde(rename = "java-runtime-gamma-snapshot")]
    pub gamma_snapshot: Vec<JavaVersionData>,
    #[serde(rename = "jre-legacy")]
    pub legacy: Vec<JavaVersionData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OSVersions {
    pub linux: JavaVersions,
    #[serde(rename = "linux-i386")]
    pub linux_i386: JavaVersions,
    #[serde(rename = "mac-os")]
    pub macos: JavaVersions,
    #[serde(rename = "mac-os-arm64")]
    pub macos_arm64: JavaVersions,
    #[serde(rename = "windows-arm64")]
    pub windows_arm64: JavaVersions,
    #[serde(rename = "windows-x64")]
    pub windows_x64: JavaVersions,
    #[serde(rename = "windows-x86")]
    pub windows_x86: JavaVersions,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum RuntimeVersion {
    #[serde(rename = "alpha")]
    Alpha,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "delta")]
    Delta,
    #[serde(rename = "gamma")]
    Gamma,
    #[serde(rename = "gamma-snapshot")]
    GammaSnapshot,
    #[serde(rename = "legacy")]
    Legacy,
}

impl Display for RuntimeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RuntimeVersion::Alpha => "alpha".to_string(),
            RuntimeVersion::Beta => "beta".to_string(),
            RuntimeVersion::Delta => "delta".to_string(),
            RuntimeVersion::Gamma => "gamma".to_string(),
            RuntimeVersion::GammaSnapshot => "gamma-snapshot".to_string(),
            RuntimeVersion::Legacy => "legacy".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum OS {
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "linux-i386")]
    LinuxI386,
    #[serde(rename = "mac-os")]
    MacOS,
    #[serde(rename = "mac-os-arm64")]
    MacOSArm64,
    #[serde(rename = "windows-arm64")]
    WindowsArm64,
    #[serde(rename = "windows-x64")]
    WindowsX64,
    #[serde(rename = "windows-x86")]
    WindowsX86,
}
