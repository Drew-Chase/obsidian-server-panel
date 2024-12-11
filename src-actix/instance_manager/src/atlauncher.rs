use crate::modpacks::{Modpack, ModpackSearchResults};
use crate::Platform;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ATLauncherPackSearchResults {
    pub data: Vec<ATLauncherPackItem>,
    pub error: bool,
    pub code: i32,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ATLauncherPackItem {
    pub id: i64,
    pub name: String,
    #[serde(rename = "safeName")]
    pub safe_name: String,
    pub description: Option<String>,
    pub versions: Vec<ATLauncherPackVersion>,
    #[serde(rename = "discordInviteURL")]
    pub discord_invite_url: Option<String>,
    #[serde(rename = "supportURL")]
    pub support_url: Option<String>,
    #[serde(rename = "websiteURL")]
    pub website_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ATLauncherPackVersion {
    pub version: String,
    pub minecraft: String,
    pub published: i64,
    #[serde(rename = "__LINK")]
    pub link: String,
}

impl ATLauncherPackSearchResults {
    pub async fn search() -> Result<Self, Box<dyn std::error::Error>> {
        let url = "https://api.atlauncher.com/v1/packs/full/all";

        Ok(reqwest::get(url).await?.json::<Self>().await?)
    }

    pub fn to_modpack_results(self) -> ModpackSearchResults {
        ModpackSearchResults {
            hits: self.clone().data.into_iter().map(|item| item.to_modpack()).collect(),
            offset: 0, // ATLauncher does not provide offset info
            limit: self.data.len() as i64,
            total_hits: self.data.len() as i64,
        }
    }
}

impl ATLauncherPackItem {
    pub fn to_modpack(self) -> Modpack {
        let latest_published = self.versions.first().map(|v| v.published).unwrap_or_default();

        let latest_published = chrono::DateTime::from_timestamp(latest_published, 0);

        Modpack {
            id: self.id.to_string(),
            name: self.name,
            description: self.description.unwrap_or_default(),
            icon: None,    // ATLauncher does not provide an icon
            gallery: None, // ATLauncher does not provide a gallery
            game_versions: Some(self.versions.iter().map(|v| v.minecraft.clone()).collect()),
            downloads: 0, // ATLauncher does not provide download count
            likes: None,  // ATLauncher does not provide likes/follows count
            last_updated: latest_published,
            published: latest_published,
            platform: Platform::AtLauncher,
            categories: None,      // ATLauncher does not provide categories
            project_url: None,     // ATLauncher does not provide project URL
            author: String::new(), // ATLauncher does not provide author info
            versions: Some(self.versions.iter().map(|v| v.version.clone()).collect()),
        }
    }
}
