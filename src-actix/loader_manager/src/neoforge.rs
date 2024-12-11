use std::path::Path;

pub fn versions() -> Vec<String> {
    todo!("return all versions of neoforge")
}
pub fn download(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    todo!("download neoforge version {}", version)
}
pub async fn install(mc_version: &str, dir: impl AsRef<Path>) -> Result<String, Box<dyn std::error::Error>> {
    todo!("install neoforge version {}", mc_version)
}
