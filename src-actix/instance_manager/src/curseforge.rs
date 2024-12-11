use crate::modpacks::{BrowseOptions, Modpack, ModpackSearchResults};
use crate::Platform;
use reqwest::header;
use serde::{Deserialize, Serialize};
const CURSEFORGE_API_KEY: &str = r#"$2a$10$qD2UJdpHaeDaQyGGaGS0QeoDnKq2EC7sX6YSjOxYHtDZSQRg04BCG"#;

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub index: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    #[serde(rename = "resultCount")]
    pub result_count: i64,
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FileIndex {
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "fileId")]
    pub file_id: i64,
    pub filename: String,
    #[serde(rename = "releaseType")]
    pub release_type: i64,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: i64,
    #[serde(rename = "modLoader")]
    pub mod_loader: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct ExtraFile {
    pub name: String,
    pub fingerprint: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GameVersion {
    #[serde(rename = "gameVersionName")]
    pub game_version_name: String,
    #[serde(rename = "gameVersionPadded")]
    pub game_version_padded: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "gameVersionReleaseDate")]
    pub game_version_release_date: String,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FileHash {
    pub value: String,
    pub algo: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FileItem {
    pub id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "modId")]
    pub mod_id: i64,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "releaseType")]
    pub release_type: i64,
    #[serde(rename = "fileStatus")]
    pub file_status: i64,
    pub hashes: Vec<FileHash>,
    #[serde(rename = "fileDate")]
    pub file_date: String,
    #[serde(rename = "fileLength")]
    pub file_length: i64,
    #[serde(rename = "downloadCount")]
    pub download_count: i64,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "sortableGameVersions")]
    pub sortable_game_versions: Vec<GameVersion>,
    #[serde(rename = "alternateFileId")]
    pub alternate_file_id: i64,
    #[serde(rename = "isServerPack")]
    pub is_server_pack: bool,
    #[serde(rename = "fileFingerprint")]
    pub file_fingerprint: i64,
    pub modules: Vec<ExtraFile>,
}

#[derive(Serialize, Deserialize)]
pub struct ImageItem {
    pub id: i64,
    #[serde(rename = "modId")]
    pub mod_id: i64,
    pub title: String,
    pub description: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Authors {
    pub id: i64,
    pub name: String,
    pub url: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Categories {
    pub id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub name: String,
    pub slug: String,
    pub url: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "isClass")]
    pub is_class: bool,
    #[serde(rename = "classId")]
    pub class_id: i64,
    #[serde(rename = "parentCategoryId")]
    pub parent_category_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "websiteUrl")]
    pub website_url: Option<String>,
    #[serde(rename = "wikiUrl")]
    pub wiki_url: Option<String>,
    #[serde(rename = "issuesUrl")]
    pub issues_url: Option<String>,
    #[serde(rename = "sourceUrl")]
    pub source_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CurseForgePackItem {
    pub id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub name: String,
    pub slug: String,
    pub links: Links,
    pub summary: String,
    pub status: i64,
    #[serde(rename = "downloadCount")]
    pub download_count: i64,
    #[serde(rename = "isFeatured")]
    pub is_featured: bool,
    #[serde(rename = "primaryCategoryId")]
    pub primary_category_id: i64,
    pub categories: Vec<Categories>,
    #[serde(rename = "classId")]
    pub class_id: i64,
    pub authors: Vec<Authors>,
    pub logo: ImageItem,
    pub screenshots: Vec<ImageItem>,
    #[serde(rename = "mainFileId")]
    pub main_file_id: i64,
    #[serde(rename = "latestFiles")]
    pub latest_files: Vec<FileItem>,
    #[serde(rename = "latestFilesIndexes")]
    pub latest_files_indexes: Vec<FileIndex>,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "dateReleased")]
    pub date_released: String,
    #[serde(rename = "allowModDistribution")]
    pub allow_mod_distribution: bool,
    #[serde(rename = "gamePopularityRank")]
    pub game_popularity_rank: i64,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "thumbsUpCount")]
    pub thumbs_up_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CurseForgePackSearchResults {
    pub data: Vec<CurseForgePackItem>,
    pub pagination: Pagination,
}

impl CurseForgePackSearchResults {
    pub async fn search(options: BrowseOptions) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!(
			"https://api.curseforge.com/v1/mods/search?searchFilter={}&gameId=432&classId=4471&pageSize={}&index={}&sortOrder=desc",
			options.search, options.limit, options.offset
		);

        let api_key = CURSEFORGE_API_KEY;
        let client = reqwest::Client::new();
        let mut headers = header::HeaderMap::new();
        headers.insert("x-api-key", header::HeaderValue::from_str(api_key)?);

        let response = client.get(&url).headers(headers).send().await?;
        let response_text = response.text().await?;
        let parsed_response = serde_json::from_str::<Self>(&response_text).map_err(|e| {
            let line = e.line();
            let column = e.column();
            let part = response_text
                .lines()
                .nth(line - 1)
                .map(|line| {
                    let start = column.saturating_sub(100);
                    let end = (column + 100).min(line.len());
                    &line[start..end]
                })
                .unwrap_or("");
            let error_message = format!("JSON parsing error: {}. Response text: {}", e, part);
            Box::<dyn std::error::Error>::from(error_message)
        })?;
        Ok(parsed_response)
    }

    pub fn to_modpack_results(self) -> ModpackSearchResults {
        ModpackSearchResults {
            hits: self.data.into_iter().map(|item| item.to_modpack()).collect(),
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
            icon: Some(self.logo.url),
            gallery: Some(self.screenshots.into_iter().map(|screenshot| screenshot.url).collect()),
            game_versions: Some(
                self.latest_files
                    .iter()
                    .flat_map(|file| file.game_versions.clone())
                    .collect(),
            ),
            downloads: self.download_count as u32,
            likes: Some(self.thumbs_up_count as u32),
            last_updated: self.date_modified.parse::<chrono::DateTime<chrono::Utc>>().ok(),
            published: self.date_created.parse::<chrono::DateTime<chrono::Utc>>().ok(),
            platform: Platform::Curseforge,
            categories: Some(self.categories.into_iter().map(|cat| cat.name).collect()),
            project_url: Some(format!("https://www.curseforge.com/minecraft/modpacks/{}", self.slug)),
            author: self
                .authors
                .into_iter()
                .map(|author| author.name)
                .collect::<Vec<String>>()
                .join(", "),
            versions: Some(self.latest_files.into_iter().map(|file| file.display_name).collect()),
        }
    }
}
