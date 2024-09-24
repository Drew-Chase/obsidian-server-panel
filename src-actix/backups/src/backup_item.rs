use crate::hashed_file::HashedFile;
use crate::{create_connection, get_backups_directory};
use log::{error, info};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sevenz_rust::{lzma, Archive, SevenZWriter};
use sha2::{Digest, Sha256};
use sqlite::{State, Statement};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use uuid::{uuid, Uuid};

pub enum BackupCreationMethod {
	AUTO,
	MANUAL,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum BackupType {
	Full,
	Incremental,
}

pub struct BackupItem {
	id: u32,
	name: String,
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
		compression_level: u8,
		r#type: BackupType,
	) -> Result<BackupItem, String> {
		let output_file = Path::join(&*get_backups_directory(), Path::new(&Uuid::new_v4().as_simple().to_string()));

		let mut archive = match SevenZWriter::create(output_file) {
			Ok(a) => a,
			Err(e) => {
				error!("Unable to create backup archive: {}", e);
				return Err(format!("Unable to create backup archive: {}", e));
			}
		};

		archive.set_content_methods(vec![
			lzma::LZMA2Options::with_preset(compression_level as u32).into(),
		]);

		if r#type == BackupType::Full {
			match archive.push_source_path(&server_directory, |e| { true }) {
				Ok(_) => {},
				Err(e) => {
					let msg = format!("Unable to archive directory for backup: {}", e);
					error!(msg);
					return Err(msg);
				}
			}
		} else if r#type == BackupType::Incremental {
			let mut changed_files: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
			let all_files = match get_all_files_in_directory(&server_directory) {
				Ok(s) => s,
				Err(e) => {
					error!("Unable to get files for backup: {}", e);
					return Err(format!("Unable to get files for backup: {}", e));
				}
			};
			all_files.par_iter().for_each(|entry: &PathBuf| {
				let mut arr = match changed_files.lock() {
					Ok(s) => s,
					Err(e) => {
						error!("Unable to lock changed files array: {}", e);
						return;
					}
				};
				if let Some(file) = HashedFile::get(entry.as_path()) { // Attempts to retrieve the hashed file from the database.
					if file.clone().has_file_been_changed() {
						HashedFile::cache_file_hash(entry);
						arr.push(entry.to_str().unwrap().to_string());
					}
				} else {
					// Files has not been previously backed up.
					HashedFile::cache_file_hash(entry);
					arr.push(entry.to_str().unwrap().to_string());
				}
			});


			match archive.push_source_path(server_directory, |e| { all_files.contains(&e.to_path_buf()) }) {
				Ok(_) => {},
				Err(e) => {
					let msg = format!("Unable to archive directory for backup: {}", e);
					error!(msg);
					return Err(msg);
				}
			}
		}

		Err("".to_string())
	}


	pub fn create_world_edit_backup(server_dir: PathBuf, world_directory: PathBuf)
	{}

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
		let mut stmt = match conn.prepare("select * from `backups` where server = ?") {
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
					error!("Unable to parse the column `id` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()?)).to_path_buf(),

				name: stmt.read::<String, _>("name").map_err(|e| {
					error!("Unable to parse the column `name` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()?,
				method: match stmt.read::<i64, _>("method").map_err(|e| {
					error!("Unable to parse the column `type` from the backups table in the `from_id` function: {}", e);
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
				timestamp: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(
					stmt.read::<String, _>("timestamp").map_err(|e| {
						error!("Unable to parse the column `timestamp` from the backups table in the `from_id` function: {}", e);
						return None::<Self>;
					}).ok()?.parse::<u64>().map_err(|e| {
						error!("Unable to convert timestamp string to u64 in the `from_id` function: {}", e);
						return None::<Self>;
					}).ok()?
				),
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
				error!("Error reading directory entry in {}: {}", path.as_ref().display(), e);
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
				},
			}
		} else {
			result.push(path);
		}
	}
	Ok(result)
}