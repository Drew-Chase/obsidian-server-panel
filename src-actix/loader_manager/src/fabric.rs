use std::io;
use std::path::Path;

const FABRIC_LOADER_API: &str = "https://meta.fabricmc.net/v2/versions/loader";
const FABRIC_INSTALLER_API: &str = "https://meta.fabricmc.net/v2/versions/installer";

pub async fn versions() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response = reqwest::get(FABRIC_LOADER_API).await?;
    let json: Vec<serde_json::Value> = response.json().await?;
    let versions = json
        .iter()
        .filter_map(|version| version["version"].as_str().map(|s| s.to_string()))
        .collect();
    Ok(versions)
}
async fn get_latest_installer_version() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(FABRIC_INSTALLER_API).await?;
    let json: Vec<serde_json::Value> = response.json().await?;
    let version = json
        .iter()
        .find_map(|v| v.get("version").and_then(|v| v.as_str()));
    if version.is_none() {
        return Err("Failed to get version".into());
    }
    Ok(version.unwrap().to_string())
}
async fn download_installer(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let response = reqwest::get(FABRIC_INSTALLER_API).await?;
    let json: Vec<serde_json::Value> = response.json().await?;
    let download_url = json
        .iter()
        .find_map(|v| v.get("url").and_then(|v| v.as_str()));

    if download_url.is_none() {
        return Err("Failed to get download url".into());
    }

    let response = reqwest::get(download_url.unwrap()).await?;
    let body = response.text().await?;
    let mut file = std::fs::File::create(path)?;
    io::copy(&mut body.as_bytes(), &mut file)?;

    Ok(())
}
pub async fn install(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let latest_installer_version = get_latest_installer_version().await?;
    let path = format!(
        "meta/loaders/installers/fabric/fabric-installer-{}.jar",
        latest_installer_version
    );
    let path = Path::new(path.as_str());
    if !path.exists() {
        download_installer(path).await?;
    }

    todo!("installing fabric version {}", version)
}
