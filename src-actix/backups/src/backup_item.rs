use crate::hashed_backup_item::HashedBackupItem;
use crate::{backup_db, get_backups_directory};
use archive_utility::archive_directory;
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum BackupCreationMethod {
    AUTO,
    MANUAL,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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

impl BackupItem {
    pub fn create_backup(
        server_id: impl AsRef<u32>,
        server_directory: impl AsRef<PathBuf>,
        method: BackupCreationMethod,
        r#type: BackupType,
    ) -> Result<BackupItem, String> {
        let output_file = Path::join(
            &get_backups_directory(),
            Path::new(&Uuid::new_v4().as_simple().to_string()),
        );
        if r#type == BackupType::Full {
            Self::create_full_backup(server_directory.as_ref(), &output_file)?;
        } else {
            Self::create_incremental_backup(server_directory.as_ref(), &output_file)?;
        }

        let output_metadata = output_file
            .metadata()
            .map_err(|e| format!("Error getting metadata for backup file: {:?}", e))?;

        backup_db::insert(BackupItem {
            id: 0,
            path: output_file,
            r#type,
            method,
            timestamp: SystemTime::now(),
            size: output_metadata.len(),
            server: *server_id.as_ref(),
        })
        .ok_or_else(|| "Error inserting backup into database".to_string())
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
    ) -> Result<(), String> {
        Ok(())
    }
}
