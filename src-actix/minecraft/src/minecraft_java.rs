use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Version {
	pub name: String,
	pub released: String,
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
	pub sha1: String,
	pub size: i64,
	pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Availability {
	pub group: i64,
	pub progress: i64,
}

#[derive(Serialize, Deserialize)]
pub struct JavaVersionData {
	pub availability: Availability,
	pub manifest: Manifest,
	pub version: Version,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

pub async fn get_java_versions() -> Result<OSVersions, String> {
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
