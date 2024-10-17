use crate::modpacks::{BrowseOptions, Modpack, ModpackSearchResults};
use crate::Platform;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct ModrinthPackSearchResults {
    pub hits: Vec<ModrinthPackItem>,
    pub offset: i64,
    pub limit: i64,
    pub total_hits: i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct ModrinthPackItem {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: String,
    pub server_side: String,
    pub project_type: String,
    pub downloads: i64,
    pub icon_url: String,
    pub color: i64,
    pub thread_id: String,
    pub monetization_status: String,
    pub project_id: String,
    pub author: String,
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub follows: i64,
    pub date_created: String,
    pub date_modified: String,
    pub latest_version: String,
    pub license: String,
    pub gallery: Vec<String>,
    pub featured_gallery: String,
}

impl ModrinthPackSearchResults {
    pub async fn search(options: BrowseOptions) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!(
            r#"	https://api.modrinth.com/v2/search?limit={limit}&offset={offset}&index={sort}&facets=[["client_side:optional","client_side:unsupported"],["server_side:optional","server_side:required"],["project_type:modpack"]]&query={query}"#,
            query = options.search,
            sort = options.sort,
            limit = options.limit,
            offset = options.offset
        );

        Ok(reqwest::get(&url).await?.json::<Self>().await?)
    }
    pub fn to_modpack_results(self) -> ModpackSearchResults {
        ModpackSearchResults {
            hits: self
                .hits
                .into_iter()
                .map(|item| item.to_modpack())
                .collect(),
            offset: self.offset,
            limit: self.limit,
            total_hits: self.total_hits,
        }
    }
}

impl ModrinthPackItem {
    pub fn to_modpack(self) -> Modpack {
        Modpack {
            id: self.project_id,
            name: self.title,
            description: self.description,
            icon: Some(self.icon_url),
            gallery: Some(self.gallery),
            game_versions: Some(self.versions),
            downloads: self.downloads as u32,
            likes: Some(self.follows as u32),
            last_updated: self
                .date_modified
                .parse::<chrono::DateTime<chrono::Utc>>()
                .ok(),
            published: self
                .date_created
                .parse::<chrono::DateTime<chrono::Utc>>()
                .ok(),
            platform: Platform::Modrinth,
            categories: Some(self.categories),
            project_url: Some(format!("https://modrinth.com/modpack/{}", self.slug)),
            author: self.author,
            versions: Some(vec![self.latest_version]),
        }
    }
}
