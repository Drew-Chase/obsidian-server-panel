pub mod fabric;
pub mod forge;
pub mod neoforge;
pub mod quilt;

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

/// Represents the various loaders available in the system.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
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
    /// ```no-error
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

pub trait FromU8 {
    fn from_u8(value: u8) -> Option<Self>
    where
        Self: Sized;
}

impl FromU8 for Loaders {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Loaders::VANILLA),
            1 => Some(Loaders::FORGE),
            2 => Some(Loaders::FABRIC),
            3 => Some(Loaders::NEOFORGE),
            4 => Some(Loaders::QUILT),
            _ => None,
        }
    }
}

pub trait AsU8 {
    fn as_u8(&self) -> u8;
}

impl AsU8 for Loaders {
    fn as_u8(&self) -> u8 {
        match self {
            Loaders::VANILLA => 0,
            Loaders::FORGE => 1,
            Loaders::FABRIC => 2,
            Loaders::NEOFORGE => 3,
            Loaders::QUILT => 4,
        }
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
