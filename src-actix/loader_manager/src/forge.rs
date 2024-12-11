use std::error::Error;
use std::path::Path;

const FORGE_VERSIONS_MANIFEST: &str = "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json";

pub async fn versions(minecraft_version: impl AsRef<str>) -> Result<Vec<String>, Box<dyn Error>> {
    let response = reqwest::get(FORGE_VERSIONS_MANIFEST).await?;
    let json: serde_json::Value = response.json().await?;
    let mc_versions: Vec<String> = json
        .as_object()
        .unwrap()
        .keys()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for mc_version in mc_versions {
        if mc_version.contains(minecraft_version.as_ref()) {
            let versions = json[&mc_version].as_array().unwrap();
            return Ok(versions
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect::<Vec<String>>());
        }
    }

    Ok(vec![])
}
pub fn download(version: &str) -> Result<(), Box<dyn Error>> {
    todo!("download forge version {}", version)
}
pub async fn install(mc_version: &str, dir: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    todo!("install forge version {}", mc_version)
}
