use std::str::FromStr;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportedLoaders {
    FABRIC,
    FORGE,
    QUILT,
    NEOFORGE,
    SPIGOT,
    PAPER,
}

impl SupportedLoaders {
    pub fn all() -> Vec<Self> {
        vec![
            SupportedLoaders::FABRIC,
            SupportedLoaders::FORGE,
            SupportedLoaders::QUILT,
            SupportedLoaders::NEOFORGE,
            SupportedLoaders::SPIGOT,
            SupportedLoaders::PAPER,
        ]
    }
    pub fn from<T>(loader: T) -> Option<Self>
    where
        T: Into<u8>,
    {
        match loader.into() {
            0 => Some(SupportedLoaders::FABRIC),
            1 => Some(SupportedLoaders::FORGE),
            2 => Some(SupportedLoaders::QUILT),
            3 => Some(SupportedLoaders::NEOFORGE),
            4 => Some(SupportedLoaders::SPIGOT),
            5 => Some(SupportedLoaders::PAPER),
            _ => None,
        }
    }
    pub fn to(&self) -> u8 {
        match self {
            SupportedLoaders::FABRIC => 0,
            SupportedLoaders::FORGE => 1,
            SupportedLoaders::QUILT => 2,
            SupportedLoaders::NEOFORGE => 3,
            SupportedLoaders::SPIGOT => 4,
            SupportedLoaders::PAPER => 5,
        }
    }
}


impl FromStr for SupportedLoaders {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FABRIC" => Ok(SupportedLoaders::FABRIC),
            "FORGE" => Ok(SupportedLoaders::FORGE),
            "QUILT" => Ok(SupportedLoaders::QUILT),
            "NEOFORGE" => Ok(SupportedLoaders::NEOFORGE),
            "SPIGOT" => Ok(SupportedLoaders::SPIGOT),
            "PAPER" => Ok(SupportedLoaders::PAPER),
            _ => Err(()),
        }
    }
}