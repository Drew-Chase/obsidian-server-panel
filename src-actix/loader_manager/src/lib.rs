pub mod fabric;
pub mod forge;
pub mod neoforge;
pub mod quilt;

use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

/// Represents the various loaders available in the system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Loaders {
    VANILLA,
    FORGE,
    FABRIC,
    NEOFORGE,
    QUILT,
}

impl Display for Loaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Loaders::VANILLA => write!(f, "Vanilla"),
            Loaders::FORGE => write!(f, "Forge"),
            Loaders::FABRIC => write!(f, "Fabric"),
            Loaders::NEOFORGE => write!(f, "NeoForge"),
            Loaders::QUILT => write!(f, "Quilt"),
        }
    }
}

impl FromStr for Loaders {
    type Err = ();

    /// Parses a string into a Loaders enum.
    ///
    /// # Examples
    ///
    /// ```
    /// let loader: Loaders = "forge".parse().unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vanilla" => Ok(Loaders::VANILLA),
            "forge" => Ok(Loaders::FORGE),
            "fabric" => Ok(Loaders::FABRIC),
            "neoforge" => Ok(Loaders::NEOFORGE),
            "quilt" => Ok(Loaders::QUILT),
            _ => Err(()),
        }
    }
}

/// Converts a u8 into a Loaders enum.
///
/// # Arguments
///
/// * `loader` - A u8 representing a loader.
///
/// # Returns
///
/// An Option containing the corresponding Loaders enum, or None if the value is invalid.
pub fn from_u8(loader: u8) -> Option<Loaders> {
    match loader {
        0 => Some(Loaders::VANILLA),
        1 => Some(Loaders::FORGE),
        2 => Some(Loaders::FABRIC),
        3 => Some(Loaders::NEOFORGE),
        4 => Some(Loaders::QUILT),
        _ => None,
    }
}

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
    loader: Loaders,
    mc_version: &str,
    dir: impl AsRef<Path>,
    loader_version: Option<impl AsRef<str>>,
) -> Result<String, Box<dyn std::error::Error>> {
    match loader {
        Loaders::FORGE => forge::install(mc_version, dir).await,
        Loaders::FABRIC => fabric::install(mc_version, loader_version.unwrap().as_ref(), dir).await,
        Loaders::NEOFORGE => neoforge::install(mc_version, dir).await,
        Loaders::QUILT => quilt::install(mc_version, dir).await,
        _ => Err("Loader not supported".into()),
    }
}
