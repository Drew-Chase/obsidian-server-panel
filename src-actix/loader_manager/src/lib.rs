pub mod fabric;
pub mod forge;
pub mod neoforge;
pub mod quilt;
pub mod supported_loaders;

use crate::supported_loaders::Loader;
use std::path::Path;

/// Installs the specified loader.
///
/// # Arguments
///
/// * `loader` - The loader to install.
/// * `mc_version` - The Minecraft version.
/// * `dir` - The directory where the loader will be installed.
/// * `loader_version` - The optional loader version.
///
/// # Returns
///
/// A Result containing the installation result as a String or an error.
pub async fn install_loader(
    loader: Loader,
    mc_version: &str,
    dir: impl AsRef<Path>,
    loader_version: Option<impl AsRef<str>>,
) -> Result<String, Box<dyn std::error::Error>> {
    match loader {
        Loader::Forge => forge::install(mc_version, dir).await,
        Loader::Fabric => fabric::install(mc_version, loader_version.unwrap().as_ref(), dir).await,
        Loader::NeoForge => neoforge::install(mc_version, dir).await,
        Loader::Quilt => quilt::install(mc_version, dir).await,
        _ => Err("Loader not supported".into()),
    }
}
