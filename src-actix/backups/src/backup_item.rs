use crate::hashed_file::{HashedFile, LazyHashedFile};
use crate::{create_connection, get_backups_directory};
use chrono::{DateTime, NaiveDateTime, Utc};
use crypto::hashids::encode;
use log::error;
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};
use sqlite::{State, Statement};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use uuid::Uuid;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

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
    id: u32,
    path: PathBuf,
    r#type: BackupType,
    method: BackupCreationMethod,
    timestamp: SystemTime,
    size: u64,
    server: u32,
}

#[derive(Debug, Serialize)]
pub struct HashedBackupItem {
    id: String,
    path: PathBuf,
    r#type: BackupType,
    method: BackupCreationMethod,
    timestamp: SystemTime,
    size: u64,
    server: u32,
}

impl BackupItem {
    pub fn create_backup(
        server_id: u32,
        server_directory: PathBuf,
        method: BackupCreationMethod,
        r#type: BackupType,
    ) -> Result<BackupItem, String> {
        let output_file = Path::join(
            &*get_backups_directory(),
            Path::new(&Uuid::new_v4().as_simple().to_string()),
        );
        let file = match File::create(&output_file) {
            Ok(f) => f,
            Err(e) => {
                error!("Unable to create backup file: {}", e);
                return Err(format!("Unable to create backup file: {}", e));
            }
        };
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

        if r#type == BackupType::Full {
            let all_files = WalkDir::new(&server_directory)
                .into_iter()
                .map(|e| e.unwrap())
                .collect::<Vec<_>>();
            for entry in all_files {
                if entry.file_type().is_dir() {
                    continue;
                }
                let relative_path = entry.path().strip_prefix(&server_directory).unwrap();
                zip.start_file_from_path(relative_path, options)
                    .map_err(|e| format!("Failed to start file from path: {}", e))?;
                let mut file = match File::open(entry.path()) {
                    Ok(f) => f,
                    Err(e) => {
                        error!("Failed to open file: {}", e);
                        return Err(format!("Failed to open file: {}", e));
                    }
                };
                let mut file_contents = vec![];
                file.read_to_end(&mut file_contents)
                    .map_err(|e| format!("Failed to read file: {}", e))?;
                zip.write(&file_contents)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
            }
        } else if r#type == BackupType::Incremental {
            let changed_files: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
            let lazy_hashed_files = Arc::new(Mutex::new(LazyHashedFile::new()));
            let all_files = match get_all_files_in_directory(&server_directory) {
                Ok(s) => s,
                Err(e) => {
                    error!("Unable to get files for backup: {}", e);
                    return Err(format!("Unable to get files for backup: {}", e));
                }
            };
            all_files.par_iter().for_each(|entry| {
                let relative_path = entry.strip_prefix(&server_directory).unwrap();
                if let Some(hashed_file) = HashedFile::get(entry) {
                    if hashed_file.has_file_been_changed() {
                        changed_files
                            .lock()
                            .unwrap()
                            .push(relative_path.to_str().unwrap().to_string());
                        lazy_hashed_files.lock().unwrap().add(entry.to_path_buf());
                    }
                } else {
                    changed_files
                        .lock()
                        .unwrap()
                        .push(relative_path.to_str().unwrap().to_string());
                    lazy_hashed_files.lock().unwrap().add(entry.to_path_buf());
                }
            });

            lazy_hashed_files.lock().unwrap().flush(); // Flush the lazy hashes to the database before creating the backup

            for entry in &*changed_files.lock().unwrap().deref_mut() {
                zip.start_file_from_path(entry, options)
                    .map_err(|e| format!("Failed to start file from path: {}", e))?;
                let mut file = match File::open(&Path::join(&server_directory, &entry)) {
                    Ok(f) => f,
                    Err(e) => {
                        error!("Failed to open file: {}", e);
                        return Err(format!("Failed to open file: {}", e));
                    }
                };
                let mut file_contents = vec![];
                file.read_to_end(&mut file_contents)
                    .map_err(|e| format!("Failed to read file: {}", e))?;
                zip.write(&file_contents)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
            }
        }

        if let Err(e) = zip.finish() {
            let msg = format!("Failed to flush archive data to file: {}", e);
            error!("{}", msg);
            return Err(msg);
        }

        let conn = match create_connection() {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("Unable to create connection: {}", e);
                error!("{}", msg);
                return Err(msg);
            }
        };

        let mut stmt = match conn.prepare(
            "INSERT INTO backups (path, type, method, size, server) VALUES (?, ?, ?, ?, ?)",
        ) {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("Failed to prepare backups insert statement: {}", e);
                error!("{}", msg);
                return Err(msg);
            }
        };
        let metadata = match output_file.metadata() {
            Ok(m) => m,
            Err(e) => {
                let msg = format!("Failed to get metadata from backup archive: {}", e);
                error!("{}", msg);
                return Err(msg);
            }
        };
        let item_type: i64 = r#type.clone() as i64;
        let item_method: i64 = method.clone() as i64;
        let item = BackupItem {
            id: 0,
            path: output_file.clone(),
            server: server_id,
            method,
            r#type,
            size: metadata.len(),
            timestamp: SystemTime::now(),
        };

        stmt.bind((1, output_file.to_str().unwrap()))
            .map_err(|e| format!("Failed to bind {} -> path: {}", output_file.display(), e))?;
        stmt.bind((2, item_type))
            .map_err(|e| format!("Failed to bind {:?} -> type: {}", item.r#type, e))?;
        stmt.bind((3, item_method))
            .map_err(|e| format!("Failed to bind {:?} -> method: {}", item.method, e))?;
        stmt.bind((4, item.size as i64))
            .map_err(|e| format!("Failed to bind {} -> size: {}", item.size, e))?;
        stmt.bind((5, item.server as i64))
            .map_err(|e| format!("Failed to bind {} -> server: {}", item.server, e))?;

        stmt.next()
            .map_err(|e| format!("Unable to insert backup into database: {}", e))?;

        let mut stmt =
            match conn.prepare("select seq from sqlite_sequence where name = 'backups' limit 1") {
                Ok(c) => c,
                Err(e) => {
                    let msg = format!("Failed to prepare sqlite_sequence selection: {}", e);
                    error!("{}", msg);
                    return Err(msg);
                }
            };

        if let Err(e) = stmt.next() {
            let msg = format!("Failed to execute sql command: {}", e);
            error!("{}", msg);
            return Err(msg);
        }

        let inserted_id = stmt
            .read::<i64, _>("seq")
            .map_err(|e| format!("Failed to execute sql command: {}", e))?;

        Ok(BackupItem {
            id: inserted_id as u32,
            path: item.path,
            server: item.server,
            method: item.method,
            r#type: item.r#type,
            size: item.size,
            timestamp: item.timestamp,
        })
    }

    pub fn create_world_edit_backup(server_dir: PathBuf, world_directory: PathBuf) {
        let output_file = Path::join(server_dir.as_path(), Path::new("backups"));

        let file = match File::create(&output_file) {
            Ok(f) => f,
            Err(e) => {
                error!("Unable to create backup file: {}", e);
                return;
            }
        };
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        let all_files = WalkDir::new(&world_directory)
            .into_iter()
            .map(|e| e.unwrap())
            .collect::<Vec<_>>();
        for entry in all_files {
            if entry.file_type().is_dir() {
                continue;
            }
            let relative_path = entry.path().strip_prefix(&world_directory).unwrap();
            zip.start_file_from_path(relative_path, options).unwrap();
            let mut file = match File::open(entry.path()) {
                Ok(f) => f,
                Err(e) => {
                    error!("Failed to open file: {}", e);
                    return;
                }
            };
            let mut file_contents = vec![];
            file.read_to_end(&mut file_contents).unwrap();
            zip.write(&file_contents).unwrap();
        }
    }

    pub fn trim(server_id: u32, items_to_keep: u32) {
        let backups = Self::get_list_of_backups_from_server(server_id);
        if backups.len() <= items_to_keep as usize {
            return;
        }
        for i in backups.len()..items_to_keep as usize {
            Self::delete_backup(backups.get(i).unwrap().id);
        }
    }

    pub fn delete_backup(id: u32) {
        if let Some(backup) = Self::from_id(id) {
            if backup.path.exists() {
                match std::fs::remove_file(&backup.path) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Unable to remove backup file '{:?}': {}", backup.path, e);
                    }
                }
            }
            let conn = match create_connection() {
                Ok(c) => c,
                Err(_) => {
                    return;
                }
            };
            let mut stmt = match conn.prepare("delete from `backups` where id = ?") {
                Ok(s) => s,
                Err(e) => {
                    error!("Unable to prepare select statement for the `delete_backup` function of the backups class: {}", e);
                    return;
                }
            };

            match stmt.bind((1, id as i64)) {
                Ok(_) => {}
                Err(e) => {
                    error!("Unable to bind parameter: {}", e)
                }
            }
        }
    }

    pub fn get_list_of_backups() -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        let conn = match create_connection() {
            Ok(c) => c,
            Err(_) => {
                return result;
            }
        };
        let mut stmt = match conn.prepare("select * from `backups`") {
            Ok(s) => s,
            Err(e) => {
                error!("Unable to prepare select statement for the `get_list_of_backups` function of the backups class: {}", e);
                return result;
            }
        };

        while let State::Row = stmt
            .next()
            .map_err(|e| {
                error!("Unable to get next row from the database: {}", e);
                return State::Done;
            })
            .unwrap()
        {
            if let Some(item) = Self::from_statement(&stmt) {
                result.push(item);
            }
        }

        result
    }
    pub fn get_list_of_backups_from_server(server_id: u32) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        let conn = match create_connection() {
            Ok(c) => c,
            Err(_) => {
                return result;
            }
        };
        let mut stmt = match conn
            .prepare("select * from `backups` where server = ? order by timestamp")
        {
            Ok(s) => s,
            Err(e) => {
                error!("Unable to prepare select statement for the `get_list_of_backups` function of the backups class: {}", e);
                return result;
            }
        };

        match stmt.bind((1, server_id as i64)) {
            Ok(_) => {}
            Err(e) => {
                error!("Unable to bind '{}' -> `server_id` in the `get_list_of_backups_from_server` function of the backups class: {}", server_id, e);
                return result;
            }
        }

        while let State::Row = stmt
            .next()
            .map_err(|e| {
                error!("Unable to get next row from the database: {}", e);
                return State::Done;
            })
            .unwrap()
        {
            if let Some(item) = Self::from_statement(&stmt) {
                result.push(item);
            }
        }

        result
    }
    pub fn from_id(id: u32) -> Option<Self> {
        let conn = match create_connection() {
            Ok(c) => c,
            Err(_) => {
                return None;
            }
        };
        let mut stmt = match conn.prepare("select * from `backups` where 'id' = ? LIMIT 1") {
            Ok(s) => s,
            Err(e) => {
                error!("Unable to prepare select statement for the `from_id` function of the backups class: {}", e);
                return None;
            }
        };
        match stmt.bind((1, id as i64)) {
            Ok(_) => {}
            Err(e) => {
                error!("Unable to bind '{}' -> `id` in the `from_id` function of the backups class: {}", id, e);
                return None;
            }
        }
        match stmt.next() {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to get result of select query in the `from_id` function of the backups class: {}", e);
                return None;
            }
        };

        Self::from_statement(&stmt)
    }

    fn from_statement(stmt: &Statement) -> Option<Self> {
        Some(
			BackupItem {
				id: stmt.read::<i64, _>("id").map_err(|e| {
					error!("Unable to parse the column `id` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()? as u32,
				path: Path::new(&(stmt.read::<String, _>("path").map_err(|e| {
					error!("Unable to parse the column `path` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()?)).to_path_buf(),
				method: match stmt.read::<i64, _>("method").map_err(|e| {
					error!("Unable to parse the column `method` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()? {
					0 => BackupCreationMethod::AUTO,
					1 => BackupCreationMethod::MANUAL,
					_ => {
						error!("Unknown type value in the `from_id` function");
						return None::<Self>;
					}
				},
				r#type: match stmt.read::<i64, _>("type").map_err(|e| {
					error!("Unable to parse the column `type` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()? {
					0 => BackupType::Full,
					1 => BackupType::Incremental,
					_ => {
						error!("Unknown type value in the `from_id` function");
						return None::<Self>;
					}
				},
				timestamp: SystemTime::from(DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::parse_from_str(
					&stmt.read::<String, _>("timestamp").map_err(|e| {
						error!("Unable to parse the column `timestamp` from the backups table in the `from_id` function: {}", e);
						return None::<Self>;
					}).ok()?,
					"%Y-%m-%d %H:%M:%S"
				).ok()?, Utc)),
				size: stmt.read::<i64, _>("size").map_err(|e| {
					error!("Unable to parse the column `size` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()? as u64,
				server: stmt.read::<i64, _>("server").map_err(|e| {
					error!("Unable to parse the column `server` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()? as u32,
			}
		)
    }

    pub fn hash(&self) -> HashedBackupItem {
        HashedBackupItem::from_backup_item(self.clone())
    }
}
impl HashedBackupItem {
    pub fn from_backup_item(item: BackupItem) -> Self {
        HashedBackupItem {
            id: encode(&[item.id as u64]),
            path: item.path,
            method: item.method,
            r#type: item.r#type,
            timestamp: item.timestamp,
            size: item.size,
            server: item.server,
        }
    }
}

fn get_all_files_in_directory<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut result: Vec<PathBuf> = vec![];
    let entries = match std::fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(e) => return Err(Box::new(e)), // Propagate the error upwards
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!(
                    "Error reading directory entry in {}: {}",
                    path.as_ref().display(),
                    e
                );
                continue; // Continue to next entry instead of propagation
            }
        };
        let path = entry.path();
        if entry.metadata()?.is_dir() {
            match get_all_files_in_directory(&path) {
                Ok(mut items) => result.append(&mut items),
                Err(e) => {
                    error!("Error reading directory {}: {}", path.display(), e);
                    // Consider whether to return the error or continue
                }
            }
        } else {
            result.push(path);
        }
    }
    Ok(result)
}
