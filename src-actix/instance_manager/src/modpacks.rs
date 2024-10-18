use crate::Platform;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::AddAssign;

#[derive(Serialize, Deserialize)]
pub struct ModpackSearchResults {
    pub hits: Vec<Modpack>,
    pub offset: i64,
    pub limit: i64,
    pub total_hits: i64,
}
#[derive(Serialize, Deserialize)]
pub struct Modpack {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) description: String,
    pub(crate) icon: Option<String>,
    pub(crate) gallery: Option<Vec<String>>,
    pub(crate) versions: Option<Vec<String>>,
    pub(crate) game_versions: Option<Vec<String>>,
    pub(crate) downloads: u32,
    pub(crate) likes: Option<u32>,
    pub(crate) last_updated: Option<DateTime<Utc>>,
    pub(crate) published: Option<DateTime<Utc>>,
    pub(crate) platform: Platform,
    pub(crate) categories: Option<Vec<String>>,
    pub(crate) project_url: Option<String>,
}

#[derive(Deserialize, Clone)]
pub enum SortOptions {
    Relevance,
    Downloads,
    Follows,
    Newest,
    Updated,
}

impl Display for SortOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SortOptions::Relevance => "relevance".to_string(),
            SortOptions::Downloads => "downloads".to_string(),
            SortOptions::Follows => "follows".to_string(),
            SortOptions::Newest => "newest".to_string(),
            SortOptions::Updated => "updated".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Deserialize, Clone)]
pub struct BrowseOptions {
    pub search: String,
    pub sort: SortOptions,
    pub platform: Platform,
    pub limit: u32,
    pub offset: u32,
}

impl ModpackSearchResults {
    pub fn merge(&mut self, other: ModpackSearchResults) {
        self.hits.extend(other.hits);
        self.offset = other.offset;
        self.limit += other.limit;
        self.total_hits += other.total_hits;
    }
}

impl AddAssign for ModpackSearchResults {
    fn add_assign(&mut self, other: Self) {
        self.merge(other);
    }
}

pub async fn search_modpacks(
    options: BrowseOptions,
) -> Result<ModpackSearchResults, Box<dyn std::error::Error>> {
    match options.platform {
        Platform::All => {
            let mut results = ModpackSearchResults {
                hits: vec![],
                offset: 0,
                limit: 0,
                total_hits: 0,
            };

            let modrinth_thread = std::thread::spawn({
                let options = options.clone();
                move || crate::modrinth::ModrinthPackSearchResults::search(options.clone())
            });

            let curseforge_thread = std::thread::spawn({
                let options = options.clone();
                move || crate::curseforge::CurseForgePackSearchResults::search(options.clone())
            });

            //            let atlauncher_thread = std::thread::spawn({
            //                crate::atlauncher::ATLauncherPackSearchResults::search
            //            });

            results += modrinth_thread.join().unwrap().await?.to_modpack_results();
            results += curseforge_thread
                .join()
                .unwrap()
                .await?
                .to_modpack_results();
            //            results += atlauncher_thread.join().unwrap().await?.to_modpack_results();

            // sort results by downloads
            results.hits.sort_by(|a, b| b.downloads.cmp(&a.downloads));

            Ok(results)
        }
        Platform::Curseforge => Ok(crate::curseforge::CurseForgePackSearchResults::search(
            options.clone(),
        )
        .await?
        .to_modpack_results()),

        Platform::Modrinth => Ok(crate::modrinth::ModrinthPackSearchResults::search(options)
            .await?
            .to_modpack_results()),
        Platform::AtLauncher => Ok(crate::atlauncher::ATLauncherPackSearchResults::search()
            .await?
            .to_modpack_results()),
    }
}
