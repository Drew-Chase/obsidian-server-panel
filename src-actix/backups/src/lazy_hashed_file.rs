use crate::hashed_file::HashedFile;
use log::error;
use std::path::Path;

pub struct LazyHashedFile {
    items: Vec<HashedFile>,
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

    pub fn add(&mut self, item: HashedFile) {
        self.items.push(item);
    }

    pub fn get(&self, path: &Path) -> Option<&HashedFile> {
        self.items.iter().find(|item| item.path == path)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn flush(&self) {
        for item in self.items.iter() {
            match item.cache() {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to cache file hash for '{:?}': {}", item.path, e);
                }
            }
        }
    }
}
