use log::info;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Loader {
    Vanilla,
    Fabric,
    Forge,
    Quilt,
    NeoForge,
    Spigot,
    Paper,
}

impl Loader {
    pub fn all() -> Vec<Self> {
        vec![
            Loader::Vanilla,
            Loader::Fabric,
            Loader::Forge,
            Loader::Quilt,
            Loader::NeoForge,
            Loader::Spigot,
            Loader::Paper,
        ]
    }
    pub fn from<T>(loader: T) -> Option<Self>
    where
        T: Into<u8>,
    {
        match loader.into() {
            0 => Some(Loader::Vanilla),
            1 => Some(Loader::Fabric),
            2 => Some(Loader::Forge),
            3 => Some(Loader::Quilt),
            4 => Some(Loader::NeoForge),
            5 => Some(Loader::Spigot),
            6 => Some(Loader::Paper),
            _ => None,
        }
    }
    pub fn to(&self) -> u8 {
        match self {
            Loader::Vanilla => 0,
            Loader::Fabric => 1,
            Loader::Forge => 2,
            Loader::Quilt => 3,
            Loader::NeoForge => 4,
            Loader::Spigot => 5,
            Loader::Paper => 6,
        }
    }
    pub fn supported_by_minecraft_version(&self, version: &str, snapshot: bool) -> bool {
        let major_version = match version.split('.').skip(1).take(1).next() {
            Some(v) => v.parse::<u8>().unwrap_or(0),
            None => return false,
        };
        info!("Major version: {}", major_version);
        match self {
            Loader::Fabric => major_version >= 14,
            Loader::Forge => !snapshot,
            Loader::Quilt => major_version >= 16,
            Loader::NeoForge => major_version >= 16 && !snapshot,
            Loader::Spigot => major_version >= 5 && !snapshot,
            Loader::Paper => major_version >= 8 && !snapshot,
            _ => true,
        }
    }
}

impl FromStr for Loader {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "VANILLA" => Ok(Loader::Vanilla),
            "FABRIC" => Ok(Loader::Fabric),
            "FORGE" => Ok(Loader::Forge),
            "QUILT" => Ok(Loader::Quilt),
            "NEOFORGE" => Ok(Loader::NeoForge),
            "SPIGOT" => Ok(Loader::Spigot),
            "PAPER" => Ok(Loader::Paper),
            _ => Err(()),
        }
    }
}
