use crate::hashed_backup_item::HashedBackupItem;
use crate::hashed_file::HashedFile;
use crate::lazy_hashed_file::LazyHashedFile;
use crate::{backup_db, get_backups_directory};
use archive_utility::archive_directory;
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
pub enum BackupCreationMethod {
    AUTO,
    MANUAL,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
pub enum BackupType {
    Full,
    Incremental,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct BackupItem {
    pub id: u32,
    pub path: PathBuf,
    pub r#type: BackupType,
    pub method: BackupCreationMethod,
    pub timestamp: SystemTime,
    pub size: u64,
    pub server: u32,
}

#[derive(Debug)]
pub struct BackupError {
    message: String,
    method: Option<BackupCreationMethod>,
    r#type: Option<BackupType>,
}

impl Display for BackupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Backup Error: {}\n\t- Method: {:?}\n\t- Type: {:?}",
            self.message, self.method, self.r#type
        )
    }
}

impl Error for BackupError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl BackupItem {
    pub fn create_backup(
        server_id: u32,
        server_directory: impl AsRef<Path>,
        method: BackupCreationMethod,
        r#type: BackupType,
    ) -> Result<BackupItem, BackupError> {
        let output_file = Path::join(
            &get_backups_directory(),
            Path::new(&Uuid::new_v4().as_simple().to_string()),
        );
        if r#type == BackupType::Full {
            Self::create_full_backup(server_directory.as_ref(), &output_file).map_err(|e| {
                BackupError {
                    message: format!("Error creating full backup: {:?}", e),
                    method: Some(method),
                    r#type: Some(r#type),
                }
            })?;
        } else {
            Self::create_incremental_backup(server_directory.as_ref(), &output_file).map_err(
                |e| BackupError {
                    message: format!("Error creating incremental backup: {:?}", e),
                    method: Some(method),
                    r#type: Some(r#type),
                },
            )?;
        }

        let output_metadata = output_file.metadata().map_err(|e| BackupError {
            message: format!("Error getting metadata for backup file: {:?}", e),
            method: Some(method),
            r#type: Some(r#type),
        })?;

        backup_db::insert(BackupItem {
            id: 0,
            path: output_file,
            r#type,
            method,
            timestamp: SystemTime::now(),
            size: output_metadata.len(),
            server: server_id,
        })
        .ok_or(BackupError {
            message: "Error inserting backup into database".to_string(),
            method: Some(method),
            r#type: Some(r#type),
        })
    }

    pub fn create_world_edit_backup(
        server_dir: PathBuf,
        world_directory: PathBuf,
    ) -> Result<(), String> {
        let output_file = Path::join(server_dir.as_path(), Path::new("backups"));
        archive_directory(world_directory, output_file, &|_| true)
            .map_err(|e| format!("Error creating WorldEdit backup: {:?}", e))?;
        Ok(())
    }

    pub fn trim(server_id: u32, items_to_keep: u32) {
        let backups = Self::from_server(server_id);
        if backups.len() <= items_to_keep as usize {
            return;
        }
        for i in backups.len()..items_to_keep as usize {
            Self::delete(backups.get(i).unwrap().id);
        }
    }

    pub fn delete(id: u32) {
        backup_db::delete(id);
    }

    pub fn list() -> Vec<Self> {
        backup_db::list()
    }
    pub fn from_server(server_id: u32) -> Vec<Self> {
        backup_db::list_by_server(server_id)
    }
    pub fn from_id(id: u32) -> Option<Self> {
        backup_db::get(id)
    }

    pub fn hash(self) -> HashedBackupItem {
        HashedBackupItem::from_backup_item(self)
    }

    fn create_full_backup(
        server_directory: impl AsRef<Path>,
        archive_path: impl AsRef<Path>,
    ) -> Result<(), String> {
        archive_directory(server_directory, archive_path, &|_| true)
            .map_err(|e| format!("Error creating full backup: {:?}", e))?;
        Ok(())
    }

    fn create_incremental_backup(
        server_directory: impl AsRef<Path>,
        archive_path: impl AsRef<Path>,
    ) -> Result<(), Box<dyn Error>> {
        let lazy_hashed_files = Arc::new(Mutex::new(LazyHashedFile::new()));
        WalkDir::new(server_directory.as_ref())
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .par_bridge()
            .for_each(|entry| {
                let path = entry.path();
                let hash = match HashedFile::from_path(path) {
                    Ok(h) => h,
                    Err(e) => {
                        log::error!("Failed to hash file '{:?}': {}", path, e);
                        return;
                    }
                };
                let changed = hash.changed();
                if changed.is_err() || changed.unwrap() {
                    let mut lazy_hashed_files = lazy_hashed_files.lock().unwrap();
                    lazy_hashed_files.add(hash);
                }
            });
        let mut lazy_hashed_files = lazy_hashed_files.lock().unwrap();
        let lazy_hashed_files = lazy_hashed_files.deref_mut();
        lazy_hashed_files.flush();
        archive_directory(server_directory, archive_path, &|path| {
            lazy_hashed_files.get(path).is_some()
        })
        .map_err(|e| format!("Error creating incremental backup: {:?}", e))?;

        Ok(())
    }
}
