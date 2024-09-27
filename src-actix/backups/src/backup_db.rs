use crate::backup_item::{BackupCreationMethod, BackupItem, BackupType};
use crate::create_connection;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, info};
use sqlite::Statement;
use std::path::Path;
use std::time::SystemTime;

/// Inserts a new backup item into the database.
///
/// # Arguments
///
/// * `item` - A `BackupItem` struct containing the details of the backup to be inserted.
pub fn insert(item: BackupItem) {
	info!("Inserting a new backup item: {:?}", item);
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(_) => {
			error!("Failed to create a connection to the database.");
			return;
		}
	};
	let mut stmt = match conn.prepare("INSERT INTO backups (path, method, type, size, server) VALUES (?, ?, ?, ?, ?)") {
		Ok(stmt) => stmt,
		Err(e) => {
			error!("Unable to prepare the statement to insert the backup: {}", e);
			return;
		}
	};

	if let Err(e) = stmt.bind((1, item.path.to_str().unwrap())) {
		error!("Unable to bind the path to the statement: {}", e);
		return;
	}
	if let Err(e) = stmt.bind((2, item.method as i64)) {
		error!("Unable to bind the method to the statement: {}", e);
		return;
	}
	if let Err(e) = stmt.bind((3, item.r#type as i64)) {
		error!("Unable to bind the type to the statement: {}", e);
		return;
	}
	if let Err(e) = stmt.bind((4, item.size as i64)) {
		error!("Unable to bind the size to the statement: {}", e);
		return;
	}
	if let Err(e) = stmt.bind((5, item.server as i64)) {
		error!("Unable to bind the server to the statement: {}", e);
		return;
	}

	if let Err(e) = stmt.next() {
		error!("Unable to insert backup into database: {}", e);
		return;
	}
	let mut stmt = match conn.prepare("select seq from sqlite_sequence where name = 'backups' limit 1") {
		Ok(stmt) => stmt,
		Err(e) => {
			error!("Unable to prepare the statement to get the last inserted ID: {}", e);
			return;
		}
	};

	match stmt.next() {
		Some(Ok(row)) => {
			let id = row.read::<i64, _>("seq").map_err(|e| {
				error!("Unable to parse the column `seq` from the backups table in the `insert` function: {}", e);
				return;
			}).ok().unwrap() as u32;
			info!("Backup item inserted successfully with ID: {}", id);
		},
		_ => error!("Failed to insert backup item."),
	}
}

/// Deletes a backup item from the database by ID.
///
/// # Arguments
///
/// * `id` - The ID of the backup item to be deleted.
pub fn delete(id: u32) {
	info!("Deleting backup item with ID: {}", id);
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(_) => {
			error!("Failed to create a connection to the database.");
			return;
		}
	};
	let mut stmt = match conn.prepare("DELETE FROM backups WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => {
			error!("Unable to prepare the statement to delete the backup by id: {}", e);
			return;
		}
	};

	if let Err(e) = stmt.bind((1, id as i64)) {
		error!("Unable to bind the id to the statement: {}", e);
		return;
	}

	match stmt.next() {
		Some(Ok(_)) => info!("Backup item with ID {} deleted successfully.", id),
		_ => error!("Failed to delete backup item with ID {}.", id),
	}
}

/// Retrieves a backup item from the database by ID.
///
/// # Arguments
///
/// * `id` - The ID of the backup item to be retrieved.
///
/// # Returns
///
/// An `Option` containing the `BackupItem` if found, otherwise `None`.
pub fn get(id: u32) -> Option<BackupItem> {
	info!("Retrieving backup item with ID: {}", id);
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(_) => {
			error!("Failed to create a connection to the database.");
			return None;
		}
	};
	let mut stmt = match conn.prepare("SELECT * FROM backups WHERE id = ? LIMIT 1") {
		Ok(stmt) => stmt,
		Err(e) => {
			error!("Unable to prepare the statement to get the backup by id: {}", e);
			return None;
		}
	};

	if let Err(e) = stmt.bind((1, id as i64)) {
		error!("Unable to bind the id to the statement: {}", e);
		return None;
	}

	match stmt.next() {
		Some(Ok(row)) => {
			let item = from_statement(&row);
			info!("Backup item with ID {} retrieved successfully.", id);
			item
		},
		_ => {
			error!("Failed to retrieve backup item with ID {}.", id);
			None
		}
	}
}

/// Retrieves a list of all backup items from the database.
///
/// # Returns
///
/// A `Vec` containing all the `BackupItem`s.
pub fn list() -> Vec<BackupItem> {
	info!("Retrieving list of all backup items.");
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(_) => {
			error!("Failed to create a connection to the database.");
			return Vec::new();
		}
	};

	let mut stmt = match conn.prepare("SELECT * FROM backups") {
		Ok(stmt) => stmt,
		Err(e) => {
			error!("Unable to prepare the statement to get the list of backups: {}", e);
			return Vec::new();
		}
	};

	let mut items = Vec::new();
	while let Some(result) = stmt.next() {
		match result {
			Ok(row) => {
				match from_statement(&row) {
					Some(item) => items.push(item),
					None => {
						error!("Failed to convert row to BackupItem");
					}
				}
			}
			Err(e) => {
				error!("Error retrieving row: {}", e);
				continue; // or continue, depending on the desired behavior
			}
		}
	}
	info!("Retrieved list of all backup items successfully.");
	items
}

/// Retrieves a list of backup items from the database by server ID.
///
/// # Arguments
///
/// * `server_id` - The ID of the server to retrieve the backup items for.
///
/// # Returns
///
/// A `Vec` containing all the `BackupItem`s for the specified server.
/// If no backup items are found, an empty `Vec` is returned.
/// If an error occurs, an empty `Vec` is returned.
pub fn list_by_server(server_id: u32) -> Vec<BackupItem> {
	info!("Retrieving list of backup items for server with ID: {}", server_id);
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(_) => {
			error!("Failed to create a connection to the database.");
			return Vec::new();
		}
	};

	let mut stmt = match conn.prepare("SELECT * FROM backups WHERE server = ?") {
		Ok(stmt) => stmt,
		Err(e) => {
			error!("Unable to prepare the statement to get the list of backups by server id: {}", e);
			return Vec::new();
		}
	};

	if let Err(e) = stmt.bind((1, server_id as i64)) {
		error!("Unable to bind the server id to the statement: {}", e);
		return Vec::new();
	}

	let mut items = Vec::new();
	while let Some(result) = stmt.next() {
		match result {
			Ok(row) => {
				match from_statement(&row) {
					Some(item) => items.push(item),
					None => {
						error!("Failed to convert row to BackupItem");
					}
				}
			}
			Err(e) => {
				error!("Error retrieving row: {}", e);
				continue; // or continue, depending on the desired behavior
			}
		}
	}
	info!("Retrieved list of backup items for server with ID {} successfully.", server_id);
	items
}

fn get_last_inserted_id(conn: &sqlite::Connection) -> Option<u32> {
	let mut stmt = match conn.prepare("select seq from sqlite_sequence where name = 'backups' limit 1") {
		Ok(stmt) => stmt,
		Err(e) => {
			error!("Unable to prepare the statement to get the last inserted ID: {}", e);
			return None;
		}
	};

	match stmt.next() {
		Some(Ok(row)) => {
			Some(row.read::<i64, _>("seq").map_err(|e| {
				error!("Unable to parse the column `seq` from the backups table in the `get_last_inserted_id` function: {}", e);
				return None::<u32>;
			}).ok()? as u32)
		},
		_ => {
			error!("Failed to get last inserted ID.");
			None
		}
	}
	
	
}

/// Converts a SQLite statement row into a `BackupItem`.
///
/// # Arguments
///
/// * `stmt` - A reference to a `Statement` representing a row in the SQLite result set.
///
/// # Returns
///
/// An `Option` containing the `BackupItem` if the conversion is successful, otherwise `None`.
fn from_statement(stmt: &Statement) -> Option<BackupItem> {
	Some(
		BackupItem {
			id: stmt.read::<i64, _>("id").map_err(|e| {
				error!("Unable to parse the column `id` from the backups table in the `from_id` function: {}", e);
				return None::<BackupItem>;
			}).ok()? as u32,
			path: Path::new(&(stmt.read::<String, _>("path").map_err(|e| {
				error!("Unable to parse the column `path` from the backups table in the `from_id` function: {}", e);
				return None::<BackupItem>;
			}).ok()?)).to_path_buf(),
			method: match stmt.read::<i64, _>("method").map_err(|e| {
				error!("Unable to parse the column `method` from the backups table in the `from_id` function: {}", e);
				return None::<BackupItem>;
			}).ok()? {
				0 => BackupCreationMethod::AUTO,
				1 => BackupCreationMethod::MANUAL,
				_ => {
					error!("Unknown type value in the `from_id` function");
					return None::<BackupItem>;
				}
			},
			r#type: match stmt.read::<i64, _>("type").map_err(|e| {
				error!("Unable to parse the column `type` from the backups table in the `from_id` function: {}", e);
				return None::<BackupItem>;
			}).ok()? {
				0 => BackupType::Full,
				1 => BackupType::Incremental,
				_ => {
					error!("Unknown type value in the `from_id` function");
					return None::<BackupItem>;
				}
			},
			timestamp: SystemTime::from(DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::parse_from_str(
				&stmt.read::<String, _>("timestamp").map_err(|e| {
					error!("Unable to parse the column `timestamp` from the backups table in the `from_id` function: {}", e);
					return None::<BackupItem>;
				}).ok()?,
				"%Y-%m-%d %H:%M:%S"
			).ok()?, Utc)),
			size: stmt.read::<i64, _>("size").map_err(|e| {
				error!("Unable to parse the column `size` from the backups table in the `from_id` function: {}", e);
				return None::<BackupItem>;
			}).ok()? as u64,
			server: stmt.read::<i64, _>("server").map_err(|e| {
				error!("Unable to parse the column `server` from the backups table in the `from_id` function: {}", e);
				return None::<BackupItem>;
			}).ok()? as u32,
		}
	)
}