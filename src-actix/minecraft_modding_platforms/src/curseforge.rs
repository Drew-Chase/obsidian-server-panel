use crate::modpacks::{BrowseOptions, Modpack, ModpackSearchResults};
use crate::Platform;
use reqwest::header;
use serde::{Deserialize, Serialize};
const CURSEFORGE_API_KEY: &str = r#"$2a$10$qD2UJdpHaeDaQyGGaGS0QeoDnKq2EC7sX6YSjOxYHtDZSQRg04BCG"#;

#[derive(Serialize, Deserialize, Clone)]
pub struct CurseForgePackSearchResults {
    pub data: Vec<CurseForgePackItem>,
    pub pagination: CurseForgePagination,
}

#[derive(Serialize, Deserialize, Clone)]
struct CurseForgePackItem {
    pub id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub name: String,
    pub slug: String,
    pub summary: String,
    #[serde(rename = "downloadCount")]
    pub download_count: i64,
    pub categories: Vec<CurseForgeCategory>,
    pub authors: Vec<CurseForgeAuthor>,
    pub logo: CurseForgeLogo,
    #[serde(rename = "latestFiles")]
    pub latest_files: Vec<CurseForgeFile>,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "dateReleased")]
    pub date_released: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct CurseForgeCategory {
    pub name: String,
    pub url: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct CurseForgeAuthor {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct CurseForgeLogo {
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct CurseForgeFile {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fileDate")]
    pub file_date: String,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct CurseForgePagination {
    pub index: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}

impl CurseForgePackSearchResults {
    pub async fn search(options: BrowseOptions) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!(
			"https://api.curseforge.com/v1/mods/search?searchFilter={}&gameId=432&classId=4471&gameVersion=1.16.5&modLoaderType=1&pageSize={}&index={}",
			options.search, options.limit, options.offset
		);

        let api_key = CURSEFORGE_API_KEY;
        let client = reqwest::Client::new();
        let mut headers = header::HeaderMap::new();
        headers.insert("x-api-key", header::HeaderValue::from_str(api_key)?);

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response.json::<Self>().await?)
    }

    pub fn to_modpack_results(self) -> ModpackSearchResults {
        ModpackSearchResults {
            hits: self
                .data
                .into_iter()
                .map(|item| item.to_modpack())
                .collect(),
            offset: self.pagination.index,
            limit: self.pagination.page_size,
            total_hits: self.pagination.total_count,
        }
    }
}

impl CurseForgePackItem {
    pub fn to_modpack(self) -> Modpack {
        Modpack {
            id: self.id.to_string(),
            name: self.name,
            description: self.summary,
            icon: Some(self.logo.thumbnail_url),
            gallery: None, // CurseForge API response in provided example does not have a gallery field
            game_versions: Some(
                self.latest_files
                    .iter()
                    .flat_map(|file| file.game_versions.clone())
                    .collect(),
            ),
            downloads: self.download_count as u32,
            likes: None, // The provided CurseForge API response example does not include a likes field, change as necessary
            last_updated: self
                .date_modified
                .parse::<chrono::DateTime<chrono::Utc>>()
                .ok(),
            published: self
                .date_created
                .parse::<chrono::DateTime<chrono::Utc>>()
                .ok(),
            platform: Platform::Curseforge,
            categories: Some(self.categories.into_iter().map(|cat| cat.name).collect()),
            project_url: Some(format!(
                "https://www.curseforge.com/minecraft/modpacks/{}",
                self.slug
            )),
            author: self
                .authors
                .into_iter()
                .map(|author| author.name)
                .collect::<Vec<String>>()
                .join(", "),
            versions: Some(
                self.latest_files
                    .into_iter()
                    .map(|file| file.display_name)
                    .collect(),
            ),
        }
    }
}
