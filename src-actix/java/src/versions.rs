use crate::data::{JavaVersionData, JavaVersions, OSVersions, RuntimeVersion, OS};
use log::{error, info};

pub fn get_os_version(os: OS, versions: &OSVersions) -> Option<&JavaVersions> {
    match os {
        OS::Linux => Some(&versions.linux),
        OS::LinuxI386 => Some(&versions.linux_i386),
        OS::MacOS => Some(&versions.macos),
        OS::MacOSArm64 => Some(&versions.macos_arm64),
        OS::WindowsArm64 => Some(&versions.windows_arm64),
        OS::WindowsX64 => Some(&versions.windows_x64),
        OS::WindowsX86 => Some(&versions.windows_x86),
    }
}

pub fn get_runtime_version(
    version: &JavaVersions,
    runtime: &RuntimeVersion,
) -> Option<JavaVersionData> {
    match runtime {
        RuntimeVersion::Alpha => version.alpha.first().cloned(),
        RuntimeVersion::Beta => version.beta.first().cloned(),
        RuntimeVersion::Delta => version.delta.first().cloned(),
        RuntimeVersion::Gamma => version.gamma.first().cloned(),
        RuntimeVersion::GammaSnapshot => version.gamma_snapshot.first().cloned(),
        RuntimeVersion::Legacy => version.legacy.first().cloned(),
    }
}

pub async fn list() -> Result<OSVersions, String> {
    info!("Getting Java versions");
    let url = "https://piston-meta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    };

    let v = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    };

    println!("Java versions: {}", v);

    let versions: OSVersions = match serde_json::from_str(v.as_str()) {
        Ok(versions) => versions,
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    };
    Ok(versions)
}
