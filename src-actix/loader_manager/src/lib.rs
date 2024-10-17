mod fabric;
mod forge;
mod neoforge;
mod quilt;

use std::fmt::Display;
use std::str::FromStr;


pub enum Loaders {
    FORGE,
    FABRIC,
    NEOFORGE,
    QUILT,
}

impl Display for Loaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Loaders::FORGE => write!(f, "Forge"),
            Loaders::FABRIC => write!(f, "Fabric"),
            Loaders::NEOFORGE => write!(f, "NeoForge"),
            Loaders::QUILT => write!(f, "Quilt"),
        }
    }
}

impl FromStr for Loaders {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forge" => Ok(Loaders::FORGE),
            "fabric" => Ok(Loaders::FABRIC),
            "neoforge" => Ok(Loaders::NEOFORGE),
            "quilt" => Ok(Loaders::QUILT),
            _ => Err(()),
        }
    }
}
