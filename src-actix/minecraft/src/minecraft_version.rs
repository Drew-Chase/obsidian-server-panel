use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestMinecraftVersions {
	#[serde(rename = "release")]
	pub release: String,
	#[serde(rename = "snapshot")]
	pub snapshot: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftVersionManifest {
	#[serde(rename = "latest")]
	pub latest: LatestMinecraftVersions,
	#[serde(rename = "versions")]
	pub versions: Vec<MinecraftVersion>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftVersion {
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "type")]
	pub r#type: MinecraftVersionType,
	#[serde(rename = "url")]
	pub url: Option<String>,
	#[serde(rename = "latest")]
	pub latest: Option<bool>,
	#[serde(rename = "time")]
	pub time: Option<String>,
	#[serde(rename = "releaseTime")]
	pub release_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MinecraftVersionType {
	#[serde(rename = "release")]
	Release,
	#[serde(rename = "snapshot")]
	Snapshot,
	#[serde(rename = "old_beta")]
	OldBeta,
	#[serde(rename = "old_alpha")]
	OldAlpha,
}

pub async fn get_versions() -> Result<Vec<MinecraftVersion>, String> {
	let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
	let response = match reqwest::get(url).await {
		Ok(response) => response,
		Err(e) => {
			return Err(format!("Error: {}", e));
		}
	};
	let text: String = match response.text().await {
		Ok(text) => text,
		Err(e) => {
			return Err(format!("Error: {}", e));
		}
	};

	let manifest: MinecraftVersionManifest = match serde_json::from_str(&text) {
		Ok(manifest) => manifest,
		Err(e) => {
			return Err(format!("Error: {}", e));
		}
	};

	let mut results: Vec<MinecraftVersion> = manifest.versions;
	let latest_release_id = manifest.latest.release;
	let latest_snapshot_id = manifest.latest.snapshot;

	let mut found_release = false;
	let mut found_snapshot = false;

	for version in results.iter_mut() {
		if version.id == latest_release_id {
			version.latest = Some(true);
			found_release = true;
		}
		if version.id == latest_snapshot_id {
			version.latest = Some(true);
			found_snapshot = true;
		}
		if found_release && found_snapshot {
			break;
		}
	}

	Ok(results)
}

pub async fn get_list_of_versions() -> Result<Vec<String>, String> {
	let versions = get_versions().await?;
	let mut version_list: Vec<String> = Vec::new();
	for version in versions {
		version_list.push(version.id);
	}
	Ok(version_list)
}

pub async fn get_latest_release() -> Result<MinecraftVersion, String> {
	let versions = get_versions().await?;
	for version in versions {
		if version.latest == Some(true) {
			return Ok(version);
		}
	}
	Err("No latest release found".to_string())
}

pub async fn get_latest_snapshot() -> Result<MinecraftVersion, String> {
	let versions = get_versions().await?;
	for version in versions {
		if version.latest == Some(true) {
			return Ok(version);
		}
	}
	Err("No latest snapshot found".to_string())
}

pub async fn get_version_by_id(
	id: &str,
	snapshot: Option<bool>,
) -> Result<MinecraftVersion, String> {
	let versions = match get_versions().await {
		Ok(versions) => versions,
		Err(e) => {
			return Err(e);
		}
	};
	for version in versions {
		if version.id == id {
			if snapshot.is_some() {
				if snapshot.unwrap() && version.r#type == MinecraftVersionType::Snapshot {
					return Ok(version);
				} else if !snapshot.unwrap() && version.r#type == MinecraftVersionType::Release {
					return Ok(version);
				}
			} else {
				return Ok(version);
			}
		}
	}
	Err("Version not found".to_string())
}
