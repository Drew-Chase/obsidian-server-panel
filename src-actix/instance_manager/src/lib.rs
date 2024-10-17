pub(crate) mod atlauncher;
pub(crate) mod curseforge;
pub mod modpacks;
pub(crate) mod modrinth;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub enum Platform {
    All,
    Modrinth,
    Curseforge,
    AtLauncher,
}
