use log::{error, info};
use sqlite::State;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub enum BackupItemType {
	AUTO,
	MANUAL
}

pub struct BackupItem {
	id: u32,
	name: String,
	path: PathBuf,
	r#type: BackupItemType,
	timestamp: SystemTime,
	size: u64,
	server: u32,
}

impl BackupItem {
	pub fn from_id(id: u32) -> Option<Self> {
		let conn = match create_connection() {
			Ok(c) => c,
			Err(_) => {
				return None;
			}
		};
		let mut stmt = match conn.prepare("select * from `backups` where 'id' = ?") {
			Ok(s) => s,
			Err(e) => {
				error!("Unable to prepare select statement for the `from_id` function of the backups class.");
				return None;
			}
		};
		match stmt.bind((1, id as i64)) {
			Ok(_) => {},
			Err(e) => {
				error!("Unable to bind '{}' -> `id` in the `from_id` function of the backups class: {}", id, e);
				return None;
			}
		}
		let result = match stmt.next() {
			Ok(r) => r,
			Err(e) => {
				error!("Failed to get result of select query in the `from_id` function of the backups class: {}", e);
				return None;
			}
		};

		if result == State::Done { return None; }

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
				r#type: match stmt.read::<String, _>("type").map_err(|e| {
					error!("Unable to parse the column `type` from the backups table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()?.as_str() {
					"AUTO" => BackupItemType::AUTO,
					"MANUAL" => BackupItemType::MANUAL,
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


pub fn initialize() {
	info!("Initializing backups database");
	let conn = create_connection().expect("Failed to connect to database");
	if let Err(e) = conn.execute(
		"
		CREATE TABLE IF NOT EXISTS backups (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL,
			path TEXT NOT NULL,
			type TEXT NOT NULL,
			timestamp TEXT NOT NULL,
			size INTEGER NOT NULL,
			server INTEGER NOT NULL
		)
	",
	) {
		error!("Failed to create backups table: {}", e);
	} else {
		info!("Successfully created or verified the backups table.");
	}
}


fn create_connection() -> Result<sqlite::Connection, sqlite::Error> {
	sqlite::Connection::open("servers.db").map_err(|e| {
		error!("Failed to open servers database connection for backups: {}", e);
		e
	})
}