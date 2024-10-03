use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct File {
	pub id: Option<String>,
	pub sha1: Option<String>,
	pub size: Option<i64>,
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Client {
	pub argument: Option<String>,
	pub file: Option<File>,
	#[serde(rename = "type")]
	pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Logging {
	pub client: Option<Client>,
}

#[derive(Serialize, Deserialize)]
pub struct Os {
	pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct2 {
	pub action: Option<String>,
	pub os: Option<Os>,
}

#[derive(Serialize, Deserialize)]
pub struct Artifact {
	pub path: Option<String>,
	pub sha1: Option<String>,
	pub size: Option<i64>,
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Downloads1 {
	pub artifact: Option<Artifact>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct1 {
	pub downloads: Option<Downloads1>,
	pub name: Option<String>,
	pub rules: Option<Vec<Struct2>>,
}

#[derive(Serialize, Deserialize)]
pub struct JavaVersion {
	pub component: Option<String>,
	#[serde(rename = "majorVersion")]
	pub major_version: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
	pub sha1: Option<String>,
	pub size: Option<i64>,
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Downloads {
	pub client: Option<Struct>,
	pub client_mappings: Option<Struct>,
	pub server: Option<Struct>,
	pub server_mappings: Option<Struct>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetIndex {
	pub id: Option<String>,
	pub sha1: Option<String>,
	pub size: Option<i64>,
	#[serde(rename = "totalSize")]
	pub total_size: Option<i64>,
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VersionAssetData {
	#[serde(rename = "assetIndex")]
	pub asset_index: Option<AssetIndex>,
	pub assets: Option<String>,
	#[serde(rename = "complianceLevel")]
	pub compliance_level: Option<i64>,
	pub downloads: Option<Downloads>,
	pub id: Option<String>,
	#[serde(rename = "javaVersion")]
	pub java_version: Option<JavaVersion>,
	pub libraries: Option<Vec<Struct1>>,
	pub logging: Option<Logging>,
	#[serde(rename = "mainClass")]
	pub main_class: Option<String>,
	#[serde(rename = "minimumLauncherVersion")]
	pub minimum_launcher_version: Option<i64>,
	#[serde(rename = "releaseTime")]
	pub release_time: Option<String>,
	pub time: Option<String>,
	#[serde(rename = "type")]
	pub r#type: Option<String>,
}
