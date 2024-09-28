use crate::hashed_file::{HashedFile, LazyHashedFile};
use log::error;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct LazyHashedFile {
    pub items: Vec<HashedFile>,
}

impl Default for LazyHashedFile {
    fn default() -> Self {
        Self::new()
    }
}

impl LazyHashedFile {
    pub fn new() -> Self {
        LazyHashedFile { items: Vec::new() }
    }

    pub fn add(&mut self, path: PathBuf) {
        let hash = match HashedFile::hash_file(path.as_path()) {
            Ok(h) => h,
            Err(e) => {
                error!("Failed to hash file '{:?}': {}", path, e);
                return;
            }
        };
        self.items.push(HashedFile {
            path,
            hash,
            timestamp: SystemTime::now(),
        });
    }

    pub fn get(&self, path: &Path) -> Option<&HashedFile> {
        self.items.iter().find(|item| item.path == path)
    }

    pub fn flush(&self) {
        for item in self.items.iter() {
            match HashedFile::cache_file_hash(item.path.as_path()) {
                Some(_) => {}
                None => {
                    error!("Failed to cache file hash for '{:?}'", item.path);
                }
            }
        }
    }
}

fn create_connection() -> Result<sqlite::Connection, sqlite::Error> {
    sqlite::Connection::open("servers.db").map_err(|e| {
        error!(
            "Failed to open servers database connection for backups: {}",
            e
        );
        e
    })
}
